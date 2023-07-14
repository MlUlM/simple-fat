use alloc::vec;
use alloc::vec::Vec;

use auto_delegate::Delegate;

use crate::bpb::BpbReadable;
use crate::dir::entry::short::{ShortDirEntry, ShortDirEntryReadable};
use crate::error::{FatError, FatResult};
use crate::FatDeviceAccessible;

#[derive(Delegate)]
pub struct RegularFile<D> {
    #[to(ShortDirEntryReadable, DirEntryReadable)]
    pub entry: ShortDirEntry<D>,
}


impl<D> RegularFile<D>
    where D: FatDeviceAccessible + Clone + BpbReadable
{
    #[inline]
    pub fn new(entry: ShortDirEntry<D>) -> RegularFile<D> {
        Self {
            entry
        }
    }


    pub fn read_buff(&self, buff: &mut [u8]) -> FatResult {
        let offset = self.entry.base.bpb.data_cluster_offset_at(self.entry.first_cluster_no()? as usize)?;
        let file_size = self.entry.file_size_usize()?;
        if buff.len() < file_size {
            return Err(FatError::BufferToSmall(file_size, buff.len()));
        }

        self.entry.base.bpb.read(buff, offset, self.entry.file_size()? as usize)?;

        Ok(())
    }


    #[cfg(feature = "alloc")]
    pub fn read_boxed(&self) -> FatResult<Vec<u8>> {
        let size = self.entry.file_size()? as usize;
        let mut buff = vec![0; size];
        self.read_buff(&mut buff)?;

        Ok(buff)
    }
}


#[cfg(test)]
mod tests {
    use crate::bpb::BpbFat32;
    use crate::test::file_device;

    #[test]
    fn it_hello_txt_file_name() {
        let file = BpbFat32::new(file_device())
            .root_dir()
            .unwrap()
            .find("HELLO.TXT")
            .unwrap()
            .into_regular_file()
            .unwrap();

        assert_eq!(&file.read_boxed().unwrap(), &[0x79, 0x65, 0x73])
    }
}