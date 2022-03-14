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

use journaldb;
use verification::{VerifierType};

pub use blockchain::Config as BlockChainConfig;
pub use evm::VMType;
pub use std::time::Duration;
pub use trace::Config as TraceConfig;

/// Client configuration. Includes configs for all sub-systems.
#[derive(Debug, PartialEq, Clone)]
pub struct ClientConfig {
    /// Blockchain configuration.
    pub blockchain: BlockChainConfig,
    /// Trace configuration.
    pub tracing: TraceConfig,
    /// VM type.
    pub vm_type: VMType,
    /// Fat DB enabled?
    pub fat_db: bool,
    /// The JournalDB ("pruning") algorithm to use.
    pub pruning: journaldb::Algorithm,
    /// The name of the client instance.
    pub name: String,
    /// The chain spec name
    pub spec_name: String,
    /// Type of block verifier used by client.
    pub verifier_type: VerifierType,
    /// State db cache-size.
    pub state_cache_size: usize,
    /// EVM jump-tables cache size.
    pub jump_table_size: usize,
    /// Minimum state pruning history size.
    pub history: u64,
    /// Ideal memory usage for state pruning history.
    pub history_mem: usize,
    /// Check seal valididity on block import
    pub check_seal: bool,
    /// Maximal number of transactions queued for verification in a separate thread.
    pub transaction_verification_queue_size: usize,
    /// Maximal number of blocks to import at each round.
    pub max_round_blocks_to_import: usize,
}

impl Default for ClientConfig {
    fn default() -> Self {
        let mb = 1024 * 1024;
        ClientConfig {
            blockchain: Default::default(),
            tracing: Default::default(),
            vm_type: Default::default(),
            fat_db: false,
            pruning: journaldb::Algorithm::OverlayRecent,
            name: "default".into(),
            spec_name: "".into(),
            verifier_type: VerifierType::Canon,
            state_cache_size: 1 * mb,
            jump_table_size: 1 * mb,
            history: 64,
            history_mem: 32 * mb,
            check_seal: true,
            transaction_verification_queue_size: 8192,
            max_round_blocks_to_import: 1,
        }
    }
}
