use huione_sdk::pubkey::{Pubkey, PUBKEY_BYTES};

huione_sdk::declare_id!("BXjynCN65MQBZCzqiHxPr2W1qCkK2RBiGVTFP9g4PQuh");

pub const NFT_ACCOUNT_LENGTH: usize = 378;
pub const NFT_ACCOUNT_OWNER_OFFSET: usize = 96;

pub(crate) trait GenericNFTAccount {
    fn valid_account_data(account_data: &[u8]) -> bool;

    // Call after account length has already been verified
    fn unpack_account_owner_unchecked(account_data: &[u8]) -> &Pubkey {
        Self::unpack_pubkey_unchecked(account_data, NFT_ACCOUNT_OWNER_OFFSET)
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
}

pub struct Account;
// impl Account {
//     pub fn get_packed_len() -> usize {
//         NFT_ACCOUNT_LENGTH
//     }
// }

impl GenericNFTAccount for Account {
    fn valid_account_data(account_data: &[u8]) -> bool {
        account_data.len() == NFT_ACCOUNT_LENGTH
    }
}
