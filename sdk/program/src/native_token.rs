//! Definitions for the native HC token and its fractional lamports.

#![allow(clippy::integer_arithmetic)]
/// There are 10^9 lamports in one HC
pub const LAMPORTS_PER_HC: u128 = 1_000_000_000;


pub fn parse_amount(amount:&str) -> (u128, f64){
    let num:Vec<&str> = amount.split('.').collect();
    if num.len() == 1 {
        return (num[0].parse::<u128>().unwrap(),0_f64)
    }else if num.len() == 2  {
        if num[1] != ""{
            let frac  = "0.".to_owned() + num[1];
            return (num[0].parse::<u128>().unwrap(),frac.parse::<f64>().unwrap())
        }else{
            return (num[0].parse::<u128>().unwrap(),0_f64)
        }
    }else {
        return (0_u128,0_f64)
    }
}

/// Approximately convert fractional native tokens (lamports) into native tokens (HC)
pub fn lamports_to_hc(lamports: u128) -> String {
    // Left-pad zeros to decimals + 1, so we at least have an integer zero
    let mut s = format!("{:01$}", lamports, 10);
    // Add the decimal point (Sorry, "," locales!)
    s.insert(s.len() - 9, '.');
    let zeros_trimmed = s.trim_end_matches('0');
    s = zeros_trimmed.trim_end_matches('.').to_string();
    s
}

/// Approximately convert native tokens (HC) into fractional native tokens (lamports)
pub fn hc_to_lamports(hc: &str) -> u128 {
    let (inte,frac) = parse_amount(hc);
    inte * LAMPORTS_PER_HC + (frac * LAMPORTS_PER_HC as f64) as u128
}

use std::fmt::{Debug, Display, Formatter, Result};
pub struct HC(pub u128);

impl HC {
    fn write_in_hc(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "◎{}.{:09}",
            self.0 / LAMPORTS_PER_HC,
            self.0 % LAMPORTS_PER_HC
        )
    }
}

impl Display for HC {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_hc(f)
    }
}

impl Debug for HC {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_hc(f)
    }
}
