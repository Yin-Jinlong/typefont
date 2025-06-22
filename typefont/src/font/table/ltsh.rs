use super::Table;
use crate::impl_table;

pub struct LTSH {
    version: u16,
    num_glyphs: u16,
    y_pixels:Vec<u8>
}

impl_table!(LTSH, "LTSH");
