use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const LOG_FILE: &str = ".kudu.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveRecord {
    pub from: PathBuf,
    pub to: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UndoLog {
    pub moves: Vec<MoveRecord>,
    pub created_dirs: Vec<PathBuf>,
}

pub fn save_log(base: &Path, log: &UndoLog) -> Result<()> {
    let path = base.join(LOG_FILE);
    fs::write(path, serde_json::to_string_pretty(log)?)?;
    Ok(())
}

pub fn load_log(base: &Path) -> Result<UndoLog> {
    let path = base.join(LOG_FILE);
    let data = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&data)?)
}

pub fn clear_log(base: &Path) -> Result<()> {
    let path = base.join(LOG_FILE);
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}
