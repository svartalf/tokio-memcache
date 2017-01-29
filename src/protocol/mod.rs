/// Memcached binary protocol
///
/// Reference: https://github.com/memcached/memcached/wiki/BinaryProtocolRevamped

pub mod command;
pub mod request;
pub mod response;

pub use self::request::Request;
pub use self::command::Command;
pub use self::response::Response;

#[cfg(test)]
mod tests;
