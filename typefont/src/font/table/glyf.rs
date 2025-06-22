use crate::impl_named;

pub struct Glyf {
    header: GlyfHeader,
    glyph_description: GlyfGlyphDescription,
}

pub struct GlyfHeader {
    number_of_contours: i16,
    x_min: i16,
    y_min: i16,
    x_max: i16,
    y_max: i16,
}

impl_named!(Glyf, "glyf");

pub enum GlyfGlyphDescription {
    Format1(SimpleGlyph),
    Format2(CompositeGlyph),
}

pub struct SimpleGlyph {
    end_pts_of_contours: Vec<u16>,
    instruction_length: u16,
    instructions: Vec<u8>,
    flags: Vec<u8>,
    /// u8 or i16
    x_coordinates_data: Vec<u8>,
    /// u8 or i16
    y_coordinates_data: Vec<u8>,
}

pub struct CompositeGlyph {
    flags: u16,
    glyph_index: u16,
    /// u8 or i8 or u16 or i16
    argument1_data: Vec<u8>,
    /// u8 or i8 or u16 or i16
    argument2_data: Vec<u8>,
    data: Vec<u8>,
}
