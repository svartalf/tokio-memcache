use std::io;
use std::io::Read;
use std::boxed::Box;
use std::fmt;

use enum_primitive::FromPrimitive;
use byteorder::{NetworkEndian, ReadBytesExt};

use ::protocol::{Magic, Command, DataType};

enum_from_primitive! {
    /// Response status variants
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

/// Parsed `memcached` response.
pub struct Response {
    // We are not storing `magic` byte, because it is always the same and is not required by clients
    opcode: Command,
    key_length: u16,
    extras_length: u8,
    data_type: DataType,
    status: Status,
    body_length: u32,
    opaque: u32,
    cas: u64,
    body: Option<Box<[u8]>>,
}

impl Response {
    pub fn command(&self) -> &Command {
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
        match self.body {
            Some(ref body) if self.extras_length > 0 => {
                let end = self.extras_length as usize;
                Some(&body[..end])
            }
            _ => None,
        }
    }

    pub fn key(&self) -> Option<&[u8]> {
        match self.body {
            Some(ref body) if self.key_length > 0 => {
                let start = self.extras_length as usize;
                let end = start + self.key_length as usize;

                Some(&body[start..end])
            },
            _ => None,
        }
    }

    pub fn value(&self) -> Option<&[u8]> {
        match self.body {
            Some(ref body) if self.body_length > 0 => {
                let start: usize = self.extras_length as usize + self.key_length as usize;

                Some(&body[start..])
            },
            _ => None,
        }
    }

    pub fn try_from(raw: &[u8]) -> Result<Response, io::Error> {
        let mut cursor = io::Cursor::new(raw);
        let magic = cursor.read_u8()?;
        if magic != Magic::Response as u8 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid packet received"));
        }

        let mut response = Response {
            opcode: Command::from_u8(cursor.read_u8()?)
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Unknown command"))?,
            key_length: cursor.read_u16::<NetworkEndian>()?,
            extras_length: cursor.read_u8()?,
            data_type: DataType::from_u8(cursor.read_u8()?)
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Unknown data type"))?,
            status: Status::from_u16(cursor.read_u16::<NetworkEndian>()?)
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Unknown status"))?,
            body_length: cursor.read_u32::<NetworkEndian>()?,
            opaque: cursor.read_u32::<NetworkEndian>()?,
            cas: cursor.read_u64::<NetworkEndian>()?,
            body: None,
        };

        if response.body_length > 0 {
            let mut body: Vec<u8> = Vec::with_capacity(response.body_length as usize);
            cursor.read_to_end(&mut body)?;
            response.body = Some(body.into_boxed_slice());
        }

        Ok(response)
    }

    pub fn is_ok(&self) -> bool {
        self.status == Status::Ok
    }

    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }
}

impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut resp = f.debug_struct("Response");

        resp
            .field("command", &self.opcode)
            .field("status", &self.status);

        if let Some(key) = self.key() {
            resp.field("key", &key);
        }

        if let Some(value) = self.value() {
            resp.field("value", &value);
        }

        if let Some(extras) = self.extras() {
            resp.field("extras", &extras);
        }

        resp.finish()
    }
}
