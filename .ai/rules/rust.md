# Rust coding guidelines

These are Rust-specific guidelines, extending and taking precedence over common rules.

## Idiomatic Rust and Tooling

- **Rust-C-1 (SHOULD)** Avoid unnecessary heap allocations and cloning; prefer borrowed slices (`&str`, `&[u8]`) where ownership is not required.
- **Rust-C-2 (SHOULD)** Use idiomatic pattern matching and combinators (`map`, `and_then`, `unwrap_or_else`) instead of manual conditional checks on `Option` and `Result`.
