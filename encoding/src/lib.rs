use extended_primitives::{Buffer, BufferError};
use std::fmt;

#[derive(Debug)]
pub enum DecodingError {
    //Overall wrapper type to enclose decoding errors.
    InvalidData(String),
    Buffer(BufferError),
    UnknownInventory,
}

impl From<BufferError> for DecodingError {
    fn from(e: BufferError) -> DecodingError {
        DecodingError::Buffer(e)
    }
}

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecodingError::InvalidData(ref e) => write!(f, "Invalid Data: {}", e),
            DecodingError::Buffer(ref e) => write!(f, "Buffer Error: {}", e),
            DecodingError::UnknownInventory => write!(f, "Unknown Inventory Type"),
        }
    }
}

pub trait Encodable {
    fn size(&self) -> usize;

    fn encode(&self) -> Buffer;
}

pub trait Decodable
where
    Self: Sized,
{
    type Err;
    fn decode(buffer: &mut Buffer) -> Result<Self, Self::Err>;
}
