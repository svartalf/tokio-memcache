use serde::Serialize;
use tokio_service::Service;

use protocol::{Request, Response, Command};
use io::handle::{ClientHandle, Result};


impl ClientHandle {

    pub fn set<K, V>(&self, key: K, value: V) -> Result<Response> where K: Serialize, V: Serialize {
        let request = Request::build(Command::Set)
            .key(Some(key))
            .value(Some(value))
            .finish();

        self.call(request)
    }
}
