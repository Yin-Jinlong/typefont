use paste::paste;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

macro_rules! impl_op {
    ($Op:ident,$fun:ident,$t:ident) => {
        impl $Op for $t {
            type Output = $t;

            fn $fun(self, rhs: Self) -> Self::Output {
                Self {
                    value: Self::$fun(self.value, rhs.value),
                }
            }
        }
        paste! {
            impl [<$Op Assign>] for $t {

                fn [<$fun _assign>](&mut self, rhs: Self) {
                     self.value = Self::$fun(self.value, rhs.value);
                }
            }
        }
    };
}

macro_rules! num_fn {
    ($nt:ty,$nt2:ty) => {
        fn add(a: $nt, b: $nt) -> $nt {
            Self::trunc(a as $nt2 + b as $nt2)
        }

        fn sub(a: $nt, b: $nt) -> $nt {
            Self::trunc(a as $nt2 - b as $nt2)
        }
    };
}

macro_rules! num_op {
    ($t:ident) => {
        impl_op!(Add, add, $t);
        impl_op!(Sub, sub, $t);
        impl_op!(Mul, mul, $t);
        impl_op!(Div, div, $t);

        impl PartialEq<Self> for $t {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        }

        impl PartialOrd<Self> for $t {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }
    };
}

pub struct U24 {
    value: u32,
}

impl U24 {
    pub const MAX: U24 = U24 { value: 0xff_ffff };
    pub const MIN: U24 = U24 { value: 0 };

    pub fn new() -> Self {
        Self { value: 0 }
    }

    fn trunc(v: u32) -> u32 {
        if v > 0xff_ffff { 0xff_ffff } else { v }
    }

    num_fn!(u32, u32);

    fn mul(a: u32, b: u32) -> u32 {
        Self::trunc(a * b)
    }

    fn div(a: u32, b: u32) -> u32 {
        Self::trunc(a / b)
    }
}

impl Display for U24 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value.to_string())
    }
}

impl Debug for U24 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value.to_string())
    }
}

impl From<u8> for U24 {
    fn from(value: u8) -> Self {
        Self {
            value: value as u32,
        }
    }
}

impl From<u16> for U24 {
    fn from(value: u16) -> Self {
        Self {
            value: value as u32,
        }
    }
}

impl From<u32> for U24 {
    fn from(value: u32) -> Self {
        Self {
            value: if value > 0xff_ffff { 0xff_ffff } else { value },
        }
    }
}

num_op!(U24);

/// 16 位有符号固定数，具有低 14 位小数 （2.14）。
///
/// F2DOT14 格式由一个有符号的 2 补码整数和一个无符号的小数组成。
/// 要计算实际值，请取整数并添加分数。
///
/// | 10 | 16 | 整 | 小 |
/// |:--|:-:|:-:|--:|
/// |  `1.999939` | `0x7fff` |  `1` | `16383/16384` |
/// |    `1.75`   | `0x7000` |  `1` | `12288/16384` |
/// |  `0.000061` | `0x0001` |  `0` |     `1/16384` |
/// | `-0.000061` | `0xffff` | `-1` | `16383/16384` |
/// | `-2.0`      | `0x8000` | `-2` |     `0/16384` |
///
/// author: YJL
///
pub struct F2D14 {
    value: i16,
}

fn complement(v: i16) -> i16 {
    if v > 0 { v } else { !v + 1 }
}

impl F2D14 {
    pub const MAX: F2D14 = F2D14 { value: 0x7fff };
    pub const MIN: F2D14 = F2D14 {
        value: 0x8000u16 as i16,
    };

    pub fn new() -> Self {
        Self { value: 0 }
    }

    fn trunc(v: i32) -> i16 {
        if v > i16::MAX as i32 {
            i16::MAX
        } else if v < i16::MIN as i32 {
            i16::MIN
        } else {
            v as i16
        }
    }

    fn mul(a: i16, b: i16) -> i16 {
        Self::trunc((a as i32 * b as i32) >> 14)
    }

    fn div(a: i16, b: i16) -> i16 {
        Self::trunc(((a as i32) << 14) / b as i32)
    }

    num_fn!(i16, i32);

    pub fn to_f32(&self) -> f32 {
        let mut int = complement(self.value >> 14) as f32;
        let fraction = (self.value & 0x3fff) as f32;
        if self.value < 0 {
            int = -int;
        };
        int + fraction / 16384f32
    }

    pub fn to_fraction_str(&self) -> String {
        let mut int = complement(self.value >> 14) as f32;
        let fraction = (self.value & 0x3fff) as f32;
        if self.value < 0 {
            int = -int;
        };
        format!("{}+{}/16384", int, fraction)
    }
}

impl Debug for F2D14 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_fraction_str())
    }
}

impl Display for F2D14 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_f32().to_string())
    }
}

impl From<f32> for F2D14 {
    fn from(value: f32) -> Self {
        const SCALE: f32 = 16384.0; // 2^14
        const MAX_Q2_14: f32 = 1.999969482; // 最大正数
        const MIN_Q2_14: f32 = -2.0; // 最小负数

        // 检查输入是否在 Q2.14 范围内
        let clamped_value = if value > MAX_Q2_14 {
            MAX_Q2_14
        } else if value < MIN_Q2_14 {
            MIN_Q2_14
        } else {
            value
        };

        // 缩放并四舍五入
        let scaled = clamped_value * SCALE;
        let rounded = scaled.round();

        // 转换为 i16 并处理溢出
        Self {
            value: if rounded > i16::MAX as f32 {
                i16::MAX
            } else if rounded < i16::MIN as f32 {
                i16::MIN
            } else {
                rounded as i16
            },
        }
    }
}

num_op!(F2D14);
