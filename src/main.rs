use dirs::desktop_dir;
use std::thread::sleep;
use std::time::Duration;
use tidii::scan_dir_for_old_files;
use utilities::{get_log_fn, hours_to_sec};

mod tidii;
mod utilities;

fn main() {
    let log_fn = get_log_fn();
    let desktop_dir_path = desktop_dir().expect("Could not find the desktop directory.");

    loop {
        match scan_dir_for_old_files(&desktop_dir_path, 8, &log_fn) {
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
