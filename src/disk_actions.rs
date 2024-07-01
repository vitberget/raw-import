use std::fs;
use std::path::{Path, PathBuf};

use anyhow::bail;
use log::{debug, info};

use crate::rename::EntryWithRename;
use crate::settings::{DuplicateAction, RawImportArgs, Settings};

pub(crate) fn create_target_paths(target_paths: Vec<&String>, args: &RawImportArgs) -> anyhow::Result<()> {
    for str in target_paths {
        debug!("Checking target directory {str}");

        let target_dir_path = Path::new(str);

        if target_dir_path.exists() && !target_dir_path.is_dir() {
            bail!("{str} is not a directory");
        }
        if !target_dir_path.exists() {
            info!("Creating target directory {str}");
            if args.dry_run == Some(true) {
                info!("  Dry run, no action");
            } else {
                fs::create_dir_all(target_dir_path)?;
            }
        }
    }  
    Ok(())
}

pub(crate) fn copy_file(entry: &EntryWithRename, settings: &Settings, args: &RawImportArgs, total_file_count: &usize) {
    let source_file = entry.entry.entry.path();
    let percentage = (100_f32 * entry.index as f32) / *total_file_count as f32;
    info!("Copying: {} -> {}/{}    ({}/{}  {:.1}%)", 
        source_file.to_str().unwrap(), 
        entry.path, 
        entry.new_name,
        entry.index +1,
        total_file_count,
        percentage
        );
    let dir = Path::new(&entry.path);
    let target_file = dir.join(Path::new(&entry.new_name));
    match (target_file.exists(), &settings.output.duplicates)  {
        (true, DuplicateAction::Ignore) => info!("  {} exists, not copying (ignoring)", target_file.to_str().unwrap()),
        (true, DuplicateAction::AlterName) => todo!("AlterName not yet implemented"),
        (true, DuplicateAction::Overwrite) => {
            info!("  {} exists, replacing", target_file.to_str().unwrap());
            actually_copy_file(source_file, target_file, args);
        }
        (false, _) => {
            actually_copy_file(source_file, target_file, args);
        }
    }
}

fn actually_copy_file(source_file: PathBuf, target_file: PathBuf, args: &RawImportArgs) {
    if args.dry_run == Some(true) {
        info!("  Dry run, no action");
    } else if let Err(error) = fs::copy(source_file, target_file) {
        info!("  Error: {}", error);
    }
}


