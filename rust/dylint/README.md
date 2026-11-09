# Custom dylint lints

[Dylint](https://github.com/tbrand/dylint) libraries implementing the repo rules
that clippy/rustc cannot express. One crate per lint under `lints/`, so individual
lints can be synced separately into other repositories.

## Lints

| Lint crate | Rule | Level |
|---|---|---|
| `minimal_main_rs` | Rust-A-1 (MUST): `main.rs` only parses CLI args, init logging, delegates | warn |
| `no_tuple_in_public_signatures` | Rust-A-2 (MUST): structs instead of tuples in public signatures | warn |
| `typed_domain_error` | Rust-ERR-2 (MUST): typed domain errors (thiserror), not `Box<dyn Error>`/`String` | warn |
| `main_returns_exit_code` | Rust-ERR-3 (MUST): `main` returns `std::process::ExitCode` | warn |
| `no_std_env_args` | Rust-CLI-2 (SHOULD): clap derive instead of `std::env::args` | warn |
| `hermetic_inline_tests` | Rust-T-1 + Commons-T-2 (MUST): inline unit tests do no fs/net/process I/O | warn |
| `external_io_in_client_module` | Commons-EXT-1 (MUST): external calls centralized in a client module | warn |
| `no_static_mut` | Commons-FN-3 (SHOULD): no mutable global state | warn |
| `prefer_assert_eq` | Rust-T-2 (SHOULD): `assert_eq!`/`assert_ne!` over `assert!(a == b)` | warn |
| `single_assert_per_test` | Commons-UT-1 (SHOULD): one assertion per unit test | warn |

All lints are `Warn` by default; they become blocking when run with
`-D warnings` (see below), consistent with the zero-warning policy (Commons-L-1).

## Build & run

Dylint libraries must be compiled with the exact nightly toolchain supported by
the pinned `dylint_linting` version — `rust-toolchain.toml` in this directory pins
it. Keep it in sync with `dylint_linting` in `lints/*/Cargo.toml`.

From the crate to check:

```sh
cargo install cargo-dylint --locked
cargo dylint --path ./rust/dylint --all -- --all-targets -D warnings
# or a single lint:
cargo dylint --path ./rust/dylint --lib minimal_main_rs -- --all-targets -D warnings
```

## Adding a lint

```sh
cargo new --lib lints/my_lint
```

Then in `lints/my_lint/Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
dylint_linting = "3"
```

and implement it in `src/lib.rs` following the pattern of the existing lints
(`dylint_linting::dylint_library!();` + `declare_lint!` + `impl_late_lint!`).
Note: the macro-based lints (assertion counting, `assert!(a == b)` detection) use
source-snippet heuristics; the HIR-based ones use resolved def paths.
