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

//! Autogenerated weights for `pallet_elections_phragmen`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_elections_phragmen
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kvp/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_elections_phragmen`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_elections_phragmen::WeightInfo for WeightInfo<T> {
	/// Storage: PhragmenElection Candidates (r:1 w:0)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Voting (r:1 w:1)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// The range of component `v` is `[1, 16]`.
	fn vote_equal(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `369 + v * (80 ±0)`
		//  Estimated: `4764 + v * (80 ±0)`
		// Minimum execution time: 32_711_000 picoseconds.
		Weight::from_parts(33_843_954, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			// Standard Error: 3_332
			.saturating_add(Weight::from_parts(148_060, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(v.into()))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:0)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Voting (r:1 w:1)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// The range of component `v` is `[2, 16]`.
	fn vote_more(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `337 + v * (80 ±0)`
		//  Estimated: `4764 + v * (80 ±0)`
		// Minimum execution time: 46_078_000 picoseconds.
		Weight::from_parts(46_574_818, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			// Standard Error: 3_834
			.saturating_add(Weight::from_parts(182_895, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(v.into()))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:0)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Voting (r:1 w:1)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// The range of component `v` is `[2, 16]`.
	fn vote_less(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `369 + v * (80 ±0)`
		//  Estimated: `4764 + v * (80 ±0)`
		// Minimum execution time: 45_677_000 picoseconds.
		Weight::from_parts(46_613_391, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			// Standard Error: 4_271
			.saturating_add(Weight::from_parts(180_095, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(v.into()))
	}
	/// Storage: PhragmenElection Voting (r:1 w:1)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn remove_voter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `891`
		//  Estimated: `4764`
		// Minimum execution time: 47_963_000 picoseconds.
		Weight::from_parts(48_833_000, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:1)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 1000]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2358 + c * (48 ±0)`
		//  Estimated: `3841 + c * (48 ±0)`
		// Minimum execution time: 39_368_000 picoseconds.
		Weight::from_parts(28_568_416, 0)
			.saturating_add(Weight::from_parts(0, 3841))
			// Standard Error: 1_416
			.saturating_add(Weight::from_parts(131_107, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 48).saturating_mul(c.into()))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:1)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 1000]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `250 + c * (48 ±0)`
		//  Estimated: `1722 + c * (48 ±0)`
		// Minimum execution time: 34_977_000 picoseconds.
		Weight::from_parts(24_677_388, 0)
			.saturating_add(Weight::from_parts(0, 1722))
			// Standard Error: 1_498
			.saturating_add(Weight::from_parts(100_855, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 48).saturating_mul(c.into()))
	}
	/// Storage: PhragmenElection Members (r:1 w:1)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:1)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	fn renounce_candidacy_members() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2599`
		//  Estimated: `4084`
		// Minimum execution time: 52_891_000 picoseconds.
		Weight::from_parts(53_852_000, 0)
			.saturating_add(Weight::from_parts(0, 4084))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: PhragmenElection RunnersUp (r:1 w:1)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	fn renounce_candidacy_runners_up() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1711`
		//  Estimated: `3196`
		// Minimum execution time: 36_514_000 picoseconds.
		Weight::from_parts(37_441_000, 0)
			.saturating_add(Weight::from_parts(0, 3196))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	fn remove_member_without_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000_000_000_000 picoseconds.
		Weight::from_parts(2_000_000_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: PhragmenElection Members (r:1 w:1)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: PhragmenElection RunnersUp (r:1 w:1)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_member_with_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2599`
		//  Estimated: `4084`
		// Minimum execution time: 73_160_000 picoseconds.
		Weight::from_parts(74_548_000, 0)
			.saturating_add(Weight::from_parts(0, 4084))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: PhragmenElection Voting (r:10001 w:10000)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Candidates (r:1 w:0)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:10000 w:10000)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:10000 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: System Account (r:10000 w:10000)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `v` is `[5000, 10000]`.
	/// The range of component `d` is `[0, 5000]`.
	fn clean_defunct_voters(v: u32, _d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `36028 + v * (808 ±0)`
		//  Estimated: `39768 + v * (3774 ±0)`
		// Minimum execution time: 434_369_619_000 picoseconds.
		Weight::from_parts(436_606_328_000, 0)
			.saturating_add(Weight::from_parts(0, 39768))
			// Standard Error: 365_744
			.saturating_add(Weight::from_parts(53_633_149, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(v.into())))
			.saturating_add(Weight::from_parts(0, 3774).saturating_mul(v.into()))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:1)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:1)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:1)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Voting (r:10001 w:0)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:967 w:967)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: PhragmenElection ElectionRounds (r:1 w:1)
	/// Proof Skipped: PhragmenElection ElectionRounds (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `v` is `[1, 10000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + e * (28 ±0) + v * (607 ±0)`
		//  Estimated: `2771984 + c * (2560 ±0) + e * (16 ±0) + v * (2744 ±4)`
		// Minimum execution time: 39_817_678_000 picoseconds.
		Weight::from_parts(40_023_537_000, 0)
			.saturating_add(Weight::from_parts(0, 2771984))
			// Standard Error: 411_583
			.saturating_add(Weight::from_parts(34_005_169, 0).saturating_mul(v.into()))
			// Standard Error: 26_412
			.saturating_add(Weight::from_parts(1_743_887, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(269))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2560).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 16).saturating_mul(e.into()))
			.saturating_add(Weight::from_parts(0, 2744).saturating_mul(v.into()))
	}
}
