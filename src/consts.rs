use ethereum_types::{Address, H160};

pub const L2_CROSS_LAYER_WITNESS: Address =
    H160([0x22, 0x10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x02, 0x21]);
pub const L2_FEE_COLLECTOR: Address =
    H160([0xfe, 0xe0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x0f, 0xee]);

pub const L2_BLOCK_MAX_GAS_LIMIT: u32 = 0x3938700;
pub const L2_BLOCK_MIN_GAS_LIMIT: u32 = 0x3938700;

pub const L2_CHAIN_ID: u64 = 21772;
// same as ethcore::spec::spec::MAX_TRANSACTION
pub const L2_MAX_TRANSACTION_SIZE: usize = 300 * 1024;
