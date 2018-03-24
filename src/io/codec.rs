use std::io::{self, Read};

use bytes::{BytesMut, BufMut};
use byteorder::{NetworkEndian, ReadBytesExt};
use tokio_io::codec::{Encoder, Decoder};
use enum_primitive::FromPrimitive;

use protocol::{Request, Response, Magic, Command, DataType, Status};

const HEADER_LENGTH: usize = 24;


#[derive(Debug, Copy, Clone)]
pub struct MemcacheCodec;

impl Encoder for MemcacheCodec {
    type Item = Request;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // TODO: Handle lossless conversion
        // TODO: Seems not very efficient
        let (key, key_length) = match *item.key() {
            None => (None, 0),
            Some(ref key) => {
                (Some(key), key.len() as u32)
            }
        };

        let extras_length: u8 = match *item.extras() {
            Some(ref extras) => extras.len() as u8,
            None => 0,
        };

        let (value, value_length) = match *item.value() {
            None => (None, 0),
            Some(ref value) => {
                (Some(value), value.len() as u32)
            }
        };
        let body_length: u32 = key_length as u32 + extras_length as u32 + value_length;

        dst.reserve(HEADER_LENGTH + body_length as usize);

        dst.put_u8(Magic::Request as u8);
        dst.put_u8(*item.command() as u8);
        dst.put_u16::<NetworkEndian>(key_length as u16);
        dst.put_u8(extras_length);
        dst.put_u8(*item.data_type() as u8);
        dst.put_u16::<NetworkEndian>(*item.vbucket_id());
        dst.put_u32::<NetworkEndian>(body_length);
        dst.put_u32::<NetworkEndian>(*item.opaque());
        dst.put_u64::<NetworkEndian>(*item.cas());

        if let Some(ref extras) = *item.extras() {
            dst.put_slice(extras);
        }

        if let Some(key) = key {
            dst.put_slice(key);
        }

        if let Some(value) = value {
            dst.put_slice(value);
        }

        Ok(())
    }
}

impl Decoder for MemcacheCodec {
    type Item = Response;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut cursor = io::Cursor::new(src);
        let magic = cursor.read_u8()?;
        if magic != Magic::Response as u8 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid packet received"));
        }

        let mut response = Response {
            magic: Magic::Response,
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
            // TODO: We are already know body length, should use `Vec::with_capacity`
            body: vec![],
        };

        if response.body_length > 0 {
            cursor.read_to_end(&mut response.body)?;
        }

        Ok(Some(response))
    }
}
