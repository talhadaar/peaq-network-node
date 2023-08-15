use sp_core::H160;
use hex_literal::hex;
use core::ops::Range;

use crate::{
	currency::{CurrencyId, CurrencyIdType},
};

/// Evm Address.
pub type EvmAddress = H160;

/// Ethereum precompiles
/// 0 - 0x0000000000000000000000000000000000000400
/// Peaq precompiles
/// 0x0000000000000000000000000000000000000400 - 0x0000000000000000000000000000000000000800
/// Each precompile will be installed into the runtime beginning with this address prefix
pub const PRECOMPILE_ADDRESS_START: EvmAddress = H160(hex!("0000000000000000000000000000000000000400"));
/// Predeployed system contracts (except Mirrored ERC20)
/// 0x0000000000000000000000000000000000000800 - 0x0000000000000000000000000000000000001000
pub const PREDEPLOY_ADDRESS_START: EvmAddress = H160(hex!("0000000000000000000000000000000000000800"));
// pub const MIRRORED_TOKENS_ADDRESS_START: EvmAddress = H160(hex!("0000000000000000000100000000000000000000"));
// pub const MIRRORED_NFT_ADDRESS_START: u64 = 0x2000000;
// /// ERC20 Holding Account used for transfer ERC20 token
// pub const ERC20_HOLDING_ACCOUNT: EvmAddress = H160(hex_literal::hex!("000000000000000000ff00000000000000000000"));
/// System contract address prefix
pub const SYSTEM_CONTRACT_ADDRESS_PREFIX: [u8; 9] = [0u8; 9];

/// Check if the given `address` is a system contract.
///
/// It's system contract if the address starts with SYSTEM_CONTRACT_ADDRESS_PREFIX.
pub fn is_system_contract(address: EvmAddress) -> bool {
	address.as_bytes().starts_with(&SYSTEM_CONTRACT_ADDRESS_PREFIX)
}

pub const H160_POSITION_CURRENCY_ID_TYPE: usize = 9;
pub const H160_POSITION_TOKEN: usize = 19;
pub const H160_POSITION_TOKEN_NFT: Range<usize> = 16..20;
pub const H160_POSITION_FOREIGN_ASSET: Range<usize> = 18..20;

/// Generate the EvmAddress from CurrencyId so that evm contracts can call the erc20 contract.
impl TryFrom<CurrencyId> for EvmAddress {
	type Error = ();

	fn try_from(val: CurrencyId) -> Result<Self, Self::Error> {
        let mut address = [0u8; 20];
		match val {
			CurrencyId::Token(token) => {
				address[H160_POSITION_CURRENCY_ID_TYPE] = CurrencyIdType::Token.into();
				address[H160_POSITION_TOKEN] = token.into();
            }
			CurrencyId::Erc20(erc20) => {
				address[..].copy_from_slice(erc20.as_bytes());
			}
            CurrencyId::ForeignAsset(foreign_asset_id) => {
				address[H160_POSITION_CURRENCY_ID_TYPE] = CurrencyIdType::ForeignAsset.into();
				address[H160_POSITION_FOREIGN_ASSET].copy_from_slice(&foreign_asset_id.to_be_bytes());
			}
		};
        Ok(EvmAddress::from_slice(&address))
	}
}