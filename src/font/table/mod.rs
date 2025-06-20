pub mod avar;
pub mod base;
pub mod cbdt;
pub mod cblc;
pub mod cff;
pub mod cff2;
pub mod cmap;
pub mod colr;
pub mod cpal;
pub mod eblc;
pub mod head;
pub mod hhea;
pub mod hmtx;
pub mod maxp;
pub mod name;
pub mod os2;
pub mod post;

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
