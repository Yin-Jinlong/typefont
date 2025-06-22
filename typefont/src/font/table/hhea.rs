use super::super::{FWord, UFWord};
use crate::impl_named;

/// # 水平头表
///
/// 此表包含水平布局的信息。
/// `min_right_sidebearing`、`min_left_side_bearing` 和 `x_max_range` 中的值
/// 应仅使用具有等高线的字形进行计算。为了进行这些计算，
/// 应忽略没有轮廓的字形。所有保留区域都必须设置为 `0`。
///
/// 此表中的 `ascender`、`descender` 和 `line_gap` 值特定于 `Apple`;
/// `OS/2` 表中的 `s_typo_ascender`、`s_typo_descender` 和 `s_typo_line_gap` 字段在 `windows` 平台上使用，
/// 建议用于新的文本布局实现。
/// 字体开发人员应评估可能使用此表或 `OS/2` 表中的字段的目标应用程序的行为，
/// 以确保布局一致。
///
/// ## `hhea` 表格和 `OpenType` 字体变体
///
/// 在可变字体中，可能需要针对不同的变体实例调整水平标题表中的各种字体度量值。
/// 量度变体 （`MVAR`） 表中提供了`hhea`条目的变体数据。
/// 不同的 `hhea` 条目使用 `value` 标签与 `MVAR` 表中的特定变体数据相关联，如下所示：
///
/// | 'hhea' 条目 | Tag |
/// |---|---|
/// | `caret_offset`     | `'hcof'` |
/// | `caret_slope_rise` | `'hcrs'` `'HCRS'` |
/// | `caret_slope_run`  | `'hcrn'`  |
///
pub struct Hhea {
    /// 1
    major_version: u16,
    /// 0
    minor_version: u16,
    /// 上沿
    ascender: FWord,
    /// 下沿
    descender: FWord,
    /// 在某些传统平台实现中，负 `line_gap` 值被视为零。
    line_gap: FWord,
    /// `hmtx`表中的最大前进宽度值。
    advance_width_max: UFWord,
    /// 对于带有轮廓的字形，`hmtx`表中的最小左边距值（应忽略空字形）。
    min_left_side_bearing: FWord,
    /// 最小右侧承载值;对于具有轮廓的字形，计算公式为 `min(aw - (lsb + xMax - xMin))` （空字形应忽略）。
    min_right_side_bearing: FWord,
    /// Max(lsb + (xMax - xMin))
    x_max_extent: FWord,
    /// 用于计算光标的斜率 （上升/运行）;1 表示垂直。
    caret_slope_rise: i16,
    ///0 表示垂直。
    caret_slope_run: i16,
    caret_offset: i16,
    /// 0
    reserved1: i16,
    /// 0
    reserved2: i16,
    /// 0
    reserved3: i16,
    /// 0
    reserved4: i16,
    /// 0 表示当前格式。
    metric_data_format: i16,
    /// `hmtx`表中的 `h_metric` 条目数
    number_of_h_metrics: u16,
}

impl_named!(Hhea, "hhea");
