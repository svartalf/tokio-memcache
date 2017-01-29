/// Memcached binary protocol
///
/// Reference: https://github.com/memcached/memcached/wiki/BinaryProtocolRevamped

use std::io;
use std::str;
use tokio_core::io::{Codec, EasyBuf};

use protocol::{Request, Response};

pub struct BinaryCodec;


impl BinaryCodec {
}


impl Codec for BinaryCodec {
    type In = String;
    type Out = String;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        unimplemented!()
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        unimplemented!()
    }

}
