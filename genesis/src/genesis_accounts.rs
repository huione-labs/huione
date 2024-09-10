use {
    crate::{
        stakes::{create_and_add_stakes, StakerInfo},
        unlocks::UnlockInfo,
    },
    huione_sdk::{genesis_config::GenesisConfig},
};

// 9 month schedule is 100% after 9 months
// const UNLOCKS_ALL_AT_9_MONTHS: UnlockInfo = UnlockInfo {
//     cliff_fraction: 1.0,
//     cliff_years: 0.75,
//     unlocks: 0,
//     unlock_years: 0.0,
//     custodian: "Mc5XB47H3DKJHym5RLa9mPzWv5snERsF3KNv5AauXK8",
// };

// 9 month schedule is 50% after 9 months, then monthly for 2 years
// const UNLOCKS_HALF_AT_9_MONTHS: UnlockInfo = UnlockInfo {
//     cliff_fraction: 0.5,
//     cliff_years: 0.75,
//     unlocks: 24,
//     unlock_years: 2.0,
//     custodian: "Mc5XB47H3DKJHym5RLa9mPzWv5snERsF3KNv5AauXK8",
// };

// no lockups
// const UNLOCKS_ALL_DAY_ZERO: UnlockInfo = UnlockInfo {
//     cliff_fraction: 1.0,
//     cliff_years: 0.0,
//     unlocks: 0,
//     unlock_years: 0.0,
//     custodian: "Mc5XB47H3DKJHym5RLa9mPzWv5snERsF3KNv5AauXK8",
// };

fn _add_stakes(
    genesis_config: &mut GenesisConfig,
    staker_infos: &[StakerInfo],
    unlock_info: &UnlockInfo,
) -> u128 {
    staker_infos
        .iter()
        .map(|staker_info| create_and_add_stakes(genesis_config, staker_info, unlock_info, None))
        .sum::<u128>()
}

pub fn add_genesis_accounts(_genesis_config: &mut GenesisConfig, issued_lamports: u128) {
    println!("issued_lamports:{}",issued_lamports);
    // create_and_add_stakes(
    //     genesis_config,
    //     &StakerInfo {
    //         name: "one thanks",
    //         staker: "5D1WMVSEbW9SKSDHazGy25TLkMnDAm3DPfv5TcefaHCu",
    //         lamports: (50000224000 * LAMPORTS_PER_HC).saturating_sub(issued_lamports),
    //         withdrawer: Some("8owPWrSc5gmmts1LfbhQgfi1qmfAAN3swdkFafL3PkPX"),
    //     },
    //     &UNLOCKS_ALL_DAY_ZERO,
    //     None,
    // );
    return ;
 
}

#[cfg(test)]
mod tests {
    use super::*;

    // ????
    #[test]
    fn test_add_genesis_accounts() {
        let mut genesis_config = GenesisConfig::default();

        add_genesis_accounts(&mut genesis_config, 0);

        let lamports = genesis_config
            .accounts
            .values()
            .map(|account| account.lamports)
            .sum::<u128>();

        // assert_eq!(500_000 * LAMPORTS_PER_HC, lamports);
        assert_eq!(0, lamports);
        
    }
}
