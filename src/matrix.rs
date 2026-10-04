// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use simplefield::field::Field;
use simplefield::orientation::RowMajor;

pub type CountMatrix = Field<RowMajor, u64>;

#[derive(Debug, Clone)]
pub enum Matrix {
    Count(CountMatrix),
}
