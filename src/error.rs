use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Unmatched opening bracket")]
    UnmatchedOpeningBracket,
    #[error("Unmatched closing bracket")]
    UnmatchedClosingBracket,
}
