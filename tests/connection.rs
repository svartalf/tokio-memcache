extern crate futures;
extern crate tokio_core;
extern crate tokio_memcache;

use tokio_core::reactor::Core;

#[test]
fn test_connection_ok() {
    let addr = "127.0.0.1:11211".parse().unwrap();
    let mut lp = Core::new().unwrap();
    let connection = tokio_memcache::Client::connect(&addr, &lp.handle());

    let val = lp.run(connection);
    assert!(val.is_ok());
}

#[test]
fn test_connection_fail() {
    // I suppose we can be sure that there is no memcached on that port
    let addr = "127.0.0.1:11311".parse().unwrap();
    let mut lp = Core::new().unwrap();
    let connection = tokio_memcache::Client::connect(&addr, &lp.handle());

    let val = lp.run(connection);
    assert!(val.is_err());
}
