use serde::Serialize;
use tokio_service::Service;

use protocol::{Request, Response, Command};
use io::handle::{ClientHandle, Result};


impl ClientHandle {
    pub fn get<K>(&self, key: K) -> Result<Response> where K: Serialize {
        let request = Request::build(Command::Get)
            .key(Some(key))
            .finish();
        self.call(request)
    }
}
