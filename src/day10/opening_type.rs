/// The opening type of the token, which can either be opening or closing.
/// This is preferred over just a boolean as it'll be more readable at the call
/// site and wherever it's referenced.
#[derive(Debug, PartialEq)]
pub enum OpeningType {
    Opening,
    Closing,
}
