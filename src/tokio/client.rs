use std::convert::From;
use std::net::SocketAddr;

use futures::Future;
use tokio_core::reactor::Handle;
use tokio_proto::TcpClient;

use super::{MemcacheProto, ClientHandle, Error};

pub struct Client;

impl Client {
    pub fn connect(addr: &SocketAddr, handle: &Handle) -> Box<Future<Item=ClientHandle, Error=Error>> {
        let handle = TcpClient::new(MemcacheProto)
            .connect(addr, handle)
            .map_err(From::from)
            .map(|client_service| {
                ClientHandle {
                    inner: client_service,
                }
            });

        Box::new(handle)
    }
}
