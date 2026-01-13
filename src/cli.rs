use crate::error::{OrganizerError, Result};
use std::path::PathBuf;

pub struct CliArgs {
    pub path: PathBuf,
    pub mode: String,
}

pub fn parse() -> Result<CliArgs> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 || args[2] != "--by" {
        return Err(OrganizerError::InvalidArgs);
    }

    Ok(CliArgs {
        path: PathBuf::from(&args[1]),
        mode: args[3].clone(),
    })
}
