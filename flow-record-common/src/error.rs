use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("expected an ext value")]
    ExpectedExtValue,

    #[error("invalid ext type id: {0}")]
    InvalidExtTypeId(i8),

    #[error(transparent)]
    Decode(#[from] rmpv::decode::Error),

    #[error(transparent)]
    Encode(#[from] rmpv::encode::Error),

    #[error(transparent)]
    BinRW(#[from] binrw::Error)
}