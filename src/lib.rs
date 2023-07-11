#![cfg_attr(not(test), no_std)]

mod bpb;

extern crate alloc;

pub fn add(left: usize, right: usize) -> usize {
    left + right
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
