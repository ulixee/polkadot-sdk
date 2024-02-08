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

//! Traits for creating, editing and destroying assets.

use super::Inspect;
use crate::traits::tokens::{AssetId, Balance};
use sp_runtime::{DispatchError, DispatchResult};

/// Trait for providing the ability to create new fungible assets.
pub trait Create<AccountId>: Inspect<AccountId> {
	/// Create a new fungible asset.
	fn create(
		id: Self::AssetId,
		admin: AccountId,
		is_sufficient: bool,
		min_balance: Self::Balance,
	) -> DispatchResult;
}

/// Trait for resetting the team configuration of an existing fungible asset.
pub trait ResetTeam<AccountId> {
	/// Means of identifying one asset class from another.
	type AssetId: AssetId;
	/// Reset the team for the asset with the given `id`.
	///
	/// ### Parameters
	///
	/// - `id`: The identifier of the asset for which the team is being reset.
	/// - `owner`: The new `owner` account for the asset.
	/// - `admin`: The new `admin` account for the asset.
	/// - `issuer`: The new `issuer` account for the asset.
	/// - `freezer`: The new `freezer` account for the asset.
	fn reset_team(
		id: Self::AssetId,
		owner: AccountId,
		admin: AccountId,
		issuer: AccountId,
		freezer: AccountId,
	) -> DispatchResult;
}

/// Trait for refunding the deposit of a target asset account.
pub trait Refund<AccountId> {
	/// Means of identifying one asset class from another.
	type AssetId: AssetId;
	/// Scalar type for representing balance of an account.
	type Balance: Balance;
	/// Returns the amount of account deposit and depositor address, if any.
	fn deposit(id: Self::AssetId, who: AccountId) -> Option<(AccountId, Self::Balance)>;
	/// Return the deposit (if any) of a target asset account.
	fn refund(id: Self::AssetId, who: AccountId) -> DispatchResult;
}

/// Trait for providing the ability to destroy existing fungible assets.
pub trait Destroy<AccountId>: Inspect<AccountId> {
	/// Start the destruction an existing fungible asset.
	/// * `id`: The `AssetId` to be destroyed. successfully.
	/// * `maybe_check_owner`: An optional account id that can be used to authorize the destroy
	///   command. If not provided, no authorization checks will be performed before destroying
	///   asset.
	fn start_destroy(id: Self::AssetId, maybe_check_owner: Option<AccountId>) -> DispatchResult;

	/// Destroy all accounts associated with a given asset.
	/// `destroy_accounts` should only be called after `start_destroy` has been called, and the
	/// asset is in a `Destroying` state
	///
	/// * `id`: The identifier of the asset to be destroyed. This must identify an existing asset.
	/// * `max_items`: The maximum number of accounts to be destroyed for a given call of the
	///   function. This value should be small enough to allow the operation fit into a logical
	///   block.
	///
	///	Response:
	/// * u32: Total number of approvals which were actually destroyed
	///
	/// Due to weight restrictions, this function may need to be called multiple
	/// times to fully destroy all approvals. It will destroy `max_items` approvals at a
	/// time.
	fn destroy_accounts(id: Self::AssetId, max_items: u32) -> Result<u32, DispatchError>;
	/// Destroy all approvals associated with a given asset up to the `max_items`
	/// `destroy_approvals` should only be called after `start_destroy` has been called, and the
	/// asset is in a `Destroying` state
	///
	/// * `id`: The identifier of the asset to be destroyed. This must identify an existing asset.
	/// * `max_items`: The maximum number of accounts to be destroyed for a given call of the
	///   function. This value should be small enough to allow the operation fit into a logical
	///   block.
	///
	///	Response:
	/// * u32: Total number of approvals which were actually destroyed
	///
	/// Due to weight restrictions, this function may need to be called multiple
	/// times to fully destroy all approvals. It will destroy `max_items` approvals at a
	/// time.
	fn destroy_approvals(id: Self::AssetId, max_items: u32) -> Result<u32, DispatchError>;

	/// Complete destroying asset and unreserve currency.
	/// `finish_destroy` should only be called after `start_destroy` has been called, and the
	/// asset is in a `Destroying` state. All accounts or approvals should be destroyed before
	/// hand.
	///
	/// * `id`: The identifier of the asset to be destroyed. This must identify an existing asset.
	fn finish_destroy(id: Self::AssetId) -> DispatchResult;
}
