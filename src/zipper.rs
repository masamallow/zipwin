use crate::encoding::convert_to_cp932;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zip::ZipWriter;
use zip::write::{ExtendedFileOptions, FileOptions};

/// Create a zip file from a target directory.
pub fn create_zip(target_path: &Path, output_file: &PathBuf) -> Result<()> {
    let zip_file = File::create(&output_file)?;
    let mut zip = ZipWriter::new(zip_file);

    let files: Vec<_> = WalkDir::new(target_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .collect();

    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(ProgressStyle::default_bar().template("[{bar:40}] {pos}/{len} {msg}")?);

    // List up all files inside the directory (recursively).
    println!("Files to be zipped:");
    // Iterate over directory and add files directly to zip.
    for entry in files {
        let path = entry.path();
        // OsStr are not necessarily UTF-8, so handle the cases where conversion is not possible.
        let relative_path = path.strip_prefix(target_path)?.to_string_lossy();
        let converted_name = convert_to_cp932(&relative_path)?;

        // Set compression method
        let options: FileOptions<'_, ExtendedFileOptions> =
            FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        // Add file to zip
        match zip.start_file(converted_name.clone(), options) {
            Ok(_) => {
                println!("{}", path.display());
                let mut file = File::open(path)?;
                std::io::copy(&mut file, &mut zip)?;
                pb.inc(1);
                pb.set_message(converted_name);
            }
            Err(e) => {
                eprintln!("Warning: Failed to add file '{}': {}", converted_name, e);
            }
        }
    }

    pb.finish_with_message("zip creation complete.");
    zip.finish()?;

    Ok(())
}
