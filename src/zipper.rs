use std::fs::File;
use std::path::{Path, PathBuf};
use anyhow::Result;
use walkdir::WalkDir;
use zip::write::{ExtendedFileOptions, FileOptions};
use zip::ZipWriter;
use crate::encoding::convert_to_cp932;

/// Create a zip file from a target directory.
pub fn create_zip(target_path: &Path, output_file: &PathBuf) -> Result<()> {
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
