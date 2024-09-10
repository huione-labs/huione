/// Partial HPL Token declarations inlined to avoid an external dependency on the hpl-token-2022 crate
use crate::inline_hpl_token::{self, GenericTokenAccount};

huione_sdk::declare_id!("CgK4eAi5GLCzdvF7f9JEgDqBUkWgYf3bASeZ759mJpbi");

// `hpl_token_program_2022::extension::AccountType::Account` ordinal value
const ACCOUNTTYPE_ACCOUNT: u8 = 2;

pub struct Account;
impl GenericTokenAccount for Account {
    fn valid_account_data(account_data: &[u8]) -> bool {
        inline_hpl_token::Account::valid_account_data(account_data)
            || ACCOUNTTYPE_ACCOUNT
                == *account_data
                    .get(inline_hpl_token::Account::get_packed_len())
                    .unwrap_or(&0)
    }
}
