use std::io;

use tokio_core::io::{Codec, EasyBuf};
use byteorder::{ByteOrder, NetworkEndian};

use protocol::{Request, Response};

pub struct BinaryCodec;

const HEADER_LENGTH: usize = 24;

impl Codec for BinaryCodec {
    type In = Response;
    type Out = Request;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        let length = buf.len();
        if length < HEADER_LENGTH {  // We had not received header yet
            return Ok(None);
        }

        let body_length = NetworkEndian::read_u32(&buf.as_slice()[8..12]) as usize;
        let packet_length = HEADER_LENGTH + body_length;
        if packet_length < length {  // Body is not received yet
            return Ok(None);
        }
        let packet = buf.drain_to(packet_length);

        match Self::In::try_from(packet.as_slice()) {
            Ok(response) => Ok(Some(response)),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid packet received"))
        }
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        msg.write(buf)
    }

}
