use num_enum::TryFromPrimitive;

use crate::error::FatResult;
use crate::raw::dir::Attribute;

pub(crate) mod buffer;


pub trait ShortDirEntryReadable {
    fn name_buff(&self) -> FatResult<[u8; 8]>;


    fn attribute_raw(&self) -> u8;


    fn first_cluster_no_hi(&self) -> u16;


    fn first_cluster_no_lo(&self) -> u16;

    //
    // fn name(&self) -> CString {
    //     unsafe {
    //         CString::from_vec_unchecked(self
    //             .name_buff()
    //             .into_iter()
    //             .skip(1)
    //             .map(|b| if b == 0x20 {
    //                 0
    //             } else {
    //                 b
    //             })
    //             .collect())
    //     }
    // }
    //

    fn first_cluster_no(&self) -> usize {
        ((self.first_cluster_no_hi() as usize) << 16) | self.first_cluster_no_lo() as usize
    }


    fn file_size(&self) -> u32;


    fn attribute(&self) -> Attribute {
        Attribute::try_from_primitive(self.attribute_raw()).unwrap()
    }
}
