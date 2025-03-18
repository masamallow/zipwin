use std::path::{Path, PathBuf};
use clap::Parser;
use anyhow::{anyhow, Result};
use walkdir::WalkDir;

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

    // List up all files inside the directory (recursively).
    let files = list_files(target_path)?;
    println!("Files to be zipped:");
    for file in &files {
        println!("{}", file.display());
    }

    Ok(())
}

/// Recursively lists all files inside a given directory.
fn list_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            files.push(path.to_path_buf());
        }
    }

    Ok(files)
}
