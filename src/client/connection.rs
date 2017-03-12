use tokio_core::net::TcpStream;
use tokio_service::Service;
use tokio_proto::pipeline::ClientService;
use futures::{future, Future, BoxFuture};

use super::protocol::{MemcachedProto};
use protocol::{Request, Response, Command, extras};
use errors::MemcacheError;
use types::{AsArgument, FromResponse, Expiration};

type Result<T> = BoxFuture<T, MemcacheError>;

pub struct Connection {
    pub inner: ClientService<TcpStream, MemcachedProto>,
}

impl Connection {

    pub fn get<K, R>(&self, key: &K) -> BoxFuture<R, MemcacheError>
            where K: AsArgument, R: FromResponse {
        let mut request = Request::new(Command::Get);
        request.set_key(key);

        self.send(request).and_then(|resp| {
            match R::try_from(resp.value().unwrap()) {
                Err(e) => future::err(MemcacheError::from(e)),
                Ok(res) => future::ok(res)
            }
        }).boxed()
    }

    pub fn set<K, V, E>(&self, key: &K, value: &V, expiration: E) -> Result<()>
            where K: AsArgument, V: AsArgument, E: Expiration {
        let mut request = Request::new(Command::Set);
        request.set_key(key);
        request.set_value(value);
        request.set_extras(extras::SetExtras{
            flags: 0,
            expiration: expiration.as_value(),
        });

        self.send(request).and_then(|_| future::ok(())).boxed()
    }

    pub fn add<K, V, E>(&self, key: &K, value: &V, expiration: E) -> BoxFuture<(), MemcacheError>
        where K: AsArgument, V: AsArgument, E: Expiration {
        let mut request = Request::new(Command::Add);
        request.set_key(key);
        request.set_extras(extras::AddExtras{
            flags: 0,
            expiration: expiration.as_value(),
        });
        request.set_value(value);

        self.send(request).and_then(|_| future::ok(())).boxed()
    }

    /// Send [`Request`][request] to server.
    ///
    /// This method can be used if you need to send a rich request
    /// and there is no available wrapper method.
    ///
    /// [request]: ./struct.Request.html
    pub fn send(&self, req: Request) -> Result<Response> {
        self.call(req)
    }

}

impl Service for Connection {
    type Request = Request;
    type Response = Response;
    type Error = MemcacheError;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        self.inner.call(req).then(|res| {
            match res {
                Ok(resp) => {
                    if resp.is_ok() {
                        future::ok(resp)
                    } else {
                        future::err(MemcacheError::from(resp))
                    }
                },
                Err(e) => future::err(MemcacheError::from(e)),
            }
        }).boxed()
    }
}

