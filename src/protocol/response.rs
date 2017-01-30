use std::io;
use std::fmt;

use byteorder::{NetworkEndian, ReadBytesExt};
use tokio_core::io::EasyBuf;

// use ::protocol::Command;

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
    // We are not storing `magic` byte, because it is always the same and is not required by clients
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

    pub fn data_type(&self) -> &u8 {
        &self.data_type
    }

    pub fn opaque(&self) -> &u32 {
        &self.opaque
    }

    pub fn cas(&self) -> &u64 {
        &self.cas
    }

    pub fn extras(&self) -> Option<&[u8]> {
        if self.extras_length > 0 {
            let end = self.extras_length as usize;
            return Some(&self.body[..end]);
        }
        None
    }

    pub fn key(&self) -> Option<&[u8]> {
        if self.key_length > 0 {
            let start = self.extras_length as usize;
            let end = start + self.key_length as usize;

            return Some(&self.body[start..end]);
        }

        None
    }

    pub fn value(&self) -> Option<&[u8]> {
        let start: usize = self.extras_length as usize + self.key_length as usize;
        if self.body_length as usize > start {
            return Some(&self.body[start..]);
        }

        None
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

        let header_buf = raw.drain_to(24);
        let mut header = header_buf.as_ref();
        let magic = header.read_u8()?;
        if magic != 0x81 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid packet received"));
        }

        let mut response = Response {
            opcode: header.read_u8()?,
            key_length: header.read_u16::<NetworkEndian>()?,
            extras_length: header.read_u8()?,
            data_type: header.read_u8()?,
            status: header.read_u16::<NetworkEndian>()?,
            body_length: header.read_u32::<NetworkEndian>()?,
            opaque: header.read_u32::<NetworkEndian>()?,
            cas: header.read_u64::<NetworkEndian>()?,
            body: vec![],
        };

        if response.body_length > 0 {
            let body = raw.drain_to(response.body_length as usize);
            response.body.extend_from_slice(body.as_slice());
        }

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
