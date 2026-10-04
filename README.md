# corpus_matrix

`corpus_matrix` builds numeric corpus matrices from a token stream and a
`token_db::TokenDb`.

The crate is deliberately small. It provides the matrix-building step between
tokenized corpus data and later numerical processing. It does not perform PDF
extraction, tokenization, SVD, or search.

## Matrix types

### Count

`MatrixType::Count { window_size }` builds a symmetric token co-occurrence
matrix backed by `simplefield::Field<RowMajor, u64>`.

A window starts at each token position. The first token is the reference token
and is paired once with every following token inside the window. The window
then advances by one position. Each positional pair is therefore counted once.
Equal token IDs at different positions are allowed and can produce non-zero
diagonal entries.

### PPMI

`MatrixType::Ppmi { window_size }` first builds the same co-occurrence counts
and converts them to positive pointwise mutual information (PPMI):

```text
PPMI(i,j) = max(0, ln(count(i,j) * total / (row(i) * column(j))))
```

Zero counts and non-positive PMI values become zero. PPMI matrices use
`simplefield::Field<RowMajor, f64>`.

## Design goals

- **Simplicity.** Keep the crate focused on constructing corpus matrices. Prefer
  small, explicit implementations over abstractions that are not needed yet.
- **Performance.** Use compact contiguous matrix storage and straightforward
  loops. Avoid unnecessary dynamic dispatch and intermediate representations.
- **Clear separation of matrix types.** Shared validation and dispatch live in
  `matrix_builder.rs`; each matrix type has its own implementation module
  below `matrix_builder/`.
- **Stable token indexing.** Matrix rows and columns correspond directly to
  `TokenId` values from the supplied `TokenDb`.
- **Consistent test layout.** Tests live under `tests/`, mirror the relative
  `src/` hierarchy, and use the source filename with a `_test.rs` suffix.

The source and test layout therefore correspond directly:

```text
src/
├── matrix_builder.rs
└── matrix_builder/
    ├── count.rs
    └── ppmi.rs

tests/
├── matrix_builder_test.rs
└── matrix_builder/
    ├── count_test.rs
    └── ppmi_test.rs
```

## API sketch

```rust
use corpus_matrix::{Matrix, MatrixBuilder, MatrixType};

let builder = MatrixBuilder::new(&token_db, &tokens);

let Matrix::Ppmi(matrix) = builder
    .build(MatrixType::Ppmi { window_size: 5 })?
else {
    unreachable!();
};
```

`MatrixBuilder` validates that all token IDs belong to the index range of the
supplied token database before constructing a matrix.

## Development

The repository contains a `Justfile`. The default recipe runs the local
build, tests, and Clippy checks:

```sh
just
```

GitHub CI runs `cargo build`, `cargo test`, and
`cargo clippy -- -D warnings` on pushes and pull requests.

## License

Apache License 2.0.
