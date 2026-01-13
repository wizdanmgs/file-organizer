use crate::error::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub fn list_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let entries = fs::read_dir(dir)?
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect();

    Ok(entries)
}
