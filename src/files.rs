use std::fs::{self, DirEntry};

use crate::settings::Settings;

pub(crate) fn get_matching_files(from_path: Option<String>, settings: &Settings) -> anyhow::Result<Vec<DirEntry>>{
    let files = match from_path {
        Some(path) => fs::read_dir(path)?,
        None => fs::read_dir(&settings.input.path)?
    };

    let raw_files: Vec<DirEntry> = files
        .flatten()
        .filter(|entry| has_correct_extension(entry, settings))
        .collect();

    Ok(raw_files)
}

fn has_correct_extension(entry: &DirEntry, settings: &Settings) -> bool {
    if let Some(ext) = entry.path().extension()
        && let Ok(lowercase) = ext.to_ascii_lowercase().into_string() {
            return settings.input.file_types.contains(&lowercase);
        } 
    false
}

