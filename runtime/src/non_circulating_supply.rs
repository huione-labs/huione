use {
    crate::{
        accounts_index::{AccountIndex, IndexKey, ScanConfig, ScanResult},
        bank::Bank,
    },
    log::*,
    huione_sdk::{
        account::ReadableAccount,
        pubkey::Pubkey,
        stake::{self, state::StakeState},
    },
    huione_stake_program::stake_state,
    std::{collections::HashSet, sync::Arc,str::FromStr},
};

pub struct NonCirculatingSupply {
    pub lamports: u128,
    pub accounts: Vec<Pubkey>,
}

pub fn calculate_non_circulating_supply(bank: &Arc<Bank>) -> ScanResult<NonCirculatingSupply> {
    debug!("Updating Bank supply, epoch: {}", bank.epoch());
    let mut non_circulating_accounts_set: HashSet<Pubkey> = HashSet::new();
    non_circulating_accounts_set.insert(Pubkey::from_str("A6pE1ZnErnh21YSoQTqF7chVmsyJTtdYiY5Wfz4vygwW").unwrap());
    for key in non_circulating_accounts() {
        non_circulating_accounts_set.insert(key);
    }
    // let withdraw_authority_list = withdraw_authority();

    // let clock = bank.clock();
    let config = &ScanConfig::default();
    let stake_accounts = if bank
        .rc
        .accounts
        .accounts_db
        .account_indexes
        .contains(&AccountIndex::ProgramId)
    {
        bank.get_filtered_indexed_accounts(
            &IndexKey::ProgramId(stake::program::id()),
            // The program-id account index checks for Account owner on inclusion. However, due to
            // the current AccountsDb implementation, an account may remain in storage as a
            // zero-lamport Account::Default() after being wiped and reinitialized in later
            // updates. We include the redundant filter here to avoid returning these accounts.
            |account| account.owner() == &stake::program::id(),
            config,
            None,
        )?
    } else {
        bank.get_program_accounts(&stake::program::id(), config)?
    };

    for (pubkey, account) in stake_accounts.iter() {
        let stake_account = stake_state::from(account).unwrap_or_default();
        match stake_account {
            StakeState::Initialized(_meta) => {
                // if meta.lockup.is_in_force(&clock, None)
                //     || withdraw_authority_list.contains(&meta.authorized.withdrawer)
                // {
                    non_circulating_accounts_set.insert(*pubkey);
                // }
            }
            StakeState::Stake(_meta, _stake) => {
                // if meta.lockup.is_in_force(&clock, None)
                //     || withdraw_authority_list.contains(&meta.authorized.withdrawer)
                // {
                    non_circulating_accounts_set.insert(*pubkey);
                // }
            }
            _ => {}
        }
    }

    let lamports = non_circulating_accounts_set
        .iter()
        .map(|pubkey| bank.get_balance(pubkey))
        .sum();

    Ok(NonCirculatingSupply {
        lamports,
        accounts: non_circulating_accounts_set.into_iter().collect(),
    })
}

// Mainnet-beta accounts that should be considered non-circulating
huione_sdk::pubkeys!(
    non_circulating_accounts,
    [
        // "Mc5XB47H3DKJHym5RLa9mPzWv5snERsF3KNv5AauXK8",
    ]
);

// Withdraw authority for autostaked accounts on mainnet-beta
huione_sdk::pubkeys!(
    withdraw_authority,
    [
        // "8CUUMKYNGxdgYio5CLHRHyzMEhhVRMcqefgE6dLqnVRK",
        // "3FFaheyqtyAXZSYxDzsr5CVKvJuvZD1WE1VEsBtDbRqB",
        // "FdGYQdiRky8NZzN9wZtczTBcWLYYRXrJ3LMDhqDPn5rM",
        // "4e6KwQpyzGQPfgVr5Jn3g5jLjbXB4pKPa2jRLohEb1QA",
        // "FjiEiVKyMGzSLpqoB27QypukUfyWHrwzPcGNtopzZVdh",
        // "DwbVjia1mYeSGoJipzhaf4L5hfer2DJ1Ys681VzQm5YY",
        // "GeMGyvsTEsANVvcT5cme65Xq5MVU8fVVzMQ13KAZFNS2",
        // "Bj3aQ2oFnZYfNR1njzRjmWizzuhvfcYLckh76cqsbuBM",
        // "4ZJhPQAgUseCsWhKvJLTmmRRUV74fdoTpQLNfKoekbPY",
        // "HXdYQ5gixrY2H6Y9gqsD8kPM2JQKSaRiohDQtLbZkRWE",
    ]
);

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::genesis_utils::genesis_sysvar_and_builtin_program_lamports,
        huione_sdk::{
            account::{Account, AccountSharedData},
            epoch_schedule::EpochSchedule,
            genesis_config::{ClusterType, GenesisConfig},
            stake::state::{Authorized, Lockup, Meta},
        },
        std::{collections::BTreeMap, sync::Arc},
    };

    fn new_from_parent(parent: &Arc<Bank>) -> Bank {
        Bank::new_from_parent(parent, &Pubkey::default(), parent.slot() + 1)
    }

    #[test]
    fn test_calculate_non_circulating_supply() {
        let mut accounts: BTreeMap<Pubkey, Account> = BTreeMap::new();
        let balance = 10;
        let num_genesis_accounts = 10;
        for _ in 0..num_genesis_accounts {
            accounts.insert(
                huione_sdk::pubkey::new_rand(),
                Account::new(balance, 0, &Pubkey::default()),
            );
        }
        let non_circulating_accounts = non_circulating_accounts();
        let num_non_circulating_accounts = non_circulating_accounts.len() as u64;
        for key in non_circulating_accounts.clone() {
            accounts.insert(key, Account::new(balance, 0, &Pubkey::default()));
        }

        let num_stake_accounts = 3;
        for _ in 0..num_stake_accounts {
            let pubkey = huione_sdk::pubkey::new_rand();
            let meta = Meta {
                authorized: Authorized::auto(&pubkey),
                lockup: Lockup {
                    epoch: 1,
                    ..Lockup::default()
                },
                ..Meta::default()
            };
            let stake_account = Account::new_data_with_space(
                balance,
                &StakeState::Initialized(meta),
                StakeState::size_of(),
                &stake::program::id(),
            )
            .unwrap();
            accounts.insert(pubkey, stake_account);
        }

        let slots_per_epoch = 32;
        let genesis_config = GenesisConfig {
            accounts,
            epoch_schedule: EpochSchedule::new(slots_per_epoch),
            cluster_type: ClusterType::MainnetBeta,
            ..GenesisConfig::default()
        };
        let mut bank = Arc::new(Bank::new_for_tests(&genesis_config));
        assert_eq!(
            bank.capitalization(),
            (num_genesis_accounts + num_non_circulating_accounts + num_stake_accounts) as u128 * balance 
                + genesis_sysvar_and_builtin_program_lamports() + 3,
        );

        let non_circulating_supply = calculate_non_circulating_supply(&bank).unwrap();
        assert_eq!(
            non_circulating_supply.lamports,
            (num_non_circulating_accounts + num_stake_accounts) as u128 * balance 
        );
        assert_eq!(
            non_circulating_supply.accounts.len(),
            num_non_circulating_accounts as usize + num_stake_accounts as usize + 1
        );

        bank = Arc::new(new_from_parent(&bank));
        let new_balance = 11;
        for key in non_circulating_accounts {
            bank.store_account(
                &key,
                &AccountSharedData::new(new_balance, 0, &Pubkey::default()),
            );
        }
        let non_circulating_supply = calculate_non_circulating_supply(&bank).unwrap();
        assert_eq!(
            non_circulating_supply.lamports,
            (num_non_circulating_accounts as u128 * new_balance) + (num_stake_accounts as u128 * balance)
        );
        assert_eq!(
            non_circulating_supply.accounts.len(),
            num_non_circulating_accounts as usize + num_stake_accounts as usize +1
        );

        // Advance bank an epoch, which should unlock stakes
        for _ in 0..slots_per_epoch {
            bank = Arc::new(new_from_parent(&bank));
        }
        assert_eq!(bank.epoch(), 1);
        let non_circulating_supply = calculate_non_circulating_supply(&bank).unwrap();
        assert_eq!(
            non_circulating_supply.lamports,
            num_stake_accounts as u128 * balance 
        );
        assert_eq!(
            non_circulating_supply.accounts.len(),
            num_stake_accounts as usize + 1
        );
    }
}
