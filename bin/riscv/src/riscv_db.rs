use alloc::sync::Arc;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::hash::Hash;

use common_types::hash::{keccak, keccak256};
use ethereum_types::{Address, H256};
use hashbrown::HashSet;

use hash_db::{AsHashDB, HashDB};
use keccak_hasher::KeccakHasher;
use memory_db::MemoryDB;
use trie_db::DBValue;

use ethcore::state::backend::ProofCheck;
use ethcore::state::{Account, Backend};

#[derive(Clone, PartialEq)]
pub struct RiscvDB(ProofCheck);

impl RiscvDB {
    pub fn new(proof: &[DBValue]) -> Self {
        return RiscvDB(ProofCheck::new(proof));
    }
}

impl HashDB<KeccakHasher, DBValue> for RiscvDB {
    fn get(&self, key: &H256) -> Option<DBValue> {
        let value = riscv_evm::runtime::preimage(key.0);
        return Some(DBValue::from_slice(value.as_slice()));
    }

    fn contains(&self, key: &H256) -> bool {
        match self.get(key) {
            Some(t) => true,
            None => false,
        }
    }

    fn insert(&mut self, value: &[u8]) -> H256 {
        keccak(value)
    }

    fn emplace(&mut self, key: H256, value: DBValue) {}

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
