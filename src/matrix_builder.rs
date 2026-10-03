use crate::{CountMatrix, Error, Matrix, Result};
use lineariterator::slice_ref_iterator::SliceRefIterator;
use token_db::TokenDb;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatrixType {
    Count { window_size: usize },
}

pub struct MatrixBuilder<'a> {
    token_db: &'a TokenDb,
    tokens: &'a [u32],
}

impl<'a> MatrixBuilder<'a> {
    #[must_use]
    pub const fn new(token_db: &'a TokenDb, tokens: &'a [u32]) -> Self {
        Self { token_db, tokens }
    }

    pub fn build(&self, matrix_type: MatrixType) -> Result<Matrix> {
        self.validate_tokens()?;

        match matrix_type {
            MatrixType::Count { window_size } => {
                self.build_count_matrix(window_size).map(Matrix::Count)
            }
        }
    }

    fn validate_tokens(&self) -> Result<()> {
        let token_count = self.token_db.len();
        for &id in self.tokens {
            if id as usize >= token_count {
                return Err(Error::InvalidTokenId(id));
            }
        }
        Ok(())
    }

    fn build_count_matrix(&self, window_size: usize) -> Result<CountMatrix> {
        if window_size == 0 {
            return Err(Error::InvalidWindowSize);
        }

        let mut matrix = CountMatrix::new(self.token_db.len());

        for window in SliceRefIterator::new(self.tokens, window_size) {
            for left in 0..window.len() {
                for right in (left + 1)..window.len() {
                    let row = window[left] as usize;
                    let column = window[right] as usize;

                    matrix.increment(row, column)?;
                    if row != column {
                        matrix.increment(column, row)?;
                    }
                }
            }
        }

        Ok(matrix)
    }
}
