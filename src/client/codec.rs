use std::io;

use bytes::BytesMut;
use tokio_io::codec::{Decoder, Encoder};
use byteorder::{ByteOrder, NetworkEndian};

use protocol::{Request, Response};

pub struct BinaryCodec;

const HEADER_LENGTH: usize = 24;

impl Decoder for BinaryCodec {
    type Item = Response;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let length = src.len();
        if length < HEADER_LENGTH {  // We had not received header yet
            return Ok(None);
        }

        let body_length = NetworkEndian::read_u32(&src[8..12]) as usize;
        let packet_length = HEADER_LENGTH + body_length;
        if packet_length < length {  // Body is not received yet
            return Ok(None);
        }
        let packet = src.split_to(packet_length);

        match Self::Item::try_from(&packet) {
            Ok(response) => Ok(Some(response)),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid packet received"))
        }
    }
}

impl Encoder for BinaryCodec {
    type Item = Request;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        item.write(dst)
    }
}
