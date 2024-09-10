#![cfg(feature = "program")]

pub use huione_program::log::*;

#[macro_export]
#[deprecated(
    since = "1.4.3",
    note = "Please use `huione_program::log::info` instead"
)]
macro_rules! info {
    ($msg:expr) => {
        $crate::log::huione_log($msg)
    };
}
