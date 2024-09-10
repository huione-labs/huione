use {
    jsonrpc_core::{Error, Result},
    huione_account_decoder::{
        parse_account_data::AccountAdditionalData, 
        parse_token::{
            get_token_account_mint,hpl_token_native_mint,hpl_token_native_min_info,hpl_token_native_mint_program_id
        }, UiAccount,
        UiAccountData, UiAccountEncoding,
    },
    huione_rpc_client_api::response::RpcKeyedAccount,
    huione_runtime::bank::Bank,
    huione_sdk::{
        account::{AccountSharedData, ReadableAccount},
        pubkey::Pubkey,
    },
    hpl_token::{huione_program::program_pack::Pack, state::Mint,state::MintMeta},
    std::{collections::HashMap, sync::Arc},
};

pub fn get_parsed_token_account(
    bank: Arc<Bank>,
    pubkey: &Pubkey,
    account: AccountSharedData,
) -> UiAccount {
    let additional_data = get_token_account_mint(account.data(),Some(pubkey))
        .and_then(|mint_pubkey| get_mint_owner_and_decimals(&bank, &mint_pubkey).ok())
        .map(|(_, decimals,symbol,name,icon)| AccountAdditionalData {
            hpl_token_decimals: Some(decimals),
            hpl_token_symbol: Some(symbol),
            hpl_token_name: Some(name),
            hpl_token_icon: Some(icon),
        });

    UiAccount::encode(
        pubkey,
        &account,
        UiAccountEncoding::JsonParsed,
        additional_data,
        None,
    )
}

pub fn get_parsed_token_accounts<I>(
    bank: Arc<Bank>,
    keyed_accounts: I,
) -> impl Iterator<Item = RpcKeyedAccount>
where
    I: Iterator<Item = (Pubkey, AccountSharedData)>,
{
    let mut mint_decimals: HashMap<Pubkey, (u8,String,String,String)> = HashMap::new();
    keyed_accounts.filter_map(move |(pubkey, account)| {
        let additional_data = get_token_account_mint(account.data(),Some(&pubkey)).map(|mint_pubkey| {
            let hpl_token = mint_decimals.get(&mint_pubkey).cloned().or_else(|| {
                let (_, decimals,symbol,name,icon) = get_mint_owner_and_decimals(&bank, &mint_pubkey).ok()?;
                mint_decimals.insert(mint_pubkey, (decimals,symbol.clone(),name.clone(),icon.clone()));
                Some((decimals,symbol,name,icon))
            });

            if let Some(hpl_token) = hpl_token{
                let hpl_token_decimals = Some(hpl_token.0);
                let hpl_token_symbol = Some(hpl_token.1);
                let hpl_token_name = Some(hpl_token.2);
                let hpl_token_icon = Some(hpl_token.3);
                AccountAdditionalData { 
                    hpl_token_decimals,
                    hpl_token_symbol,
                    hpl_token_name,
                    hpl_token_icon
                }
            }else{
                AccountAdditionalData { 
                    hpl_token_decimals: None,
                    hpl_token_symbol: None,
                    hpl_token_name: None,
                    hpl_token_icon: None, 
                }
            }
        });

        let maybe_encoded_account = UiAccount::encode(
            &pubkey,
            &account,
            UiAccountEncoding::JsonParsed,
            additional_data,
            None,
        );
        if let UiAccountData::Json(_) = &maybe_encoded_account.data {
            Some(RpcKeyedAccount {
                pubkey: pubkey.to_string(),
                account: maybe_encoded_account,
            })
        } else {
            None
        }
    })
}

/// Analyze a mint Pubkey that may be the native_mint and get the mint-account owner (token
/// program_id) and decimals
pub fn get_mint_owner_and_decimals(bank: &Arc<Bank>, mint: &Pubkey) -> Result<(Pubkey, u8,String,String,String)> {
    if mint == &hpl_token_native_mint() {

        let mint_info_account = bank.get_account( &hpl_token_native_min_info()).ok_or_else(|| {
            Error::invalid_params("Invalid param: could not find mint".to_string())
        })?;
        let symbol = get_mint_symbol(mint_info_account.data())?;
        let name = get_mint_name(mint_info_account.data())?;
        let icon = get_mint_icon(mint_info_account.data())?;

        Ok((
            hpl_token_native_mint_program_id(),
            hpl_token::native_mint::DECIMALS,
            symbol,
            name,
            icon,
        ))
    } else {
        let mint_account = bank.get_account(mint).ok_or_else(|| {
            Error::invalid_params("Invalid param: could not find mint".to_string())
        })?;

        let decimals = get_mint_decimals(mint_account.data())?;

        let (mint_info ,_) = Pubkey::find_program_address(&[b"MintMeta",&mint.to_bytes()], &hpl_token::id());

        let mint_info_account = bank.get_account(&mint_info);

        match mint_info_account {
            Some(mint_info_account) => {
                let symbol = get_mint_symbol(mint_info_account.data())?;
                let name = get_mint_name(mint_info_account.data())?;
                let icon = get_mint_icon(mint_info_account.data())?;
                Ok((*mint_account.owner(), decimals,symbol,name,icon))
            },
            None => Ok((*mint_account.owner(), decimals,"".to_string(),"".to_string(),"".to_string())),
        }
    }
}

fn get_mint_decimals(data: &[u8]) -> Result<u8> {
    Mint::unpack(data)
        .map_err(|_| {
            Error::invalid_params("Invalid param: Token mint could not be unpacked".to_string())
        })
        .map(|mint| mint.decimals)
}


fn get_mint_symbol(data: &[u8]) -> Result<String> {
    MintMeta::unpack(data)
        .map_err(|_| {
            Error::invalid_params("Invalid param: Token mint could not be unpacked".to_string())
        })
        .map(|mint| mint.symbol.to_string())
}

fn get_mint_name(data: &[u8]) -> Result<String> {
    MintMeta::unpack(data)
        .map_err(|_| {
            Error::invalid_params("Invalid param: Token mint could not be unpacked".to_string())
        })
        .map(|mint| mint.name.to_string())
}


fn get_mint_icon(data: &[u8]) -> Result<String> {
    MintMeta::unpack(data)
        .map_err(|_| {
            Error::invalid_params("Invalid param: Token mint could not be unpacked".to_string())
        })
        .map(|mint| mint.icon.to_string())
}

pub fn get_nft_mint_owner(bank: &Arc<Bank>, mint: &Pubkey) -> Result<Pubkey> {

    let mint_account = bank.get_account(mint).ok_or_else(|| {
        Error::invalid_params("Invalid param: could not find mint".to_string())
    })?;
    Ok(*mint_account.owner())
}

// Universal UiAccounts Conversion Tool
pub fn get_parsed_ui_accounts<I>(
    keyed_accounts: I,
) -> impl Iterator<Item = RpcKeyedAccount>
    where
        I: Iterator<Item = (Pubkey, AccountSharedData)>,
{
    keyed_accounts.filter_map(move |(pubkey, account)| {
        let maybe_encoded_account = UiAccount::encode(
            &pubkey,
            &account,
            UiAccountEncoding::JsonParsed,
            None,
            None,
        );
        if let UiAccountData::Json(_) = &maybe_encoded_account.data {
            Some(RpcKeyedAccount {
                pubkey: pubkey.to_string(),
                account: maybe_encoded_account,
            })
        } else {
            None
        }
    })
}
