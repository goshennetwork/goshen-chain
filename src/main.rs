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
use riscv_l2chain::state_transition;

#[no_mangle]
pub extern "C" fn _start() {
    // riscv_evm::runtime::debug("start");
    // let mut db = RiscvDB::new(&[]);
    // #[cfg(feature = "riscv")]
    // riscv_evm::runtime::debug("start read input");
    // let hash = H256::from(riscv_evm::runtime::input());
    // #[cfg(feature = "riscv")]
    // riscv_evm::runtime::debug(format!("input is 0x{}", hash.to_hex()).as_str());
    // let header = state_transition(db, hash);
    // riscv_evm::runtime::ret(header.0);

    riscv_evm::runtime::debug("start");
    let mut hash_map = hashbrown::HashMap::new();
    riscv_evm::runtime::debug("new");
    hash_map.insert("19", 10);
    riscv_evm::runtime::debug("insert");
    riscv_evm::runtime::ret(H256::zero().0);
}
