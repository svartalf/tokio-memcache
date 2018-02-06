extern crate futures;
extern crate tokio_core;
extern crate tokio_memcache;

use std::str;
use std::error::Error;

use futures::Future;
use tokio_core::reactor::Core;

use tokio_memcache::Client;

fn main() {
    let addr = "127.0.0.1:11211".parse().unwrap();
    let mut lp = Core::new().unwrap();

    let res = Client::connect(&addr, &lp.handle())
        .and_then(|conn| {
            conn.version()
        });

    lp.run(res)
        .map(|response| {
            match response.value() {
                Some(bytes) => println!("Memcached version: {}", str::from_utf8(bytes).unwrap()),
                None => println!("Got an invalid response")
            }
        }).map_err(|error| {
            println!("Got an error: {}", error.description());
        });
}
