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


    fn bytes_per_sector(&self) -> u16 {
        buff_read_u16(self.buff, 11)
    }


    fn sectors_per_cluster(&self) -> u8 {
        self.buff[13]
    }


    fn reserved_sectors(&self) -> u16 {
        buff_read_u16(self.buff, 14)
    }


    fn total_sector16(&self) -> u16 {
        buff_read_u16(self.buff, 19)
    }


    fn total_sector32(&self) -> u32 {
        buff_read_u32(self.buff, 32)
    }


    fn num_fats(&self) -> u8 {
        self.buff[16]
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
    fn it_sectors_per_cluster() {
        let buff = read_fat32_buffer();
        let common = CommonBootSectorBuffer::new(buff.as_slice());
        assert_eq!(common.sectors_per_cluster(), 2);
    }


    #[test]
    fn it_reserved_sectors() {
        let buff = read_fat32_buffer();
        let common = CommonBootSectorBuffer::new(buff.as_slice());
        assert_eq!(common.reserved_sectors(), 0x20);
    }


    #[test]
    fn it_total_sectors16_is_zero_if_fat32() {
        let buff = read_fat32_buffer().into_boxed_slice();
        let common = CommonBootSectorBuffer::new(&buff);
        assert_eq!(common.total_sector16(), 0);
    }


    #[test]
    fn it_total_sectors32_is_non_zero_if_fat32() {
        let buff = read_fat32_buffer();
        let common = CommonBootSectorBuffer::new(buff.as_slice());

        assert_eq!(common.total_sector32(), 4 << 16);
    }


    #[test]
    fn it_num_fats() {
        let buff = read_fat32_buffer();
        let common = CommonBootSectorBuffer::new(buff.as_slice());
        assert_eq!(common.num_fats(), 2);
    }
}
