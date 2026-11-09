# Commons coding guidelines

These are generic guidelines for all languages, which must be read and followed.

## How to read the rule annotations

Each rule below ends with one or more tags of the form `[<kind>(<language>): <tool>]`:

- `gate` — enforced automatically (compiler, linter, custom lint, CI gate). A violation fails the build or test run with a deterministic message: run the gate locally and fix what it reports instead of self-checking.
- `warn` — reported by a tool, but non-blocking.
- `guideline` — not reliably automatable; apply your own judgment during implementation and review.

The `(<language>)` qualifier scopes a tag: when working in language X, only the tags for X apply (the Rust gates live in the `rust/` directory of this repository: `clippy.toml`, `cargo-lints.snippet.toml`, `dylint/`). Tags for other languages will be added together with their dedicated configs. If a rule has no tag for your language, treat it as `[guideline]`.

## Implementation Best Practices

### 1 — Before Coding

- **Commons-BP-1 (MUST)** Ask the user clarifying questions when specifications or requirements are ambiguous or incomplete. — `[guideline]`
- **Commons-BP-2 (SHOULD)** Draft and confirm an approach before implementing complex work. — `[guideline]`
- **Commons-BP-3 (SHOULD)** When multiple approaches exist, list clear trade-offs and pros/cons. — `[guideline]`

---

### 2 — While Coding

- **Commons-C-1 (MUST)** Name functions and data models using existing domain vocabulary for consistency. — `[guideline]`
- **Commons-C-2 (MUST)** Prefer small, composable, and testable functions. — `[guideline]`
- **Commons-C-3 (SHOULD)** Delete dead code and stale comments rather than commenting them out. — `[gate(rust): rustc dead_code lint]`
- **Commons-C-4 (SHOULD)** Functions should be pure as much as possible. — `[guideline]`
- **Commons-C-5 (SHOULD)** Structs should be immutable as much as possible. — `[guideline]`
- **Commons-C-6 (SHOULD)** Variables should be defined close to their usage. — `[guideline]`

---

### 3 — Testing

- **Commons-T-1 (MUST)** Separate pure-logic unit tests from external service/network integration tests. — `[gate(rust): dylint hermetic_inline_tests + tests/ convention]`
- **Commons-T-2 (MUST)** Keep unit tests hermetic by avoiding network calls, external processes, or disk state. — `[gate(rust): dylint hermetic_inline_tests]`
- **Commons-T-3 (SHOULD)** Prioritize fast unit tests for rapid feedback loops. — `[warn(rust): cargo nextest --slow-timeout]`
- **Commons-T-4 (SHOULD)** Prefer integration tests over extensive internal state mocking. — `[guideline]`
- **Commons-T-5 (SHOULD)** Unit-test algorithmic logic, state transitions, and edge cases thoroughly. — `[warn(rust): cargo llvm-cov (measures lines, not case quality)]`
- **Commons-T-6 (SHOULD)** Parameterize test cases instead of duplicating test logic. — `[guideline]`
- **Commons-T-7 (NEVER)** Add tests with trivial or self-fulfilling assertions (e.g., asserting hardcoded equality without exercising code). — `[gate(rust): clippy eq_op, assertions_on_constants]`

#### 3.1 — Test Quality

- **Commons-TQ-1 (SHOULD)** State the specific behavior and condition being validated in the test name. — `[guideline]`
- **Commons-TQ-2 (SHOULD)** Verify outputs against pre-computed expectations or domain invariants, not by re-evaluating the system under test. — `[guideline]`
- **Commons-TQ-3 (SHOULD)** Cover boundary values, empty collections, whitespace variations, and malformed inputs. — `[guideline]`
- **Commons-TQ-4 (NEVER)** Write tests that only verify conditions guaranteed statically by the compiler or type system. — `[gate(rust): clippy eq_op, absurd_extreme_comparisons (partial)]`

#### 3.2 Unit Tests

- **Commons-UT-1 (SHOULD)** Limit to one assertion per test maximum in unit tests. — `[warn(rust): dylint single_assert_per_test]`

---

### 4 — Linting and Formatting

- **Commons-L-1 (MUST)** Ensure code adheres to project formatting and linter checks with zero warnings. — `[gate(rust): clippy -D warnings + fmt --check (Taskfile/CI, already enforced)]`

---

### 5 — Documentation

- **Commons-D-1 (MUST)** Document public APIs, exported types, and modules with clear descriptions. — `[gate(rust): rustc missing_docs=deny]`
- **Commons-D-2 (MUST)** Keep existing documentation and usage examples synchronized with code changes. — `[gate(rust): doctests run by cargo test (partial)]`
- **Commons-D-3 (SHOULD)** Document parameters, return values, failure conditions, and errors. — `[warn(rust): clippy missing_errors_doc/missing_panics_doc]`
- **Commons-D-4 (SHOULD)** Include runnable or illustrative examples in API docs. — `[warn(rust): doctests must compile and pass]`
- **Commons-D-5 (SHOULD)** Create Architecture Decision Records (ADRs) under `docs/` for significant design decisions. — `[guideline]`
- **Commons-D-6 (SHOULD)** Add deprecation notices. — `[gate(rust): rustc deprecated lint]`

---

### 6 — Code Review

- **Commons-R-1 (MUST)** Verify correctness: business logic, edge conditions, null/empty checks, and error paths. — `[guideline]`
- **Commons-R-2 (MUST)** Verify security: validate CLI inputs, sanitize paths, and safeguard credentials. — `[guideline]`
- **Commons-R-3 (MUST)** Verify performance: avoid unnecessary allocations, redundant roundtrips, or unbounded resource consumption. — `[guideline]`
- **Commons-R-4 (MUST)** Verify test coverage: ensure bug fixes contain regression tests and features have functional coverage. — `[warn(rust): cargo llvm-cov reports]`
- **Commons-R-5 (MUST)** Verify observability: ensure that the code is covered by logs, metrics & traces adequatly. — `[guideline]`

---

### 7 — Observability

- **Commons-O-1 (MUST)** Direct data output to `stdout` and diagnostic logs/error messages to `stderr`. — `[gate(rust): clippy print_stdout=deny]`
- **Commons-O-2 (SHOULD)** Support configurable logging/verbosity levels (`--quiet`, `-v`, `-vv`). — `[guideline]`
- **Commons-O-3 (SHOULD)** Ensure errors provide actionable diagnostic context (e.g., path that failed, connection target). — `[guideline]`

#### 7.1 — Webapps and backends

When editing a web application, a REST backend, ...
- **Commons-O-4 (SHOULD)** Ensure metrics are emitted for critical operations. — `[guideline]`
- **Commons-O-5 (SHOULD)** Include distributed tracing at important points of an application. — `[guideline]`
- **Commons-O-6 (SHOULD)** Propagate correlation IDs for distributed tracing. — `[guideline]`

---

### 8 — Architecture and Dependencies

- **Commons-A-1 (SHOULD)** Prefer simple, modular designs; avoid premature abstraction. — `[guideline]`
- **Commons-A-2 (SHOULD)** Separate interface/presentation logic (CLI commands) from domain and backend interactions. — `[gate(rust): dylint minimal_main_rs]`
- **Commons-A-3 (SHOULD)** Use dedicated or delegated types, or newtypes, rather than raw unbounded `String`s and other types. — `[guideline]`
- **Commons-DEP-1 (MUST)** Avoid unnecessary dependencies; evaluate footprint and transitive dependencies before adding crates or packages. — `[gate(rust): cargo udeps/machete/deny]`

#### 8.1 — Function design

- **Commons-FN-1 (SHOULD)** Keep cyclomatic complexity low; break down deeply nested conditionals into guard clauses or dedicated helper functions. — `[warn(rust): clippy cognitive_complexity]`
- **Commons-FN-2 (SHOULD)** Eliminate unused function arguments and dead parameters. — `[gate(rust): rustc unused_variables (fatal via -D warnings)]`
- **Commons-FN-3 (SHOULD)** Make dependencies explicit via function parameters rather than relying on global state. — `[warn(rust): dylint no_static_mut]`
- **Commons-FN-4 (MUST)** Keep the number of function arguments to under 3 (at most 3 parameters). When a function requires more parameters, encapsulate related arguments into dedicated structs. — `[gate(rust): clippy too-many-arguments-threshold=3 (rust/clippy.toml)]`

---

### 9 — External Systems

- **Commons-EXT-1 (MUST)** Always use an exponential backoff algorithm and a bounded timeout/retry limit for connections to systems external to the program. — `[gate(rust): dylint external_io_in_client_module (enforces centralization; the backoff algorithm itself is reviewed)]`

---

### 10 — Database

---

### 11 — Code Organization

---

### 12 — Tooling Gates

---
