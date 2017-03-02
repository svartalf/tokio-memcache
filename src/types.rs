use std::boxed::Box;

use errors::MemcacheError;

pub trait AsArgument {
    fn as_boxed_slice(&self) -> Box<[u8]>;
}

pub trait FromResponse: Sized + Send + 'static {
    fn try_from(resp: &[u8]) -> Result<Self, MemcacheError>;
}

#[cfg(not(feature = "with-serde"))]
mod plain {
    use super::{AsArgument, FromResponse, MemcacheError};

    impl<T> AsArgument for T where T: AsRef<[u8]> {
        fn as_boxed_slice(&self) -> Box<[u8]> {
            self.as_ref().to_vec().into_boxed_slice()
        }
    }

    impl FromResponse for Vec<u8> {
        fn try_from(resp: &[u8]) -> Result<Self, MemcacheError> {
            Ok(Vec::from(resp))
        }
    }
}

#[cfg(feature = "with-serde")]
mod serde_compat {
    use serde::{Serialize, Deserialize};
    use rmp_serde::{Deserializer, to_vec};

    use super::{AsArgument, FromResponse, MemcacheError};

    impl<T> AsArgument for T where T: Serialize {
        fn as_boxed_slice(&self) -> Box<[u8]> {
            to_vec(&self).expect("Properly serializable object").into_boxed_slice()
        }
    }

    impl<T> FromResponse for T where T: Deserialize + Send + 'static {
        fn try_from(resp: &[u8]) -> Result<Self, MemcacheError> {
            unimplemented!()
        }
    }
}
