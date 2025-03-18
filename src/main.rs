use std::path::{Path, PathBuf};
use clap::Parser;
use anyhow::{anyhow, Result};
use encoding_rs::SHIFT_JIS;
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

    // List up all files inside the directory (recursively).
    let files = list_files(target_path)?;
    println!("Files to be zipped:");
    for file in &files {
        println!("{}", file.display());
        let converted_name = convert_to_cp932(file)?;
        // Checking for debugging purposes.
        println!("{} -> {}", file.display(), converted_name);
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

/// Convert UTF-8 file name to CP932 encoding.
fn convert_to_cp932(path: &Path) -> Result<String> {
    let utf8_name = path
        .file_name()
        .ok_or_else(|| anyhow!("Failed to get file name"))?
        .to_string_lossy(); // OsStr are not necessarily UTF-8, so handle the cases where conversion is not possible.

    let (encoded, _, _) = SHIFT_JIS.encode(&utf8_name);

    Ok(String::from_utf8_lossy(&encoded).to_string())
}
