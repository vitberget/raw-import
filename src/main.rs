extern crate clap;

use clap::Parser;
use log:: info;

use crate::import::import_files;
use crate::logging::setup_logging;
use crate::settings::{get_settings, show_config, RawImportArgs, RawImportCommand};

mod disk_actions;
mod exif;
mod files;
mod import;
mod logging;
mod rename;
mod settings;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> anyhow::Result<()>{
    let args = RawImportArgs::parse();
    let _log_guard = setup_logging(&args);
    rexiv2::initialize()?;

    info!("RAW importer v{VERSION}");

    let settings = get_settings()?;
    
    match args.command {
        RawImportCommand::Import => import_files(&args, &settings),
        RawImportCommand::ShowConfiguration => show_config(&args, &settings),
    }
}

