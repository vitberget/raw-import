use anyhow::bail;
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

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let args = RawImportArgs::parse();
    let _log_guard = setup_logging(&args);

    info!("RAW importer v{VERSION}");

    // play_udisk2("sdc").await?;

    let settings = get_settings()?;

    match args.command {
        None => import_files(None, &args, &settings),
        Some(RawImportCommand::Import { ref from_path }) => import_files(from_path.clone(), &args, &settings),
        Some(RawImportCommand::ShowConfiguration) => show_config(&settings),
        Some(RawImportCommand::DefaultConfiguration) => show_default_config()
    }
}

async fn play_udisk2(device: &str) -> anyhow::Result<()> {
    let client = udisks2::Client::new().await?;
    let object = client
        .object(format!("/org/freedesktop/UDisks2/block_devices/{device}"))
        .expect("No {device} device found");

    info!("obj {object:?}");
    info!("");
    let block = object.block().await?;
    info!("x {block:?}");
    info!("");
    
    let dfb = client.drive_for_block(&block).await?;
    info!("dfb {dfb:?}");
    info!("");

    bail!("not so fast there now");
}
