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

//! Ethereum transaction

use common_types::l2_cfg::INTRINSIC_GAS_FACTOR;
use evm::Schedule;
use types::l2_cfg::TX_BASE_SIZE;
use types::transaction::{self, Action};

/// Extends transaction with gas verification method.
pub trait Transaction {
    /// Get the transaction cost in gas for this transaction.
    fn gas_required(&self, schedule: &Schedule) -> u64;
}

impl Transaction for transaction::Transaction {
    fn gas_required(&self, schedule: &Schedule) -> u64 {
        gas_required_for(
            match self.action {
                Action::Create => true,
                Action::Call(_) => false,
            },
            &self.data,
            schedule,
        )
    }
}

/// Get the transaction cost in gas for the given params.
fn gas_required_for(is_create: bool, data: &[u8], schedule: &Schedule) -> u64 {
    data.iter().fold(
        (TX_BASE_SIZE * schedule.tx_data_non_zero_gas * INTRINSIC_GAS_FACTOR
            + if is_create { schedule.tx_create_gas } else { schedule.tx_gas }) as u64,
        |g, b| {
            g + (match *b {
                0 => schedule.tx_data_zero_gas * INTRINSIC_GAS_FACTOR,
                _ => schedule.tx_data_non_zero_gas * INTRINSIC_GAS_FACTOR,
            }) as u64
        },
    )
}
