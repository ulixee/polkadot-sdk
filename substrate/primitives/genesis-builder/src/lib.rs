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

#![cfg_attr(not(feature = "std"), no_std)]

//! Substrate genesis config builder
//!
//! This module provides means to interact with `RuntimeGenesisConfig`. Runtime provides a default
//! `RuntimeGenesisConfig` structire in form of json blob.
//!
//! Additionally the runtime may provide a number of partial predefined `RuntimeGenesisConfig`
//! configurations in the form of patches which shall be applied on top of the default
//! `RuntimeGenesisConfig`. The patch is a JSON blob, which essentially comprises the list of
//! key-value pairs that are to be customized in the default runtime genesis config.
//! These predefined configurations are refered to as presets.
//!
//! This allows the runtime to provide a number of predefined configs (e.g. for different
//! testnets) without neccessity to leak the runtime types outside the itself.
//!
//! This Runtime API allows to interact with `RuntimeGenesisConfig`, in particular:
//! - provide the list of available preset names,
//! - provide a number of named, built-in, presets of `RuntimeGenesisConfig`,
//! - serialize the default `RuntimeGenesisConfig` struct into json format,
//! - deserialize the `RuntimeGenesisConfig` from given json blob and put the resulting
//!   `RuntimeGenesisConfig` into the state storage creating the initial runtime's state. Allows to
//!   build customized genesis. This operation internally calls `GenesisBuild::build` function for
//!   all runtime pallets.
//!
//! Providing externalities with empty storage and putting `RuntimeGenesisConfig` into storage
//! allows to catch and build the raw storage of `RuntimeGenesisConfig` which is the foundation for
//! genesis block.

extern crate alloc;
use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

/// The result type alias, used in build methods. `Err` contains formatted error message.
pub type Result = core::result::Result<(), sp_runtime::RuntimeString>;

/// Wrapper representing preset id.
#[derive(Decode, Encode, Debug, TypeInfo, PartialEq)]
pub struct PresetId(Vec<u8>);

impl From<&str> for PresetId {
	fn from(v: &str) -> Self {
		Self(v.as_bytes().to_vec())
	}
}

impl<'a> TryFrom<&'a PresetId> for &'a str {
	type Error = core::str::Utf8Error;
	fn try_from(v: &'a PresetId) -> core::result::Result<&'a str, Self::Error> {
		core::str::from_utf8(&v.0)
	}
}

sp_api::decl_runtime_apis! {
	/// API to interact with RuntimeGenesisConfig for the runtime
	pub trait GenesisBuilder {
		/// Build `RuntimeGenesisConfig` from a JSON blob not using any defaults and store it in the
		/// storage.
		///
		/// This function deserializes the full `RuntimeGenesisConfig` from the given JSON blob and
		/// puts it into the storage. If the provided JSON blob is incorrect or incomplete or the
		/// deserialization fails, an error is returned.
		///
		/// Please note that provided json blob must contain all `RuntimeGenesisConfig` fields, no
		/// defaults will be used.
		fn build_state(json: Vec<u8>) -> Result;

		/// Returns a JSON blob representation of the built-in `RuntimeGenesisConfig` identified by
		/// `id`.
		///
		/// If `id` is `None` the function returns JSON blob representation of the default
		/// `RuntimeGenesisConfig` struct of the runtime. Implementation must provide default
		/// `RuntimeGenesisConfig`.
		///
		/// Otherwise function returns a JSON representation of the built-in, named `RuntimeGenesisConfig` preset
		/// identified by `id`, or `None` if such preset does not exists. Returned `Vec<u8>` contains bytes of JSON blob
		/// (patch) which comprises a list of (potentially nested) key-value pairs that are intended for custimizing the
		/// default runtime genesis config. The patch shall be merged (rfc7386) with default genesis config in order to
		/// obtain a full representation of genesis config that can be used in `build_state` method.
		fn get_preset(id: &Option<PresetId>) -> Option<Vec<u8>>;

		/// Returns a list of names for available builtin `RuntimeGenesisConfig` presets.
		///
		/// The presets from the list can be queried with [`GenesisBuilder::get_preset`] method. If
		/// no named presets are provided by the runtime the list is empty.
		fn preset_names() -> Vec<PresetId>;
	}
}
