use bytes::{BytesMut, BufMut};
use byteorder::NetworkEndian;

use super::Extras;

/// Extras container for `Flush` requests.
///
/// Since `FlushQ` requests are using the same format,
/// associated type alias can be used in order to provide consistent interface.
///
/// See [FlushQ](type.FlushQ.html) type alias for more.
///
/// # Examples
///
/// ```
/// use tokio_memcache::protocol::extras::Flush;
///
/// let extras = Flush::new(3600);
/// ```
#[derive(Debug)]
pub struct Flush {
    expiration: u32,
}

impl Flush {
    pub fn new(expiration: u32) -> Flush {
        Flush {
            expiration,
        }
    }

    pub fn expiration(&self) -> &u32 {
        &self.expiration
    }

    pub fn expiration_mut(&mut self) -> &mut u32 {
        &mut self.expiration
    }
}

impl Extras for Flush {
    fn to_vec(&self) -> Vec<u8> {
        let mut buf = BytesMut::with_capacity(4);
        buf.put_u32::<NetworkEndian>(self.expiration);

        buf.to_vec()
    }
}

/// Extras container for `FlushQ` requests.
///
/// It is an alias for [Flush](struct.Flush.html) struct,
/// see [the module documentation](struct.Flush.html) for more.
pub type FlushQ = Flush;

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use super::{Flush, Extras};

    #[test]
    fn test_to_vec() {
        let extras = Flush::new(3600);
        let expected = Bytes::from_static(&[0x00, 0x00, 0x0e, 0x10]);

        assert_eq!(extras.to_vec(), expected);
    }
}