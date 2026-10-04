// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use corpus_matrix::{Matrix, MatrixBuilder, MatrixType, PpmiMatrix};
use token_db::TokenDb;

fn token_db() -> TokenDb {
    let mut db = TokenDb::new();
    db.insert("a").unwrap();
    db.insert("b").unwrap();
    db.insert("c").unwrap();
    db
}

fn value(matrix: &PpmiMatrix, row: usize, column: usize) -> f64 {
    matrix.get_data()[row * matrix.column_count() + column]
}

#[test]
fn builds_ppmi_from_count_matrix() {
    let db = token_db();
    let a = db.id("a").unwrap();
    let b = db.id("b").unwrap();
    let c = db.id("c").unwrap();
    let tokens = [a, b, c, b];
    let builder = MatrixBuilder::new(&db, &tokens);

    let Matrix::Ppmi(matrix) = builder
        .build(MatrixType::Ppmi { window_size: 3 })
        .unwrap()
    else {
        panic!("expected PPMI matrix");
    };

    let expected_ab = (9.0_f64 / 8.0).ln();
    let expected_ac = (3.0_f64 / 2.0).ln();

    assert!((value(&matrix, 0, 1) - expected_ab).abs() < 1.0e-12);
    assert!((value(&matrix, 1, 0) - expected_ab).abs() < 1.0e-12);
    assert!((value(&matrix, 0, 2) - expected_ac).abs() < 1.0e-12);
    assert!((value(&matrix, 1, 2) - expected_ac).abs() < 1.0e-12);
    assert_eq!(value(&matrix, 1, 1), 0.0);
}

#[test]
fn empty_cooccurrence_matrix_produces_zero_ppmi() {
    let db = token_db();
    let a = db.id("a").unwrap();
    let tokens = [a];
    let builder = MatrixBuilder::new(&db, &tokens);

    let Matrix::Ppmi(matrix) = builder
        .build(MatrixType::Ppmi { window_size: 3 })
        .unwrap()
    else {
        panic!("expected PPMI matrix");
    };

    assert!(matrix.get_data().iter().all(|&value| value == 0.0));
}
