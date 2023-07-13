use num_enum::TryFromPrimitive;

use crate::error::{FatError, FatResult};
use crate::FatDeviceAccessible;
use crate::raw::dir::short::ShortDirEntry;

pub mod long;
pub mod short;


#[derive(Debug)]
pub enum Entry<D> where D: FatDeviceAccessible + Clone {
    RegularFile,
    Dir(ShortDirEntry<D>),
}


impl<D> Entry<D> where D: FatDeviceAccessible + Clone {
    pub fn into_dir(self) -> FatResult<ShortDirEntry<D>> {
        if let Self::Dir(dir) = self {
            Ok(dir)
        } else {
            Err(FatError::InvalidDirEntryType)
        }
    }



    #[inline]
    pub fn is_regular_file(&self) -> bool{
        matches!(self, Self::RegularFile)
    }
}


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


