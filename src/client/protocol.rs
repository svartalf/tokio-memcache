use std::io;

use tokio_core::io::{Io, Framed};
use tokio_proto::pipeline::ClientProto;

use protocol::{Request, Response};
use super::codec::BinaryCodec;

pub struct MemcachedProto;

impl<T: Io + 'static> ClientProto<T> for MemcachedProto {
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, BinaryCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(BinaryCodec))
    }
}
