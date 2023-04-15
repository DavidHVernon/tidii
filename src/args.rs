use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Number of days to allow items to remain on the desktop.
    #[arg(short, long)]
    pub days: Option<u16>,

    /// Frequency (in hours) to run a desktop scan.
    #[arg(short, long)]
    pub frequency: Option<u64>,
}

impl Args {
    pub fn parse_command_line() -> Args {
        Args::parse()
    }
}
