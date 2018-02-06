use bytes::{BytesMut, BufMut};
use byteorder::NetworkEndian;

use super::Extras;

/// Extras container for `Get` responses.
///
/// Since `GetK`, `GetQ` and `GetKQ` responses are using the same format,
/// associated type alias can be used in order to provide consistent interface.
///
/// See [GetK](type.GetK.html), [GetQ](type.GetQ.html) and [GetKQ](type.GetKQ.html) type aliases
/// for more.
///
/// # Examples
///
/// ```
/// use tokio_memcache::protocol::extras::Get;
///
/// let extras = Get::new(0xdeadbeef);
/// ```
#[derive(Debug)]
pub struct Get {
    flags: u32,
}

/// Extras container for `GetQ` requests.
///
/// It is an alias for [Get](struct.Get.html) struct,
/// see [the module documentation](struct.Get.html) for more.
pub type GetQ = Get;

/// Extras container for `GetK` requests.
///
/// It is an alias for [Get](struct.Get.html) struct,
/// see [the module documentation](struct.Get.html) for more.
pub type GetK = Get;

/// Extras container for `GetKQ` requests.
///
/// It is an alias for [Get](struct.Get.html) struct,
/// see [the module documentation](struct.Get.html) for more.
pub type GetKQ = Get;

impl Get {
    /// Create new extras container.
    pub fn new(flags: u32) -> Get {
        Get {
            flags,
        }
    }

    /// Returns a reference to the associated flags.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio_memcache::protocol::extras::Get;
    ///
    /// let get = Get::new(42);
    ///
    /// assert_eq!(*get.flags(), 42);
    /// ```
    pub fn flags(&self) -> &u32 {
        &self.flags
    }

    /// Returns a mutable reference to the associated flags.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio_memcache::protocol::extras::Get;
    ///
    /// let mut get = Get::new(0);
    /// *get.flags_mut() = 42;
    ///
    /// assert_eq!(*get.flags(), 42);
    /// ```
    pub fn flags_mut(&mut self) -> &mut u32 {
        &mut self.flags
    }

}

impl Extras for Get {
    fn to_vec(&self) -> Vec<u8> {
        let mut buf = BytesMut::with_capacity(4);
        buf.put_u32::<NetworkEndian>(self.flags);

        buf.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::{Get, Extras};

    #[test]
    fn test_to_vec() {
        let extras = Get::new(0xdeadbeef);
        let expected = vec![0xdeu8, 0xad, 0xbe, 0xef];

        assert_eq!(extras.to_vec(), expected);
    }
}
