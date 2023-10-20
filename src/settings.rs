use config::{Config, File, FileFormat, ConfigBuilder};
use config::builder::DefaultState;
use log::debug;

#[derive(Debug)]
pub(crate) struct Settings {
    pub(crate) input: InputSettings,
    pub(crate) output: OutputSettings
}

#[derive(Debug)]
pub(crate) struct InputSettings {
    pub(crate) path: String, 
    pub(crate) filter: Vec<String>
}

#[derive(Debug)]
pub(crate) struct OutputSettings {
    pub(crate) base: String, 
    pub(crate) path: String, 
    pub(crate) filename: String, 
}

pub(crate) fn get_settings() -> anyhow::Result<Settings> {
    let config = get_config()?;

    let filter: Vec<String> = config.get_array("input.filter")?
        .iter()
        .map(|value| value.to_string())
        .collect();

    Ok(Settings {
        input: InputSettings { 
            path: config.get_string("input.path")?,
            filter,
        },
        output: OutputSettings { 
            base: config.get_string("output.base")?,
            path: config.get_string("output.path")?,
            filename: config.get_string("output.filename")?,
        }
    })
}

fn get_config() -> anyhow::Result<Config> {
    let builder = Config::builder()
        .add_source(File::from_str(
                include_str!("../resources/default_properties.toml"),
                FileFormat::Toml));
    let builder = add_config_file(builder);
    Ok(builder.build()?)
}

fn add_config_file(builder: ConfigBuilder<DefaultState>) -> ConfigBuilder<DefaultState> {
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
