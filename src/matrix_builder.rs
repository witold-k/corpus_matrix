// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::{CountMatrix, Error, Matrix, PpmiMatrix, Result};
use simplefield::field::Field;
use simplefield::orientation::RowMajor;
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
                self.build_count_matrix(window_size).map(Matrix::Count)
            }
            MatrixType::Ppmi { window_size } => {
                self.build_ppmi_matrix(window_size).map(Matrix::Ppmi)
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

    fn build_count_matrix(&self, window_size: usize) -> Result<CountMatrix> {
        if window_size == 0 {
            return Err(Error::InvalidWindowSize);
        }

        let size = self.token_db.len();
        let mut values = vec![0_u64; size * size];

        for left in 0..self.tokens.len() {
            let end = left.saturating_add(window_size).min(self.tokens.len());

            for right in (left + 1)..end {
                let row = self.tokens[left].get() as usize;
                let column = self.tokens[right].get() as usize;

                increment(&mut values, size, row, column)?;
                if row != column {
                    increment(&mut values, size, column, row)?;
                }
            }
        }

        Ok(Field::<RowMajor, u64>::new_data(size, size, values))
    }

    fn build_ppmi_matrix(&self, window_size: usize) -> Result<PpmiMatrix> {
        let counts = self.build_count_matrix(window_size)?;
        let size = counts.row_count();
        let count_values = counts.get_data();

        let mut marginals = vec![0.0_f64; size];
        let mut total = 0.0_f64;

        for row in 0..size {
            for column in 0..size {
                let count = count_values[row * size + column] as f64;
                marginals[row] += count;
                total += count;
            }
        }

        let mut values = vec![0.0_f64; size * size];

        if total > 0.0 {
            for row in 0..size {
                for column in 0..size {
                    let count = count_values[row * size + column] as f64;
                    if count == 0.0 {
                        continue;
                    }

                    let denominator = marginals[row] * marginals[column];
                    if denominator == 0.0 {
                        continue;
                    }

                    let pmi = (count * total / denominator).ln();
                    values[row * size + column] = pmi.max(0.0);
                }
            }
        }

        Ok(Field::<RowMajor, f64>::new_data(size, size, values))
    }
}

fn increment(values: &mut [u64], size: usize, row: usize, column: usize) -> Result<()> {
    let value = &mut values[row * size + column];
    *value = value.checked_add(1).ok_or(Error::CountOverflow)?;
    Ok(())
}
