use std::io;
use std::fmt;
use std::boxed::Box;

use byteorder::{NetworkEndian, WriteBytesExt};

use super::Extras;
use ::protocol::{Magic, Command, DataType};
use types::AsArgument;

/// Memcached request instance. In case if you need to construct request manually.
pub struct Request {
    opcode: Command,
    key_length: u16,
    extras_length: u8,
    data_type: DataType,
    vbucket_id: u16,
    body_length: u32,
    opaque: u32,
    cas: u64,

    // body
    extras: Option<Box<[u8]>>,
    key: Option<Box<[u8]>>,
    value: Option<Box<[u8]>>,
}

impl Request {

    /// Create new Request with all fields blank.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut request = Request::new(Command::Get);
    /// ```
    pub fn new(command: Command) -> Request {
        Request {
            opcode: command,
            key_length: 0,
            extras_length: 0,
            data_type: DataType::RawBytes,
            vbucket_id: 0x00,
            body_length: 0,
            opaque: 0,
            cas: 0,
            extras: None,
            key: None,
            value: None,
        }
    }

    /// Provide key field.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut request = Request::new(Command::Get);
    /// request.set_key(b"Hello");
    /// ```
    pub fn set_key<T>(&mut self, key: &T) where T: AsArgument {
        let value = key.as_boxed_slice();

        self.key_length = value.len() as u16; // TODO: Possible value truncation
        self.key = Some(value);
        self.body_length += self.key_length as u32;
    }

    /// Provide value field.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut request = Request::new(Command::Set);
    /// request.set_key(b"Hello");
    /// request.set_value(b"World");
    /// ```
    pub fn set_value<T>(&mut self, value: &T) where T: AsArgument {
        let value = value.as_boxed_slice();

        self.body_length += value.len() as u32; // TODO: Possible value truncation
        self.value = Some(value);
    }

    pub fn set_extras<T: Extras>(&mut self, extras: T) {
        let mut buf: Vec<u8> = vec![];
        extras.write(&mut buf).expect("Failed to set extras");

        self.extras_length = buf.len() as u8; // TODO: Possible value truncation
        self.extras = Some(buf.into_boxed_slice());
        self.body_length += self.extras_length as u32;
    }

    /// Write serialized request as a bytes into `T`
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`][Error] if write had failed somehow.
    ///
    /// [Error]: ../../std/io/struct.Error.html
    pub fn write<T: io::Write>(&self, out: &mut T) -> io::Result<()> {
        out.write_u8(Magic::Request as u8)?;
        out.write_u8(self.opcode as u8)?;
        out.write_u16::<NetworkEndian>(self.key_length)?;
        out.write_u8(self.extras_length)?;
        out.write_u8(self.data_type as u8)?;
        out.write_u16::<NetworkEndian>(self.vbucket_id)?;
        out.write_u32::<NetworkEndian>(self.body_length)?;
        out.write_u32::<NetworkEndian>(self.opaque)?;
        out.write_u64::<NetworkEndian>(self.cas)?;

        if let Some(ref extras) = self.extras {
            out.write_all(extras)?;
        }

        if let Some(ref key) = self.key {
            out.write_all(key)?;
        }

        if let Some(ref value) = self.value {
            out.write_all(value)?;
        }

        Ok(())
    }
}


impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut resp = f.debug_struct("Request");

        resp
            .field("command", &self.opcode);

        resp.finish()
    }
}
