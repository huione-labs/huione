use huione_sdk::pubkey::{Pubkey, PUBKEY_BYTES};

huione_sdk::declare_id!("DomJn7sam48AHBFyGyeDy2iRJ4JwGK8dqevDdf1gATxr");

pub const DOMAIN_ACCOUNT_LENGTH: usize = 80;
pub const DOMAIN_ACCOUNT_OWNER_OFFSET: usize = 34;
pub const DOMAIN_ACCOUNT_TYPE_OFFSET: usize = 0;
pub const DOMAIN_RESOLVE_ACCOUNT_TYPE_OFFSET: usize = 0;
pub const DOMAIN_RESOLVE_ACCOUNT_PARENT_OFFSET: usize = 2;

pub(crate) trait GenericNameAccount {
    fn valid_domain_account_data(account_data: &[u8]) -> bool;

    fn valid_domain_resolve_account_data(account_data: &[u8]) -> bool;

    // Call after account length has already been verified
    fn unpack_domain_account_owner_unchecked(account_data: &[u8]) -> &Pubkey {
        Self::unpack_pubkey_unchecked(account_data, DOMAIN_ACCOUNT_OWNER_OFFSET)
    }

    fn unpack_domain_resolve_account_owner_unchecked(account_data: &[u8]) -> &Pubkey {
        Self::unpack_pubkey_unchecked(account_data, DOMAIN_RESOLVE_ACCOUNT_PARENT_OFFSET)
    }

    // Call after account length has already been verified
    fn unpack_pubkey_unchecked(account_data: &[u8], offset: usize) -> &Pubkey {
        bytemuck::from_bytes(&account_data[offset..offset + PUBKEY_BYTES])
    }

    fn unpack_domain_account_owner(account_data: &[u8]) -> Option<&Pubkey> {
        if Self::valid_domain_account_data(account_data) {
            Some(Self::unpack_domain_account_owner_unchecked(account_data))
        } else {
            None
        }
    }


    fn unpack_domain_resolve_account_parent(account_data: &[u8]) -> Option<&Pubkey> {
        if Self::valid_domain_resolve_account_data(account_data) {
            Some(Self::unpack_domain_resolve_account_owner_unchecked(account_data))
        } else {
            None
        }
    }
}

pub struct Account;
// impl Account {
//     pub fn get_packed_len() -> usize {
//         DOMAIN_ACCOUNT_LENGTH
//     }
// }

impl GenericNameAccount for Account {

    fn valid_domain_account_data(account_data: &[u8]) -> bool {
        account_data[0] == 1
    }

    fn valid_domain_resolve_account_data(account_data: &[u8]) -> bool {
        account_data[0] == 2
    }
}
