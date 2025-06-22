use crate::io::error::IOError;
use bit_struct::{i24, i40, i48, i56, u24, u40, u48, u56};
use std::ops::ShlAssign;

#[cfg(feature = "writer")]
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

#[cfg(feature = "writer")]
macro_rules! write_fn_as {
    (
        $name:ident,$f:ident,$ast:ty,$($t:ident),*
    )=>{
        $(
        impl Write<$t> for dyn $name  {
            fn write(&mut self, v: $t) -> Result {
                self.$f(v as $ast)
            }
        }

        impl ShlAssign<$t> for dyn Writer{
            fn shl_assign(&mut self, rhs: $t) {
                self.$f(rhs as $ast).unwrap();
            }
        }

        )*
    };
}

#[cfg(feature = "writer")]
type Result = std::result::Result<usize, IOError>;

#[cfg(feature = "writer")]
pub trait Write<T: ?Sized> {
    fn write(&mut self, v: T) -> Result;
}

#[cfg(feature = "writer")]
pub trait Writer {
    /// 已写入的数据长度
    fn written(&self) -> usize;

    fn write_byte(&mut self, v: u8) -> Result;

    /// 读取指定长度的数据
    ///
    /// `len` 最大读取长度
    fn write_bytes(&mut self, bytes: &[u8], off: usize, len: usize) -> Result;
}

#[cfg(feature = "writer")]
impl Write<Vec<u8>> for dyn Writer {
    fn write(&mut self, v: Vec<u8>) -> Result {
        self.write_bytes(&v, 0, v.len())
    }
}

#[cfg(feature = "writer")]
impl Write<&Vec<u8>> for dyn Writer {
    fn write(&mut self, v: &Vec<u8>) -> Result {
        self.write_bytes(&v, 0, v.len())
    }
}

#[cfg(feature = "writer")]
impl Write<&[u8]> for dyn Writer {
    fn write(&mut self, v: &[u8]) -> Result {
        self.write_bytes(v, 0, v.len())
    }
}

#[cfg(feature = "writer")]
write_fn_as!(Writer, write_byte, u8, bool, i8, u8);

#[cfg(feature = "writer")]
write_fn!(
    Writer, i16, i24, i32, i40, i48, i56, i64, i128, u16, u24, u32, u40, u48, u56, u64, u128, f32,
    f64
);
