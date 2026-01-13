use crate::error::Result;
use crate::fs_utils::list_files;
use chrono::{DateTime, Local};
use std::fs;
use std::path::Path;

pub fn organize(dir: &Path) -> Result<()> {
    let files = list_files(dir)?;

    for file in files {
        let metadata = fs::metadata(&file)?;
        let modified = metadata.modified()?;

        let datetime: DateTime<Local> = modified.into();
        let folder_name = datetime.format("%Y-%m").to_string();

        let target_dir = dir.join(folder_name);
        fs::create_dir_all(&target_dir)?;

        let target_path = target_dir.join(file.file_name().unwrap());
        fs::rename(file, target_path)?;
    }

    Ok(())
}
