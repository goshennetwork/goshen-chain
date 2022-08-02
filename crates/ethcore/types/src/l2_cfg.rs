use ethereum_types::{Address, H160};

pub const INTRINSIC_GAS_FACTOR: usize = 100;
pub const INITIAL_ENQUEUE_TX_NONCE: u64 = 1 << 63;
pub const L1_CROSS_LAYER_WITNESS: Address =
    H160([0x79, 0x69, 0xc5, 0xed, 0x33, 0x56, 0x50, 0x69, 0x2b, 0xc0, 0x42, 0x93, 0xb0, 0x7f, 0x5b, 0xf2, 0xe7, 0xa6, 0x73, 0xc0]);