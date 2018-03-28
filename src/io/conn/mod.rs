use tokio::prelude::*;
use tokio::net::TcpStream;
use tokio_io::codec::Framed;
use tower::Service;
use bytes::BytesMut;

use io::errors::Error;
use io::codec::MemcacheCodec;
use protocol::{Request, Response};
//
//mod sink;
//mod stream;
mod interface;

#[derive(Debug)]
pub struct Connection {
    socket: Framed<TcpStream, MemcacheCodec>,
    rd: BytesMut,
    wr: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            socket: socket.framed(MemcacheCodec),
            rd: BytesMut::new(),
            wr: BytesMut::new(),
        }
    }
}

impl Service for Connection {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), <Self as Service>::Error> {
        // TODO: Is it really ready all the time?
        Ok(Async::Ready(()))
    }

    fn call(&mut self, req: <Self as Service>::Request) -> <Self as Service>::Future {
        unimplemented!()
    }
}
