pub mod error;
pub mod matrix;
pub mod matrix_builder;

pub use error::{Error, Result};
pub use matrix::{CountMatrix, Matrix};
pub use matrix_builder::{MatrixBuilder, MatrixType};
