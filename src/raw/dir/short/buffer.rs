// use alloc::vec::Vec;
// use core::fmt::{Debug, Formatter};
//
// use crate::bpb::{buff_read_u16, buff_read_u32};
// use crate::Buffer;
// use crate::dir::short::ShortDirEntryReadable;
//
// pub struct ShortDirEntryBuffer {
//     buff: Box<[u8]>,
// }
//
//
// impl ShortDirEntryBuffer {
//     pub const fn new(buff: Box<[u8]>) -> Self {
//         Self {
//             buff
//         }
//     }
// }
//
//
// impl ShortDirEntryReadable for ShortDirEntryBuffer {
//     fn name_buff(&self) -> Vec<u8> {
//         self
//             .buff
//             .iter()
//             .take(8)
//             .copied()
//             .collect()
//     }
//
//
//     fn attribute_raw(&self) -> u8 {
//         self.buff.at(11)
//     }
//
//
//     fn first_cluster_no_hi(&self) -> u16 {
//         buff_read_u16(&self.buff, 20)
//     }
//
//
//     fn first_cluster_no_lo(&self) -> u16 {
//         buff_read_u16(&self.buff, 26)
//     }
//
//
//     fn file_size(&self) -> u32 {
//         buff_read_u32(&self.buff, 28)
//     }
// }
//
//
// impl Debug for ShortDirEntryBuffer {
//     fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
//         f
//             .debug_struct("DirEntry")
//             .field("name_buff", &self.name())
//             .field("first_cluster_no", &self.first_cluster_no())
//             .field("file_size", &self.file_size())
//             .finish()
//     }
// }
//
//
// #[cfg(test)]
// mod tests {
//     use alloc::ffi::CString;
//
//     use crate::raw::dir::short::buffer::ShortDirEntryBuffer;
//     use crate::test::read_fat32_buffer;
//
//     #[test]
//     fn it_dir_name() {
//         let buff = read_fat32_buffer();
//         let dir = ShortDirEntryBuffer::new(buff[0x102000..].to_vec().into_boxed_slice());
//
//         println!("{:?}", unsafe { CString::from_vec_unchecked(dir.name_buff()) })
//     }
//
//
//     #[test]
//     fn it_dir_attribute() {
//         let buff = read_fat32_buffer();
//         let dir = ShortDirEntryBuffer::new(buff[0x102000..].to_vec().into_boxed_slice());
//         assert_eq!(dir.attribute(), Attribute::VolumeLabel);
//     }
//
//
//     #[test]
//     fn it_dir_attribute_lfn_entry() {
//         let buff = read_fat32_buffer();
//         let dir = ShortDirEntryBuffer::new(buff[0x102020..].to_vec().into_boxed_slice());
//         assert_eq!(dir.attribute(), Attribute::LongName);
//     }
//
//
//     #[test]
//     fn it_dir_fist_cluster_is_zero_if_volume_label() {
//         let buff = read_fat32_buffer();
//         let dir = ShortDirEntryBuffer::new(buff[0x102000..].to_vec().into_boxed_slice());
//         assert_eq!(dir.first_cluster_no(), 0);
//     }
//
//
//     #[test]
//     fn it_dir_fist_cluster_no() {
//         let buff = read_fat32_buffer();
//         let dir = ShortDirEntryBuffer::new(buff[0x102040..].to_vec().into_boxed_slice());
//
//         assert_eq!(dir.first_cluster_no(), 7);
//     }
//
//
//     #[test]
//     fn it_file_size_is_zero_if_volume_label() {
//         let buff = read_fat32_buffer();
//         let dir = ShortDirEntryBuffer::new(buff[0x102000..].to_vec().into_boxed_slice());
//         assert_eq!(dir.file_size(), 0);
//     }
//
//
//     #[test]
//     fn it_file_size_if_text_file() {
//         let buff = read_fat32_buffer();
//         let dir = ShortDirEntryBuffer::new(buff[0x102040..].to_vec().into_boxed_slice());
//         assert_eq!(dir.file_size(), 6);
//     }
// }