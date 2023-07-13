use num_enum::TryFromPrimitive;

pub mod long;
pub mod short;


#[derive(Debug, Copy, Clone, TryFromPrimitive, Eq, PartialEq)]
#[repr(u8)]
pub enum Attribute {
    Readonly = 0x01,
    Hidden = 0x02,
    System = 0x04,
    VolumeLabel = 0x08,
    Dir = 0x10,
    Archive = 0x20,
    LongName = 0x0f,
}


