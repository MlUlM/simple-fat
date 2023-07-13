#![cfg_attr(not(test), no_std)]


extern crate alloc;


use crate::error::{FatDeviceError, FatResult};
use crate::raw::bpb::{buff_read_u16, buff_read_u32};

pub mod error;

pub mod raw;


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


pub struct Fat<D> {
    device: D,
}


impl<D> Fat<D> where D: FatDeviceAccessible + Clone {
    pub fn new(device: D) -> Fat<D> {
        Self {
            device
        }
    }
}


#[cfg(test)]
pub mod test {
    use alloc::boxed::Box;

    use crate::{Fat, FatDeviceAccessible};
    use crate::error::FatDeviceError;

    #[derive(Clone, Debug)]
    pub struct FileDevice;

    impl FatDeviceAccessible for FileDevice {
        fn read(&self, buff: &mut [u8], offset: usize, bytes: usize) -> Result<(), FatDeviceError> {
            let src = std::fs::read("./fat_disk_32").unwrap();
            buff.copy_from_slice(&src[offset..(offset + bytes)]);

            Ok(())
        }

        fn write(&mut self, buff: &[u8], offset: usize) -> Result<(), FatDeviceError> {
            let mut src = std::fs::read("./fat_disk_32").unwrap();
            src[offset..(offset + buff.len())].copy_from_slice(buff);

            std::fs::write("./fat_disk_32", src).unwrap();
            Ok(())
        }
    }

    #[inline]
    pub(crate) fn open_fat32_file() -> Fat<FileDevice> {
        Fat::new(FileDevice)
    }


    #[inline]
    pub(crate) fn file_device() -> FileDevice {
        FileDevice
    }


    #[inline]
    pub(crate) fn read_fat32_buffer() -> Box<[u8]> {
        std::fs::read("./fat_disk_32").unwrap().into_boxed_slice()
    }
}


#[cfg(test)]
mod tests {
    use crate::test::open_fat32_file;

    //
    // #[test]
    // fn it_exists_hello_txt() {
    //     let fat = open_fat32_file();
    //     let hello_txt = fat.open("/hello.txt");
    //     assert!(hello_txt.is_ok());
    // }

    //
    // #[test]
    // fn it_read_hello_txt_buffer() {
    //     let fat = open_fat32_file();
    //     let buff = fat
    //         .open_file("/hello.txt")
    //         .unwrap()
    //         .read_boxed();
    //
    //     assert_eq!(&[buff], &[0x68, 0x65, 0x6C, 0x6C, 0x6F]);
    // }
}
