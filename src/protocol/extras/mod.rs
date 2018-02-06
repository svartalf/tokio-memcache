use std::fmt::Debug;

mod get;
mod set;
mod flush;
mod increment;
mod verbosity;

pub use self::get::*;
pub use self::set::*;
pub use self::increment::*;
pub use self::flush::*;
pub use self::verbosity::*;

pub trait Extras: Sized + Debug {
    fn to_vec(&self) -> Vec<u8>;
}
