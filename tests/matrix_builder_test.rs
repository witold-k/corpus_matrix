// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use retrieval::{Error, MatrixBuilder, MatrixType};
use token_db::TokenDb;

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
