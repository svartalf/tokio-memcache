use std::io;
use std::marker::PhantomData;

use tokio_proto::pipeline::ClientProto;
use tokio_io::codec::Framed;
use tokio_io::{AsyncRead, AsyncWrite};
use serde::Serialize;

use super::codec::MemcacheCodec;
use protocol::{Request, Response};

#[derive(PartialEq, Debug, Clone)]
pub struct MemcacheProto<K> {
    _key: PhantomData<K>,
}

impl<K> MemcacheProto<K> {
    pub fn new() -> MemcacheProto<K> {
        MemcacheProto {
            _key: PhantomData,
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

impl<K: 'static, T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for MemcacheProto<K>
        where K: Serialize {
    type Request = Request<K>;
    type Response = Response;
    type Transport = Framed<T, MemcacheCodec<K>>;
    type BindTransport = io::Result<Self::Transport>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let codec = MemcacheCodec::new();
        Ok(io.framed(codec))
    }
}
