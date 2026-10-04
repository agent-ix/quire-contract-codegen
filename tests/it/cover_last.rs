//! FR-015-AC-7, AC-53 and AC-58: every harness the generator emits ends with exactly one
//! `kani::cover!`, as the last statement of its body.
//!
//! The guard has two halves. The inspection drives every emitting entry point, parses each
//! emitted source with `syn` and checks every function attributed `#[kani::proof]` or
//! `#[kani::proof_for_contract]`. The scan reads the non-test string literals of `src/` and fails
//! when a file outside the driven set spells either proof attribute, so a new emitter cannot be
//! added without being driven here. The emitters have no single seam today (AD-004 step 4b's
//! `HarnessSpec` does not exist), so this is an inspection of emitted text.

use std::collections::BTreeSet;

use syn::{
    visit::{self, Visit},
    Attribute, Expr, ItemFn, Macro, Stmt,
};

use crate::layout::{non_test_string_literals, source_files};

/// Every family the inspection drives, with the one source file that emits it. A file in this
/// table is a file the scan allows to spell a proof attribute.
const FAMILIES: [(&str, &str); 11] = [
    ("precondition", "kani/generate/precondition.rs"),
    ("v1 contract postcondition", "kani/generate/contract.rs"),
    ("v1 contract invariant", "kani/generate/contract.rs"),
    ("scalar", "kani/generate/scalar.rs"),
    ("state clause", "kani/generate/frame.rs"),
    ("frame effect", "kani/generate/frame.rs"),
    ("v1 bundle", "kani/generate/v1_bundle.rs"),
    (
        "v1 bundle with an assumed and a stubbed dependency",
        "kani/generate/v1_bundle.rs",
    ),
    (
        "corpus arithmetic",
        "kani/generate/corpus/bounded_kani_corpus.rs",
    ),
    (
        "corpus graph",
        "kani/generate/corpus/bounded_kani_corpus.rs",
    ),
    (
        "corpus collection",
        "kani/generate/corpus/bounded_kani_corpus.rs",
    ),
];

/// Test code the scan skips whole: `kani/test_support.rs` has no `#[cfg(test)]` of its own, its
/// gate is the `mod` declaration in `kani/mod.rs`.
const TEST_CODE_FILES: [&str; 1] = ["kani/test_support.rs"];

fn is_path(path: &syn::Path, segments: [&str; 2]) -> bool {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .eq(segments)
}

fn is_proof(attributes: &[Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        is_path(attribute.path(), ["kani", "proof"])
            || is_path(attribute.path(), ["kani", "proof_for_contract"])
    })
}

fn is_cover(mac: &Macro) -> bool {
    is_path(&mac.path, ["kani", "cover"])
}

/// Counts the `kani::cover!` invocations anywhere in a body.
#[derive(Default)]
struct Covers(usize);

impl<'ast> Visit<'ast> for Covers {
    fn visit_macro(&mut self, mac: &'ast Macro) {
        if is_cover(mac) {
            self.0 += 1;
        }
        visit::visit_macro(self, mac);
    }
}

/// The proof functions of one emitted source, and what is wrong with each.
#[derive(Default)]
struct Inspection {
    proofs: usize,
    problems: Vec<String>,
}

impl<'ast> Visit<'ast> for Inspection {
    fn visit_item_fn(&mut self, function: &'ast ItemFn) {
        if is_proof(&function.attrs) {
            self.proofs += 1;
            let name = &function.sig.ident;
            let mut covers = Covers::default();
            covers.visit_block(&function.block);
            let last_is_cover = match function.block.stmts.last() {
                Some(Stmt::Macro(statement)) => is_cover(&statement.mac),
                Some(Stmt::Expr(Expr::Macro(expression), _)) => is_cover(&expression.mac),
                _ => false,
            };
            if covers.0 != 1 {
                self.problems.push(format!(
                    "`{name}` has {} `kani::cover!`, not exactly one",
                    covers.0
                ));
            }
            if !last_is_cover {
                self.problems
                    .push(format!("`{name}` does not end with its `kani::cover!`"));
            }
        }
        visit::visit_item_fn(self, function);
    }
}

/// Parses `source` and inspects every proof function in it, inline `mod` blocks included (the
/// visitor descends into them).
fn inspect(source: &str) -> Inspection {
    let file = syn::parse_file(source).unwrap_or_else(|error| {
        panic!("an emitted source must parse: {error}\n{source}");
    });
    let mut inspection = Inspection::default();
    inspection.visit_file(&file);
    inspection
}

fn assert_cover_last(family: &str, source: &str) {
    let inspection = inspect(source);
    assert_eq!(
        inspection.proofs, 1,
        "{family}: one proof function per emitted source:\n{source}"
    );
    assert!(
        inspection.problems.is_empty(),
        "{family}: {:?}\n{source}",
        inspection.problems
    );
}

fn emitted() -> Vec<(&'static str, String)> {
    [
        crate::kani_obligations::guard_sources(),
        crate::kani_obligations_state_frame::guard_sources(),
        crate::kani_generation::guard_sources(),
        crate::bounded_kani_corpus::guard_sources(),
    ]
    .concat()
}

/// Every emitting entry point's harness has exactly one cover, as its last statement.
///
/// Trace: FR-015-AC-7, FR-015-AC-53, FR-015-AC-54, FR-015-AC-55, FR-015-AC-58, TC-025
#[test]
fn tc_025_every_emitted_harness_ends_with_exactly_one_cover() {
    let emitted = emitted();
    for (family, source) in &emitted {
        assert_cover_last(family, source);
    }
    let driven: BTreeSet<&str> = emitted.iter().map(|(family, _)| *family).collect();
    let expected: BTreeSet<&str> = FAMILIES.iter().map(|(family, _)| *family).collect();
    assert_eq!(
        driven, expected,
        "the entry points driven are exactly the families the guard lists"
    );
}

/// The inspection fails a harness whose cover precedes an assertion, a harness with two covers
/// and a harness with none, so it cannot pass on a source it fails to read.
///
/// Trace: FR-015-AC-58, TC-025
#[test]
fn tc_025_the_cover_inspection_rejects_a_misplaced_duplicated_or_missing_cover() {
    let harness =
        |body: &str| format!("#[cfg(kani)]\nmod m {{\n#[kani::proof]\nfn h() {{\n{body}}}\n}}\n");
    let problems = |body: &str| inspect(&harness(body)).problems;
    assert!(problems("assert!(true);\nkani::cover!(true, \"c\");\n").is_empty());
    assert!(!problems("kani::cover!(true, \"c\");\nassert!(true);\n").is_empty());
    assert!(!problems("kani::cover!(true, \"a\");\nkani::cover!(true, \"b\");\n").is_empty());
    assert!(!problems("assert!(true);\n").is_empty());
    assert!(!problems("if true {\nkani::cover!(true, \"c\");\n}\n").is_empty());
    assert_eq!(inspect(&harness("")).proofs, 1);
    assert_eq!(inspect("fn plain() {}").proofs, 0);
}

/// No file outside the driven set emits either proof attribute, and every file the inspection
/// drives does.
///
/// Trace: FR-015-AC-58, TC-025
#[test]
fn tc_025_no_source_file_outside_the_driven_set_emits_a_proof_attribute() {
    let emitting: BTreeSet<String> = source_files()
        .into_iter()
        .filter(|(file, _)| !TEST_CODE_FILES.contains(&file.as_str()))
        .filter(|(_, text)| {
            non_test_string_literals(text)
                .iter()
                .any(|literal| literal.contains("kani::proof"))
        })
        .map(|(file, _)| file)
        .collect();
    let expected: BTreeSet<String> = FAMILIES
        .iter()
        .map(|(_, file)| (*file).to_owned())
        .collect();
    assert_eq!(
        emitting, expected,
        "the files that emit a proof attribute are exactly the files the cover inspection drives"
    );
}
