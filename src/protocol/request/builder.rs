use protocol::{Request, Command};


pub struct Builder<K>(Request<K>);

impl<K> Builder<K> {

    pub fn new(command: Command) -> Builder<K> {
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

    pub fn key(mut self, value: Option<K>) -> Self {
        *self.0.key_mut() = value;
        self
    }

    pub fn value(mut self, value: Option<Vec<u8>>) -> Self {
        *self.0.value_mut() = value;
        self
    }

    pub fn finish(self) -> Request<K> {
        self.0
    }
}
