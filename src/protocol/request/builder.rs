use std::convert::AsRef;

use protocol::{Request, Command};


pub struct Builder(Request);

impl Builder {

    pub fn new(command: Command) -> Builder {
        Builder(Request::new()).command(command)
    }

    pub fn command(mut self, value: Command) -> Self {
        *self.0.command_mut() = value;
        self
    }

    pub fn vbucket_id(mut self, value: u16) -> Self {
        *self.0.vbucket_id_mut() = value;
        self
    }

    pub fn opaque(mut self, value: u32) -> Self {
        *self.0.opaque_mut() = value;
        self
    }

    pub fn cas(mut self, value: u64) -> Self {
        *self.0.cas_mut() = value;
        self
    }

    pub fn extras(mut self, value: Option<Vec<u8>>) -> Self {
        *self.0.extras_mut() = value;
        self
    }

    pub fn key<K>(mut self, key: Option<K>) -> Self where K: AsRef<[u8]> {
        *self.0.key_mut() = match key {
            None => None,
            Some(ref key) => Some(key.as_ref().to_vec()),
        };
        self
    }

    pub fn value<V>(mut self, value: Option<V>) -> Self where V: AsRef<[u8]> {
        *self.0.value_mut() = match value {
            None => None,
            Some(ref value) => Some(value.as_ref().to_vec())
        };
        self
    }

    pub fn finish(self) -> Request {
        self.0
    }
}
