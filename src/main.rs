use std::path::Path;
use clap::Parser;
use anyhow::{anyhow, Result};

/// CLI tool for creating zip files that don't get garbled on Windows.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Target directory to zip.
    target_dir: String,

    /// Output zip file name. (optional; defaults to <directory_name>.zip)
    output_file: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let target_path = Path::new(&args.target_dir);
    if !target_path.is_dir() {
        return Err(anyhow!("Directory '{}' does not exist.", args.target_dir));
    }

    // Checking for debugging purposes.
    println!("Directory '{}' exists.", args.target_dir);

    Ok(())
}
