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

use client::Client;
use std::fmt;

/// Message type for external and internal events
#[derive(Debug)]
pub enum ClientIoMessage {
    /// Best Block Hash in chain has been changed
    NewChainHead,
    /// A block is ready
    BlockVerified,
    /// Execute wrapped closure
    Execute(Callback),
}

impl ClientIoMessage {
    /// Create new `ClientIoMessage` that executes given procedure.
    pub fn execute<F: Fn(&Client) + Send + Sync + 'static>(fun: F) -> Self {
        ClientIoMessage::Execute(Callback(Box::new(fun)))
    }
}

/// A function to invoke in the client thread.
pub struct Callback(pub Box<dyn Fn(&Client) + Send + Sync>);

impl fmt::Debug for Callback {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<callback>")
    }
}
