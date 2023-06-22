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

//! Autogenerated weights for `pallet_message_queue`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-e8ezs4ez-project-163-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --pallet=pallet_message_queue
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_message_queue`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_message_queue::WeightInfo for WeightInfo<T> {
	/// Storage: MessageQueue ServiceHead (r:1 w:0)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(6), added: 501, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(55), added: 2530, mode: MaxEncodedLen)
	fn ready_ring_knit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `248`
		//  Estimated: `6050`
		// Minimum execution time: 12_778_000 picoseconds.
		Weight::from_parts(13_167_000, 0)
			.saturating_add(Weight::from_parts(0, 6050))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(55), added: 2530, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(6), added: 501, mode: MaxEncodedLen)
	fn ready_ring_unknit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `248`
		//  Estimated: `6050`
		// Minimum execution time: 11_910_000 picoseconds.
		Weight::from_parts(12_318_000, 0)
			.saturating_add(Weight::from_parts(0, 6050))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(55), added: 2530, mode: MaxEncodedLen)
	fn service_queue_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3520`
		// Minimum execution time: 5_070_000 picoseconds.
		Weight::from_parts(5_266_000, 0)
			.saturating_add(Weight::from_parts(0, 3520))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65586), added: 68061, mode: MaxEncodedLen)
	fn service_page_base_completion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115`
		//  Estimated: `69051`
		// Minimum execution time: 6_812_000 picoseconds.
		Weight::from_parts(7_085_000, 0)
			.saturating_add(Weight::from_parts(0, 69051))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65586), added: 68061, mode: MaxEncodedLen)
	fn service_page_base_no_completion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115`
		//  Estimated: `69051`
		// Minimum execution time: 7_136_000 picoseconds.
		Weight::from_parts(7_392_000, 0)
			.saturating_add(Weight::from_parts(0, 69051))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn service_page_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 92_069_000 picoseconds.
		Weight::from_parts(92_769_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(6), added: 501, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:1 w:0)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(55), added: 2530, mode: MaxEncodedLen)
	fn bump_service_head() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `149`
		//  Estimated: `3520`
		// Minimum execution time: 7_443_000 picoseconds.
		Weight::from_parts(7_670_000, 0)
			.saturating_add(Weight::from_parts(0, 3520))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(55), added: 2530, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65586), added: 68061, mode: MaxEncodedLen)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: unknown `0x3a72656c61795f64697370617463685f71756575655f72656d61696e696e675f` (r:0 w:1)
	/// Proof Skipped: unknown `0x3a72656c61795f64697370617463685f71756575655f72656d61696e696e675f` (r:0 w:1)
	/// Storage: unknown `0xf5207f03cfdce586301014700e2c2593fad157e461d71fd4c1f936839a5f1f3e` (r:0 w:1)
	/// Proof Skipped: unknown `0xf5207f03cfdce586301014700e2c2593fad157e461d71fd4c1f936839a5f1f3e` (r:0 w:1)
	fn reap_page() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66030`
		//  Estimated: `69051`
		// Minimum execution time: 67_176_000 picoseconds.
		Weight::from_parts(68_406_000, 0)
			.saturating_add(Weight::from_parts(0, 69051))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(55), added: 2530, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65586), added: 68061, mode: MaxEncodedLen)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: unknown `0x3a72656c61795f64697370617463685f71756575655f72656d61696e696e675f` (r:0 w:1)
	/// Proof Skipped: unknown `0x3a72656c61795f64697370617463685f71756575655f72656d61696e696e675f` (r:0 w:1)
	/// Storage: unknown `0xf5207f03cfdce586301014700e2c2593fad157e461d71fd4c1f936839a5f1f3e` (r:0 w:1)
	/// Proof Skipped: unknown `0xf5207f03cfdce586301014700e2c2593fad157e461d71fd4c1f936839a5f1f3e` (r:0 w:1)
	fn execute_overweight_page_removed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66030`
		//  Estimated: `69051`
		// Minimum execution time: 83_156_000 picoseconds.
		Weight::from_parts(85_134_000, 0)
			.saturating_add(Weight::from_parts(0, 69051))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(55), added: 2530, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65586), added: 68061, mode: MaxEncodedLen)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: unknown `0x3a72656c61795f64697370617463685f71756575655f72656d61696e696e675f` (r:0 w:1)
	/// Proof Skipped: unknown `0x3a72656c61795f64697370617463685f71756575655f72656d61696e696e675f` (r:0 w:1)
	/// Storage: unknown `0xf5207f03cfdce586301014700e2c2593fad157e461d71fd4c1f936839a5f1f3e` (r:0 w:1)
	/// Proof Skipped: unknown `0xf5207f03cfdce586301014700e2c2593fad157e461d71fd4c1f936839a5f1f3e` (r:0 w:1)
	fn execute_overweight_page_updated() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66030`
		//  Estimated: `69051`
		// Minimum execution time: 125_205_000 picoseconds.
		Weight::from_parts(127_325_000, 0)
			.saturating_add(Weight::from_parts(0, 69051))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
