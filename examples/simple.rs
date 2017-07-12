extern crate futures;
extern crate tokio_core;
extern crate tokio_memcache;

use futures::Future;
use tokio_core::reactor::Core;

use tokio_memcache::{Client, extras};

fn main() {
    let addr = "127.0.0.1:11211".parse().unwrap();
    let mut lp = Core::new().unwrap();

    let res = Client::connect(&addr, &lp.handle())
        .and_then(|conn| {
            conn.set(b"Hello", b"world", 3600)
                .and_then(move |_| {
                    conn.get(b"Hello")
                })
        });

    let val: Vec<u8> = lp.run(res).unwrap();
    println!("{:?}", val);
}