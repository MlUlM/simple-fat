mod common;
mod fat32;


#[inline]
pub(crate) fn buff_read_u16(buff: &[u8], index: usize) -> u16 {
    let (_, buff, _) = unsafe { buff[index..=index + 1].align_to::<u16>() };
    buff[0]
}


#[inline]
pub(crate) fn buff_read_u32(buff: &[u8], index: usize) -> u32 {
    let (_, buff, _) = unsafe { buff[index..=index + 3].align_to::<u32>() };
    buff[0]
}