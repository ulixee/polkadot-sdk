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

//! Minimal example of a runtime that uses the multi-block migrations framework.
//! The most relevant part is the `pallet_migrations::Config` implementation. Here, we define the
//! `Migrations` type as a tuple of the migrations that we want to run. In this case, we only have
//! one migration, which is the [`v0::Migration`].

use frame_support::{construct_runtime, derive_impl, traits::ConstU32};
use pallet_migrations_examples_simple::{migrations::*, pallet};

type Block = frame_system::mocking::MockBlock<Runtime>;

impl pallet::Config for Runtime {}

impl pallet_migrations::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	/// The type that implements
	/// [`SteppedMigrations`](`frame_support::migrations::SteppedMigrations`).
	///
	/// We list in this tuple the migrations to run.
	///
	/// # Example
	/// ```ignore
	/// type Migrations = (v0::Migration<Runtime>, v1::Migration<Runtime>, v3::Migration<Runtime>);
	/// ```
	type Migrations = (v1::LazyMigrationV1<Runtime>,);
	type CursorMaxLen = ConstU32<256>;
	type IdentifierMaxLen = ConstU32<256>;
	type OnMigrationUpdate = ();
	type ServiceWeight = ();
	type WeightInfo = ();
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	type Block = Block;
}

construct_runtime! {
	pub struct Runtime
	{
		System: frame_system,
		Pallet: pallet,
		Migrations: pallet_migrations,
	}
}
