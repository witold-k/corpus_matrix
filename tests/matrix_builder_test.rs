use corpus_matrix::{Error, Matrix, MatrixBuilder, MatrixType};
use token_db::TokenDb;

fn token_db() -> TokenDb {
    let mut db = TokenDb::new();
    db.insert("a").unwrap();
    db.insert("b").unwrap();
    db.insert("c").unwrap();
    db
}

#[test]
fn builds_symmetric_sliding_window_counts() {
    let db = token_db();
    let tokens = [0, 1, 2, 1];
    let builder = MatrixBuilder::new(&db, &tokens);

    let Matrix::Count(matrix) = builder
        .build(MatrixType::Count { window_size: 3 })
        .unwrap();

    assert_eq!(matrix.size(), 3);
    assert_eq!(matrix.get(0, 1), Some(1));
    assert_eq!(matrix.get(1, 0), Some(1));
    assert_eq!(matrix.get(0, 2), Some(1));
    assert_eq!(matrix.get(2, 0), Some(1));
    assert_eq!(matrix.get(1, 2), Some(2));
    assert_eq!(matrix.get(2, 1), Some(2));
    assert_eq!(matrix.get(1, 1), Some(1));
}

#[test]
fn rejects_token_ids_outside_database() {
    let db = token_db();
    let tokens = [0, 3];
    let builder = MatrixBuilder::new(&db, &tokens);

    assert_eq!(
        builder.build(MatrixType::Count { window_size: 2 }),
        Err(Error::InvalidTokenId(3))
    );
}

#[test]
fn rejects_zero_window_size() {
    let db = token_db();
    let tokens = [0, 1];
    let builder = MatrixBuilder::new(&db, &tokens);

    assert_eq!(
        builder.build(MatrixType::Count { window_size: 0 }),
        Err(Error::InvalidWindowSize)
    );
}
