use chrono::{DateTime, Datelike, Local, TimeZone};
use dirs::desktop_dir;
use std::fs::{create_dir_all, read_dir, rename, DirEntry, Metadata};
use std::path::PathBuf;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

#[derive(Debug)]
enum TidiiError {
    IoError(std::io::Error),
    SystemTimeError(SystemTimeError),
}

impl From<std::io::Error> for TidiiError {
    fn from(error: std::io::Error) -> Self {
        TidiiError::IoError(error)
    }
}

impl From<SystemTimeError> for TidiiError {
    fn from(error: SystemTimeError) -> Self {
        TidiiError::SystemTimeError(error)
    }
}

fn system_time_to_date_time(t: SystemTime) -> DateTime<Local> {
    let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => {
            // unlikely but should be handled
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        }
    };
    Local.timestamp_opt(sec, nsec).single().expect("Boom!")
}

fn duration_in_days(meta_data: &Metadata) -> Result<f64, TidiiError> {
    let now = SystemTime::now();
    let accessed_time = meta_data.accessed()?;
    let file_age_in_sec = now.duration_since(accessed_time)?.as_secs() as f64;

    // 60 sec / min * 60 min / hour * 24 hour / day
    Ok(file_age_in_sec / 60.0 / 60.0 / 24.0)
}

fn move_to_file_cabinet(
    file_path: &PathBuf,
    dir_entry: &DirEntry,
    meta_data: &Metadata,
) -> Result<(), TidiiError> {
    let month_name = vec![
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
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

fn sweep(dir_path: PathBuf, n: u16) -> Result<(), TidiiError> {
    assert!(
        dir_path.is_dir(),
        "Invalid directory: {}.",
        dir_path
            .to_str()
            .expect("Input directory path can not be parsed.")
    );

    // Scan the dir for files over n days old.
    for dir_entry in read_dir(&dir_path)? {
        let dir_entry = dir_entry?;
        let meta_data = dir_entry.metadata()?;
        if !meta_data.is_dir() {
            // File or sym_link
            if duration_in_days(&meta_data)? >= n as f64 {
                // This file should be moved to the file cabinet.
                let file_path = dir_path.clone().join(dir_entry.file_name());
                move_to_file_cabinet(&file_path, &dir_entry, &meta_data)?;
            }
        }
    }

    Ok(())
}

fn main() {
    let dir_path = desktop_dir().expect("Could not find the desktop directory.");
    if let Err(err) = sweep(dir_path, 30) {
        panic!("{:#?}", err);
    }
}
