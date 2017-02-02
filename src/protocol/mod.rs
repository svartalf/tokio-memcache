/// Memcached binary protocol
///
/// Reference: [](https://github.com/memcached/memcached/wiki/BinaryProtocolRevamped)

use std::io;

pub mod command;
pub mod request;
pub mod response;
pub mod extras;

pub use self::request::Request;
pub use self::command::Command;
pub use self::response::{Status, Response};
pub use self::extras::*;

enum Magic {
    Request = 0x80,
    Response = 0x81,
}

enum_from_primitive! {
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum DataType {
        RawBytes = 0x00,
    }
}

pub trait IntoValue {
    fn write<T: io::Write>(&self, buf: &mut T) -> io::Result<()>;
}

pub trait Extras: IntoValue {}

#[cfg(test)]
mod tests;
