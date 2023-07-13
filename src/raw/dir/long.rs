use num_enum::TryFromPrimitive;

use crate::raw::dir::Attribute;

pub(crate) mod buffer;

pub trait LongDirEntryReadable {
    fn attribute_raw(&self) -> u8;


    fn attribute(&self) -> Attribute {
        Attribute::try_from_primitive(self.attribute_raw()).unwrap()
    }
}