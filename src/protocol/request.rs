use std::io;

use byteorder::{NetworkEndian, WriteBytesExt};

use super::Extras;
use ::protocol::{Magic, Command, DataType};

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
    extras: Option<Vec<u8>>,
    key: Option<Vec<u8>>,
    value: Option<Vec<u8>>,
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
    pub fn set_key(&mut self, key: &[u8]) {
        self.key_length = key.len() as u16; // TODO: Possible cast failure
        self.key = Some(key.to_owned()); // TODO: must use `key` directly and remove additional allocation
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
    pub fn set_value(&mut self, value: &[u8]) {
        self.body_length += value.len() as u32; // TODO: Possible cast failure
        self.value = Some(value.to_owned()); // TODO: Must use `value` directly and remove additional allocation
    }

    pub fn set_extras<T: Extras>(&mut self, extras: T) {
        let mut buf: Vec<u8> = vec![];
        extras.write(&mut buf).expect("Failed to set extras");

        self.extras_length = buf.len() as u8; // TODO: Possible cast failure
        self.extras = Some(buf);
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

