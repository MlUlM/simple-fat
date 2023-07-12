use alloc::vec::Vec;

use crate::bpb::{buff_read_u16, buff_read_u32};
use crate::bpb::common::CommonBootSectorReadable;

pub struct CommonBootSectorBuffer<'buff> {
    buff: &'buff [u8],
}


impl<'buff> CommonBootSectorBuffer<'buff> {
    pub const fn new(buff: &'buff [u8]) -> CommonBootSectorBuffer<'buff> {
        Self {
            buff
        }
    }
}


impl<'buff> CommonBootSectorReadable for CommonBootSectorBuffer<'buff> {
    fn oem_name_buff(&self) -> Vec<u8> {
        self.buff
            .iter()
            .skip(3)
            .take(8)
            .copied()
            .collect()
    }


    fn bytes_per_sec(&self) -> u16 {
        buff_read_u16(self.buff, 11)
    }


    fn total_sector16(&self) -> u16 {
        buff_read_u16(self.buff, 19)
    }


    fn total_sector32(&self) -> u32 {
        buff_read_u32(self.buff, 32)
    }
}


#[cfg(test)]
mod tests {
    use crate::bpb::common::buffer::CommonBootSectorBuffer;
    use crate::bpb::common::CommonBootSectorReadable;
    use crate::read_fat32_buffer;

    #[test]
    fn it_oem_name() {
        let buff = read_fat32_buffer();
        let common = CommonBootSectorBuffer::new(buff.as_slice());
        assert_eq!(common.oem_name().to_str().unwrap(), "mkfs.fat");
    }


    #[test]
    fn it_total_sectors16_is_zero_if_fat32() {
        let buff = read_fat32_buffer();
        let common = CommonBootSectorBuffer::new(buff.as_slice());
        assert_eq!(common.total_sector16(), 0);
    }


    #[test]
    fn it_total_sectors32_is_non_zero_if_fat32() {
        let buff = read_fat32_buffer();
        let common = CommonBootSectorBuffer::new(buff.as_slice());
        assert_ne!(common.total_sector32(), 0);
    }




}
