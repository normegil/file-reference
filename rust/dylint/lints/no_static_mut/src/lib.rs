//! Dylint lint enforcing rule **Commons-FN-3** (SHOULD): dependencies are made
//! explicit via function parameters, not global state.
//!
//! Enforced proxy: flag `static mut` items (use immutable statics, interior
//! mutability, or explicit parameters instead).

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
use rustc_hir::{Item, ItemKind, Mutability};
use rustc_lint::{LateContext, LateLintPass};

rustc_session::declare_lint! {
    /// ### What it does
    /// Flags `static mut` declarations.
    ///
    /// ### Why is this bad? (rule Commons-FN-3)
    /// Mutable global state is unsafe and untestable; make dependencies explicit
    /// via function parameters, or use interior mutability behind an immutable
    /// static.
    pub NO_STATIC_MUT,
    Warn,
    "avoid `static mut`; pass dependencies explicitly instead"
}

impl_late_lint! {
    fn check_item(&mut self, cx: &LateContext<'_>, item: &Item<'_>) {
        if let ItemKind::Static(_, mutability, _) = &item.kind {
            if matches!(mutability, Mutability::Mut) {
                cx.span_lint(NO_STATIC_MUT, item.span, |diag| {
                    diag.build("avoid `static mut`; make dependencies explicit via function parameters (rule Commons-FN-3)")
                });
            }
        }
    }
}
