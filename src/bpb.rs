use crate::bpb::common::buffer::CommonBootSectorBuffer;
use crate::bpb::common::CommonBootSectorReadable;
use crate::bpb::fat32::buffer::Fat32BootSectorBuffer;
use crate::bpb::fat32::Fat32BootSectorReadable;

mod common;
mod fat32;


pub struct BpbFat32Buffer<'buff> {
    common: CommonBootSectorBuffer<'buff>,
    fat32: Fat32BootSectorBuffer<'buff>,
}


impl<'buff> BpbFat32Buffer<'buff> {
    pub const fn new(buff: &'buff [u8]) -> BpbFat32Buffer<'buff> {
        Self {
            common: CommonBootSectorBuffer::new(buff),
            fat32: Fat32BootSectorBuffer::new(buff),
        }
    }


    pub fn data_region_offset_fat32(&self) -> usize {
        let bytes_per_sector = self.common.bytes_per_sector() as usize;
        let reserve_bytes = self.common.reserved_sectors() as usize * bytes_per_sector;
        let fat_bytes = self.common.num_fats() as usize * self.fat32.sectors_per_fat() as usize * bytes_per_sector;

        reserve_bytes + fat_bytes
    }
}


#[inline]
pub(crate) fn buff_read_u16(buff: &[u8], index: usize) -> u16 {
    // let (_, buff, _) = unsafe { buff[index..=index + 1].align_to::<u16>() };
    // buff[0]
    (buff[index + 1] as u16) << 8 | buff[index] as u16
}


#[inline]
pub(crate) fn buff_read_u32(buff: &[u8], index: usize) -> u32 {
    let shift = |i| (buff[index + i] as u32) << (i * 8);
    shift(3) | shift(2) | shift(1) | shift(0)
}


#[cfg(test)]
mod tests {
    use crate::bpb::BpbFat32Buffer;
    use crate::read_fat32_buffer;

    #[test]
    fn it_data_region_offset_fat32() {
        let buff = read_fat32_buffer();
        let bpb = BpbFat32Buffer::new(buff.as_slice());

        assert_eq!(bpb.data_region_offset_fat32(), 0x102000);
    }
}
