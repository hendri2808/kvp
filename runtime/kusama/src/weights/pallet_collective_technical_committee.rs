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
//! Autogenerated weights for `pallet_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-15, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/kvp
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=pallet_collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	/// Storage: TechnicalCommittee Members (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:100 w:100)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (3232 ±0) + p * (3190 ±0)`
		//  Estimated: `19320 + m * (7799 ±16) + p * (10110 ±16)`
		// Minimum execution time: 17_755_000 picoseconds.
		Weight::from_parts(18_022_000, 0)
			.saturating_add(Weight::from_parts(0, 19320))
			// Standard Error: 48_475
			.saturating_add(Weight::from_parts(5_505_299, 0).saturating_mul(m.into()))
			// Standard Error: 48_475
			.saturating_add(Weight::from_parts(8_260_850, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 7799).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 10110).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `175 + m * (32 ±0)`
		//  Estimated: `1661 + m * (32 ±0)`
		// Minimum execution time: 16_765_000 picoseconds.
		Weight::from_parts(15_653_912, 0)
			.saturating_add(Weight::from_parts(0, 1661))
			// Standard Error: 25
			.saturating_add(Weight::from_parts(1_539, 0).saturating_mul(b.into()))
			// Standard Error: 261
			.saturating_add(Weight::from_parts(17_896, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `175 + m * (32 ±0)`
		//  Estimated: `5302 + m * (64 ±0)`
		// Minimum execution time: 19_194_000 picoseconds.
		Weight::from_parts(18_366_867, 0)
			.saturating_add(Weight::from_parts(0, 5302))
			// Standard Error: 19
			.saturating_add(Weight::from_parts(1_342, 0).saturating_mul(b.into()))
			// Standard Error: 200
			.saturating_add(Weight::from_parts(22_738, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalCount (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `465 + m * (32 ±0) + p * (36 ±0)`
		//  Estimated: `9880 + m * (165 ±0) + p * (180 ±0)`
		// Minimum execution time: 24_958_000 picoseconds.
		Weight::from_parts(25_925_520, 0)
			.saturating_add(Weight::from_parts(0, 9880))
			// Standard Error: 54
			.saturating_add(Weight::from_parts(2_430, 0).saturating_mul(b.into()))
			// Standard Error: 570
			.saturating_add(Weight::from_parts(17_303, 0).saturating_mul(m.into()))
			// Standard Error: 563
			.saturating_add(Weight::from_parts(119_736, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 165).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 180).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[5, 100]`.
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `914 + m * (64 ±0)`
		//  Estimated: `6776 + m * (128 ±0)`
		// Minimum execution time: 22_620_000 picoseconds.
		Weight::from_parts(23_356_968, 0)
			.saturating_add(Weight::from_parts(0, 6776))
			// Standard Error: 273
			.saturating_add(Weight::from_parts(40_919, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 128).saturating_mul(m.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `503 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `8367 + m * (260 ±0) + p * (144 ±0)`
		// Minimum execution time: 27_667_000 picoseconds.
		Weight::from_parts(29_094_490, 0)
			.saturating_add(Weight::from_parts(0, 8367))
			// Standard Error: 842
			.saturating_add(Weight::from_parts(25_691, 0).saturating_mul(m.into()))
			// Standard Error: 821
			.saturating_add(Weight::from_parts(133_244, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 260).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 144).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `805 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `12528 + b * (4 ±0) + m * (264 ±0) + p * (160 ±0)`
		// Minimum execution time: 41_678_000 picoseconds.
		Weight::from_parts(42_218_269, 0)
			.saturating_add(Weight::from_parts(0, 12528))
			// Standard Error: 59
			.saturating_add(Weight::from_parts(1_661, 0).saturating_mul(b.into()))
			// Standard Error: 624
			.saturating_add(Weight::from_parts(16_946, 0).saturating_mul(m.into()))
			// Standard Error: 608
			.saturating_add(Weight::from_parts(129_170, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 4).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 264).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 160).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `523 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `10435 + m * (325 ±0) + p * (180 ±0)`
		// Minimum execution time: 30_447_000 picoseconds.
		Weight::from_parts(32_661_910, 0)
			.saturating_add(Weight::from_parts(0, 10435))
			// Standard Error: 531
			.saturating_add(Weight::from_parts(29_960, 0).saturating_mul(m.into()))
			// Standard Error: 517
			.saturating_add(Weight::from_parts(120_475, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 325).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 180).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `825 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `14770 + b * (5 ±0) + m * (330 ±0) + p * (200 ±0)`
		// Minimum execution time: 44_068_000 picoseconds.
		Weight::from_parts(44_673_420, 0)
			.saturating_add(Weight::from_parts(0, 14770))
			// Standard Error: 59
			.saturating_add(Weight::from_parts(1_779, 0).saturating_mul(b.into()))
			// Standard Error: 625
			.saturating_add(Weight::from_parts(17_794, 0).saturating_mul(m.into()))
			// Standard Error: 609
			.saturating_add(Weight::from_parts(134_062, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 5).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 330).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 200).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `332 + p * (32 ±0)`
		//  Estimated: `2481 + p * (96 ±0)`
		// Minimum execution time: 15_528_000 picoseconds.
		Weight::from_parts(17_434_864, 0)
			.saturating_add(Weight::from_parts(0, 2481))
			// Standard Error: 405
			.saturating_add(Weight::from_parts(111_909, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 96).saturating_mul(p.into()))
	}
}
