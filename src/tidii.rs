use crate::utilities::{days_since_last_access, is_dot_file, system_time_to_date_time, TidiiError};
use chrono::offset;
use chrono::Datelike;
use dirs::desktop_dir;
use std::fs::{create_dir_all, read_dir, rename, DirEntry, Metadata};
use std::path::PathBuf;

pub fn scan_dir_for_old_files(
    dir_path: &PathBuf,
    cutoff_days: u16,
    log_fn: &Box<dyn Fn(&str) -> ()>,
) -> Result<(), TidiiError> {
    assert!(
        dir_path.is_dir(),
        "Not a directory: {}.",
        dir_path.display()
    );

    log_fn("------ ");
    log_fn(&format!(
        "       {}",
        offset::Local::now().format("%Y-%m-%d %H:%M:%S")
    ));
    log_fn(&format!("Scan:  {}", dir_path.display()));

    // Scan the dir for files over n days old.
    for child_entry in read_dir(&dir_path)? {
        let child_entry = child_entry?;
        let child_path = dir_path.clone().join(child_entry.file_name());

        if is_dot_file(&child_path) {
            continue;
        }

        match move_to_file_cabinet_if_old(&child_path, &child_entry, cutoff_days) {
            Ok(moved) => {
                if moved {
                    log_fn(&format!("Moved: {:?}", child_entry.file_name()));
                } else {
                    log_fn(&format!("Left:  {:?}", child_entry.file_name()));
                }
            }
            Err(err) => {
                log_fn(&format!(
                    "Error: {:?} - {:#?}",
                    child_entry.file_name(),
                    err
                ));
            }
        }
    }
    log_fn(&format!(
        "       {}",
        offset::Local::now().format("%Y-%m-%d %H:%M:%S")
    ));

    Ok(())
}

fn move_to_file_cabinet_if_old(
    file_path: &PathBuf,
    file_entry: &DirEntry,
    cutoff_days: u16,
) -> Result<bool, TidiiError> {
    let meta_data = file_entry.metadata()?;
    if !meta_data.is_dir() {
        // File or sym_link
        if days_since_last_access(&meta_data)? >= cutoff_days as f64 {
            move_to_file_cabinet(&file_path, &file_entry, &meta_data)?;

            return Ok(true);
        }
    }

    Ok(false)
}

fn move_to_file_cabinet(
    file_path: &PathBuf,
    dir_entry: &DirEntry,
    meta_data: &Metadata,
) -> Result<(), TidiiError> {
    let month_name = vec![
        "", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    // Build string like: 2023-01 (Jan)
    let accessed_time = system_time_to_date_time(meta_data.accessed()?);
    let (_, year) = accessed_time.year_ce();
    let month = accessed_time.month() as usize;
    let month_str = month_name
        .get(month)
        .expect(&format!("Month index out of bounds: {}.", month));
    let folder_name = format!("{:04}-{:02} ({})", year, month, month_str);

    // ~/Desktop/File Cabinet/2023-01 (Jan)/{file_name}
    let file_cabinet_dir = desktop_dir()
        .expect("Could not find the desktop directory.")
        .join("File Cabinet")
        .join(folder_name);
    let file_cabinet_file_name = file_cabinet_dir.join(dir_entry.file_name());

    // Create the dir (if not present).
    create_dir_all(file_cabinet_dir)?;

    // Move the file from the desktop to the file cabinet.
    rename(file_path, file_cabinet_file_name)?;

    Ok(())
}
