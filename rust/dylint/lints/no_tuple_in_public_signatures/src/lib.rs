//! Dylint lint enforcing rule **Rust-A-2** (MUST): use structs instead of tuples
//! for return types, arguments and parameters.
//!
//! Flags tuples with more than two elements in public function signatures
//! (2-tuples are tolerated as idiomatic pairs).

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
use rustc_hir::{FnRetTy, FnSig, Item, ItemKind, TraitItem, TraitItemKind, Ty, TyKind};
use rustc_lint::{LateContext, LateLintPass};

rustc_session::declare_lint! {
    /// ### What it does
    /// Flags tuples with more than two elements in public function signatures
    /// (parameters and return type).
    ///
    /// ### Why is this bad? (rule Rust-A-2)
    /// Wide tuples are hard to read and easy to misuse; use a struct with named
    /// fields instead.
    ///
    /// ### Example
    /// ```ignore
    /// pub fn load() -> (String, u32, bool) { ... } // flagged
    /// ```
    pub NO_TUPLE_IN_PUBLIC_SIGNATURES,
    Warn,
    "use a struct instead of a tuple (> 2 elements) in public signatures"
}

fn is_public(cx: &LateContext<'_>, item_owner: rustc_hir::def_id::LocalDefId) -> bool {
    cx.tcx.visibility(item_owner).is_public()
}

fn check_ty(cx: &LateContext<'_>, ty: &Ty<'_>) {
    if let TyKind::Tup(elems) = &ty.kind {
        if elems.len() > 2 {
            cx.span_lint(NO_TUPLE_IN_PUBLIC_SIGNATURES, ty.span, |diag| {
                diag.build("tuple with more than two elements in a public signature; use a struct with named fields (rule Rust-A-2)")
            });
        }
    }
}

fn check_fn_sig(cx: &LateContext<'_>, public: bool, sig: &FnSig<'_>) {
    if !public {
        return;
    }
    for input in sig.decl.inputs {
        check_ty(cx, input);
    }
    if let FnRetTy::Return(ret) = &sig.decl.output {
        check_ty(cx, ret);
    }
}

impl_late_lint! {
    fn check_item(&mut self, cx: &LateContext<'_>, item: &Item<'_>) {
        if let ItemKind::Fn { sig, .. } = &item.kind {
            check_fn_sig(cx, is_public(cx, item.owner_id.def_id), sig);
        }
    }

    fn check_trait_item(&mut self, cx: &LateContext<'_>, item: &TraitItem<'_>) {
        if let TraitItemKind::Fn(sig, _) = &item.kind {
            check_fn_sig(cx, is_public(cx, item.owner_id.def_id), sig);
        }
    }
}
