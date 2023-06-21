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

use sp_core::{uint, U256};
use sp_core::H160;
use sp_std::{marker::PhantomData, prelude::*};

use precompile_utils::prelude::*;
use peaq_primitives_xcm::{currency::CurrencyId, evm::{EvmAddress, Erc20InfoMappingT}, Balance};

use fp_evm::PrecompileHandle;

// frame imports
use frame_support::{
	log,
	traits::{Currency, Get},
};
use frame_support::sp_runtime::{traits::Convert, RuntimeDebug};
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

		let total_issuance =
			<Runtime as orml_currencies::Config>::MultiCurrency::total_issuance(currency_id);

		Ok(Balance::default())
	}

}