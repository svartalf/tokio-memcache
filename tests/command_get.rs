extern crate futures;
extern crate tokio_core;
extern crate tokio_memcache;

use tokio_core::reactor::Core;

/*
#[test]
fn test_get_for_non_existent_key() {
    let addr = "127.0.0.1:11211".parse().unwrap();
    let mut lp = Core::new().unwrap();

    let res = tokio_memcache::Client::connect(&addr, &lp.handle())
        .and_then(|client| {
            client.get(b"key")
        });

    let response = lp.run(res).unwrap();
    assert!(response.value().is_none());
}
*/