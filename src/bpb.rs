mod common;
mod fat32;


#[inline]
pub(crate) fn buff_read_u16(buff: &[u8], index: usize) -> u16 {
    let b = &buff[index..=index + 1];
    ((b[1] as u16) << 8) | b[0] as u16
}


#[inline]
pub(crate) fn buff_read_u32(buff: &[u8], index: usize) -> u32 {
    let (_, buff, _) = unsafe { buff[index..=index + 3].align_to::<u32>() };
    buff[0]
}