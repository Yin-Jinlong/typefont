use super::ebdt::EbdtComponent;

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
    Format1(GlyphBitmapDataFormat1),
    Format2(GlyphBitmapDataFormat2),
    Format3(GlyphBitmapFormat3),
    Format4(GlyphBitmapDataFormat4),
    Format5(GlyphBitmapDataFormat5),
    Format6(GlyphBitmapDataFormat6),
    Format7(GlyphBitmapDataFormat7),
    Format8(GlyphBitmapDataFormat8),
    Format9(GlyphBitmapDataFormat9),
    Format17(GlyphBitmapDataFormat17),
    Format18(GlyphBitmapDataFormat18),
    Format19(GlyphBitmapDataFormat19),
}

/// small metrics, byte-aligned data
pub struct GlyphBitmapDataFormat1 {
    small_metrics: SmallGlyphMetrics,
    image_data: Vec<u8>,
}

/// small metrics, bit-aligned data
pub struct GlyphBitmapDataFormat2 {
    small_metrics: SmallGlyphMetrics,
    image_data: Vec<u8>,
}

/// obsolete
pub struct GlyphBitmapFormat3 {
    // Not supported yet
}

/// metrics in EBLC, compressed data
pub struct GlyphBitmapDataFormat4 {
    // Not supported yet
}

/// metrics in EBLC, bit-aligned image data only
pub struct GlyphBitmapDataFormat5 {
    image_data: Vec<u8>,
}

/// big metrics, byte-aligned data
pub struct GlyphBitmapDataFormat6 {
    big_metrics: BigGlyphMetrics,
    image_data: Vec<u8>,
}

/// big metrics, bit-aligned data
pub struct GlyphBitmapDataFormat7 {
    big_metrics: BigGlyphMetrics,
    image_data: Vec<u8>,
}

/// small metrics, component data
pub struct GlyphBitmapDataFormat8 {
    small_metrics: SmallGlyphMetrics,
    pad: u8,
    num_components: u16,
    components: Vec<EbdtComponent>,
}

/// big metrics, component data
pub struct GlyphBitmapDataFormat9 {
    big_metrics: BigGlyphMetrics,
    pad: u8,
    num_components: u16,
    components: Vec<EbdtComponent>,
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
