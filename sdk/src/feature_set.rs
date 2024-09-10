//! Collection of all runtime features.
//!
//! Steps to add a new feature are outlined below. Note that these steps only cover
//! the process of getting a feature into the core HC code.
//! - For features that are unambiguously good (ie bug fixes), these steps are sufficient.
//! - For features that should go up for community vote (ie fee structure changes), more
//!   information on the additional steps to follow can be found at:
//!   <https://spl.huione.com/feature-proposal#feature-proposal-life-cycle>
//!
//! 1. Generate a new keypair with `huione-keygen new --outfile feature.json --no-passphrase`
//!    - Keypairs should be held by core contributors only. If you're a non-core contirbutor going
//!      through these steps, the PR process will facilitate a keypair holder being picked. That
//!      person will generate the keypair, provide pubkey for PR, and ultimately enable the feature.
//! 2. Add a public module for the feature, specifying keypair pubkey as the id with
//!    `huione_sdk::declare_id!()` within the module.
//!    Additionally, add an entry to `FEATURE_NAMES` map.
//! 3. Add desired logic to check for and switch on feature availability.
//!
//! For more information on how features are picked up, see comments for `Feature`.

use {
    lazy_static::lazy_static,
    huione_sdk::{
        clock::Slot,
        hash::{Hash, Hasher},
        pubkey::Pubkey,
    },
    std::collections::{HashMap, HashSet},
};

// ## 1 
pub mod curve25519_syscall_enabled {
    huione_sdk::declare_id!("Mp1rjHb6t5jkj6KjEPTWwanW8U2HnhoYoMVCf858umc");
}


pub mod libsecp256k1_fail_on_bad_count2 {
    huione_sdk::declare_id!("F3YisHzQt8iztemVwEGjmNsjiRgxs631rj7tMNXrQbf5");
}


pub mod credits_auto_rewind {
    huione_sdk::declare_id!("AUr5B4BRP4tqDYtKSUGgnNENYu9qWTjLeVD8mW9UaWoR");
}


pub mod stake_deactivate_delinquent_instruction {
    huione_sdk::declare_id!("AsRuTH5CmNBjEywiAX5iW2PXjsbh5AdeJss1rtPZWC2v");
}
// ## 5 
pub mod stake_redelegate_instruction {
    huione_sdk::declare_id!("GKsbgoKzEqvuXrnANF19Ttv3Gg3ZKmn5LcqRPTwmsBp2");
}



pub mod check_slice_translation_size {
    huione_sdk::declare_id!("9ki7E7DjLCJth3D1fDipi4mSorF4ZnMHv2NkJXea9tX6");
}

pub mod stake_split_uses_rent_sysvar {
    huione_sdk::declare_id!("4gv2XmEtbuqUA5zDTV9a8rSk6yNWzqxuY7VwvofR3pmL");
}

pub mod add_get_minimum_delegation_instruction_to_stake_program {
    huione_sdk::declare_id!("8gMU4RHHR4sMEtm9q218CPdySGpXrJG7JYMj5o1rNUPg");
}

pub mod error_on_syscall_bpf_function_hash_collisions {
    huione_sdk::declare_id!("9dvcTUnsDwathLKaBeLsYH8pNzddeD6PKW9nC6jtvsgP");
}
// ## 10
pub mod reject_callx_r10 {
    huione_sdk::declare_id!("GnrLyx7Pk9Y4GFJq3NC2Fc7w6G6dg7oK6KUw3Jh9t2Xq");
}

pub mod drop_redundant_turbine_path {
    huione_sdk::declare_id!("3bo359du8jeJZpDmrgAxkL3dnL7jGxUSwvkBPL37VQ4W");
}

pub mod executables_incur_cpi_data_cost {
    huione_sdk::declare_id!("BdiS83yA7ukFPeeHiiHEmVwczFsku6TMqR4LzmXFS5ZN");
}

pub mod fix_recent_blockhashes {
    huione_sdk::declare_id!("BQC9ammnPEwN8U7Nx8JDLqXCNjZT5AEvVFpgXUZdfXcq");
}

pub mod update_rewards_from_cached_accounts {
    huione_sdk::declare_id!("8Y4UugFSYt458rvKLAZbFZxVs3LkMPoPiqXPn6XuS4YV");
}
// ## 15
pub mod default_units_per_instruction {
    huione_sdk::declare_id!("2Ef2TvJvzZuF2tPbrSWwLyvqJihqHCa88cxyMvn192Rn");
}

pub mod stake_allow_zero_undelegated_amount {
    huione_sdk::declare_id!("691qaCrV1ZWUseGbxcbE8Fhw8uBS5XKMK1b53UkjsCHv");
}

pub mod require_static_program_ids_in_transaction {
    huione_sdk::declare_id!("4C477H44LGKi1kQ6wapgWJ3i3mt3UZ27gt69uFzQY6zL");
}

pub mod stake_raise_minimum_delegation_to_1_sol {
    // This is a feature-proposal *feature id*.  The feature keypair address is `GQXzC7YiSNkje6FFUk6sc2p53XRvKoaZ9VMktYzUMnpL`.
    huione_sdk::declare_id!("3EX6n2iwsFjpnpxMgDKJEP2UnAf2Ska3JFAHLCewKHxk");
}

pub mod stake_minimum_delegation_for_rewards {
    huione_sdk::declare_id!("7V1pzTxfunC3nBqGZJJY1H8TZGmoMMG8WEEyy63bK7pD");
}
// ## 20
pub mod add_set_compute_unit_price_ix {
    huione_sdk::declare_id!("FUG9aiENpXnJAHghn2VAJuYx5efKQhKhbR6eV1TW4aq6");
}

pub mod disable_deploy_of_alloc_free_syscall {
    huione_sdk::declare_id!("67JQ4kXTtjS9ePjvcmx5DcBGcYvfQckcwmXMAnUDJNfy");
}

pub mod include_account_index_in_rent_error {
    huione_sdk::declare_id!("J1oonKcbEzuaaRn7WSNxVqfdHwXni8sBTj2DVdXfMxf5");
}

pub mod add_shred_type_to_shred_seed {
    huione_sdk::declare_id!("B8AQ9oNFjqi3EQVrr14iAQPo7p8F8T4GjksWNZhJM3eQ");
}

pub mod warp_timestamp_with_a_vengeance {
    huione_sdk::declare_id!("AAV3UptGem8rHXRgfAjRR7EcaBVAUELeGyZrKsobpzjD");
}
// ## 25
pub mod separate_nonce_from_blockhash {
    huione_sdk::declare_id!("5EF2UM9GckmZL4QhXiTDWtwePxTVrJWvJmEiwn3cTsYQ");
}

pub mod enable_durable_nonce {
    huione_sdk::declare_id!("7KZbW5x7tUuMNWZCWfekPBq85XWKSSvPmNznLjXxiFEa");
}

pub mod vote_state_update_credit_per_dequeue {
    huione_sdk::declare_id!("8mExJPFSELkaiRmAHYJefMvbWdd3om9M8ESL1Qwr7oUr");
}

pub mod quick_bail_on_panic {
    huione_sdk::declare_id!("CgV7iGiLa4Uk79rRZFLKbXptXikBgUwAaem2KAvTghPg");
}

pub mod nonce_must_be_authorized {
    huione_sdk::declare_id!("9qGoLN9dtptFsdLJD1K7Ka4xoGYpWHq4EpdGJV5MHvqf");
}
// ## 30
pub mod nonce_must_be_advanceable {
    huione_sdk::declare_id!("Hk6CC7CM4URAy83M6M6rHA18kzyuFZ4JnRC8Msvhb3Wm");
}

pub mod vote_authorize_with_seed {
    huione_sdk::declare_id!("8UdHntvxjx35MndgEUNni2zZ2EnBP27ZYJEaAhL6AZ8b");
}

pub mod cap_accounts_data_size_per_block {
    huione_sdk::declare_id!("3UwBVnKMG75agNkFZKREhmfsjmaU5jxFjU7Nrfhu32LL");
}

pub mod preserve_rent_epoch_for_rent_exempt_accounts {
    huione_sdk::declare_id!("HaDaWRQTNhthK1aKVnR95Y4GiLCujjHzRebjWZQQFtnr");
}

pub mod enable_bpf_loader_extend_program_ix {
    huione_sdk::declare_id!("7w9dSHaMiy5hHtQKrAHkPyR2P59f3GXmuWbKwDxfKRLd");
}
// ## 35
pub mod enable_early_verification_of_account_modifications {
    huione_sdk::declare_id!("HMPA2AhG76hrrXNRoxWg1oMSSUyxqQVRGoUK1swFercH");
}

pub mod skip_rent_rewrites {
    huione_sdk::declare_id!("BzjYLdv2cU1eT4qdbmcj6pn6VwnpaozDsRi73qFWzbT7");
}

pub mod prevent_crediting_accounts_that_end_rent_paying {
    huione_sdk::declare_id!("2SNnNhMqe4H5m4hBMWSo9kevkymiAcYka7bKA9tCKE1X");
}

pub mod cap_bpf_program_instruction_accounts {
    huione_sdk::declare_id!("G4PTcDsjETHn8ETFWWUmXx12z7AHc9sSCo6UGSLLhWEQ");
}

pub mod loosen_cpi_size_restriction {
    huione_sdk::declare_id!("FoyNYcQ7HCWNujZiKocTxDaaaCHA4oviprhTpYy8ziy4");
}
// ## 40
pub mod use_default_units_in_fee_calculation {
    huione_sdk::declare_id!("9Bvm37Rjtobxs7ProGjCYJDarBAMjdKmbaZwLvsj2jnz");
}

pub mod compact_vote_state_updates {
    huione_sdk::declare_id!("EvnjGsXGPNPEDYuKBhLQQboWBQotdRuwiCRgXNiy6StN");
}

pub mod incremental_snapshot_only_incremental_hash_calculation {
    huione_sdk::declare_id!("27amsDpAzH4cnMhDRenYorArCksMFLFZPpGGBvskeHN1");
}

pub mod disable_cpi_setting_executable_and_rent_epoch {
    huione_sdk::declare_id!("C56L9wCRT1ZvcZCon18TB4W8fTgz54f1e1Gsx6R6cyUH");
}

pub mod on_load_preserve_rent_epoch_for_rent_exempt_accounts {
    huione_sdk::declare_id!("J6Dua4YXzRPycDKMarUvMErwtwG5djayedgR5ZsLcoCV");
}
// ## 45
pub mod account_hash_ignore_slot {
    huione_sdk::declare_id!("75uABuDfz7vHqNJStLrVsZqbfkLVfUrbKnZnQGwa5NHX");
}

pub mod set_exempt_rent_epoch_max {
    huione_sdk::declare_id!("6uWugMk5kVDoYrXnDs8CQojuKghvLZHYycBGpwfhuVch");
}

pub mod relax_authority_signer_check_for_lookup_table_creation {
    huione_sdk::declare_id!("489Qxg5fDsZzxq6RmJNP4cuCY6tkvhR1fNofpxGQ6ERz");
}

pub mod stop_sibling_instruction_search_at_parent {
    huione_sdk::declare_id!("84u69tNbp8nyVPCewt9GQHBVo8vx4kaN1cf4o8LQ7WbU");
}

pub mod vote_state_update_root_fix {
    huione_sdk::declare_id!("5hmjFc8kWmjmnvU5YW2AjzWApSCmJXKpmhM77ArEZwx7");
}
// ## 50
pub mod cap_accounts_data_allocations_per_transaction {
    huione_sdk::declare_id!("HQW9AkUPxBh4uNSULWwdZLceaq9bCRQ1e75JRpH7UMm6");
}

pub mod epoch_accounts_hash {
    huione_sdk::declare_id!("8Xr13wED3K3NBeM7XKviHeNfqy7R2mEGd17Sc4LY26y5");
}

pub mod remove_deprecated_request_unit_ix {
    huione_sdk::declare_id!("E9DGvsZicZCjcW9PjSZfGuTS8iqFKopcsxQiYz3KjrTh");
}

pub mod disable_rehash_for_rent_epoch {
    huione_sdk::declare_id!("4ujziirewXCT7N5og7noqEKayQSKUU6iK7u5T2wfgFkU");
}

pub mod increase_tx_account_lock_limit {
    huione_sdk::declare_id!("2C6RXkghHC9rqgpCyjXJ7HhEvvkcNNfpnXSMABasxk5y");
}
// ## 55
pub mod limit_max_instruction_trace_length {
    huione_sdk::declare_id!("9a59kGzbS5Cujq5NJAYZxgAjWQEZxpNDTmy77EsmtaJw");
}

pub mod check_syscall_outputs_do_not_overlap {
    huione_sdk::declare_id!("E8M4u6yqkFKcS1HEeg74NWkJMY7VQ76g5hJGv6cGAtP4");
}

pub mod enable_bpf_loader_set_authority_checked_ix {
    huione_sdk::declare_id!("4U7fAzjWGvGLc22s58kfiJyRpodptVQLdVw9bUXh2YCs");
}

pub mod enable_alt_bn128_syscall {
    huione_sdk::declare_id!("BeohTScywW84mzbCnqVAZW8t11fhPYkdE1siHB1WRi8R");
}

pub mod enable_program_redeployment_cooldown {
    huione_sdk::declare_id!("MFkUyXrbk6ZrcUxwKVPejLZevWAnhey6RbxdDZ9Aas7");
}
// ## 60
pub mod commission_updates_only_allowed_in_first_half_of_epoch {
    huione_sdk::declare_id!("5x3gcoawg1XWVQJHRnL1evZe31BHpmeXF7v3AqWv5hSk");
}

pub mod enable_turbine_fanout_experiments {
    huione_sdk::declare_id!("ErfE1c9EyZwShf7CBFPncwACjSusXft1iqL3XFnYsRwR");
}

pub mod disable_turbine_fanout_experiments {
    huione_sdk::declare_id!("H8Hu2d3yU2jei1twECPpFoT7gzwQ6MVBAfUkWJAdtiFc");
}

pub mod drop_merkle_shreds {
    huione_sdk::declare_id!("21uSWdSFYXrbHWhxqdmogJRvs82HYQzWJQiNb2cLLYWP");
}

pub mod keep_merkle_shreds {
    huione_sdk::declare_id!("BdtnFunXcLyJTrsf7J1wtxxNzaeMaXZ72yCnBxTFHfLr");
}
// ## 65
pub mod move_serialized_len_ptr_in_cpi {
    huione_sdk::declare_id!("BZc74EFqr54Vvm2GQ51QTy36e5Q7cxvaJY8NcyuLAUby");
}

pub mod update_hashes_per_tick {
    huione_sdk::declare_id!("FLZVHKg5SKdm6JFyoFSHnuxgzZXczpbpAiLvZ1JqzLG3");
}

pub mod enable_big_mod_exp_syscall {
    huione_sdk::declare_id!("DxYJMcqEjA45gzqpfqzVEoW1tvtSf48u2BTosRGvFjmm");
}

pub mod disable_builtin_loader_ownership_chains {
    huione_sdk::declare_id!("ByqxEY815JfiFEvsPKy83FcBXGmAMUHPJsdrw62XWJq8");
}

pub mod cap_transaction_accounts_data_size {
    huione_sdk::declare_id!("2oHNLowcNXFo1yxogKrv5qxMQ28gX5d4WhLi5HZhbxdc");
}
// ## 70
pub mod remove_congestion_multiplier_from_fee_calculation {
    huione_sdk::declare_id!("B7qbnwuXrf1JNjMr8NdjXBXPinGeKkz4qFAz4cj64j1e");
}

pub mod enable_request_heap_frame_ix {
    huione_sdk::declare_id!("4xpAKWmFh9n1ukkML93PeinfxB4jsXhAoe4ur3prgVb1");
}

pub mod prevent_rent_paying_rent_recipients {
    huione_sdk::declare_id!("4u6w7U52LSqT5zhfj3ygB8t1CvRayrzgbjB7PJTX1ajm");
}

pub mod delay_visibility_of_program_deployment {
    huione_sdk::declare_id!("nnAArNNgNnztaJz7rgZX2N2v26WLBixszJH8XWDuD3f");
}

pub mod apply_cost_tracker_during_replay {
    huione_sdk::declare_id!("3XRBjxhQWmtiD87tQTyeSr7uhPCt9wJ4VYeNEyJ1kkLG");
}
// ## 75
pub mod bpf_account_data_direct_mapping {
    huione_sdk::declare_id!("CSTvuFeMvLNifa5Hu4KNwg3mW1QovbXdXYA769ZxKRP7");
}

pub mod add_set_tx_loaded_accounts_data_size_instruction {
    huione_sdk::declare_id!("5xMpySXL55XRVYCb9GNGCbrFUuVq96J7gBGLMcFAjuFZ");
}

pub mod switch_to_new_elf_parser {
    huione_sdk::declare_id!("3MEyuZeX5eK4vHJxDsWDAtvQaBm8Zxqi8uEBD1fxa23c");
}

pub mod round_up_heap_size {
    huione_sdk::declare_id!("CSbyBinLe3RVB5jVLyBUs5A8f1NSrvVUrgP3RUUyhs9A");
}

pub mod remove_bpf_loader_incorrect_program_id {
    huione_sdk::declare_id!("3M4cKRjRuPQo9nWEFsDDjcHSyyYkB2pnfck7xVB2JhDn");
}
// ## 80
pub mod include_loaded_accounts_data_size_in_fee_calculation {
    huione_sdk::declare_id!("G5Ycq566K7ybr2NM54p8NiYAH2rB1Hqtvk2CiLomjYPj");
}

pub mod native_programs_consume_cu {
    huione_sdk::declare_id!("8n2TJhuBWkaABpbie6ftKTZz3NeXD5Dh6DCcSJpWLR8A");
}

pub mod simplify_writable_program_account_check {
    huione_sdk::declare_id!("EgZDuK4Wuk1hjA6brDTYUmZ3xsK4oyWRqUh8nutkRQjA");
}

pub mod stop_truncating_strings_in_syscalls {
    huione_sdk::declare_id!("3XCeBu9qT42MNji6wvFQGjWKz8KJsj6Qi8Pe13bkczG9");
}

pub mod clean_up_delegation_errors {
    huione_sdk::declare_id!("8V11WFpaXKiG73CZDEgPMf1UsqGbjjd3gfvwtbHt7aiy");
}
// ## 85
pub mod vote_state_add_vote_latency {
    huione_sdk::declare_id!("3LWzmZQ9tmq4Kdqt5f33Lp9QA3hXGTWEqVESGACWqueU");
}

pub mod checked_arithmetic_in_fee_validation {
    huione_sdk::declare_id!("YP3jWGhzdK3RQeJQAAKz5AJgVk8b3w3jV7KoXDLMX2H");
}

pub mod reduce_stake_warmup_cooldown {
    use huione_program::{epoch_schedule::EpochSchedule, stake_history::Epoch};
    huione_sdk::declare_id!("4wtqo6ovpqqhMVgWM4khvnaKt5aX4eNPypDp3ywrxQz");

    pub trait NewWarmupCooldownRateEpoch {
        fn new_warmup_cooldown_rate_epoch(&self, epoch_schedule: &EpochSchedule) -> Option<Epoch>;
    }
    impl NewWarmupCooldownRateEpoch for super::FeatureSet {
        fn new_warmup_cooldown_rate_epoch(&self, epoch_schedule: &EpochSchedule) -> Option<Epoch> {
            self.activated_slot(&id())
                .map(|slot| epoch_schedule.get_epoch(slot))
        }
    }
}


lazy_static! {
    /// Map of feature identifiers to user-visible description
    pub static ref FEATURE_NAMES: HashMap<Pubkey, &'static str> = [

        // (curve25519_syscall_enabled::id(), "enable curve25519 syscalls"),
       
        // (libsecp256k1_fail_on_bad_count2::id(), "fail libsec256k1_verify if count appears wrong"),
        
        // (credits_auto_rewind::id(), "Auto rewind stake's credits_observed if (accidental) vote recreation is detected #22546"),

        // (stake_deactivate_delinquent_instruction::id(), "enable the deactivate delinquent stake instruction #23932"),
        // (check_slice_translation_size::id(), "check size when translating slices"),
        // (stake_split_uses_rent_sysvar::id(), "stake split instruction uses rent sysvar"),
        // (add_get_minimum_delegation_instruction_to_stake_program::id(), "add GetMinimumDelegation instruction to stake program"),
        // (error_on_syscall_bpf_function_hash_collisions::id(), "error on bpf function hash collisions"),
        // (reject_callx_r10::id(), "Reject bpf callx r10 instructions"),
        // (drop_redundant_turbine_path::id(), "drop redundant turbine path"),
        // (executables_incur_cpi_data_cost::id(), "Executables incur CPI data costs"),
        // (fix_recent_blockhashes::id(), "stop adding hashes for skipped slots to recent blockhashes"),
        // (update_rewards_from_cached_accounts::id(), "update rewards from cached accounts"),
        // (spl_token_v3_4_0::id(), "SPL Token Program version 3.4.0 release #24740"), //--
        // (spl_associated_token_account_v1_1_0::id(), "SPL Associated Token Account Program version 1.1.0 release #24741"), //--
        // (default_units_per_instruction::id(), "Default max tx-wide compute units calculated per instruction"),
        // (stake_allow_zero_undelegated_amount::id(), "Allow zero-lamport undelegated amount for initialized stakes #24670"),
        // (require_static_program_ids_in_transaction::id(), "require static program ids in versioned transactions"),
        // (stake_raise_minimum_delegation_to_1_sol::id(), "Raise minimum stake delegation to 1.0 SOL #24357"),
        // (stake_minimum_delegation_for_rewards::id(), "stakes must be at least the minimum delegation to earn rewards"),
        // (add_set_compute_unit_price_ix::id(), "add compute budget ix for setting a compute unit price"),
        // (disable_deploy_of_alloc_free_syscall::id(), "disable new deployments of deprecated sol_alloc_free_ syscall"),
        // (include_account_index_in_rent_error::id(), "include account index in rent tx error #25190"),
        // (add_shred_type_to_shred_seed::id(), "add shred-type to shred seed #25556"),
        // (warp_timestamp_with_a_vengeance::id(), "warp timestamp again, adjust bounding to 150% slow #25666"),
        // (separate_nonce_from_blockhash::id(), "separate durable nonce and blockhash domains #25744"),
        // (enable_durable_nonce::id(), "enable durable nonce #25744"),
        // (vote_state_update_credit_per_dequeue::id(), "Calculate vote credits for VoteStateUpdate per vote dequeue to match credit awards for Vote instruction"),
        // (quick_bail_on_panic::id(), "quick bail on panic"),
        // (nonce_must_be_authorized::id(), "nonce must be authorized"),
        // (nonce_must_be_advanceable::id(), "durable nonces must be advanceable"),
        // (vote_authorize_with_seed::id(), "An instruction you can use to change a vote accounts authority when the current authority is a derived key #25860"),
        // (cap_accounts_data_size_per_block::id(), "cap the accounts data size per block #25517"),
        // (stake_redelegate_instruction::id(), "enable the redelegate stake instruction #26294"),
        // (preserve_rent_epoch_for_rent_exempt_accounts::id(), "preserve rent epoch for rent exempt accounts #26479"),
        // (enable_bpf_loader_extend_program_ix::id(), "enable bpf upgradeable loader ExtendProgram instruction #25234"),
        // (skip_rent_rewrites::id(), "skip rewriting rent exempt accounts during rent collection #26491"),
        // (enable_early_verification_of_account_modifications::id(), "enable early verification of account modifications #25899"),
        // (disable_rehash_for_rent_epoch::id(), "on accounts hash calculation, do not try to rehash accounts #28934"),
        // (account_hash_ignore_slot::id(), "ignore slot when calculating an account hash #28420"),
        // (set_exempt_rent_epoch_max::id(), "set rent epoch to Epoch::MAX for rent-exempt accounts #28683"),
        // (on_load_preserve_rent_epoch_for_rent_exempt_accounts::id(), "on bank load account, do not try to fix up rent_epoch #28541"),
        // (prevent_crediting_accounts_that_end_rent_paying::id(), "prevent crediting rent paying accounts #26606"),
        // (cap_bpf_program_instruction_accounts::id(), "enforce max number of accounts per bpf program instruction #26628"),
        // (loosen_cpi_size_restriction::id(), "loosen cpi size restrictions #26641"),
        // (use_default_units_in_fee_calculation::id(), "use default units per instruction in fee calculation #26785"),
        // (compact_vote_state_updates::id(), "Compact vote state updates to lower block size"),
        // (incremental_snapshot_only_incremental_hash_calculation::id(), "only hash accounts in incremental snapshot during incremental snapshot creation #26799"),
        // (disable_cpi_setting_executable_and_rent_epoch::id(), "disable setting is_executable and_rent_epoch in CPI #26987"),
        // (relax_authority_signer_check_for_lookup_table_creation::id(), "relax authority signer check for lookup table creation #27205"),
        // (stop_sibling_instruction_search_at_parent::id(), "stop the search in get_processed_sibling_instruction when the parent instruction is reached #27289"),
        // (vote_state_update_root_fix::id(), "fix root in vote state updates #27361"),
        // (cap_accounts_data_allocations_per_transaction::id(), "cap accounts data allocations per transaction #27375"),
        // (epoch_accounts_hash::id(), "enable epoch accounts hash calculation #27539"),
        // (remove_deprecated_request_unit_ix::id(), "remove support for RequestUnitsDeprecated instruction #27500"),
        // (increase_tx_account_lock_limit::id(), "increase tx account lock limit to 128 #27241"),
        // (limit_max_instruction_trace_length::id(), "limit max instruction trace length #27939"),
        // (check_syscall_outputs_do_not_overlap::id(), "check syscall outputs do_not overlap #28600"),
        // (enable_bpf_loader_set_authority_checked_ix::id(), "enable bpf upgradeable loader SetAuthorityChecked instruction #28424"),
        // (enable_alt_bn128_syscall::id(), "add alt_bn128 syscalls #27961"),
        // (enable_program_redeployment_cooldown::id(), "enable program redeployment cooldown #29135"),
        // (commission_updates_only_allowed_in_first_half_of_epoch::id(), "validator commission updates are only allowed in the first half of an epoch #29362"),
        // (enable_turbine_fanout_experiments::id(), "enable turbine fanout experiments #29393"),
        // (disable_turbine_fanout_experiments::id(), "disable turbine fanout experiments #29393"),
        // (drop_merkle_shreds::id(), "drop merkle shreds #29711"),
        // (keep_merkle_shreds::id(), "keep merkle shreds #29711"),
        // (move_serialized_len_ptr_in_cpi::id(), "cpi ignore serialized_len_ptr #29592"),
        // (update_hashes_per_tick::id(), "Update desired hashes per tick on epoch boundary"),
        // (enable_big_mod_exp_syscall::id(), "add big_mod_exp syscall #28503"),
        // (disable_builtin_loader_ownership_chains::id(), "disable builtin loader ownership chains #29956"),
        // (cap_transaction_accounts_data_size::id(), "cap transaction accounts data size up to a limit #27839"),
        // (remove_congestion_multiplier_from_fee_calculation::id(), "Remove congestion multiplier from transaction fee calculation #29881"),
        // (enable_request_heap_frame_ix::id(), "Enable transaction to request heap frame using compute budget instruction #30076"),
        // (prevent_rent_paying_rent_recipients::id(), "prevent recipients of rent rewards from ending in rent-paying state #30151"),
        // (delay_visibility_of_program_deployment::id(), "delay visibility of program upgrades #30085"),
        // (apply_cost_tracker_during_replay::id(), "apply cost tracker to blocks during replay #29595"),
        // (add_set_tx_loaded_accounts_data_size_instruction::id(), "add compute budget instruction for setting account data size per transaction #30366"),
        // (switch_to_new_elf_parser::id(), "switch to new ELF parser #30497"),
        // (round_up_heap_size::id(), "round up heap size when calculating heap cost #30679"),
        // (remove_bpf_loader_incorrect_program_id::id(), "stop incorrectly throwing IncorrectProgramId in bpf_loader #30747"),
        // (include_loaded_accounts_data_size_in_fee_calculation::id(), "include transaction loaded accounts data size in base fee calculation #30657"),
        // (native_programs_consume_cu::id(), "Native program should consume compute units #30620"),
        // (simplify_writable_program_account_check::id(), "Simplify checks performed for writable upgradeable program accounts #30559"),
        // (stop_truncating_strings_in_syscalls::id(), "Stop truncating strings in syscalls #31029"),
        // (clean_up_delegation_errors::id(), "Return InsufficientDelegation instead of InsufficientFunds or InsufficientStake where applicable #31206"),
        // (vote_state_add_vote_latency::id(), "replace Lockout with LandedVote (including vote latency) in vote state #31264"),
        // (checked_arithmetic_in_fee_validation::id(), "checked arithmetic in fee validation #31273"),
        // (bpf_account_data_direct_mapping::id(), "use memory regions to map account data into the rbpf vm instead of copying the data"),
        // (reduce_stake_warmup_cooldown::id(), "reduce stake warmup cooldown from 25% to 9%"),
        /*************** ADD NEW FEATURES HERE ***************/
    ]
    .iter()
    .cloned()
    .collect();

    /// Unique identifier of the current software's feature set
    pub static ref ID: Hash = {
        let mut hasher = Hasher::default();
        let mut feature_ids = FEATURE_NAMES.keys().collect::<Vec<_>>();
        feature_ids.sort();
        for feature in feature_ids {
            hasher.hash(feature.as_ref());
        }
        hasher.result()
    };

    pub static ref INIT_FEATURE_NAMES: HashSet<Pubkey> = [
        /*************** ADD NEW FEATURES HERE ***************/
    ]
    .iter()
    .cloned()
    .collect();

}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FullInflationFeaturePair {
    pub vote_id: Pubkey, // Feature that grants the candidate the ability to enable full inflation
    pub enable_id: Pubkey, // Feature to enable full inflation by the candidate
}

lazy_static! {
    // Set of feature pairs that once enabled will trigger full inflation
    pub static ref FULL_INFLATION_FEATURE_PAIRS: HashSet<FullInflationFeaturePair> = [
        // FullInflationFeaturePair {
        //     vote_id: full_inflation::mainnet::certusone::vote::id(),
        //     enable_id: full_inflation::mainnet::certusone::enable::id(),
        // },
    ]
    .iter()
    .cloned()
    .collect();
}

/// `FeatureSet` holds the set of currently active/inactive runtime features
#[derive(AbiExample, Debug, Clone, Eq, PartialEq)]
pub struct FeatureSet {
    pub active: HashMap<Pubkey, Slot>,
    pub inactive: HashSet<Pubkey>,
}
impl Default for FeatureSet {
    fn default() -> Self {
        // All features disabled
        Self {
            active: HashMap::new(),
            inactive: FEATURE_NAMES.keys().cloned().collect(),
        }
    }
}
impl FeatureSet {
    pub fn is_active(&self, feature_id: &Pubkey) -> bool {
        self.active.contains_key(feature_id)
    }

    pub fn activated_slot(&self, feature_id: &Pubkey) -> Option<Slot> {
        self.active.get(feature_id).copied()
    }

    /// List of enabled features that trigger full inflation
    pub fn full_inflation_features_enabled(&self) -> HashSet<Pubkey> {
        let  hash_set = FULL_INFLATION_FEATURE_PAIRS
            .iter()
            .filter_map(|pair| {
                if self.is_active(&pair.vote_id) && self.is_active(&pair.enable_id) {
                    Some(pair.enable_id)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        // if self.is_active(&full_inflation::devnet_and_testnet::id()) {
        //     hash_set.insert(full_inflation::devnet_and_testnet::id());
        // }
        hash_set
    }

    /// All features enabled, useful for testing
    pub fn all_enabled() -> Self {
        Self {
            active: FEATURE_NAMES.keys().cloned().map(|key| (key, 0)).collect(),
            inactive: HashSet::new(),
        }
    }

    /// Activate a feature
    pub fn activate(&mut self, feature_id: &Pubkey, slot: u64) {
        self.inactive.remove(feature_id);
        self.active.insert(*feature_id, slot);
    }

    /// Deactivate a feature
    pub fn deactivate(&mut self, feature_id: &Pubkey) {
        self.active.remove(feature_id);
        self.inactive.insert(*feature_id);
    }
}

#[derive(AbiExample, Debug, Clone)]
pub struct InitFeatureSet {
    pub init: HashSet<Pubkey>,
}
impl Default for InitFeatureSet {
    fn default() -> Self {
        // All features disabled
        Self {
            init: INIT_FEATURE_NAMES.iter().cloned().collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_inflation_features_enabled_devnet_and_testnet() {
        // let mut feature_set = FeatureSet::default();
        // assert!(feature_set.full_inflation_features_enabled().is_empty());
        // feature_set
        //     .active
        //     .insert(full_inflation::devnet_and_testnet::id(), 42);
        // assert_eq!(
        //     feature_set.full_inflation_features_enabled(),
        //     [full_inflation::devnet_and_testnet::id()]
        //         .iter()
        //         .cloned()
        //         .collect()
        // );
    }

    #[test]
    fn test_full_inflation_features_enabled() {
        // // Normal sequence: vote_id then enable_id
        // let mut feature_set = FeatureSet::default();
        // assert!(feature_set.full_inflation_features_enabled().is_empty());
        // feature_set
        //     .active
        //     .insert(full_inflation::mainnet::certusone::vote::id(), 42);
        // assert!(feature_set.full_inflation_features_enabled().is_empty());
        // feature_set
        //     .active
        //     .insert(full_inflation::mainnet::certusone::enable::id(), 42);
        // assert_eq!(
        //     feature_set.full_inflation_features_enabled(),
        //     [full_inflation::mainnet::certusone::enable::id()]
        //         .iter()
        //         .cloned()
        //         .collect()
        // );

        // // Backwards sequence: enable_id and then vote_id
        // let mut feature_set = FeatureSet::default();
        // assert!(feature_set.full_inflation_features_enabled().is_empty());
        // feature_set
        //     .active
        //     .insert(full_inflation::mainnet::certusone::enable::id(), 42);
        // assert!(feature_set.full_inflation_features_enabled().is_empty());
        // feature_set
        //     .active
        //     .insert(full_inflation::mainnet::certusone::vote::id(), 42);
        // assert_eq!(
        //     feature_set.full_inflation_features_enabled(),
        //     [full_inflation::mainnet::certusone::enable::id()]
        //         .iter()
        //         .cloned()
        //         .collect()
        // );
    }

    #[test]
    fn test_feature_set_activate_deactivate() {
        let mut feature_set = FeatureSet::default();

        let feature = Pubkey::new_unique();
        assert!(!feature_set.is_active(&feature));
        feature_set.activate(&feature, 0);
        assert!(feature_set.is_active(&feature));
        feature_set.deactivate(&feature);
        assert!(!feature_set.is_active(&feature));
    }
}
