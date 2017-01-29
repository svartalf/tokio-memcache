use std::io;

use byteorder::{NetworkEndian, WriteBytesExt};

use ::protocol::Command;

// Request header structure for reference
//
//     Byte/     0       |       1       |       2       |       3       |
//        /              |               |               |               |
//       |0 1 2 3 4 5 6 7|0 1 2 3 4 5 6 7|0 1 2 3 4 5 6 7|0 1 2 3 4 5 6 7|
//       +---------------+---------------+---------------+---------------+
//      0| Magic         | Opcode        | Key length                    |
//       +---------------+---------------+---------------+---------------+
//      4| Extras length | Data type     | vbucket id                    |
//       +---------------+---------------+---------------+---------------+
//      8| Total body length                                             |
//       +---------------+---------------+---------------+---------------+
//     12| Opaque                                                        |
//       +---------------+---------------+---------------+---------------+
//     16| CAS                                                           |
//       |                                                               |
//       +---------------+---------------+---------------+---------------+
//       Total 24 bytes

pub struct Request {
    // header
    magic: u8,
    opcode: u8,
    key_length: u16,
    extras_length: u8,
    data_type: u8,
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

    pub fn new(command: Command) -> Request {
        Request {
            magic: 0x80,
            opcode: command as u8,
            key_length: 0,
            extras_length: 0,
            data_type: 0x00,
            vbucket_id: 0x00,
            body_length: 0,
            opaque: 0,
            cas: 0,
            extras: None,
            key: None,
            value: None,
        }
    }

    pub fn set_key(&mut self, key: &[u8]) {
        self.key_length = key.len() as u16; // TODO: Possible cast failure
        self.key = Some(key.to_owned()); // TODO: must use `key` directly and remove additional allocation
        self.body_length += self.key_length as u32;
    }

    pub fn set_value(&mut self, value: &[u8]) {
        self.body_length += value.len() as u32; // TODO: Possible cast failure
        self.value = Some(value.to_owned()); // TODO: Must use `value` directly and remove additional allocation
    }

    pub fn set_extras(&mut self, extras: &[u8]) {
        self.extras_length = extras.len() as u8; // TODO: Possible cast failure
        self.extras = Some(extras.to_owned()); // Must use `extras` directly and remove additional allocation
        self.body_length += self.extras_length as u32;
    }

    // TODO: Not sure if proper command name
    pub fn write<T: io::Write>(&self, out: &mut T) -> io::Result<()> {
        out.write_u8(self.magic)?;
        out.write_u8(self.opcode)?;
        out.write_u16::<NetworkEndian>(self.key_length)?;
        out.write_u8(self.extras_length)?;
        out.write_u8(self.data_type)?;
        out.write_u16::<NetworkEndian>(self.vbucket_id)?;
        out.write_u32::<NetworkEndian>(self.body_length)?;
        out.write_u32::<NetworkEndian>(self.opaque)?;
        out.write_u64::<NetworkEndian>(self.cas)?;

        // TODO: Check if there is no additional allocation made
        if let Some(ref extras) = self.extras {
            out.write(&extras)?;
        }

        // TODO: Check if there is no additional allocation made
        if let Some(ref key) = self.key {
            out.write(&key)?;
        }

        // TODO: Check if there is no additional allocation made
        if let Some(ref value) = self.value {
            out.write(&value)?;
        }

        Ok(())
    }
}

