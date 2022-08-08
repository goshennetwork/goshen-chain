use alloc::sync::Arc;
use alloc::vec::Vec;
use core::cell::RefCell;

use ethereum_types::{Address, H256};
use hashbrown::HashSet;
use riscv_evm::runtime::{ecrecover, ret};

use hash_db::{AsHashDB, HashDB};
use keccak_hasher::KeccakHasher;
use memory_db::MemoryDB;
use trie::DBValue;

use crate::state::{Account, Backend};
use crate::state::backend::ProofCheck;

#[derive(Clone, PartialEq)]
pub struct RiscvDB(ProofCheck);

impl RiscvDB {
    pub fn new(proof: &[DBValue]) -> Self {
        return RiscvDB(ProofCheck::new(proof));
    }
}

impl HashDB<KeccakHasher, DBValue> for RiscvDB {
    fn get(&self, key: &H256) -> Option<DBValue> {
        if let Some(value) = self.0.get(key) {
            return Some(value);
        } else {
            let value = riscv_evm::runtime::preimage(key.0);
            return Some(DBValue::from_slice(value.as_slice()));
        }
    }

    fn contains(&self, key: &H256) -> bool {
        self.0.contains(key)
    }

    fn insert(&mut self, value: &[u8]) -> H256 {
        self.0.insert(value)
    }

    fn emplace(&mut self, key: H256, value: DBValue) {
        self.0.emplace(key, value)
    }

    fn remove(&mut self, _key: &H256) {}
}

impl AsHashDB<KeccakHasher, DBValue> for RiscvDB {
    fn as_hash_db(&self) -> &dyn HashDB<KeccakHasher, DBValue> {
        self
    }
    fn as_hash_db_mut(&mut self) -> &mut dyn HashDB<KeccakHasher, DBValue> {
        self
    }
}

impl Backend for RiscvDB {
    fn as_hash_db(&self) -> &dyn HashDB<KeccakHasher, DBValue> {
        self
    }
    fn as_hash_db_mut(&mut self) -> &mut dyn HashDB<KeccakHasher, DBValue> {
        self
    }
    fn add_to_account_cache(&mut self, _addr: Address, _data: Option<Account>, _modified: bool) {}
    fn cache_code(&self, _hash: H256, _code: Arc<Vec<u8>>) {}
    fn get_cached_account(&self, _addr: &Address) -> Option<Option<Account>> {
        None
    }
    fn get_cached<F, U>(&self, _a: &Address, _f: F) -> Option<U>
        where
            F: FnOnce(Option<&mut Account>) -> U,
    {
        None
    }
    fn get_cached_code(&self, _hash: &H256) -> Option<Arc<Vec<u8>>> {
        None
    }
}