use anyhow::ensure;
use log::debug;

use crate::disk_actions::{copy_file, create_target_paths};
use crate::exif::{enhance_with_exif, DirEntryWithExif};
use crate::files::get_matching_files;
use crate::rename::{rename_entry, EntryWithRename};
use crate::settings::{RawImportArgs, Settings};
use crate::udisks2::{get_partitions, mount, unmount, wait_for_device};

pub(crate) fn import_files(from_path: Option<String>, args: &RawImportArgs, settings: &Settings) -> anyhow::Result<()> {
    rexiv2::initialize()?;
    debug!("Running with settings {:?}", settings);

    let mut raw_files: Vec<DirEntryWithExif> = get_matching_files(from_path, settings)?.into_iter()
        .filter_map(|entry| enhance_with_exif(entry).ok())
        .collect();

    raw_files.sort_by(|a,b| a.date_time.partial_cmp(&b.date_time).unwrap());

    let files: Vec<EntryWithRename> = raw_files.into_iter()
        .enumerate()
        .filter_map(|(index, enhanched)| rename_entry(enhanched, index + 1, settings).ok())
        .collect();

    let total_file_count: usize = files.len();
    let total_file_size: u64 = files.iter().map(|entry| entry.entry.size).sum();

    debug!("total count {total_file_count}, size {total_file_size}");

    ensure!(total_file_count > 0, "No files to be imported found");

    let target_paths: Vec<&String> = files.iter().map(|entry| &entry.path).collect();

    create_target_paths(target_paths, args)?;

    files.iter().for_each(|entry| { copy_file(entry, settings, args, &total_file_count); });

    Ok(())
}

pub(crate) async fn wait_and_import(args: &RawImportArgs, settings: &Settings) -> anyhow::Result<()> {
    loop {
        let device = wait_for_device().await?;

        for partition in get_partitions(&device).await? {
            let path = mount(&partition).await?;
            import_files(Some(path), args, settings)?;
            unmount(&partition).await?;
        }
    }
}
