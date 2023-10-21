use std::fs::{self, DirEntry};

use crate::settings::Settings;

pub(crate) fn get_matching_files(settings: &Settings) -> anyhow::Result<Vec<DirEntry>>{
    let raw_files: Vec<DirEntry> = fs::read_dir(&settings.input.path)?
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap())
        .filter(|entry| has_correct_extension(entry, settings))
        .collect();

    Ok(raw_files)
}

fn has_correct_extension(entry: &DirEntry, settings: &Settings) -> bool {
    if let Some(ext) = entry.path().extension() {
        if let Ok(lowercase) = ext.to_ascii_lowercase().into_string() {
            return settings.input.file_types.contains(&lowercase);
        } 
    } 
    return false
}

