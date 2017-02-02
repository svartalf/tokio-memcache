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

type GetQExtras = GetExtras;
type GetKExtras = GetExtras;
type GetKQExtras = GetExtras;

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

type AddExtras = SetExtras;
type ReplaceExtras = SetExtras;

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

type DecrExtras = IncrExtras;
