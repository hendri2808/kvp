// Copyright 2017-2021 Parity Technologies (UK) Ltd.
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
//! Autogenerated weights for `pallet_multisig`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-01-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 128

// Executed Command:
// target/release/polkadot
// benchmark
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=pallet_multisig
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/pallet_multisig.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_multisig`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		(15_003_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(z as Weight))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		(36_756_000 as Weight)
			// Standard Error: 0
			.saturating_add((145_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(z as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: Multisig Calls (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	fn as_multi_create_store(s: u32, z: u32, ) -> Weight {
		(40_129_000 as Weight)
			// Standard Error: 0
			.saturating_add((149_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(z as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		(22_525_000 as Weight)
			// Standard Error: 0
			.saturating_add((151_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(z as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: Multisig Calls (r:1 w:1)
	fn as_multi_approve_store(s: u32, z: u32, ) -> Weight {
		(37_995_000 as Weight)
			// Standard Error: 0
			.saturating_add((163_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(z as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: Multisig Calls (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		(48_573_000 as Weight)
			// Standard Error: 0
			.saturating_add((234_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((4_000 as Weight).saturating_mul(z as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	fn approve_as_multi_create(s: u32, ) -> Weight {
		(36_558_000 as Weight)
			// Standard Error: 0
			.saturating_add((152_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: Multisig Calls (r:1 w:0)
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		(22_369_000 as Weight)
			// Standard Error: 0
			.saturating_add((150_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: Multisig Calls (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn approve_as_multi_complete(s: u32, ) -> Weight {
		(83_083_000 as Weight)
			// Standard Error: 0
			.saturating_add((230_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: Multisig Calls (r:1 w:1)
	fn cancel_as_multi(s: u32, ) -> Weight {
		(60_105_000 as Weight)
			// Standard Error: 0
			.saturating_add((154_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
