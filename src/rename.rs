use std::ffi::OsString;
use std::path::Path;

use anyhow::{bail, Context};

use crate::exif::DirEntryWithExif;
use crate::settings::Settings;

#[derive(Debug)]
pub(crate) struct EntryWithRename {
    pub(crate) entry: DirEntryWithExif,
    pub(crate) new_name: String,
    pub(crate) path: String,
    pub(crate) index: usize,
}

pub(crate) fn rename_entry(enhanched: DirEntryWithExif, index: usize, settings: &Settings) -> anyhow::Result<EntryWithRename> {
    let entry = &enhanched.entry;
    let date_time = &enhanched.date_time;

    let (filename, extension) = get_filename_and_extension(&entry.file_name())?;

    let new_name = settings.output.filename
        .replace("{yyyy}", date_time.year.as_str())
        .replace("{MM}", date_time.month.as_str())
        .replace("{dd}", date_time.day.as_str())
        .replace("{HH}", date_time.hour.as_str())
        .replace("{mm}", date_time.minute.as_str())
        .replace("{ss}", date_time.second.as_str())
        .replace("{seq}", format!("{:04}", index).as_str())
        .replace("{filename}", filename.as_str())
        .replace("{extension}", extension.as_str()) ;

    let path = settings.output.path
        .replace("{yyyy}", date_time.year.as_str())
        .replace("{MM}", date_time.month.as_str())
        .replace("{dd}", date_time.day.as_str())
        .replace("{HH}", date_time.hour.as_str())
        .replace("{mm}", date_time.minute.as_str())
        .replace("{ss}", date_time.second.as_str())
        .replace("{seq}", format!("{:04}", index).as_str())
        .replace("{filename}", filename.as_str())
        .replace("{extension}", extension.as_str()) ;

    Ok(EntryWithRename {
        entry: enhanched,
        new_name,
        path,
        index
    })
}

fn get_filename_and_extension(full_name: &OsString) -> anyhow::Result<(String, String)> {
    match full_name.to_str() {
        Some(filename) => {
            let path = Path::new(filename);

            let stem: String = path.file_stem()
                .context("Not a stem")?.to_str()
                .context("Could not unwrap stem")?.to_string();

            let extension: String = path.extension()
                .context("Not an extension")?.to_str()
                .context("Could not unwrap extension")?.to_string();

            Ok((stem, extension))
        },
        None => bail!("Error getting filename from dir entry"),
    }
}
