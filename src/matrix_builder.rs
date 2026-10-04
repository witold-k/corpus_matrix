// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

mod count;
mod ppmi;

use crate::{Error, Matrix, Result};
use token_db::{TokenDb, TokenId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatrixType {
    /// Builds a symmetric token co-occurrence count matrix.
    ///
    /// A window starts at each token position and contains at most
    /// `window_size` consecutive positions. The first token in the window is
    /// the reference token. It is paired once with every following token in
    /// that window. The window then advances by one position.
    ///
    /// Consequently, two token positions co-occur when their distance is
    /// smaller than `window_size`. Each pair of positions is counted exactly
    /// once, and all distances within the window have equal weight.
    ///
    /// Equal token IDs at different positions are counted, so diagonal entries
    /// may be non-zero.
    ///
    /// This is intentionally a simple baseline heuristic intended for later
    /// weighting, such as PPMI.
    Count { window_size: usize },

    /// Builds a positive pointwise mutual information matrix from the
    /// co-occurrence counts produced with the same `window_size` semantics as
    /// the count matrix.
    ///
    /// For a non-zero count `c(i,j)`, the matrix value is
    /// `max(0, ln(c(i,j) * total / (row(i) * column(j))))`.
    /// Zero counts and non-positive PMI values become zero.
    Ppmi { window_size: usize },
}

pub struct MatrixBuilder<'a> {
    token_db: &'a TokenDb,
    tokens: &'a [TokenId],
}

impl<'a> MatrixBuilder<'a> {
    #[must_use]
    pub const fn new(token_db: &'a TokenDb, tokens: &'a [TokenId]) -> Self {
        Self { token_db, tokens }
    }

    pub fn build(&self, matrix_type: MatrixType) -> Result<Matrix> {
        self.validate_tokens()?;

        match matrix_type {
            MatrixType::Count { window_size } => {
                count::build(self.token_db, self.tokens, window_size).map(Matrix::Count)
            }
            MatrixType::Ppmi { window_size } => {
                ppmi::build(self.token_db, self.tokens, window_size).map(Matrix::Ppmi)
            }
        }
    }

    fn validate_tokens(&self) -> Result<()> {
        let token_count = self.token_db.len();
        for &id in self.tokens {
            if id.get() as usize >= token_count {
                return Err(Error::InvalidTokenId(id));
            }
        }
        Ok(())
    }
}
