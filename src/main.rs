use crate::tidii::scan_dir_for_old_files;
use crate::utilities::get_log_fn;
use dirs::desktop_dir;
use std::thread::sleep;
use std::time::Duration;
use utilities::hours_to_sec;

mod tidii;
mod utilities;

fn main() {
    let log_fn = get_log_fn();
    let dir_path = desktop_dir().expect("Could not find the desktop directory.");

    loop {
        match scan_dir_for_old_files(&dir_path, 8, &log_fn) {
            Ok(()) => {
                log_fn("Scan Complete");
                sleep(Duration::from_secs(hours_to_sec(12)));
            }
            Err(err) => {
                log_fn(&format!("{:#?}", err));
            }
        }
    }
}
