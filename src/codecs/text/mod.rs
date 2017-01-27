/// Memcached text protocol
///
/// Reference: https://github.com/memcached/memcached/blob/master/doc/protocol.txt

use std::io;
use std::str;
use tokio_core::io::{Codec, EasyBuf};

pub struct TextCodec {}


impl TextCodec {
    fn find_command(&self, data: &[u8]) -> Option<usize> {
        for (idx, (a, b)) in data.iter().zip(data.iter().skip(1)).enumerate() {
            if *a == 13 && *b == 10 {
                return Some(idx);
            }
        }
        None
    }
}


impl Codec for TextCodec {
    type In = String;
    type Out = String;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        if let Some(pos) = self.find_command(buf.as_slice()) {
            let line = buf.drain_to(pos);
            buf.drain_to(2);

            match str::from_utf8(line.as_slice()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::from(io::ErrorKind::Other)),
            }
        } else {
            Err(io::Error::from(io::ErrorKind::InvalidData))
        }
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        unimplemented!()
    }

}

#[cfg(test)]
mod tests;
