use std::marker::PhantomData;
use std::convert::From;
use std::net::SocketAddr;

use futures::Future;
use tokio_core::reactor::Handle;
use tokio_proto::TcpClient;
use serde::Serialize;

use super::{MemcacheProto, ClientHandle, Error};

pub struct Client<K> {
    _key: PhantomData<K>,
}

impl<K> Client<K> where K: Serialize {
    pub fn connect(addr: &SocketAddr, handle: &Handle) -> Box<Future<Item=ClientHandle<K>, Error=Error>> {
        let handle = TcpClient::new(MemcacheProto::new())
            .connect(addr, handle)
            .map_err(From::from)
            .map(|client_service| {
                ClientHandle::new(client_service)
            });

        Box::new(handle)
    }
}
