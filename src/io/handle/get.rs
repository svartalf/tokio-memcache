use tokio_service::Service;

use protocol::{Request, Response, Command};
use io::handle::{ClientHandle, Result};


impl ClientHandle {
    pub fn get(&self, key: &[u8]) -> Result<Response> {
        let request = Request::build(Command::Get)
            .key(Some(key))
            .finish();
        self.call(request)
    }
}
