mod args;
mod encoding;
mod zipper;

use crate::args::Args;
use crate::zipper::create_zip;
use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let args = Args::parse_args();

    let target_path = Path::new(&args.target_dir);
    if !target_path.is_dir() {
        eprintln!("Error: Directory '{}' does not exist.", args.target_dir);
        std::process::exit(1);
    }

    // Determine output zip file name
    let output_file = match &args.output_file {
        Some(name) => PathBuf::from(name),
        None => {
            let dir_name = target_path
                .file_name()
                .ok_or_else(|| anyhow!("Invalid directory name"))?
                .to_string_lossy()
                .to_string();
            PathBuf::from(format!("{}.zip", dir_name))
        }
    };

    if let Err(e) = create_zip(target_path, &output_file) {
        eprintln!("Error: Failed to create ZIP file: {}", e);
        std::process::exit(1);
    }

    println!("zip file created successfully: {:?}", output_file);
    Ok(())
}
