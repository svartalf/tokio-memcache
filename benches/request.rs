#![feature(test)]

extern crate test;
extern crate tokio_memcache;

use tokio_memcache::{Request, Command};

#[bench]
fn bench_request_write(b: &mut test::Bencher) {
    let mut request = Request::new(Command::Get);
    request.set_key(b"hello");

    let mut result: Vec<u8> = vec![];
    result.reserve_exact(29);

    b.iter(|| {
        request.write(&mut result).unwrap();
        result.clear();
    });
}
