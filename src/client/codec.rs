use std::io;
use std::str;
use tokio_core::io::{Codec, EasyBuf};

use protocol::{Request, Response};

pub struct BinaryCodec;


impl BinaryCodec {
}


impl Codec for BinaryCodec {
    type In = Response;
    type Out = Request;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        let length = buf.len();

        Self::In::try_from(buf)
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        msg.write(buf)
    }

}
