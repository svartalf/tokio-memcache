use std::io;
use std::convert::From;

use tokio::prelude::*;
use tokio::net::TcpStream;
use tower::Service;
use bytes::BytesMut;

use io::errors::Error;
use protocol::{Request, Response};

mod sink;
mod stream;
mod interface;

#[derive(Debug)]
pub struct Connection {
    socket: TcpStream,
    rd: BytesMut,
    wr: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            socket: socket,
            rd: BytesMut::new(),
            wr: BytesMut::new(),
        }
    }

    fn poll_flush(&mut self) -> Poll<(), io::Error> {
        while !self.wr.is_empty() {
            let n = try_ready!(self.socket.poll_write(&self.wr));

            assert!(n > 0);

            let _ = self.wr.split_to(n);
        }

        Ok(Async::Ready(()))
    }

    fn fill_read_buf(&mut self) -> Poll<(), io::Error> {
        loop {
            // TODO: Check required `reserve` size, might be too big
            self.rd.reserve(1024);
            let n = try_ready!(self.socket.read_buf(&mut self.rd));

            if n == 0 {
                return Ok(Async::Ready(()));
            }
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
        let f = self.send(req).map_err(From::from);

        Box::new(f)
    }
}
