use std::convert::From;

use futures::{Future, Poll, Async};
use futures::sync::oneshot;

use protocol::Response;
use io::errors::Error;

/// Future that will eventually resolve to some kind of response.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct ResponseFuture{
    pub rx: oneshot::Receiver<Response>
}

impl Future for ResponseFuture {
    type Item = Response;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.rx.poll() {
            Ok(Async::Ready(resp)) => Ok(Async::Ready(resp)),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => Err(From::from(e)),
        }
    }
}
