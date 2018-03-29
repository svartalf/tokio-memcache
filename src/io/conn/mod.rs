use std::collections::VecDeque;

use futures;
use futures::sync::oneshot;
use tokio::prelude::*;
use tokio::net::TcpStream;
use tokio_io::codec::Framed;
use tower::Service;
use bytes::BytesMut;

use io::errors::Error;
use io::codec::MemcacheCodec;
use protocol::{Request, Response};
pub use self::response::ResponseFuture;
//
//mod sink;
//mod stream;
mod interface;
mod response;

#[derive(Debug)]
pub struct Connection {
    socket: Framed<TcpStream, MemcacheCodec>,
    rd: BytesMut,
    wr: BytesMut,
    queue: VecDeque<oneshot::Sender<Response>>,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            socket: socket.framed(MemcacheCodec),
            rd: BytesMut::new(),
            wr: BytesMut::new(),
            queue: VecDeque::new(),
        }
    }
}

impl Service for Connection {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = ResponseFuture;

    fn poll_ready(&mut self) -> Poll<(), <Self as Service>::Error> {
        // TODO: Is it really ready all the time?
        // It might be useful to limit `self.queue` size and return `NotReady` to prevent excessive buffer fill.
        Ok(Async::Ready(()))
    }

    fn call(&mut self, req: <Self as Service>::Request) -> <Self as Service>::Future {
        let (tx, rx) = oneshot::channel();

        self.queue.push_back(tx);
        self.socket.send(req);

        ResponseFuture { rx }
    }
}
