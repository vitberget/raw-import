use std::fs::DirEntry;
use rexiv2::Metadata;

use crate::exif::date_time::get_date_time;

use self::date_time::ExifDateTime;

pub(crate) mod date_time;

#[derive(Debug)]
pub(crate) struct DirEntryWithExif {
    entry: DirEntry,
    date_time: ExifDateTime 
}


pub(crate) fn enhance_with_exif(entry: DirEntry) -> anyhow::Result<DirEntryWithExif> {
    let meta = Metadata::new_from_path(entry.path())?;
    let date_time = get_date_time(&meta)?;

    println!("meta! {:?} {:?}", meta, date_time);

    Ok(DirEntryWithExif { 
        entry,
        date_time
    })
}


