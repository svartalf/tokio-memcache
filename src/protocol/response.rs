use std::io;
use std::convert;

use byteorder::{NetworkEndian, ReadBytesExt};

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
}

impl convert::TryFrom<Vec<u8>> for Response {
    // TODO: Replace with our own Error type
    type Err = io::Error;

    // TODO: Use `AsRef` here
    fn try_from(raw: Vec<u8>) -> Result<Self, Self::Err> {
        let mut response: Response = Default::default();
        response.magic = raw.read_u8();
        unimplemented!()
    }
}
