//! Dylint lint enforcing rule **Rust-CLI-2** (SHOULD): use `clap` with derive
//! macros for CLI parsing.
//!
//! Enforced proxy: flag any use of `std::env::args` / `std::env::args_os`.

#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_lint;
extern crate rustc_middle;
#[macro_use]
extern crate rustc_session;
extern crate rustc_span;

dylint_linting::dylint_library!();

use dylint_linting::impl_late_lint;
use rustc_hir::{Expr, ExprKind, QPath};
use rustc_lint::{LateContext, LateLintPass};

rustc_session::declare_lint! {
    /// ### What it does
    /// Flags uses of `std::env::args` and `std::env::args_os`.
    ///
    /// ### Why is this bad? (rule Rust-CLI-2)
    /// Parse CLI flags/arguments with `clap` derive macros for declarative,
    /// consistent, self-documenting parsing.
    pub NO_STD_ENV_ARGS,
    Warn,
    "parse CLI arguments with clap (derive) instead of std::env::args"
}

impl_late_lint! {
    fn check_expr(&mut self, cx: &LateContext<'_>, expr: &Expr<'_>) {
        if let ExprKind::Path(QPath::Resolved(_, path)) = &expr.kind {
            if let Some(did) = path.res.opt_def_id() {
                let def_path = cx.tcx.def_path_str(did);
                if def_path == "std::env::args" || def_path == "std::env::args_os" {
                    cx.span_lint(NO_STD_ENV_ARGS, expr.span, |diag| {
                        diag.build("use clap with derive macros for CLI parsing instead of std::env::args (rule Rust-CLI-2)")
                    });
                }
            }
        }
    }
}
