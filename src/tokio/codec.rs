use std::io::{self, Read};
use std::marker::PhantomData;

use bytes::{BytesMut, BufMut, ByteOrder};
use tokio_io::codec::{Encoder, Decoder};
use byteorder::{NetworkEndian, ReadBytesExt};
use enum_primitive::FromPrimitive;
use serde::Serialize;
use serde_json;

use protocol::{Request, Response, Magic, DataType, Command, Status};

const HEADER_LENGTH: usize = 24;


#[derive(PartialEq, Debug, Clone)]
pub struct MemcacheCodec<K> {
    _key: PhantomData<K>,
}

impl<K> Encoder for MemcacheCodec<K> where K: Serialize {
    type Item = Request<K>;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // TODO: Handle lossless conversion
        // TODO: Seems not very efficient
        let (key, key_length) = match *item.key() {
            None => (None, 0),
            Some(ref key) => {
                let key_bytes = serde_json::to_vec(key)?;
                let length = key_bytes.len();
                (Some(key_bytes), length)
            }
        };

        let extras_length: u8 = match *item.extras() {
            Some(ref extras) => extras.len() as u8,
            None => 0,
        };
        let value_length = match *item.value() {
            Some(ref value) => value.len() as u32,
            None => 0,
        };
        let body_length: u32 = key_length as u32 + extras_length as u32 + value_length;

        dst.reserve(body_length as usize);

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

        if let Some(ref key) = key {
            dst.put_slice(&key);
        }

        if let Some(ref value) = *item.value() {
            dst.put_slice(value);
        }

        Ok(())
    }
}

impl<K> Decoder for MemcacheCodec<K> {
    type Item = Response;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let length = src.len();
        if length < HEADER_LENGTH {
            // We had not received header yet
            return Ok(None);
        }

        let body_length = NetworkEndian::read_u32(&src[8..12]) as usize;
        let packet_length = HEADER_LENGTH + body_length;
        if packet_length < length {
            // Body is not received yet
            return Ok(None);
        }
        let mut packet = src.split_to(packet_length);

        // TODO: Convert error response to an error struct
        Self::read_response(&mut packet)
    }

}

impl<K> MemcacheCodec<K> {
    pub fn new() -> MemcacheCodec<K> {
        MemcacheCodec {
            _key: PhantomData,
        }
    }

    /// Read `Response` from the properly-sized byte array
    fn read_response(src: &mut BytesMut) -> Result<Option<<Self as Decoder>::Item>, <Self as Decoder>::Error> {
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
