#![allow(dead_code)] // TODO: Temporary, during `io` module rewrite

#[macro_use] extern crate futures;
extern crate tokio;
extern crate tokio_io;
extern crate tower;
extern crate bytes;
extern crate byteorder;
#[macro_use] extern crate enum_primitive;

mod io;
mod protocol;

pub use io::client::Client;
