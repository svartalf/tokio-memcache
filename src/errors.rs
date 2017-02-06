use std::io;
use std::fmt;
use std::str;
use std::error::Error;
use std::convert;

use protocol::Response;


#[derive(Debug)]
pub enum ErrorKind {
    Io(io::Error),
    Response(Response),
}

#[derive(Debug)]
pub struct MemcacheError {
    kind: ErrorKind,
}

impl Error for MemcacheError {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::Io(ref err) => err.description(),
            ErrorKind::Response(ref resp) => {
                str::from_utf8(resp.value().unwrap_or(b"Unknown")).expect("Failed to parse memcached response")
            },
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self.kind {
            ErrorKind::Io(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for MemcacheError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl convert::From<io::Error> for MemcacheError {
    fn from(err: io::Error) -> Self {
        MemcacheError {
            kind: ErrorKind::Io(err),
        }
    }
}

impl convert::From<Response> for MemcacheError {
    fn from(resp: Response) -> Self {
        MemcacheError {
            kind: ErrorKind::Response(resp)
        }
    }
}