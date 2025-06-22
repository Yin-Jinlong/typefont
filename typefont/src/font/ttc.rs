use super::{Offset32, Tag};

/// # 字体集合文件结构
///
/// 一个字体集合文件由单个TTC头部表、一个或多个表目录（每个对应于不同的字体资源）
/// 和若干OpenType表组成。TTC头部必须位于TTC文件的开头。
///
/// TTC文件必须为每个字体资源包含一个完整的表目录。
/// 在集合文件中的每个字体资源使用与非集合文件中相同的TableDirectory格式。
/// TTC文件内所有表目录中的表偏移量是从TTC文件的开头开始计算的。
///
/// TTC文件中的每个OpenType表通过使用该表的每个字体的表目录进行引用。
/// 一些OpenType表必须针对TTC中包含的每个字体出现一次；
/// 而其他表可能被TTC中的多个字体共享。
///
/// 例如，考虑一个结合了两个日文字体（Font1和Font2）的TTC文件。
/// 这些字体有不同的假名设计（Kana1和Kana2），但使用相同的汉字设计。
/// TTC文件包含一个单独的'glyf'表，其中包括两种假名设计以及汉字；
/// 两个字体的表目录都指向这个'glyf'表。
/// 但是，每个字体的表目录指向不同的'cmap'表，以识别要使用的字形集。
/// Font1的'cmap'表指向用于假名字形的'loca'和'glyf'表的Kana1区域，
/// 并指向汉字区域获取汉字。
/// Font2的'cmap'表指向用于假名字形的'loca'和'glyf'表的Kana2区域，
/// 并且也指向相同的汉字区域获取汉字。
///
/// 对于每个字体应拥有唯一副本的表是那些用于系统在识别字体及其字符映射时使用的表，
/// 包括'cmap'、`'name'`和OS/2。应该在TTC中的字体之间共享的表是那些定义字形
/// 和指令数据或使用字形索引来访问数据的表：
/// 'glyf'、'loca'、'hmtx'、'hdmx'、LTSH、`'cvt '`、`'fpgm'`、`'prep'`、EBLC、EBDT、EBSC、`'maxp'`等。
/// 实际上，任何对于两个或更多字体具有相同数据的表都可以共享。
///
/// 注意：当从单独的字体文件构建集合文件时，
/// 需要密切关注字体内部字形重新编号的问题及其对'cmap'表和其他地方可能产生的副作用。
/// 要合并的字体还需要有兼容的TrueType指令；
/// 也就是说，它们的预编程、函数定义和控制值不能冲突。
///
/// 包含TrueType字形轮廓的集合文件应使用.TTC作为文件扩展名。
/// 包含CFF或CFF2轮廓的集合文件应使用.OTC作为文件扩展名。
pub struct Ttc<H: TtcHeader> {
    header: H,
}

/// `TTC`头部有两个版本：`1.0`版用于没有数字签名的`TTC`文件。
/// `2.0`版可用于带有或不带有数字签名的`TTC`文件——如果没有签名，
/// 则版本`2.0`头部的最后三个字段留空。
///
/// 如果使用数字签名，
/// 则文件的`DSIG表`必须位于`TTC`文件末尾，
/// 位于任何其他字体表之后。
/// `TTC`文件中的签名预期为格式1签名。
///
/// `TTC`头部表的目的在于定位`TTC`文件内不同的表目录。
/// `TTC`头部位于`TTC`文件的开头（偏移量=`0`）。
/// 它由一个识别标签、一个版本号、文件中`OpenType`字体数量的计数
/// 以及指向每个表目录的偏移数组组成。
pub trait TtcHeader {
    fn ttc_tag(&self) -> &Tag;
    fn major_version(&self) -> u16;
    fn minor_version(&self) -> u16;
    fn num_fonts(&self) -> u32;
    fn table_directory_offsets(&self) -> &Vec<Offset32>;

    fn as_v1(&self) -> Option<&TtcV1>;
    fn as_v2(&self) -> Option<&TtcV2>;
}

pub struct TtcV1 {
    ttc_tag: Tag,
    /// 1
    major_version: u16,
    /// 0
    minor_version: u16,
    num_fonts: u32,
    table_directory_offsets: Vec<Offset32>,
}

pub struct TtcV2 {
    ttc_tag: Tag,
    /// 2
    major_version: u16,
    /// 0
    minor_version: u16,
    num_fonts: u32,
    table_directory_offsets: Vec<Offset32>,
    dsig_tag: u32,
    dsig_length: u32,
    dsig_offset: u32,
}

impl TtcHeader for TtcV1 {
    fn ttc_tag(&self) -> &Tag {
        &self.ttc_tag
    }

    fn major_version(&self) -> u16 {
        self.major_version
    }

    fn minor_version(&self) -> u16 {
        self.minor_version
    }

    fn num_fonts(&self) -> u32 {
        self.num_fonts
    }

    fn table_directory_offsets(&self) -> &Vec<Offset32> {
        &self.table_directory_offsets
    }

    fn as_v1(&self) -> Option<&TtcV1> {
        Some(self)
    }

    fn as_v2(&self) -> Option<&TtcV2> {
        None
    }
}

impl AsMut<TtcV1> for TtcV1 {
    fn as_mut(&mut self) -> &mut TtcV1 {
        self
    }
}

impl TtcHeader for TtcV2 {
    fn ttc_tag(&self) -> &Tag {
        &self.ttc_tag
    }

    fn major_version(&self) -> u16 {
        self.major_version
    }

    fn minor_version(&self) -> u16 {
        self.minor_version
    }

    fn num_fonts(&self) -> u32 {
        self.num_fonts
    }

    fn table_directory_offsets(&self) -> &Vec<Offset32> {
        &self.table_directory_offsets
    }

    fn as_v1(&self) -> Option<&TtcV1> {
        None
    }

    fn as_v2(&self) -> Option<&TtcV2> {
        Some(self)
    }
}

impl AsMut<TtcV2> for TtcV2 {
    fn as_mut(&mut self) -> &mut TtcV2 {
        self
    }
}
