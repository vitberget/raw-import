use std::fs::{self, DirEntry};

use crate::settings::Settings;

pub(crate) fn get_matching_files(settings: &Settings) -> anyhow::Result<()>{
    let dir_entries = fs::read_dir(&settings.input.path)?;

    let raw_files: Vec<DirEntry> = dir_entries
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap())
        .filter(|entry| has_correct_extension(entry, settings))
        .collect();

    println!("{:?}", raw_files);

    for file in raw_files {
        println!("{:?}", file.file_name());
    }

    Ok(())
}

fn has_correct_extension(entry: &DirEntry, settings: &Settings) -> bool {
    if let Some(ext) = entry.path().extension() {
        if let Ok(lowercase) = ext.to_ascii_lowercase().into_string() {
            settings.input.file_types.contains(&lowercase)
        } else {
            false
        }
    } else {
        false
    }
}

