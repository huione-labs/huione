use {
    huione_account_decoder::parse_token::real_number_string_trimmed,
    huione_sdk::native_token::lamports_to_hc,
    std::{
        fmt::{Debug, Display, Formatter, Result},
        ops::Add,
    },
};

const HC_SYMBOL: &str = "◎";

#[derive(PartialEq)]
pub enum TokenType {
    HC,
    HplToken,
}

pub struct Token {
    amount: u128,
    decimals: u8,
    token_type: TokenType,
}

impl Token {
    fn write_with_symbol(&self, f: &mut Formatter) -> Result {
        match &self.token_type {
            TokenType::HC => {
                let amount = lamports_to_hc(self.amount );
                write!(f, "{HC_SYMBOL}{amount}")
            }
            TokenType::HplToken => {
                let amount = real_number_string_trimmed(self.amount, self.decimals);
                write!(f, "{amount} tokens")
            }
        }
    }

    pub fn hc(amount: u128) -> Self {
        Self {
            amount : amount,
            decimals: 9,
            token_type: TokenType::HC,
        }
    }

    pub fn hpl_token(amount: u128, decimals: u8) -> Self {
        Self {
            amount,
            decimals,
            token_type: TokenType::HplToken,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_with_symbol(f)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_with_symbol(f)
    }
}

impl Add for Token {
    type Output = Token;

    fn add(self, other: Self) -> Self {
        if self.token_type == other.token_type {
            Self {
                amount: self.amount + other.amount,
                decimals: self.decimals,
                token_type: self.token_type,
            }
        } else {
            self
        }
    }
}
