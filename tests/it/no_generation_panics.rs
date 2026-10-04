//! NFR-005 / TC-042: no panic token on a generation or analysis path.
//!
//! The scan (AC-1), the public-surface seam (AC-3) and the body scan of the five IR-577
//! functions (AC-8) live here. The seams that are private to the crate (AC-2, AC-4 to AC-7) are
//! `#[cfg(test)]` tests beside the code they reach, named `tc_042_*`.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use quire_contract_codegen::{
    negotiate_kani_obligations, CapabilityKind, Cause, ClaimDisposition, Disposition,
    ItemSettlement, KaniObligationOutcome, KaniObligationRequest, ObligationDisposition,
    ObligationItem, UnsupportedObligation,
};
use quire_contract_model::CheckedNodeId;

use crate::common::panic_scan::{non_test_code_outside_literals, panic_tokens_in};
use crate::kani_obligations::scalar_package;

/// The file holding the one dated exception (NFR-005 Scope, IR-344).
const CORPUS: &str = "src/kani/generate/corpus/bounded_kani_corpus.rs";

/// The seven files that hold a measured site, plus the files FR-014-AC-39, FR-018-AC-19 and
/// FR-021-AC-21 name, deduplicated (`src/oracle/function/mod.rs` is in both): nine distinct files.
const MUST_SCAN: [&str; 9] = [
    "src/oracle/function/mod.rs",
    "src/kani/generate/scalar.rs",
    CORPUS,
    "src/routed/capability.rs",
    "src/routed/generate.rs",
    "src/evidence/bound_coverage.rs",
    "src/oracle/boolean_v1.rs",
    "src/oracle/scalar/mod.rs",
    "src/oracle/equality/mod.rs",
];

/// The fewest files the walk must read.
const MIN_FILES: usize = 60;

fn rust_files(dir: &Path, found: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .expect("read a source directory")
        .map(|entry| entry.expect("read a directory entry").path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            rust_files(&path, found);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            found.push(path);
        }
    }
}

/// True when the module file `file` is declared by its parent under `#[cfg(test)]`: a line
/// `mod <name>;` (with any visibility) whose attribute lines include `#[cfg(test)]`.
fn declared_under_cfg_test(file: &Path) -> bool {
    let stem = file
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("");
    let directory = file.parent().expect("a source file has a directory");
    let (name, parents) = if stem == "mod" {
        let name = directory.file_name().and_then(|name| name.to_str());
        let grand = directory.parent().expect("a module directory has a parent");
        (
            name.unwrap_or("").to_owned(),
            vec![
                grand.join("mod.rs"),
                grand.with_extension("rs"),
                grand.join("lib.rs"),
            ],
        )
    } else {
        (
            stem.to_owned(),
            vec![
                directory.join("mod.rs"),
                directory.with_extension("rs"),
                directory.join("lib.rs"),
            ],
        )
    };
    parents
        .iter()
        .filter(|parent| parent.as_path() != file)
        .filter_map(|parent| fs::read_to_string(parent).ok())
        .any(|parent_source| {
            let mut gated = false;
            for line in parent_source.lines().map(str::trim) {
                if line.starts_with("#[") {
                    gated |= line.starts_with("#[cfg(test)]");
                    continue;
                }
                let declares = line
                    .strip_suffix(';')
                    .and_then(|line| line.rsplit_once("mod "))
                    .is_some_and(|(prefix, declared)| {
                        declared.trim() == name && (prefix.is_empty() || prefix.starts_with("pub"))
                    });
                if declares && gated {
                    return true;
                }
                gated = false;
            }
            false
        })
}

/// The byte range of the first braced block of `code` at or after `from`, braces included.
fn braced_body(code: &str, from: usize) -> Option<std::ops::Range<usize>> {
    let open = from + code[from..].find('{')?;
    let mut depth = 0usize;
    for (offset, byte) in code[open..].bytes().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(open..open + offset + 1);
                }
            }
            _ => {}
        }
    }
    None
}

/// The byte range of the body of `fn digest` of `impl CaseIdentity<'_>` in `code`, braces included.
fn digest_body(code: &str) -> Option<std::ops::Range<usize>> {
    let implementation = code.find("impl CaseIdentity<'_>")?;
    let function = implementation + code[implementation..].find("fn digest")?;
    braced_body(code, function)
}

/// The byte range of the body of `fn <name>` in `code`, braces included: the first `fn <name>`
/// whose name ends there, so `fn observe_clause_row` is not `observe_clause`.
fn function_body(code: &str, name: &str) -> Option<std::ops::Range<usize>> {
    let needle = format!("fn {name}");
    let mut from = 0;
    while let Some(found) = code[from..].find(&needle) {
        let end = from + found + needle.len();
        let named = code[end..]
            .bytes()
            .next()
            .is_none_or(|byte| !(byte.is_ascii_alphanumeric() || byte == b'_'));
        if named {
            return braced_body(code, end);
        }
        from = end;
    }
    None
}

/// The five functions NFR-005-AC-8 scans: the file its body lives in, its name, and an identifier
/// that only that function's own body holds, so a locator that returned an empty or wrong range
/// would fail the scan instead of passing it vacuously.
const INDEX_FREE_BODIES: [(&str, &str, &str); 5] = [
    ("src/routed/generate.rs", "generate_kani", "route_records("),
    ("src/routed/generate.rs", "route_records", "pair_records("),
    (
        "src/routed/generate.rs",
        "rewrite_duplicate_position",
        "KaniDuplicatePositionOutOfRange",
    ),
    (
        "src/evidence/bound_coverage.rs",
        "observe_clause",
        "consequent_regions",
    ),
    (
        "src/oracle/boolean_v1.rs",
        "generate_boolean_oracle_inner",
        "probe_at(",
    ),
];

/// The keywords a body can precede a `[` or `-` with; any other identifier ends an operand.
const NON_OPERAND_KEYWORDS: [&str; 12] = [
    "in", "let", "mut", "ref", "return", "break", "if", "else", "match", "while", "as", "move",
];

/// What NFR-005-AC-8 forbids in the four bodies.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Unchecked {
    /// A `[` whose previous non-whitespace token ends an operand: an index or a range slice.
    Index,
    /// A `-` (or `-=`) that is not `->` and follows an operand.
    Subtraction,
}

/// Every index and subtraction token of `body`, with its byte offset. An operand-ending token is
/// `)`, `]`, `}`, `?`, a numeric literal, or an identifier that is not one of
/// [`NON_OPERAND_KEYWORDS`]; the body is literal-free, so a string is not a token here.
fn unchecked_tokens(body: &str) -> Vec<(Unchecked, usize)> {
    let bytes = body.as_bytes();
    let mut found = Vec::new();
    let mut after_operand = false;
    let mut at = 0;
    while at < bytes.len() {
        let byte = bytes[at];
        if byte.is_ascii_whitespace() {
            at += 1;
        } else if byte.is_ascii_alphanumeric() || byte == b'_' {
            let start = at;
            while at < bytes.len() && (bytes[at].is_ascii_alphanumeric() || bytes[at] == b'_') {
                at += 1;
            }
            after_operand =
                byte.is_ascii_digit() || !NON_OPERAND_KEYWORDS.contains(&&body[start..at]);
        } else {
            match byte {
                b'[' if after_operand => found.push((Unchecked::Index, at)),
                b'-' if after_operand && bytes.get(at + 1) != Some(&b'>') => {
                    found.push((Unchecked::Subtraction, at));
                }
                _ => {}
            }
            after_operand = matches!(byte, b')' | b']' | b'}' | b'?');
            at += 1;
        }
    }
    found
}

/// Trace: NFR-005-AC-1, TC-042. The helper keeps a panic token in code and drops one in a string
/// literal, a character literal, a comment and a `#[cfg(test)]` item, and keeps `abort` as a bare
/// identifier.
#[test]
fn tc_042_ac1_the_literal_free_scan_keeps_code_and_drops_literals_comments_and_test_items() {
    let source = "\
// a.unwrap() in a comment
fn code() { a.unwrap() }
fn literal() -> &'static str { \"b.unwrap() assert!(x)\" }
fn character() -> char { '!' }
fn raw() -> &'static str { r#\"panic!(\"x\")\"# }
#[cfg(test)]
mod tests { fn t() { c.expect(\"m\") } }
fn bare() { abort() }
";
    let code = non_test_code_outside_literals(source);
    assert_eq!(
        panic_tokens_in(&code),
        vec!["abort".to_owned(), "unwrap".to_owned()],
        "code kept: {code}"
    );
    assert_eq!(
        code.matches("unwrap").count(),
        1,
        "only the code one: {code}"
    );
    for dropped in ["assert", "panic", "expect", "comment"] {
        assert!(!code.contains(dropped), "`{dropped}` survived in:\n{code}");
    }
}

/// Trace: NFR-005-AC-1, TC-042. No file under `src/` holds a panic token in its non-test,
/// literal-free code, except the one `expect` of `CaseIdentity::digest` that the dated IR-344
/// exception allows; the scan read every file that holds a measured site.
#[test]
fn tc_042_ac1_src_holds_no_panic_token_outside_the_dated_digest_exception() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    rust_files(&root.join("src"), &mut files);

    let mut scanned = BTreeSet::new();
    let mut exempt = BTreeSet::new();
    let mut offences = Vec::new();
    let mut digest_expects = None;
    for file in &files {
        let relative = file
            .strip_prefix(&root)
            .expect("a file under the manifest directory")
            .to_string_lossy()
            .replace('\\', "/");
        if declared_under_cfg_test(file) {
            exempt.insert(relative);
            continue;
        }
        let source = fs::read_to_string(file).expect("read a source file");
        let code = non_test_code_outside_literals(&source);
        let checked = if relative == CORPUS {
            let body = digest_body(&code).expect("`fn digest` of `impl CaseIdentity<'_>` exists");
            let inside = &code[body.clone()];
            assert_eq!(
                panic_tokens_in(inside),
                vec!["expect".to_owned()],
                "the dated exception is one `expect` and nothing else in `digest`"
            );
            digest_expects = Some(inside.matches("expect").count());
            format!("{}{}", &code[..body.start], &code[body.end..])
        } else {
            code
        };
        let tokens = panic_tokens_in(&checked);
        if !tokens.is_empty() {
            offences.push(format!("{relative}: {tokens:?}"));
        }
        scanned.insert(relative);
    }

    assert!(
        exempt.contains("src/kani/test_support.rs"),
        "the `#[cfg(test)]` file is exempt: {exempt:?}"
    );
    for required in MUST_SCAN {
        assert!(
            scanned.contains(required),
            "the scan did not read {required}"
        );
    }
    assert!(
        scanned.len() >= MIN_FILES,
        "the scan read {} files, fewer than {MIN_FILES}",
        scanned.len()
    );
    assert_eq!(digest_expects, Some(1), "exactly one `expect` in `digest`");
    assert!(
        offences.is_empty(),
        "panic tokens on a generation or analysis path: {offences:#?}"
    );
}

/// Trace: NFR-005-AC-8, TC-042. The bodies of `generate_kani`, `route_records`, `rewrite_duplicate_position`,
/// `observe_clause` and `generate_boolean_oracle_inner`, located in the literal-free non-test code
/// of their files, hold no index token and no subtraction token.
#[test]
fn tc_042_ac8_the_ir_577_bodies_hold_no_index_or_subtraction() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut offences = Vec::new();
    for (file, name, marker) in INDEX_FREE_BODIES {
        let source = fs::read_to_string(root.join(file)).expect("read a source file");
        let code = non_test_code_outside_literals(&source);
        let body = function_body(&code, name)
            .unwrap_or_else(|| panic!("`fn {name}` was not found in {file}"));
        assert!(
            code[body.clone()].contains(marker),
            "the located body of `fn {name}` ({} bytes) does not hold `{marker}`",
            body.len()
        );
        for (kind, at) in unchecked_tokens(&code[body.clone()]) {
            let from = body.start + at;
            let context = code[from.saturating_sub(30)..]
                .chars()
                .take(60)
                .collect::<String>();
            offences.push(format!(
                "{name}: {kind:?} near `{}`",
                context.replace('\n', " ")
            ));
        }
    }
    assert!(
        offences.is_empty(),
        "index or subtraction in an IR-577 body: {offences:#?}"
    );
}

/// Trace: NFR-005-AC-8, TC-042. The body check flags every index, range slice and subtraction
/// spelling the requirement names, and passes the array, pattern, attribute, macro, arrow and
/// unary-minus shapes it names.
#[test]
fn tc_042_ac8_the_body_check_flags_index_and_subtraction_and_passes_their_lookalikes() {
    for (body, expected) in [
        ("a[1]", Unchecked::Index),
        ("a[2..]", Unchecked::Index),
        ("f(x)[0]", Unchecked::Index),
        ("x?[0]", Unchecked::Index),
        ("t.0[1]", Unchecked::Index),
        ("{ v }[0]", Unchecked::Index),
        ("n - 1", Unchecked::Subtraction),
        ("x? - 1", Unchecked::Subtraction),
        ("n -= 1", Unchecked::Subtraction),
    ] {
        assert_eq!(
            unchecked_tokens(body)
                .into_iter()
                .map(|(kind, _)| kind)
                .collect::<Vec<_>>(),
            vec![expected],
            "`{body}`"
        );
    }
    for body in [
        "for l in [a, b] {}",
        "vec![a]",
        "let [x, ..] = y;",
        "#[must_use]",
        "let a: [u8; 4] = b;",
        "fn f() -> u8 { 0 }",
        "-n",
        "(-n)",
        "f(a, -1)",
        "x = -1",
        "match x { _ => -1 }",
    ] {
        assert!(
            unchecked_tokens(body).is_empty(),
            "`{body}` is not an index or subtraction: {:?}",
            unchecked_tokens(body)
        );
    }
}

/// Trace: NFR-005-AC-2, TC-042. `classify_claim` reports a generated claim with no checked bound,
/// and one whose first checked bound is not among the derived integer ranges, as
/// `OperationNotRendered`; the unmodified claim is supported.
#[test]
fn tc_042_ac2_a_claim_map_this_generator_did_not_produce_is_operation_not_rendered() {
    let (package, claim_map) = scalar_package();
    let target = claim_map
        .items
        .iter()
        .find(|claim| {
            claim.operation.identity == "quire.op.integer.add"
                && matches!(claim.result, ClaimDisposition::Generated(_))
        })
        .map(|claim| claim.node_id.clone())
        .expect("the corpus generates an integer.add claim");
    let absent: CheckedNodeId = serde_json::from_value(serde_json::json!({
        "domain": "quire.checked-semantic-node/v1",
        "digest": "9".repeat(64),
    }))
    .expect("a node id");

    let disposition_of = |claim_map: &quire_contract_codegen::ClaimMap<_>| {
        let items = [ObligationItem::ScalarClaim {
            package: &package,
            claim_map,
            node_id: &target,
        }];
        let outcome = negotiate_kani_obligations(&KaniObligationRequest {
            items: &items,
            subject_path: "crate::subject",
            unwind: 1,
        })
        .expect("FR-015 accepts the request");
        let (KaniObligationOutcome::Emitted { mut records, .. }
        | KaniObligationOutcome::Rejected { mut records }) = outcome;
        assert_eq!(records.len(), 1);
        records.remove(0).disposition
    };

    assert!(matches!(
        disposition_of(&claim_map),
        ObligationDisposition::Supported { .. }
    ));
    for checked_bounds in [Vec::new(), vec![absent]] {
        let mut altered = claim_map.clone();
        for claim in &mut altered.items {
            if claim.node_id == target {
                let ClaimDisposition::Generated(generated) = &mut claim.result else {
                    panic!("the target is generated");
                };
                generated.checked_bounds.clone_from(&checked_bounds);
            }
        }
        assert!(
            matches!(
                disposition_of(&altered),
                ObligationDisposition::Unsupported {
                    reason: UnsupportedObligation::OperationNotRendered { .. }
                }
            ),
            "checked bounds {checked_bounds:?}"
        );
    }
}

/// Trace: NFR-005-AC-3, TC-042. A hand-built `Unsupported` settlement carrying an
/// `invalid_capability` cause has no warning to name; each `unsupported_projection` cause still
/// carries its warning.
#[test]
fn tc_042_ac3_an_unsupported_settlement_warns_only_for_an_unsupported_projection_cause() {
    let settle = |cause: Cause| ItemSettlement {
        request_index: 0,
        kind: Some(CapabilityKind::Refinement),
        disposition: Disposition::Unsupported { cause },
    };
    let invalid = [
        Cause::AbsentKind,
        Cause::UnknownKind {
            received: "nope".to_owned(),
        },
        Cause::AbsentExtent,
        Cause::UnknownBackend {
            backend: "cvc5".to_owned(),
        },
        Cause::InconsistentCandidates {
            candidates: Vec::new(),
        },
        Cause::AmbiguousBackend {
            candidates: Vec::new(),
        },
    ];
    for cause in invalid {
        assert_eq!(cause.code(), "invalid_capability");
        assert_eq!(settle(cause.clone()).warning(), None, "{cause:?}");
    }
    let projection = [
        Cause::UnsupportedRequestedCapability {
            kind: CapabilityKind::Refinement,
            backend: None,
        },
        Cause::UnboundedExtent {
            kind: CapabilityKind::Refinement,
            backend: "kani".to_owned(),
        },
    ];
    for cause in projection {
        assert_eq!(cause.code(), "unsupported_projection");
        let warning = settle(cause.clone()).warning();
        assert!(
            warning
                .as_deref()
                .is_some_and(|text| text.contains(CapabilityKind::Refinement.label())),
            "{cause:?} still warns: {warning:?}"
        );
    }
}
