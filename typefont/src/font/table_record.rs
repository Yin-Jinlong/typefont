use super::{Offset32, Tag};
use crate::font::io::ReadFrom;
use crate::io::error::IOError;
use crate::io::reader::ReaderBoxed;

/// 表标签是赋予`OpenType`字体文件中表格的名称。
/// `table_records`数组使得给定字体只包含它实际需要的那些表格成为可能。
/// 因此，`num_tables`没有标准值。数组中的记录必须按标签以升序排序。
///
/// 对于本规范中定义的表格标签，字体资源应最多包含一个使用给定标签的表格记录。
/// 如果字体资源确实包含多于一个给定类型的表格，
/// 则行为不可预测：应用程序或平台可能会任意选择其中一个表格，或者可能拒绝该字体为无效。
///
/// 平台供应商可以定义额外的表格和关联的标签以提供特定于平台的功能。
/// 例如，参见苹果的TrueType参考手册，其定义了各种在`OpenType`中未定义的关联标签的表格。
/// 一些字体开发工具也可能定义特殊的表格。如果满足本规范的要求，
/// 包含此类额外表格的字体仍然可以作为`OpenType`字体。
/// 对于在本规范之外定义的自定义表格，
/// 这种表格的外部规范可能允许单个字体资源内存在多个该类型的表格。
///
/// 所有表格必须以四字节边界开始，且表格之间剩余的空间必须用零填充。
/// 每个表格的长度应在表格记录中记录为实际数据长度，而不是填充后的长度。
///
/// 某些表格具有带有位于指定偏移量的子表格的内部结构，因此，
/// 可以构造一个数据在不同表格间交错的字体。
/// 通常，表格应连续排列而不重叠各个独立表格的范围。
/// 然而，在任何情况下，表格长度测量的是包含一个表格所有数据的连续字节范围。
/// 这同样适用于任何子表格以及顶级表格。
///
pub struct TableRecord {
    /// 表标识
    table_tag: Tag,
    /// 此表的校验和
    checksum: u32,
    /// 字体文件开头的偏移量。
    offset: Offset32,
    /// 此表的长度。
    length: u32,
}

/// `head`表格在校验和计算中是一个特例，
/// 因为它包含一个`checksum_adjustment`字段，
/// 该字段是在表格的校验和被计算并写入表格目录条目之后计算并写入的，
/// 因此必然会使该表格的校验和值失效。
///
/// 在生成字体数据时，为了计算并写入`head`表格的校验和及`checksum_adjustment`字段，
/// 请执行以下步骤：
///
/// - 将`checksum_adjustment`字段设置为0。
/// - 计算所有表格（包括`head`表格）的校验和，并将每个表格的值输入到表格目录中的相应记录中。
/// - 计算整个字体的校验和。
/// - 从`0xB1B0AFBA`中减去该值。
/// - 将结果存储在`head`表格的`checksum_adjustment`字段中。
///
/// 尝试验证`head`表格是否发生变化的应用程序应在校验该表格时假设`checksum_adjustment`值为零，
/// 而不是字体中的实际值，然后再将结果与表格目录中的`head`表格记录进行比较。
///
/// 在字体集合文件中，表格校验和必须反映这些表格在集合文件中的状态。
/// `head`表格中的`checksumAdjustment`字段不用于集合文件中，可以设置为零。
pub fn calc_table_checksum(table: &Vec<u32>) -> u32 {
    let mut sum = 0u32;
    for v in table {
        sum += v;
    }
    sum
}

impl ReadFrom<ReaderBoxed> for TableRecord {
    fn read_from(reader: &mut ReaderBoxed) -> Result<Self, IOError> {
        Ok(Self {
            table_tag: Tag::read_from(reader)?,
            checksum: reader.read_u32()?,
            offset: reader.read_u32()?,
            length: reader.read_u32()?,
        })
    }
}
