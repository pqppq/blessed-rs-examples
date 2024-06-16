use std::io;

use thiserror::Error;

pub fn example() {}

#[derive(Error, Debug)]
pub enum DataStoreError {
    // generate Display trail by #[error(...)]
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}
