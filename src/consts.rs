use core::ops::Add;
use ethereum_types::{Address, H160};

pub const L1_CROSS_LAYER_WITNESS: Address =
    H160([0x79, 0x69, 0xc5, 0xed, 0x33, 0x56, 0x50, 0x69, 0x2b, 0xc0, 0x42, 0x93, 0xb0, 0x7f, 0x5b, 0xf2, 0xe7, 0xa6, 0x73, 0xc0]);
pub const L2_FEE_COLLECTOR: Address =
    H160([0xfe, 0xe0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x0f, 0xee]);

pub const L2_BLOCK_MAX_GAS_LIMIT: u32 = 20000000;
pub const L2_BLOCK_MIN_GAS_LIMIT: u32 = 10000000;

pub const L2_CHAIN_ID: u64 = 21772;
// same as ethcore::spec::spec::MAX_TRANSACTION
pub const L2_MAX_TRANSACTION_SIZE: usize = 300 * 1024;
