#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::format;
use alloc::string::ToString;
use alloc::sync::Arc;
use alloc::vec::Vec;

use common_types::bytes::ToPretty;
use ethereum_types::H256;

use common_types::header::Header;
use common_types::l2_cfg::{
    L2_BLOCK_MAX_GAS_LIMIT, L2_BLOCK_MIN_GAS_LIMIT, L2_CHAIN_ID, L2_CROSS_LAYER_WITNESS, L2_FEE_COLLECTOR
};
use ethcore::engines::L2Seal;
use ethcore::miner::{generate_block, BlockGenInfo};
use hash_db::HashDB;
use input::RollupInput;
use keccak_hasher::KeccakHasher;
use trie_db::DBValue;

use crate::input::load_last_hashes;

mod input;
mod machine;

type HashDBOracle = dyn HashDB<KeccakHasher, DBValue>;

pub fn state_transition(
    db: impl HashDB<KeccakHasher, DBValue> + Clone + 'static, entry_hash: H256,
) -> H256 {
    state_transition_to_header(db, entry_hash).hash()
}

pub fn state_transition_to_header(
    db: impl HashDB<KeccakHasher, DBValue> + Clone + 'static, entry_hash: H256,
) -> Header {
    let input = RollupInput::load_from_hashdb(&db, entry_hash);
    let mut prev = input.prev_header;
    let batches = input.batches;

    let latest_hashes = load_last_hashes(&db, prev.hash(), prev.number());
    let machine = machine::create_l2_machine();
    let mut engine = L2Seal::new(0, machine);
    for mut batch in batches {
        let db_clone = Box::new(db.clone());
        engine.set_timestamp(batch.timestamp);

        // remove tx that's chain id is incorrectly
        batch.transactions.retain(|t| t.chain_id.unwrap_or_default() == L2_CHAIN_ID);

        let info = BlockGenInfo::new(
            prev,
            Arc::new(latest_hashes.clone()),
            L2_FEE_COLLECTOR,
            (L2_BLOCK_MIN_GAS_LIMIT.into(), L2_BLOCK_MAX_GAS_LIMIT.into()),
            Vec::new(),
        );
        if let Some(block) =
            generate_block(db_clone, &engine, &info, batch.transactions, L2_CROSS_LAYER_WITNESS)
        {
            prev = block.header.clone();
        } else {
            prev = info.parent_block_header;
        }
    }

    prev
}
