// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::{PpmiMatrix, Result};
use simplefield::field::Field;
use simplefield::orientation::RowMajor;
use token_db::{TokenDb, TokenId};

use super::count;

pub(crate) fn build(
    token_db: &TokenDb,
    tokens: &[TokenId],
    window_size: usize,
) -> Result<PpmiMatrix> {
    let counts = count::build(token_db, tokens, window_size)?;
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
