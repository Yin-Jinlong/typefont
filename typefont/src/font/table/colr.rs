use crate::font::table::Table;
use crate::font::{FWord, Fixed, Offset24, Offset32, UFWord};
use crate::impl_table;
use crate::types::F2D14;
use paste::paste;

pub enum COLR {
    V0(COLR0),
    V1(COLR1),
}

impl_table!(COLR, "COLR");

pub struct COLR0 {
    version: u16,
    num_base_glyph_records: u16,
    base_glyph_records_offset: Offset32,
    layer_records_offset: Offset32,
    num_layer_records: u16,
}

pub struct COLR1 {
    base: COLR0,
    base_glyph_list_offset: Offset32,
    layer_list_offset: Offset32,
    clip_list_offset: Offset32,
    var_index_map_offset: Offset32,
    item_variation_store_offset: Offset32,
}

pub struct BaseGlyph {
    glyph_id: u16,
    first_layer_index: u16,
    num_layers: u16,
}

pub struct Layer {
    glyph_id: u16,
    palette_index: u16,
}

pub struct BaseGlyphList {
    num_base_glyph_paint_records: u32,
    base_glyph_paint_records: Vec<BaseGlyphPaintRecord>,
}

pub struct BaseGlyphPaintRecord {
    glyph_id: u16,
    paint_offset: Offset32,
}

pub struct LayerList {
    num_layers: u32,
    paint_offsets: Vec<Offset32>,
}

pub struct ClipList {
    /// 1
    format: u8,
    num_clips: u32,
    clips: Vec<Clip>,
}

pub struct Clip {
    start_glyph_id: u16,
    end_glyph_id: u16,
    clip_box_offset: Offset24,
}

pub enum ClipBox {
    Format1(ClipBoxFormat1),
    Format2(ClipBoxFormat2),
}

pub struct ClipBoxFormat1 {
    /// 1
    format: u8,
    x_min: FWord,
    y_min: FWord,
    x_max: FWord,
    y_max: FWord,
}

pub struct ClipBoxFormat2 {
    /// 1
    format: u8,
    x_min: FWord,
    y_min: FWord,
    x_max: FWord,
    y_max: FWord,
    var_index_base: u32,
}

pub struct ColorStop {
    stop_offset: F2D14,
    palette_index: u16,
    alpha: F2D14,
}

pub struct VarColorStop {
    stop_offset: F2D14,
    palette_index: u16,
    alpha: F2D14,
    var_index_base: u32,
}

pub struct ColorLineBase<T> {
    extend: u8,
    num_stops: u16,
    color_stops: Vec<T>,
}

pub type ColorLine = ColorLineBase<ColorStop>;
pub type VarColorLine = ColorLineBase<VarColorStop>;

pub enum Paint {
    Format1(PaintColrLayers),
    Format2(PaintSolid),
    Format3(PaintVarSolid),
    Format4(PaintLinearGradient),
    Format5(PaintVarLinearGradient),
    Format6(PaintRadialGradient),
    Format7(PaintVarRadialGradient),
    Format8(PaintSweepGradient),
    Format9(PaintVarSweepGradient),
    Format10(PaintGlyph),
    Format11(PaintColrGlyph),
    Format12(PaintTransform),
    Format13(PaintVarTransform),
    Format14(PaintTranslate),
    Format15(PaintVarTranslate),
    Format16(PaintScale),
    Format17(PaintVarScale),
    Format18(PaintScaleAroundCenter),
    Format19(PaintVarScaleAroundCenter),
    Format20(PaintScaleUniform),
    Format21(PaintVarScaleUniform),
    Format22(PaintScaleUniformAroundCenter),
    Format23(PaintVarScaleUniformAroundCenter),
    Format24(PaintRotate),
    Format25(PaintVarRotate),
    Format26(PaintRotateAroundCenter),
    Format27(PaintVarRotateAroundCenter),
    Format28(PaintSkew),
    Format29(PaintVarSkew),
    Format30(PaintSkewAroundCenter),
    Format31(PaintVarSkewAroundCenter),
    Format32(PaintComposite),
}

macro_rules! var {
    ($t:ident) => {
        paste! {
        type [<PaintVar $t>] = PaintVar<[<Paint $t>]>;
        }
    };
}

pub struct PaintVar<T> {
    base: T,
    var_index_base: u32,
}

pub struct PaintColrLayers {
    /// 1
    format: u8,
    num_layers: u8,
    first_layer_index: u32,
}

pub struct PaintSolid {
    /// 2
    format: u8,
}

// format 3
var!(Solid);

pub struct PaintLinearGradient {
    /// 4
    format: u8,
    color_line_offset: Offset24,
    x0: FWord,
    y0: FWord,
    x1: FWord,
    y1: FWord,
    x2: FWord,
    y2: FWord,
}

// format 5
var!(LinearGradient);

pub struct PaintRadialGradient {
    /// 6
    format: u8,
    color_line_offset: Offset24,
    x0: FWord,
    y0: FWord,
    radius0: UFWord,
    x1: FWord,
    y1: FWord,
    radius1: UFWord,
}

// format 7
var!(RadialGradient);

pub struct PaintSweepGradient {
    /// 8
    format: u8,
    color_line_offset: Offset24,
    center_x: FWord,
    center_y: FWord,
    start_angle: F2D14,
    end_angle: F2D14,
}

// format 9
var!(SweepGradient);

pub struct PaintGlyph {
    /// 10
    format: u8,
    paint_offset: Offset24,
    glyph_id: u16,
}

pub struct PaintColrGlyph {
    /// 11
    format: u8,
    glyph_id: u16,
}

pub struct PaintTransform {
    /// 12
    format: u8,
    paint_offset: Offset24,
    transform_offset: Offset24,
}

pub struct PaintVarTransform {
    /// 13
    format: u8,
    paint_offset: Offset24,
    transform_offset: Offset24,
}

pub struct Affine2x3 {
    xx: Fixed,
    yx: Fixed,
    xy: Fixed,
    yy: Fixed,
    dx: Fixed,
    dy: Fixed,
}

pub struct VarAffine2x3 {
    xx: Fixed,
    yx: Fixed,
    xy: Fixed,
    yy: Fixed,
    dx: Fixed,
    dy: Fixed,
    var_index_base: u32,
}

pub struct PaintTranslate {
    /// 14
    format: u8,
    paint_offset: Offset24,
    dx: FWord,
    dy: FWord,
}

// format 15
var!(Translate);

pub struct PaintScale {
    /// 16
    format: u8,
    paint_offset: Offset24,
    scale_x: F2D14,
    scale_y: F2D14,
}

// format 17
var!(Scale);

pub struct PaintScaleAroundCenter {
    /// 18
    format: u8,
    paint_offset: Offset24,
    scale_x: F2D14,
    scale_y: F2D14,
    center_x: FWord,
    center_y: FWord,
}

// format 19
var!(ScaleAroundCenter);

pub struct PaintScaleUniform {
    /// 20
    format: u8,
    paint_offset: Offset24,
    scale: F2D14,
}

// format 21
var!(ScaleUniform);

pub struct PaintScaleUniformAroundCenter {
    /// 22
    format: u8,
    paint_offset: Offset24,
    scale: F2D14,
    center_x: FWord,
    center_y: FWord,
}

// format 23
var!(ScaleUniformAroundCenter);

pub struct PaintRotate {
    /// 24
    format: u8,
    paint_offset: Offset24,
    angle: F2D14,
}

// format 25
var!(Rotate);

pub struct PaintRotateAroundCenter {
    /// 26
    format: u8,
    paint_offset: Offset24,
    angle: F2D14,
    center_x: FWord,
    center_y: FWord,
}

// format 27
var!(RotateAroundCenter);

pub struct PaintSkew {
    /// 28
    format: u8,
    paint_offset: Offset24,
    x_skew_angle: F2D14,
    y_skew_angle: F2D14,
}

// format 29
var!(Skew);

pub struct PaintSkewAroundCenter {
    /// 30
    format: u8,
    paint_offset: Offset24,
    x_skew_angle: F2D14,
    y_skew_angle: F2D14,
    center_x: FWord,
    center_y: FWord,
}

// format 31
var!(SkewAroundCenter);

pub struct PaintComposite {
    /// 32
    format: u8,
    paint_offset: Offset24,
    composite_mode: u8,
    backdrop_paint_offset: Offset24,
}
