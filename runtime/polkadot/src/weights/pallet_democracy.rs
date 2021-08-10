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
//! Autogenerated weights for pallet_democracy
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
// --pallet=pallet_democracy
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

/// Weight functions for pallet_democracy.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::WeightInfo for WeightInfo<T> {
    fn propose() -> Weight {
        (55_166_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn second(s: u32) -> Weight {
        (36_768_000 as Weight)
            // Standard Error: 0
            .saturating_add((143_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn vote_new(r: u32) -> Weight {
        (42_144_000 as Weight)
            // Standard Error: 0
            .saturating_add((183_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn vote_existing(r: u32) -> Weight {
        (42_316_000 as Weight)
            // Standard Error: 0
            .saturating_add((174_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn emergency_cancel() -> Weight {
        (26_501_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn blacklist(p: u32) -> Weight {
        (73_883_000 as Weight)
            // Standard Error: 4_000
            .saturating_add((460_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(6 as Weight))
    }
    fn external_propose(v: u32) -> Weight {
        (12_461_000 as Weight)
            // Standard Error: 0
            .saturating_add((78_000 as Weight).saturating_mul(v as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn external_propose_majority() -> Weight {
        (2_393_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn external_propose_default() -> Weight {
        (2_401_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn fast_track() -> Weight {
        (25_767_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn veto_external(v: u32) -> Weight {
        (26_834_000 as Weight)
            // Standard Error: 0
            .saturating_add((116_000 as Weight).saturating_mul(v as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn cancel_proposal(p: u32) -> Weight {
        (49_339_000 as Weight)
            // Standard Error: 0
            .saturating_add((436_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn cancel_referendum() -> Weight {
        (15_793_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn cancel_queued(r: u32) -> Weight {
        (27_553_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((1_614_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn on_initialize_base(r: u32) -> Weight {
        (6_724_000 as Weight)
            // Standard Error: 4_000
            .saturating_add((5_014_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
    }
    fn delegate(r: u32) -> Weight {
        (50_940_000 as Weight)
            // Standard Error: 4_000
            .saturating_add((7_009_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
    }
    fn undelegate(r: u32) -> Weight {
        (22_520_000 as Weight)
            // Standard Error: 5_000
            .saturating_add((6_964_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
    }
    fn clear_public_proposals() -> Weight {
        (2_251_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn note_preimage(b: u32) -> Weight {
        (40_109_000 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(b as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn note_imminent_preimage(b: u32) -> Weight {
        (25_678_000 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(b as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn reap_preimage(b: u32) -> Weight {
        (36_132_000 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(b as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn unlock_remove(r: u32) -> Weight {
        (36_160_000 as Weight)
            // Standard Error: 0
            .saturating_add((51_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn unlock_set(r: u32) -> Weight {
        (33_706_000 as Weight)
            // Standard Error: 0
            .saturating_add((165_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn remove_vote(r: u32) -> Weight {
        (18_398_000 as Weight)
            // Standard Error: 0
            .saturating_add((149_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn remove_other_vote(r: u32) -> Weight {
        (18_520_000 as Weight)
            // Standard Error: 0
            .saturating_add((151_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
}
