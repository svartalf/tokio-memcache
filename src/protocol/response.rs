use std::io;
use std::fmt;

use enum_primitive::FromPrimitive;
use byteorder::{NetworkEndian, ReadBytesExt};
use tokio_core::io::EasyBuf;

enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum Status {
        Ok = 0x0000,
        KeyNotFound = 0x0001,
        KeyExists = 0x0002,
        ValueTooLarge = 0x0003,
        InvalidArguments = 0x0004,
        ItemNotStored = 0x0005,
        IncrDecrOnNonNumericValue = 0x0006,
        VBucketBelongsToAnotherServer = 0x0007,
        AuthenticationError = 0x0008,
        AuthenticationContinue = 0x0009,
        UnknownCommand = 0x0081,
        OutOfMemory = 0x0082,
        NotSupported = 0x00083,
        InternalError = 0x0084,
        Busy = 0x0085,
        TemporaryFailure = 0x0086,
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum DataType {
        RawBytes = 0x00,
    }
}

pub struct Response {
    // We are not storing `magic` byte, because it is always the same and is not required by clients
    opcode: u8,
    key_length: u16,
    extras_length: u8,
    data_type: DataType,
    status: Status,
    body_length: u32,
    opaque: u32,
    cas: u64,
    body: Vec<u8>,
}

impl Response {
    pub fn command(&self) -> &u8 {
        &self.opcode
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn data_type(&self) -> &DataType {
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
            // TODO: Get rid of `unwrap` here
            data_type: DataType::from_u8(header.read_u8()?).unwrap(),
            // TODO: Get rid of `unwrap` here
            status: Status::from_u16(header.read_u16::<NetworkEndian>()?).unwrap(),
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
