use std::collections::HashMap;
use std::io;

use anyhow::{bail, ensure};
use log::{debug, error, info};

use crate::disk_actions::{copy_file, create_target_paths};
use crate::exif::{enhance_with_exif, DirEntryWithExif};
use crate::files::get_matching_files;
use crate::rename::{rename_entry, EntryWithRename};
use crate::settings::{RawImportArgs, Settings};
use crate::udisks2::{get_cards, get_partitions, mount, print_udisk_status, unmount, wait_for_card};

pub(crate) fn import_files(from_path: &String, args: &RawImportArgs, settings: &Settings) -> anyhow::Result<()> {
    rexiv2::initialize()?;

    debug!("Running with settings {:?}", settings);
    info!("Searching for matching files");

    let mut raw_files: Vec<DirEntryWithExif> = get_matching_files(from_path, settings)?.into_iter()
        .filter_map(|entry| enhance_with_exif(entry).ok())
        .collect();

    raw_files.sort_by(|a,b| a.date_time.partial_cmp(&b.date_time).unwrap());

    let files: Vec<EntryWithRename> = raw_files.into_iter()
        .enumerate()
        .filter_map(|(index, enhanched)| rename_entry(enhanched, index + 1, settings).ok())
        .collect();

    let total_file_count: usize = files.len();
    let total_file_size: u64 = files.iter()
        .map(|entry| entry.entry.size)
        .sum();

    info!("Found {total_file_count} raw files, size {total_file_size}");

    ensure!(total_file_count > 0, "No files to be imported found");

    let target_paths: Vec<&String> = files.iter().map(|entry| &entry.path).collect();

    create_target_paths(target_paths, args)?;

    files.iter().for_each(|entry| { copy_file(entry, settings, args, &total_file_count); });

    Ok(())
}

pub(crate) async fn wait_and_import(args: &RawImportArgs, settings: &Settings) -> anyhow::Result<()> {
    let _ = print_udisk_status().await;
    loop {
         match wait_for_card().await {
            Ok(device) =>  import_card(&device, args, settings).await?,
            Err(err) => error!("Error waiting for device {err}"),
        }
    }
}

pub(crate) async fn do_card(args: &RawImportArgs, settings: &Settings) -> anyhow::Result<()> {
    let mut choices: HashMap<usize, String> = HashMap::new();
    let cards = get_cards().await?;
    let mut index: usize = 0;
    for card in cards {
        info!("Card {}", card.info);
        for partition in card.partitions {
            index += 1;
            info!(" {index:>2}: {partition}");
            choices.insert(index, partition);
        }
    }
    info!("");
    info!("Selection? ");

    if index == 0 {
        info!("No suitable card/partition available!");
    } else {
        let mut input_line: String = String::new();
        match io::stdin().read_line(&mut input_line)  {
            Ok(_) => {
                match input_line.trim().parse::<usize>() {
                    Ok(selection) => {
                        match choices.get(&selection) {
                            Some(partition) => {
                                info!("{selection}: {partition} selected, proceeding to import files.");
                                import_card(partition, args, settings).await?;
                            }
                            None => { bail!("{selection} is not in range"); }
                        }
                    }
                    Err(_) => bail!("Input is not a acceptable number")
                }
            }
            Err(_) => bail!("Failed to read input!")
        }
    }

    Ok(())
}


async fn import_card(device: &String, args: &RawImportArgs, settings: &Settings) -> anyhow::Result<()> {
    info!("Importing from {device}");

    for partition in get_partitions(device).await? {
        let path = mount(&partition).await?;
        info!("Mounted {partition} on {path}");

        import_files(&path, args, settings)?;

        unmount(&partition).await?;
        info!("Unmounted {partition}");
    }

    Ok(())
}
