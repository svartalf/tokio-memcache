use std::convert::From;
use std::marker::PhantomData;

use futures::Future;
use tokio_service::Service;
use tokio_proto::pipeline::ClientService;
use tokio_core::net::TcpStream;
use serde::Serialize;

use super::proto::MemcacheProto;
use super::Error;
use protocol::{Request, Response, Status};

// TODO: Sort these `mod`s in the same way as Command variants
mod get;
mod set;
mod quit;
mod delete;
mod version;

type Result<I> = Box<Future<Item=I, Error=Error>>;

// Wrapper around an inner service.
// Memcache methods should be implemented here
pub struct ClientHandle {
    // TODO: It would be nice to store Box<Service> instead
    inner: ClientService<TcpStream, MemcacheProto>,
}

impl ClientHandle {
    pub fn new(inner: ClientService<TcpStream, MemcacheProto>) -> ClientHandle {
        ClientHandle {
            inner: inner,
        }
    }

    /// Since other commands are not implemented here yet,
    /// this method can be used to send custom `Request`s.
    ///
    /// Sender is in charge of creating proper request.
    pub fn request(&self, request: Request) -> Result<Response> {
        self.call(request)
    }
}

impl Service for ClientHandle {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    // TODO: Get rid of Box when available
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        // TODO: Get rid of Box when available
        let res = self.inner.call(req)
            .map_err(From::from)
            .and_then(|response| {
                if *response.status() == Status::Ok {
                    Ok(response)
                } else {
                    Err(From::from(response))
                }
            });
        Box::new(res)
    }
}
