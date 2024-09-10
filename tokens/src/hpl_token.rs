use {
    crate::{
        args::{DistributeTokensArgs, HplTokenArgs},
        commands::{get_fee_estimate_for_messages, Allocation, Error, FundingSource},
    },
    console::style,
    huione_account_decoder::parse_token::{
        pubkey_from_hpl_token, real_number_string, real_number_string_trimmed, hpl_token_pubkey,
    },
    huione_rpc_client::rpc_client::RpcClient,
    huione_sdk::{instruction::Instruction, message::Message, native_token::lamports_to_hc},
    huione_transaction_status::parse_token::hpl_token_instruction,
    hpl_associated_token_account::{
        create_associated_token_account, get_associated_token_address
    },
    hpl_token::{
        huione_program::program_pack::Pack,
        state::{Account as HplTokenAccount, Mint},
    },
    
};

pub fn update_token_args(client: &RpcClient, args: &mut Option<HplTokenArgs>) -> Result<(), Error> {
    if let Some(hpl_token_args) = args {
        let sender_account = client
            .get_account(&hpl_token_args.token_account_address)
            .unwrap_or_default();
	    hpl_token_args.mint = HplTokenAccount::unpack(&sender_account.data)?.mint;
        update_decimals(client, args)?;
    }
    Ok(())
}

pub fn update_decimals(client: &RpcClient, args: &mut Option<HplTokenArgs>) -> Result<(), Error> {
    if let Some(hpl_token_args) = args {
        let mint_account = client.get_account(&hpl_token_args.mint).unwrap_or_default();
        let mint = Mint::unpack(&mint_account.data)?;
        hpl_token_args.decimals = mint.decimals;
    }
    Ok(())
}

pub fn hpl_token_amount(amount: f64, decimals: u8) -> u128 {
    (amount * 10_usize.pow(decimals as u32) as f64) as u128
}

pub fn build_hpl_token_instructions(
    allocation: &Allocation,
    args: &DistributeTokensArgs,
    do_create_associated_token_account: bool,
) -> Vec<Instruction> {
    let hpl_token_args = args
        .hpl_token_args
        .as_ref()
        .expect("hpl_token_args must be some");
    let wallet_address = allocation.recipient.parse().unwrap();
    let associated_token_address =
        get_associated_token_address(&wallet_address, &hpl_token_args.mint);
    let mut instructions = vec![];
    if do_create_associated_token_account {
        let create_associated_token_account_instruction = create_associated_token_account(
            &hpl_token_pubkey(&args.fee_payer.pubkey()),
            &wallet_address,
            &hpl_token_pubkey(&hpl_token_args.mint),
        );
        instructions.push(hpl_token_instruction(
            create_associated_token_account_instruction,
        ));
    }
    let hpl_instruction = hpl_token::instruction::transfer_checked(
        &hpl_token::id(),
        &hpl_token_pubkey(&hpl_token_args.token_account_address),
        &hpl_token_pubkey(&hpl_token_args.mint),
        &associated_token_address,
        &hpl_token_pubkey(&args.sender_keypair.pubkey()),
        &[],
        allocation.amount,
        hpl_token_args.decimals,
    )
    .unwrap();
    instructions.push(hpl_token_instruction(hpl_instruction));
    instructions
}

pub fn check_hpl_token_balances(
    messages: &[Message],
    allocations: &[Allocation],
    client: &RpcClient,
    args: &DistributeTokensArgs,
    created_accounts: u64,
) -> Result<(), Error> {
    let hpl_token_args = args
        .hpl_token_args
        .as_ref()
        .expect("hpl_token_args must be some");
    let allocation_amount: u128 = allocations.iter().map(|x| x.amount).sum();
    let fees = get_fee_estimate_for_messages(messages, client)?;

    let token_account_rent_exempt_balance =
        client.get_minimum_balance_for_rent_exemption(HplTokenAccount::LEN)?;
    let account_creation_amount = created_accounts as u128 * token_account_rent_exempt_balance;
    let fee_payer_balance = client.get_balance(&args.fee_payer.pubkey())?;
    if fee_payer_balance < fees + account_creation_amount {
        return Err(Error::InsufficientFunds(
            vec![FundingSource::FeePayer].into(),
            lamports_to_hc(fees + account_creation_amount).to_string(),
        ));
    }
    let source_token_account = client
        .get_account(&hpl_token_args.token_account_address)
        .unwrap_or_default();
    let source_token = HplTokenAccount::unpack(&source_token_account.data)?;
    if source_token.amount < allocation_amount {
        return Err(Error::InsufficientFunds(
            vec![FundingSource::HplTokenAccount].into(),
            real_number_string_trimmed(allocation_amount, hpl_token_args.decimals),
        ));
    }
    Ok(())
}

pub fn print_token_balances(
    client: &RpcClient,
    allocation: &Allocation,
    hpl_token_args: &HplTokenArgs,
) -> Result<(), Error> {
    let address = allocation.recipient.parse().unwrap();
    let expected = allocation.amount;
    let associated_token_address = get_associated_token_address(
        &hpl_token_pubkey(&address),
        &hpl_token_pubkey(&hpl_token_args.mint),
    );
    let recipient_account = client
        .get_account(&pubkey_from_hpl_token(&associated_token_address))
        .unwrap_or_default();
    let (actual, difference) = if let Ok(recipient_token) =
        HplTokenAccount::unpack(&recipient_account.data)
    {
        let actual_ui_amount = real_number_string(recipient_token.amount, hpl_token_args.decimals);
        let delta_string =
            real_number_string(recipient_token.amount - expected, hpl_token_args.decimals);
        (
            style(format!("{:>24}", actual_ui_amount)),
            format!("{:>24}", delta_string),
        )
    } else {
        (
            style("Associated token account not yet created".to_string()).yellow(),
            "".to_string(),
        )
    };
    println!(
        "{:<44}  {:>24}  {:>24}  {:>24}",
        allocation.recipient,
        real_number_string(expected, hpl_token_args.decimals),
        actual,
        difference,
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    // The following unit tests were written for v1.4 using the ProgramTest framework, passing its
    // BanksClient into the `huione-tokens` methods. 
    // These tests were removed rather than rewritten to avoid accruing technical debt. Once a new
    // rpc/client framework is implemented, they should be restored.
    //
    // async fn test_process_hpl_token_allocations()
    // async fn test_process_hpl_token_transfer_amount_allocations()
    // async fn test_check_hpl_token_balances()
    //
}
