use crate::tidii::scan_dir_for_old_files;
use dirs::desktop_dir;

mod tidii;
mod utilities;

fn main() {
    let dir_path = desktop_dir().expect("Could not find the desktop directory.");
    match scan_dir_for_old_files(dir_path, 30) {
        Ok(()) => {
            println!("Scan Complete");
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}
