extern crate futures;
extern crate tokio_core;
extern crate tokio_service;
extern crate tokio_proto;
extern crate tokio_io;
extern crate bytes;
extern crate byteorder;
#[macro_use] extern crate enum_primitive;

mod io;
pub mod protocol;

pub use io::{Client, ClientHandle};
