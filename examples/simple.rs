extern crate tokio;
extern crate tokio_memcache;
extern crate futures;

use tokio::executor::current_thread;

fn main() {
    let client = tokio_memcache::Client::new("127.0.0.1:11211");
    let f = client.connect();

    let res = current_thread::block_on_all(f).unwrap();
    println!("{:#?}", res);
}
