use {
    crate::blockstore::Blockstore,
    crossbeam_channel::bounded,
    log::*,
    huione_measure::measure::Measure,
    huione_sdk::clock::Slot,
    std::{
        collections::HashSet,
        result::Result,
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        time::Duration,
    },
};

// Attempt to upload this many blocks in parallel
const NUM_BLOCKS_TO_UPLOAD_IN_PARALLEL: usize = 32;

// Read up to this many blocks from blockstore before blocking on the upload process
const BLOCK_READ_AHEAD_DEPTH: usize = NUM_BLOCKS_TO_UPLOAD_IN_PARALLEL * 2;

pub async fn upload_confirmed_blocks(
    blockstore: Arc<Blockstore>,
    hbase: huione_storage_hbase::LedgerStorage,
    starting_slot: Slot,
    ending_slot: Option<Slot>,
    force_reupload: bool,
    exit: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut measure = Measure::start("entire upload");

    info!("Loading ledger slots starting at {}...", starting_slot);

    // let  get_db_blocks = Measure::start("get_db_blocks");
    // let blockstore_slots: Vec<_> = blockstore
    //     .slot_meta_iterator(starting_slot)
    //     .map_err(|err| {
    //         format!(
    //             "Failed to load entries starting from slot {}: {:?}",
    //             starting_slot, err
    //         )
    //     })?
    //     .filter_map(|(slot, _slot_meta)| {
    //         if let Some(ending_slot) = &ending_slot {
    //             if slot > *ending_slot {
    //                 return None;
    //             }
    //         }
    //         Some(slot)
    //     })
    //     .collect();
    // get_db_blocks.stop();
    // info!("get blocks times:{} s", get_db_blocks.as_s());
    // if blockstore_slots.is_empty() {
    //     return Err(format!(
    //         "Ledger has no slots from {} to {:?}",
    //         starting_slot, ending_slot
    //     )
    //     .into());
    // }
    let end_solt = ending_slot.unwrap_or(starting_slot+1000);
    let mut  blockstore_slots: Vec<u64> = vec![];
    for i in starting_slot..end_solt {
        blockstore_slots.push(i)
    }


    info!(
        "Found {} slots in the range ({}, {})",
        blockstore_slots.len(),
        blockstore_slots.first().unwrap(),
        blockstore_slots.last().unwrap()
    );

    // Gather the blocks that are already present in hbase, by slot
    let hbase_slots = if !force_reupload {
        let mut hbase_slots = vec![];
        let first_blockstore_slot = *blockstore_slots.first().unwrap();
        let last_blockstore_slot = *blockstore_slots.last().unwrap();
        info!(
            "Loading list of hbase blocks between slots {} and {}...",
            first_blockstore_slot, last_blockstore_slot
        );

        let mut start_slot = *blockstore_slots.first().unwrap();
        while start_slot <= last_blockstore_slot {
            let mut next_hbase_slots = loop {
                match hbase.get_confirmed_blocks(start_slot, 1000) {
                    Ok(slots) => break slots,
                    Err(err) => {
                        error!("get_confirmed_blocks for {} failed: {:?}", start_slot, err);
                        // Consider exponential backoff...
                        tokio::time::sleep(Duration::from_secs(2)).await;
                    }
                }
            };
            if next_hbase_slots.is_empty() {
                break;
            }
            hbase_slots.append(&mut next_hbase_slots);
            start_slot = hbase_slots.last().unwrap() + 1;
        }
        hbase_slots
            .into_iter()
            .filter(|slot| *slot <= last_blockstore_slot)
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    // The blocks that still need to be uploaded is the difference between what's already in the
    // hbase and what's in blockstore...
    let blocks_to_upload = {
        let blockstore_slots = blockstore_slots.iter().cloned().collect::<HashSet<_>>();
        let hbase_slots = hbase_slots.into_iter().collect::<HashSet<_>>();

        let mut blocks_to_upload = blockstore_slots
            .difference(&hbase_slots)
            .cloned()
            .collect::<Vec<_>>();
        blocks_to_upload.sort_unstable();
        blocks_to_upload
    };
    if blocks_to_upload.is_empty() {
        info!("No blocks need to be uploaded to hbase");
        return Ok(());
    }
    info!(
        "{} blocks to be uploaded to the bucket in the range ({}, {})",
        blocks_to_upload.len(),
        blocks_to_upload.first().unwrap(),
        blocks_to_upload.last().unwrap()
    );

    // Load the blocks out of blockstore in a separate thread to allow for concurrent block uploading
    let (_loader_thread, receiver) = {
        let exit = exit.clone();

        let (sender, receiver) = bounded(BLOCK_READ_AHEAD_DEPTH);
        (
            std::thread::spawn(move || {
                let mut measure = Measure::start("block loader thread");
                for (i, slot) in blocks_to_upload.iter().enumerate() {
                    if exit.load(Ordering::Relaxed) {
                        break;
                    }

                    let _ = match blockstore.get_rooted_block(*slot, true) {
                        Ok(confirmed_block) => sender.send((*slot, Some(confirmed_block))),
                        Err(err) => {
                            warn!(
                                "Failed to get load confirmed block from slot {}: {:?}",
                                slot, err
                            );
                            sender.send((*slot, None))
                        }
                    };

                    if i > 0 && i % NUM_BLOCKS_TO_UPLOAD_IN_PARALLEL == 0 {
                        info!(
                            "{}% of blocks processed ({}/{})",
                            i * 100 / blocks_to_upload.len(),
                            i,
                            blocks_to_upload.len()
                        );
                    }
                }
                measure.stop();
                info!("{} to load {} blocks", measure, blocks_to_upload.len());
            }),
            receiver,
        )
    };
    let mut failures = 0;
    use futures::stream::StreamExt;

    let mut stream =
        tokio_stream::iter(receiver.into_iter()).chunks(NUM_BLOCKS_TO_UPLOAD_IN_PARALLEL);

    while let Some(blocks) = stream.next().await {
        if exit.load(Ordering::Relaxed) {
            break;
        }

        let mut measure_upload = Measure::start("Upload");
        let mut num_blocks = blocks.len();
        info!("Preparing the next {} blocks for upload", num_blocks);

        let uploads = blocks.into_iter().filter_map(|(slot, block)| match block {
            None => {
                num_blocks -= 1;
                None
            }
            Some(confirmed_block) => Some(hbase.upload_confirmed_block(slot, confirmed_block)),
        });

        for result in futures::future::join_all(uploads).await {
            if result.is_err() {
                error!("upload_confirmed_block() failed: {:?}", result.err());
                failures += 1;
            }
        }

        measure_upload.stop();
        info!("{} for {} blocks", measure_upload, num_blocks);
    }

    measure.stop();
    info!("{}", measure);
    if failures > 0 {
        Err(format!("Incomplete upload, {} operations failed", failures).into())
    } else {
        Ok(())
    }
}
