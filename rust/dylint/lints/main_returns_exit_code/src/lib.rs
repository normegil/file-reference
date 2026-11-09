//! Dylint lint enforcing rule **Rust-ERR-3** (MUST): errors bubble up to
//! `main.rs`, which formats them to `stderr` and exits with a non-zero
//! `std::process::ExitCode`.
//!
//! Enforced proxy: `fn main` in `src/main.rs` must return `std::process::ExitCode`.
//! (The stderr part is enforced by clippy's `print_stdout` = deny.)

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
use rustc_hir::{FnRetTy, Item, ItemKind, QPath, Ty, TyKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;

rustc_session::declare_lint! {
    /// ### What it does
    /// Flags `fn main` in `src/main.rs` whose return type is not
    /// `std::process::ExitCode`.
    ///
    /// ### Why is this bad? (rule Rust-ERR-3)
    /// Returning `ExitCode` makes failures exit non-zero with the error formatted
    /// to stderr, instead of an implicit exit code 0 or an unwrap panic.
    pub MAIN_RETURNS_EXIT_CODE,
    Warn,
    "`fn main` must return `std::process::ExitCode`"
}

fn is_main_rs(cx: &LateContext<'_>, span: Span) -> bool {
    cx.sess()
        .source_map()
        .span_to_filename(span)
        .to_string()
        .ends_with("main.rs")
}

fn last_segment_name(ty: &Ty<'_>) -> Option<&str> {
    if let TyKind::Path(QPath::Resolved(_, path)) = &ty.kind {
        path.segments.last().map(|s| s.ident.as_str())
    } else {
        None
    }
}

impl_late_lint! {
    fn check_item(&mut self, cx: &LateContext<'_>, item: &Item<'_>) {
        if item.ident.as_str() != "main" || !is_main_rs(cx, item.span) {
            return;
        }
        if let ItemKind::Fn { sig, .. } = &item.kind {
            let ok = matches!(&sig.decl.output, FnRetTy::Return(ty) if last_segment_name(ty) == Some("ExitCode"));
            if !ok {
                cx.span_lint(MAIN_RETURNS_EXIT_CODE, item.span, |diag| {
                    diag.build("fn main must return std::process::ExitCode so failures exit non-zero with the error on stderr (rule Rust-ERR-3)")
                });
            }
        }
    }
}
