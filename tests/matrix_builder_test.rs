// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use corpus_matrix::{Error, Matrix, MatrixBuilder, MatrixType};
use token_db::TokenDb;

fn token_db() -> TokenDb {
    let mut db = TokenDb::new();
    db.insert("a").unwrap();
    db.insert("b").unwrap();
    db.insert("c").unwrap();
    db
}

fn value(matrix: &corpus_matrix::CountMatrix, row: usize, column: usize) -> u64 {
    matrix.get_data()[row * matrix.column_count() + column]
}

fn ppmi_value(matrix: &corpus_matrix::PpmiMatrix, row: usize, column: usize) -> f64 {
    matrix.get_data()[row * matrix.column_count() + column]
}

#[test]
fn counts_each_cooccurring_position_pair_once() {
    let db = token_db();
    let a = db.id("a").unwrap();
    let b = db.id("b").unwrap();
    let c = db.id("c").unwrap();
    let tokens = [a, b, c, b];
    let builder = MatrixBuilder::new(&db, &tokens);

    let Matrix::Count(matrix) = builder
        .build(MatrixType::Count { window_size: 3 })
        .unwrap()
    else {
        panic!("expected count matrix");
    };

    assert_eq!(matrix.row_count(), 3);
    assert_eq!(matrix.column_count(), 3);
    assert_eq!(value(&matrix, 0, 1), 1);
    assert_eq!(value(&matrix, 1, 0), 1);
    assert_eq!(value(&matrix, 0, 2), 1);
    assert_eq!(value(&matrix, 2, 0), 1);
    assert_eq!(value(&matrix, 1, 2), 2);
    assert_eq!(value(&matrix, 2, 1), 2);
    assert_eq!(value(&matrix, 1, 1), 1);
}

#[test]
fn each_window_pairs_only_its_reference_token_with_following_tokens() {
    let db = token_db();
    let a = db.id("a").unwrap();
    let b = db.id("b").unwrap();
    let c = db.id("c").unwrap();
    let tokens = [a, b, c, b];
    let builder = MatrixBuilder::new(&db, &tokens);

    let Matrix::Count(matrix) = builder
        .build(MatrixType::Count { window_size: 3 })
        .unwrap()
    else {
        panic!("expected count matrix");
    };

    // [a, b, c] contributes a-b and a-c from reference a.
    // [b, c, b] contributes b-c and b-b from reference b.
    // [c, b] contributes c-b from reference c.
    assert_eq!(value(&matrix, 0, 1), 1);
    assert_eq!(value(&matrix, 0, 2), 1);
    assert_eq!(value(&matrix, 1, 2), 2);
    assert_eq!(value(&matrix, 1, 1), 1);
}

#[test]
fn rejects_token_ids_outside_database() {
    let mut db = TokenDb::new();
    let valid = db.insert("a").unwrap();

    let mut other_db = TokenDb::new();
    other_db.insert("a").unwrap();
    let invalid = other_db.insert("b").unwrap();

    let tokens = [valid, invalid];
    let builder = MatrixBuilder::new(&db, &tokens);

    assert!(matches!(
        builder.build(MatrixType::Count { window_size: 2 }),
        Err(Error::InvalidTokenId(id)) if id == invalid
    ));
}

#[test]
fn rejects_zero_window_size() {
    let db = token_db();
    let a = db.id("a").unwrap();
    let b = db.id("b").unwrap();
    let tokens = [a, b];
    let builder = MatrixBuilder::new(&db, &tokens);

    assert!(matches!(
        builder.build(MatrixType::Count { window_size: 0 }),
        Err(Error::InvalidWindowSize)
    ));
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

    assert!((ppmi_value(&matrix, 0, 1) - expected_ab).abs() < 1.0e-12);
    assert!((ppmi_value(&matrix, 1, 0) - expected_ab).abs() < 1.0e-12);
    assert!((ppmi_value(&matrix, 0, 2) - expected_ac).abs() < 1.0e-12);
    assert!((ppmi_value(&matrix, 1, 2) - expected_ac).abs() < 1.0e-12);
    assert_eq!(ppmi_value(&matrix, 1, 1), 0.0);
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
