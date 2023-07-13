// use alloc::boxed::Box;
//
// use crate::dir::long::LongDirEntryReadable;
//
// pub struct LongDirEntryBuffer {
//     buff: Box<[u8]>,
// }
//
//
// impl LongDirEntryBuffer {
//     #[inline]
//     pub const fn new(buff: Box<[u8]>) -> Self {
//         Self {
//             buff
//         }
//     }
// }
//
//
// impl LongDirEntryReadable for LongDirEntryBuffer {
//     fn attribute_raw(&self) -> u8 {
//         self.buff[11]
//     }
// }
//
//
// #[cfg(test)]
// mod tests {
//     use crate::raw::dir::long::buffer::LongDirEntryBuffer;
//     use crate::test::read_fat32_buffer;
//
//     #[test]
//     fn it_attribute() {
//         let buff = read_fat32_buffer();
//         let long = LongDirEntryBuffer::new(buff[0x102040..].to_vec().into_boxed_slice());
//         assert_eq!(long.attribute(), Attribute::LongName);
//     }
// }