#![cfg_attr(feature = "nightly", feature(test))]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[cfg(feature = "nightly")]
extern crate test;

extern crate futures;
extern crate tokio_core;
extern crate tokio_service;
extern crate tokio_proto;
extern crate byteorder;
#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate log;

mod protocol;
mod client;
mod errors;

pub use client::Client;
pub use protocol::{Request, Response, Command, extras};
pub use errors::{ErrorKind, MemcacheError as Error};
