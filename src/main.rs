use std::fs;
use std::str::FromStr;
use std::sync::Arc;

use ethereum_types::H256;
use rustc_hex::FromHex;

use common_types::bytes::ToPretty;
use ethcore::state::backend::ProofCheck;
use hash_db::HashDB;
use keccak_hasher::KeccakHasher;
use riscv_l2chain::state_transition_to_header;

fn main() {
	let contents = fs::read_to_string("./batch.data").unwrap();
	let mut db = ProofCheck::new(&[]);
	for line in contents.lines() {
		db.insert(&line.from_hex().unwrap());
	}
	let hash = H256::from_str("221ad9e357799236814b0e2d1539e30eb816bba2450cbda91f91a64a187f1dc1").unwrap();
	let header = state_transition_to_header(db, hash);
	println!("{}", header.hash().to_hex());
}
