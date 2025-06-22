use crate::io::error::IOError;

pub trait ReadFrom<R> {
    fn read_from(reader: &mut R) -> Result<Self, IOError> where Self: Sized;
}

pub trait WriteTo<W> {
    fn write_to(&self, writer: &mut W) -> Result<usize, IOError>;
}
