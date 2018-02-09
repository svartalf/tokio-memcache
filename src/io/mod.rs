mod codec;
mod client;
mod handle;
mod proto;
mod errors;

pub use self::codec::MemcacheCodec;
pub use self::client::Client;
pub use self::handle::ClientHandle;
pub use self::proto::MemcacheProto;
pub use self::errors::Error;

#[cfg(test)]
mod tests;
