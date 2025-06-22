use crate::io::error::IOError;
use bit_struct::{i24, i40, i48, i56, u24, u40, u48, u56};
use std::ops::ShlAssign;

macro_rules! write_fn {
    (
        $name:ident,$($t:ident),*
    )=>{
        $(
        impl Write<$t> for dyn $name  {
            fn write(&mut self, v: $t) -> Result {
                self.write(v.to_be_bytes().as_slice())
            }
        }

        impl ShlAssign<$t> for dyn Writer{
            fn shl_assign(&mut self, rhs: $t) {
                self.write(rhs.to_be_bytes().as_slice()).unwrap();
            }
        }

        )*
    };
}

pub type Result = std::result::Result<usize, IOError>;

pub trait Write<T: ?Sized> {
    fn write(&mut self, v: T) -> Result;
}

pub trait Writer {
    /// 已写入的数据长度
    fn written(&self) -> usize;

    /// 读取指定长度的数据
    ///
    /// `len` 最大读取长度
    fn write_bytes(&mut self, bytes: &[u8], off: usize, len: usize) -> Result;
}

impl Write<Vec<u8>> for dyn Writer {
    fn write(&mut self, v: Vec<u8>) -> Result {
        self.write_bytes(&v, 0, v.len())
    }
}

impl Write<&Vec<u8>> for dyn Writer {
    fn write(&mut self, v: &Vec<u8>) -> Result {
        self.write_bytes(&v, 0, v.len())
    }
}

impl Write<&[u8]> for dyn Writer {
    fn write(&mut self, v: &[u8]) -> Result {
        self.write_bytes(v, 0, v.len())
    }
}

impl Write<bool> for dyn Writer {
    fn write(&mut self, v: bool) -> Result {
        let bs = [if v { 1 } else { 0 }];
        self.write_bytes(&bs, 0, 1)
    }
}

write_fn!(
    Writer, i8, i16, i24, i32, i40, i48, i56, i64, i128, u8, u16, u24, u32, u40, u48, u56, u64,
    u128, f32, f64
);
