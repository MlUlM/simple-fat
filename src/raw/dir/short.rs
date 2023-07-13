#[cfg(feature = "alloc")]
use alloc::ffi::CString;
use core::fmt::{Debug, Formatter};

use num_enum::TryFromPrimitive;

use crate::error::FatResult;
use crate::FatDeviceAccessible;
use crate::raw::bpb::BpbFat32;
use crate::raw::dir;
use crate::raw::dir::{Attribute, Entry};

pub(crate) mod buffer;


#[derive(TryFromPrimitive)]
#[repr(u8)]
pub enum EntryStatus {
    End = 0x00,
    Deleted = 0xE5,
    JapaneseName = 0x05,
}


pub trait ShortDirEntryReadable {
    fn name_buff(&self) -> FatResult<[u8; 11]>;


    fn attribute_raw(&self) -> FatResult<u8>;


    fn first_cluster_no_hi(&self) -> FatResult<u16>;


    fn first_cluster_no_lo(&self) -> FatResult<u16>;


    fn status(&self) -> Option<EntryStatus> {
        let first = self.name_buff().ok()?[0];
        EntryStatus::try_from_primitive(first).ok()
    }


    #[cfg(feature = "alloc")]
    fn name(&self) -> FatResult<CString> {
        let buff = self
            .name_buff()?
            .iter()
            .skip(1)
            .map(|b| {
                if *b == 0x20 {
                    0
                } else {
                    *b
                }
            })
            .collect();

        unsafe {
            Ok(CString::from_vec_unchecked(buff))
        }
    }


    fn file_size(&self) -> FatResult<u32>;


    #[inline]
    fn first_cluster_no(&self) -> FatResult<u32> {
        let hi = (self.first_cluster_no_hi()? as u32) << 16;
        let lo = self.first_cluster_no_lo()? as u32;

        Ok(hi | lo)
    }


    #[inline]
    fn attribute(&self) -> FatResult<Attribute> {
        Ok(Attribute::try_from_primitive(self.attribute_raw()?)?)
    }
}


pub struct ShortDirEntry<D> {
    device: D,
    bpb: BpbFat32<D>,
    base_offset: usize,
}


impl<D> ShortDirEntry<D> where D: FatDeviceAccessible + Clone {
    #[inline]
    pub const fn new(device: D, bpb: BpbFat32<D>, offset: usize) -> Self {
        Self {
            device,
            bpb,
            base_offset: offset,
        }
    }


    #[inline]
    fn offset(&self, offset: usize) -> usize {
        self.base_offset + offset
    }


    fn find_entry(&mut self, entry: ShortDirEntry<D>, offset: usize) -> Option<dir::Entry<D>> {
        #[cfg(test)]
        println!("{:?}", entry);

        let attribute = entry.attribute().ok()?;
        match attribute {
            Attribute::LongName => self.find_next(offset + 0x20),
            Attribute::Dir => {
                // let offset = self.bpb.data_cluster_at(self.first_cluster_no().ok()? as usize).ok()?;
                Some(Entry::Dir(ShortDirEntry::new(self.device.clone(), self.bpb.clone(), offset)))
            }
            _ => {
                // TODO: File Handleを表す構造体を渡す
                Some(dir::Entry::RegularFile)
            }
        }
    }

    fn find_next(&mut self, offset: usize) -> Option<dir::Entry<D>> {
        let entry = ShortDirEntry::new(self.device.clone(), self.bpb.clone(), offset);
        if let Some(status) = entry.status() {
            match status {
                EntryStatus::End => None,
                _ => self.find_next(offset + 0x20)
            }
        } else {
            self.find_entry(entry, offset)
        }
    }
}


impl<D> ShortDirEntryReadable for ShortDirEntry<D>
    where D: FatDeviceAccessible + Clone
{
    #[inline]
    fn name_buff(&self) -> FatResult<[u8; 11]> {
        let mut buff = [0; 11];
        self.device.read(&mut buff, self.offset(0), 11)?;

        Ok(buff)
    }


    #[inline]
    fn attribute_raw(&self) -> FatResult<u8> {
        self.device.read_u8(self.offset(11))
    }


    #[inline]
    fn first_cluster_no_hi(&self) -> FatResult<u16> {
        self.device.read_u16(self.offset(20))
    }


    #[inline]
    fn first_cluster_no_lo(&self) -> FatResult<u16> {
        self.device.read_u16(self.offset(26))
    }


    #[inline]
    fn file_size(&self) -> FatResult<u32> {
        self.device.read_u32(self.offset(28))
    }
}


impl<D> Debug for ShortDirEntry<D> where D: FatDeviceAccessible + Clone {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f
            .debug_struct("DirEntry")
            .field("name_buff", &self.name())
            .field("first_cluster_no", &self.first_cluster_no())
            .field("file_size", &self.file_size())
            .field("attribute", &self.attribute())
            .finish()
    }
}


impl<D> Iterator for ShortDirEntry<D>
    where D: FatDeviceAccessible + Clone
{
    type Item = dir::Entry<D>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_offset = self.offset(0x20);
        self.find_next(next_offset)
    }
}


#[cfg(test)]
mod tests {
    use crate::raw::bpb::BpbFat32;
    use crate::raw::dir::Attribute;
    use crate::raw::dir::short::{ShortDirEntry, ShortDirEntryReadable};
    use crate::test::{file_device, FileDevice};

    fn root_dir() -> ShortDirEntry<FileDevice> {
        BpbFat32::new(file_device())
            .root_dir()
            .unwrap()
    }

    #[test]
    fn it_dir_name() {
        let dir = BpbFat32::new(file_device())
            .root_dir()
            .unwrap();

        println!("{:?}", dir.name())
    }


    #[test]
    fn it_dir_attribute() {
        let dir = BpbFat32::new(file_device())
            .root_dir()
            .unwrap();

        assert_eq!(dir.attribute(), Ok(Attribute::VolumeLabel));
    }


    #[test]
    fn it_dir_fist_cluster_is_zero_if_volume_label() {
        let dir = BpbFat32::new(file_device())
            .root_dir()
            .unwrap();

        assert_eq!(dir.first_cluster_no(), Ok(0x00));
    }


    #[test]
    fn it_fist_regular_file() {
        let mut root = root_dir();
        assert!(root.next().unwrap().is_regular_file())
    }


    #[test]
    fn it_second_dir() {
        let mut root = root_dir();
        root.next();

        let dir = root
            .next()
            .unwrap()
            .into_dir()
            .unwrap()
            .name()
            .unwrap();

        let dir_name = dir.to_str().unwrap();
        assert_eq!(dir_name, "test");
    }
}