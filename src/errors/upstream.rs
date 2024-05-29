use crate::UpstreamError::Parsing;
use std::error;
use std::fmt::Formatter;
use std::io::Error;
use std::net::AddrParseError;

#[derive(Debug)]
pub enum UpstreamError {
    IO(Error),
    Parsing(AddrParseError),
}

impl std::fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for UpstreamError {}

// implementing from trait for the error
impl From<Error> for UpstreamError {
    fn from(value: Error) -> Self {
        UpstreamError::IO(value)
    }
}

impl From<AddrParseError> for UpstreamError {
    fn from(value: AddrParseError) -> Self {
        Parsing(value)
    }
}
