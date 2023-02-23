use crate::syscall;
use alloc::vec::Vec;
const RUNTIME_INPUT: usize = 0;
const RUNTIME_RETURN: usize = 1;
const RUNTIME_PREIMAGE_LEN: usize = 2;
const RUNTIME_PREIMAGE: usize = 3;
const RUNTIME_PANIC: usize = 4;
const RUNTIME_DEBUG: usize = 5;
const RUNTIME_ECRECOVER: usize = 6;
const RUNTIME_BLOB: usize = 7;

#[cfg(target_arch = "riscv32")]
#[inline]
unsafe fn jump(ptr: u32) {
    core::arch::asm!( "jalr ra, {x}, 0", x= in(reg) ptr );
}

pub fn debug(msg: &str) {
    let ptr = msg.as_ptr() as usize;
    let len = msg.len() as usize;
    unsafe {
        syscall::syscall2_readonly(RUNTIME_DEBUG, ptr, len);
    }
}

pub fn input() -> [u8; 32] {
    let mut hash: [u8; 32] = [0; 32];
    unsafe {
        syscall::syscall1(RUNTIME_INPUT, hash.as_mut_ptr() as usize);
    }
    hash
}

pub fn ret(state: [u8; 32]) -> ! {
    unsafe { syscall::syscall1_noreturn(RUNTIME_RETURN, state.as_ptr() as usize) }
}

pub fn panic(msg: &str) -> ! {
    let ptr = msg.as_ptr() as usize;
    let len = msg.len() as usize;
    unsafe { syscall::syscall2_noreturn(RUNTIME_PANIC, ptr, len) }
}

pub fn preimage_at(hash: [u8; 32], offset: usize) -> u32 {
    let ptr = hash.as_ptr() as usize;
    unsafe { syscall::syscall2_readonly(RUNTIME_PREIMAGE, ptr, offset) as u32 }
}

pub fn preimage_len(hash: [u8; 32]) -> usize {
    let ptr = hash.as_ptr() as usize;
    unsafe { syscall::syscall1_readonly(RUNTIME_PREIMAGE_LEN, ptr) }
}

pub fn preimage(hash: [u8; 32]) -> Vec<u8> {
    let len = preimage_len(hash);
    let mut result = Vec::with_capacity((len + 3) / 4 * 4);
    (0..len).step_by(4).for_each(|offset| {
        let value = preimage_at(hash, offset).to_le_bytes();
        result.extend_from_slice(value.as_slice());
    });
    result.resize(len, 0);

    return result;
}

pub fn blob_at(hash: [u8;32], index: usize) -> [u8;32] {
	let ptr = hash.as_ptr() as usize;
	let mut output: [u8; 32] = [0; 32];
	unsafe { syscall::syscall3(RUNTIME_BLOB, ptr, index, output.as_mut_ptr() as usize); }

	output
}

pub fn ecrecover(hash: [u8; 32], r: [u8; 32], s: [u8; 32], v: usize) -> [u8; 20] {
    let mut addr: [u8; 20] = [0; 20];
    let addr_ptr = addr.as_mut_ptr() as usize;
    let hash_ptr = hash.as_ptr() as usize;
    let r_ptr = r.as_ptr() as usize;
    let s_ptr = s.as_ptr() as usize;
    unsafe {
        syscall::syscall5(RUNTIME_ECRECOVER, addr_ptr, hash_ptr, r_ptr, s_ptr, v);
    }
    addr
}
