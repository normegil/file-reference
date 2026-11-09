//! Dylint lint enforcing rule **Rust-ERR-2** (MUST): internal modules return
//! typed domain errors (e.g. via `thiserror`).
//!
//! Flags public functions whose `Result` error type is an opaque type
//! (`Box<dyn Error>`, `String`, `&str`, `anyhow::Error`) instead of a dedicated
//! domain error type.

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
use rustc_hir::{FnRetTy, GenericArg, GenericArgs, Item, ItemKind, QPath, Ty, TyKind};
use rustc_lint::{LateContext, LateLintPass};

rustc_session::declare_lint! {
    /// ### What it does
    /// Flags public functions returning `Result<T, E>` where `E` is not a
    /// dedicated (usually `thiserror`-derived) domain error type.
    ///
    /// ### Why is this bad? (rule Rust-ERR-2)
    /// `Box<dyn Error>`/`String`/`anyhow::Error` erase the error domain and make
    /// programmatic handling impossible; use a typed domain error.
    pub TYPED_DOMAIN_ERROR,
    Warn,
    "return a dedicated domain error type instead of Box<dyn Error>/String/&str/anyhow"
}

fn is_public(cx: &LateContext<'_>, owner: rustc_hir::def_id::LocalDefId) -> bool {
    cx.tcx.visibility(owner).is_public()
}

/// If `ty` is `Result<T, E>`, returns `E`.
fn result_error_ty<'hir>(ty: &Ty<'hir>) -> Option<&Ty<'hir>> {
    let TyKind::Path(QPath::Resolved(_, path)) = &ty.kind else {
        return None;
    };
    let seg = path.segments.last()?;
    if seg.ident.as_str() != "Result" {
        return None;
    }
    let args = seg.args?;
    let GenericArgs::AngleBracketed(ab) = &*args else {
        return None;
    };
    ab.args
        .iter()
        .filter_map(|a| match a {
            GenericArg::Type(t) => Some(t),
            _ => None,
        })
        .nth(1)
}

fn last_segment_name(ty: &Ty<'_>) -> Option<&str> {
    if let TyKind::Path(QPath::Resolved(_, path)) = &ty.kind {
        path.segments.last().map(|s| s.ident.as_str())
    } else {
        None
    }
}

fn is_opaque_error_ty(ty: &Ty<'_>) -> bool {
    match &ty.kind {
        TyKind::Ref(_, inner) => last_segment_name(&inner.ty) == Some("str"),
        TyKind::Path(QPath::Resolved(_, path)) => {
            let name = path.segments.last().map(|s| s.ident.as_str()).unwrap_or("");
            if name == "String" {
                return true;
            }
            if path.segments.iter().any(|s| s.ident.as_str() == "anyhow") {
                return true;
            }
            // Box<dyn ...> — crude check on the debug rendering of the generic args
            if name == "Box" {
                if let Some(seg) = path.segments.last() {
                    if let Some(args) = seg.args {
                        return format!("{args:?}").contains("dyn");
                    }
                }
            }
            false
        }
        _ => false,
    }
}

impl_late_lint! {
    fn check_item(&mut self, cx: &LateContext<'_>, item: &Item<'_>) {
        if let ItemKind::Fn { sig, .. } = &item.kind {
            if !is_public(cx, item.owner_id.def_id) {
                return;
            }
            if let FnRetTy::Return(ret) = &sig.decl.output {
                if let Some(err_ty) = result_error_ty(ret) {
                    if is_opaque_error_ty(err_ty) {
                        cx.span_lint(TYPED_DOMAIN_ERROR, ret.span, |diag| {
                            diag.build("public functions must return a dedicated domain error type (e.g. via thiserror), not Box<dyn Error>/String/&str/anyhow (rule Rust-ERR-2)")
                        });
                    }
                }
            }
        }
    }
}
