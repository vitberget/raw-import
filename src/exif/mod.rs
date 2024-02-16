use std::fs::DirEntry;
use rexiv2::Metadata;

use crate::exif::date_time::get_date_time;

use self::date_time::ExifDateTime;

pub(crate) mod date_time;

#[derive(Debug)]
pub(crate) struct DirEntryWithExif {
    pub(crate) entry: DirEntry,
    pub(crate) date_time: ExifDateTime,
    pub(crate) size: u64,
}

pub(crate) fn enhance_with_exif(entry: DirEntry) -> anyhow::Result<DirEntryWithExif> {
    let meta = Metadata::new_from_path(entry.path())?;
    let date_time = get_date_time(&meta)?;

    let size = entry.metadata()?.len();

    Ok(DirEntryWithExif { 
        entry,
        date_time,
        size
    })
}
