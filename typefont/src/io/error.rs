#[derive(Debug)]
pub enum IOError {
    BadFormat(String),
    NoEnoughData,
    UnexpectedEof,
    UnableCast,
    UnableOperate(String),
}
