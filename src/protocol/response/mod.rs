use std::fmt;

use protocol::{Magic, Command, DataType};

enum_from_primitive! {
    /// Response status variants
    #[repr(u16)]
    #[derive(Debug, Copy, Clone, PartialEq)]
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

pub struct Response {
    // TODO: Get rid of the `pub(crate)`
    pub(crate) magic: Magic,
    pub(crate) opcode: Command,
    pub(crate) key_length: u16,
    pub(crate) extras_length: u8,
    pub(crate) data_type: DataType,
    pub(crate) status: Status,
    pub(crate) body_length: u32,
    pub(crate) opaque: u32,
    pub(crate) cas: u64,
    pub(crate) body: Vec<u8>,
}

impl Response {
    pub fn magic(&self) -> &Magic {
        &self.magic
    }

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
