use clap::Parser;
use anyhow::Result;

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

    // Checking argument contents (for debugging)
    println!("Target directory: {}", args.target_dir);
    if let Some(out) = args.output_file.as_ref() {
        println!("Output file name: {}", out);
    } else {
        println!("Output file name omitted");
    }

    Ok(())
}
