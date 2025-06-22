use crate::impl_named;

pub struct LTSH {
    version: u16,
    num_glyphs: u16,
    y_pixels: Vec<u8>,
}

impl_named!(LTSH, "LTSH");
