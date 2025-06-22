use super::super::Version16Dot16;
use super::Table;
use crate::impl_table;

pub enum Maxp {
    V0_5(MaxpV0_5),
    V1_0(MaxpV1_0),
}

impl_table!(Maxp, "maxp");

pub struct MaxpV0_5 {
    version: Version16Dot16,
    num_glyphs: u16,
}

pub struct MaxpV1_0 {
    version: Version16Dot16,
    num_glyphs: u16,
    max_points: u16,
    max_contours: u16,
    max_composite_points: u16,
    max_composite_contours: u16,
    max_zones: u16,
    max_twilight_points: u16,
    max_storage: u16,
    max_function_defs: u16,
    max_instruction_defs: u16,
    max_stack_elements: u16,
    max_size_of_instructions: u16,
    max_component_elements: u16,
    max_component_depth: u16,
}
