use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrganizerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Time error: {0}")]
    Time(#[from] chrono::ParseError),

    #[error("Invalid arguments")]
    InvalidArgs,

    #[error("Unknown organize mode: {0}")]
    UnknownMode(String),
}

pub type Result<T> = std::result::Result<T, OrganizerError>;
