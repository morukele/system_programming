use crate::UpstreamError::Parsing;
use std::error;
use std::fmt::Formatter;
use std::io::Error;
use std::net::AddrParseError;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum UpstreamError {
    Network(smoltcp::Error),
    InvalidUrl,
    Content(Utf8Error),
    IO(Error),
    Parsing(AddrParseError),
}

impl std::fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<smoltcp::Error> for UpstreamError {
    fn from(value: smoltcp::Error) -> Self {
        UpstreamError::Network(value)
    }
}

impl From<Utf8Error> for UpstreamError {
    fn from(value: Utf8Error) -> Self {
        UpstreamError::Content(value)
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
