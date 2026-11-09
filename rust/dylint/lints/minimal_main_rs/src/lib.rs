//! Dylint lint enforcing rule **Rust-A-1** (MUST): `main.rs` only parses CLI
//! arguments, initializes logging and delegates to library functions.
//!
//! Enforced proxy: no `mod` declarations in `main.rs` (business logic belongs in
//! `src/lib.rs` modules) and no function other than `main` longer than a threshold.

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
use rustc_hir::{Item, ItemKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;

rustc_session::declare_lint! {
    /// ### What it does
    /// Flags `mod` declarations and long functions in `src/main.rs`.
    ///
    /// ### Why is this bad? (rule Rust-A-1)
    /// `main.rs` should only parse CLI arguments, initialize logging and delegate
    /// to library functions; business logic belongs in `src/lib.rs` modules.
    pub MINIMAL_MAIN_RS,
    Warn,
    "`main.rs` must stay minimal: no module declarations, no long functions"
}

/// Functions in `main.rs` (other than `main`) longer than this are flagged.
const MAX_MAIN_RS_FN_LINES: usize = 60;

fn is_main_rs(cx: &LateContext<'_>, span: Span) -> bool {
    cx.sess()
        .source_map()
        .span_to_filename(span)
        .to_string()
        .ends_with("main.rs")
}

fn line_count(cx: &LateContext<'_>, span: Span) -> usize {
    cx.sess()
        .source_map()
        .span_to_snippet(span)
        .map(|s| s.matches('\n').count() + 1)
        .unwrap_or(0)
}

impl_late_lint! {
    fn check_item(&mut self, cx: &LateContext<'_>, item: &Item<'_>) {
        if !is_main_rs(cx, item.span) {
            return;
        }
        match &item.kind {
            ItemKind::Mod(..) => {
                cx.span_lint(MINIMAL_MAIN_RS, item.span, |diag| {
                    diag.build("do not declare modules in main.rs; move business logic to the library crate (rule Rust-A-1)")
                });
            }
            ItemKind::Fn { body, .. } if item.ident.as_str() != "main" => {
                if line_count(cx, body.value.span) > MAX_MAIN_RS_FN_LINES {
                    cx.span_lint(MINIMAL_MAIN_RS, item.span, |diag| {
                        diag.build("keep main.rs minimal: move this logic into the library crate (rule Rust-A-1)")
                    });
                }
            }
            _ => {}
        }
    }
}
