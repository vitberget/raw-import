extern crate clap;

use crate::settings::get_settings;
use clap::Parser;
use log::{debug, info};

mod settings;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(author,version,about)]
struct RawImportArgs {
    #[arg(short, long, default_value = "info")]
    verbosity: String,
}

fn main() -> anyhow::Result<()>{
    
    let args = RawImportArgs::parse();

    println!("args {:?}", args);

    info!("RAW importer v{VERSION}");
    
    let settings = get_settings()?;
    debug!("Running with settings {:?}", settings);

    Ok(())
}
