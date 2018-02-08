use serde::Serialize;
use tokio_service::Service;

use protocol::{Request, Response, Command};
use tokio::handle::{ClientHandle, Result};


impl<K> ClientHandle<K> where K: Serialize {

    // TODO: Check if `Response` even needed in returned result
    pub fn delete(&self, key: K) -> Result<Response> {
        let request = Request::build(Command::Delete)
            .key(Some(key))
            .finish();

        self.call(request)
    }
}