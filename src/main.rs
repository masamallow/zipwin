use std::fs::File;
use std::path::{Path, PathBuf};
use anyhow::{anyhow, Result};
use clap::Parser;
use encoding_rs::SHIFT_JIS;
use walkdir::WalkDir;
use zip::write::{ExtendedFileOptions, FileOptions};
use zip::ZipWriter;

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

    // Determine output zip file name
    let output_file = match &args.output_file {
        Some(name) => PathBuf::from(name),
        None => {
            let dir_name = target_path.file_name()
                .ok_or_else(|| anyhow!("Invalid directory name"))?
                .to_string_lossy()
                .to_string();
            PathBuf::from(format!("{}.zip", dir_name))
        }
    };

    let zip_file = File::create(&output_file)?;
    let mut zip = ZipWriter::new(zip_file);

    // List up all files inside the directory (recursively).
    println!("Files to be zipped:");
    // Iterate over directory and add files directly to zip.
    for entry in WalkDir::new(target_path) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            println!("{}", path.display());
            // OsStr are not necessarily UTF-8, so handle the cases where conversion is not possible.
            let relative_path = path.strip_prefix(target_path)?.to_string_lossy();
            let converted_name = convert_to_cp932(&relative_path)?;

            // Set compression method
            let options: FileOptions<'_, ExtendedFileOptions> = FileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated);

            // Add file to zip
            zip.start_file(converted_name, options)?;
            let mut file = File::open(path)?;
            std::io::copy(&mut file, &mut zip)?;
        }
    }

    zip.finish()?;
    println!("zip file created: {:?}", output_file);

    Ok(())
}

/// Convert UTF-8 file name to CP932 encoding.
fn convert_to_cp932(filename: &str) -> Result<String> {
    let (encoded, _, _) = SHIFT_JIS.encode(filename);
    Ok(String::from_utf8_lossy(&encoded).to_string())
}
