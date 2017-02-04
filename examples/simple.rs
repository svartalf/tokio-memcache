extern crate futures;
extern crate tokio_core;
extern crate tokio_memcache;

use std::default::Default;
use futures::Future;
use tokio_core::reactor::Core;

use tokio_memcache::{Client, extras};

fn main() {
    let addr = "127.0.0.1:11211".parse().unwrap();
    let mut lp = Core::new().unwrap();

    let res = Client::connect(&addr, &lp.handle())
        .and_then(|conn| {
            conn.set(b"Hello", b"world", extras::SetExtras{
                expiration: 3600,
                ..Default::default()
            }).and_then(move |_| {
                conn.get(b"Hello")
            })
        })
    ;

    let val = lp.run(res).unwrap();
    println!("{:?}", val.value());
}