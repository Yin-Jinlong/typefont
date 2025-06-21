pub struct BigGlyphMetrics {
    height: u8,
    width: u8,
    hori_bearing_x: i8,
    hori_bearing_y: i8,
    hori_advance: u8,
    vert_bearing_x: i8,
    vert_bearing_y: i8,
    vert_advance: u8,
}

pub struct SmallGlyphMetrics {
    height: u8,
    width: u8,
    bearing_x: i8,
    bearing_y: i8,
    advance: u8,
}

pub enum GlyphBitmapData {
    Format17(GlyphBitmapDataFormat17),
    Format18(GlyphBitmapDataFormat18),
    Format19(GlyphBitmapDataFormat19),
}

/// small metrics, PNG image data
pub struct GlyphBitmapDataFormat17 {
    glyph_metrics: SmallGlyphMetrics,
    data_len: u32,
    data: Vec<u8>,
}

/// big metrics, PNG image data
pub struct GlyphBitmapDataFormat18 {
    glyph_metrics: BigGlyphMetrics,
    data_len: u32,
    data: Vec<u8>,
}

/// metrics in CBLC table, PNG image data
pub struct GlyphBitmapDataFormat19 {
    data_len: u32,
    data: Vec<u8>,
}
