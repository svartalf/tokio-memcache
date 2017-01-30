use std::io;
use std::boxed::Box;
use std::net::SocketAddr;

use tokio_proto::TcpClient;
use tokio_core::net::TcpStream;
use tokio_service::Service;
use tokio_proto::pipeline::ClientService;
use tokio_core::reactor::Handle;
use futures::{Future, BoxFuture};

use super::protocol::{MemcachedProto};
use protocol::{Request, Response, Command};


pub struct Client {
    inner: ClientService<TcpStream, MemcachedProto>,
}

impl Client {

    pub fn connect(addr: &SocketAddr, handle: &Handle) -> Box<Future<Item=Client, Error=io::Error>> {
        let result = TcpClient::new(MemcachedProto)
            .connect(addr, handle)
            .map(|client_service| {
                Client {
                    inner: client_service,
                }
            });

        Box::new(result)
    }

    pub fn get(&self, key: &[u8]) -> BoxFuture<Response, io::Error> {
        let mut request = Request::new(Command::Get);
        request.set_key(key);

        self.call(request)
    }

    pub fn set(&self, key: &[u8], value: &[u8]) -> BoxFuture<Response, io::Error> {
        let mut request = Request::new(Command::Set);
        request.set_key(key);
        request.set_value(value);
        request.set_extras(&vec![0xde, 0xad, 0xbe, 0xef, 0x00, 0x00, 0x0e, 0x10]);

        self.call(request)
    }

}

impl Service for Client {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        self.inner.call(req).boxed()
    }
}
