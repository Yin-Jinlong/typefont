use super::Table;
use crate::font::Offset32;
use crate::impl_table;
use crate::types::U24;

/// # cmap — 字符到字形索引映射表
/// 2024/05/29
///
/// 此表定义了字符代码到默认字形索引的映射。
/// 可以定义不同的子表，每个子表包含不同字符编码方案的映射。
/// 表头指示存在哪些字符编码的子表。
///
/// 无论采用何种编码方案，
/// 未对应到字体中任何字形的字符代码都应映射到字形索引0。
/// 该位置的字形必须是一个表示缺失字符的特殊字形，
/// 通常称为`.notdef`。
///
/// 每个子表采用七种可能格式之一，
/// 并以一个格式字段开始，指示所使用的格式。
/// 前四种格式——格式0、2、4和6——在`Unicode 2.0`之前已定义。
/// 这些格式允许使用8位单字节、8位多字节和16位编码。
/// 随着`Unicode 2.0`引入补充平面，`Unicode`可寻址的编码空间扩展超过16位。
/// 为此，增加了三种额外的格式——格式8、10和12——允许32位编码方案。
///
/// Unicode中的其他增强功能导致添加了其他子表格式。
/// 子表格式13允许许多字符高效地映射到单个字形；
/// 这对于“最后手段”字体很有用，这种字体为所有可能的`Unicode`字符提供回退渲染，
/// 并对不同的`Unicode`范围使用不同的回退字形。子表格式14提供了一种支持`Unicode`变体序列的统一机制。
///
/// 在这七种可用格式中，并非所有都在当今常用。
/// 对于大多数新字体来说，根据支持的Unicode字符集，格式4或12是合适的。
/// 格式14用于许多应用程序中以支持Unicode变体序列。
/// 一些平台也使用格式13作为最后手段的回退字体。
/// 不推荐在新字体中使用其他子表格式。
/// 然而，应用程序开发者应该预料到字体中可能会使用任何一种格式。
///
/// 注意：使用较新的子表格式的字体的`cmap`表版本号保持为`0x0000`。
pub struct Cmap {
    /// 0
    version: u16,
    num_tables: u16,
    encoding_records: Vec<EncodingRecord>,
}

impl_table!(Cmap, "cmap");

///
/// 编码记录中的平台ID和平台特定的编码ID用于指定特定的字符编码。
/// 对于Macintosh平台，映射子表中的语言字段也用于此目的。
///
/// `cmap`头中的编码记录条目必须首先按平台ID排序，然后按平台特定的编码ID排序，
/// 最后按照对应子表中的语言字段排序。
/// 每个平台ID、平台特定的编码ID和子表语言组合在'cmap'表中只能出现一次。
///
/// 除了`格式14`的子表外，所有其他子表都是排他的：
/// 应用程序应选择并使用其中一个，并忽略其余的。
/// 如果使用了`Unicode`子表（平台0，或平台3/编码1或10），
/// 则也可以补充一个使用平台0/编码5的`格式14`子表，以映射`Unicode`变体序列。
///
/// 如果字体既包括16位编码（通常是`格式4`）的Unicode子表，
/// 也包括32位编码（`格式10或12`）的Unicode子表，
/// 则32位编码子表支持的字符应该是16位编码子表支持字符的超集，
/// 且应用程序应该使用32位编码。
///
/// 字体不应同时包含使用`格式4`和`格式6`的16位`Unicode`子表；应当使用`格式4`。
/// 类似地，字体不应同时包含使用`格式10`和`格式12`的32位`Unicode`子表；
/// 应当使用`格式12`。
///
/// 如果字体为相同格式但不同平台ID的Unicode子表包含编码记录，
/// 则应用程序可以选择使用哪一个，
/// 但应在每次使用该字体时保持这种选择的一致性。
pub struct EncodingRecord {
    platform_id: u16,
    encoding_id: u16,
    /// 此编码从表开头到子表的字节偏移量。
    subtable_offset: Offset32,
    sub_tables: Vec<CmapSubTable>,
}

///
/// 平台 ID 值 240 到 255 是为用户定义的平台保留的，绝不能分配给已注册的平台。
///
pub mod platform_id {
    const UNICODE: u16 = 0;
    const MACINTOSH: u16 = 1;
    #[deprecated]
    const ISO: u16 = 2;
    const WINDOWS: u16 = 3;
    const CUSTOM: u16 = 4;
}

/// Unicode Platform Encoding Id
pub mod unicode_platform_encoding {
    #[deprecated]
    const UNICODE1_0: u16 = 0;
    #[deprecated]
    const UNICODE1_1: u16 = 1;
    #[deprecated]
    const ISO_IEC_10646: u16 = 2;
    /// 编码 ID 3 应与 'cmap' 子表格式 4 或 6 结合使用。
    const UNICODE2_0_BMP: u16 = 3;
    /// 编码 ID 4 应与子表格式 10 或 12 结合使用。
    const UNICODE2_0_FULL: u16 = 4;
    /// 用于子表格式 14
    ///
    /// 字体支持的 Unicode 变体序列应在 'cmap' 表中使用格式 14 子表指定。
    /// 格式 14 子表只能在平台 ID 0 和编码 ID 5 下使用;
    /// 编码 ID 5 只能与格式 14 子表一起使用。
    const UNICODE_VARIATION: u16 = 5;
    /// 用于子表格式 13
    ///
    /// 编码 ID 6 只能与 'cmap' 子表格式 13 结合使用;
    /// 子表格式 13 只能在平台 ID 0 和编码 ID 6 下使用。
    const UNICODE_FULL: u16 = 6;
}

/// 较旧的 Macintosh 版本要求字体具有平台 ID 1 的“cmap”子表。
/// 对于当前的 Apple 平台，不建议使用平台 ID 1。
pub mod macintosh_platform_encoding {}

#[deprecated]
pub mod iso_platform_encoding {
    #[deprecated]
    const ASCII: u16 = 0;
    #[deprecated]
    const ISO_10646: u16 = 1;
    #[deprecated]
    const ISO_8859_1: u16 = 2;
}

/// `Windows` 平台支持多种编码。
/// 为 `Windows` 创建字体时，
/// 应始终使用 `Unicode` `cmap`子表 - 平台 ID 3 和编码 ID 1 或编码 ID 10。
///
/// 在 `Windows` 平台上仅支持 `Unicode` BMP 字符（`U+0000` 到 `U+FFFF`）的字体
/// 必须使用编码 1 和格式 4 子表。此编码不得用于支持 `Unicode` 补充平面字符。
///
/// 在 `Windows` 平台上支持 `Unicode` 补充平面字符（`U+10000` 到 `U+10FFFF`）的字体
/// 必须使用编码 10 和`格式 12` 子表。
///
/// 创建元件编码是为了支持具有任意修饰或 `Unicode`
/// 或其他标准编码中不支持的元件的字体。
/// 将使用`格式 4` 子表，通常在以 `0xF020` 开头的代码位置分配最多 `224` 个图形字符。
/// 这对应于 `Unicode` 专用区域 （PUA） 中的子范围，尽管这不是 `Unicode` 编码。
/// 在传统用法中，某些应用程序将使用单字节编码表示文本中的符号字符，
/// 然后将 `0x20` 映射到字体中的 `OS/2.usFirstCharIndex` 值。在新字体中，
/// 非 `Unicode` 的符号或字符应使用 `Unicode` `cmap`子表中的 PUA 码位进行编码。
///
pub mod windows_platform_encoding {
    const SYMBOL: u16 = 0;
    const UNICODE_BMP: u16 = 1;
    const SHIFT_JIS: u16 = 2;
    const PRC: u16 = 3;
    const BIG5: u16 = 4;
    const WANSUNG: u16 = 5;
    const JOHAB: u16 = 6;
    const UNICODE_FULL: u16 = 10;
}

///
/// 所有'cmap'子表格式都包含一个语言字段。
/// 对于平台ID不是Macintosh（平台ID 1）的所有'cmap'子表，
/// 这个语言字段必须设置为零。
/// 对于平台ID是Macintosh的'cmap'子表，
/// 将此字段设置为'cmap'子表的Macintosh语言ID加一，
/// 或者如果'cmap'子表不是特定语言的话则设置为零。
///
/// 例如，一个Mac OS土耳其'cmap'子表必须将此字段设置为18，
/// 因为土耳其的Macintosh语言ID是17。
/// 一个Mac OS Roman 'cmap'子表必须将此字段设置为0，
/// 因为Mac OS Roman不是一个特定语言的编码。
///
pub enum CmapSubTable {
    Format0(ByteEncodingTable),
    Format2(HighByteMappingThrough),
    Format4(SegmentMappingToDeltaValues),
    Format6(TrimmedTableMapping),
    Format8(Mixed16And32BitCoverage),
    Format10(TrimmedArray),
    Format12(SegmentedCoverage),
    Format13(ManyToOneRangeMappings),
    Format14(UnicodeVariationSequences),
}

///
/// # 格式 0
///
/// 格式 0 是旧版 Macintosh 平台上使用的标准映射子表，
/// 但在较新的 Apple 平台上不是必需的。
///
/// 这是字符代码到字形索引的简单 1 对 1 映射。
/// 字形集限制为 `256` 个。
/// 如果此格式用于索引到更大的字形集中，则只能访问前 `256` 个字形。
///
pub struct ByteEncodingTable {
    /// 0
    format: u16,
    /// 这是子表的长度（以字节为单位）。
    length: u16,
    language: u16,
    glyph_id_array: [u8; 256],
}

///
/// # 格式 2
///
/// 此子表格式是按照用于日语、中文和朝鲜字符的国家字符代码标准
/// 为“双字节”编码创建的。这些代码标准使用 8 位/16 位混合编码。
/// 这种格式目前并不常用。
///
/// 在这些混合 8 位/16 位编码中，
/// 某些字节值表示 2 字节字符的第一个字节。
/// （这些字节值也可用作 2 字节字符的第二个字节。
/// 此外，即使对于 2 字节字符，字符代码到字形索引值的映射也在很大程度上取决于第一个字节。
/// 因此，该表以一个数组开头，该数组将第一个字节映射到 `SubHeader` 记录。
/// 对于 2 字节字符代码，`SubHeader` 用于将第二个字节的值映射到字形索引数组的子范围（ 子数组 ），
/// 如下所述。
/// 当处理混合的 8/16 位文本时，`SubHeader` 0 很特殊：
/// 它用于单字节字符代码。当使用 `SubHeader` 0 时，
/// 不需要第二个字节;单字节值通过指定的子数组进行映射。
///
pub struct HighByteMappingThrough {
    /// 2
    format: u16,
    /// 这是子表的长度（以字节为单位）。
    length: u16,
    language: u16,
    /// 将高字节映射到 `sub_headers` 数组的数组：值为 `sub_headers` 索引 × 8。
    sub_header_keys: [u16; 256],
    sub_headers: Vec<HighByteMappingThroughTableSubHeader>,
    /// entry_count
    glyph_id_array: [u16; 256],
}

///
/// `first_code` 和 `entry_count` 值指定一个子范围，
/// 该子范围从 `first_code` 开始，长度等于 `entry_count` 的值。
/// 此子范围保持在所映射字节的 `0-255` 范围内。
/// 此子范围之外的字节将映射到字形索引 0 （缺少字形） 。
/// 然后，此子范围中字节的偏移量将用作 `glyph_id_array` 的相应子数组的索引。
/// 此子数组的长度也是 `entry_count`。`id_range_offset` 的值
/// 是超出 `id_range_offset` 单词实际位置的字节数，
/// 其中显示了与 `first_code` 对应的 `glyph_id_array` 元素。
///
/// 最后，如果从子数组获得的值不是 `0`（这表示缺少字形），
/// 则应向其添加 `idDelta` 以获取 `glyph_index`。
/// 值 `id_delta` 允许将同一子数组用于多个不同的子标题。
/// `id_delta` 算法是模 `65536`。如果将 `id_delta` 添加到子数组的值后，
/// 结果小于零，则添加 `65536` 以获取有效的字形 `ID`。
///
pub struct HighByteMappingThroughTableSubHeader {
    /// 此 SubHeader 的第一个有效低字节。
    first_code: u16,
    /// 此 SubHeader 的有效低字节数。
    entry_count: u16,
    id_delta: u16,
    id_range_offset: u16,
}

/// # 格式 4
///
/// 这是仅支持 `Unicode` 基本多语言平面字符（`U+0000` 到 `U+FFFF`）
/// 的字体的标准字符到字形索引映射子表。
///
/// 注意： 要支持 `Unicode` 补充平面字符，应使用`格式 12`。
///
/// 当字体所表示的字符的字符代码属于多个连续范围时，将使用此格式，
/// 其中部分或全部范围中可能存在孔（即，范围内的某些代码可能在字体中没有表示形式）。
/// 与格式相关的数据分为三个部分，必须按以下顺序出现：
///
/// 1. 四个单词的标题提供了用于优化区段列表搜索的参数。
/// 2. 四个并行数组描述段（每个连续的代码范围对应一个段）。
/// 3. 字形 ID （无符号单词） 的可变长度数组。
///
/// 段数由 `seg_count` 指定，它不是直接在 Headers 中给出的，
/// 而是很容易从 `seg_count_x2` 派生的。
/// 所有其他标头参数都是从它派生的。
/// `search_range` 值是小于或等于 `seg_count` 的 2 的最大幂的两倍。
/// 例如，如果 `seg_count=39`，则有以下结果：
///
/// |  |  |
/// |---|---|
/// | seg_count_x2   | 78 |
/// | search_range   | 64 (= 2 × (2 的最大幂 <=39)) |
/// | entry_selector | 5 (= log232) |
/// | range_shift | 14 (= 2 × 39 - 64) |
///
/// 为了帮助快速进行二进制搜索，
/// `search_range`、`entry_selector` 和 `range_shift` 字段
/// 作为参数包含在内，可用于配置搜索算法。
/// 特别是，当条目数是 2 的幂时，二分搜索是最佳的。
/// `search_range` 字段提供使用该约束可搜索的最大项目数（最大幂 2）。
/// `range_shift` 字段提供还需要搜索的剩余项数。
/// `entry_selector` 字段指示需要输入二叉树的最大级别数。
///
/// 在硬件功能有限的设备上的早期实现中，
/// `search_range`、`entry_selector` 和 `range_shift` 字段提供的优化非常重要。
/// 它们在现代设备上的重要性较低，但仍可用于某些实现。
/// 但是，不正确的值可能会被用作针对某些实现的攻击媒介。
/// 由于在解析文件时可以从 `seg_count_x2` 字段派生这些值，
/// 因此强烈建议解析实现不依赖于字体中的
/// `search_range`、`entry_selector` 和 `range_shift` 字段，
/// 而是独立于 `seg_count_x2` 派生它们。
/// 但是，字体文件应继续为这些字段提供有效值，
/// 以保持与所有现有实现的兼容性。
///
/// 每个段都由 `start_code` 和 `end_code` 以及 `id_delta` 和 `id_range_offset` 描述，
/// 它们用于映射段中的字符代码。
/// 段按递增 `end_code` 值的顺序排序，
/// 段值在四个并行数组中指定。
/// 搜索大于或等于要映射的字符代码的第一个 `end_code`。
/// 如果对应的 `start_code` 小于或等于字符代码，
/// 则使用对应的 `id_delta` 和 `id_range_offset`
/// 将字符代码映射到字形索引（否则返回 missingGlyph）。
/// 要终止搜索，必须 `0xFFFF` 最终的 `start_code` 和 `end_code` 值。
/// 此区段不需要包含任何有效的映射。
/// （它可以只将单个字符代码 `0xFFFF` 映射到 `missing_glyph`）。
/// 但是，该区段必须存在。
///
/// 如果段的 `id_range_offset` 值不为 `0`，
/// 则字符代码的映射依赖于 `glyph_id_array`。
/// `star_code` 的字符代码偏移量将添加到 `id_range_offset` 值中。
/// 此总和用作 `id_range_offset` 本身中当前位置的偏移量，
/// 以索引出正确的 `glyph_id_array` 值。这个晦涩难懂的索引技巧之所以有效，
/// 是因为 `glyph_id_array` 紧跟在字体文件中的 `id_range_offset` 之后。
/// 生成字形索引的 C 表达式为：
/// ```c
/// glyphId = *(id_range_offset[i]/2
///             + (c - start_code[i])
///             + &id_range_offset[i])
/// ```
///
/// 值 c 是有问题的字符代码，i 是 c 出现的段索引。
/// 如果从索引作获取的值不是 `0` （指示 `missing_glyph`） ，
/// 则向其添加 `id_delta`\[i] 以获取字形索引。
/// `id_delta` 算法是模数 `65536`。
///
/// 如果 `id_range_offset` 为 0，
/// 则直接将 `id_delta` 值添加到字符代码偏移量（即 `id_delta[i] + c`）中，
/// 以获取对应的字形索引。同样，`idDelta` 算法是模 `65536`。
/// 如果添加 `id_delta[i] + c` 后的结果小于零，
/// 则添加 `65536` 以获取有效的字形 `ID`。
///
/// 例如，用于将字符 10-20、30-90 和 153-480
/// 映射到连续的字形索引范围的表格的变体部分可能如下所示：
///
/// | | |
/// |---|---|
/// | seg_count_x2    | 8 |
/// | search_range    | 8 |
/// | entry_selector  |2 |
/// | range_shift     | 0 |
/// | end_code        | 20 90 480 0xffff |
/// | reserved_pad    | 0 |
/// | start_code      | 10 30 153 0xffff |
/// | id_delta        | -9 -18 -80 1 | |
/// | id_range_offset | 0 0 0 0 |
///
/// 此表生成以下映射：
/// ```text
/// 10 ⇒ 10 - 9 = 1
/// 20 ⇒ 20 - 9 = 11
/// 30 ⇒ 30 - 18 = 12
/// 90 ⇒ 90 - 18 = 72
/// 153 ⇒ 153 - 80 = 73
/// 480 ⇒ 480 - 80 = 400
/// 0xffff ⇒ 0
/// ```
///
/// 请注意，可以重新设计 `delta` 值，以便对区段重新排序。
///
pub struct SegmentMappingToDeltaValues {
    /// 4
    format: u16,
    /// 这是子表的长度（以字节为单位）。
    length: u16,
    language: u16,
    /// 2 × seg_count
    seg_count_x2: u16,
    /// ((2**floor(log2(seg_count))) * 2
    search_range: u16,
    /// (log2(search_range/2) 等价于  floor(log2(seg_count))
    entry_selector: u16,
    /// ((seg_count * 2) - search_range)
    range_shift: u16,
    /// end_code\[seg_count]
    ///
    /// 每个段的结束 `character_code`，last=0xFFFF。
    end_code: Vec<u16>,
    /// 0
    reserved_pad: u16,
    /// startCode\[seg_count]
    ///
    /// 每个句段的起始字符代码。
    start_code: Vec<u16>,
    /// id_delta\[seg_count]
    ///
    /// 句段中所有字符代码的 Delta。
    id_delta: Vec<u16>,
    /// id_range_offset\[seg_count]
    ///
    /// 句段中所有字符代码的 Delta。
    id_range_offset: Vec<u16>,
    /// entry_count
    glyph_id_array: Vec<u16>,
}

/// # 格式 6
///
/// 格式 6 旨在当字体的字符代码属于单个连续范围时，将 16 位字符映射到字形索引。
///
/// `first_code` 和 `entry_count` 值指定可能的字符代码范围内的子范围
/// （从 `first_code` 开始，`length = entry_count`）。
/// 此子范围之外的代码将映射到字形索引_0。
/// 此子范围中代码 （与第一个代码） 的偏移量用作 `glyph_id_array` 的索引，
/// 该索引提供字形索引值。
///
pub struct TrimmedTableMapping {
    /// 6
    format: u16,
    /// 这是子表的长度（以字节为单位）。
    length: u16,
    /// 此子表的字节长度（包括标头）
    language: u16,
    /// 子范围的第一个字符代码。
    first_code: u16,
    /// 子范围中的字符代码数。
    entry_count: u16,
    /// entry_count
    glyph_id_array: Vec<u16>,
}

/// # 格式 8
///
/// 子表`格式 8` 旨在支持 `UTF-16` 编码中的 `Unicode` 补充平面字符，尽管它并不常用。
/// `格式 8` 与`格式 2` 类似，因为它都提供混合长度的字符代码。
/// 但是，它允许使用 `16` 位和 `32` 位字符代码，而不是允许 `8` 位和 `16` 位字符代码。
///
/// 如果字体包含 `Unicode` 补充平面字符（`U+10000` 到 `U+10FFFF`），
/// 那么它可能也包括 `Unicode BMP` 字符（`U+0000` 到 `U+FFFF`）。
/// 因此，需要映射 `16` 位和 `32` 位字符代码的混合。
/// 我们做了一个简化的假设：即，没有 `32` 位字符代码与任何 `16` 位字符代码共享相同的前 `16` 位。
/// （由于 `Unicode` 代码空间仅扩展到 `U+10FFFF`，
/// 因此仅字符 `U+0000` 到 `U+0010` 存在潜在冲突，
/// 这些字符是非打印控制字符。
/// 这意味着可以通过直接查看 `16` 位值来确定特定 `16` 位值是独立字符代码还是 `32` 位字符代码的开头，
/// 而无需进一步的信息。
///
/// 每个顺序映射组记录都指定一个字符范围和从第一个字符映射的起始字形 ID。后续字符的字形 ID 按顺序排列
///
///
///
/// 这里有一些说明。使用 `end_char_code` 而不是计数，
/// 因为组匹配的比较通常是在现有字符代码上完成的，
/// 并且使用 `end_char_code` 显式可以节省每个组添加的必要性。
/// 必须通过增加 `start_char_code` 对组进行排序。
/// 组的 `end_char_code` 必须小于以下组的 `start_char_code`（如果有）。
///
/// 要确定特定单词（cp）是否是 32 位代码点的前半部分，
/// 可以使用像 ( `is32[ cp / 8 ] & ( 1 << ( 7 - ( cp % 8 ) ) ) )` 这样的表达式）。
/// 如果该值为非零，则字是 32 位码位的前半部分。
pub struct Mixed16And32BitCoverage {
    /// 8
    format: u16,
    /// 0
    reserved: u16,
    /// 此子表的字节长度（包括标头）
    length: u32,
    language: u32,
    /// 紧密排列的位数组（总共 8K 字节），
    /// 指示特定的 16 位（索引）值是否是 32 位字符代码的开头
    is32: [u8; 8192],
    /// 随后的分组数
    num_groups: u32,
    /// groups\[num_groups]
    ///
    /// `0` 不是 `32` 位码位的高字的特殊值。
    /// 字体不能同时具有码位 `0x0000` 字形和字数较高的` 0x0000` 码位的字形。
    ///
    /// 即使字体不包含特定 `16` 位起始值的字形，
    /// 也存在指示特定 `16` 位值是否是 `32` 位字符代码开头的打包位数组也很有用。
    /// 这是因为系统软件通常需要知道下一个字符开始前多少字节，
    /// 即使当前字符映射到缺失的字形也是如此。
    /// 通过在此表中显式包含此信息，无需将 “秘密” 知识编码到 `OS` 中。
    ///
    /// 尽管创建此格式是为了支持 `Unicode` 补充平面字符，
    /// 但它并未得到广泛支持或使用。此外，除 `Unicode` 外，
    /// 其他字符编码均不使用混合 `16/32` 位字符。不建议使用此格式。
    ///
    /// 请注意，
    /// 如果此组用于一个或多个 `16` 位字符代码（由 `is32` 数组确定），
    /// 则此 `32` 位值将把 `16` 位高位设置为零
    groups: Vec<SequentialMapGroup>,
}

pub struct SequentialMapGroup {
    /// 此组中的第一个字符代码
    start_char_code: u32,
    /// 此组中的最后一个字符代码;与上面列出的 startCharCode 条件相同
    end_char_code: u32,
    /// 与起始字符代码对应的字形索引
    start_glyph_id: u32,
}

/// # 格式10
///
/// 此格式未广泛使用，在 `Windows` 平台上不受支持。
/// 它最适合于仅支持连续的 `Unicode` 补充平面字符范围的字体，但此类字体很少见。
pub struct TrimmedArray {
    /// 10
    format: u16,
    /// 0
    reserved: u16,
    /// 此子表的字节长度（包括标头）
    length: u32,
    language: u32,
    /// 覆盖的第一个字符代码
    start_char_code: u32,
    /// 覆盖的字符代码数量
    num_chars: u32,
    /// 所涵盖的字符代码的字形索引数组
    glyph_id_array: Vec<u16>,
}

/// # 格式12
///
/// 这是支持包含补充平面字符（`U+10000` 到 `U+10FFFF`）的 `Unicode` 字符指令表的
/// 字体的标准字符到字形索引映射子表。
///
/// 包含`格式 12` 子表的字体也可以包含`格式 4` 子表，以便与较旧的应用程序兼容。但是，这不是必需的。
///
/// `格式 12` 与`格式 4` 类似，因为它定义了用于稀疏表示的段。
/// 但是，它的不同之处在于它使用 `32` 位字符代码。
///
/// 顺序映射组记录的格式与用于格式 `8` 子表的格式相同。
/// 但是，有关 `16` 位字符代码的限定条件在此处不适用，
/// 因为字符代码统一为 `32` 位。
///
pub struct SegmentedCoverage {
    /// 12
    format: u16,
    /// 0
    reserved: u16,
    /// 此子表的字节长度（包括标头）
    length: u32,
    language: u32,
    /// 随后的分组数
    num_groups: u32,
    ///groups\[num_groups]
    ///
    /// 必须通过增加 `start_char_code` 对组进行排序。
    /// 组的 `end_char_code` 必须小于以下组的 `start_char_code`（如果有）。
    /// 使用 `end_char_code` 而不是计数，因为组匹配的比较通常是在现有字符代码上完成的，
    /// 并且使用 `end_char_code` 显式可以节省每个组添加的必要性。
    groups: Vec<SequentialMapGroup>,
}

/// # 格式 13
///
/// 此子表适用于将同一字形用于跨越代码空间的多个范围的数百甚至数千个连续字符的情况。
/// 此子表格式可能对 “last voc” 字体有用，
/// 尽管这些字体也可能使用其他合适的子表格式。
/// （对于 “last voc” 字体，另请参阅 'head' 表标志，位 14。
///
/// 注意： 子表格式 13 与格式 12 具有相同的结构;它仅在 `start_glyph_id/glyph_id` 字段的解释上有所不同。
///
pub struct ManyToOneRangeMappings {
    /// 13
    format: u16,
    /// 0
    reserved: u16,
    /// 此子表的字节长度（包括标头）
    length: u32,
    language: u32,
    /// 随后的分组数
    num_groups: u32,
    ///groups\[num_groups]
    groups: Vec<ConstantMapGroup>,
}

type ConstantMapGroup = SequentialMapGroup;

/// # 格式 14
///
/// 子`表格式 14` 指定字体支持的 `Unicode` 变体序列 （UVS）。
/// 根据 `Unicode` 标准，变体序列包括一个基本字符和一个变体选择器。
/// 例如，<`U+82A6`、`U+E0101`>。
///
/// 此子表格式只能在平台 ID 0 和编码 ID 5 下使用。
///
/// 该子表将字体支持的 UVS 分为两类：
/// “默认”和“非默认”UVS。给定一个 UVS，
/// 如果通过在 `Unicode` `cmap`子表（即 BMP 子表或 BMP + 补充平面子表）
/// 中查找该序列的基本字符获得的字形是用于该序列的字形，则该序列是“默认”UVS。
/// 否则，它是“非默认”UVS，并且用于该序列的字形在`格式 14` 子表本身中指定。
///
/// 每个 `VariationSelector` 记录都指定一个变体选择器字符，
/// 并偏移到用于使用该变体选择器映射变体序列的 “default” 和 “non-default” 表。
///
/// ## 例子
///
/// 下面是一个示例，说明如何在识别 `JIS-2004` 变体字形的字体中使用`格式 14` `cmap` 子表。
/// 此示例中的 CID（字符 ID）是指 Adobe 字符集`Adobe-Japan1`中的 CID（字符 ID），
/// 可以假定它们与示例中字体中的字形 ID 相同。
///
/// `JIS-2004` 更改了某些码位的默认字形变体。例如：
///
/// - `JIS-90: U+82A6 ⇒ CID 1142`
/// - `JIS-2004: U+82A6 ⇒ CID 7961`
///
/// 通过使用 Unicode 变体序列来支持这两种字形变体，如 Unicode 的 UVS 注册表中的以下示例所示：
/// ```text
/// U+82A6 U+E0100 ⇒ CID 1142
/// ```
///
/// 如果字体希望默认支持 JIS-2004 变体，它将：
///
/// 在 Unicode 'cmap' 子表中的 `U+82A6` 处对字形 ID 7961 进行编码;
///
/// 在 UVS“cmap”子表的默认 UVS 表中指定 `<U+82A6, U+E0101>`
/// （`var_selector` 将为 `0x0E0101`，`default_uvs_offset` 将指向包含 `0x0082A6` `Unicode` 值的数据）;
///
/// 在 UVS“cmap”子表的“非默认 UVS”表中指定 `<U+82A6, U+E0100> ⇒字形` ID 1142
/// （`var_selector` 将为 `0x0E0100`，
/// `non_default_base_uvsoffset` 将指向包含 `unicode_value` `0_x_0082_a_6` 和 `glyph_id` 1142_的数据）。
///
/// 但是，如果字体希望默认支持 `JIS-90` 变体，它将：
///
/// 在 `Unicode` `cmap` 子表中的 `U+82A6` 处对字形 ID 1142 进行编码;
/// 在 UVS 'cmap' 子表的 Default UVS 表中指定 `<U+82A6, U+E0100>`;
/// 在 UVS“cmap”子表的“非默认 UVS”表中指定 `<U+82A6, U+E0101> ⇒ 字形` `ID 7961`。
///
pub struct UnicodeVariationSequences {
    /// 14
    format: u16,
    /// 此子表的字节长度（包括此标头）
    length: u16,
    /// 变体选择器记录数
    num_var_selector_records: u16,
    /// var_selector\[num_var_selector_records]
    var_selector: Vec<VariationSelector>,
}

///
/// `VariationSelector` 记录按 `var_selector` 的升序排序。
/// 任何两条记录都不能具有相同的 `var_selector` 值。
///
/// VariationSelector 记录及其偏移量指向的数据，
/// 用于指定变体选择器是记录的 `var_selector` 值的字体支持的 UVS。
/// UVS 的基本字符存储在偏移量指向的表中。
/// UVS 按它们是默认 UVS 还是非默认 UVS 进行分区。
///
/// 用于非默认 UVS 的字形 ID 在非默认 UVS 表中指定。
///
///
///
pub struct VariationSelector {
    var_selector: U24,
    /// 从`格式 14` 子表的开头到默认 UVS 表的偏移量。可能是 0。
    default_uvs_offset: Offset32,
    /// 从`格式 14` 子表的开头到非默认 UVS 表的偏移量。可能是 0。
    non_default_uvs_offset: Offset32,
}

/// 默认 UVS 表只是 `Unicode` 标量值的范围压缩列表，
/// 表示使用关联 `VariationSelector` 记录的 `var_selector` 的默认 UVS 的基本字符。
pub struct DefaultUVSTable {
    /// Unicode 字符范围的数量。
    num_unicode_value_ranges: u32,
    /// ranges\[num_unicode_value_ranges]
    ///
    /// 每个 `Unicode` 范围记录都指定 `Unicode` 值的连续范围。
    ///
    ranges: Vec<UnicodeRange>,
}
///
/// 例如，范围 `U+4E4D – U+4E4F`（3 个值）会将 `start_unicode_value` 设置为_0_x_004_e_4_d，
/// 将 `additional_count` 设置为_2。单一实例范围会将 `additional_count` 设置为_0。
///
/// 总和 （`start_unicode_value + additional_count`） 不得超过 `0xFFFFFF`。
///
/// `Unicode` 值范围按 `start_unicode_value` 的升序排序。范围不得重叠;
/// 即 （`start_unicode_value + additional_count`）
/// 必须小于以下范围（如果有）的 `start_unicode_value`。
///
/// `ranges` 数组中列出的所有代码点都应该在 `Unicode` `cmap` 子表中具有相应的条目。
/// 但是，应用程序可能会遇到并非如此的字体。
///
pub struct UnicodeRange {
    /// 此范围内的第一个值
    start_unicode_value: U24,
    /// 此范围内的附加值数
    additional_count: u8,
}

/// 非默认 UVS 表是 `Unicode` 标量值和字形 ID 对的列表。
/// `Unicode` 值表示使用关联 `VariationSelector` 记录的 `var_selector` 的
/// 所有非默认 UVS 的基本字符，字形 ID 指定要用于 UVS 的字形 ID。
pub struct NonDefaultUVSTable {
    /// 随后的 UVS 映射数。
    num_uvs_mappings: u32,
    /// uvs_mappings\[num_uvs_mappings]
    uvs_mappings: Vec<UVSMapping>,
}

/// UVS 映射按 `unicode_value` 的升序排序。此表中没有两个映射可以具有相同的 `unicode_value` 值。
///
/// 通常，`uvs_mappings` 数组中列出的代码点在 `Unicode` `cmap` 子表中将具有相应的条目。
/// 但是，这不是必需的。
/// 例如，如果字体旨在用于给定 `Unicode` 字符仅以变体序列出现的内容，
/// 则情况可能并非如此。
pub struct UVSMapping {
    /// UVS 的基本 `Unicode` 值
    unicode_value: U24,
    /// UVS 的字形 `ID`
    glyph_id: u16,
}
