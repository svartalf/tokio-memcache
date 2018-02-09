use serde::Serialize;
use tokio_service::Service;

use protocol::{Request, Response, Command};
use io::handle::{ClientHandle, Result};


impl<K> ClientHandle<K> where K: Serialize {

    // TODO: Replace `Result` (which is a `Box<Future>`) with a `impl Future`
    pub fn get(&self, key: K) -> Result<Response> {
        let request = Request::build(Command::Get)
            .key(Some(key))
            .finish();
        self.call(request)
    }
}