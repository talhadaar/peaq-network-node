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

// TODO refactor this implementation to balances-erc20's structure
// use super::{
// 	input::{Input, InputPricer, InputT, Output},
// 	target_gas_limit,
// };
// use crate::WeightToGas;
use frame_support::{
	log,
	traits::{Currency, Get},
};
use precompile_utils::prelude::*;
use sp_core::{uint, U256};

use orml_currencies::WeightInfo;

// TODO move GasWeightMapping to a common crate
// pub struct PeaqGasWeightMapping;
// impl pallet_evm::GasWeightMapping for PeaqGasWeightMapping {
// 	fn gas_to_weight(gas: u64, _without_base_weight: bool) -> Weight {
// 		Weight::from_ref_time(gas.saturating_mul(WEIGHT_PER_GAS))
// 	}

// 	fn weight_to_gas(weight: Weight) -> u64 {
// 		weight.ref_time().wrapping_div(WEIGHT_PER_GAS)
// 	}
// }

// RESEARCH what it do? - reexports stuff from pallet_evm
// use pallet_evm::{
// 	precompiles::Precompile,
// 	runner::state::{PrecompileFailure, PrecompileOutput, PrecompileResult},
// 	Context, ExitError, ExitRevert, ExitSucceed,
// };

// // RESEARCH why is this needed?
// use module_support::Erc20InfoMapping as Erc20InfoMappingT;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use orml_traits::MultiCurrency as MultiCurrencyT;

// what it do?
use frame_support::sp_runtime::{traits::Convert, RuntimeDebug};
use peaq_primitives_xcm::{currency::CurrencyId, evm::EvmAddress, Balance};
use sp_core::H160;
use sp_std::{marker::PhantomData, prelude::*};
// TODO implement this
/// A mapping between u32 and Erc20 address.
/// provide a way to encode/decode for CurrencyId;
pub trait Erc20InfoMappingT {
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

// TODO implement this somewhere
#[cfg(feature = "std")]
impl Erc20InfoMappingT for () {
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

pub struct Erc20InfoMapping;
impl Erc20InfoMappingT for Erc20InfoMapping {
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
// /// The `MultiCurrency` impl precompile.
// ///
// ///
// /// `input` data starts with `action` and `currency_id`.
// ///
// /// Actions:
// /// - Query total issuance.
// /// - Query balance. Rest `input` bytes: `account_id`.
// /// - Transfer. Rest `input` bytes: `from`, `to`, `amount`.

// WIP refactoring here

use pallet_evm::AddressMapping;
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
		let currency_id = Erc20InfoMapping::decode_evm_address(handle.context().caller).unwrap();

		Ok(Erc20InfoMapping::name(currency_id).unwrap().into())
	}

	#[precompile::public("symbol()")]
	#[precompile::view]
	fn symbol(handle: &mut impl PrecompileHandle) -> EvmResult<UnboundedBytes> {
		handle.record_cost(RuntimeHelper::<Runtime>::db_read_gas_cost())?;
		let currency_id = Erc20InfoMapping::decode_evm_address(handle.context().caller).unwrap();

		Ok(Erc20InfoMapping::symbol(currency_id).unwrap().into())
	}

	#[precompile::public("decimals()")]
	#[precompile::view]
	fn decimals(handle: &mut impl PrecompileHandle) -> EvmResult<u8> {
		handle.record_cost(RuntimeHelper::<Runtime>::db_read_gas_cost())?;
		let currency_id = Erc20InfoMapping::decode_evm_address(handle.context().caller).unwrap();

		Ok(Erc20InfoMapping::decimals(currency_id).unwrap())
	}

	#[precompile::public("total_issuance()")]
	#[precompile::view]
	fn total_issuance(handle: &mut impl PrecompileHandle) -> EvmResult<Balance> {
		handle.record_cost(RuntimeHelper::<Runtime>::db_read_gas_cost())?;
		let currency_id = Erc20InfoMapping::decode_evm_address(handle.context().caller).unwrap();

		// let total_issuance =
		// 	<Runtime as orml_currencies::Config>::MultiCurrency::total_issuance(currency_id);

		Ok(Balance::default())
	}

	#[precompile::public("balanceOf(address)")]
	#[precompile::view]
	fn balance(handle: &mut impl PrecompileHandle, who: Address) -> EvmResult<Balance> {
		handle.record_cost(RuntimeHelper::<Runtime>::db_read_gas_cost())?;
		let who: H160 = who.into();
		let who: Runtime::AccountId = Runtime::AddressMapping::into_account_id(who);

		let currency_id = Erc20InfoMapping::decode_evm_address(handle.context().caller).unwrap();

		// let balance = if currency_id == <Runtime as
		// 			orml_currencies::Config>::GetNativeCurrencyId::get(){
		// 						return <Runtime as pallet_evm::Config>::Currency::free_balance(&who)
		// 					} else {
		// 						return <Runtime as orml_currencies::Config>::MultiCurrency::total_balance(currency_id,
		// &who) 				};

		Ok(Balance::default())
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

		let currency_id = Erc20InfoMapping::decode_evm_address(handle.context().caller).unwrap();

		<orml_currencies::Pallet<Runtime> as MultiCurrencyT<Runtime::AccountId>>::transfer(
			currency_id,
			&from,
			&to,
			amount,
		)
		.unwrap();

		Ok(bool::default())
	}
}
// 			Action::Transfer => {
// 				let from = input.account_id_at(1)?;
// 				let to = input.account_id_at(2)?;
// 				let amount = input.balance_at(3)?;
// 				log::debug!(target: "evm", "multicurrency: transfer from: {:?}, to: {:?}, amount: {:?}", from,
// to, amount);

// 				<orml_currencies::Pallet<Runtime> as MultiCurrencyT<Runtime::AccountId>>::transfer(
// 					currency_id,
// 					&from,
// 					&to,
// 					amount,
// 				)
// 				.map_err(|e| PrecompileFailure::Revert {
// 					exit_status: ExitRevert::Reverted,
// 					output: Output::encode_error_msg("Multicurrency Transfer failed", e),
// 					cost: target_gas_limit(target_gas).unwrap_or_default(),
// 				})?;

// 				Ok(PrecompileOutput {
// 					exit_status: ExitSucceed::Returned,
// 					cost: gas_cost,
// 					output: vec![],
// 					logs: Default::default(),
// 				})
// 			}
// 		}
// 	}
// }

// struct Pricer<R>(PhantomData<R>);

// impl<Runtime> Pricer<Runtime>
// where
// 	Runtime:
// 		orml_currencies::Config + pallet_evm::Config + module_prices::Config +
// pallet_transaction_payment::Config, {
// 	const BASE_COST: u64 = 200;

// 	fn cost(
// 		input: &Input<
// 			Action,
// 			Runtime::AccountId,
// 			<Runtime as pallet_evm::Config>::AddressMapping,
// 			Runtime::Erc20InfoMapping,
// 		>,
// 		currency_id: CurrencyId,
// 	) -> Result<u64, PrecompileFailure> {
// 		let action = input.action()?;

// 		// Decode CurrencyId from EvmAddress
// 		let read_currency = InputPricer::<Runtime>::read_currency(currency_id);

// 		let cost = match action {
// 			Action::QueryName | Action::QuerySymbol | Action::QueryDecimals => Self::erc20_info(currency_id),
// 			Action::QueryTotalIssuance => {
// 				// Currencies::TotalIssuance (r: 1)
// 				WeightToGas::convert(<Runtime as frame_system::Config>::DbWeight::get().reads(1))
// 			}
// 			Action::QueryBalance => {
// 				let cost = InputPricer::<Runtime>::read_accounts(1);
// 				// Currencies::Balance (r: 1)
// 				cost.saturating_add(WeightToGas::convert(
// 					<Runtime as frame_system::Config>::DbWeight::get().reads(2),
// 				))
// 			}
// 			Action::Transfer => {
// 				let cost = InputPricer::<Runtime>::read_accounts(2);

// 				// transfer weight
// 				let weight = if currency_id == <Runtime as
// pallet_transaction_payment::Config>::GetNativeCurrencyId::get() 				{
// 					<Runtime as orml_currencies::Config>::WeightInfo::transfer_native_currency()
// 				} else {
// 					<Runtime as orml_currencies::Config>::WeightInfo::transfer_non_native_currency()
// 				};

// 				cost.saturating_add(WeightToGas::convert(weight))
// 			}
// 		};

// 		Ok(Self::BASE_COST.saturating_add(read_currency).saturating_add(cost))
// 	}

// 	fn erc20_info(currency_id: CurrencyId) -> u64 {
// 		match currency_id {
// 			CurrencyId::Erc20(_) => {
// 				WeightToGas::convert(Runtime::DbWeight::get().reads(1))
// 			}
// 			_ => Self::BASE_COST,
// 		}
// 	}
// }

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	use crate::precompile::mock::{
// 		aca_evm_address, alice, ausd_evm_address, bob, erc20_address_not_exists, lp_aca_ausd_evm_address,
// new_test_ext, 		Balances, Test,
// 	};
// 	use frame_support::assert_noop;
// 	use hex_literal::hex;

// 	type MultiCurrencyPrecompile = crate::MultiCurrencyPrecompile<Test>;

// 	#[test]
// 	fn handles_invalid_currency_id() {
// 		new_test_ext().execute_with(|| {
// 			// call with not exists erc20
// 			let context = Context {
// 				address: Default::default(),
// 				caller: erc20_address_not_exists(),
// 				apparent_value: Default::default(),
// 			};

// 			// symbol() -> 0x95d89b41
// 			let input = hex! {"
// 				95d89b41
// 			"};

// 			assert_noop!(
// 				MultiCurrencyPrecompile::execute(&input, Some(10_000), &context, false),
// 				PrecompileFailure::Revert {
// 					exit_status: ExitRevert::Reverted,
// 					output: "invalid currency id".into(),
// 					cost: target_gas_limit(Some(10_000)).unwrap(),
// 				}
// 			);
// 		});
// 	}

// 	#[test]
// 	fn name_works() {
// 		new_test_ext().execute_with(|| {
// 			let mut context = Context {
// 				address: Default::default(),
// 				caller: Default::default(),
// 				apparent_value: Default::default(),
// 			};

// 			// name() -> 0x06fdde03
// 			let input = hex! {"
// 				06fdde03
// 			"};

// 			// Token
// 			context.caller = aca_evm_address();

// 			let expected_output = hex! {"
// 				0000000000000000000000000000000000000000000000000000000000000020
// 				0000000000000000000000000000000000000000000000000000000000000005
// 				4163616c61000000000000000000000000000000000000000000000000000000
// 			"};

// 			let resp = MultiCurrencyPrecompile::execute(&input, None, &context, false).unwrap();
// 			assert_eq!(resp.exit_status, ExitSucceed::Returned);
// 			assert_eq!(resp.output, expected_output.to_vec());

// 			// DexShare
// 			context.caller = lp_aca_ausd_evm_address();

// 			let expected_output = hex! {"
// 				0000000000000000000000000000000000000000000000000000000000000020
// 				0000000000000000000000000000000000000000000000000000000000000017
// 				4c50204163616c61202d204163616c6120446f6c6c6172000000000000000000
// 			"};

// 			let resp = MultiCurrencyPrecompile::execute(&input, None, &context, false).unwrap();
// 			assert_eq!(resp.exit_status, ExitSucceed::Returned);
// 			assert_eq!(resp.output, expected_output.to_vec());
// 		});
// 	}

// #[test]
// fn symbol_works() {
// 	new_test_ext().execute_with(|| {
// 		let mut context = Context {
// 			address: Default::default(),
// 			caller: Default::default(),
// 			apparent_value: Default::default(),
// 		};

// 		// symbol() -> 0x95d89b41
// 		let input = hex! {"
// 			95d89b41
// 		"};

// 		// Token
// 		context.caller = aca_evm_address();

// 		let expected_output = hex! {"
// 			0000000000000000000000000000000000000000000000000000000000000020
// 			0000000000000000000000000000000000000000000000000000000000000003
// 			4143410000000000000000000000000000000000000000000000000000000000
// 		"};

// 		let resp = MultiCurrencyPrecompile::execute(&input, None, &context, false).unwrap();
// 		assert_eq!(resp.exit_status, ExitSucceed::Returned);
// 		assert_eq!(resp.output, expected_output.to_vec());

// 		// DexShare
// 		context.caller = lp_aca_ausd_evm_address();

// 		let expected_output = hex! {"
// 			0000000000000000000000000000000000000000000000000000000000000020
// 			000000000000000000000000000000000000000000000000000000000000000b
// 			4c505f4143415f41555344000000000000000000000000000000000000000000
// 		"};

// 		let resp = MultiCurrencyPrecompile::execute(&input, None, &context, false).unwrap();
// 		assert_eq!(resp.exit_status, ExitSucceed::Returned);
// 		assert_eq!(resp.output, expected_output.to_vec());
// 	});
// }

// #[test]
// fn decimals_works() {
// 	new_test_ext().execute_with(|| {
// 		let mut context = Context {
// 			address: Default::default(),
// 			caller: Default::default(),
// 			apparent_value: Default::default(),
// 		};

// 		// decimals() -> 0x313ce567
// 		let input = hex! {"
// 			313ce567
// 		"};

// 		// Token
// 		context.caller = aca_evm_address();

// 		let expected_output = hex! {"
// 			00000000000000000000000000000000 0000000000000000000000000000000c
// 		"};

// 		let resp = MultiCurrencyPrecompile::execute(&input, None, &context, false).unwrap();
// 		assert_eq!(resp.exit_status, ExitSucceed::Returned);
// 		assert_eq!(resp.output, expected_output.to_vec());

// 		// DexShare
// 		context.caller = lp_aca_ausd_evm_address();

// 		let resp = MultiCurrencyPrecompile::execute(&input, None, &context, false).unwrap();
// 		assert_eq!(resp.exit_status, ExitSucceed::Returned);
// 		assert_eq!(resp.output, expected_output.to_vec());
// 	});
// }

// #[test]
// fn total_supply_works() {
// 	new_test_ext().execute_with(|| {
// 		let mut context = Context {
// 			address: Default::default(),
// 			caller: Default::default(),
// 			apparent_value: Default::default(),
// 		};

// 		// totalSupply() -> 0x18160ddd
// 		let input = hex! {"
// 			18160ddd
// 		"};

// 		// Token
// 		context.caller = ausd_evm_address();

// 		// 2_000_000_000
// 		let expected_output = hex! {"
// 			00000000000000000000000000000000 00000000000000000000000077359400
// 		"};

// 		let resp = MultiCurrencyPrecompile::execute(&input, None, &context, false).unwrap();
// 		assert_eq!(resp.exit_status, ExitSucceed::Returned);
// 		assert_eq!(resp.output, expected_output.to_vec());

// 		// DexShare
// 		context.caller = lp_aca_ausd_evm_address();

// 		let expected_output = hex! {"
// 			00000000000000000000000000000000 00000000000000000000000000000000
// 		"};

// 		let resp = MultiCurrencyPrecompile::execute(&input, None, &context, false).unwrap();
// 		assert_eq!(resp.exit_status, ExitSucceed::Returned);
// 		assert_eq!(resp.output, expected_output.to_vec());
// 	});
// }

// #[test]
// fn balance_of_works() {
// 	new_test_ext().execute_with(|| {
// 		let mut context = Context {
// 			address: Default::default(),
// 			caller: Default::default(),
// 			apparent_value: Default::default(),
// 		};

// 		// balanceOf(address) -> 0x70a08231
// 		// account
// 		let input = hex! {"
// 			70a08231
// 			000000000000000000000000 1000000000000000000000000000000000000001
// 		"};

// 		// Token
// 		context.caller = aca_evm_address();

// 		// INITIAL_BALANCE = 1_000_000_000_000
// 		let expected_output = hex! {"
// 			00000000000000000000000000000000 0000000000000000000000e8d4a51000
// 		"};

// 		let resp = MultiCurrencyPrecompile::execute(&input, None, &context, false).unwrap();
// 		assert_eq!(resp.exit_status, ExitSucceed::Returned);
// 		assert_eq!(resp.output, expected_output.to_vec());

// 		// DexShare
// 		context.caller = lp_aca_ausd_evm_address();

// 		let expected_output = hex! {"
// 			00000000000000000000000000000000 00000000000000000000000000000000
// 		"};

// 		let resp = MultiCurrencyPrecompile::execute(&input, None, &context, false).unwrap();
// 		assert_eq!(resp.exit_status, ExitSucceed::Returned);
// 		assert_eq!(resp.output, expected_output.to_vec());
// 	})
// }

// #[test]
// fn transfer_works() {
// 	new_test_ext().execute_with(|| {
// 		let mut context = Context {
// 			address: Default::default(),
// 			caller: Default::default(),
// 			apparent_value: Default::default(),
// 		};

// 		// transfer(address,address,uint256) -> 0xbeabacc8
// 		// from
// 		// to
// 		// amount
// 		let input = hex! {"
// 			beabacc8
// 			000000000000000000000000 1000000000000000000000000000000000000001
// 			000000000000000000000000 1000000000000000000000000000000000000002
// 			00000000000000000000000000000000 00000000000000000000000000000001
// 		"};

// 		let from_balance = Balances::free_balance(alice());
// 		let to_balance = Balances::free_balance(bob());

// 		// Token
// 		context.caller = aca_evm_address();

// 		let resp = MultiCurrencyPrecompile::execute(&input, None, &context, false).unwrap();
// 		assert_eq!(resp.exit_status, ExitSucceed::Returned);
// 		assert_eq!(resp.output, [0u8; 0].to_vec());

// 		assert_eq!(Balances::free_balance(alice()), from_balance - 1);
// 		assert_eq!(Balances::free_balance(bob()), to_balance + 1);

// 		// DexShare
// 		context.caller = lp_aca_ausd_evm_address();
// 		assert_noop!(
// 			MultiCurrencyPrecompile::execute(&input, Some(100_000), &context, false),
// 			PrecompileFailure::Revert {
// 				exit_status: ExitRevert::Reverted,
// 				output: "Multicurrency Transfer failed: BalanceTooLow".into(),
// 				cost: target_gas_limit(Some(100_000)).unwrap(),
// 			}
// 		);
// 	})
// }
// }
