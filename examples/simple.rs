extern crate futures;
extern crate tokio_core;
extern crate tokio_memcache;

use futures::Future;
use tokio_core::reactor::Core;

fn main() {
    let addr = "127.0.0.1:11211".parse().unwrap();
    let mut lp = Core::new().unwrap();

    let res = tokio_memcache::Client::connect(&addr, &lp.handle())
        .and_then(|client| {
            client.set("Hello".as_bytes(), "world".as_bytes())
                .and_then(move |_| {
                    client.get("Hello".as_bytes())
                })
        });

    let val = lp.run(res).unwrap();
    println!("{:#?}", val);
}