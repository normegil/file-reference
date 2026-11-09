//! Dylint lint enforcing rule **Rust-T-2** (SHOULD): use `assert_eq!` /
//! `assert_ne!` (or `assert_matches!`) instead of `assert!(a == b)`.
//!
//! Heuristic: scans the source snippet of `#[test]` functions for `assert!(...)`
//! whose argument contains a top-level `==` or `!=` comparison.

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
use rustc_span::sym;

rustc_session::declare_lint! {
    /// ### What it does
    /// Flags `assert!(a == b)` / `assert!(a != b)` in test functions.
    ///
    /// ### Why is this bad? (rule Rust-T-2)
    /// `assert_eq!`/`assert_ne!` show both operands on failure; `assert!` only
    /// shows a boolean.
    pub PREFER_ASSERT_EQ,
    Warn,
    "use assert_eq!/assert_ne! instead of assert!(a == b)"
}

fn is_test_fn(cx: &LateContext<'_>, item: &Item<'_>) -> bool {
    cx.tcx
        .hir()
        .attrs(item.hir_id())
        .iter()
        .any(|attr| attr.has_name(sym::test))
}

fn body_snippet(cx: &LateContext<'_>, item: &Item<'_>) -> Option<String> {
    if let ItemKind::Fn { body, .. } = &item.kind {
        cx.sess().source_map().span_to_snippet(body.value.span).ok()
    } else {
        None
    }
}

/// Returns true if `src` contains an `assert!(...)` whose parenthesized argument
/// contains a `==` or `!=` comparison.
fn assert_with_comparison(src: &str) -> bool {
    let bytes = src.as_bytes();
    let mut i = 0;
    while let Some(off) = src[i..].find("assert!") {
        let start = i + off;
        i = start + 1;
        // word boundary: not part of debug_assert!, assert_eq!, a custom macro...
        if start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_') {
            continue;
        }
        let open = start + "assert!".len();
        if !src[open..].starts_with('(') {
            continue;
        }
        let mut depth = 0usize;
        let mut end = src.len();
        for (k, c) in src[open..].char_indices() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        end = open + k;
                        break;
                    }
                }
                _ => {}
            }
        }
        let inner = &src[open + 1..end];
        if inner.contains("==") || inner.contains("!=") {
            return true;
        }
    }
    false
}

impl_late_lint! {
    fn check_item(&mut self, cx: &LateContext<'_>, item: &Item<'_>) {
        if is_test_fn(cx, item) {
            if let Some(src) = body_snippet(cx, item) {
                if assert_with_comparison(&src) {
                    cx.span_lint(PREFER_ASSERT_EQ, item.span, |diag| {
                        diag.build("use assert_eq!/assert_ne! instead of assert!(a == b) so failures show both operands (rule Rust-T-2)")
                    });
                }
            }
        }
    }
}
