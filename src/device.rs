use auto_delegate::delegate;

use crate::bpb::{buff_read_u16, buff_read_u32};
use crate::error::{FatDeviceError, FatResult};

#[delegate]
pub trait FatDeviceAccessible {
    fn read(&self, buff: &mut [u8], offset: usize, bytes: usize) -> Result<(), FatDeviceError>;


    fn write(&mut self, buff: &[u8], offset: usize) -> Result<(), FatDeviceError>;


    fn read_u8(&self, offset: usize) -> FatResult<u8> {
        let mut buff = [0; 1];
        self.read(&mut buff, offset, 1)?;

        Ok(buff[0])
    }


    fn read_u16(&self, offset: usize) -> FatResult<u16> {
        let mut buff = [0; 2];
        self.read(&mut buff, offset, 2)?;

        Ok(buff_read_u16(&buff, 0))
    }


    fn read_u32(&self, offset: usize) -> FatResult<u32> {
        let mut buff = [0; 4];
        self.read(&mut buff, offset, 4)?;

        Ok(buff_read_u32(&buff, 0))
    }
}
