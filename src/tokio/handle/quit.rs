use tokio_service::Service;

use protocol::{Request, Response, Command};
use tokio::handle::{ClientHandle, Result};


impl ClientHandle<()> {

    // TODO: Should not `quit()` be called on the handle drop?
    pub fn quit(&self) -> Result<Response> {
        let request: Request<()> = Request::build(Command::Quit)
            .finish();

        self.call(request)
    }
}