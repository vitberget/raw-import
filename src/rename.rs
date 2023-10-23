use std::{path::Path, ffi::OsString};

use anyhow::{bail, Context};

use crate::exif::DirEntryWithExif;
use crate::settings::Settings;

#[derive(Debug)]
pub(crate) struct EntryWithRename {
    pub(crate) entry: DirEntryWithExif,
    pub(crate) new_name: String,
}

pub(crate) fn rename_entry(enhanched: Result<DirEntryWithExif, anyhow::Error>, index:usize, settings: &Settings) -> anyhow::Result<EntryWithRename> {
    let dewe = enhanched?;
    let entry = &dewe.entry;
    let date_time = &dewe.date_time;


    let (stem, extension) = get_stem_and_extension(&entry.file_name())?;

    let filename_pattern = &settings.output.filename;
   
    let new_name = filename_pattern
        .replace("{yyyy}", date_time.year.as_str())
        .replace("{MM}", date_time.month.as_str())
        .replace("{dd}", date_time.day.as_str())
        .replace("{HH}", date_time.hour.as_str())
        .replace("{mm}", date_time.minute.as_str())
        .replace("{ss}", date_time.second.as_str())
        .replace("{seq}", format!("{:04}", index).as_str())
        .replace("{filename}", stem.as_str())
        .replace("{extension}", extension.as_str()) ;

    Ok(EntryWithRename {
        entry: dewe,
        new_name,
    })

}

fn get_stem_and_extension(full_name: &OsString) -> anyhow::Result<(String, String)> {
    match full_name.to_str() {
        Some(filename) => {
            let path = Path::new(filename);
            let extension: String = path.extension()
                .context("Not an extension")
                ?.to_str()
                .context("Could not unwrap extension")?
                .to_string();
            let stem: String = path.file_stem()
                .context("Not a stem")
                ?.to_str()
                .context("Could not unwrap stem")?
                .to_string();
            Ok((stem, extension))
        },
        None => bail!("Error getting filename from dir entry"),
    }
}
