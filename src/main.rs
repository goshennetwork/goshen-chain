#![no_std]

use ethereum_types::H256;
use hex::FromHex;

use common_types::bytes::ToPretty;
use ethcore::state::riscv_db::RiscvDB;
use hash_db::HashDB;
use riscv_l2chain::state_transition_to_header;

fn main() {
    let mut db = RiscvDB::new(&[]);
    // let contents = fs::read_to_string("./batch.data").unwrap();
    // for line in contents.lines() {
    //     db.insert(&line.from_hex().unwrap());
    // }
    let buffer = <[u8; 32]>::from_hex("eb611f33cdb62869b3c051d5c00021cc7e407095af80b50645b7ddc33aa92541")?;
    let hash = H256::from(buffer);
    let header = state_transition_to_header(db, hash);
    println!("0x{}", header.hash().to_hex());
}
