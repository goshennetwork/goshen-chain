#![no_std]
#![no_main]

extern crate riscv_evm;
extern crate alloc;

use alloc::format;
use core::str::FromStr;
use ethereum_types::H256;
use rustc_hex::FromHex;

use common_types::bytes::ToPretty;
use riscv_l2chain::riscv_db::RiscvDB;
use hash_db::HashDB;
use riscv_l2chain::{state_transition, state_transition_to_header};

#[no_mangle]
pub extern "C" fn _start() {
    riscv_evm::runtime::debug("start");
    let mut db = RiscvDB::new(&[]);
    let hash = H256::from(riscv_evm::runtime::input());
    let header = state_transition_to_header(db, hash);
    riscv_evm::runtime::ret(header.hash().0);
}
