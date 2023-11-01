// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Minimal example of a pallet that uses the multi-block migrations framework.
//!
//! The most important part of this pallet is the [`migrations::v0`] module, which contains the
//! [`SteppedMigration`](`frame_support::migrations::SteppedMigration`) implementation. This
//! implementation is used to migrate the values in the [`StoredValue`](`pallet::StoredValue`)
//! storage map from `u32` to `u64`.
//!
//! The [`StoredValue`](`pallet::StoredValue`) storage item is defined in this `pallet`, and is
//! aliased to [`old::StoredValue`](`migrations::v0::old::StoredValue`) in the [`migrations::v0`]
//! module.

pub mod migrations;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::StorageMap, Blake2_128Concat};

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::storage]
	/// Dummy storage item used to demonstrate the multi-block migrations framework.
	pub type StoredValue<T: Config> = StorageMap<_, Blake2_128Concat, u32, u64>;
}
