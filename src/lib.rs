#![cfg_attr(not(test), no_std)]


extern crate alloc;


pub mod bpb;
pub mod dir;
pub mod error;


#[cfg(test)]
pub(crate) fn read_fat32_buffer() -> alloc::vec::Vec<u8> {
    std::fs::read("./fat_disk_32").unwrap()
}


#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use std::fs::{self};

    use super::*;

    #[test]
    fn it_works() {
        let buff: Vec<u8> = fs::read("./fat_disk")
            .unwrap()
            .into_iter()
            .take(1024 * 16)
            .collect();
        println!("{:?}", buff);
    }
}
