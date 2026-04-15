use std::fs::{self, DirEntry};
use std::path::Path;

use crate::settings::Settings;

pub(crate) fn get_matching_files(from_path: Option<String>, settings: &Settings) -> anyhow::Result<Vec<DirEntry>>{
    let text: String = match from_path {
        Some(path) => path.clone(),
        None => settings.input.path.clone()
    };

    let the_path = Path::new(&text);

    get_matching_files_from_path(&the_path, settings)
}

fn get_matching_files_from_path(from_path: &Path, settings: &Settings) -> anyhow::Result<Vec<DirEntry>>{
    let files = fs::read_dir(from_path)?;

    let mut raw_files: Vec<DirEntry> = files
        .flatten()
        .filter(|entry| has_correct_extension(entry, settings))
        .collect();

    if settings.input.recursive {
        let files = fs::read_dir(from_path)?;
        for dir in files.flatten().filter(|entry| entry.path().is_dir()) {
            let mut entries = get_matching_files_from_path(&dir.path(), settings)?;
            raw_files.append(&mut entries);
        }
    }

    Ok(raw_files)
}

fn has_correct_extension(entry: &DirEntry, settings: &Settings) -> bool {
    if let Some(ext) = entry.path().extension()
        && let Ok(lowercase) = ext.to_ascii_lowercase().into_string() {
            return settings.input.file_types.contains(&lowercase);
        } 
    false
}
