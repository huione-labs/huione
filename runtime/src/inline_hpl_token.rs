/// Partial HPL Token declarations inlined to avoid an external dependency on the hpl-token crate
use huione_sdk::pubkey::{Pubkey, PUBKEY_BYTES};

huione_sdk::declare_id!("HuiToken11111111111111111111111111111111111");

// pub(crate) mod new_token_program {
//     huione_sdk::declare_id!("4TBARuunXmGk7pBmzWEvgQMLq5fkmZ2XLwmKS2w5AKod");
// }

/*
    hpl_token::state::Account {
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: COption<Pubkey>,
        state: AccountState,
        is_native: COption<u64>,
        delegated_amount: u64,
        close_authority: COption<Pubkey>,
    }
*/
pub const HPL_TOKEN_ACCOUNT_MINT_OFFSET: usize = 0;
pub const HPL_TOKEN_ACCOUNT_OWNER_OFFSET: usize = 32;
const HPL_TOKEN_ACCOUNT_LENGTH: usize = 165+16+8;

pub(crate) trait GenericTokenAccount {
    fn valid_account_data(account_data: &[u8]) -> bool;

    // Call after account length has already been verified
    fn unpack_account_owner_unchecked(account_data: &[u8]) -> &Pubkey {
        Self::unpack_pubkey_unchecked(account_data, HPL_TOKEN_ACCOUNT_OWNER_OFFSET)
    }

    // Call after account length has already been verified
    fn unpack_account_mint_unchecked(account_data: &[u8]) -> &Pubkey {
        Self::unpack_pubkey_unchecked(account_data, HPL_TOKEN_ACCOUNT_MINT_OFFSET)
    }

    // Call after account length has already been verified
    fn unpack_pubkey_unchecked(account_data: &[u8], offset: usize) -> &Pubkey {
        bytemuck::from_bytes(&account_data[offset..offset + PUBKEY_BYTES])
    }

    fn unpack_account_owner(account_data: &[u8]) -> Option<&Pubkey> {
        if Self::valid_account_data(account_data) {
            Some(Self::unpack_account_owner_unchecked(account_data))
        } else {
            None
        }
    }

    fn unpack_account_mint(account_data: &[u8]) -> Option<&Pubkey> {
        if Self::valid_account_data(account_data) {
            Some(Self::unpack_account_mint_unchecked(account_data))
        } else {
            None
        }
    }
}

pub struct Account;
impl Account {
    pub fn get_packed_len() -> usize {
        HPL_TOKEN_ACCOUNT_LENGTH
    }
}

impl GenericTokenAccount for Account {
    fn valid_account_data(account_data: &[u8]) -> bool {
        account_data.len() == HPL_TOKEN_ACCOUNT_LENGTH
    }
}

pub mod native_mint {
    huione_sdk::declare_id!("Hc11111111111111111111111111111111111111111");

    /*
        Mint {
            mint_authority: COption::None,
            supply: 0,
            decimals: 9,
            is_initialized: true,
            freeze_authority: COption::None,
            symbol: TokenSymbol::from_str(hpl_token::native_mint::SYMBOL).unwrap(),
            name: TokenName::from_str(hpl_token::native_mint::NAME).unwrap(),
            icon: TokenIcon::from_str(hpl_token::native_mint::ICON).unwrap(),
        }
    */

    pub const ACCOUNT_DATA: [u8; 90] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        9, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0
    ];

}

pub mod native_mint_info {
    huione_sdk::declare_id!("HcMeta1111111111111111111111111111111111111");
    
    pub const ACCOUNT_DATA: [u8; 205] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 72, 67, 10, 87, 114, 97, 112, 32, 72, 67, 10, 104, 116, 116, 
    112, 115, 58, 47, 47, 112, 117, 98, 99, 104, 97, 105, 110, 45, 105, 99, 111, 110, 46, 120, 111, 110, 101, 
    46, 108, 97, 47, 97, 54, 114, 57, 122, 122, 49, 56, 53, 120, 101, 118, 51, 119, 57, 100, 103, 121, 108, 98, 
    114, 119, 98, 99, 120, 105, 52, 48, 118, 50, 119, 104, 46, 112, 110, 103, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0];
}
