use std::boxed::Box;

use errors::MemcacheError;

pub trait AsArgument {
    fn as_boxed_slice(&self) -> Box<[u8]>;
}

pub trait FromResponse: Sized + Send + 'static {
    fn try_from(resp: &[u8]) -> Result<Self, MemcacheError>;
}

impl<T> AsArgument for T where T: AsRef<[u8]> {
    fn as_boxed_slice(&self) -> Box<[u8]> {
        // TODO: inefficient, but working
        self.as_ref().to_vec().into_boxed_slice()
    }
}

impl FromResponse for Vec<u8> {
    fn try_from(resp: &[u8]) -> Result<Self, MemcacheError> {
        Ok(Vec::from(resp))
    }
}