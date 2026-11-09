//! Dylint lint enforcing rule **Commons-EXT-1** (MUST): connections to external
//! systems use an exponential backoff and a bounded timeout/retry limit.
//!
//! Enforced proxy: external I/O calls (`reqwest`, `hyper`, `ureq`, `surf`,
//! `std::net`, `tokio::net`) must live in a dedicated client module (named
//! `client`, `http` or `api`) — that single module is where the backoff/timeout
//! wrapper is implemented and reviewed.

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
use rustc_hir::{Expr, ExprKind, HirId, ItemKind, Node, QPath};
use rustc_lint::{LateContext, LateLintPass};

rustc_session::declare_lint! {
    /// ### What it does
    /// Flags calls/paths to external I/O crates (`reqwest`, `hyper`, `ureq`,
    /// `surf`, `std::net`, `tokio::net`) used outside a dedicated client module.
    ///
    /// ### Why is this bad? (rule Commons-EXT-1)
    /// External calls must go through one client module that applies a bounded
    /// timeout and an exponential backoff with a retry limit.
    pub EXTERNAL_IO_IN_CLIENT_MODULE,
    Warn,
    "external I/O must be centralized in a dedicated client module (backoff + bounded timeout)"
}

/// Def-path prefixes considered "external I/O".
const EXTERNAL_PREFIXES: &[&str] = &[
    "reqwest::",
    "hyper::",
    "ureq::",
    "surf::",
    "std::net::",
    "tokio::net::",
];

/// Module names allowed to perform external I/O.
const CLIENT_MODULES: &[&str] = &["client", "http", "api"];

fn is_external_path(cx: &LateContext<'_>, qpath: &QPath<'_>) -> bool {
    if let QPath::Resolved(_, path) = qpath {
        if let Some(did) = path.res.opt_def_id() {
            let def_path = cx.tcx.def_path_str(did);
            return EXTERNAL_PREFIXES
                .iter()
                .any(|prefix| def_path.starts_with(prefix));
        }
    }
    false
}

fn in_client_module(cx: &LateContext<'_>, hir_id: HirId) -> bool {
    cx.tcx.hir().parent_iter(hir_id).any(|(_, node)| {
        matches!(node, Node::Item(item)
            if matches!(&item.kind, ItemKind::Mod(..))
                && CLIENT_MODULES.iter().any(|m| *m == item.ident.as_str()))
    })
}

impl_late_lint! {
    fn check_expr(&mut self, cx: &LateContext<'_>, expr: &Expr<'_>) {
        if let ExprKind::Path(qpath) = &expr.kind {
            if is_external_path(cx, qpath) && !in_client_module(cx, expr.hir_id) {
                cx.span_lint(EXTERNAL_IO_IN_CLIENT_MODULE, expr.span, |diag| {
                    diag.build("external I/O must be centralized in a dedicated client module applying a bounded timeout and exponential backoff (rule Commons-EXT-1)")
                });
            }
        }
    }
}
