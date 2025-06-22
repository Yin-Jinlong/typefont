use super::table_record::TableRecord;
use crate::font::io::ReadFrom;
use crate::font::table::base::BASE;
use crate::font::table::{Table, WithTag};
use crate::io::array_reader::ArrayReader;
use crate::io::error::IOError;
use crate::io::file_reader::FileReader;
use crate::io::reader::{Reader, ReaderBoxed};

const SFNT_TTF: u32 = 0x00010000;
const SFNT_OTF: u32 = 0x4F54544F;

/// # 表目录
///
/// `OpenType` 格式的一个关键特性是 `TrueType` `sfnt` “包装器”，
/// 它以一种通用且可扩展的方式为一系列表提供组织结构。
///
/// `OpenType` 字体从表目录开始，该目录是字体中顶级表的目录。
/// 如果字体文件仅包含一个字体，则表目录将从文件的第0字节开始。
/// 如果字体文件是一个 OpenType 字体集合文件（见下文），
/// 每个字体的表目录起始点会在 `TTCHeader` 中标明。
///
/// 包含`TrueType`轮廓的`OpenType`字体应当为`sfntVersion`使用值`0x00010000`。
/// 而包含CFF数据（版本1或2）的`OpenType`字体则应使用`0x4F54544F`
/// （当重新解释为标签时为`'OTTO'`）作为`sfnt_version`。
///
/// 表目录格式允许存在大量的表格。为了辅助快速二分查找，
/// 包含了`search_range`、`entry_selector`和`range_shift`字段作为参数，可用于配置搜索算法。
/// 当条目数量是_2_的幂时，二分查找是最优的。
/// `search_range`字段提供了在此约束下可搜索的最大项目数（最大_2_的幂）。
/// `range_shift`字段提供了还需搜索的剩余项目数。
/// `entry_selector`字段指示了进入二叉树所需的最大层级数。
/// 这些值乘以_16，即每个`table_record`的大小。
///
/// 在硬件能力有限设备的早期实现中，
/// 由`search_range`、`entry_selector`和`range_shift`字段提供的优化具有高度重要性。
/// 在现代设备上它们的重要性降低，但可能仍在某些实现中使用。
/// 然而，不正确的值可能被用作针对某些实现的攻击向量。
/// 由于这些值可以在解析文件时从`num_tables`字段推导出来，
/// 强烈建议解析实现不要依赖字体中的`search_range`、`entry_selector`和`range_shift`字段，
/// 而是独立于`num_tables`推导出它们。
/// 然而，字体文件应继续为这些字段提供有效的值，
/// 以保持与所有现有实现的兼容性。
///
/// 表目录以`tableRecords`数组结束。
pub struct TableDirectory {
    /// `0x00010000` 或 `0x4F54544F`（`OTTO`）
    sfnt_version: u32,
    num_tables: u16,
    /// `(2**floor(log2(numTables)) * 16`，其中 “**” 是幂运算符）
    search_range: u16,
    /// `(log2(searchRange/16)`等价于 `floor(log2(numTables))`
    entry_selector: u16,
    /// `(numTables * 16) - searchRange`
    range_shift: u16,
    table_records: Vec<TableRecord>,
}

/// # OpenType字体文件
///
/// **2024/05/29**
///
/// `OpenType`字体文件以表格格式包含用于文本渲染的数据。
/// 这些数据的部分内容被应用程序用来计算使用该字体的文本布局；
/// 即，字形的选择及其在一行内的放置。
/// 其他数据则提供了作为`TrueType`或紧凑字体格式（CFF）轮廓的字形描述。
/// 还有其他数据可以提供单色或彩色位图、SVG文档或二进制2D矢量图形组合作为替代字形描述。
/// 字体数据还包括元信息，例如可以在字体选择用户界面中用作展示的名称字符串。
/// 每种类型的数据都存储在一个或多个表中，每个表都针对特定目的设计。
///
/// ## 文件名
///
/// `OpenType`字体文件可能具有`.OTF`、`.TTF`、`.OTC`或`.TTC`的扩展名。
/// （扩展名可以是大写或小写。）`.OTC`和`.TTC`扩展名仅应被用于字体集合文件。
///
/// ## 表版本号
///
/// 大多数表都有版本号，整个字体的版本号包含在表目录中。
/// 请注意，存在五种不同的表版本号类型，每种类型都有其自己的编号方案。
///
/// - 单个 `uint16` 字段。这被用于许多表中，通常版本从零（`0`）开始。
/// - 分开的 `uint16` 主版本和次版本字段。这也被用于许多表中，通常版本从 `1.0` 开始。
/// - 具有枚举值的 `uint32` 字段。
/// - 具有数值的 `uint32` 字段。这仅用于 `DSIG` 和 `'meta'` 表。
/// - `Version16Dot16` 字段用于主/次版本号。这仅用于 `'maxp'`, `'post'` 和 `'vhea'` 表。
///
/// `Version16Dot16` 类型用于某些表的版本字段，且仅出于向后兼容的原因。
/// （在早期版本中，这些字段被记录为使用 `Fixed` 值，但其小版本号并不遵循 `Fixed` 类型的定义。）
/// `Version16Dot16` 是一个打包值：高16位组成主版本号，低16位组成次版本号。
/// 非零次版本号使用低16位的最高有效半字节中的数字 0 到 9 表示。
/// 例如，`'maxp'` 表版本 0.5 的版本字段是 `0x00005000`，
/// 而 `'vhea'` 表版本 1.1 的版本字段是 `0x00011000`。
/// 此类型仅用于 `'maxp'`, `'post'` 和 `'vhea'` 表，并且将来不会用于其他任何表。
///
/// 表目录使用一个 `uint32` 字段 `sfnt_version`，它有一个已定义值的枚举，
/// 其中一些表示四字符标签；
///
/// 读取表的实现必须包含检查版本号的代码，
/// 以便当格式和版本号发生变化时，旧的实现能够优雅地处理新版本。
///
/// 次版本号的变化总是意味着兼容扩展的格式变化。
/// 如果实现理解了主版本号，则可以安全地继续读取表。
/// 如果次版本号大于实现所识别的最新版本，则实现将无法检测到扩展字段。
///
/// 为了兼容性起见，使用单个 `uint16` 或 `uint32` 值表示的版本号被视为次版本号，
/// 任何格式变化都是兼容扩展。
///
/// 请注意，在次要版本更改中，以前未定义或保留的某些字段值可能会被赋予意义。
/// 实现不应对接口或未分配的值或位域中的保留位做出假设，并可以在遇到它们时忽略它们。
/// 在写入字体数据时，工具应始终为保留字段或位写入零。
/// 验证器应对针对给定版本验证的数据中未定义的字段或位的任何非零值发出警告。
///
/// 如果主版本号不被识别，实现不得读取该表，因为它不能对二进制数据的解释做出任何假设。
/// 实现应将该表视为缺失。
///
/// ## 字体集合
///
/// `OpenType`字体集合（`TTC`或`OTC`，以前称为`TrueType Collectio`n）
/// 是一种以单一文件结构提供多个OpenType字体资源的方法。
/// 字体集合格式允许两个或更多字体之间相同的表格共享。
/// 当要一起交付的字体共享许多共同的字形时，
/// 包含轮廓字形数据（`TrueType`、`CFF`、`CFF2`或`SVG`）的字体集合非常有用。
/// 通过允许多个字体共享字形集和其他常见字体表格，
/// 字体集合可以显著节省文件空间。
///
/// 例如，一组日文字体可能各自拥有自己设计的假名字形，
/// 但共享相同设计的汉字字形。使用普通的`OpenType`字体文件，
/// 包含共同汉字字形的唯一方法是将它们的字形数据复制到每个字体中。
/// 由于汉字比假名占据更多的数据量，这会导致大量的字形数据重复浪费。
/// 定义字体集合正是为了解决这个问题。
///
/// 注意：尽管字体集合的原始定义（作为`TrueType`规范的一部分）
/// 旨在与包含`TrueType`轮廓的字体一起使用，
/// 并且在早期的`OpenType`版本中保持了这一限制，
/// 但在`OpenType`中这已不再是限制。
/// 字体集合文件可以包含各种类型的轮廓（或它们的混合），
/// 无论字体是否包含布局表格。为了向后兼容和简单起见，
/// 描述字体集合文件结构时使用术语“TrueType Collection”或“TTC”，
/// 虽然理解它是用来标识包含任何类型轮廓表格的通用字体集合结构。
///
/// 注意：`OpenType`可变字体在功能上等同于多个非可变字体。
/// 可变字体不需要包含在集合文件内。
/// 然而，一个集合文件可以包括一个甚至多个可变字体，
/// 并且可以结合可变字体和非可变字体。
///
pub struct OpenType {
    pub table_directory: TableDirectory,
    pub tables: Vec<Table>,
}

impl ReadFrom<ReaderBoxed> for TableDirectory {
    fn read_from(reader: &mut ReaderBoxed) -> Result<Self, IOError> {
        let sfnt = reader.read_u32()?;
        if sfnt != SFNT_TTF && sfnt != SFNT_OTF {
            return Err(IOError::BadFormat(format!(
                "Expected 0x00010000 or 0x4F54544F , found {:X}",
                sfnt
            )));
        }
        let num_tables = reader.read_u16()?;
        let search_range = reader.read_u16()?;
        let entry_selector = reader.read_u16()?;
        let range_shift = reader.read_u16()?;
        let mut table_records: Vec<TableRecord> = vec![];

        for _ in 0..num_tables {
            table_records.push(TableRecord::read_from(reader)?);
        }

        Ok(Self {
            sfnt_version: sfnt,
            num_tables,
            search_range,
            entry_selector,
            range_shift,
            table_records,
        })
    }
}

impl ReadFrom<ReaderBoxed> for OpenType {
    fn read_from(reader: &mut ReaderBoxed) -> Result<Self, IOError> {
        let table_directory = TableDirectory::read_from(reader)?;
        let mut tables: Vec<Table> = vec![];

        for record in &table_directory.table_records {
            reader.seek(record.offset as usize)?;
            let data = reader.read_bytes(record.length as usize)?;
            let mut sub_reader: ReaderBoxed = Box::new(ArrayReader::from(data));
            match record.table_tag.to_u32() {
                BASE::TAG_U32 => tables.push(Table::BASE(BASE::read_from(&mut sub_reader)?)),

                _ => {}
            }
        }

        Ok(Self {
            table_directory,
            tables,
        })
    }
}

pub fn read_font(path: &str) -> Result<OpenType, IOError> {
    let mut reader: ReaderBoxed = Box::new(match FileReader::open(path) {
        Ok(reader) => reader,
        Err(e) => {
            return Err(IOError::UnableOperate(e.to_string()));
        }
    });
    OpenType::read_from(&mut reader)
}
