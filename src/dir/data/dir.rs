use alloc::format;
use core::fmt::{Debug, Formatter};

use auto_delegate::Delegate;

use crate::bpb::BpbReadable;
use crate::dir::data::DataEntries;
use crate::dir::entry::base::{BaseDirEntry, DirEntryReadable};
use crate::dir::entry::EntryStatus;
use crate::dir::entry::short::ShortDirEntry;
use crate::FatDeviceAccessible;

#[derive(Delegate)]
pub struct DirEntries<D>
    where D: FatDeviceAccessible + Clone + BpbReadable
{
    #[to(DirEntryReadable, ShortDirEntryReadable, BpbReadable, FatDeviceAccessible)]
    entry: ShortDirEntry<D>,

    base_offset: usize,

    offset: usize
}


impl<D> DirEntries<D>
    where D: FatDeviceAccessible + Clone + BpbReadable
{
    pub fn root(bpb: D, base_offset: usize) -> DirEntries<D> {
        let entry = ShortDirEntry::new(BaseDirEntry::new(bpb, base_offset));
        Self {
            entry,
            base_offset,
            offset: base_offset
        }
    }

    pub fn from_entry(entry: ShortDirEntry<D>, base_offset: usize) -> DirEntries<D> {
        Self {
            entry,
            base_offset,
            offset: base_offset
        }
    }


    #[inline]
    pub fn into_data_entries(self) -> DataEntries<D> {
        DataEntries::new(self)
    }


    #[inline]
    fn offset(&self, offset: usize) -> usize {
        self.offset + offset
    }


    fn find_next(&mut self, offset: usize) -> Option<BaseDirEntry<D>> {
        let entry = BaseDirEntry::new(self.entry.base.bpb.clone(), offset);

        if let Some(status) = entry.status() {
            match status {
                EntryStatus::End => None,
                _ => self.find_next(offset + 0x20)
            }
        } else {
            self.offset = offset + 0x20;
            Some(entry)
        }
    }
}


impl<D> Clone for DirEntries<D>
 where D: FatDeviceAccessible + Clone + BpbReadable
{
    fn clone(&self) -> Self {
        Self{
            entry: self.entry.clone(),
            base_offset: self.base_offset,
            offset: 0,
        }
    }
}



impl<D> Iterator for DirEntries<D>
    where D: FatDeviceAccessible + Clone + BpbReadable
{
    type Item = BaseDirEntry<D>;

    fn next(&mut self) -> Option<Self::Item> {
        self.find_next(self.offset(0))
    }
}



impl<D> Debug for DirEntries<D> where D: FatDeviceAccessible + Clone + BpbReadable {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f
            .debug_struct("DirEntries")
            .field("base_offset", &format!("0x{:X}", self.base_offset))
            .finish()
    }
}