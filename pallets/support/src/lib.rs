// This file is part of Acala.

// Copyright (C) 2020-2023 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use codec::{Decode, Encode};
use peaq_primitives_xcm::currency::AssetIds;
use peaq_primitives_xcm::{
	evm::{EvmAddress},
	Balance, CurrencyId,
};
use fp_evm::CallInfo;
use sp_core::H160;
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize},
	DispatchError, DispatchResult, RuntimeDebug,
};
use sp_std::{
	cmp::{Eq, PartialEq},
	prelude::*,
};

use pallet_evm::{InvokeContext, ExecutionMode};
/// Return true if the call of EVM precompile contract is allowed.
pub trait PrecompileCallerFilter {
	fn is_allowed(caller: H160) -> bool;
}

/// Return true if the EVM precompile is paused.
pub trait PrecompilePauseFilter {
	fn is_paused(address: H160) -> bool;
}

// #[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug)]
// pub enum ExecutionMode {
// 	Execute,
// 	/// Discard any state changes
// 	View,
// 	/// Also discard any state changes and use estimate gas mode for evm config
// 	EstimateGas,
// }

// #[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug)]
// pub struct InvokeContext {
// 	pub contract: EvmAddress,
// 	/// similar to msg.sender
// 	pub sender: EvmAddress,
// 	/// similar to tx.origin
// 	pub origin: EvmAddress,
// }

/// An abstraction of EVMBridge
pub trait EVMBridge<AccountId, Balance> {
	/// Execute ERC20.name() to read token name from ERC20 contract
	fn name(context: InvokeContext) -> Result<Vec<u8>, DispatchError>;
	/// Execute ERC20.symbol() to read token symbol from ERC20 contract
	fn symbol(context: InvokeContext) -> Result<Vec<u8>, DispatchError>;
	/// Execute ERC20.decimals() to read token decimals from ERC20 contract
	fn decimals(context: InvokeContext) -> Result<u8, DispatchError>;
	/// Execute ERC20.totalSupply() to read total supply from ERC20 contract
	fn total_supply(context: InvokeContext) -> Result<Balance, DispatchError>;
	/// Execute ERC20.balanceOf(address) to read balance of address from ERC20
	/// contract
	fn balance_of(context: InvokeContext, address: EvmAddress) -> Result<Balance, DispatchError>;
	/// Execute ERC20.transfer(address, uint256) to transfer value to `to`
	fn transfer(context: InvokeContext, to: EvmAddress, value: Balance) -> DispatchResult;
}

#[cfg(feature = "std")]
impl<AccountId, Balance: Default> EVMBridge<AccountId, Balance> for () {
	fn name(_context: InvokeContext) -> Result<Vec<u8>, DispatchError> {
		Err(DispatchError::Other("unimplemented evm bridge"))
	}
	fn symbol(_context: InvokeContext) -> Result<Vec<u8>, DispatchError> {
		Err(DispatchError::Other("unimplemented evm bridge"))
	}
	fn decimals(_context: InvokeContext) -> Result<u8, DispatchError> {
		Err(DispatchError::Other("unimplemented evm bridge"))
	}
	fn total_supply(_context: InvokeContext) -> Result<Balance, DispatchError> {
		Err(DispatchError::Other("unimplemented evm bridge"))
	}
	fn balance_of(_context: InvokeContext, _address: EvmAddress) -> Result<Balance, DispatchError> {
		Err(DispatchError::Other("unimplemented evm bridge"))
	}
	fn transfer(_context: InvokeContext, _to: EvmAddress, _value: Balance) -> DispatchResult {
		Err(DispatchError::Other("unimplemented evm bridge"))
	}
}

/// A mapping between `AccountId` and `EvmAddress`.
pub trait AddressMapping<AccountId> {
	/// Returns the AccountId used go generate the given EvmAddress.
	fn get_account_id(evm: &EvmAddress) -> AccountId;
	/// Returns the EvmAddress associated with a given AccountId or the
	/// underlying EvmAddress of the AccountId.
	/// Returns None if there is no EvmAddress associated with the AccountId
	/// and there is no underlying EvmAddress in the AccountId.
	fn get_evm_address(account_id: &AccountId) -> Option<EvmAddress>;
	/// Returns the EVM address associated with an account ID and generates an
	/// account mapping if no association exists.
	fn get_or_create_evm_address(account_id: &AccountId) -> EvmAddress;
	/// Returns the default EVM address associated with an account ID.
	fn get_default_evm_address(account_id: &AccountId) -> EvmAddress;
	/// Returns true if a given AccountId is associated with a given EvmAddress
	/// and false if is not.
	fn is_linked(account_id: &AccountId, evm: &EvmAddress) -> bool;
}

/// A mapping between AssetId and AssetMetadata.
pub trait AssetIdMapping<ForeignAssetId, MultiLocation, AssetMetadata> {
	/// Returns the AssetMetadata associated with a given `AssetIds`.
	fn get_asset_metadata(asset_ids: AssetIds) -> Option<AssetMetadata>;
	/// Returns the MultiLocation associated with a given ForeignAssetId.
	fn get_multi_location(foreign_asset_id: ForeignAssetId) -> Option<MultiLocation>;
	/// Returns the CurrencyId associated with a given MultiLocation.
	fn get_currency_id(multi_location: MultiLocation) -> Option<CurrencyId>;
}

/// A mapping between u32 and Erc20 address.
/// provide a way to encode/decode for CurrencyId;
pub trait Erc20InfoMapping {
	/// Returns the name associated with a given CurrencyId.
	/// If CurrencyId is CurrencyId::DexShare and contain DexShare::Erc20,
	/// the EvmAddress must have been mapped.
	fn name(currency_id: CurrencyId) -> Option<Vec<u8>>;
	/// Returns the symbol associated with a given CurrencyId.
	/// If CurrencyId is CurrencyId::DexShare and contain DexShare::Erc20,
	/// the EvmAddress must have been mapped.
	fn symbol(currency_id: CurrencyId) -> Option<Vec<u8>>;
	/// Returns the decimals associated with a given CurrencyId.
	/// If CurrencyId is CurrencyId::DexShare and contain DexShare::Erc20,
	/// the EvmAddress must have been mapped.
	fn decimals(currency_id: CurrencyId) -> Option<u8>;
	/// Encode the CurrencyId to EvmAddress.
	/// If is CurrencyId::DexShare and contain DexShare::Erc20,
	/// will use the u32 to get the DexShare::Erc20 from the mapping.
	fn encode_evm_address(v: CurrencyId) -> Option<EvmAddress>;
	/// Decode the CurrencyId from EvmAddress.
	/// If is CurrencyId::DexShare and contain DexShare::Erc20,
	/// will use the u32 to get the DexShare::Erc20 from the mapping.
	fn decode_evm_address(v: EvmAddress) -> Option<CurrencyId>;
}

#[cfg(feature = "std")]
impl Erc20InfoMapping for () {
	fn name(_currency_id: CurrencyId) -> Option<Vec<u8>> {
		None
	}

	fn symbol(_currency_id: CurrencyId) -> Option<Vec<u8>> {
		None
	}

	fn decimals(_currency_id: CurrencyId) -> Option<u8> {
		None
	}

	fn encode_evm_address(_v: CurrencyId) -> Option<EvmAddress> {
		None
	}

	fn decode_evm_address(_v: EvmAddress) -> Option<CurrencyId> {
		None
	}
}

pub mod limits {
	pub struct Limit {
		pub gas: u64,
		pub storage: u32,
	}

	impl Limit {
		pub const fn new(gas: u64, storage: u32) -> Self {
			Self { gas, storage }
		}
	}

	pub mod erc20 {
		use super::*;

		pub const NAME: Limit = Limit::new(100_000, 0);
		pub const SYMBOL: Limit = Limit::new(100_000, 0);
		pub const DECIMALS: Limit = Limit::new(100_000, 0);
		pub const TOTAL_SUPPLY: Limit = Limit::new(100_000, 0);
		pub const BALANCE_OF: Limit = Limit::new(100_000, 0);
		pub const TRANSFER: Limit = Limit::new(200_000, 960);
	}

	pub mod liquidation {
		use super::*;

		pub const LIQUIDATE: Limit = Limit::new(200_000, 1_000);
		pub const ON_COLLATERAL_TRANSFER: Limit = Limit::new(200_000, 1_000);
		pub const ON_REPAYMENT_REFUND: Limit = Limit::new(200_000, 1_000);
	}
}