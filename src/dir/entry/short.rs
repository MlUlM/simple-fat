use alloc::ffi::CString;
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter};

use auto_delegate::{delegate, Delegate};

use crate::bpb::BpbReadable;
use crate::dir::data::Data;
use crate::dir::data::dir::DirEntries;
use crate::dir::data::file::RegularFile;
use crate::dir::entry::Attribute;
use crate::dir::entry::base::{BaseDirEntry, DirEntryReadable};
use crate::error::FatResult;
use crate::FatDeviceAccessible;

#[delegate]
pub trait ShortDirEntryReadable {
    fn name_buff(&self) -> FatResult<[u8; 11]>;


    fn first_cluster_no_hi(&self) -> FatResult<u16>;


    fn first_cluster_no_lo(&self) -> FatResult<u16>;


    fn file_size(&self) -> FatResult<u32>;


    #[cfg(feature = "alloc")]
    fn name(&self) -> FatResult<CString> {
        let buff = self.name_buff()?;

        let mut prefix = buff
            .iter()
            .take(8)
            .copied()
            .collect::<Vec<u8>>();

        while prefix.last().map(|b| *b == 0x20).unwrap_or(false) {
            prefix.pop();
        }

        let mut suffix = buff
            .iter()
            .skip(8)
            .take_while(|b| **b != 0x20)
            .peekable();

        if suffix.peek().map(|b| **b != 0x20).unwrap_or(false) {
            prefix.push(0x2E);
            prefix.extend(suffix);
        }

        unsafe {
            Ok(CString::from_vec_unchecked(prefix))
        }
    }


    #[inline]
    fn first_cluster_no(&self) -> FatResult<u32> {
        let hi = (self.first_cluster_no_hi()? as u32) << 16;
        let lo = self.first_cluster_no_lo()? as u32;

        Ok(hi | lo)
    }


    #[inline]
    fn file_size_usize(&self) -> FatResult<usize> {
        Ok(self.file_size()? as usize)
    }
}


#[derive(Clone, Delegate)]
pub struct ShortDirEntry<D>
    where D: FatDeviceAccessible + BpbReadable
{
    #[to(DirEntryReadable, BpbReadable, FatDeviceAccessible)]
    pub(crate) base: BaseDirEntry<D>,
}


impl<D> ShortDirEntry<D>
    where D: FatDeviceAccessible + Clone + BpbReadable
{
    #[inline]
    pub const fn new(base: BaseDirEntry<D>) -> Self {
        Self {
            base
        }
    }


    pub fn data(&self) -> FatResult<Data<D>> {
        if let Attribute::Dir = self.attribute()? {
            Ok(Data::Dir(DirEntries::from_entry(self.clone(), self.first_cluster_no()? as usize)))
        } else {
            Ok(Data::RegularFile(RegularFile::new(self.clone())))
        }
    }


    #[inline]
    fn offset(&self, offset: usize) -> usize {
        self.base.offset + offset
    }
}


impl<D> ShortDirEntryReadable for ShortDirEntry<D>
    where D: FatDeviceAccessible + Clone + BpbReadable
{
    #[inline]
    fn name_buff(&self) -> FatResult<[u8; 11]> {
        let mut buff = [0; 11];
        self.base.bpb.read(&mut buff, self.offset(0), 11)?;

        Ok(buff)
    }


    #[inline]
    fn first_cluster_no_hi(&self) -> FatResult<u16> {
        self.base.bpb.read_u16(self.offset(20))
    }


    #[inline]
    fn first_cluster_no_lo(&self) -> FatResult<u16> {
        self.base.bpb.read_u16(self.offset(26))
    }


    #[inline]
    fn file_size(&self) -> FatResult<u32> {
        self.base.bpb.read_u32(self.offset(28))
    }
}


impl<D> Debug for ShortDirEntry<D> where D: FatDeviceAccessible + Clone + BpbReadable {
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


#[cfg(test)]
mod tests {
    use crate::bpb::BpbFat32;
    use crate::dir::entry::Attribute;
    use crate::dir::entry::base::DirEntryReadable;
    use crate::dir::entry::short::ShortDirEntryReadable;
    use crate::test::file_device;

    #[test]
    fn it_volume_label_name() {
        let mut root = BpbFat32::new(file_device())
            .root_dir()
            .unwrap();

        let volume_label = root
            .next()
            .unwrap()
            .into_regular_file()
            .unwrap()
            .name()
            .unwrap();

        assert_eq!(volume_label.to_str().unwrap(), "MIKAN OS")
    }


    #[test]
    fn it_hello_txt_file_name() {
        let file_name = BpbFat32::new(file_device())
            .root_dir()
            .unwrap()
            .find("HELLO.TXT")
            .unwrap()
            .name()
            .unwrap();

        assert_eq!(file_name.to_str().unwrap(), "HELLO.TXT")
    }


    #[test]
    fn it_volume_label_attribute() {
        let volume_label = BpbFat32::new(file_device())
            .root_dir()
            .unwrap()
            .next()
            .unwrap()
            .into_regular_file()
            .unwrap();

        assert_eq!(volume_label.attribute(), Ok(Attribute::VolumeLabel));
    }


    #[test]
    fn it_dir_fist_cluster_is_zero_if_volume_label() {
        let mut root = BpbFat32::new(file_device())
            .root_dir()
            .unwrap();

        assert_eq!(root.next().unwrap().into_regular_file().unwrap().first_cluster_no(), Ok(0x00));
    }
}