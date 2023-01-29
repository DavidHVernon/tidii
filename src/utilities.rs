use chrono::{DateTime, Local, TimeZone};
use std::fs::Metadata;
use std::path::PathBuf;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

#[derive(Debug)]
pub enum TidiiError {
    StdIoError(std::io::Error),
    SystemTimeError(SystemTimeError),
}

impl From<std::io::Error> for TidiiError {
    fn from(error: std::io::Error) -> Self {
        TidiiError::StdIoError(error)
    }
}

impl From<SystemTimeError> for TidiiError {
    fn from(error: SystemTimeError) -> Self {
        TidiiError::SystemTimeError(error)
    }
}

pub fn system_time_to_date_time(t: SystemTime) -> DateTime<Local> {
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

pub fn days_since_last_access(meta_data: &Metadata) -> Result<f64, TidiiError> {
    let now = SystemTime::now();
    let accessed_time = meta_data.accessed()?;
    let file_age_in_sec = now.duration_since(accessed_time)?.as_secs() as f64;
    // 60 sec / min * 60 min / hour * 24 hour / day
    let file_age_in_days = file_age_in_sec / 60.0 / 60.0 / 24.0;

    Ok(file_age_in_days)
}

pub fn is_dot_file(file_path: &PathBuf) -> bool {
    let first_char = file_path
        .file_name()
        .expect("No file name.")
        .to_string_lossy()
        .chars()
        .nth(0)
        .expect("No file name.");

    if first_char == '.' {
        true
    } else {
        false
    }
}
