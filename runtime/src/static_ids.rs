use {
    crate::{inline_hpl_associated_token_account, inline_hpl_token, inline_hpl_token_2022},
    huione_sdk::pubkey::Pubkey,
};

lazy_static! {
    /// Vector of static token & mint IDs
    pub static ref STATIC_IDS: Vec<Pubkey> = vec![
        inline_hpl_associated_token_account::id(),
        // inline_hpl_associated_token_account::program_v1_1_0::id(),
        inline_hpl_token::id(),
        inline_hpl_token::native_mint::id(),
        inline_hpl_token_2022::id(),
    ];
}
