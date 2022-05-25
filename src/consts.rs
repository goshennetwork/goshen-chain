use core::ops::Add;
use ethereum_types::{Address, H160};

pub const L1_CROSS_LAYER_WITNESS: Address =
    H160([0x58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
pub const L2_FEE_COLLECTOR: Address =
    H160([0x58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);

pub const L2_BLOCK_MAX_GAS_LIMIT: u32 = 20000000;
pub const L2_BLOCK_MIN_GAS_LIMIT: u32 = 10000000;

pub const L2_CHAIN_ID: u64 = 1234;
// same as ethcore::spec::spec::MAX_TRANSACTION
pub const L2_MAX_TRANSACTION_SIZE: usize = 300 * 1024;
