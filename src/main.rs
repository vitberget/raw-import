use clap::Parser;
use log::info;

use crate::import::import_files;
use crate::logging::setup_logging;
use crate::settings::{RawImportArgs, RawImportCommand, get_settings, show_config, show_default_config};

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

    info!("RAW importer v{VERSION}");

    let settings = get_settings()?;
    
    match args.command {
        None => import_files(None, &args, &settings),
        Some(RawImportCommand::Import { ref from_path }) => import_files(from_path.clone(), &args, &settings),
        Some(RawImportCommand::ShowConfiguration) => show_config(&settings),
        Some(RawImportCommand::DefaultConfiguration) => show_default_config()
    }
}
