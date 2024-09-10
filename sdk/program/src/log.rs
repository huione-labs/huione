//! Logging utilities for Rust-based HC programs.
//!
//! Logging is the main mechanism for getting debugging information out of
//! running HC programs, and there are several functions available for doing
//! so efficiently, depending on the type of data being logged.
//!
//! The most common way to emit logs is through the [`msg!`] macro, which logs
//! simple strings, as well as [formatted strings][fs].
//!
//! [`msg!`]: msg
//! [fs]: https://doc.rust-lang.org/std/fmt/
//!
//! Logs can be viewed in multiple ways:
//!
//! - The `huione logs` command displays logs for all transactions executed on a
//!   network. Note though that transactions that fail during pre-flight
//!   simulation are not displayed here.
//! - When submitting transactions via [`RpcClient`], if Rust's own logging is
//!   active then the `huione_client` crate logs at the "debug" level any logs
//!   for transactions that failed during simulation. If using [`env_logger`]
//!   these logs can be activated by setting `RUST_LOG=huione_client=debug`.
//! - Logs can be retrieved from a finalized transaction by calling
//!   [`RpcClient::get_transaction`].
//! - Block explorers may display logs.
//!
//! [`RpcClient`]: https://docs.rs/huione-client/latest/huione_client/rpc_client/struct.RpcClient.html
//! [`env_logger`]: https://docs.rs/env_logger
//! [`RpcClient::get_transaction`]: https://docs.rs/huione-client/latest/huione_client/rpc_client/struct.RpcClient.html#method.get_transaction
//!
//! While most logging functions are defined in this module, [`Pubkey`]s can
//! also be efficiently logged with the [`Pubkey::log`] function.
//!
//! [`Pubkey`]: crate::pubkey::Pubkey
//! [`Pubkey::log`]: crate::pubkey::Pubkey::log

use crate::account_info::AccountInfo;

/// Print a message to the log.
#[macro_export]
#[deprecated(since = "1.4.14", note = "Please use `msg` macro instead")]
macro_rules! info {
    ($msg:expr) => {
        $crate::log::huione_log($msg)
    };
    ($arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
        $crate::log::huione_log_64(
            $arg1 as u64,
            $arg2 as u64,
            $arg3 as u64,
            $arg4 as u64,
            $arg5 as u64,
        )
    };
}

/// Print a message to the log.
///
/// Supports simple strings as well as Rust [format strings][fs]. When passed a
/// single expression it will be passed directly to [`sol_log`]. The expression
/// must have type `&str`, and is typically used for logging static strings.
/// When passed something other than an expression, particularly
/// a sequence of expressions, the tokens will be passed through the
/// [`format!`] macro before being logged with `sol_log`.
///
/// [fs]: https://doc.rust-lang.org/std/fmt/
/// [`format!`]: https://doc.rust-lang.org/std/fmt/fn.format.html
///
/// Note that Rust's formatting machinery is relatively CPU-intensive
/// for constrained environments like the HC VM.
///
/// # Examples
///
/// ```
/// use huione_program::msg;
///
/// // The fast form
/// msg!("verifying multisig");
///
/// // With formatting
/// let err = "not enough signers";
/// msg!("multisig failed: {}", err);
/// ```
#[macro_export]
macro_rules! msg {
    ($msg:expr) => {
        $crate::log::huione_log($msg)
    };
    ($($arg:tt)*) => ($crate::log::huione_log(&format!($($arg)*)));
}

/// Print a string to the log.
#[inline]
pub fn huione_log(message: &str) {
    #[cfg(target_os = "solana")]
    unsafe {
        crate::syscalls::sol_log_(message.as_ptr(), message.len() as u64);
    }

    #[cfg(not(target_os = "solana"))]
    crate::program_stubs::sol_log(message);
}

/// Print 64-bit values represented as hexadecimal to the log.
#[inline]
pub fn huione_log_64(arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64) {
    #[cfg(target_os = "solana")]
    unsafe {
        crate::syscalls::sol_log_64_(arg1, arg2, arg3, arg4, arg5);
    }

    #[cfg(not(target_os = "solana"))]
    crate::program_stubs::sol_log_64(arg1, arg2, arg3, arg4, arg5);
}

/// Print 128-bit values represented as hexadecimal to the log.
#[inline]
pub fn huione_log_128(arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64) {
    #[cfg(target_arch = "bpf")]
    unsafe {
        sol_log_128_(arg1, arg2, arg3, arg4, arg5);
    }

    #[cfg(not(target_arch = "bpf"))]
    crate::program_stubs::sol_log_128(arg1, arg2, arg3, arg4, arg5);
}


#[cfg(target_arch = "bpf")]
extern "C" {
    fn sol_log_64_(arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64);
}

#[cfg(target_arch = "bpf")]
extern "C" {
    fn sol_log_128_(arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64);
}


/// Print some slices as base64.
pub fn huione_log_data(data: &[&[u8]]) {
    #[cfg(target_os = "solana")]
    unsafe {
        crate::syscalls::sol_log_data(data as *const _ as *const u8, data.len() as u64)
    };

    #[cfg(not(target_os = "solana"))]
    crate::program_stubs::sol_log_data(data);
}

/// Print the hexadecimal representation of a slice.
#[allow(dead_code)]
pub fn huione_log_slice(slice: &[u8]) {
    for (i, s) in slice.iter().enumerate() {
        huione_log_64(0, 0, 0, i as u64, *s as u64);
    }
}

/// Print the hexadecimal representation of the program's input parameters.
///
/// - `accounts` - A slice of [`AccountInfo`].
/// - `data` - The instruction data.
#[allow(dead_code)]
pub fn huione_log_params(accounts: &[AccountInfo], data: &[u8]) {
    for (i, account) in accounts.iter().enumerate() {
        msg!("AccountInfo");
        huione_log_64(0, 0, 0, 0, i as u64);
        msg!("- Is signer");
        huione_log_64(0, 0, 0, 0, account.is_signer as u64);
        msg!("- Key");
        account.key.log();
        msg!("- Lamports");
        huione_log_128(0, 0, 0,  (account.lamports() >> 64) as u64, (account.lamports() & 0x00000000FFFFFFFF )  as u64);
        msg!("- Account data length");
        huione_log_64(0, 0, 0, 0, account.data_len() as u64);
        msg!("- Owner");
        account.owner.log();
    }
    msg!("Instruction data");
    huione_log_slice(data);
}

/// Print the remaining compute units available to the program.
#[inline]
pub fn huione_log_compute_units() {
    #[cfg(target_os = "solana")]
    unsafe {
        crate::syscalls::sol_log_compute_units_();
    }
    #[cfg(not(target_os = "solana"))]
    crate::program_stubs::sol_log_compute_units();
}
