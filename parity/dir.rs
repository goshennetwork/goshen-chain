// Copyright 2015, 2016 Parity Technologies (UK) Ltd.
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

use std::fs;
use std::path::{PathBuf, Path};
use util::{H64, H256};
use util::journaldb::Algorithm;
use helpers::replace_home;
use app_dirs::{AppInfo, get_app_root, AppDataType};

// this const is irrelevent cause we do have migrations now,
// but we still use it for backwards compatibility
const LEGACY_CLIENT_DB_VER_STR: &'static str = "5.3";

#[derive(Debug, PartialEq)]
pub struct Directories {
	pub base: String,
	pub db: String,
	pub keys: String,
	pub signer: String,
	pub dapps: String,
}

impl Default for Directories {
	fn default() -> Self {
		let data_dir = default_data_path();
		Directories {
			base: replace_home(&data_dir, "$BASE"),
			db: replace_home(&data_dir, "$BASE/chains"),
			keys: replace_home(&data_dir, "$BASE/keys"),
			signer: replace_home(&data_dir, "$BASE/signer"),
			dapps: replace_home(&data_dir, "$BASE/dapps"),
		}
	}
}

impl Directories {
	pub fn create_dirs(&self, dapps_enabled: bool, signer_enabled: bool) -> Result<(), String> {
		try!(fs::create_dir_all(&self.base).map_err(|e| e.to_string()));
		try!(fs::create_dir_all(&self.db).map_err(|e| e.to_string()));
		try!(fs::create_dir_all(&self.keys).map_err(|e| e.to_string()));
		if signer_enabled {
			try!(fs::create_dir_all(&self.signer).map_err(|e| e.to_string()));
		}
		if dapps_enabled {
			try!(fs::create_dir_all(&self.dapps).map_err(|e| e.to_string()));
		}
		Ok(())
	}

	/// Database paths.
	pub fn database(&self, genesis_hash: H256, fork_name: Option<String>, spec_name: String) -> DatabaseDirectories {
		DatabaseDirectories {
			path: self.db.clone(),
			legacy_path: self.base.clone(),
			genesis_hash: genesis_hash,
			fork_name: fork_name,
			spec_name: spec_name,
		}
	}

	/// Get the ipc sockets path
	pub fn ipc_path(&self) -> PathBuf {
		let mut dir = Path::new(&self.base).to_path_buf();
		dir.push("ipc");
		dir
	}

	// TODO: remove in 1.7
	pub fn legacy_keys_path(&self, testnet: bool) -> PathBuf {
		let mut dir = Path::new(&self.base).to_path_buf();
		if testnet {
			dir.push("testnet_keys");
		} else {
			dir.push("keys");
		}
		dir
	}

	pub fn keys_path(&self, spec_name: &str) -> PathBuf {
		let mut dir = PathBuf::from(&self.keys);
		dir.push(spec_name);
		dir
	}
}

#[derive(Debug, PartialEq)]
pub struct DatabaseDirectories {
	pub path: String,
	pub legacy_path: String,
	pub genesis_hash: H256,
	pub fork_name: Option<String>,
	pub spec_name: String,
}

impl DatabaseDirectories {
	/// Base DB directory for the given fork.
	// TODO: remove in 1.7
	pub fn legacy_fork_path(&self) -> PathBuf {
		let mut dir = Path::new(&self.legacy_path).to_path_buf();
		dir.push(format!("{:?}{}", H64::from(self.genesis_hash), self.fork_name.as_ref().map(|f| format!("-{}", f)).unwrap_or_default()));
		dir
	}

	pub fn spec_root_path(&self) -> PathBuf {
		let mut dir = Path::new(&self.path).to_path_buf();
		dir.push(&self.spec_name);
		dir
	}

	pub fn client_path(&self, pruning: Algorithm) -> PathBuf {
		let mut dir = self.db_root_path();
		dir.push(pruning.as_internal_name_str());
		dir.push("db");
		dir
	}

	pub fn db_root_path(&self) -> PathBuf {
		let mut dir = self.spec_root_path();
		dir.push("db");
		dir.push(H64::from(self.genesis_hash).hex());
		dir
	}

	pub fn db_path(&self, pruning: Algorithm) -> PathBuf {
		let mut dir = self.db_root_path();
		dir.push(pruning.as_internal_name_str());
		dir
	}

	/// Get the root path for database
	// TODO: remove in 1.7
	pub fn legacy_version_path(&self, pruning: Algorithm) -> PathBuf {
		let mut dir = self.legacy_fork_path();
		dir.push(format!("v{}-sec-{}", LEGACY_CLIENT_DB_VER_STR, pruning.as_internal_name_str()));
		dir
	}

	/// Get user defaults path
	// TODO: remove in 1.7
	pub fn legacy_user_defaults_path(&self) -> PathBuf {
		let mut dir = self.legacy_fork_path();
		dir.push("user_defaults");
		dir
	}

	/// Get user defaults path
	// TODO: remove in 1.7
	pub fn legacy_snapshot_path(&self) -> PathBuf {
		let mut dir = self.legacy_fork_path();
		dir.push("snapshot");
		dir
	}

	/// Get user defaults path
	// TODO: remove in 1.7
	pub fn legacy_network_path(&self) -> PathBuf {
		let mut dir = self.legacy_fork_path();
		dir.push("network");
		dir
	}

	pub fn user_defaults_path(&self) -> PathBuf {
		let mut dir = self.spec_root_path();
		dir.push("user_defaults");
		dir
	}

	/// Get the path for the snapshot directory given the genesis hash and fork name.
	pub fn snapshot_path(&self) -> PathBuf {
		let mut dir = self.db_root_path();
		dir.push("snapshot");
		dir
	}

	/// Get the path for the network directory.
	pub fn network_path(&self) -> PathBuf {
		let mut dir = self.spec_root_path();
		dir.push("network");
		dir
	}
}

pub fn default_data_path() -> String {
	let app_info = AppInfo { name: "parity", author: "parity" };
	get_app_root(AppDataType::UserData, &app_info).map(|p| p.to_string_lossy().into_owned()).unwrap_or_else(|_| "$HOME/.parity".to_owned())
}

#[cfg(test)]
mod tests {
	use super::Directories;
	use helpers::replace_home;

	#[test]
	fn test_default_directories() {
		let data_dir = super::default_data_path();
		let expected = Directories {
			base: replace_home(&data_dir, "$BASE"),
			db: replace_home(&data_dir, "$BASE/chains"),
			keys: replace_home(&data_dir, "$BASE/keys"),
			signer: replace_home(&data_dir, "$BASE/signer"),
			dapps: replace_home(&data_dir, "$BASE/dapps"),
		};
		assert_eq!(expected, Directories::default());
	}
}
