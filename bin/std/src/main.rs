use std::fs;
use std::str::FromStr;

use ethereum_types::H256;
use rustc_hex::FromHex;

use common_types::bytes::ToPretty;
use ethcore::state::backend::ProofCheck;
use goshen_chain::state_transition;
use hash_db::HashDB;

fn main() {
    let contents = fs::read_to_string("./batch.data").unwrap();
    let mut db = ProofCheck::new(&[]);
    for line in contents.lines() {
        let data: Vec<u8> = line.from_hex().unwrap();
        db.insert(&data);
    }
    let hash =
        H256::from_str("2a6b1be06ab62c350d6f42c16653280f6ffbda39ef3cc7fb64b478c329e45742").unwrap();
    let header = state_transition(db, hash);
    println!("0x{}", header.to_hex());
}
