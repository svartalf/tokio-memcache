use std::io;

use tokio::prelude::*;

use super::Connection;
use protocol::Response;


impl Stream for Connection {
    type Item = Response;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<<Self as Stream>::Item>, <Self as Stream>::Error> {
        unimplemented!()
    }
}
