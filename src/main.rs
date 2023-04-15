use crate::args::Args;
use crate::tidii::tidii_run_loop;

mod args;
mod tidii;
mod utilities;

fn main() {
    tidii_run_loop(Args::parse_command_line())
}
