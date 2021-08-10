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
//! Autogenerated weights for pallet_collective
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-18, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 128

// Executed Command:
// target/release/polkadot
// benchmark
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=pallet_collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_collective.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
    fn set_members(m: u32, n: u32, p: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 4_000
            .saturating_add((14_001_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 4_000
            .saturating_add((106_000 as Weight).saturating_mul(n as Weight))
            // Standard Error: 4_000
            .saturating_add((19_318_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(p as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
    }
    fn execute(b: u32, m: u32) -> Weight {
        (20_878_000 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(b as Weight))
            // Standard Error: 0
            .saturating_add((83_000 as Weight).saturating_mul(m as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
    }
    fn propose_execute(b: u32, m: u32) -> Weight {
        (25_662_000 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(b as Weight))
            // Standard Error: 0
            .saturating_add((162_000 as Weight).saturating_mul(m as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
    }
    fn propose_proposed(b: u32, m: u32, p: u32) -> Weight {
        (42_102_000 as Weight)
            // Standard Error: 0
            .saturating_add((4_000 as Weight).saturating_mul(b as Weight))
            // Standard Error: 0
            .saturating_add((89_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 0
            .saturating_add((364_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn vote(m: u32) -> Weight {
        (31_866_000 as Weight)
            // Standard Error: 0
            .saturating_add((196_000 as Weight).saturating_mul(m as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn close_early_disapproved(m: u32, p: u32) -> Weight {
        (40_751_000 as Weight)
            // Standard Error: 0
            .saturating_add((167_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 0
            .saturating_add((333_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn close_early_approved(b: u32, m: u32, p: u32) -> Weight {
        (56_534_000 as Weight)
            // Standard Error: 0
            .saturating_add((3_000 as Weight).saturating_mul(b as Weight))
            // Standard Error: 0
            .saturating_add((166_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 0
            .saturating_add((347_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn close_disapproved(m: u32, p: u32) -> Weight {
        (45_673_000 as Weight)
            // Standard Error: 0
            .saturating_add((167_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 0
            .saturating_add((336_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn close_approved(b: u32, m: u32, p: u32) -> Weight {
        (61_152_000 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(b as Weight))
            // Standard Error: 0
            .saturating_add((167_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 0
            .saturating_add((340_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn disapprove_proposal(p: u32) -> Weight {
        (25_103_000 as Weight)
            // Standard Error: 0
            .saturating_add((346_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
}
