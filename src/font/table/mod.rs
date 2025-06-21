pub mod avar;
pub mod base;
pub mod bm;
pub mod cbdt;
pub mod cblc;
pub mod cff;
pub mod cff2;
pub mod cmap;
pub mod colr;
pub mod cpal;
pub mod cvar;
pub mod cvt;
pub mod dsig;
pub mod ebdt;
pub mod eblc;
pub mod ebsc;
pub mod fpgm;
pub mod fvar;
pub mod gasp;
pub mod gdef;
pub mod glyf;
pub mod glyph;
pub mod hdmx;
pub mod head;
pub mod hhea;
pub mod hmtx;
pub mod maxp;
pub mod name;
pub mod os2;
pub mod post;
pub mod var;

pub trait Table {
    fn name() -> String
    where
        Self: Sized;
}

#[macro_export]
macro_rules! impl_table {
    ($table:ty,$name:literal) => {
        impl Table for $table {
            fn name() -> String {
                String::from($name)
            }
        }
    };
}
