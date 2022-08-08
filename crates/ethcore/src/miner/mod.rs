// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of OpenEthereum.

// OpenEthereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// OpenEthereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with OpenEthereum.  If not, see <http://www.gnu.org/licenses/>.

#![warn(missing_docs)]

//! Miner module
//! Keeps track of transactions and currently sealed pending block.

use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;

use bytes::{Bytes, ToPretty};
use ethereum_types::{Address, BigEndianHash, H64, U256, U64};
use hash::{H256, keccak};

use ethtrie::TrieFactory;
use evm::VMType;
use hash_db::HashDB;
use keccak_hasher::KeccakHasher;
use trie::{DBValue, TrieSpec};
use types::header::Header;
use types::transaction;
use types::transaction::UnverifiedTransaction;
use vm::LastHashes;

use crate::block::{OpenBlock, SealedBlock};
use crate::engines::EthEngine;
use crate::error::Error;
use crate::executed::ExecutionError;
use crate::factory::{Factories, VmFactory};
use crate::state_db::StateDB;

/// Riscv evm execution env.
pub struct BlockGenInfo {
    pub parent_block_header: Header,
    pub last_hashes: Arc<LastHashes>,
    pub author: Address,
    pub gas_range_target: (U256, U256),
    pub extra_data: Bytes,
}

const MB: usize = 1024 * 1024;

impl BlockGenInfo {
    /// Create a new client with given parameters.
    pub fn new(
        parent_block_header: Header, last_hashes: Arc<LastHashes>, author: Address,
        gas_range_target: (U256, U256), extra_data: Bytes,
    ) -> BlockGenInfo {
        BlockGenInfo { last_hashes, parent_block_header, author, gas_range_target, extra_data }
    }
}

/// Allowed number of skipped transactions when constructing pending block.
///
/// When we push transactions to pending block, some of the transactions might
/// get skipped because of block gas limit being reached.
/// This constant controls how many transactions we can skip because of that
/// before stopping attempts to push more transactions to the block.
/// This is an optimization that prevents traversing the entire pool
/// in case we have only a fraction of available block gas limit left.
const MAX_SKIPPED_TRANSACTIONS: usize = 128;

/// generate and seal new block.
pub fn generate_block(
    db: Box<dyn HashDB<KeccakHasher, DBValue>>, engine: &impl EthEngine, info: &BlockGenInfo,
    txes: Vec<UnverifiedTransaction>, l2_witness_layer: Address,
) -> Option<SealedBlock> {
    let trie_factory = TrieFactory::new(TrieSpec::Secure);
    let factories = Factories {
        vm: VmFactory::new(VMType::Interpreter, MB),
        trie: trie_factory,
        accountdb: Default::default(),
    };
    let state_db = StateDB::new(db, MB);

    let mut open_block = OpenBlock::new(
        engine,
        factories,
        true,
        state_db,
        &info.parent_block_header,
        info.last_hashes.clone(),
        info.author,
        info.gas_range_target,
        info.extra_data.clone(),
    )
        .ok()?;

    let block_number = open_block.header.number();
    let mut skipped_transactions = 0usize;
    let schedule = engine.schedule(block_number);
    let min_tx_gas: U256 = schedule.tx_gas.into();

    let event_sig = "MessageSent(uint64,address,address,bytes32,bytes)".as_bytes();
    let event_id = keccak(event_sig);
    let mut mmr_size: u64 = 0;
    let mut mmr_root = H256::zero();

    for transaction in txes {
        let transaction = {
            match engine.machine().verify_transaction_unordered(transaction, &open_block.header) {
                Err(e) => continue,
                Ok(t) => t,
            }
        };
        // Re-verify transaction again vs current state.
        let result = engine
            .machine()
            .verify_transaction_basic(&transaction, &open_block.header)
            .map_err(|e| e.into())
            .and_then(|_| open_block.push_transaction(transaction, None));

        match result {
            Err(Error::Execution(ExecutionError::BlockGasLimitReached {
                                     gas_limit,
                                     gas_used,
                                     gas,
                                 })) => {
                //debug!(target: "miner", "Skipping adding transaction to block because of gas limit: {:?} (limit: {:?}, used: {:?}, gas: {:?})", hash, gas_limit, gas_used, gas);
                // Exit early if gas left is smaller then min_tx_gas
                let gas_left = gas_limit - gas_used;
                if gas_left < min_tx_gas {
                    //debug!(target: "miner", "Remaining gas is lower than minimal gas for a transaction. Block is full.");
                    break;
                }

                // Avoid iterating over the entire queue in case block is almost full.
                skipped_transactions += 1;
                if skipped_transactions > MAX_SKIPPED_TRANSACTIONS {
                    //debug!(target: "miner", "Reached skipped transactions threshold. Assuming block is full.");
                    break;
                }
            }
            // Invalid nonce error can happen only if previous transaction is skipped because of gas limit.
            // If there is errornous state of transaction queue it will be fixed when next block is imported.
            Err(Error::Execution(ExecutionError::InvalidNonce { .. })) => {}
            // already have transaction - ignore
            Err(Error::Transaction(transaction::Error::AlreadyImported)) => {}
            Err(Error::Transaction(transaction::Error::NotAllowed)) => {
                //debug!(target: "miner", "Skipping non-allowed transaction for sender {:?}", hash);
            }
            Err(_e) => {
                panic!("{}", _e);
            }
            // imported ok
            Ok(receipt) => {
                for log in receipt.logs.iter() {
                    if log.address == l2_witness_layer {
                        if log.topics[0] == event_id {
                            mmr_size = U256::from(log.topics[1].as_bytes()).as_u64();
                            mmr_root = H256::from_slice(&log.data[0..32]);
                        }
                    }
                }
            }
        }
    }

    if mmr_root != H256::zero() {
        open_block.update_mmr(mmr_size + 1, mmr_root);
    }
    let closed_block = open_block.close();
    match closed_block {
        Ok(t) => {
            let sealed_block = t.lock().try_seal(engine, Vec::new()).expect("seal failed");
            Some(sealed_block)
        }
        Err(e) => panic!("{}", e),
    }
}
