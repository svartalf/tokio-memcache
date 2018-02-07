use std::fmt;
use std::default::Default;

use bytes::{BytesMut, BufMut};
use byteorder::NetworkEndian;

use super::Extras;

/// Extras container for `Set` requests.
///
/// All fields are zeroed by default.
///
/// Since `SetQ`, `Add`, `AddQ`, `Replace` and `ReplaceQ` requests use the same format,
/// [SetQ](type.SetQ.html), [Add](type.Add.html), [AddQ](type.AddQ.html),
/// [Replace](type.Replace.html) and [Replace](type.Replace.html) type aliases
/// can be used in order to provide consistent interface.
pub struct Set {
    flags: u32,
    expiration: u32,
}

/// Extras container for `SetQ` requests.
///
/// It is an alias for [Set](struct.Set.html) struct,
/// see [the module documentation](struct.Set.html) for more.
pub type SetQ = Set;

/// Extras container for `Add` requests.
///
/// It is an alias for [Set](struct.Set.html) struct,
/// see [the module documentation](struct.Set.html) for more.
pub type Add = Set;

/// Extras container for `AddQ` requests.
///
/// It is an alias for [Set](struct.Set.html) struct,
/// see [the module documentation](struct.Set.html) for more.
pub type AddQ = Set;

/// Extras container for `Replace` requests.
///
/// It is an alias for [Set](struct.Set.html) struct,
/// see [the module documentation](struct.Set.html) for more.
pub type Replace = Set;

/// Extras container for `ReplaceQ` requests.
///
/// It is an alias for [Set](struct.Set.html) struct,
/// see [the module documentation](struct.Set.html) for more.
pub type ReplaceQ = Set;

impl Set {

    pub fn new(flags: u32, expiration: u32) -> Set {
        Self {
            flags,
            expiration,
        }
    }

    pub fn flags(&self) -> &u32 {
        &self.flags
    }

    pub fn flags_mut(&mut self) -> &mut u32 {
        &mut self.flags
    }

    pub fn expiration(&self) -> &u32 {
        &self.expiration
    }

    pub fn expiration_mut(&mut self) -> &mut u32 {
        &mut self.expiration
    }
}


impl Extras for Set {
    fn to_vec(&self) -> Vec<u8> {
        let mut buf = BytesMut::with_capacity(8);
        buf.put_u32::<NetworkEndian>(self.flags);
        buf.put_u32::<NetworkEndian>(self.expiration);

        buf.to_vec()
    }
}

impl Default for Set {
    fn default() -> Self {
        Self {
            flags: 0,
            expiration: 0,
        }
    }
}

impl fmt::Debug for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Set")
            .field("flags", &self.flags)
            .field("expiration", &self.expiration)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::{Set, Extras};

    #[test]
    fn test_to_vec() {
        let extras = Set::new(0xdeadbeef, 3600);
        let expected = vec![0xdeu8, 0xad, 0xbe, 0xef, 0x00, 0x00, 0x0e, 0x10];

        assert_eq!(extras.to_vec(), expected);
    }
}
