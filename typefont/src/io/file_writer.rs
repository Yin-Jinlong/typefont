use crate::io::writer::Writer;
use std::fs::File;
use std::io::Write;

pub struct FileWriter {
    file: File,
    pos: usize,
}

impl FileWriter {
    pub fn open(path: &str) -> Result<FileWriter, std::io::Error> {
        let file = File::open(path)?;
        Ok(Self { file, pos: 0 })
    }
}

impl Writer for FileWriter {
    fn written(&self) -> usize {
        self.pos
    }

    fn write_bytes(&mut self, bytes: &[u8], off: usize, len: usize) -> crate::io::writer::Result {
        match self.file.write(&bytes[off..off + len]) {
            Ok(size) => {
                self.pos += size;
                Ok(len)
            }
            Err(e) => Err(crate::io::error::IOError::UnableOperate(e.to_string())),
        }
    }
}
