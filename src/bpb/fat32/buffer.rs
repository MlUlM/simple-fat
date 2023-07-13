use crate::bpb::buff_read_u32;
use crate::bpb::fat32::Fat32BootSectorReadable;

pub struct Fat32BootSectorBuffer<'buff> {
    buff: &'buff [u8],
}


impl<'buff> Fat32BootSectorBuffer<'buff> {
    #[inline]
    pub const fn new(buff: &'buff [u8]) -> Fat32BootSectorBuffer<'buff> {
        Self {
            buff
        }
    }
}


impl<'buff> Fat32BootSectorReadable for Fat32BootSectorBuffer<'buff> {
    fn sectors_per_fat(&self) -> u32 {
        buff_read_u32(self.buff, 36)
    }


    fn root_cluster_no(&self) -> u32 {
        buff_read_u32(self.buff, 44)
    }
}


#[cfg(test)]
mod tests {
    use crate::bpb::fat32::buffer::Fat32BootSectorBuffer;
    use crate::bpb::fat32::Fat32BootSectorReadable;
    use crate::read_fat32_buffer;

    #[test]
    fn it_root_clusters() {
        let buff = read_fat32_buffer();
        let fat32 = Fat32BootSectorBuffer::new(buff.as_slice());
        assert_eq!(fat32.root_cluster_no(), 2);
    }
}
