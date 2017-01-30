use std::io;
use std::fmt;

use byteorder::{NetworkEndian, ReadBytesExt};
use tokio_core::io::EasyBuf;

use ::protocol::Command;

/*
     Byte/     0       |       1       |       2       |       3       |
        /              |               |               |               |
       |0 1 2 3 4 5 6 7|0 1 2 3 4 5 6 7|0 1 2 3 4 5 6 7|0 1 2 3 4 5 6 7|
       +---------------+---------------+---------------+---------------+
      0| Magic         | Opcode        | Key Length                    |
       +---------------+---------------+---------------+---------------+
      4| Extras length | Data type     | Status                        |
       +---------------+---------------+---------------+---------------+
      8| Total body length                                             |
       +---------------+---------------+---------------+---------------+
     12| Opaque                                                        |
       +---------------+---------------+---------------+---------------+
     16| CAS                                                           |
       |                                                               |
       +---------------+---------------+---------------+---------------+
       Total 24 bytes

       */

#[derive(Default)]
pub struct Response {
    magic: u8,
    opcode: u8,
    key_length: u16,
    extras_length: u8,
    data_type: u8,
    status: u16,
    body_length: u32,
    opaque: u32,
    cas: u64,
    body: Vec<u8>,
}

impl Response {
    pub fn command(&self) -> &u8 {
        &self.opcode
    }

    pub fn status(&self) -> &u16 {
        &self.status
    }

    /// Trying to create a `Response` from the bytes array.
    ///
    /// If `raw` is incomplete, returns `Ok(None)`, otherwise returns `Ok(Response)`,
    /// it will be compatible with a Codec struct
    pub fn try_from(raw: &mut EasyBuf) -> io::Result<Option<Response>> {
        let length = raw.len();

        // Quick checking if we have at least a response header
        if length < 24 {
            return Ok(None);
        }

        let mut response = Response::default();
        response.magic = 0x81;

        Ok(Some(response))
    }
}

impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Response")
            .field("command", &self.opcode)
            .field("status", &self.status)
            .finish()
    }
}

struct Parser<'a> {
    buf: &'a [u8],
    pos: 0,
}

impl<'a> Parser<'a> {
    pub fn new(buf: &'a [u8]) -> Parser {
        Parser {
            buf: buf,
            pos: 0,
        }
    }

    pub fn parse(&self) -> io::Result<Option<Response>> {
        let magic = self.expect_u8(0x81)?;
    }

    fn expect_u8(&self) -> io::Result<u8> {
        
    }
}