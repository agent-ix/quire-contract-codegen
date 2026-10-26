---
id: SR-585007
title: 'gap-analysis of IR-329 CG PR #588'
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-contract-codegen; PR #588 IR-329; spec/kani/functional/FR-015-bounded-kani-obligations.md (FR-015-AC-7, -58, -95, -96, -99, -100, -110, -111), spec/assurance/AD-004-cg-crate-layout.md (step 4h, L-2, R-1), src/kani/generate/{collection,render,spec}.rs, tests/it/kani_obligations_symbolic_domains.rs, tests/it/cover_last.rs, tests/it/layout.rs'
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: references
---

## Summary

Ticket: IR-329. Planless gap analysis of CG PR #588 at exact head
`23f9a049ed722d6366f4893e6d02577c6131be4c` (merge-base
`1d5e8e95fc3ee38ded6bf92b4cf19f3769b1af81`), scoped to the symbolic collection rendering slice
(AD-004 step 4h). Tools: quoin 0.29.1, quire 0.36.2 (engine 0.50.2). `quire matrix --format tsv`
at this head lists 560 tagged, 16 tagged-by-ignored-test, 318 untagged and 53
method-without-symbol rows repo-wide. This review measured only the in-scope rows. No Cargo or Kani
run was made (static brief). Plan completion: not assessed. Reviewer model: Claude Opus 5.5
(claude-opus-5-5).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-015-AC-99's status prose now contradicts the computed matrix. It says the symbolic-draw, cover-after-assertions and no-`Vec<i128>` clauses are "backed only by tests tagged AC-96/AC-100 … no AC-99-tagged test asserts them", but this PR tags both cited tests FR-015-AC-99. The AC row and AD-004 step 4h were not reconciled with the code change. | spec/kani/functional/FR-015-bounded-kani-obligations.md:847, tests/it/kani_obligations_symbolic_domains.rs:940, tests/it/kani_obligations_symbolic_domains.rs:1333 |
| FND-002 | medium | The installed-backend AC-99 control (`tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies`, `#[ignore]`) has a recorded run only on f43adca/fe20ecc, not on this head. The renderer moved, and the bytes are unchanged only by static reading. FR-015-AC-110's gate requires a recorded green `make kani` on the slice head. | spec/kani/functional/FR-015-bounded-kani-obligations.md:858, tests/it/kani_obligations_symbolic_domains.rs:2030 |

## Coverage

- FR-015-AC-99 (examined): tagged at this head by
  `kani_obligations_symbolic_domains.rs` :940 (negotiation, newly tagged), :1243, :1333
  (inclusive bounds, newly tagged), :2014 (native replay through `qsl_replay::replay`) and :2030
  (ignored Kani lane). The new assertions in the negotiation test pin the symbolic length and array
  draws, the absence of `Vec<i128>`, a single cover and the claim assertion before the cover. The
  cover-placement assertion is weak (SR-585006 FND-005). AC-99 stays PARTIAL. The tag gap the
  spec records is closed in code, but the spec text was not updated and the real-Kani evidence
  predates this head.
- FR-015-AC-96 and FR-015-AC-100 (examined): bindings unchanged, and the generated text is
  byte-identical, so identity evidence carries over.
- FR-015-AC-7 and FR-015-AC-58 (examined): `cover_last.rs` drives the "symbolic collection"
  family through the public generator, so the cover-last inspection still applies to the moved
  template. The template census half of AC-58 is broken at this head (SR-585006 FND-001).
- FR-015-AC-110 (examined, residual): untagged and PLANNED. `prepare_bounded_collection_query`,
  `render_collection_qsl`, `lower/bounded_collections.rs` and the corpus `Collection` arm all remain
  in `src/` and `tests/`. This PR supplies only the AD-004 step-4h prerequisite and none of the
  deletion.
- FR-015-AC-111 (examined, residual): untagged and PLANNED. `prepare_finite_graph_reaches`,
  `generate_bounded_kani_corpus_case`, `BoundedCorpusRequest` and the corpus API remain. The PR does
  not touch them, and the slice is additionally gated on IR-805.
- FR-015-AC-95 (context_only): untagged and PLANNED until AC-109 to AC-111 all hold.
- V1 retirement (ADR-001 Q3, IR-329 description): out of scope here. `generate_kani_bundle` and
  `v1_bundle.rs` remain untouched.

## Verdict

PARTIAL. The PR advances FR-015-AC-99 and AD-004 step 4h in code without introducing an orphan.
Every changed function is owned by FR-015-AC-96/99/100 through `collection.rs` and `render.rs`.
It leaves the FR-015-AC-99 row and AD-004 step 4h text stale (FND-001), and it carries no
installed-Kani evidence for the moved renderer (FND-002). FR-015-AC-110, FR-015-AC-111 and V1
retirement remain residual IR-329 scope.
