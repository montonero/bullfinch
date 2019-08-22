use actix_web::{error, http, HttpResponse};
use clap;
use std::fmt;
use std::io;

/// Error types for Bullfinch
/// At the moment we wrap around other error types
#[derive(Debug)]
pub enum BfError {
    Io(io::Error),
    UnexpectedCommandType,
    Parse(String),
    UrlError(reqwest::UrlError),
    CliError(clap::Error),
    DomainNotRegistered(u32),
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


impl From<io::Error> for BfError {
    fn from(e: io::Error) -> BfError {
        BfError::Io(e)
    }
}

impl error::ResponseError for BfError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::new(http::StatusCode::BAD_REQUEST)
    }
}

impl fmt::Display for BfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            BfError::UrlError(err) => write!(f, "(UrlError {})", err.to_string()),
            BfError::DomainNotRegistered(domain_id) => {
                write!(f, "(Domain Not Registered. Domain id: {})", domain_id)
            }
            _ => write!(f, "(Other bullfinch error)"),
        }
    }
}
