//! Dylint lint enforcing rule **Commons-UT-1** (SHOULD): at most one assertion
//! per unit test.
//!
//! Heuristic: counts assert-family macro invocations (`assert!`, `assert_eq!`,
//! `assert_ne!`, `assert_matches!`, ...) in the source snippet of `#[test]`
//! functions and flags functions with more than one.

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
    /// Flags `#[test]` functions containing more than one assertion macro.
    ///
    /// ### Why is this bad? (rule Commons-UT-1)
    /// One assertion per test keeps failures precise and test names meaningful;
    /// split multiple assertions into separate tests.
    pub SINGLE_ASSERT_PER_TEST,
    Warn,
    "at most one assertion per unit test"
}

const ASSERT_SUFFIXES: &[&str] = &[
    "!", "_eq!", "_ne!", "_le!", "_lt!", "_ge!", "_gt!", "_matches!",
];

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

/// Counts assert-family macro invocations in `src` (word-boundary aware, so
/// `debug_assert!` and custom macros are not counted).
fn count_assert_macros(src: &str) -> usize {
    let bytes = src.as_bytes();
    let mut count = 0;
    let mut i = 0;
    while i + 6 <= src.len() {
        if &src[i..i + 6] == "assert" {
            let boundary_ok =
                i == 0 || !(bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_');
            if boundary_ok {
                for suffix in ASSERT_SUFFIXES {
                    if src[i + 6..].starts_with(suffix) {
                        count += 1;
                        break;
                    }
                }
            }
        }
        i += 1;
    }
    count
}

impl_late_lint! {
    fn check_item(&mut self, cx: &LateContext<'_>, item: &Item<'_>) {
        if is_test_fn(cx, item) {
            if let Some(src) = body_snippet(cx, item) {
                let n = count_assert_macros(&src);
                if n > 1 {
                    cx.span_lint(SINGLE_ASSERT_PER_TEST, item.span, |diag| {
                        diag.build(format!("{n} assertions in one unit test; keep at most one assertion per test (rule Commons-UT-1)"))
                    });
                }
            }
        }
    }
}
