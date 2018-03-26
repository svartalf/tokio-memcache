use std::io;

use tokio::prelude::*;
use futures::StartSend;

use protocol::Request;
use io::codec::MemcacheCodec;
use super::Connection;


impl Sink for Connection {
    type SinkItem = Request;
    type SinkError = io::Error;

    fn start_send(&mut self, item: <Self as Sink>::SinkItem) -> StartSend<<Self as Sink>::SinkItem, <Self as Sink>::SinkError> {
        MemcacheCodec::encode(&item, &mut self.wr)?;

        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), <Self as Sink>::SinkError> {
        while !self.wr.is_empty() {
            let n = try_ready!(self.socket.poll_write(&self.wr));
            assert!(n > 0);

            let _ = self.wr.split_to(n);
        }

        Ok(Async::Ready(()))
    }
    fn close(&mut self) -> Result<Async<()>, <Self as Sink>::SinkError> {
        self.poll_complete()?;

        AsyncWrite::shutdown(&mut self.socket)
    }
}
