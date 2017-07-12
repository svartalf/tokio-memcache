use std::io;

use tokio_proto::pipeline::ClientProto;
use tokio_io::codec::{Framed};
use tokio_io::{AsyncRead, AsyncWrite};

use protocol::{Request, Response};
use super::codec::BinaryCodec;

pub struct MemcachedProto;

impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for MemcachedProto {
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, BinaryCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(BinaryCodec))
    }
}
