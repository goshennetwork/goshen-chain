use std::fs;
use std::str::FromStr;

use ethereum_types::H256;
use rustc_hex::FromHex;

use common_types::bytes::ToPretty;
use ethcore::state::backend::ProofCheck;
use hash_db::HashDB;
use riscv_l2chain::state_transition_to_header;

fn main() {
    let contents = fs::read_to_string("./batch.data").unwrap();
    let mut db = ProofCheck::new(&[]);
    for line in contents.lines() {
        db.insert(&line.from_hex().unwrap());
    }
    // batch: 300
    // headers: [6203, 6320]
    // prevHeaders: [5947, 6202]
    // queue: 3405, 67
    // proofs: [6202, 6320]
    let hash = H256::from_str("9d74fc5445995e09d0f9c3f057a1354c51b2fb3cf2384877b03c4788a21896c8").unwrap();
    let header = state_transition_to_header(db, hash);
    println!("{}", header.hash().to_hex());
}
