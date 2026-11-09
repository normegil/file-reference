//! Dylint lint enforcing rules **Rust-T-1** and **Commons-T-2** (MUST): inline
//! unit tests (`#[cfg(test)]` modules) must be hermetic — no filesystem, network,
//! process or testcontainers access. Tests needing those belong in `tests/`.
//!
//! Only fires inside items carrying `#[cfg(test)]`; files under `tests/` are
//! integration tests and are not flagged.

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
use rustc_hir::intravisit::{self, Visitor};
use rustc_hir::{Expr, HirId, Item, QPath, Ty};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::sym;

rustc_session::declare_lint! {
    /// ### What it does
    /// Flags `std::fs`, `std::net`, `std::process` and `testcontainers` uses
    /// inside inline `#[cfg(test)]` modules.
    ///
    /// ### Why is this bad? (rules Rust-T-1, Commons-T-2)
    /// Unit tests must be hermetic (no network, external processes or disk
    /// state). Tests that need them belong in the `tests/` folder.
    pub HERMETIC_INLINE_TESTS,
    Warn,
    "inline unit tests must be hermetic; move fs/net/process tests to tests/"
}

const FORBIDDEN_PREFIXES: &[&str] = &[
    "std::fs::",
    "std::net::",
    "std::process::",
    "testcontainers",
];

fn has_cfg_test(cx: &LateContext<'_>, hir_id: HirId) -> bool {
    cx.tcx.hir().attrs(hir_id).iter().any(|attr| {
        attr.has_name(sym::cfg)
            && cx
                .sess()
                .source_map()
                .span_to_snippet(attr.span)
                .map(|s| s.contains("test"))
                .unwrap_or(false)
    })
}

fn is_forbidden_path(cx: &LateContext<'_>, qpath: &QPath<'_>) -> bool {
    if let QPath::Resolved(_, path) = qpath {
        if let Some(did) = path.res.opt_def_id() {
            let def_path = cx.tcx.def_path_str(did);
            return FORBIDDEN_PREFIXES
                .iter()
                .any(|prefix| def_path.starts_with(prefix));
        }
    }
    false
}

struct ForbiddenVisitor<'a, 'tcx> {
    cx: &'a LateContext<'tcx>,
}

impl<'tcx> Visitor<'tcx> for ForbiddenVisitor<'_, 'tcx> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        if let rustc_hir::ExprKind::Path(qpath) = &expr.kind {
            if is_forbidden_path(self.cx, qpath) {
                self.cx.span_lint(HERMETIC_INLINE_TESTS, expr.span, |diag| {
                    diag.build("inline unit tests must be hermetic: no filesystem/network/process access; move integration tests to tests/ (rules Rust-T-1, Commons-T-2)")
                });
            }
        }
        intravisit::walk_expr(self, expr);
    }

    fn visit_ty(&mut self, ty: &'tcx Ty<'tcx>) {
        if let rustc_hir::TyKind::Path(qpath) = &ty.kind {
            if is_forbidden_path(self.cx, qpath) {
                self.cx.span_lint(HERMETIC_INLINE_TESTS, ty.span, |diag| {
                    diag.build("inline unit tests must be hermetic: no filesystem/network/process types; move integration tests to tests/ (rules Rust-T-1, Commons-T-2)")
                });
            }
        }
        intravisit::walk_ty(self, ty);
    }
}

impl_late_lint! {
    fn check_item(&mut self, cx: &LateContext<'_>, item: &Item<'_>) {
        if has_cfg_test(cx, item.hir_id()) {
            let mut visitor = ForbiddenVisitor { cx };
            intravisit::walk_item(&mut visitor, item);
        }
    }
}
