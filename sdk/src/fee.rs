//! Fee structures.

use crate::native_token::hc_to_lamports;

/// A fee and its associated compute unit limit
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct FeeBin {
    /// maximum compute units for which this fee will be charged
    pub limit: u128,
    /// fee in lamports
    pub fee: u128,
}

/// Information used to calculate fees
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FeeStructure {
    /// lamports per signature
    pub lamports_per_signature: u128,
    /// lamports_per_write_lock
    pub lamports_per_write_lock: u128,
    /// Compute unit fee bins
    pub compute_fee_bins: Vec<FeeBin>,
}

impl FeeStructure {
    pub fn new(
        huione_per_signature: f64,
        huione_per_write_lock: f64,
        compute_fee_bins: Vec<(u128, f64)>,
    ) -> Self {
        let compute_fee_bins = compute_fee_bins
            .iter()
            .map(|(limit, hc)| FeeBin {
                limit: *limit,
                fee: hc_to_lamports((*hc).to_string().as_str()),
            })
            .collect::<Vec<_>>();
        FeeStructure {
            lamports_per_signature: hc_to_lamports((huione_per_signature).to_string().as_str()),
            lamports_per_write_lock: hc_to_lamports((huione_per_write_lock).to_string().as_str()),
            compute_fee_bins,
        }
    }

    pub fn get_max_fee(&self, num_signatures: u128, num_write_locks: u128) -> u128 {
        num_signatures
            .saturating_mul(self.lamports_per_signature)
            .saturating_add(num_write_locks.saturating_mul(self.lamports_per_write_lock))
            .saturating_add(
                self.compute_fee_bins
                    .last()
                    .map(|bin| bin.fee)
                    .unwrap_or_default(),
            )
    }
}

impl Default for FeeStructure {
    fn default() -> Self {
        Self::new(0.00003, 0.0, vec![(1_400_000, 0.0)])
    }
}

#[cfg(RUSTC_WITH_SPECIALIZATION)]
impl ::huione_frozen_abi::abi_example::AbiExample for FeeStructure {
    fn example() -> Self {
        FeeStructure::default()
    }
}
