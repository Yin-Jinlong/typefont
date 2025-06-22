use crate::io::array_reader::ArrayReader;
use crate::io::error::IOError;
use bit_struct::{i24, i40, i48, i56, u24, u40, u48, u56};
use paste::paste;

macro_rules! read_fn {
    (
        $($t:ident($size:literal)),*
    )=>{
        $(
        paste! {
            fn [<read_ $t>](&mut self) -> Result<$t> {
                let res = self.read_bytes_expected($size)?;
                let list = res.as_slice();
                let bytes: [u8; $size] = match list.try_into() {
                    Ok(bytes) => bytes,
                    Err(_) => {
                        return Err(IOError::UnableCast);
                    }
                };
                Ok($t::from_be_bytes(bytes))
            }
            fn [<read_ $t _list>](&mut self, count: usize) -> Result<Vec<$t>> {
                let mut list=Vec::with_capacity(count);
                for _ in 0..count {
                    list.push(self.[<read_ $t>]()?);
                }
                Ok(list)
            }
        }
        )*
    };
}

pub type Result<R> = std::result::Result<R, IOError>;

pub trait Reader {
    /// 当前位置，相对于数据开头
    fn position(&self) -> usize;

    /// 剩余可读数据长度
    fn remaining(&self) -> usize {
        self.size() - self.position()
    }

    fn size(&self) -> usize;

    /// 标记当前位置，用于后续回退
    fn mark(&mut self) -> Result<()>;

    fn get_mark(&self) -> usize;

    /// 回退到上一次标记的位置
    fn reset(&mut self) -> Result<()>;

    fn can_read(&self, len: usize) -> bool {
        self.remaining() >= len
    }

    fn skip(&mut self, len: usize) -> Result<()> {
        self.seek(self.position() + len)
    }

    fn seek(&mut self, pos: usize) -> Result<()>;

    fn eof(&self) -> bool {
        self.remaining() == 0
    }

    fn read_bool(&mut self) -> Result<bool> {
        Ok(self.read_u8()? != 0)
    }

    read_fn!(
        i8(1),
        i16(2),
        i24(3),
        i32(4),
        i40(5),
        i48(6),
        i56(7),
        i64(8),
        i128(16),
        u8(1),
        u16(2),
        u24(3),
        u32(4),
        u40(5),
        u48(6),
        u56(7),
        u64(8),
        u128(16),
        f32(4),
        f64(8)
    );

    /// 读取指定长度的数据
    ///
    /// `len` 最大读取长度
    fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>>;
    /// 读取指定长度的数据，若读取到的数据长度不等于指定长度，则返回错误
    fn read_bytes_expected(&mut self, expected_len: usize) -> Result<Vec<u8>> {
        if !self.can_read(expected_len) {
            return Err(IOError::NoEnoughData);
        }

        let res = self.read_bytes(expected_len)?;
        if res.len() != expected_len {
            Err(IOError::UnexpectedEof)
        } else {
            Ok(res)
        }
    }

    /// 读取指定长度的子数据
    fn read_sub(&mut self, len: usize) -> Result<Box<dyn Reader>> {
        let data = self.read_bytes_expected(len)?;
        Ok(Box::new(ArrayReader::from(data)))
    }
}

pub type ReaderBoxed = Box<dyn Reader>;
