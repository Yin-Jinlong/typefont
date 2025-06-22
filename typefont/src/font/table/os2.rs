use super::super::{FWord, Tag, UFWord};
use crate::impl_named;

pub enum Os2 {
    V5(Os2V5),
    V4(Os2V4),
    V3(Os2V3),
    V2(Os2V2),
    V1(Os2V1),
    V0(Os2V0),
}

impl_named!(Os2, "OS/2");

pub struct Os2V5 {
    // 继承
    base: Os2V4,
    us_lower_optical_point_size: u16,
    us_upper_optical_point_size: u16,
}

pub struct Os2V4 {
    // 继承
    base: Os2V1,
    sx_height: FWord,
    s_cap_height: FWord,
    us_default_char: u16,
    us_break_char: u16,
    us_max_context: u16,
}

type Os2V3 = Os2V4;
type Os2V2 = Os2V3;

pub struct Os2V1 {
    // 继承
    base: Os2V0,
    ul_code_page_range1: u32,
    ul_code_page_range2: u32,
}

pub struct Os2V0 {
    version: u16,
    x_avg_char_width: FWord,
    us_weight_class: u16,
    us_width_class: u16,
    fs_type: u16,
    y_subscript_x_size: FWord,
    y_subscript_y_size: FWord,
    y_subscript_x_offset: FWord,
    y_subscript_y_offset: FWord,
    y_superscript_x_size: FWord,
    y_superscript_y_size: FWord,
    y_superscript_x_offset: FWord,
    y_superscript_y_offset: FWord,
    y_strikeout_size: FWord,
    y_strikeout_position: FWord,
    s_family_class: i16,
    panose: [u8; 10],
    ul_unicode_range_1: u32,
    ul_unicode_range_2: u32,
    ul_unicode_range_3: u32,
    ul_unicode_range_4: u32,
    ach_vend_id: Tag,
    fs_selection: u16,
    us_first_char_index: u16,
    us_last_char_index: u16,
    s_typo_ascender: FWord,
    s_typo_descender: FWord,
    s_typo_line_gap: FWord,
    us_win_ascent: UFWord,
    us_win_descent: UFWord,
}
