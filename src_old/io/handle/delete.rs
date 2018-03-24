use tokio_service::Service;

use protocol::{Request, Response, Command};
use io::handle::{ClientHandle, Result};


impl ClientHandle {

    // TODO: Check if `Response` even needed in returned result
    pub fn delete(&self, key: &[u8]) -> Result<Response> {
        let request = Request::build(Command::Delete)
            .key(Some(key))
            .finish();

        self.call(request)
    }
}
