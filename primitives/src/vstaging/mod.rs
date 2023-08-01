// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Staging Primitives.

// Put any primitives used by staging APIs functions here
pub use crate::v5::*;
use sp_std::prelude::*;

use parity_scale_codec::{Decode, Encode};
use primitives::RuntimeDebug;
use scale_info::TypeInfo;

/// Candidate's acceptance limitations for asynchronous backing per relay parent.
#[derive(
	RuntimeDebug,
	Copy,
	Clone,
	PartialEq,
	Encode,
	Decode,
	TypeInfo,
	serde::Serialize,
	serde::Deserialize,
)]

pub struct AsyncBackingParams {
	/// The maximum number of para blocks between the para head in a relay parent
	/// and a new candidate. Restricts nodes from building arbitrary long chains
	/// and spamming other validators.
	///
	/// When async backing is disabled, the only valid value is 0.
	pub max_candidate_depth: u32,
	/// How many ancestors of a relay parent are allowed to build candidates on top
	/// of.
	///
	/// When async backing is disabled, the only valid value is 0.
	pub allowed_ancestry_len: u32,
}

/// Approval voting configuration parameters
#[derive(
	RuntimeDebug,
	Copy,
	Clone,
	PartialEq,
	Encode,
	Decode,
	TypeInfo,
	serde::Serialize,
	serde::Deserialize,
)]
pub struct ApprovalVotingParams {
	/// The maximum number of candidates `approval-voting` can vote for with
	/// a single signatures.
	///
	/// Setting it to 1, means we send the approval as soon as we have it available.
	pub max_approval_coalesce_count: u32,
	/// The maximum ticks we await for a candidate approval to be coalesced with
	/// the ones for other candidate before we sign it and distribute to our peers
	pub max_approval_coalesce_wait_ticks: u32,
}
