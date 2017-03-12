use std::time;
use std::boxed::Box;

use errors::MemcacheError;

const EXPIRATION_THRESHOLD: u32 = 60 * 60 * 24 * 30;

pub trait AsArgument {
    fn as_boxed_slice(&self) -> Box<[u8]>;
}

pub trait FromResponse: Sized + Send + 'static {
    fn try_from(resp: &[u8]) -> Result<Self, MemcacheError>;
}

/// Convert any integer to proper expiration value
pub trait Expiration {
    fn as_value(&self) -> u32;
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

impl Expiration for u8 {
    fn as_value(&self) -> u32 { *self as u32 }
}

impl Expiration for i8 {
    fn as_value(&self) -> u32 {
        if *self < 0 {
            panic!("Expiration value must be greater than zero")
        } else {
            *self as u32
        }
    }
}

impl Expiration for u16 {
    fn as_value(&self) -> u32 { *self as u32 }
}

impl Expiration for i16 {
    fn as_value(&self) -> u32 {
        if *self < 0 {
            panic!("Expiration value must be greater than zero")
        } else {
            *self as u32
        }
    }
}

impl Expiration for u32 {
    fn as_value(&self) -> u32 {
        if *self < EXPIRATION_THRESHOLD {
            *self
        } else {
            let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH)
                .expect("Current timestamp");
            let diff = now + time::Duration::from_secs(*self as u64);

            diff.as_secs() as u32
        }
    }
}

impl Expiration for i32 {
    fn as_value(&self) -> u32 {
        if *self < 0 {
            panic!("Expiration value must be greater than zero");
        }

        if (*self as u32) < EXPIRATION_THRESHOLD {
            *self as u32
        } else {
            let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH)
                .expect("Current timestamp");
            let diff = now + time::Duration::from_secs(*self as u64);

            diff.as_secs() as u32
        }
    }
}

impl Expiration for u64 {
    fn as_value(&self) -> u32 {
        if *self < EXPIRATION_THRESHOLD as u64 {
            *self as u32
        } else {
            let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH)
                .expect("Current timestamp");
            let diff = now + time::Duration::from_secs(*self);

            diff.as_secs() as u32
        }
    }
}

impl Expiration for i64 {
    fn as_value(&self) -> u32 {
        if *self < 0 {
            panic!("Expiration value must be greater than zero");
        }

        if *self < EXPIRATION_THRESHOLD as i64 {
            *self as u32
        } else {
            let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH)
                .expect("Current timestamp");
            let diff = now + time::Duration::from_secs(*self as u64);

            diff.as_secs() as u32
        }
    }
}