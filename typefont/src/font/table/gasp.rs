use crate::impl_tag;

pub struct Gasp {
    head: GaspHeader,
}

pub struct GaspHeader {
    /// 0 或 1 （新字体）
    version: u16,
    num_ranges: u16,
    gasp_ranges: Vec<GaspRange>,
}

impl_tag!(Gasp, "gasp");

pub struct GaspRange {
    range_max_ppem: u16,
    range_gasp_behavior: u16,
}
