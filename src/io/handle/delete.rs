use serde::Serialize;
use tokio_service::Service;

use protocol::{Request, Response, Command};
use io::handle::{ClientHandle, Result};


impl ClientHandle {

    // TODO: Check if `Response` even needed in returned result
    pub fn delete<K>(&self, key: K) -> Result<Response> where K: Serialize {
        let request = Request::build(Command::Delete)
            .key(Some(key))
            .finish();

        self.call(request)
    }
}
