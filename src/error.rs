use std::fmt;
use token_db::TokenId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidTokenId(TokenId),
    InvalidWindowSize,
    CountOverflow,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTokenId(id) => write!(f, "token ID {} is outside the token database", id.get()),
            Self::InvalidWindowSize => write!(f, "window size must be greater than zero"),
            Self::CountOverflow => write!(f, "matrix count overflow"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
