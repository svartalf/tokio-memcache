use tokio_service::Service;

use protocol::{Request, Response, Command};
use io::handle::{ClientHandle, Result};


impl ClientHandle {
    pub fn version(&self) -> Result<Response> {
        let request = Request::build(Command::Version)
            .finish();

        self.call(request)
    }
}