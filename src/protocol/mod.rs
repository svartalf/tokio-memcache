/// Memcached binary protocol
///
/// Reference: https://github.com/memcached/memcached/wiki/BinaryProtocolRevamped

pub mod command;
pub mod request;
pub mod response;

pub use self::request::Request;
pub use self::command::Command;
pub use self::response::{Status, Response};

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


#[cfg(test)]
mod tests;
