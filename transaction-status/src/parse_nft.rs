use {
    crate::parse_instruction::{
        check_num_accounts, ParsableProgram, ParseInstructionError, ParsedInstructionEnum,
    },
    hpl_nft::{
        instruction::{
            AuthorityType, InitializeMintArgs, SetAuthorityArgs, TokenInstruction, UpdateType,MintNftArgs
        },
    },
    huione_sdk::{
        instruction::CompiledInstruction,
        message::AccountKeys,
    },
    serde_json::{json, Map, Value},
};

pub fn parse_nft(
    instruction: &CompiledInstruction,
    account_keys: &AccountKeys,
) -> Result<ParsedInstructionEnum, ParseInstructionError> {
    let token_instruction = TokenInstruction::deserialize(&instruction.data)
        .map_err(|_| ParseInstructionError::InstructionNotParsable(ParsableProgram::HplNft))?;
    match instruction.accounts.iter().max() {
        Some(index) if (*index as usize) < account_keys.len() => {}
        _ => {
            // Runtime should prevent this from ever happening
            return Err(ParseInstructionError::InstructionKeyMismatch(
                ParsableProgram::HplNft,
            ));
        }
    }
    match token_instruction {
        TokenInstruction::Transfer => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "transfer".to_string(),
                info: json!({
                    "from": account_keys[instruction.accounts[0] as usize].to_string(),
                    "to": account_keys[instruction.accounts[1] as usize].to_string(),
                    "nft": account_keys[instruction.accounts[2] as usize].to_string(),
                }),
            })
        }
        TokenInstruction::InitializeMint(args) => {
            check_num_token_accounts(&instruction.accounts, 4)?;
            let InitializeMintArgs {
                total_supply,
                mint_authority,
                freeze_authority,
                name,
                symbol,
                icon_uri,
            } = args;
            let value = json!({
                "mint":account_keys[instruction.accounts[0] as usize].to_string(),
                "iconUri":icon_uri,
                "totalSupply":total_supply,
                "mintAuthority":mint_authority.to_string(),
                "freezeAuthority":freeze_authority.map(|a| a.to_string()),
                "name":name,
                "symbol":symbol,
                "rentSysvar": account_keys[instruction.accounts[3] as usize].to_string(),
            });
            Ok(ParsedInstructionEnum {
                instruction_type: "initializeMint".to_string(),
                info: value,
            })
        }
        TokenInstruction::MintTo (args) => {
            check_num_token_accounts(&instruction.accounts, 7)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "mintTo".to_string(),
                info: json!({
                    "collection_mint_authority": account_keys[instruction.accounts[0] as usize].to_string(),
                    "nft_author": account_keys[instruction.accounts[1] as usize].to_string(),
                    "nft_proposal": account_keys[instruction.accounts[2] as usize].to_string(),
                    "collection": account_keys[instruction.accounts[3] as usize].to_string(),
                    "nft": account_keys[instruction.accounts[4] as usize].to_string(),
                    "system_program": account_keys[instruction.accounts[5] as usize].to_string(),
                    "rent": account_keys[instruction.accounts[6] as usize].to_string(),
                    "nft-uri":args.nft_uri,
                    "nft-owner":args.nft_uri,
                    "nft-time":args.time_stamp,
                }),
            })
        }
        TokenInstruction::Update(update_type) => {
            check_num_token_accounts(&instruction.accounts, 2)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "update".to_string(),
                info: json!({
                    "nft": account_keys[instruction.accounts[0] as usize].to_string(),
                    "owner": account_keys[instruction.accounts[1] as usize].to_string(),
                    "update": match update_type {
                        UpdateType::Icon {icon_uri} =>{
                            json!({
                                "type":"icon",
                                "uri":icon_uri,
                            })
                        }
                        UpdateType::NftAsset { nft_uri }=>{
                            json!({
                                "type":"asset",
                                "uri":nft_uri,
                            })
                        }
                    },
                }),
            })
        }
        TokenInstruction::Freeze => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "freeze".to_string(),
                info: json!({
                    "nft": account_keys[instruction.accounts[0] as usize].to_string(),
                    "authority": account_keys[instruction.accounts[1] as usize].to_string(),
                    "mint": account_keys[instruction.accounts[2] as usize].to_string(),
                }),
            })
        }
        TokenInstruction::Thaw => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "thaw".to_string(),
                info: json!({
                    "nft": account_keys[instruction.accounts[0] as usize].to_string(),
                    "authority": account_keys[instruction.accounts[1] as usize].to_string(),
                    "mint": account_keys[instruction.accounts[2] as usize].to_string(),
                }),
            })
        }
        TokenInstruction::SetAuthority(args) => {
            check_num_token_accounts(&instruction.accounts, 2)?;
            let SetAuthorityArgs {
                authority_type,
                new_authority,
            } = args;
            Ok(ParsedInstructionEnum {
                instruction_type: "setAuthorityArgs".to_string(),
                info: json!({
                    "authority": account_keys[instruction.accounts[0] as usize].to_string(),
                    "newAuthority": match new_authority {
                        Some(new)=>{
                            json!(new.to_string())
                        }
                        None=>{
                            Value::Null
                        }
                    },
                    "authority_type":match authority_type {
                        AuthorityType::MintTokens=>{
                            json!("MintTokens")
                        }
                        AuthorityType::FreezeAccount=>{
                            json!("FreezeAccount")
                        }
                        AuthorityType::CloseAccount=>{
                            json!("CloseAccount")
                        }
                    },
                    "owner": account_keys[instruction.accounts[1] as usize].to_string(),
                }),
            })
        }
        TokenInstruction::Burn => {
            check_num_token_accounts(&instruction.accounts, 2)?;

            Ok(ParsedInstructionEnum {
                instruction_type: "burn".to_string(),
                info: json!({
                    "nft": account_keys[instruction.accounts[0] as usize].to_string(),
                    "authority": account_keys[instruction.accounts[1] as usize].to_string(),
                }),
            })
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum UiAuthorityType {
    MintTokens,
    FreezeAccount,
    AccountOwner,
    CloseAccount,
}

impl From<AuthorityType> for UiAuthorityType {
    fn from(authority_type: AuthorityType) -> Self {
        match authority_type {
            AuthorityType::MintTokens => UiAuthorityType::MintTokens,
            AuthorityType::FreezeAccount => UiAuthorityType::FreezeAccount,
            AuthorityType::CloseAccount => UiAuthorityType::CloseAccount,
        }
    }
}

fn parse_signers(
    map: &mut Map<String, Value>,
    last_nonsigner_index: usize,
    account_keys: &AccountKeys,
    accounts: &[u8],
    owner_field_name: &str,
    multisig_field_name: &str,
) {
    if accounts.len() > last_nonsigner_index + 1 {
        let mut signers: Vec<String> = vec![];
        for i in accounts[last_nonsigner_index + 1..].iter() {
            signers.push(account_keys[*i as usize].to_string());
        }
        map.insert(
            multisig_field_name.to_string(),
            json!(account_keys[accounts[last_nonsigner_index] as usize].to_string()),
        );
        map.insert("signers".to_string(), json!(signers));
    } else {
        map.insert(
            owner_field_name.to_string(),
            json!(account_keys[accounts[last_nonsigner_index] as usize].to_string()),
        );
    }
}

fn check_num_token_accounts(accounts: &[u8], num: usize) -> Result<(), ParseInstructionError> {
    check_num_accounts(accounts, num, ParsableProgram::HplNft)
}
