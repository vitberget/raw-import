extern crate clap;

use crate::disk_actions::{copy_file, create_target_paths};
use crate::exif::{enhance_with_exif, DirEntryWithExif};
use crate::files::get_matching_files;
use crate::logging::setup_logging;
use crate::rename::{rename_entry, EntryWithRename};
use crate::settings::{get_settings, RawImportArgs};
use clap::Parser;
use log::{debug, info};

mod settings;
mod logging;
mod files;
mod exif;
mod rename;
mod disk_actions;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> anyhow::Result<()>{
    let args = RawImportArgs::parse();
    let _log_guard = setup_logging(&args);
    rexiv2::initialize()?;

    info!("RAW importer v{VERSION}");

    let settings = get_settings()?;
    debug!("Running with settings {:?}", settings);

    let mut raw_files: Vec<DirEntryWithExif> = get_matching_files(&settings)?.into_iter()
        .filter_map(|entry| enhance_with_exif(entry).ok())
        .collect();

    raw_files.sort_by(|a,b| a.date_time.partial_cmp(&b.date_time).unwrap());

    let files: Vec<EntryWithRename> = raw_files.into_iter()
        .enumerate()
        .filter_map(|(index, enhanched)| rename_entry(enhanched, index, &settings).ok())
        .collect();

    let total_file_count: usize = files.len();
    let total_file_size: u64 = files.iter().map(|entry| entry.entry.size).sum();

    debug!("total count {total_file_count}, size {total_file_size}");

    let target_paths: Vec<&String> = files.iter().map(|entry| &entry.path).collect();

    create_target_paths(target_paths, &args)?;

    files.iter().for_each(|entry| { copy_file(entry, &settings, &args, &total_file_count); });

    Ok(())
}
