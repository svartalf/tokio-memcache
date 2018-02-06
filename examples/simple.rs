extern crate futures;
extern crate tokio_core;
extern crate tokio_memcache;

use std::error::Error;

use futures::Future;
use tokio_core::reactor::Core;

use tokio_memcache::Client;

fn main() {
    let addr = "127.0.0.1:11211".parse().unwrap();
    let mut lp = Core::new().unwrap();

    let res = Client::connect(&addr, &lp.handle())
        .and_then(|conn| {
            let key = b"hello";
            conn.get(key)
        });

    lp.run(res)
        .map(|response| {
            println!("Got a response: {:?}", response);
        }).map_err(|error| {
            // Probably you will see that line with a "Not found" text.
            // And it means that everything is working as intended.
            println!("Got an error: {}", error.description());
        });
}
