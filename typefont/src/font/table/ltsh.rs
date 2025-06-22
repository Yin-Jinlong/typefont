use crate::impl_tag;

pub struct LTSH {
    version: u16,
    num_glyphs: u16,
    y_pixels: Vec<u8>,
}

impl_tag!(LTSH, "LTSH");
