extern crate clap;

use crate::exif::enhance_with_exif;
use crate::files::get_matching_files;
use crate::logging::setup_logging;
use crate::settings::{RawImportArgs, get_settings};
use clap::Parser;
use log::{debug, info};

mod settings;
mod logging;
mod files;
mod exif;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> anyhow::Result<()>{
    let args = RawImportArgs::parse();
    let _log_guard = setup_logging(&args);
    rexiv2::initialize()?;

    info!("RAW importer v{VERSION}");

    let settings = get_settings()?;
    debug!("Running with settings {:?}", settings);

    let raw_files = get_matching_files(&settings)?;
    
    for entry in raw_files {
        let enhanched = enhance_with_exif(entry);
        println!("{:?}", enhanched);
    }

    Ok(())
}
