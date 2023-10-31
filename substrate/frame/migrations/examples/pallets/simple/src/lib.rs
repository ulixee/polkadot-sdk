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

pub mod migrations;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::{MaxEncodedLen, StorageMap},
		Parameter, Twox128,
	};

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Foo: Parameter + MaxEncodedLen;
		type Bar: Parameter + MaxEncodedLen;
	}

	#[pallet::storage]
	pub type StoredValue<T: Config> = StorageMap<_, Twox128, T::Foo, T::Bar>;
}
