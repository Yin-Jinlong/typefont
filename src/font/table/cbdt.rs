use crate::font::table::Table;
use crate::impl_table;

pub struct CBDT {
    header: CbdtHeader,
    /// CBDT 表其余部分为位图数据。数据可采用三种可能的格式呈现，
    /// 这些格式由 CBLC 表中的信息进行指示。
    /// 部分格式包含指标信息及图像数据，
    /// 其他格式仅包含图像数据。
    /// 这些子表无需长字对齐，仅需字节对齐即可。
    bmp_data: Vec<u8>,
}

impl_table!(CBDT, "CBDT");

pub struct CbdtHeader {
    /// 3
    major_version: u16,
    /// 0
    minor_version: u16,
}

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
