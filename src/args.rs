use clap::Parser;

/// CLI tool for creating zip files that don't get garbled on Windows.
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Target directory to zip.
    pub target_dir: String,

    /// Output zip file name. (optional; defaults to <directory_name>.zip)
    pub output_file: Option<String>,
}

impl Args {
    pub fn parse_args() -> Self {
        Args::parse()
    }
}
