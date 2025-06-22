use crate::io::reader::Reader;
use std::cmp::min;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub struct FileReader {
    file: File,
    pos: usize,
    mark: usize,
    metadata: std::fs::Metadata,
}

impl FileReader {
    pub fn open(path: &str) -> Result<FileReader, std::io::Error> {
        let file = File::open(path)?;
        let metadata = file.metadata()?;
        Ok(Self {
            file,
            pos: 0,
            mark: 0,
            metadata,
        })
    }
}

impl Reader for FileReader {
    fn position(&self) -> usize {
        self.pos
    }

    fn size(&self) -> usize {
        self.metadata.len() as usize
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
        match self.file.seek(SeekFrom::Start(at as u64)) {
            Ok(_) => {
                self.pos = at;
                Ok(())
            }

            Err(e) => Err(crate::io::error::IOError::UnableOperate(e.to_string())),
        }
    }

    fn read_bytes(&mut self, len: usize) -> crate::io::reader::Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(len);
        match self.file.read(buf.as_mut_slice()) {
            Ok(_) => {
                self.pos += len;
                Ok(buf)
            }
            Err(e) => Err(crate::io::error::IOError::UnableOperate(e.to_string())),
        }
    }
}
