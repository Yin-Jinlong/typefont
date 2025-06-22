use crate::impl_named;

pub struct VORG {
    /// 1
    major_version: u16,
    /// 0
    minor_version: u16,
    default_vert_origin_y: i16,
    num_vert_origin_ymetrics: u16,
    vert_origin_ymetrics: Vec<VertOriginYMetrics>,
}

impl_named!(VORG, "VORG");

pub struct VertOriginYMetrics {
    glyph_index: u16,
    vert_origin_y: i16,
}
