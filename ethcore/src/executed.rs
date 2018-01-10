// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Transaction execution format module.

use ethereum_types::{U256, U512, Address};
use bytes::Bytes;
use trie;
use vm;
use trace::{VMTrace, FlatTrace};
use log_entry::LogEntry;
use state_diff::StateDiff;

use std::fmt;

/// Transaction execution receipt.
#[derive(Debug, PartialEq, Clone)]
pub struct Executed<T = FlatTrace, V = VMTrace> {
	/// True if the outer call/create resulted in an exceptional exit.
	pub exception: Option<vm::Error>,

	/// Gas paid up front for execution of transaction.
	pub gas: U256,

	/// Gas used during execution of transaction.
	pub gas_used: U256,

	/// Gas refunded after the execution of transaction.
	/// To get gas that was required up front, add `refunded` and `gas_used`.
	pub refunded: U256,

	/// Cumulative gas used in current block so far.
	///
	/// `cumulative_gas_used = gas_used(t0) + gas_used(t1) + ... gas_used(tn)`
	///
	/// where `tn` is current transaction.
	pub cumulative_gas_used: U256,

	/// Vector of logs generated by transaction.
	pub logs: Vec<LogEntry>,

	/// Addresses of contracts created during execution of transaction.
	/// Ordered from earliest creation.
	///
	/// eg. sender creates contract A and A in constructor creates contract B
	///
	/// B creation ends first, and it will be the first element of the vector.
	pub contracts_created: Vec<Address>,
	/// Transaction output.
	pub output: Bytes,
	/// The trace of this transaction.
	pub trace: Vec<T>,
	/// The VM trace of this transaction.
	pub vm_trace: Option<V>,
	/// The state diff, if we traced it.
	pub state_diff: Option<StateDiff>,
}

/// Result of executing the transaction.
#[derive(PartialEq, Debug, Clone)]
pub enum ExecutionError {
	/// Returned when there gas paid for transaction execution is
	/// lower than base gas required.
	NotEnoughBaseGas {
		/// Absolute minimum gas required.
		required: U256,
		/// Gas provided.
		got: U256
	},
	/// Returned when block (gas_used + gas) > gas_limit.
	///
	/// If gas =< gas_limit, upstream may try to execute the transaction
	/// in next block.
	BlockGasLimitReached {
		/// Gas limit of block for transaction.
		gas_limit: U256,
		/// Gas used in block prior to transaction.
		gas_used: U256,
		/// Amount of gas in block.
		gas: U256
	},
	/// Returned when transaction nonce does not match state nonce.
	InvalidNonce {
		/// Nonce expected.
		expected: U256,
		/// Nonce found.
		got: U256
	},
	/// Returned when cost of transaction (value + gas_price * gas) exceeds
	/// current sender balance.
	NotEnoughCash {
		/// Minimum required balance.
		required: U512,
		/// Actual balance.
		got: U512
	},
	/// When execution tries to modify the state in static context
	MutableCallInStaticContext,
	/// Returned when transacting from a non-existing account with dust protection enabled.
	SenderMustExist,
	/// Returned when internal evm error occurs.
	Internal(String),
	/// Returned when generic transaction occurs
	TransactionMalformed(String),
}

impl From<Box<trie::TrieError>> for ExecutionError {
	fn from(err: Box<trie::TrieError>) -> Self {
		ExecutionError::Internal(format!("{}", err))
	}
}

impl fmt::Display for ExecutionError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::ExecutionError::*;

		let msg = match *self {
			NotEnoughBaseGas { ref required, ref got } =>
				format!("Not enough base gas. {} is required, but only {} paid", required, got),
			BlockGasLimitReached { ref gas_limit, ref gas_used, ref gas } =>
				format!("Block gas limit reached. The limit is {}, {} has \
					already been used, and {} more is required", gas_limit, gas_used, gas),
			InvalidNonce { ref expected, ref got } =>
				format!("Invalid transaction nonce: expected {}, found {}", expected, got),
			NotEnoughCash { ref required, ref got } =>
				format!("Cost of transaction exceeds sender balance. {} is required \
					but the sender only has {}", required, got),
			MutableCallInStaticContext => "Mutable Call in static context".to_owned(),
			SenderMustExist => "Transacting from an empty account".to_owned(),
			Internal(ref msg) => msg.clone(),
			TransactionMalformed(ref err) => format!("Malformed transaction: {}", err),
		};

		f.write_fmt(format_args!("Transaction execution error ({}).", msg))
	}
}

/// Result of executing the transaction.
#[derive(PartialEq, Debug, Clone)]
pub enum CallError {
	/// Couldn't find the transaction in the chain.
	TransactionNotFound,
	/// Couldn't find requested block's state in the chain.
	StatePruned,
	/// Couldn't find an amount of gas that didn't result in an exception.
	Exceptional,
	/// Corrupt state.
	StateCorrupt,
	/// Error executing.
	Execution(ExecutionError),
}

impl From<ExecutionError> for CallError {
	fn from(error: ExecutionError) -> Self {
		CallError::Execution(error)
	}
}

impl fmt::Display for CallError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::CallError::*;

		let msg = match *self {
			TransactionNotFound => "Transaction couldn't be found in the chain".into(),
			StatePruned => "Couldn't find the transaction block's state in the chain".into(),
			Exceptional => "An exception happened in the execution".into(),
			StateCorrupt => "Stored state found to be corrupted.".into(),
			Execution(ref e) => format!("{}", e),
		};

		f.write_fmt(format_args!("Transaction execution error ({}).", msg))
	}
}

/// Transaction execution result.
pub type ExecutionResult = Result<Executed, ExecutionError>;
