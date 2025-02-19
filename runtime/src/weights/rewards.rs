// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of 3DPass.
//
// Copyright (c) 2020 Wei Tang.
// Copyright (c) 2020 Shawn Tabrizi.
// Copyright (c) 2022 3DPass.
//
// 3DPass is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// 3DPass is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with 3DPass. If not, see <http://www.gnu.org/licenses/>.

// Executed Command:
// ./target/release/poscan-consensus
// benchmark
// --chain
// dev
// --steps
// 50
// --repeat
// 20
// --pallet
// rewards
// --extrinsic
// *
// --raw
// --execution=wasm
// --wasm-execution=compiled
// --output
// runtime/src/weights/rewards.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for rewards.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> rewards::WeightInfo for WeightInfo<T> {
	fn on_initialize() -> Weight {
		(14_700_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn on_finalize() -> Weight {
		(121_300_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn unlock() -> Weight {
		(45_200_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn lock() -> Weight {
		(45_200_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_schedule() -> Weight {
		(32_500_000 as Weight).saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn set_lock_params() -> Weight {
		(0 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_miner_share() -> Weight {
		(0 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
