use tokio_service::Service;

use protocol::{Request, Response, Command};
use io::handle::{ClientHandle, Result};


impl ClientHandle {

    pub fn set(&self, key: &[u8], value: &[u8]) -> Result<Response> {
        let request = Request::build(Command::Set)
            .key(Some(key))
            .value(Some(value))
            .finish();

        self.call(request)
    }
}
