use crate::currency::CurrencyId;

extern crate alloc;
use alloc::vec::Vec;

/// Evm Address.
pub type EvmAddress = sp_core::H160;

/// Metadata of an ERC20 token.
pub trait Erc20MetadataT {
	/// Returns the name of the token.
	fn name() -> &'static str;

	/// Returns the symbol of the token.
	fn symbol() -> &'static str;

	/// Returns the decimals places of the token.
	fn decimals() -> u8;

	/// Must return `true` only if it represents the main native currency of
	/// the network. It must be the currency used in `pallet_evm`.
	fn is_native_currency() -> bool;
}

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