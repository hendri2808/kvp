// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of kvp.

// kvp is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// kvp is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with kvp.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `runtime_parachains::initializer`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-19, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-e8ezs4ez-project-163-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kvp-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/kvp
// benchmark
// pallet
// --chain=kvp-dev
// --steps=50
// --repeat=20
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --pallet=runtime_parachains::initializer
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kvp/src/weights/runtime_parachains_initializer.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `runtime_parachains::initializer`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::initializer::WeightInfo for WeightInfo<T> {
	/// Storage: System Digest (r:1 w:1)
	/// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `d` is `[0, 65536]`.
	fn force_approve(d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + d * (11 ±0)`
		//  Estimated: `1480 + d * (11 ±0)`
		// Minimum execution time: 3_619_000 picoseconds.
		Weight::from_parts(3_743_000, 0)
			.saturating_add(Weight::from_parts(0, 1480))
			// Standard Error: 17
			.saturating_add(Weight::from_parts(3_045, 0).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 11).saturating_mul(d.into()))
	}
}
