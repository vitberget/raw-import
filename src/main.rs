use clap::Parser;
use log::info;

use crate::import::{import_files, wait_and_import};
use crate::logging::setup_logging;
use crate::settings::{RawImportArgs, RawImportCommand, get_settings, show_config, show_default_config};

mod disk_actions;
mod exif;
mod files;
mod import;
mod logging;
mod rename;
mod settings;
mod udisks2;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let args = RawImportArgs::parse();
    let _log_guard = setup_logging(&args);

    info!("RAW importer v{APP_VERSION}");

    let settings = get_settings()?;

    match args.command {
        RawImportCommand::Import { ref from_path } => import_files(from_path.clone(), &args, &settings),
        RawImportCommand::WaitForDevice => wait_and_import(&args, &settings).await,
        RawImportCommand::ShowConfiguration => show_config(&settings),
        RawImportCommand::DefaultConfiguration => show_default_config()
    }
}
