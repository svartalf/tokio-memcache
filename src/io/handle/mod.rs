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
mod quit;
mod delete;
mod version;

type Result<I> = Box<Future<Item=I, Error=Error>>;

// Wrapper around an inner service.
// Memcache methods should be implemented here
pub struct ClientHandle<K: Serialize + 'static> {
    // TODO: Get rid of `pub`
    // TODO: It would be nice to store Box<Service> instead
    inner: ClientService<TcpStream, MemcacheProto<K>>,
    _key: PhantomData<K>,
}

impl<K> ClientHandle<K> where K: Serialize + 'static {
    pub fn new(inner: ClientService<TcpStream, MemcacheProto<K>>) -> ClientHandle<K> {
        ClientHandle {
            inner: inner,
            _key: PhantomData,
        }
    }
}

impl<K> ClientHandle<K> where K: Serialize {

//
//    pub fn set(&self, key: K, value: &[u8], extras: &extras::Set) -> Result<Response> {
//        let request = Request::build(Command::Set)
//            .key(Some(key))
//            .value(Some(value.to_vec()))
//            .extras(Some(extras.to_vec()))
//            .finish();
//
//        self.call(request)
//    }
//
//    pub fn add(&self, key: K, value: &[u8], extras: &extras::Add) -> Result<Response> {
//        let request = Request::build(Command::Add)
//            .key(Some(key))
//            .value(Some(value.to_vec()))
//            .extras(Some(extras.to_vec()))
//            .finish();
//
//        self.call(request)
//    }
//
//    pub fn replace(&self, key: K, value: &[u8], extras: &extras::Replace) -> Result<Response> {
//        let request = Request::build(Command::Replace)
//            .key(Some(key))
//            .value(Some(value.to_vec()))
//            .extras(Some(extras.to_vec()))
//            .finish();
//
//        self.call(request)
//    }
//

//
//    pub fn increment(&self, key: K, extras: &extras::Increment) -> Result<Response> {
//        let request = Request::build(Command::Increment)
//            .key(Some(key))
//            .extras(Some(extras.to_vec()))
//            .finish();
//
//        self.call(request)
//    }
//
//    pub fn decrement(&self, key: K, extras: &extras::Decrement) -> Result<Response> {
//        let request = Request::build(Command::Decrement)
//            .key(Some(key))
//            .extras(Some(extras.to_vec()))
//            .finish();
//
//        self.call(request)
//    }

    /// Since other commands are not implemented here yet,
    /// this method can be used to send custom `Request`s.
    ///
    /// Sender is in charge of creating proper request.
    pub fn request(&self, request: Request<K>) -> Result<Response> {
        self.call(request)
    }
}

impl<K> Service for ClientHandle<K> where K: Serialize {
    type Request = Request<K>;
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
