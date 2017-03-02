#![feature(test)]

extern crate test;
extern crate futures;
extern crate tokio_core;
extern crate tokio_memcache;

use futures::Future;
use tokio_core::reactor::Core;

#[ignore] // TODO: Enable when connections reusage will be available
#[bench]
fn bench_command_get(b: &mut test::Bencher) {
    // TODO: clean up memcached after each test call
    let addr = "127.0.0.1:11211".parse().unwrap();
    let mut lp = Core::new().unwrap();

    b.iter(|| {
        let f = tokio_memcache::Client::connect(&addr, &lp.handle())
            .and_then(|conn| {
                conn.get(b"TestGet")
            });

        let res: Vec<u8> = lp.run(f).unwrap_or_default();
    });
}
