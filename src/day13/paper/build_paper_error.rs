/// The kind of error that can occur when trying to build a new
/// sheet of paper.
#[derive(Debug, PartialEq)]
pub enum BuildPaperError {
    InvalidHeight,
    InvalidWidth,
    NonEqualWidth,
}
