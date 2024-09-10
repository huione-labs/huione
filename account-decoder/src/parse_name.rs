use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use hpl_name::state::{AddressResolveAccount, DomainAccount, DomainResolveAccount};
use {
    crate::{
        parse_account_data::{ParsableAccount, ParseAccountError},
    },
    huione_sdk::pubkey::Pubkey,

};
use borsh::{BorshDeserialize};
use chrono::{DateTime, Utc};

fn name_program_id() -> Pubkey {
    Pubkey::new_from_array(hpl_name::id().to_bytes())
}

// Check if the provided program id as a known SPL Token program id
pub fn is_known_name_id(program_id: &Pubkey) -> bool {
    *program_id == name_program_id()
}

pub fn parse_name(
    data: &[u8],
) -> Result<NameAccountType, ParseAccountError> {
    let mut data = data;
    match data[0] {
        // AccountType::Domain
        1 => {
            let domain_account : DomainAccount = DomainAccount::deserialize (&mut data)
                .map_err(|_| ParseAccountError::AccountNotParsable(ParsableAccount::HplName))?;
            let time_now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            if time_now > domain_account.expire_time {
                return Err(ParseAccountError::AccountNotParsable(
                    ParsableAccount::HplName,
                ));
            }
            let duration = Duration::from_secs(domain_account.expire_time as u64);
            let expired_time = DateTime::<Utc>::from(UNIX_EPOCH.add(duration));
            // Formats the combined date and time with the specified format string.
            let ui_expired_time = expired_time.format("%Y-%m-%d %H:%M:%S").to_string();
            Ok(NameAccountType::Domain(UiDomainAccount {
                account_type: domain_account.account_type.to_string(),
                account_state: domain_account.account_state.to_string(),
                parent_key: domain_account.parent_key.to_string(),
                owner: domain_account.owner.to_string(),
                expire_time: ui_expired_time,
                max_space: domain_account.max_space,
                domain_name: domain_account.domain_name
            }))
        }
        // AccountType::DomainResolve
        2 => {
            let resolve_account : DomainResolveAccount = DomainResolveAccount::deserialize (&mut data)
                .map_err(|_| ParseAccountError::AccountNotParsable(ParsableAccount::HplName))?;
            let value_str = if resolve_account.value.is_some() {
                bs58::encode(resolve_account.value.unwrap()).into_string()
            } else { "None".to_string() };
            Ok(NameAccountType::DomainResolve(UiDomainResolveAccount {
                account_type: resolve_account.account_type.to_string(),
                account_state: resolve_account.account_state.to_string(),
                parent_key: resolve_account.parent_key.to_string(),
                value: value_str,
                domain_name: resolve_account.domain_name
            }))
        }
        // AccountType::AddressResolve
        3 => {
            let address_account : AddressResolveAccount = AddressResolveAccount::deserialize (&mut data)
                .map_err(|_| ParseAccountError::AccountNotParsable(ParsableAccount::HplName))?;

            Ok(NameAccountType::AddressResolve(UiAddressResolveAccount {
                account_type: address_account.account_type.to_string(),
                account_state: address_account.account_state.to_string(),

                domain: address_account.domain
            }))
        }
        _ => {
            return Err(ParseAccountError::AccountNotParsable(
                ParsableAccount::HplName,
            ));
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type", content = "info")]
#[allow(clippy::large_enum_variant)]
pub enum NameAccountType {
    Domain(UiDomainAccount),
    DomainResolve(UiDomainResolveAccount),
    AddressResolve(UiAddressResolveAccount),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UiDomainAccount {
    /// Account type
    pub account_type: String,  // 1 byte
    /// Account state
    pub account_state: String, // 1 byte
    /// Parent domain
    pub parent_key: String, // 32 byte
    /// owner
    pub owner: String, // 32 byte
    /// Expire time
    pub expire_time: String, // 8 byte
    /// Domain parse account max space
    pub max_space: u16, // 2 byte
    /// The domain name
    pub domain_name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UiDomainResolveAccount {
    /// Account type
    pub account_type: String,
    /// Account state
    pub account_state: String,
    /// Parent domain
    pub parent_key: String,
    /// The domain value
    pub value: String,
    /// The domain name
    pub domain_name: String,

}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UiAddressResolveAccount {
    /// Account type
    pub account_type: String,
    /// Account state
    pub account_state: String,
    /// The domain name
    pub domain: String,
}



#[cfg(test)]
mod test {
    #[test]
    fn test_parse_token() {
    }
}