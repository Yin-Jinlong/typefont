use super::super::Fixed;
use super::super::LongDataTime;
use crate::impl_named;

/// 此表提供了有关字体的全局信息。
/// 边界框值应仅使用具有轮廓的字形进行计算。
/// 为了进行这些计算，应忽略没有轮廓的字形。
///
/// `font_direction_hint` 字段旨在支持从右到左的脚本。
/// 从左到右、从右到左和中性是字符的属性。
/// 中性角色没有固有的方向性;它不是宽度为零 （0） 的字符。
/// 空格和标点符号是中性字符的示例。非中性字符是具有固有方向性的字符。
/// 例如，罗马字母（从左到右）和阿拉伯字母（从右到左）具有很强的方向性。
/// 在存在空格和标点符号的“普通”罗马字体中，字体方向提示应设置为两 （2）。
///
/// `units_per_em` 值确定字体坐标网格的粒度，
/// 在该粒度上，可以为可视元素（如轮廓控制点或标记附件锚点位置）指定坐标。
///
/// 字形的边界框是包含字形所有控制点的最小矩形。
/// （有关更多信息，请参见 'glyf' 表一章。xMin、yMin、xMax 和 yMax 值提供了一个边界框，
/// 该边界框将包含字体中的所有字形。没有轮廓的字形将被忽略。
///
/// `mac_style` 位必须与 OS/2 表中的 `fs_selection` 位一致。
/// `fs_selection` 位在 `windows` 中的 `mac_style` 位上使用。
/// 在确定粗体或斜体字体时，将忽略 `PANOSE` 值和 `post` 表格值。
///
/// 由于历史原因，`Windows` 不使用此表中包含的 `font_revision` 值来确定字体的版本。
/// 相反，`Windows` 会评估 'name' 表中的版本字符串 （ID 5）。
///
/// 每个字形的左边距点对应于第一个幻像点（下面的“pp1”——请参阅幻像点 ），并与字形左边距 （lsb） 和 xMin 相关，如下所示：
/// ```txt
/// pp1 = xMin - lsb
/// ```
///
/// 如果设置了 `flags` 字段的位 1，则所有字形的 `pp1 = 0`，并且每个字形的 `x_min` 和左边距必须相等。
///
/// 在具有 `TrueType` 轮廓的可变字体中，
/// 每个字形的左侧方位必须等于 `x_min`，
/// 并且必须设置 `flags` 字段中的位 `1`。
/// 此外，必须在所有可变字体中清除位 `5`。
///
/// 此外，在可变字体中，控制点的最小或最大 `x` 或 `y` 值可能会发生变化，
/// 并且包含任何给定字形实例的轮廓或所有点的紧密边界矩形可以小于或大于该字形的默认实例。
/// 此表中的 `x_min`、`y_min`、`x_max` 和 `y_max` 值可能包含也可能不包含实例的派生字形轮廓。
/// 此外，没有为这些值提供变体增量。
/// 如果应用程序需要一个包含字体非默认实例字形的边界矩形，
/// 则应处理该实例的派生字形轮廓以确定边界矩形。
pub struct Head {
    /// 1
    major_version: u16,
    /// 0
    minor_version: u16,
    /// 由字体制造商设置。
    font_revision: Fixed,
    /// 计算方法：将其设置为 `0`，将整个字体求和 为 `uint32`，
    /// 然后存储 `0xB1B0AFBA - sum`。如果字体用作字体集合文件中的组件，
    /// 则此字段的值将因文件结构和字体表目录的更改而失效，必须忽略。
    checksum_adjustment: u32,
    /// `0x5F0F3CF5`
    magic_number: u32,
    /// - 0 : y=0 时的字体基线。
    /// - 1 : x=0 处的左侧承载点（仅与 `TrueType` 光栅化器相关）— 请参阅下面有关可变字体的其他信息。
    /// - 2 : 说明可能取决于点大小。
    /// - 3 : 对于所有内部缩放器数学运算，强制 ppem 为整数值;如果此位很清楚，则可以使用分数 PPEM 大小。强烈建议在提示字体中设置此项。
    /// - 4 : 指令可能会更改前进宽度（前进宽度可能不会线性缩放）。
    /// - 5 : 此位未在 OpenType 中使用，不应设置此位，以确保在所有平台上的行为兼容。如果设置，则可能会导致某些平台中的垂直布局具有不同的行为。
    /// - 6-10 : 这些位不在 OpenType 中使用，应始终清除。
    /// - 11 : 字体数据是“无损”的，因为经过了优化的转换和/或压缩（例如 ISO/IEC 14496-18、MicroType® Express、WOFF 2.0 或类似定义的压缩机制），其中保留了原始字体功能和特性，但不能保证输入和输出字体文件之间的二进制兼容性。由于应用了转换，DSIG 表也可能失效。
    /// - 12 : 字体转换（生成兼容的量度）。
    /// - 13 : 针对 ClearType® 优化的字体。请注意，不应将依赖嵌入位图 （EBDT） 进行渲染的字体视为针对 ClearType 进行了优化，因此应保持此位处于清除状态。
    /// - 14 : Last Resort 字体。如果设置，则表示在 'cmap' 子表中编码的字形只是码位范围的通用符号表示形式，并不真正表示对这些码位的支持。如果未设置，则表示 'cmap' 子表中编码的字形表示对这些码位的正确支持。
    /// - 15 : 0
    flags: u16,
    units_per_em: u16,
    created: LongDataTime,
    modified: LongDataTime,
    /// 所有字形边界框的最小 x 坐标。
    x_min: i16,
    /// 所有字形边界框的最小 y 坐标。
    y_min: i16,
    /// 所有字形边界框的最大 x 坐标。
    x_max: i16,
    /// 所有字形边界框的最大 y 坐标。
    y_max: i16,
    mac_style: u16,
    lowest_rec_ppem: u16,
    font_direction_hint: i16,
    /// `0` 表示短偏移量 （`Offset16`），`1` 表示长偏移量 （`Offset32`）。
    index_to_loc_format: i16,
    /// `0` 表示当前格式。
    glyph_data_format: i16,
}

pub mod mac_flag {
    /// 粗体
    const BOLD: u16 = 1;
    /// 斜体
    const ITALIC: u16 = 1 << 1;
    /// 下划线
    const UNDERLINE: u16 = 1 << 2;
    /// 轮廓
    const OUTLINE: u16 = 1 << 3;
    /// 阴影
    const SHADOW: u16 = 1 << 4;
    /// 压缩
    const CONDENSED: u16 = 1 << 5;
    /// 扩展
    const EXTENDED: u16 = 1 << 6;
}

pub mod font_direction_hint {
    /// 完全混合的定向字形
    #[deprecated(note = "use 2")]
    const FULLY_MIXED_DIRECTIONAL_GLYPHS: i16 = 0;
    /// 仅强烈从左到右
    const ONLY_STRONGLY_LEFT_TO_RIGHT: i16 = 1;
    /// 与 1 类似，但也包含中性色
    const L2R_NEUTRALS: i16 = 2;
    /// 仅强烈从右到左
    const ONLY_STRONGLY_TO_RIGHT_LEFT: i16 = -1;
    /// 与 -1 类似，但也包含中性色
    const R2L_NEUTRALS: i16 = -2;
}

impl_named!(Head, "head");
