use alloc::format;
use core::fmt::{Debug, Formatter};

use auto_delegate::delegate;
use num_enum::TryFromPrimitive;

use crate::bpb::BpbReadable;
use crate::dir::entry::{Attribute, DirEntry, EntryStatus};
use crate::dir::entry::short::ShortDirEntry;
use crate::error::{FatError, FatResult};
use crate::FatDeviceAccessible;

#[delegate]
pub trait DirEntryReadable {
    fn status_raw(&self) -> FatResult<u8>;


    fn attribute_raw(&self) -> FatResult<u8>;


    #[inline]
    fn status(&self) -> Option<EntryStatus> {
        Some(EntryStatus::try_from_primitive(self.status_raw().ok()?).ok()?)
    }


    #[inline]
    fn attribute(&self) -> FatResult<Attribute> {
        if self.attribute_raw()? == 0x00 {
            return Ok(Attribute::LongName);
        }

        let attribute = Attribute::try_from_primitive(self.attribute_raw()?)?;
        Ok(attribute)
    }
}


#[derive(Clone, )]
pub struct BaseDirEntry<D> {
    pub(crate) bpb: D,
    pub(crate) offset: usize,
}


impl<D> BaseDirEntry<D>
    where D: FatDeviceAccessible + Clone + BpbReadable
{
    #[inline]
    pub const fn new(bpb: D, offset: usize) -> BaseDirEntry<D> {
        Self {
            bpb,
            offset,
        }
    }


    #[inline]
    pub fn into_detail(self) -> FatResult<DirEntry<D>> {
        match self.attribute()? {
            Attribute::LongName => Err(FatError::InvalidDirEntryType),
            _ => Ok(DirEntry::Short(ShortDirEntry::new(self)))
        }
    }
}


impl<D> DirEntryReadable for BaseDirEntry<D>
    where D: FatDeviceAccessible
{
    #[inline]
    fn status_raw(&self) -> FatResult<u8> {
        self.bpb.read_u8(self.offset)
    }


    #[inline]
    fn attribute_raw(&self) -> FatResult<u8> {
        self.bpb.read_u8(self.offset + 11)
    }
}


impl<D> Debug for BaseDirEntry<D> where D: FatDeviceAccessible {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f
            .debug_struct("BaseDirEntry")
            .field("attribute", &self.attribute())
            .field("status", &self.status())
            .field("offset", &format!("0x{:X}", self.offset))
            .finish()
    }
}