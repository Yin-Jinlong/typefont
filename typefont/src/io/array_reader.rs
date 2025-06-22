use crate::io::error::IOError;
use crate::io::reader::Reader;
use std::cmp::min;

pub struct ArrayReader {
    data: Vec<u8>,
    pos: usize,
    mark: usize,
}

impl ArrayReader {
    pub fn new(data: &[u8]) -> Result<ArrayReader, std::io::Error> {
        Ok(Self {
            data: data.to_vec(),
            pos: 0,
            mark: 0,
        })
    }
}

impl From<Vec<u8>> for ArrayReader {
    fn from(data: Vec<u8>) -> Self {
        Self {
            data,
            pos: 0,
            mark: 0,
        }
    }
}

impl Reader for ArrayReader {
    fn position(&self) -> usize {
        self.pos
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn mark(&mut self) -> crate::io::reader::Result<()> {
        self.mark = self.pos;
        Ok(())
    }

    fn get_mark(&self) -> usize {
        self.mark
    }

    fn reset(&mut self) -> crate::io::reader::Result<()> {
        self.seek(self.mark)
    }

    fn seek(&mut self, pos: usize) -> crate::io::reader::Result<()> {
        let at = min(self.size(), pos);
        self.pos = at;
        Ok(())
    }

    fn read_bytes(&mut self, len: usize) -> crate::io::reader::Result<Vec<u8>> {
        if self.eof() {
            return Err(IOError::UnexpectedEof);
        }
        let read_len = min(len, self.remaining());
        let r = self.data[self.pos..self.pos + read_len].to_vec();
        self.pos += read_len;
        Ok(r)
    }
}
