use std::convert::From;

use futures::Future;
use tokio_service::Service;
use tokio_proto::pipeline::ClientService;
use tokio_core::net::TcpStream;

use super::proto::MemcacheProto;
use super::Error;
use protocol::{Request, Response, Command, Status, extras};
use protocol::extras::Extras;

type Result<I> = Box<Future<Item=I, Error=Error>>;

// Wrapper around an inner service.
// Memcache methods should be implemented here
pub struct ClientHandle {
    // TODO: Get rid of `pub`
    // TODO: It would be nice to store Box<Service> instead
    pub inner: ClientService<TcpStream, MemcacheProto>
}

impl ClientHandle {
    // TODO: Replace `Result` (which is a `Box<Future>`) with a `impl Future`
    pub fn get(&self, key: &[u8]) -> Result<Response> {
        let request = Request::build(Command::Get)
            .key(Some(key.to_vec()))
            .finish();
        self.call(request)
    }

    pub fn set(&self, key: &[u8], value: &[u8], extras: &extras::Set) -> Result<Response> {
        let request = Request::build(Command::Set)
            .key(Some(key.to_vec()))
            .value(Some(value.to_vec()))
            .extras(Some(extras.to_vec()))
            .finish();

        self.call(request)
    }

    pub fn add(&self, key: &[u8], value: &[u8], extras: &extras::Add) -> Result<Response> {
        let request = Request::build(Command::Add)
            .key(Some(key.to_vec()))
            .value(Some(value.to_vec()))
            .extras(Some(extras.to_vec()))
            .finish();

        self.call(request)
    }

    pub fn replace(&self, key: &[u8], value: &[u8], extras: &extras::Replace) -> Result<Response> {
        let request = Request::build(Command::Replace)
            .key(Some(key.to_vec()))
            .value(Some(value.to_vec()))
            .extras(Some(extras.to_vec()))
            .finish();

        self.call(request)
    }

    // TODO: Check if `Response` even needed in returned result
    pub fn delete(&self, key: &[u8]) -> Result<Response> {
        let request = Request::build(Command::Delete)
            .key(Some(key.to_vec()))
            .finish();

        self.call(request)
    }

    pub fn increment(&self, key: &[u8], extras: &extras::Increment) -> Result<Response> {
        let request = Request::build(Command::Increment)
            .key(Some(key.to_vec()))
            .extras(Some(extras.to_vec()))
            .finish();

        self.call(request)
    }

    pub fn decrement(&self, key: &[u8], extras: &extras::Decrement) -> Result<Response> {
        let request = Request::build(Command::Decrement)
            .key(Some(key.to_vec()))
            .extras(Some(extras.to_vec()))
            .finish();

        self.call(request)
    }

    // TODO: Should not `quit()` be called on the handle drop?
    pub fn quit(&self) -> Result<Response> {
        let request = Request::build(Command::Quit)
            .finish();

        self.call(request)
    }

    pub fn version(&self) -> Result<Response> {
        let request = Request::build(Command::Version)
            .finish();

        self.call(request)
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
