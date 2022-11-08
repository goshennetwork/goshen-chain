#![no_std]
#![no_main]

#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

mod riscv_db;

extern crate alloc;
extern crate riscv_evm;

use alloc::format;
use core::str::FromStr;
use ethereum_types::H256;
use rustc_hex::FromHex;

use common_types::bytes::ToPretty;
use goshen_chain::{state_transition, state_transition_to_header};
use hash_db::HashDB;
use riscv_db::RiscvDB;

#[no_mangle]
pub extern "C" fn _start() {
    riscv_evm::runtime::debug("start");
    let mut db = RiscvDB::new(&[]);
    let hash = H256::from(riscv_evm::runtime::input());
    let header = state_transition_to_header(db, hash);
    riscv_evm::runtime::ret(header.hash().0);
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	let msg = info.message().map(|msg| format!("{}", msg)).unwrap_or_default();
	let (file, line) =
		if let Some(loc) = info.location() { (loc.file(), loc.line()) } else { ("", 0) };

	let panic_msg = format!("{} at {}:{}", msg, file, line);
	riscv_evm::runtime::panic(&panic_msg)
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
	panic!("memory allocation of {} bytes failed", layout.size())
}

use buddy_alloc::{BuddyAllocParam, FastAllocParam, NonThreadsafeAlloc};

const FAST_HEAP_SIZE: usize = 512 * 1024 * 1024; // 512M
const HEAP_SIZE: usize = 1*1024 * 1024 * 1024; // 2G
const LEAF_SIZE: usize = 16;

pub static mut FAST_HEAP: [u8; FAST_HEAP_SIZE] = [0u8; FAST_HEAP_SIZE];
pub static mut HEAP: [u8; HEAP_SIZE] = [0u8; HEAP_SIZE];

// This allocator can't work in tests since it's non-threadsafe.
#[global_allocator]
static HEAP_ALLOCATOR: NonThreadsafeAlloc = unsafe {
	let fast_param = FastAllocParam::new(FAST_HEAP.as_ptr(), FAST_HEAP_SIZE);
	let buddy_param = BuddyAllocParam::new(HEAP.as_ptr(), HEAP_SIZE, LEAF_SIZE);
	NonThreadsafeAlloc::new(fast_param, buddy_param)
};
