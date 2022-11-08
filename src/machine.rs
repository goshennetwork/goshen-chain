use crate::consts::{L2_BLOCK_MIN_GAS_LIMIT, L2_CHAIN_ID, L2_MAX_TRANSACTION_SIZE};
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use common_types::BlockNumber;
use ethcore::machine::EthereumMachine;
use ethcore::spec::CommonParams;
use ethcore_builtin::{
    AltBn128ConstOperations, AltBn128PairingPrice, AltBn128PairingPricer, Blake2F, Blake2FPricer, Bn128Add, Bn128Mul, Bn128Pairing, Builtin, EcRecover, EthereumBuiltin, Identity, Linear, Modexp, Modexp2565Pricer, ModexpPricer, Pricing, Ripemd160, Sha256
};
use ethereum_types::{Address, H160, U256};

pub fn create_l2_machine() -> EthereumMachine {
    let params = create_params();
    let builtins = create_builtins();
    EthereumMachine::regular(params, builtins)
}

fn create_builtins() -> BTreeMap<Address, Builtin> {
    let mut address = [0u8; 20];
    let mut map = BTreeMap::new();
    address[19] = 1;
    map.insert(
        H160(address),
        Builtin {
            pricer: BTreeMap::from([(0, Pricing::Linear(Linear { base: 3000, word: 0 }))]),
            native: EthereumBuiltin::EcRecover(EcRecover),
        },
    );
    address[19] = 2;
    map.insert(
        H160(address),
        Builtin {
            pricer: BTreeMap::from([(0, Pricing::Linear(Linear { base: 60, word: 12 }))]),
            native: EthereumBuiltin::Sha256(Sha256),
        },
    );

    address[19] = 3;
    map.insert(
        H160(address),
        Builtin {
            pricer: BTreeMap::from([(0, Pricing::Linear(Linear { base: 600, word: 120 }))]),
            native: EthereumBuiltin::Ripemd160(Ripemd160),
        },
    );
    address[19] = 4;
    map.insert(
        H160(address),
        Builtin {
            pricer: BTreeMap::from([(0, Pricing::Linear(Linear { base: 15, word: 3 }))]),
            native: EthereumBuiltin::Identity(Identity),
        },
    );

    address[19] = 5;
    map.insert(
        H160(address),
        Builtin {
            pricer: BTreeMap::from([(0, Pricing::Modexp(ModexpPricer { divisor: 20 }))]),
            native: EthereumBuiltin::Modexp(Modexp),
        },
    );
    address[19] = 6;
    map.insert(
        H160(address),
        Builtin {
            pricer: BTreeMap::from([(
                0,
                Pricing::AltBn128ConstOperations(AltBn128ConstOperations { price: 150 }),
            )]),
            native: EthereumBuiltin::Bn128Add(Bn128Add),
        },
    );
    address[19] = 7;
    map.insert(
        H160(address),
        Builtin {
            pricer: BTreeMap::from([(
                0,
                Pricing::AltBn128ConstOperations(AltBn128ConstOperations { price: 6000 }),
            )]),
            native: EthereumBuiltin::Bn128Mul(Bn128Mul),
        },
    );
    address[19] = 8;
    map.insert(
        H160(address),
        Builtin {
            pricer: BTreeMap::from([(
                0,
                Pricing::AltBn128Pairing(AltBn128PairingPricer {
                    price: AltBn128PairingPrice { base: 45000, pair: 34000 },
                }),
            )]),
            native: EthereumBuiltin::Bn128Pairing(Bn128Pairing),
        },
    );

    address[19] = 9;
    map.insert(
        H160(address),
        Builtin {
            pricer: BTreeMap::from([(0, Pricing::Blake2F(1))]),
            native: EthereumBuiltin::Blake2F(Blake2F),
        },
    );

    map
}

fn create_params() -> CommonParams {
    CommonParams {
        account_start_nonce: U256::zero(),
        maximum_extra_data_size: 0x20,
        network_id: L2_CHAIN_ID,
        chain_id: L2_CHAIN_ID,
        subprotocol_name: "eth".to_string(),
        min_gas_limit: L2_BLOCK_MIN_GAS_LIMIT.into(),
        fork_block: None,
        eip150_transition: 0u32.into(),
        eip160_transition: 0u32.into(),
        eip161abc_transition: 0u32.into(),
        eip161d_transition: 0u32.into(),
        eip155_transition: 0u32.into(),
        validate_receipts_transition: 0u32.into(),
        validate_chain_id_transition: 0u32.into(),
        eip98_transition: BlockNumber::MAX.into(),
        eip140_transition: 0u32.into(),
        eip211_transition: 0u32.into(),
        eip214_transition: 0u32.into(),
        eip145_transition: 0u32.into(),
        eip658_transition: 0u32.into(),
        eip1014_transition: 0u32.into(),
        eip1052_transition: 0u32.into(),
        eip1283_transition: 0u32.into(),
        eip1283_disable_transition: BlockNumber::MAX.into(),
        eip1283_reenable_transition: BlockNumber::MAX.into(),
        eip1344_transition: 0u32.into(),
        eip1706_transition: 0u32.into(),
        eip1884_transition: 0u32.into(),
        eip2028_transition: 0u32.into(),
        eip2315_transition: BlockNumber::MAX.into(),
        eip2929_transition: BlockNumber::MAX.into(),
        eip2930_transition: 0u32.into(),
        eip3198_transition: BlockNumber::MAX.into(),
        eip3529_transition: BlockNumber::MAX.into(),
        eip3541_transition: BlockNumber::MAX.into(),
        dust_protection_transition: BlockNumber::MAX.into(),
        eip3607_transition: 0u32.into(),
        nonce_cap_increment: 64u32.into(),
        remove_dust_contracts: false,
        gas_limit_bound_divisor: 0x400u32.into(),
        registrar: Address::default(),
        max_code_size: 0x6000, // 24k
        max_transaction_size: L2_MAX_TRANSACTION_SIZE,
        max_code_size_transition: 0u32.into(),
        kip4_transition: BlockNumber::MAX.into(),
        kip6_transition: BlockNumber::MAX.into(),
        eip1559_transition: BlockNumber::MAX.into(), // eip1559 disabled
        eip1559_base_fee_max_change_denominator: None,
        eip1559_elasticity_multiplier: 0x2u32.into(),
        eip1559_base_fee_initial_value: 0x3B9ACA00u32.into(), // 1Gwei
        eip1559_base_fee_min_value: None,
        eip1559_base_fee_min_value_transition: BlockNumber::MAX.into(),
        eip1559_fee_collector: None,
        eip1559_fee_collector_transition: BlockNumber::MAX.into(),
    }
}
