use std::io;
use std::fmt;
use std::str;
use std::error::{self, Error as StdError};
use std::convert::From;

use serde_json;

use protocol::Response;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Response(Response),
    Serde(serde_json::Error),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref cause) => cause.description(),
            Error::Response(ref response) => {
                match response.value() {
                    Some(bytes) => str::from_utf8(bytes).unwrap(),
                    None => "Unknown server response"
                }
            },
            Error::Serde(ref cause) => cause.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref cause) => Some(cause),
            Error::Serde(ref cause) => Some(cause),
            Error::Response(_) => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<Response> for Error {
    fn from(response: Response) -> Self {
        Error::Response(response)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Serde(error)
    }
}