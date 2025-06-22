#[derive(Debug)]
pub enum IOError {
    NoEnoughData,
    UnexpectedEof,
    UnableCast,
    UnableOperate(String),
}
