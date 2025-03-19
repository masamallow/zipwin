mod args;
mod encoding;
mod zipper;

use std::path::{Path, PathBuf};
use anyhow::{anyhow, Result};
use crate::args::Args;
use crate::zipper::create_zip;

fn main() -> Result<()> {
    let args = Args::parse_args();

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

    create_zip(target_path, &output_file)?;

    Ok(())
}
