//! FR-015-AC-7, AC-53 and AC-58: every harness the generator emits ends with exactly one
//! `kani::cover!`, as the last statement of its body.
//!
//! The guard has two halves. The inspection drives every emitting entry point, parses each
//! emitted source with `syn` and checks every function attributed `#[kani::proof]` or
//! `#[kani::proof_for_contract]`. The scan reads the non-test string literals of `src/` and fails
//! when a file outside the driven set spells either proof attribute, so a new emitter cannot be
//! added without being driven here. The emitters have no single seam today (AD-004 step 4b's
//! `HarnessSpec` does not exist), so this is an inspection of emitted text.

use std::collections::{BTreeMap, BTreeSet};

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

/// The proof-attribute templates each driven file holds, as `proof_spellings` counts them. A
/// driven file with more spellings than this has a template no entry point above drives; raise
/// the count only together with a driver and a family in `FAMILIES`.
const PROOF_TEMPLATES: [(&str, usize); 6] = [
    ("kani/generate/precondition.rs", 1),
    ("kani/generate/contract.rs", 1),
    ("kani/generate/scalar.rs", 1),
    ("kani/generate/frame.rs", 2),
    ("kani/generate/v1_bundle.rs", 1),
    ("kani/generate/corpus/bounded_kani_corpus.rs", 1),
];

/// `literal` as the compiler reads it with respect to `\` line continuations: each backslash
/// that ends a line is dropped with the line break and the indentation after it.
fn joined(literal: &str) -> String {
    let mut out = String::with_capacity(literal.len());
    let mut chars = literal.chars().peekable();
    while let Some(character) = chars.next() {
        let continues = character == '\\' && matches!(chars.peek(), Some('\n' | '\r'));
        if !continues {
            out.push(character);
            continue;
        }
        while chars.next_if(|next| next.is_whitespace()).is_some() {}
    }
    out
}

/// The spellings of a kani proof attribute in `text`: each `#[kani::` that does not open one of
/// the known non-proof attributes (so `#[kani::proof]`, `#[kani::proof_for_contract(..)]`, a
/// split `"#[kani::"` + `"proof]"` and a `#[kani::{}]` template each count once), each
/// `kani::proof` outside such an attribute, and each `proof_for_contract` outside `kani::`. A
/// spelling built from fragments that never put `#[kani::` or `kani::proof` in one literal is not
/// seen, which is the scan's stated limit (FR-015-AC-58).
fn proof_spellings(text: &str) -> usize {
    const NON_PROOF: [&str; 4] = ["requires", "ensures", "stub", "unwind"];
    let text = joined(text);
    let before = |index: usize, suffix: &str| {
        text.get(..index)
            .is_some_and(|prefix| prefix.ends_with(suffix))
    };
    let attributes = text
        .match_indices("#[kani::")
        .filter(|(index, marker)| {
            let rest = text.get(index + marker.len()..).unwrap_or_default();
            !NON_PROOF.iter().any(|name| rest.starts_with(name))
        })
        .count();
    let paths = text
        .match_indices("kani::proof")
        .filter(|(index, _)| !before(*index, "#["))
        .count();
    let contracts = text
        .match_indices("proof_for_contract")
        .filter(|(index, _)| !before(*index, "kani::"))
        .count();
    attributes + paths + contracts
}

/// The counter sees each way a literal can spell a proof attribute, and none of the other
/// `kani::` text the generator emits.
///
/// Trace: FR-015-AC-58, TC-025
#[test]
fn tc_025_the_proof_spelling_counter_sees_split_formatted_and_continued_attributes() {
    for spelled in [
        "#[kani::proof]",
        "#[kani::proof_for_contract(f)]",
        "#[kani::{}]",
        "\"#[kani::\"",
        "kani::proof",
        "#[kani::\\\n        proof]",
        "kani\\\n::proof",
        "proof_for_contract",
    ] {
        assert_eq!(proof_spellings(spelled), 1, "{spelled:?}");
    }
    for other in [
        "#[kani::requires(x)]",
        "#[kani::ensures(|r: &u8| true)]",
        "#[kani::stub(a, b)]",
        "kani::cover!(true, \"c\")",
        "kani::any()",
        "kani::assume(x)",
    ] {
        assert_eq!(proof_spellings(other), 0, "{other:?}");
    }
}

/// A `#[cfg(test)]` item whose signature holds a `;` inside a bracket (an array type) still ends
/// at its body, so its literals stay out of the scan.
///
/// Trace: FR-015-AC-58, TC-025
#[test]
fn tc_025_the_literal_scan_skips_a_test_item_with_an_array_type() {
    let source = "#[cfg(test)]\nfn t() -> [u8; 2] { let _ = \"#[kani::proof]\"; [0, 0] }\n\
                  #[cfg(test)]\nconst X: [u8; 2] = [0, 0];\n\
                  #[cfg(test)]\nmod tests { fn u() { let _ = \"#[kani::proof]\"; } }\n\
                  fn live() { let _ = \"kept\"; }\n";
    assert_eq!(non_test_string_literals(source), ["\"kept\""]);
}

/// Every proof-attribute spelling in the non-test string literals of `src/` is in a file the
/// inspection drives, and a driven file spells exactly the templates counted above, so a new
/// emitter file, or a new template in a driven file, fails here until it is driven.
///
/// Trace: FR-015-AC-58, TC-025
#[test]
fn tc_025_no_proof_attribute_is_spelled_outside_what_the_inspection_drives() {
    let spelled: BTreeMap<String, usize> = source_files()
        .into_iter()
        .filter(|(file, _)| !TEST_CODE_FILES.contains(&file.as_str()))
        .map(|(file, text)| {
            let count = non_test_string_literals(&text)
                .iter()
                .map(|literal| proof_spellings(literal))
                .sum::<usize>();
            (file, count)
        })
        .filter(|(_, count)| *count > 0)
        .collect();
    let expected: BTreeMap<String, usize> = PROOF_TEMPLATES
        .iter()
        .map(|(file, count)| ((*file).to_owned(), *count))
        .collect();
    assert_eq!(
        spelled, expected,
        "the proof attributes spelled in src/ are exactly the templates the inspection drives"
    );
    let driven: BTreeSet<&str> = FAMILIES.iter().map(|(_, file)| *file).collect();
    let counted: BTreeSet<&str> = PROOF_TEMPLATES.iter().map(|(file, _)| *file).collect();
    assert_eq!(driven, counted, "every counted file has a driven family");
}
