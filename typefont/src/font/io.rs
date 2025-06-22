use crate::io::error::IOError;

pub trait ReadFrom<R, T> {
    fn read_from(reader: &mut R) -> Result<T, IOError>;
}

pub trait WriteTo<W> {
    fn write_to(&self, writer: &mut W) -> Result<usize, IOError>;
}
