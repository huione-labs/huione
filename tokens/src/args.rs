use huione_sdk::{pubkey::Pubkey, signature::Signer};

pub struct SenderStakeArgs {
    pub stake_account_address: Pubkey,
    pub stake_authority: Box<dyn Signer>,
    pub withdraw_authority: Box<dyn Signer>,
    pub lockup_authority: Option<Box<dyn Signer>>,
}

pub struct StakeArgs {
    pub unlocked_hc: u128,
    pub lockup_authority: Option<Pubkey>,
    pub sender_stake_args: Option<SenderStakeArgs>,
}

pub struct DistributeTokensArgs {
    pub inhuione_csv: String,
    pub transaction_db: String,
    pub outhuione_path: Option<String>,
    pub dry_run: bool,
    pub sender_keypair: Box<dyn Signer>,
    pub fee_payer: Box<dyn Signer>,
    pub stake_args: Option<StakeArgs>,
    pub hpl_token_args: Option<HplTokenArgs>,
    pub transfer_amount: Option<u128>,
}

#[derive(Default)]
pub struct HplTokenArgs {
    pub token_account_address: Pubkey,
    pub mint: Pubkey,
    pub decimals: u8,
}

pub struct BalancesArgs {
    pub inhuione_csv: String,
    pub hpl_token_args: Option<HplTokenArgs>,
}

pub struct TransactionLogArgs {
    pub transaction_db: String,
    pub outhuione_path: String,
}

#[allow(clippy::large_enum_variant)]
pub enum Command {
    DistributeTokens(DistributeTokensArgs),
    Balances(BalancesArgs),
    TransactionLog(TransactionLogArgs),
}

pub struct Args {
    pub config_file: String,
    pub url: Option<String>,
    pub command: Command,
}
