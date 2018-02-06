use bytes::ByteOrder;
use byteorder::NetworkEndian;

use super::Extras;

/// Extras container for `Verbosity` requests.
///
/// # Examples
///
/// ```
/// use tokio_memcache::protocol::extras::Verbosity;
///
/// let extras = Verbosity::new(2);
/// ```
#[derive(Debug)]
pub struct Verbosity {
    verbosity: u32,
}

impl Verbosity {
    pub fn new(verbosity: u32) -> Verbosity {
        Verbosity {
            verbosity
        }
    }

    pub fn verbosity(&self) -> &u32 {
        &self.verbosity
    }

    pub fn verbosity_mut(&mut self) -> &mut u32 {
        &mut self.verbosity
    }
}

impl Extras for Verbosity {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(4);
        NetworkEndian::write_u32(&mut vec, self.verbosity);

        vec
    }
}
