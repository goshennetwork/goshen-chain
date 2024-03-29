// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of OpenEthereum.

// OpenEthereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// OpenEthereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with OpenEthereum.  If not, see <http://www.gnu.org/licenses/>.

//! Set of different helpers for client tests

use ethereum_types::U256;
use evm::Factory as EvmFactory;

use crate::factory::Factories;
use crate::state::*;
use crate::state_db::StateDB;

/// Returns temp state
pub fn get_temp_state() -> State<StateDB> {
    let journal_db = get_temp_state_db();
    State::new(journal_db, U256::from(0u32), Default::default())
}

/// Returns temp state using coresponding factory
pub fn get_temp_state_with_factory(factory: EvmFactory) -> State<StateDB> {
    let journal_db = get_temp_state_db();
    let mut factories = Factories::default();
    factories.vm = factory.into();
    State::new(journal_db, U256::from(0u32), factories)
}

/// Returns temp state db
pub fn get_temp_state_db() -> StateDB {
    let hashdb = Box::new(memory_db::MemoryDB::from_null_node(
        &rlp::NULL_RLP,
        rlp::NULL_RLP.as_ref().into(),
    ));
    StateDB::new(hashdb, 5 * 1024 * 1024)
}
