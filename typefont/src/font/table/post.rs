use super::super::{Fixed, Version16Dot16};
use crate::font::FWord;
use crate::impl_named;

pub enum Post {
    V1(PostV1),
    V2(PostV2),
    #[deprecated]
    #[allow(deprecated)]
    V2_5(PostV2_5),
    V3(PostV3),
}

impl_named!(Post, "post");

pub struct PostHeader {
    version: Version16Dot16,
    italic_angle: Fixed,
    underline_position: FWord,
    underline_thickness: FWord,
    is_fixed_pitch: u32,
    min_mem_type42: u32,
    max_mem_type42: u32,
    min_mem_type1: u32,
    max_mem_type1: u32,
}

pub struct PostV1 {
    // 展开
    header: PostHeader,
}

pub struct PostV2 {
    // 展开
    header: PostHeader,
    num_glyphs: u16,
    glyph_name_index: Vec<u16>,
    // u8
    string_data: String,
}

#[deprecated]
pub struct PostV2_5 {
    // 展开
    header: PostHeader,
    num_glyphs: u16,
    offset: Vec<i8>,
}

pub struct PostV3 {
    // 展开
    header: PostHeader,
}
