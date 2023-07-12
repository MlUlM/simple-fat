use alloc::ffi::CString;
use alloc::vec::Vec;

use crate::error::{FatError, FatResult};

pub mod buffer;

pub trait CommonBootSectorReadable {
    fn oem_name_buff(&self) -> Vec<u8>;


    fn bytes_per_sector(&self) -> u16;


    fn sectors_per_cluster(&self) -> u8;


    fn reserved_sectors(&self) -> u16;


    fn total_sector16(&self) -> u16;


    fn total_sector32(&self) -> u32;


    fn num_fats(&self) -> u8;


    fn oem_name(&self) -> CString {
        unsafe { CString::from_vec_unchecked(self.oem_name_buff()) }
    }


    fn checked_sectors_per_cluster(&self) -> FatResult<u8> {
        match self.sectors_per_cluster() {
            valid @ (1 | 2 | 4 | 8 | 16 | 32 | 64 | 128) => Ok(valid),
            invalid => Err(FatError::InvalidSecPerClus(invalid))
        }
    }


    fn data_region_offset_fat32(&self) -> usize {
        let bytes_per_sector = self.bytes_per_sector() as usize;
        let reserve_bytes = self.reserved_sectors() as usize * bytes_per_sector;
        let fat_bytes = self.num_fats() as usize * self.total_sector32() as usize * bytes_per_sector;

        reserve_bytes + fat_bytes
    }
}




