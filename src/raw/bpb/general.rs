#[cfg(feature = "alloc")]
use alloc::ffi::CString;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::error::{FatError, FatResult};

pub mod buffer;

pub trait GeneralBootSectorReadable {
    fn oem_name_buff(&self) -> FatResult<[u8; 8]>;


    fn bytes_per_sector(&self) -> FatResult<u16>;


    fn sectors_per_cluster(&self) -> FatResult<u8>;


    fn reserved_sectors(&self) -> FatResult<u16>;


    fn total_sector16(&self) -> FatResult<u16>;


    fn total_sector32(&self) -> FatResult<u32>;


    fn num_fats(&self) -> FatResult<u8>;


    #[cfg(feature = "alloc")]
    fn oem_name(&self) -> FatResult<CString> {
        Ok(unsafe { CString::from_vec_unchecked(Vec::from(self.oem_name_buff()?)) })
    }


    fn checked_sectors_per_cluster(&self) -> FatResult<u8> {
        match self.sectors_per_cluster()? {
            valid @ (1 | 2 | 4 | 8 | 16 | 32 | 64 | 128) => Ok(valid),
            invalid => Err(FatError::InvalidSecPerClus(invalid))
        }
    }
}





