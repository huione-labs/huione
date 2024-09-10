use {
    crate::parse_instruction::{
        check_num_accounts, ParsableProgram, ParseInstructionError, ParsedInstructionEnum,
    },
    hpl_name::instruction::NameInstruction,
    huione_sdk::{
        instruction::CompiledInstruction,
        message::AccountKeys,
    },
    serde_json::{json, Map, Value},
};

pub fn parse_name(
    instruction: &CompiledInstruction,
    account_keys: &AccountKeys,
) -> Result<ParsedInstructionEnum, ParseInstructionError> {
    let name_instruction = NameInstruction::deserialize(&instruction.data)
        .map_err(|_| ParseInstructionError::InstructionNotParsable(ParsableProgram::HplName))?;
    match instruction.accounts.iter().max() {
        Some(index) if (*index as usize) < account_keys.len() => {}
        _ => {
            // Runtime should prevent this from ever happening
            return Err(ParseInstructionError::InstructionKeyMismatch(
                ParsableProgram::HplName,
            ));
        }
    }
    match name_instruction {
        NameInstruction::CreateTopDomain {
            domain_name,
            rule,
            max_space,
        } => {
            check_num_token_accounts(&instruction.accounts, 7)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "CreateTopDomain".to_string(),
                info: json!({
                    "domainAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                    "payer": account_keys[instruction.accounts[1] as usize].to_string(),
                    "multiSigAccount": account_keys[instruction.accounts[2] as usize].to_string(),
                    "proposalAccount": account_keys[instruction.accounts[3] as usize].to_string(),
                    "hpl_sig": account_keys[instruction.accounts[4] as usize].to_string(),
                    "name":domain_name,
                }),
            })
        }
        NameInstruction::CreateDomain { domain_name } => {
            check_num_token_accounts(&instruction.accounts, 7)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "CreateDomain".to_string(),
                info: json!({
                    "domainAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                    "owner": account_keys[instruction.accounts[1] as usize].to_string(),
                    "parent": account_keys[instruction.accounts[2] as usize].to_string(),
                    "payer": account_keys[instruction.accounts[3] as usize].to_string(),
                    "receipt": account_keys[instruction.accounts[4] as usize].to_string(),
                    "name":domain_name,
                }),
            })
        }
        NameInstruction::CreateRareDomain { domain_name } => {
            check_num_token_accounts(&instruction.accounts, 7)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "CreateRareDomain".to_string(),
                info: json!({
                    "domainAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                    "owner": account_keys[instruction.accounts[1] as usize].to_string(),
                    "parent": account_keys[instruction.accounts[2] as usize].to_string(),
                    "payer": account_keys[instruction.accounts[3] as usize].to_string(),
                    "receipt": account_keys[instruction.accounts[4] as usize].to_string(),
                    "name":domain_name,
                }),
            })
        }
        NameInstruction::UpdateDomainResolveAccount {
            domain_name,
            new_value,
        } => {
            check_num_token_accounts(&instruction.accounts, 10)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "UpdateDomainResolveAccount".to_string(),
                info: json!({
                    "domainAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                    "owner": account_keys[instruction.accounts[1] as usize].to_string(),
                    "parent": account_keys[instruction.accounts[2] as usize].to_string(),
                    "old": account_keys[instruction.accounts[3] as usize].to_string(),
                    "new": account_keys[instruction.accounts[4] as usize].to_string(),
                    "payer": account_keys[instruction.accounts[5] as usize].to_string(),
                    "topDomain": account_keys[instruction.accounts[6] as usize].to_string(),
                    "topDomainReceipt": account_keys[instruction.accounts[7] as usize].to_string(),
                    "name":domain_name,
                    "new_value":base64::encode(new_value),
                    "encode":"base64"
                }),
            })
        }
        NameInstruction::CloseDomainResolveAccount => {
            check_num_token_accounts(&instruction.accounts, 6)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "CloseDomainResolveAccount".to_string(),
                info: json!({
                    "domainResolveAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                    "owner": account_keys[instruction.accounts[1] as usize].to_string(),
                    "parent": account_keys[instruction.accounts[2] as usize].to_string(),
                    "addressResolveAccount": account_keys[instruction.accounts[3] as usize].to_string(),
                    "topDomain": account_keys[instruction.accounts[6] as usize].to_string(),
                    "topDomainReceipt": account_keys[instruction.accounts[7] as usize].to_string(),
                }),
            })
        }
        NameInstruction::UnbindAddressAccount => {
            check_num_token_accounts(&instruction.accounts, 6)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "UnbindAddressAccount".to_string(),
                info: json!({
                    "domainResolveAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                    "owner": account_keys[instruction.accounts[1] as usize].to_string(),
                    "parent": account_keys[instruction.accounts[2] as usize].to_string(),
                    "addressResolveAccount": account_keys[instruction.accounts[3] as usize].to_string(),
                    "topDomain": account_keys[instruction.accounts[6] as usize].to_string(),
                    "topDomainReceipt": account_keys[instruction.accounts[7] as usize].to_string(),
                }),
            })
        }
        NameInstruction::Renewal => {
            check_num_token_accounts(&instruction.accounts, 5)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "Renewal".to_string(),
                info: json!({
                    "domainAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                    "parent": account_keys[instruction.accounts[1] as usize].to_string(),
                    "payer": account_keys[instruction.accounts[2] as usize].to_string(),
                    "receipt": account_keys[instruction.accounts[3] as usize].to_string(),
                }),
            })
        }
        NameInstruction::CreateDomainResolveAccount { domain_name, value } => {
            check_num_token_accounts(&instruction.accounts, 7)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "CreateDomainResolveAccount".to_string(),
                info: json!({
                    "domainAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                    "owner": account_keys[instruction.accounts[1] as usize].to_string(),
                    "parent": account_keys[instruction.accounts[2] as usize].to_string(),
                    "payer": account_keys[instruction.accounts[3] as usize].to_string(),
                    "resolveAccount": account_keys[instruction.accounts[4] as usize].to_string(),
                    "name":domain_name,
                    "data":base64::encode(value),
                    "encode":"base64"
                }),
            })
        }
        NameInstruction::SetTopReceipt {
            new_receipt_account,
        } => {
            check_num_token_accounts(&instruction.accounts, 1)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "SetTopReceipt".to_string(),
                info: json!({
                    "topDomainAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                    "new_receipt_account":new_receipt_account.to_string(),
                }),
            })
        }
        NameInstruction::Transfer => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "Transfer".to_string(),
                info: json!({
                    "domainAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                    "owner": account_keys[instruction.accounts[1] as usize].to_string(),
                    "receipt": account_keys[instruction.accounts[2] as usize].to_string(),
                }),
            })
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
    check_num_accounts(accounts, num, ParsableProgram::HplName)
}
