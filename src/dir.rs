use alloc::ffi::CString;
use alloc::vec::Vec;

use num_enum::TryFromPrimitive;

pub(crate) mod buffer;
pub mod long;

#[derive(Debug, Copy, Clone, TryFromPrimitive, Eq, PartialEq)]
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


pub trait DirEntryReadable {
    fn name_buff(&self) -> Vec<u8>;


    fn attribute_raw(&self) -> u8;


    fn first_cluster_no_hi(&self) -> u16;


    fn first_cluster_no_lo(&self) -> u16;


    fn name(&self) -> CString {
        unsafe {
            CString::from_vec_unchecked(self
                .name_buff()
                .into_iter()
                .skip(1)
                .map(|b| if b == 0x20 {
                    0
                } else {
                    b
                })
                .collect())
        }
    }


    fn first_cluster_no(&self) -> usize {
        ((self.first_cluster_no_hi() as usize) << 16) | self.first_cluster_no_lo() as usize
    }


    fn file_size(&self) -> u32;


    fn attribute(&self) -> Attribute {
        Attribute::try_from_primitive(self.attribute_raw()).unwrap()
    }
}

