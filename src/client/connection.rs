use std::io;
use std::convert::AsRef;

use tokio_core::net::TcpStream;
use tokio_service::Service;
use tokio_proto::pipeline::ClientService;
use futures::{Future, BoxFuture};

use super::protocol::{MemcachedProto};
use protocol::{Request, Response, Command, extras};

pub struct Connection {
    pub inner: ClientService<TcpStream, MemcachedProto>,
}

impl Connection {
    /// Send [`Request`][request] to server.
    ///
    /// This method can be used if you need to send a rich request
    /// and there is no available wrapper method.
    ///
    /// [request]: ./struct.Request.html
    pub fn send(&self, req: Request) -> BoxFuture<Response, io::Error> {
        self.call(req)
    }

    pub fn get<T: AsRef<[u8]>>(&self, key: T) -> BoxFuture<Response, io::Error> {
        let mut request = Request::new(Command::Get);
        request.set_key(key);

        self.send(request)
    }

    pub fn set<T: AsRef<[u8]>>(&self, key: T, value: T, extras: extras::SetExtras) -> BoxFuture<Response, io::Error> {
        let mut request = Request::new(Command::Set);
        request.set_key(key);
        request.set_value(value);
        request.set_extras(extras);

        self.send(request)
    }
}

impl Service for Connection {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        self.inner.call(req).boxed()
    }
}

