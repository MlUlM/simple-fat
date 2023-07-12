use alloc::ffi::CString;
use alloc::vec::Vec;

pub mod buffer;

pub trait CommonBootSectorReadable {
    fn oem_name_buff(&self) -> Vec<u8>;


    fn bytes_per_sec(&self) -> u16;


    fn total_sector16(&self) -> u16;


    fn total_sector32(&self) -> u32;


    fn oem_name(&self) -> CString {
        unsafe { CString::from_vec_unchecked(self.oem_name_buff()) }
    }
}




