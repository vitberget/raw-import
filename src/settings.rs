use std::collections::HashSet;

use anyhow::format_err;
use clap::{Parser, ValueEnum};
use config::{Config, File, FileFormat, ConfigBuilder};
use config::builder::DefaultState;
use log::debug;

#[derive(Parser, Debug)]
#[command(author,version,about)]
pub(crate) struct RawImportArgs {
    #[arg(short, long, value_enum, default_value_t = RawImportLogLevel::Info)]
    pub(crate) verbosity: RawImportLogLevel,
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
    pub(crate) file_types: HashSet<String>
}

#[derive(Debug)]
pub(crate) struct OutputSettings {
    pub(crate) base: String, 
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
        },
        output: OutputSettings { 
            base: config.get_string("output.base")?,
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
    let builder = add_xdg_config_file(builder);
    Ok(builder.build()?)
}

fn add_xdg_config_file(builder: ConfigBuilder<DefaultState>) -> ConfigBuilder<DefaultState> {
    if let Ok(xdg_dirs) = xdg::BaseDirectories::with_prefix("raw-import") {
        if let Ok(config_path) = xdg_dirs.place_config_file("configuration.toml") {
            if let Some(config_path) = config_path.to_str() {
                debug!("Adding config file {config_path}");
                return builder.add_source(File::with_name(config_path).required(false));
            }
        }
    }
    return builder
}
