use huione_sdk::{
    account::{Account, AccountSharedData},
    bpf_loader_upgradeable::UpgradeableLoaderState,
    pubkey::Pubkey,
    rent::Rent,
};

mod hpl_token {
    huione_sdk::declare_id!("HuiToken11111111111111111111111111111111111");
}
mod hpl_memo_1_0 {
    huione_sdk::declare_id!("8jjXD6tToXM9zJiLG44t1P4nhrf8zrVLy3T39CvBEgaa");
}
mod hpl_memo_3_0 {
    huione_sdk::declare_id!("HuiMemo111111111111111111111111111111111111");
}
mod hpl_associated_token_account {
    huione_sdk::declare_id!("HuiATA1111111111111111111111111111111111111");
}

static HPL_PROGRAMS: &[(Pubkey, &[u8])] = &[
    (hpl_token::ID, include_bytes!("programs/hpl_token.so")),
    (
        hpl_memo_1_0::ID,
        include_bytes!("programs/hpl_memo.so"),
    ),
    (
        hpl_memo_3_0::ID,
        include_bytes!("programs/hpl_memo.so"),
    ),
    (
        hpl_associated_token_account::ID,
        include_bytes!("programs/hpl_associated_token_account.so"),
    ),
];

pub fn hpl_programs(rent: &Rent) -> Vec<(Pubkey, AccountSharedData)> {
    HPL_PROGRAMS
        .iter()
        .map(|(program_id, elf)| {
            (
                *program_id,
                AccountSharedData::from(Account {
                    lamports: rent.minimum_balance(elf.len()).min(1),
                    data: elf.to_vec(),
                    owner: huione_sdk::bpf_loader::id(),
                    executable: true,
                    rent_epoch: 0,
                }),
            )
        })
        .collect()
}

static SPL_PROGRAMS: &[(Pubkey, Pubkey, &[u8])] = &[
    (
        hpl_token::ID,
        huione_sdk::bpf_loader::ID,
        include_bytes!("programs/hpl_token.so"),
    ),
    (
        hpl_memo_1_0::ID,
        huione_sdk::bpf_loader_upgradeable::ID,
        include_bytes!("programs/hpl_token.so"),
    ),
    (
        hpl_memo_1_0::ID,
        huione_sdk::bpf_loader::ID,
        include_bytes!("programs/hpl_memo.so"),
    ),
    (
        hpl_memo_3_0::ID,
        huione_sdk::bpf_loader::ID,
        include_bytes!("programs/hpl_memo.so"),
    ),
    (
        hpl_associated_token_account::ID,
        huione_sdk::bpf_loader::ID,
        include_bytes!("programs/hpl_associated_token_account.so"),
    ),
];

pub fn spl_programs(rent: &Rent) -> Vec<(Pubkey, AccountSharedData)> {
    SPL_PROGRAMS
        .iter()
        .flat_map(|(program_id, loader_id, elf)| {
            let mut accounts = vec![];
            let data = if *loader_id == huione_sdk::bpf_loader_upgradeable::ID {
                let (programdata_address, _) =
                    Pubkey::find_program_address(&[program_id.as_ref()], loader_id);
                let mut program_data = bincode::serialize(&UpgradeableLoaderState::ProgramData {
                    slot: 0,
                    upgrade_authority_address: Some(Pubkey::default()),
                })
                .unwrap();
                program_data.extend_from_slice(elf);
                accounts.push((
                    programdata_address,
                    AccountSharedData::from(Account {
                        lamports: rent.minimum_balance(program_data.len()).max(1),
                        data: program_data,
                        owner: *loader_id,
                        executable: false,
                        rent_epoch: 0,
                    }),
                ));
                bincode::serialize(&UpgradeableLoaderState::Program {
                    programdata_address,
                })
                .unwrap()
            } else {
                elf.to_vec()
            };
            accounts.push((
                *program_id,
                AccountSharedData::from(Account {
                    lamports: rent.minimum_balance(data.len()).max(1),
                    data,
                    owner: *loader_id,
                    executable: true,
                    rent_epoch: 0,
                }),
            ));
            accounts.into_iter()
        })
        .collect()
}
