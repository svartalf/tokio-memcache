#![allow(dead_code)]

use std::io;

use byteorder::{NetworkEndian, WriteBytesExt};

use super::{Extras, IntoValue};

/// Extras for `get`/`getq`/`getk`/`getkq` commands.
#[derive(Default)]
pub struct GetExtras {
    pub flags: u32,
}

impl IntoValue for GetExtras {
    fn write<T: io::Write>(&self, buf: &mut T) -> io::Result<()> {
        buf.write_u32::<NetworkEndian>(self.flags)
    }
}
impl Extras for GetExtras {}

pub type GetQExtras = GetExtras;
pub type GetKExtras = GetExtras;
pub type GetKQExtras = GetExtras;

/// Extras for `set`/`add`/`replace` commands
#[derive(Default)]
pub struct SetExtras {
    pub flags: u32,
    pub expiration: u32,
}

impl IntoValue for SetExtras {
    fn write<T: io::Write>(&self, buf: &mut T) -> io::Result<()> {
        buf.write_u32::<NetworkEndian>(self.flags)?;
        buf.write_u32::<NetworkEndian>(self.expiration)
    }
}
impl Extras for SetExtras {}

pub type AddExtras = SetExtras;
pub type ReplaceExtras = SetExtras;

/// Extras for `incr`/`decr` commands
#[derive(Default)]
pub struct IncrExtras {
    pub amount: u64,
    pub initial: u64,
    pub expiration: u32,
}

impl IntoValue for IncrExtras {
    fn write<T: io::Write>(&self, buf: &mut T) -> io::Result<()> {
        buf.write_u64::<NetworkEndian>(self.amount)?;
        buf.write_u64::<NetworkEndian>(self.initial)?;
        buf.write_u32::<NetworkEndian>(self.expiration)
    }
}
impl Extras for IncrExtras {}

pub type DecrExtras = IncrExtras;
