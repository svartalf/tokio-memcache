#![cfg_attr(feature = "nightly", feature(test))]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[cfg(feature = "nightly")]
extern crate test;

extern crate futures;
extern crate tokio_core;
extern crate tokio_service;
extern crate tokio_proto;
extern crate tokio_io;
extern crate bytes;
extern crate byteorder;
// extern crate serde;
// extern crate rmp_serde;

#[macro_use] extern crate enum_primitive;

mod protocol;
mod client;
mod errors;
mod types;

pub use client::{Client, Connection};
pub use protocol::{Request, Response, Command, extras};
pub use errors::{ErrorKind, MemcacheError};
pub use types::AsArgument;