extern crate clap;

use crate::logging::setup_logging;
use crate::settings::{RawImportArgs, get_settings};
use clap::Parser;
use log::{debug, info};

mod settings;
mod logging;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> anyhow::Result<()>{
    let args = RawImportArgs::parse();
    let _log_guard = setup_logging(&args);
    info!("RAW importer v{VERSION}");

    let settings = get_settings()?;
    debug!("Running with settings {:?}", settings);

    Ok(())
}
