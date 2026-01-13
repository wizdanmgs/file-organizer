mod cli;
mod error;
mod fs_utils;
mod organizer;

use error::{OrganizerError, Result};

fn main() -> Result<()> {
    let args = cli::parse()?;

    match args.mode.as_str() {
        "extension" => organizer::by_extension::organize(&args.path),
        "date" => organizer::by_date::organize(&args.path),
        other => Err(OrganizerError::UnknownMode(other.to_string())),
    }
}
