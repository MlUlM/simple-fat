use auto_delegate::Delegate;
use num_enum::TryFromPrimitive;
use crate::bpb::BpbReadable;

use crate::dir::entry::base::DirEntryReadable;
use crate::dir::entry::short::ShortDirEntry;
use crate::error::{FatError, FatResult};
use crate::FatDeviceAccessible;

pub mod short;
pub mod base;


#[derive(Debug, Copy, Clone, TryFromPrimitive, Eq, PartialEq, )]
#[repr(u8)]
pub enum Attribute {
    Readonly = 0x01,
    Hidden = 0x02,
    System = 0x04,
    VolumeLabel = 0x08,
    Dir = 0x10,
    Archive = 0x20,
    LongName = 0x0f,
}


#[derive(TryFromPrimitive, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum EntryStatus {
    End = 0x00,
    Deleted = 0xE5,
    JapaneseName = 0x05,
}


#[derive(Delegate)]
#[to(DirEntryReadable)]
pub enum DirEntry<D> where D: FatDeviceAccessible + Clone + BpbReadable {
    Short(ShortDirEntry<D>),
}


impl<D> DirEntry<D> where D: FatDeviceAccessible + DirEntryReadable{
    #[inline]
    pub const fn short(short: ShortDirEntry<D>) -> DirEntry<D> {
        Self::Short(short)
    }


    #[inline]
    pub fn into_short(self) -> FatResult<ShortDirEntry<D>> {
        if let Self::Short(short) = self {
            Ok(short)
        } else {
            Err(FatError::InvalidDirEntryType)
        }
    }
}


