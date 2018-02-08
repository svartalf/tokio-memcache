extern crate futures;
extern crate tokio_core;
extern crate tokio_memcache;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use std::error::Error;

use futures::Future;
use tokio_core::reactor::Core;

use tokio_memcache::Client;


#[derive(Debug, Serialize, Deserialize)]
struct FakeKey(String);


fn main() {
    let addr = "127.0.0.1:11211".parse().unwrap();
    let mut lp = Core::new().unwrap();

    let res = Client::connect(&addr, &lp.handle())
        .and_then(|conn| {
            let key = FakeKey("Hello".to_string());
            conn.get(key)
        });

    lp.run(res)
        .map(|response| {
            println!("Got a response: {:?}", response);
        }).map_err(|error| {
            // Probably you will see that line with a "Not found" text.
            // And it means that everything is working as intended.
            println!("Got an error: {}", error.description());
        });
}
