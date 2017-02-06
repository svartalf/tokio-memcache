use std::boxed::Box;
use std::net::SocketAddr;

use tokio_proto::TcpClient;
use tokio_core::reactor::Handle;
use futures::Future;

use super::protocol::{MemcachedProto};
use super::connection::Connection;
use errors::MemcacheError;

/// TCP Client to `memcached` server
pub struct Client;

impl Client {

    pub fn connect(addr: &SocketAddr, handle: &Handle) -> Box<Future<Item=Connection, Error=MemcacheError>> {
        let result = TcpClient::new(MemcachedProto)
            .connect(addr, handle)
            .map_err(MemcacheError::from)
            .map(|client_service| {
                Connection {
                    inner: client_service,
                }
            });

        Box::new(result)
    }

}
