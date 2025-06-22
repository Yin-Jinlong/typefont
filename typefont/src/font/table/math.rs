use super::Table;
use crate::font::{FWord, Offset16, UFWord};
use crate::impl_table;

pub struct MATH {}

pub struct MATHeader {
    /// 1
    major_version: u16,
    /// 0
    minor_version: u16,
    math_constants_offset: u16,
    math_glyph_info_offset: u16,
    math_variants_offset: u16,
}

impl_table!(MATH, "MATH");

pub struct MathValueRecord {
    value: FWord,
    device_offset: Offset16,
}

pub struct MathConstants {
    script_percent_scale_down: i16,
    script_script_percent_scale_down: i16,
    delimited_sub_formula_min_height: UFWord,
    display_operator_min_height: UFWord,
    math_leading: MathValueRecord,
    axis_height: MathValueRecord,
    accent_base_height: MathValueRecord,
    flattened_accent_base_height: MathValueRecord,
    subscript_shift_down: MathValueRecord,
    subscript_top_max: MathValueRecord,
    subscript_baseline_drop_min: MathValueRecord,
    superscript_shift_up: MathValueRecord,
    superscript_shift_up_cramped: MathValueRecord,
    superscript_bottom_min: MathValueRecord,
    superscript_baseline_drop_max: MathValueRecord,
    sub_superscript_gap_min: MathValueRecord,
    superscript_bottom_max_with_subscript: MathValueRecord,
    space_after_script: MathValueRecord,
    upper_limit_gap_min: MathValueRecord,
    upper_limit_baseline_rise_min: MathValueRecord,
    lower_limit_gap_min: MathValueRecord,
    lower_limit_baseline_drop_min: MathValueRecord,
    stack_top_shift_up: MathValueRecord,
    stack_top_display_style_shift_up: MathValueRecord,
    stack_bottom_shift_down: MathValueRecord,
    stack_bottom_display_style_shift_down: MathValueRecord,
    stack_gap_min: MathValueRecord,
    stack_display_style_gap_min: MathValueRecord,
    stretch_stack_top_shift_up: MathValueRecord,
    stretch_stack_bottom_shift_down: MathValueRecord,
    stretch_stack_gap_above_min: MathValueRecord,
    stretch_stack_gap_below_min: MathValueRecord,
    fraction_numerator_shift_up: MathValueRecord,
    fraction_numerator_display_style_shift_up: MathValueRecord,
    fraction_denominator_shift_down: MathValueRecord,
    fraction_denominator_display_style_shift_down: MathValueRecord,
    fraction_numerator_gap_min: MathValueRecord,
    fraction_num_display_style_gap_min: MathValueRecord,
    fraction_rule_thickness: MathValueRecord,
    fraction_denominator_gap_min: MathValueRecord,
    fraction_denom_display_style_gap_min: MathValueRecord,
    skewed_fraction_horizontal_gap: MathValueRecord,
    skewed_fraction_vertical_gap: MathValueRecord,
    overbar_vertical_gap: MathValueRecord,
    overbar_rule_thickness: MathValueRecord,
    overbar_extra_ascender: MathValueRecord,
    underbar_vertical_gap: MathValueRecord,
    underbar_rule_thickness: MathValueRecord,
    underbar_extra_descender: MathValueRecord,
    radical_vertical_gap: MathValueRecord,
    radical_display_style_vertical_gap: MathValueRecord,
    radical_rule_thickness: MathValueRecord,
    radical_extra_ascender: MathValueRecord,
    radical_kern_before_degree: MathValueRecord,
    radical_kern_after_degree: MathValueRecord,
    radical_degree_bottom_raise_percent: i16,
}

pub struct MathGlyphInfo {
    math_italics_correction_info_offset: Offset16,
    math_top_accent_attachment_offset: Offset16,
    extended_shape_coverage_offset: Offset16,
    math_kern_info_offset: Offset16,
}

pub struct MathItalicsCorrectionInfo {
    italics_correction_coverage_offset: Offset16,
    italics_correction_count: i16,
    italics_correction: Vec<MathValueRecord>,
}

pub struct MathTopAccentAttachment {
    top_accent_coverage_offset: Offset16,
    top_accent_attachment_count: u16,
    top_accent_attachment: Vec<MathValueRecord>,
}

pub struct MathKernInfo {
    math_kern_coverage_offset: Offset16,
    math_kern_count: u16,
    math_kern_info_records: Vec<MathKernInfoRecord>,
}

pub struct MathKernInfoRecord {
    top_right_math_kern_offset: Offset16,
    top_left_math_kern_offset: Offset16,
    bottom_right_math_kern_offset: Offset16,
    bottom_left_math_kern_offset: Offset16,
}

pub struct MathKern {
    height_count: u16,
    correction_height: Vec<MathValueRecord>,
    kern_values: Vec<MathValueRecord>,
}

pub struct MathVariants {
    min_connector_overlap: UFWord,
    vert_glyph_coverage_offset: Offset16,
    horiz_glyph_coverage_offset: Offset16,
    vert_glyph_count: u16,
    horiz_glyph_count: u16,
    vert_glyph_construction_offsets: Vec<Offset16>,
    horiz_glyph_construction_offsets: Vec<Offset16>,
}

pub struct MathGlyphConstruction {
    glyph_assembly_offset: Offset16,
    variant_count: u16,
    math_glyph_variant_records: Vec<MathGlyphVariantRecord>,
}

pub struct MathGlyphVariantRecord {
    variant_glyph: u16,
    advance_measurement: UFWord,
}

pub struct GlyphAssembly {
    italics_correction: MathValueRecord,
    part_count: u16,
    part_records: Vec<GlyphPart>,
}

pub struct GlyphPart {
    glyph_id: u16,
    start_connector_length: UFWord,
    end_connector_length: UFWord,
    full_advance: UFWord,
    part_flags: u16,
}
