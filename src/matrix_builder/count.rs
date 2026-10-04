// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::{CountMatrix, Error, Result};
use simplefield::field::Field;
use simplefield::orientation::RowMajor;
use token_db::{TokenDb, TokenId};

pub(crate) fn build(
    token_db: &TokenDb,
    tokens: &[TokenId],
    window_size: usize,
) -> Result<CountMatrix> {
    if window_size == 0 {
        return Err(Error::InvalidWindowSize);
    }

    let size = token_db.len();
    let mut values = vec![0_u64; size * size];

    for left in 0..tokens.len() {
        let end = left.saturating_add(window_size).min(tokens.len());

        for right in (left + 1)..end {
            let row = tokens[left].get() as usize;
            let column = tokens[right].get() as usize;

            increment(&mut values, size, row, column)?;
            if row != column {
                increment(&mut values, size, column, row)?;
            }
        }
    }

    Ok(Field::<RowMajor, u64>::new_data(size, size, values))
}

fn increment(values: &mut [u64], size: usize, row: usize, column: usize) -> Result<()> {
    let value = &mut values[row * size + column];
    *value = value.checked_add(1).ok_or(Error::CountOverflow)?;
    Ok(())
}
