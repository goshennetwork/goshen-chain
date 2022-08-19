use std::fs;
use std::str::FromStr;

use ethereum_types::H256;
use rustc_hex::FromHex;

use common_types::bytes::ToPretty;
use ethcore::state::backend::ProofCheck;
use hash_db::HashDB;
use riscv_l2chain::state_transition;

fn main() {
    let contents = fs::read_to_string("./batch.data").unwrap();
    let mut db = ProofCheck::new(&[]);
    for line in contents.lines() {
        let data: Vec<u8> = line.from_hex().unwrap();
        db.insert(&data);
    }
    let hash =
        H256::from_str("6929f9b018ab65ae845cefba93f41474d6e3a849c5cfb3ca7ab365832108842f").unwrap();
    let header = state_transition(db, hash);
    println!("0x{}", header.to_hex());
}
