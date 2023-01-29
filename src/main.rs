use crate::tidii::scan_dir_for_old_files;
use crate::utilities::get_log_fn;
use dirs::desktop_dir;

mod tidii;
mod utilities;

fn main() {
    let log_fn = get_log_fn();
    let dir_path = desktop_dir().expect("Could not find the desktop directory.");
    match scan_dir_for_old_files(dir_path, 30, &log_fn) {
        Ok(()) => {
            log_fn("Scan Complete");
        }
        Err(err) => {
            log_fn(&format!("{:#?}", err));
        }
    }
}
