use crate::error::Result;
use crate::fs_utils::list_files;
use std::fs;
use std::path::Path;

pub fn organize(dir: &Path) -> Result<()> {
    for file in list_files(dir)? {
        let ext = file
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("unknown");

        let target_dir = dir.join(ext);
        fs::create_dir_all(&target_dir)?;

        let target_path = target_dir.join(file.file_name().unwrap());
        fs::rename(file, target_path)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn organizes_files_by_extension() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("a.txt")).unwrap();
        File::create(dir.path().join("b.jpg")).unwrap();

        organize(dir.path()).unwrap();

        assert!(dir.path().join("txt/a.txt").exists());
        assert!(dir.path().join("jpg/b.jpg").exists());
    }
}
