extern crate clap;

use crate::exif::enhance_with_exif;
use crate::files::get_matching_files;
use crate::logging::setup_logging;
use crate::rename::{rename_entry, EntryWithRename};
use crate::settings::{RawImportArgs, get_settings};
use clap::Parser;
use log::{debug, info, trace};

mod settings;
mod logging;
mod files;
mod exif;
mod rename;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> anyhow::Result<()>{
    let args = RawImportArgs::parse();
    let _log_guard = setup_logging(&args);
    rexiv2::initialize()?;

    info!("RAW importer v{VERSION}");

    let settings = get_settings()?;
    debug!("Running with settings {:?}", settings);

    let raw_files = get_matching_files(&settings)?;

    let files: Vec<EntryWithRename> = raw_files.into_iter()
        .map(|entry| enhance_with_exif(entry))
        .enumerate()
        .map(|(index, enhanched)| rename_entry(enhanched, index, &settings))
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap())
        .collect();

    let total_file_count: usize = files.len();
    let total_file_size: u64 = files.iter().map(|entry| entry.entry.size).sum();

    debug!("total count {total_file_count}, size {total_file_size}");

    Ok(())
}
