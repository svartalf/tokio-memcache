use std::fmt;
use std::default;

use protocol::{Magic, Command, DataType};
use self::builder::Builder;

mod builder;

/// Represents an Memcached request.
///
/// A Memcached request have optional key, value and extras fields.
///
/// Key and value fields eventually will be generic, but as for early stages development
/// they are simple `Vec<u8>`.
///
/// # Examples
///
/// Creating a `Request` to send
///
/// ```
///
/// use tokio_memcache::protocol::Request;
///
/// let mut request = Request::new();
/// *request.key_mut() = Some(b"some-cached-value".to_vec());
/// ```
pub struct Request {
    magic: Magic,
    opcode: Command,
    data_type: DataType,
    vbucket_id: u16,
    opaque: u32,
    cas: u64,

    extras: Option<Vec<u8>>,
    key: Option<Vec<u8>>,
    value: Option<Vec<u8>>,
}

impl Request {

    /// Create a new blank `Request`.
    ///
    /// All fields will set to their defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio_memcache::protocol::Request;
    ///
    /// let request = Request::new();
    /// ```
    pub fn new() -> Request {
        Request {
            ..Self::default()
        }
    }

    /// Creates a new builder-style object to manufacture a `Request`.
    ///
    /// This method returns an instance of `Builder` which can be used to create a `Request`.
    pub fn build(command: Command) -> Builder {
        Builder::new(command)
    }

    /// Returns a reference to the associated `Command`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio_memcache::protocol::{Request, Command};
    ///
    /// let mut request = Request::new();
    /// *request.command_mut() = Command::Set;
    ///
    /// assert_eq!(*request.command(), Command::Set);
    /// ```
    pub fn command(&self) -> &Command {
        &self.opcode
    }

    /// Returns a mutable reference to the associated `Command`.
    ///
    /// Useful only at request creation, since `Request` struct is parametrized over `Command`,
    /// so it is impossible to replace command for an already instatiated `Request`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio_memcache::protocol::{Request, Command};
    ///
    /// let mut request = Request::new();
    /// *request.command_mut() = Command::Get;
    /// ```
    pub fn command_mut(&mut self) -> &mut Command {
        &mut self.opcode
    }

    /// Returns a reference to the data type.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio_memcache::protocol::{Request, DataType};
    ///
    /// let mut request = Request::new();
    ///
    /// assert_eq!(*request.data_type(), DataType::RawBytes);
    /// ```
    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }

    /// Returns a mutable reference to the associated data type.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio_memcache::protocol::{Request, DataType};
    ///
    /// let mut request = Request::new();
    /// *request.data_type_mut() = DataType::RawBytes;
    /// ```
    pub fn data_type_mut(&mut self) -> &mut DataType {
        &mut self.data_type
    }

    /// Returns a reference to the associated Virtual Bucket ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio_memcache::protocol::Request;
    ///
    /// let mut request = Request::new();
    ///
    /// assert_eq!(*request.vbucket_id(), 0);
    /// ```
    pub fn vbucket_id(&self) -> &u16 {
        &self.vbucket_id
    }

    /// Returns a mutable reference to the associated Virtual Bucket ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio_memcache::protocol::Request;
    ///
    /// let mut request = Request::new();
    /// *request.vbucket_id_mut() = 5;
    ///
    /// assert_eq!(*request.vbucket_id(), 5);
    /// ```
    pub fn vbucket_id_mut(&mut self) -> &mut u16 {
        &mut self.vbucket_id
    }

    /// Returns a reference to the associated Opaque value.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio_memcache::protocol::Request;
    ///
    /// let mut request = Request::new();
    ///
    /// assert_eq!(*request.opaque(), 0);
    /// ```
    pub fn opaque(&self) -> &u32 {
        &self.opaque
    }

    /// Returns a mutable reference to the associated Opaque value.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio_memcache::protocol::Request;
    ///
    /// let mut request = Request::new();
    /// *request.opaque_mut() = 5;
    ///
    /// assert_eq!(*request.opaque(), 5);
    /// ```
    pub fn opaque_mut(&mut self) -> &mut u32 {
        &mut self.opaque
    }

    /// Returns a reference to the associated Compare-and-swap value.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio_memcache::protocol::Request;
    ///
    /// let mut request = Request::new();
    ///
    /// assert_eq!(*request.cas(), 0);
    /// ```
    pub fn cas(&self) -> &u64 {
        &self.cas
    }

    /// Returns a mutable reference to the associated Compare-and-swap value.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio_memcache::protocol::Request;
    ///
    /// let mut request = Request::new();
    /// *request.cas_mut() = 42;
    ///
    /// assert_eq!(*request.cas(), 42);
    /// ```
    pub fn cas_mut(&mut self) -> &mut u64 {
        &mut self.cas
    }

    /// Returns a reference to the associated extras.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio_memcache::protocol::Request;
    ///
    /// let mut request = Request::new();
    ///
    /// assert!(request.extras().is_none());
    /// ```
    pub fn extras(&self) -> &Option<Vec<u8>> {
        &self.extras
    }

    /// Returns a mutable reference to the associated extras.
    ///
    /// Extras type is defined by Request' `Command`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use memcache_proto::{Request, command, extras};
    ///
    /// let mut request: Request<command::Set, Vec<u8>, Vec<u8>> = Request::new();
    /// let extras = extras::Set::new(0xdeadbeef, 3600);
    /// *request.extras_mut() = Some(extras);
    ///
    /// assert!(request.extras().is_some());
    ///
    /// let my_extras = request.extras().as_ref().unwrap();
    /// assert_eq!(*my_extras.flags(), 0xdeadbeef);
    /// assert_eq!(*my_extras.expiration(), 3600);
    /// ```
    pub fn extras_mut(&mut self) -> &mut Option<Vec<u8>> {
        &mut self.extras
    }

    pub fn key(&self) -> &Option<Vec<u8>> {
        &self.key
    }

    pub fn key_mut(&mut self) -> &mut Option<Vec<u8>> {
        &mut self.key
    }

    pub fn value(&self) -> &Option<Vec<u8>> {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut Option<Vec<u8>> {
        &mut self.value
    }

}


impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Request")
            .field("command", &self.opcode)
            .field("vbucket_id", &self.vbucket_id)
            .field("opaque", &self.opaque)
            .field("cas", &self.cas)
            .field("key", &self.key)
            .field("value", &self.value)
            .field("extras", &self.extras)
            .finish()
    }
}


impl default::Default for Request {
    fn default() -> Self {
        Request {
            magic: Magic::Request,
            opcode: Command::Get,
            data_type: DataType::RawBytes,
            vbucket_id: 0,
            opaque: 0,
            cas: 0,
            extras: None,
            key: None,
            value: None,
        }
    }
}
