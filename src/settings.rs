use std::collections::HashSet;
use std::{fs, io};

use anyhow::format_err;
use clap::{CommandFactory as _, Parser, Subcommand, ValueEnum};
use clap_complete::{Generator, Shell, generate};
use config::builder::DefaultState;
use config::{Config, File, FileFormat, ConfigBuilder};
use log::{debug, info};

#[derive(Parser, Debug)]
pub(crate) struct RawImportArgs {
    #[command(subcommand)]
    pub(crate) command: RawImportCommand,

    #[arg(short, long, value_enum, default_value_t = RawImportLogLevel::Info)]
    pub(crate) verbosity: RawImportLogLevel,

    #[arg(short, long)]
    pub(crate) dry_run: Option<bool>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum RawImportCommand {
    /// Import raw files
    Import {  
        #[arg(short, long)]
        from_path: Option<String> 
    },

    /// Show information about configuration from all sources
    ShowConfiguration,

    /// Show default configuration
    DefaultConfiguration,

    /// Wait for device being inserted, mount, import, unmount, repeat
    WaitForDevice,

    /// Shell completion
    Completion { shell: Shell }
}

#[derive(ValueEnum,Clone,Debug)]
pub(crate) enum RawImportLogLevel {
    Info,
    Debug,
    Trace
}

#[derive(Debug)]
pub(crate) struct Settings {
    pub(crate) input: InputSettings,
    pub(crate) output: OutputSettings
}

#[derive(Debug)]
pub(crate) struct InputSettings {
    pub(crate) path: String, 
    pub(crate) file_types: HashSet<String>,
    pub(crate) recursive: bool
}

#[derive(Debug)]
pub(crate) struct OutputSettings {
    pub(crate) path: String, 
    pub(crate) filename: String, 
    pub(crate) duplicates: DuplicateAction,
}

#[derive(Debug)]
pub(crate) enum DuplicateAction {
    Ignore,
    AlterName,
    Overwrite
}

impl TryFrom<String> for DuplicateAction {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "ignore" => Ok(Self::Ignore),
            "alter_name" => Ok(Self::AlterName),
            "altername" => Ok(Self::AlterName),
            "overwrite" => Ok(Self::Overwrite),

            bad_value => Err(format_err!("Could not parse {bad_value} into an DuplicateAction"))
        }
    }
}


pub(crate) fn get_settings() -> anyhow::Result<Settings> {
    let config = get_config()?;

    let file_types: HashSet<String> = config.get_array("input.file_types")?
        .iter()
        .map(|value| value.to_string().to_lowercase())
        .collect();

    Ok(Settings {
        input: InputSettings { 
            path: config.get_string("input.path")?,
            file_types,
            recursive: config.get_bool("input.recursive")?,
        },
        output: OutputSettings { 
            path: config.get_string("output.path")?,
            filename: config.get_string("output.filename")?,
            duplicates: config.get_string("output.duplicates")?.try_into()?,
        }
    })
}

fn get_config() -> anyhow::Result<Config> {
    let builder = Config::builder()
        .add_source(File::from_str(
            include_str!("../resources/default_properties.toml"),
            FileFormat::Toml));
    Ok(add_xdg_config_file(builder).build()?)
}

fn add_xdg_config_file(builder: ConfigBuilder<DefaultState>) -> ConfigBuilder<DefaultState> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("raw-import");
    if let Ok(config_path) = xdg_dirs.place_config_file("configuration.toml")
    && let Some(config_path) = config_path.to_str() {
        debug!("Adding config file {config_path}");
        return builder.add_source(File::with_name(config_path).required(false));
    }
    builder
}

fn get_xdg_config_file_content() -> Option<(String, String)> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("raw-import");
    if let Ok(config_path) = xdg_dirs.place_config_file("configuration.toml")
    && let Some(config_filename) = config_path.to_str() {
        let content = match fs::read_to_string(&config_path) {
            Ok(content) => content,
            Err(error) => format!("{}", error).to_string()
        };
        return Some((config_filename.to_string(), content));
    }
    None
}

pub(crate) fn show_config(settings: &Settings) -> anyhow::Result<()> {
    info!("Running with settings:");
    info!("{settings:?}");
    info!("");
    show_default_config()?;
    info!("");

    match get_xdg_config_file_content() {
        Some((file_path, file_content)) => {
            info!("=== XDG Config settings from {file_path} ===");
            info!("{file_content}");
        },
        None => info!("=== No XDG Config settings ===")
    }

    Ok(())
}

pub(crate) fn show_default_config() -> anyhow::Result<()> {
    info!("=== Default settings ===");
    info!("{}", include_str!("../resources/default_properties.toml"));
    Ok(())
}

pub(crate) fn print_completions<G: Generator>(generator: G) -> anyhow::Result<()> {
    let mut cmd = RawImportArgs::command();
    let name = cmd.get_name().to_string();
    generate(generator, &mut cmd, name, &mut io::stdout());
    Ok(())
}
