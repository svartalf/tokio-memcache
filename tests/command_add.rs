extern crate futures;
extern crate tokio_core;
extern crate tokio_memcache;

use futures::Future;
use tokio_core::reactor::Core;

#[test]
fn test_command_add_ok() {
    // TODO: clean up memcached after each test call
    let addr = "127.0.0.1:11211".parse().unwrap();
    let mut lp = Core::new().unwrap();
    let connection = tokio_memcache::Client::connect(&addr, &lp.handle())
        .and_then(|conn| {
            conn.add(b"AddOk", b"1", 1)
        });

    let val = lp.run(connection);
    assert!(val.is_ok());
}
