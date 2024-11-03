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

//! Autogenerated weights for `runtime_common::slots`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-14, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner--ss9ysm1-project-163-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/kvp
// benchmark
// pallet
// --chain=westend-dev
// --steps=50
// --repeat=20
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --pallet=runtime_common::slots
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/westend/src/weights/runtime_common_slots.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `runtime_common::slots`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_common::slots::WeightInfo for WeightInfo<T> {
	/// Storage: Slots Leases (r:1 w:1)
	/// Proof Skipped: Slots Leases (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_lease() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `181`
		//  Estimated: `3646`
		// Minimum execution time: 30_033_000 picoseconds.
		Weight::from_parts(30_893_000, 0)
			.saturating_add(Weight::from_parts(0, 3646))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Paras Parachains (r:1 w:0)
	/// Proof Skipped: Paras Parachains (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Slots Leases (r:101 w:100)
	/// Proof Skipped: Slots Leases (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras ParaLifecycles (r:200 w:200)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras ActionsQueue (r:1 w:1)
	/// Proof Skipped: Paras ActionsQueue (max_values: None, max_size: None, mode: Measured)
	/// Storage: Registrar Paras (r:100 w:100)
	/// Proof Skipped: Registrar Paras (max_values: None, max_size: None, mode: Measured)
	/// The range of component `c` is `[0, 100]`.
	/// The range of component `t` is `[0, 100]`.
	fn manage_lease_period_start(c: u32, t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + c * (47 ±0) + t * (308 ±0)`
		//  Estimated: `2694 + c * (2526 ±0) + t * (2789 ±0)`
		// Minimum execution time: 737_927_000 picoseconds.
		Weight::from_parts(769_816_000, 0)
			.saturating_add(Weight::from_parts(0, 2694))
			// Standard Error: 112_360
			.saturating_add(Weight::from_parts(3_515_663, 0).saturating_mul(c.into()))
			// Standard Error: 112_360
			.saturating_add(Weight::from_parts(14_335_474, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(t.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(t.into())))
			.saturating_add(Weight::from_parts(0, 2526).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 2789).saturating_mul(t.into()))
	}
	/// Storage: Slots Leases (r:1 w:1)
	/// Proof Skipped: Slots Leases (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:8 w:8)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn clear_all_leases() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2653`
		//  Estimated: `21814`
		// Minimum execution time: 153_832_000 picoseconds.
		Weight::from_parts(170_790_000, 0)
			.saturating_add(Weight::from_parts(0, 21814))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	/// Storage: Slots Leases (r:1 w:0)
	/// Proof Skipped: Slots Leases (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras ParaLifecycles (r:1 w:1)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras ActionsQueue (r:1 w:1)
	/// Proof Skipped: Paras ActionsQueue (max_values: None, max_size: None, mode: Measured)
	/// Storage: Registrar Paras (r:1 w:1)
	/// Proof Skipped: Registrar Paras (max_values: None, max_size: None, mode: Measured)
	fn trigger_onboard() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `601`
		//  Estimated: `4066`
		// Minimum execution time: 31_647_000 picoseconds.
		Weight::from_parts(41_735_000, 0)
			.saturating_add(Weight::from_parts(0, 4066))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
