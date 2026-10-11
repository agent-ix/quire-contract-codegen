---
id: SR-7200
title: IR-746 candidate code review with Rust lane
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@952e81e5d694f8ec3e2c6ae320b56a45d178ef02; Cargo.toml, src/core/{canonical,source_map}.rs, src/kani/generate/{census_validation,corpus/bounded_kani_corpus,negotiate,record,scalar,v1_bundle}.rs, src/oracle/{boolean_v1,equality/mod,function/mod,scalar/mod}.rs, src/strategy/{bound/generation,campaign,harness}.rs, tests/it/{bounded_kani_corpus,kani_generation,no_generation_panics,oracle_generation}.rs"
---

## Summary

Pre-PR candidate review of the exact local head above against merged CG main 75a7e5319170b9c91f9626a8efc498770ea5cac8. Ticket: IR-746. The Rust lane checked encoder placement, typed refusals, finite projection, copied logic, and changed tests. No source was edited and no full Cargo gate was run while IR-630 owns shared locks.

## Examined Units

- AD-004 L-9 (examined): canonical byte encoder and content digest called from core/canonical.rs only; generated source maps and proof graphs use canonical JSON with one newline and a 16 MiB ceiling; corpus case names hash a separate 1 MiB canonical preimage.
- NFR-005-AC-1 (examined): production Rust contains zero panic tokens, including CaseIdentity::digest.
- FR-015-AC-93 (examined): corpus generation gives typed refusals and no artifact or new identity claim.
- FR-015-AC-94 (examined): equal cases produce equal paths and contents; changing a request, input, ceilings, or dependencies changes identity.
- FR-028-AC-1 (examined): ceilings contribute to identity.
- TC-023 (examined): bounded Kani corpus parity and canonical case names.
- TC-042 (examined): no generation panic paths.

## Verdict

**CONDITIONAL.** The implementation has one test-oracle gap: the generated corpus name has no fixed canonical expectation. The changed source-map and proof-graph tests do distinguish their old field-order JSON. The changed panic scan would reject restoration of the old `expect`. A full build, CI, and real Kani run were outside this reviewer pass.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Corpus case-name tests do not assert an independently fixed canonical digest; replacing `CaseIdentity::digest` with serde field-order hashing can retain all observed name equality and inequality assertions. | src/kani/generate/corpus/bounded_kani_corpus.rs:1716 |

## Coverage

The source scan in `tests/it/no_generation_panics.rs` bans a new `fn deterministic_json` and direct `quire_canonical` calls outside core, but does not reject an inline serde hash in `CaseIdentity::digest`. The wide-integer test checks `RequestIdentity` canonical bytes directly, not the emitted case name. No runtime gate was run in this read-only review.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 75c46eddec8aef7ae1f5bd2c534cc5b91e6077dc |

After excerpt (`src/kani/generate/corpus/bounded_kani_corpus.rs:952-978`): `const DIGEST: &str = "1018f1df0ac4ccf591a06fbcb2e7e7f0e7255331b3f9117aa4f3b1003fd8afb4";` and `assert_eq!(case_name(&case), format!("arithmetic_{DIGEST}"));`. The 634-byte hand-authored preimage independently hashes to that digest; an old struct-field-order hash differs.
