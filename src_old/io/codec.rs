use std::io::{self, Read};

use bytes::{BytesMut, BufMut, ByteOrder};
use tokio_io::codec::{Encoder, Decoder};
use byteorder::{NetworkEndian, ReadBytesExt};
use enum_primitive::FromPrimitive;

use protocol::{Request, Response, Magic, DataType, Command, Status};

const HEADER_LENGTH: usize = 24;


#[derive(PartialEq, Debug, Clone)]
pub struct MemcacheCodec {}


impl Encoder for MemcacheCodec {
    type Item = Request;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
    }
}

impl Decoder for MemcacheCodec {
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

impl MemcacheCodec {
    pub fn new() -> MemcacheCodec {
        MemcacheCodec {
        }
    }

    /// Read `Response` from the properly-sized byte array
    fn read_response(src: &mut BytesMut) -> Result<Option<<Self as Decoder>::Item>, <Self as Decoder>::Error> {
    }

}
