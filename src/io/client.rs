use std::io;
use std::net::{SocketAddr, ToSocketAddrs};

use futures::Future;
use futures::sync::oneshot;
use tokio::net::TcpStream;
use tower::NewService;

use io::errors::Error;
use io::conn::{Connection, ResponseFuture};
use protocol::{Request, Response};

#[derive(Debug, Copy, Clone)]
pub struct Client {
    addr: SocketAddr,
}

impl Client {
    pub fn new<T: ToSocketAddrs>(addr: T) -> Client {
        Client {
            // TODO: Temporary
            addr: addr.to_socket_addrs().unwrap().next().unwrap(),
        }
    }

    #[inline]
    pub fn connect(&self) -> <Self as NewService>::Future {
        self.new_service()
    }
}

impl NewService for Client {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Service = Connection;
    type InitError = io::Error;
    type Future = Box<Future<Item=Self::Service, Error=Self::InitError>>;

    fn new_service(&self) -> <Self as NewService>::Future {
        let future = TcpStream::connect(&self.addr).and_then(|stream| {
            Ok(Connection::new(stream))
        });

        Box::new(future)
    }
}
