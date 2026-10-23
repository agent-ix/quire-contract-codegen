// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Structural consumer boundaries. Semantic output/refusal preservation belongs
//! to the scalar, equality and function generation tests, not this source walk.

use ix_trace_rs::trace;
use std::path::Path;
use syn::visit::{self, Visit};

const CONSUMERS: [&str; 3] = [
    "src/oracle/equality/mod.rs",
    "src/oracle/function/mod.rs",
    "src/oracle/scalar/mod.rs",
];

fn test_only(attributes: &[syn::Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        attribute.path().is_ident("test")
            || (attribute.path().is_ident("cfg")
                && attribute
                    .parse_args::<syn::Path>()
                    .is_ok_and(|path| path.is_ident("test")))
    })
}

fn string(expression: &syn::Expr) -> Option<String> {
    if let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Str(value),
        ..
    }) = expression
    {
        Some(value.value())
    } else {
        None
    }
}

fn reference_to(ty: &syn::Type, expected: &str) -> bool {
    let syn::Type::Reference(reference) = ty else {
        return false;
    };
    let syn::Type::Path(path) = reference.elem.as_ref() else {
        return false;
    };
    path.path
        .segments
        .last()
        .is_some_and(|segment| segment.ident == expected)
}

#[derive(Default)]
struct ConsumerBoundary {
    symbol: Vec<String>,
    term_dispatch: Vec<String>,
    node_map_aliases: Vec<String>,
}

impl ConsumerBoundary {
    fn at(&self, operation: &str) -> String {
        format!("{}: {operation}", self.symbol.join("::"))
    }
}

impl<'ast> Visit<'ast> for ConsumerBoundary {
    fn visit_item_mod(&mut self, module: &'ast syn::ItemMod) {
        if !test_only(&module.attrs) {
            self.symbol.push(module.ident.to_string());
            visit::visit_item_mod(self, module);
            self.symbol.pop();
        }
    }

    fn visit_item_fn(&mut self, function: &'ast syn::ItemFn) {
        if !test_only(&function.attrs) {
            self.symbol.push(function.sig.ident.to_string());
            visit::visit_item_fn(self, function);
            self.symbol.pop();
        }
    }

    fn visit_impl_item_fn(&mut self, function: &'ast syn::ImplItemFn) {
        if !test_only(&function.attrs) {
            self.symbol.push(function.sig.ident.to_string());
            visit::visit_impl_item_fn(self, function);
            self.symbol.pop();
        }
    }

    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        if let Some(key) = call.args.first().and_then(string) {
            let term_lookup = call.method == "get" && key == "term";
            let term_pointer = call.method == "pointer"
                && key.starts_with('/')
                && key.rsplit('/').next() == Some("term");
            if term_lookup || term_pointer {
                self.term_dispatch
                    .push(self.at(&format!("{}({key:?})", call.method)));
            }
        }
        visit::visit_expr_method_call(self, call);
    }

    fn visit_expr_index(&mut self, index: &'ast syn::ExprIndex) {
        if string(&index.index).as_deref() == Some("term") {
            self.term_dispatch.push(self.at("index[\"term\"]"));
        }
        visit::visit_expr_index(self, index);
    }

    fn visit_item_type(&mut self, alias: &'ast syn::ItemType) {
        if test_only(&alias.attrs) {
            return;
        }
        if let syn::Type::Path(path) = alias.ty.as_ref() {
            if let Some(segment) = path.path.segments.last() {
                if segment.ident == "BTreeMap" {
                    if let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments {
                        let types: Vec<_> = arguments
                            .args
                            .iter()
                            .filter_map(|argument| match argument {
                                syn::GenericArgument::Type(ty) => Some(ty),
                                _ => None,
                            })
                            .collect();
                        if let [key, value] = types.as_slice() {
                            if reference_to(key, "CheckedNodeId")
                                && reference_to(value, "CheckedSemanticNodeV2")
                            {
                                self.node_map_aliases.push(alias.ident.to_string());
                            }
                        }
                    }
                }
            }
        }
        visit::visit_item_type(self, alias);
    }
}

fn boundaries() -> Vec<(String, ConsumerBoundary)> {
    CONSUMERS
        .iter()
        .map(|relative| {
            let source =
                std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(relative))
                    .unwrap_or_else(|error| panic!("{relative}: {error}"));
            let syntax = syn::parse_file(&source)
                .unwrap_or_else(|error| panic!("{relative}: Rust must parse: {error}"));
            let mut boundary = ConsumerBoundary::default();
            boundary.visit_file(&syntax);
            ((*relative).to_owned(), boundary)
        })
        .collect()
}

// Suite-only traces: these structural checks do not claim semantic ACs or
// establish generated Rust/native agreement by inspecting source text.
#[trace("TC-024", "TC-029", "TC-031")]
#[test]
fn admitted_term_consumers_do_not_dispatch_on_raw_json_term_keys() {
    let findings: Vec<_> = boundaries()
        .into_iter()
        .flat_map(|(file, boundary)| {
            boundary
                .term_dispatch
                .into_iter()
                .map(move |site| format!("{file}::{site}"))
        })
        .collect();
    assert!(
        findings.is_empty(),
        "admitted-term consumers must use the owner's CheckedTerm API, not JSON term dispatch:\n{}",
        findings.join("\n")
    );
}

#[trace("TC-024", "TC-029", "TC-031")]
#[test]
fn consumers_do_not_define_separate_node_map_aliases() {
    let findings: Vec<_> = boundaries()
        .into_iter()
        .flat_map(|(file, boundary)| {
            boundary
                .node_map_aliases
                .into_iter()
                .map(move |alias| format!("{file}::{alias}"))
        })
        .collect();
    assert!(
        findings.is_empty(),
        "the three consumers still define separate node-map implementations:\n{}",
        findings.join("\n")
    );
}
