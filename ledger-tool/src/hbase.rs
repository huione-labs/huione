//! The `hbase` subcommand
use {
    crate::ledger_path::canonicalize_ledger_path,
    clap::{
        value_t, value_t_or_exit, values_t_or_exit, App, AppSettings, Arg, ArgMatches, SubCommand,
    },
    log::info,
    serde_json::json,
    huione_clap_utils::{
        input_parsers::pubkey_of,
        input_validators::{is_slot, is_valid_pubkey},
    },
    huione_cli_output::{
        display::println_transaction, CliBlock, CliTransaction, CliTransactionConfirmation,
        OutputFormat,
    },
    huione_ledger::{blockstore::Blockstore, blockstore_options::AccessType,},
    huione_sdk::{clock::Slot, pubkey::Pubkey, signature::Signature},
    huione_transaction_status::{
        BlockEncodingOptions, ConfirmedBlock, EncodeError, TransactionDetails,
        UiTransactionEncoding,
    },
    std::{
        collections::HashSet,
        path::Path,
        process::exit,
        result::Result,
        sync::{atomic::AtomicBool, Arc},
    },
};

async fn upload(
    thrift2_url: String,
    blockstore: Blockstore,
    starting_slot: Slot,
    ending_slot: Option<Slot>,
    force_reupload: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let hbase = huione_storage_hbase::LedgerStorage::new(thrift2_url,None)
        .map_err(|err| format!("Failed to connect to storage: {:?}", err))?;

    huione_ledger::hbase_upload::upload_confirmed_blocks(
        Arc::new(blockstore),
        hbase,
        starting_slot,
        ending_slot,
        force_reupload,
        Arc::new(AtomicBool::new(false)),
    )
    .await
}

async fn delete_slots(thrift2_url: String, slots: Vec<Slot>, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    let hbase = huione_storage_hbase::LedgerStorage::new(thrift2_url,None)
        .map_err(|err| format!("Failed to connect to storage: {:?}", err))?;

    huione_ledger::hbase_delete::delete_confirmed_blocks(hbase, slots, dry_run)
}

async fn first_available_block(thrift2_url: String) -> Result<(), Box<dyn std::error::Error>> {
    let hbase = huione_storage_hbase::LedgerStorage::new(thrift2_url,None)?;
    match hbase.get_first_available_block()? {
        Some(block) => println!("{}", block),
        None => println!("No blocks available"),
    }

    Ok(())
}

async fn block(thrift2_url: String, slot: Slot, outhuione_format: OutputFormat) -> Result<(), Box<dyn std::error::Error>> {
    let hbase = huione_storage_hbase::LedgerStorage::new(thrift2_url,None)
        .map_err(|err| format!("Failed to connect to storage: {:?}", err))?;

    let confirmed_block = hbase.get_confirmed_block(slot)?;
    let encoded_block = confirmed_block
        .encode_with_options(
            UiTransactionEncoding::Base64,
            BlockEncodingOptions {
                transaction_details: TransactionDetails::Full,
                show_rewards: true,
                max_supported_transaction_version: None,
            },
        )
        .map_err(|err| match err {
            EncodeError::UnsupportedTransactionVersion(version) => {
                format!(
                    "Failed to process unsupported transaction version ({}) in block",
                    version
                )
            }
        })?;

    let cli_block = CliBlock {
        encoded_confirmed_block: encoded_block.into(),
        slot,
    };
    println!("{}", outhuione_format.formatted_string(&cli_block));
    Ok(())
}

async fn blocks(thrift2_url: String, starting_slot: Slot, limit: usize) -> Result<(), Box<dyn std::error::Error>> {
    let hbase = huione_storage_hbase::LedgerStorage::new(thrift2_url,None)
        .map_err(|err| format!("Failed to connect to storage: {:?}", err))?;

    let slots = hbase.get_confirmed_blocks(starting_slot, limit)?;
    println!("{:?}", slots);
    println!("{} blocks found", slots.len());

    Ok(())
}

async fn compare_blocks(
    thrift2_url: String,
    starting_slot: Slot,
    limit: usize,
    credential_path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    assert!(!credential_path.is_empty());

    let owned_hbase = huione_storage_hbase::LedgerStorage::new(thrift2_url,None)
        .map_err(|err| format!("failed to connect to owned hbase: {:?}", err))?;
    let owned_hbase_slots = owned_hbase
        .get_confirmed_blocks(starting_slot, limit)?;
    info!(
        "owned hbase {} blocks found ",
        owned_hbase_slots.len()
    );
    let reference_hbase =
        huione_storage_hbase::LedgerStorage::new(credential_path,None)
            .map_err(|err| format!("failed to connect to reference hbase: {:?}", err))?;

    let reference_hbase_slots = reference_hbase
        .get_confirmed_blocks(starting_slot, limit)?;
    info!(
        "reference hbase {} blocks found ",
        reference_hbase_slots.len(),
    );

    println!(
        "{}",
        json!({
            "num_reference_slots": json!(reference_hbase_slots.len()),
            "num_owned_slots": json!(owned_hbase_slots.len()),
            "reference_last_block": json!(reference_hbase_slots.len().checked_sub(1).map(|i| reference_hbase_slots[i])),
            "missing_blocks":  json!(missing_blocks(&reference_hbase_slots, &owned_hbase_slots)),
        })
    );

    Ok(())
}

async fn confirm(
    thrift2_url: String,
    signature: &Signature,
    verbose: bool,
    outhuione_format: OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    let hbase = huione_storage_hbase::LedgerStorage::new(thrift2_url,None)
        .map_err(|err| format!("Failed to connect to storage: {:?}", err))?;

    let transaction_status = hbase.get_signature_status(signature)?;

    let mut transaction = None;
    let mut get_transaction_error = None;
    if verbose {
        match hbase.get_confirmed_transaction(signature) {
            Ok(Some(confirmed_tx)) => {
                let decoded_tx = confirmed_tx.get_transaction();
                let encoded_tx_with_meta = confirmed_tx
                    .tx_with_meta
                    .encode(UiTransactionEncoding::Json, Some(0), true)
                    .map_err(|_| "Failed to encode transaction in block".to_string())?;
                transaction = Some(CliTransaction {
                    transaction: encoded_tx_with_meta.transaction,
                    meta: encoded_tx_with_meta.meta,
                    block_time: confirmed_tx.block_time,
                    slot: Some(confirmed_tx.slot),
                    decoded_transaction: decoded_tx,
                    prefix: "  ".to_string(),
                    sigverify_status: vec![],
                });
            }
            Ok(None) => {}
            Err(err) => {
                get_transaction_error = Some(format!("{:?}", err));
            }
        }
    }
    let cli_transaction = CliTransactionConfirmation {
        confirmation_status: Some(transaction_status.confirmation_status()),
        transaction,
        get_transaction_error,
        err: transaction_status.err.clone(),
    };
    println!("{}", outhuione_format.formatted_string(&cli_transaction));
    Ok(())
}

pub async fn transaction_history(
    thrift2_url: String,
    address: &Pubkey,
    mut limit: usize,
    mut before: Option<Signature>,
    until: Option<Signature>,
    verbose: bool,
    show_transactions: bool,
    query_chunk_size: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let hbase = huione_storage_hbase::LedgerStorage::new(thrift2_url,None)?;

    let mut loaded_block: Option<(Slot, ConfirmedBlock)> = None;
    while limit > 0 {
        let results = hbase
            .get_confirmed_signatures_for_address(
                address,
                before.as_ref(),
                until.as_ref(),
                limit.min(query_chunk_size),
            )?;

        if results.is_empty() {
            break;
        }
        before = Some(results.last().unwrap().0.signature);
        assert!(limit >= results.len());
        limit = limit.saturating_sub(results.len());

        for (result, index) in results {
            if verbose {
                println!(
                    "{}, slot={}, memo=\"{}\", status={}",
                    result.signature,
                    result.slot,
                    result.memo.unwrap_or_else(|| "".to_string()),
                    match result.err {
                        None => "Confirmed".to_string(),
                        Some(err) => format!("Failed: {:?}", err),
                    }
                );
            } else {
                println!("{}", result.signature);
            }

            if show_transactions {
                // Instead of using `hbase.get_confirmed_transaction()`, fetch the entire block
                // and keep it around.  This helps reduce Hbase query traffic and speeds up the
                // results for high-volume addresses
                loop {
                    if let Some((slot, block)) = &loaded_block {
                        if *slot == result.slot {
                            match block.transactions.get(index as usize).map(|tx_with_meta| {
                                (
                                    tx_with_meta.get_transaction(),
                                    tx_with_meta.get_status_meta(),
                                )
                            }) {
                                None => {
                                    println!(
                                        "  Transaction info for {} is corrupt",
                                        result.signature
                                    );
                                }
                                Some((transaction, meta)) => {
                                    println_transaction(
                                        &transaction,
                                        meta.map(|m| m.into()).as_ref(),
                                        "  ",
                                        None,
                                        None,
                                    );
                                }
                            }
                            break;
                        }
                    }
                    match hbase.get_confirmed_block(result.slot) {
                        Err(err) => {
                            println!("  Unable to get confirmed transaction details: {}", err);
                            break;
                        }
                        Ok(confirmed_block) => {
                            loaded_block = Some((result.slot, confirmed_block));
                        }
                    }
                }
                println!();
            }
        }
    }
    Ok(())
}

pub trait HbaseSubCommand {
    fn hbase_subcommand(self) -> Self;
}

impl HbaseSubCommand for App<'_, '_> {
    fn hbase_subcommand(self) -> Self {
        self.subcommand(
            SubCommand::with_name("hbase")
                .about("Ledger data on a Hbase instance")
                .setting(AppSettings::InferSubcommands)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("upload")
                        .about("Upload the ledger to Hbase")
                        .arg(
                            Arg::with_name("starting_slot")
                                .long("starting-slot")
                                .validator(is_slot)
                                .value_name("START_SLOT")
                                .takes_value(true)
                                .index(1)
                                .help(
                                    "Start uploading at this slot [default: first available slot]",
                                ),
                        )
                        .arg(
                            Arg::with_name("ending_slot")
                                .long("ending-slot")
                                .validator(is_slot)
                                .value_name("END_SLOT")
                                .takes_value(true)
                                .index(2)
                                .help("Stop uploading at this slot [default: last available slot]"),
                        )
                        .arg(
                            Arg::with_name("hbase_rpc")
                                .long("hbase-rpc")
                                .value_name("HBASE_RPC")
                                .takes_value(true)
                                .index(3)
                                .help("Hbase thrift2 RPC"),
                        )
                        .arg(
                            Arg::with_name("force_reupload")
                                .long("force")
                                .takes_value(false)
                                .help(
                                    "Force reupload of any blocks already present in Hbase instance\
                                    Note: reupload will *not* delete any data from the tx-by-addr table;\
                                    Use with care.",
                                ),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("delete-slots")
                        .about("Delete ledger information from Hbase")
                        .arg(
                            Arg::with_name("slots")
                                .index(1)
                                .value_name("SLOTS")
                                .takes_value(true)
                                .multiple(true)
                                .required(true)
                                .help("Slots to delete"),
                        )
                        .arg(
                            Arg::with_name("hbase_rpc")
                                .long("hbase-rpc")
                                .value_name("HBASE_RPC")
                                .takes_value(true)
                                .help("Hbase thrift2 RPC"),
                        )
                        .arg(
                            Arg::with_name("force")
                                .long("force")
                                .takes_value(false)
                                .help(
                                    "Deletions are only performed when the force flag is enabled. \
                                    If force is not enabled, show stats about what ledger data \
                                    will be deleted in a real deletion. "),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("first-available-block")
                        .about("Get the first available block in the storage")
                        .arg(
                            Arg::with_name("hbase_rpc")
                                .long("hbase-rpc")
                                .value_name("HBASE_RPC")
                                .takes_value(true)
                                .index(1)
                                .help("Hbase thrift2 RPC"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("blocks")
                        .about("Get a list of slots with confirmed blocks for the given range")
                        .arg(
                            Arg::with_name("starting_slot")
                                .long("starting-slot")
                                .validator(is_slot)
                                .value_name("SLOT")
                                .takes_value(true)
                                .index(1)
                                .required(true)
                                .default_value("0")
                                .help("Start listing at this slot"),
                        )
                        .arg(
                            Arg::with_name("limit")
                                .long("limit")
                                .validator(is_slot)
                                .value_name("LIMIT")
                                .takes_value(true)
                                .index(2)
                                .required(true)
                                .default_value("1000")
                                .help("Maximum number of slots to return"),
                        ).arg(
                            Arg::with_name("hbase_rpc")
                                .long("hbase-rpc")
                                .value_name("HBASE_RPC")
                                .takes_value(true)
                                .index(3)
                                .help("Hbase thrift2 RPC"),
                        ),
                        
                )
                .subcommand(
                    SubCommand::with_name("compare-blocks")
                        .about("Find the missing confirmed blocks of an owned hbase for a given range \
                                by comparing to a reference hbase")
                        .arg(
                            Arg::with_name("starting_slot")
                                .validator(is_slot)
                                .value_name("SLOT")
                                .takes_value(true)
                                .index(1)
                                .required(true)
                                .default_value("0")
                                .help("Start listing at this slot"),
                        )
                        .arg(
                            Arg::with_name("limit")
                                .validator(is_slot)
                                .value_name("LIMIT")
                                .takes_value(true)
                                .index(2)
                                .required(true)
                                .default_value("1000")
                                .help("Maximum number of slots to check"),
                        )
                        .arg(
                            Arg::with_name("hbase_rpc")
                                .long("hbase-rpc")
                                .value_name("HBASE_RPC")
                                .takes_value(true)
                                .index(3)
                                .help("Hbase thrift2 RPC"),
                        )
                        .arg(
                            Arg::with_name("reference_credential")
                                .long("reference-credential")
                                .short("c")
                                .value_name("REFERENCE_CREDENTIAL_FILEPATH")
                                .takes_value(true)
                                .required(true)
                                .help("File path for a credential to a reference hbase"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("block")
                        .about("Get a confirmed block")
                        .arg(
                            Arg::with_name("slot")
                                .long("slot")
                                .validator(is_slot)
                                .value_name("SLOT")
                                .takes_value(true)
                                .index(1)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("hbase_rpc")
                                .long("hbase-rpc")
                                .value_name("HBASE_RPC")
                                .takes_value(true)
                                .index(2)
                                .help("Hbase thrift2 RPC"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("confirm")
                        .about("Confirm transaction by signature")
                        .arg(
                            Arg::with_name("signature")
                                .long("signature")
                                .value_name("TRANSACTION_SIGNATURE")
                                .takes_value(true)
                                .required(true)
                                .index(1)
                                .help("The transaction signature to confirm"),
                        )
                        .arg(
                            Arg::with_name("hbase_rpc")
                                .long("hbase-rpc")
                                .value_name("HBASE_RPC")
                                .takes_value(true)
                                .index(2)
                                .help("Hbase thrift2 RPC"),
                        ),
                        
                )
                .subcommand(
                    SubCommand::with_name("transaction-history")
                        .about(
                            "Show historical transactions affecting the given address \
                             from newest to oldest",
                        )
                        .arg(
                            Arg::with_name("address")
                                .index(1)
                                .value_name("ADDRESS")
                                .required(true)
                                .validator(is_valid_pubkey)
                                .help("Account address"),
                        )
                        .arg(
                            Arg::with_name("limit")
                                .long("limit")
                                .takes_value(true)
                                .value_name("LIMIT")
                                .validator(is_slot)
                                .index(2)
                                .default_value("18446744073709551615")
                                .help("Maximum number of transaction signatures to return"),
                        )
                        .arg(
                            Arg::with_name("hbase_rpc")
                                .long("hbase-rpc")
                                .value_name("HBASE_RPC")
                                .takes_value(true)
                                .index(3)
                                .help("Hbase thrift2 RPC"),
                        )
                        .arg(
                            Arg::with_name("query_chunk_size")
                                .long("query-chunk-size")
                                .takes_value(true)
                                .value_name("AMOUNT")
                                .validator(is_slot)
                                .default_value("1000")
                                .help(
                                    "Number of transaction signatures to query at once. \
                                       Smaller: more responsive/lower throughput. \
                                       Larger: less responsive/higher throughput",
                                ),
                        )
                        .arg(
                            Arg::with_name("before")
                                .long("before")
                                .value_name("TRANSACTION_SIGNATURE")
                                .takes_value(true)
                                .help("Start with the first signature older than this one"),
                        )
                        .arg(
                            Arg::with_name("until")
                                .long("until")
                                .value_name("TRANSACTION_SIGNATURE")
                                .takes_value(true)
                                .help("End with the last signature newer than this one"),
                        )
                        .arg(
                            Arg::with_name("show_transactions")
                                .long("show-transactions")
                                .takes_value(false)
                                .help("Display the full transactions"),
                        ),
                ),
        )
    }
}

pub fn hbase_process_command(ledger_path: &Path, matches: &ArgMatches<'_>) {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    let verbose = matches.is_present("verbose");
    let force_update_to_open = matches.is_present("force_update_to_open");
    let outhuione_format = OutputFormat::from_matches(matches, "outhuione_format", verbose);

    let future = match matches.subcommand() {
        ("upload", Some(arg_matches)) => {
            let starting_slot = value_t!(arg_matches, "starting_slot", Slot).unwrap_or(0);
            let ending_slot = value_t!(arg_matches, "ending_slot", Slot).ok();
            let rpc = value_t!(arg_matches, "hbase_rpc", String).unwrap();
            let force_reupload = arg_matches.is_present("force_reupload");
            let blockstore = crate::open_blockstore(
                &canonicalize_ledger_path(ledger_path),
                AccessType::Secondary,
                None,
                force_update_to_open,
            );

            runtime.block_on(upload(
                rpc,
                blockstore,
                starting_slot,
                ending_slot,
                force_reupload,
            ))
        }
        ("delete-slots", Some(arg_matches)) => {
            let slots = values_t_or_exit!(arg_matches, "slots", Slot);
            let rpc = value_t!(arg_matches, "hbase_rpc", String).unwrap();
            let dry_run = !arg_matches.is_present("force");
            runtime.block_on(delete_slots(rpc,slots, dry_run))
        }
        ("first-available-block", Some(arg_matches)) => {
            let rpc = value_t!(arg_matches, "hbase_rpc", String).unwrap();
            runtime.block_on(first_available_block(rpc))
        }
        ("block", Some(arg_matches)) => {
            let slot = value_t_or_exit!(arg_matches, "slot", Slot);
            let rpc = value_t!(arg_matches, "hbase_rpc", String).unwrap();
            runtime.block_on(block(rpc,slot, outhuione_format))
        }
        ("blocks", Some(arg_matches)) => {
            let starting_slot = value_t_or_exit!(arg_matches, "starting_slot", Slot);
            let rpc = value_t!(arg_matches, "hbase_rpc", String).unwrap();
            let limit = value_t_or_exit!(arg_matches, "limit", usize);

            runtime.block_on(blocks(rpc,starting_slot, limit))
        }
        ("compare-blocks", Some(arg_matches)) => {
            let starting_slot = value_t_or_exit!(arg_matches, "starting_slot", Slot);
            let rpc = value_t!(arg_matches, "hbase_rpc", String).unwrap();
            let limit = value_t_or_exit!(arg_matches, "limit", usize);
            let reference_credential_filepath =
                value_t_or_exit!(arg_matches, "reference_credential", String);

            runtime.block_on(compare_blocks(
                rpc,
                starting_slot,
                limit,
                reference_credential_filepath,
            ))
        }
        ("confirm", Some(arg_matches)) => {
            let signature = arg_matches
                .value_of("signature")
                .unwrap()
                .parse()
                .expect("Invalid signature");

            let rpc = value_t!(arg_matches, "hbase_rpc", String).unwrap();
            runtime.block_on(confirm(rpc,&signature, verbose, outhuione_format))
        }
        ("transaction-history", Some(arg_matches)) => {
            let address = pubkey_of(arg_matches, "address").unwrap();
            let limit = value_t_or_exit!(arg_matches, "limit", usize);
            let rpc = value_t!(arg_matches, "hbase_rpc", String).unwrap();
            let query_chunk_size = value_t_or_exit!(arg_matches, "query_chunk_size", usize);
            let before = arg_matches
                .value_of("before")
                .map(|signature| signature.parse().expect("Invalid signature"));
            let until = arg_matches
                .value_of("until")
                .map(|signature| signature.parse().expect("Invalid signature"));
            let show_transactions = arg_matches.is_present("show_transactions");

            runtime.block_on(transaction_history(
                rpc,
                &address,
                limit,
                before,
                until,
                verbose,
                show_transactions,
                query_chunk_size,
            ))
        }
        _ => unreachable!(),
    };

    future.unwrap_or_else(|err| {
        eprintln!("{:?}", err);
        exit(1);
    });
}

fn missing_blocks(reference: &[Slot], owned: &[Slot]) -> Vec<Slot> {
    if owned.is_empty() && !reference.is_empty() {
        return reference.to_owned();
    } else if owned.is_empty() {
        return vec![];
    }

    let owned_hashset: HashSet<_> = owned.iter().collect();
    let mut missing_slots = vec![];
    for slot in reference {
        if !owned_hashset.contains(slot) {
            missing_slots.push(slot.to_owned());
        }
    }
    missing_slots
}
