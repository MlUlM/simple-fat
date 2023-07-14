use num_enum::TryFromPrimitiveError;
use thiserror_no_std::Error;

use crate::dir::entry::Attribute;

#[derive(Debug, PartialEq)]
pub enum FatDeviceError {
    StatusCode(isize)
}


pub type FatResult<T = ()> = Result<T, FatError>;


#[derive(Error, Debug, PartialEq)]
pub enum FatError {
    #[error("'sectors per clusters' must be one value of 1,2,4,8,16,32,64 or 128, but was {0}")]
    InvalidSecPerClus(u8),

    #[error("invalid 'DIR_Attr' = {0}")]
    InvalidAttribute(u8),

    #[error("{0:?}")]
    FailedDeviceAccess(FatDeviceError),

    #[error("Failed dir dir type")]
    InvalidDirEntryType,

    #[error("Expected buffer size is {0} but was {1}")]
    BufferToSmall(usize, usize),
}


impl From<FatDeviceError> for FatError {
    fn from(e: FatDeviceError) -> Self {
        Self::FailedDeviceAccess(e)
    }
}


impl From<TryFromPrimitiveError<Attribute>> for FatError {
    fn from(value: TryFromPrimitiveError<Attribute>) -> Self {
        Self::InvalidAttribute(value.number)
    }
}
