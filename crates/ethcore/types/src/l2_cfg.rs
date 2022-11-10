use ethereum_types::{Address, H160};

pub const MAX_TX_EXEC_GAS: u64 = 20000000; // limit the execution gas: tx.Gas - tx.IntrinsicGas
pub const TX_BASE_SIZE: usize = 213;
pub const INTRINSIC_GAS_FACTOR: usize = 100;
pub const INITIAL_ENQUEUE_TX_NONCE: u64 = 1 << 63;
pub const MAX_SENDER_NONCE: u64 = 1 << 62;
pub const L2_CHAIN_ID: u64 = 21772;
// same as ethcore::spec::spec::MAX_TRANSACTION
pub const L2_MAX_TRANSACTION_SIZE: usize = 32 * 1024;

pub const L2_CROSS_LAYER_WITNESS: Address =
    H160([0x22, 0x10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x02, 0x21]);
pub const L1_CROSS_LAYER_WITNESS: Address = H160([
    0x7E, 0x5F, 0x45, 0x52, 0x09, 0x1A, 0x69, 0x12, 0x5d, 0x5D, 0xfC, 0xb7, 0xb8, 0xC2, 0x65, 0x90,
    0x29, 0x39, 0x5B, 0xdf,
]);
pub const L2_FEE_COLLECTOR: Address =
    H160([0xfe, 0xe0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x0f, 0xee]);

pub const L2_BLOCK_MAX_GAS_LIMIT: u32 = 0x3938700;
pub const L2_BLOCK_MIN_GAS_LIMIT: u32 = 0x3938700;
