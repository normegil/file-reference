# Rust rule enforcement (automatic gates)

This directory is the Rust implementation of the automatic gates for the rules in
`.ai/rules/`. It is a template: files here are synced into target repositories, so
keep them free of project-specific names.

Most MUST/NEVER Rust rules are enforced by tooling instead of relying on agent
judgment, so feedback is deterministic (non-zero exit + precise message).

## Files

| File | Purpose |
|---|---|
| `clippy.toml` | Clippy config: `too-many-arguments-threshold = 3` (Commons-FN-4); unwrap/expect/panic allowed in tests only (Rust-ERR-1) |
| `cargo-lints.snippet.toml` | `[lints]` section to merge into the crate `Cargo.toml`: deny `unwrap_used`/`expect_used`/`panic` (Rust-ERR-1), `print_stdout` (Rust-CLI-1 / Commons-O-1), `missing_docs` (Commons-D-1), trivial-assertion lints (Commons-T-7/TQ-4), etc. |
| `dylint/` | Custom dylint lints for the rules clippy/rustc cannot express (see `dylint/README.md`) |
| `Taskfile.yml` | `lint` / `verify` tasks run clippy + fmt + dylint |
| `.gitlab-ci.yml` | `lint`, `dylint`, `test` jobs run the same gates in CI |

## Rule -> gate mapping

| Rule | Gate |
|---|---|
| Rust-A-1 (MUST) | dylint `minimal_main_rs` |
| Rust-A-2 (MUST) | dylint `no_tuple_in_public_signatures` |
| Rust-ERR-1 (NEVER) | clippy `unwrap_used`/`expect_used`/`panic` = deny (`cargo-lints.snippet.toml` + `clippy.toml` allow-in-tests) |
| Rust-ERR-2 (MUST) | dylint `typed_domain_error` |
| Rust-ERR-3 (MUST) | dylint `main_returns_exit_code` + clippy `print_stdout` = deny |
| Rust-CLI-1 / Commons-O-1 (MUST) | clippy `print_stdout` = deny |
| Rust-CLI-2 (SHOULD) | dylint `no_std_env_args` |
| Rust-C-1/C-2, Rust-GATE-1/2/3, Commons-L-1 | `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`, `cargo test` (Taskfile lint/verify, CI jobs) |
| Rust-T-1 / Commons-T-2 (MUST) | dylint `hermetic_inline_tests` |
| Rust-T-2 (SHOULD) | dylint `prefer_assert_eq` |
| Commons-FN-3 (SHOULD) | dylint `no_static_mut` (warn) |
| Commons-FN-4 (MUST) | clippy `too-many-arguments-threshold = 3` (`clippy.toml`) |
| Commons-UT-1 (SHOULD) | dylint `single_assert_per_test` (warn) |
| Commons-EXT-1 (MUST) | dylint `external_io_in_client_module` (centralization; the backoff algorithm itself is reviewed) |
| Commons-D-1 (MUST) | rustc `missing_docs` = deny |
| Commons-D-3 (SHOULD) | clippy `missing_errors_doc` / `missing_panics_doc` (warn) |
| Commons-D-6 | rustc `deprecated` lint (fatal via `-D warnings`) |
| Commons-FN-1 (SHOULD) | clippy `cognitive_complexity` (warn) |
| Commons-FN-2 / Commons-C-3 | rustc `unused_variables` / `dead_code` (fatal via `-D warnings`) |
| Commons-T-7 / Commons-TQ-4 (NEVER) | clippy `eq_op`, `assertions_on_constants`, `absurd_extreme_comparisons` = deny |
| Commons-DEP-1 (MUST) | `cargo udeps` / `cargo machete` / `cargo deny` (add to CI alongside `cargo audit`) |
| Commons-T-3 (SHOULD) | `cargo nextest run --slow-timeout 60s` |
| Commons-D-2 / D-4 | doctests run by `cargo test --doc` |
| Commons-R-4 / T-5 | `cargo llvm-cov --fail-under-lines <threshold>` (CI) |

## Usage in a target repository

1. Copy `clippy.toml` next to the crate `Cargo.toml`; merge `cargo-lints.snippet.toml` into its `[lints]` section.
2. Copy `dylint/` in the folder `./rust/dylint` from `Taskfile.yml` and run `task lint` (or directly `cargo dylint --path dylint --all -- --all-targets -D warnings`).
3. Keep the `lint`/`dylint`/`test` tasks and CI jobs from `Taskfile.yml` / `.gitlab-ci.yml`.

The rules that remain manual (SHOULD-level style guidance) are listed in
`.ai/rules/rust.md`; commons rules and their gate annotations are in
`.ai/rules/commons.md`.
