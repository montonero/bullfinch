use std::io;
use clap;

/// Error types for Bullfinch
/// At the moment we wrap around other error types
#[derive(Debug)]
pub enum BfError {
    Io(io::Error),
    UnexpectedCommandType,
    PageNotExist,
    Parse(String),
    UrlError(reqwest::UrlError),
    CliError(clap::Error),

}

impl From<reqwest::UrlError> for BfError {
    fn from(e: reqwest::UrlError) -> BfError {
        BfError::UrlError(e)
    }
}

impl From<clap::Error> for BfError {
    fn from(e: clap::Error) -> BfError {
        BfError::CliError(e)
    }
}