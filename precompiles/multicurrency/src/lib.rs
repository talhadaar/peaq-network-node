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

// primitives and utils imports
use num_enum::{IntoPrimitive, TryFromPrimitive};

use sp_core::{uint, H160, U256};
use sp_std::{marker::PhantomData, prelude::*};

use peaq_primitives_xcm::{
	currency::CurrencyId,
	evm::{Erc20InfoMappingT, EvmAddress},
	Balance,
};
use precompile_utils::prelude::*;

use fp_evm::PrecompileHandle;

// frame imports
use frame_support::{
	log,
	sp_runtime::{traits::Convert, RuntimeDebug},
	traits::{Currency, Get},
};
use pallet_evm::AddressMapping;

// orml imports
use orml_currencies::WeightInfo;
use orml_traits::MultiCurrency as MultiCurrencyT;

// /// The `MultiCurrency` impl precompile.
// ///
// ///
// /// `input` data starts with `action` and `currency_id`.
// ///
// /// Actions:
// /// - Query total issuance.
// /// - Query balance. Rest `input` bytes: `account_id`.
// /// - Transfer. Rest `input` bytes: `from`, `to`, `amount`.
pub struct MultiCurrencyPrecompile<Runtime, Metadata: Erc20InfoMappingT>(
	PhantomData<(Runtime, Metadata)>,
);

#[precompile_utils::precompile]
impl<Runtime, Metadata> MultiCurrencyPrecompile<Runtime, Metadata>
where
	Metadata: Erc20InfoMappingT,
	Runtime: pallet_evm::Config + orml_currencies::Config,
	orml_currencies::Pallet<Runtime>:
		MultiCurrencyT<Runtime::AccountId, CurrencyId = CurrencyId, Balance = Balance>,
{
	#[precompile::public("name()")]
	#[precompile::view]
	fn name(handle: &mut impl PrecompileHandle) -> EvmResult<UnboundedBytes> {
		handle.record_cost(RuntimeHelper::<Runtime>::db_read_gas_cost())?;
		let currency_id = Metadata::decode_evm_address(handle.context().caller).unwrap();

		Ok(Metadata::name(currency_id).unwrap().into())
	}

	#[precompile::public("symbol()")]
	#[precompile::view]
	fn symbol(handle: &mut impl PrecompileHandle) -> EvmResult<UnboundedBytes> {
		handle.record_cost(RuntimeHelper::<Runtime>::db_read_gas_cost())?;
		let currency_id = Metadata::decode_evm_address(handle.context().caller).unwrap();

		Ok(Metadata::symbol(currency_id).unwrap().into())
	}

	#[precompile::public("decimals()")]
	#[precompile::view]
	fn decimals(handle: &mut impl PrecompileHandle) -> EvmResult<u8> {
		handle.record_cost(RuntimeHelper::<Runtime>::db_read_gas_cost())?;
		let currency_id = Metadata::decode_evm_address(handle.context().caller).unwrap();

		Ok(Metadata::decimals(currency_id).unwrap())
	}

	#[precompile::public("total_issuance()")]
	#[precompile::view]
	fn total_issuance(handle: &mut impl PrecompileHandle) -> EvmResult<Balance> {
		handle.record_cost(RuntimeHelper::<Runtime>::db_read_gas_cost())?;
		let currency_id = Metadata::decode_evm_address(handle.context().caller).unwrap();

		let total_issuance = <orml_currencies::Pallet<Runtime> as MultiCurrencyT<
			Runtime::AccountId,
		>>::total_issuance(currency_id);

		Ok(total_issuance)
	}

	#[precompile::public("balanceOf(address)")]
	#[precompile::view]
	fn balance(handle: &mut impl PrecompileHandle, who: Address) -> EvmResult<Balance> {
		handle.record_cost(RuntimeHelper::<Runtime>::db_read_gas_cost())?;
		let who: H160 = who.into();
		let who: Runtime::AccountId = Runtime::AddressMapping::into_account_id(who);

		let currency_id = Metadata::decode_evm_address(handle.context().caller).unwrap();

		// TODO what if currency in question is native currency
		let balance =
			<orml_currencies::Pallet<Runtime> as MultiCurrencyT<Runtime::AccountId>>::total_balance(
				currency_id,
				&who,
			);

		Ok(balance)
	}

	#[precompile::public("transfer(address, address, uint256)")]
	fn transfer(
		handle: &mut impl PrecompileHandle,
		from: Address,
		to: Address,
		amount: U256,
	) -> EvmResult<bool> {
		handle.record_cost(RuntimeHelper::<Runtime>::db_read_gas_cost())?;
		let from: H160 = from.into();
		let from: Runtime::AccountId = Runtime::AddressMapping::into_account_id(from);

		let to: H160 = to.into();
		let to: Runtime::AccountId = Runtime::AddressMapping::into_account_id(to);
		let amount: Balance = amount.try_into().unwrap();

		let currency_id = Metadata::decode_evm_address(handle.context().caller).unwrap();

		<orml_currencies::Pallet<Runtime> as MultiCurrencyT<Runtime::AccountId>>::transfer(
			currency_id,
			&from,
			&to,
			amount,
		)
		.unwrap();

		Ok(true)
	}
}
