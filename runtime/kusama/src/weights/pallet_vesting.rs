// Copyright 2017-2020 Parity Technologies (UK) Ltd.
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
//! Autogenerated weights for pallet_vesting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-19, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 128

// Executed Command:
// target/release/polkadot
// benchmark
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=pallet_vesting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_vesting.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vesting::WeightInfo for WeightInfo<T> {
	fn vest_locked(l: u32, s: u32, ) -> Weight {
		(58_117_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((136_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 12_000
			.saturating_add((177_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn vest_unlocked(l: u32, s: u32, ) -> Weight {
		(57_903_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((126_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 6_000
			.saturating_add((51_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn vest_other_locked(l: u32, s: u32, ) -> Weight {
		(57_736_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((140_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 4_000
			.saturating_add((159_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
		(57_278_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((131_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 5_000
			.saturating_add((68_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn last_vested_transfer(l: u32, _s: u32, ) -> Weight {
		(101_019_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((135_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn first_vested_transfer(l: u32, ) -> Weight {
		(118_733_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((138_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn first_force_vested_transfer(l: u32, ) -> Weight {
		(117_482_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((146_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn last_force_vested_transfer(l: u32, _s: u32, ) -> Weight {
		(100_191_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((144_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		(71_299_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((115_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 6_000
			.saturating_add((101_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		(70_606_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((151_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 6_000
			.saturating_add((91_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
