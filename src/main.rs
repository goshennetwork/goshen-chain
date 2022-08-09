#![no_std]
#![no_main]

extern crate riscv_evm;

use core::str::FromStr;
use ethereum_types::H256;
use rustc_hex::FromHex;

use common_types::bytes::ToPretty;
use ethcore::state::riscv_db::RiscvDB;
use hash_db::HashDB;
use riscv_l2chain::state_transition_to_header;

#[no_mangle]
pub extern "C" fn _start() {
    riscv_evm::runtime::debug("start");
    let mut db = RiscvDB::new(&[]);
    // let contents = fs::read_to_string("./batch.data").unwrap();
    // for line in contents.lines() {
    //     db.insert(&line.from_hex().unwrap());
    // }
    let hash = H256::from_str("eb611f33cdb62869b3c051d5c00021cc7e407095af80b50645b7ddc33aa92541").unwrap();
    let header = state_transition_to_header(db, hash);
    // println!("0x{}", header.hash().to_hex());
    // riscv_evm::runtime::debug(header.hash().to_hex().as_str());
    riscv_evm::runtime::ret(header.hash().0);
}
