use std::io;

use tokio_proto::pipeline::ClientProto;
use tokio_io::codec::Framed;
use tokio_io::{AsyncRead, AsyncWrite};

use super::codec::MemcacheCodec;
use protocol::{Request, Response};

#[derive(PartialEq, Debug, Clone)]
pub struct MemcacheProto {
}

impl MemcacheProto {
    pub fn new() -> MemcacheProto {
        MemcacheProto {
        }
    }
}


// TODO: Got a *Q requests we should ignore some empty responses.
//
// NOTE: @svartalf I did that with a multiplexed, not pipelined, protocol,
// but the principle should be similar.
// Put a VecDeque FIFO in the protocol struct (with a pipelined protocol, an Option may be enough)
// and push an element to it when sending a "solo" request.
// Wrap the I/O source and make a Stream impl with a poll() which will first check
// for outstanding elements in the FIFO and, for each element,
// return a fake response to the initiating future

impl<T> ClientProto<T> for MemcacheProto
        where T: AsyncRead + AsyncWrite + 'static {
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, MemcacheCodec>;
    type BindTransport = io::Result<Self::Transport>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let codec = MemcacheCodec::new();
        Ok(io.framed(codec))
    }
}
