use thiserror_no_std::Error;

pub type FatResult<T = ()> = Result<T, FatError>;

#[derive(Error, Debug)]
pub enum FatError {
    #[error("'sectors per clusters' must be one value of 1,2,4,8,16,32,64 or 128, but was {0}")]
    InvalidSecPerClus(u8),

    #[error("invalid 'DIR_Attr' = {0}")]
    InvalidAttribute(u8)
}



