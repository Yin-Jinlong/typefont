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
pub mod hvar;
pub mod jstf;
pub mod kern;
pub mod loca;
pub mod ltsh;
pub mod math;
pub mod maxp;
pub mod merg;
pub mod meta;
pub mod mvar;
pub mod name;
pub mod os2;
pub mod pclt;
pub mod post;
pub mod prep;
pub mod sbix;
pub mod stat;
pub mod svg;
pub mod var;
pub mod vdmx;

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
