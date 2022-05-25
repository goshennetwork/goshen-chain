#![no_std]

extern crate alloc;

mod consts;
mod input;
mod machine;

use crate::consts::{L2_BLOCK_MAX_GAS_LIMIT, L2_BLOCK_MIN_GAS_LIMIT, L2_FEE_COLLECTOR};
use crate::input::load_last_hashes;
use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
use ethcore::engines::L2Seal;
use ethcore::miner::{generate_block, BlockGenInfo};
use ethereum_types::{Address, H256};
use hash_db::HashDB;
use input::RollupInput;
use keccak_hasher::KeccakHasher;
use trie_db::DBValue;

type HashDBOracle = dyn HashDB<KeccakHasher, DBValue>;

pub fn state_transition(
    db: impl HashDB<KeccakHasher, DBValue> + Clone + 'static, hash: H256,
) -> H256 {
    let input = RollupInput::load_from_hashdb(&db, hash);
    let mut prev = input.prev_header;
    let batches = input.batches;

    let mut latest_hashes = load_last_hashes(&db, prev.hash(), prev.number());
    let machine = machine::create_l2_machine();
    let mut engine = L2Seal::new(0, machine);
    for batch in batches {
        let db_clone = Box::new(db.clone());
        engine.set_timestamp(batch.timestamp);

        let info = BlockGenInfo::new(
            prev,
            Arc::new(latest_hashes.clone()),
            L2_FEE_COLLECTOR,
            (L2_BLOCK_MIN_GAS_LIMIT.into(), L2_BLOCK_MAX_GAS_LIMIT.into()),
            Vec::new(),
        );
        if let Some(block) = generate_block(db_clone, &engine, &info, batch.transactions) {
            prev = block.header.clone();
        } else {
            prev = info.parent_block_header;
        }
    }

    prev.hash()
}
