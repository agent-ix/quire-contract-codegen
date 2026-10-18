---
id: "SR-12221"
title: "IR-863 gap analysis: acceptance criteria against the diff"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@8aa1f25a81450ea82ae2ca6ba486162201403cdf; src/kani/**, tests/it/kani_obligations*.rs, tests/it/kani_generation.rs, tests/it/kani_gate.rs, tests/it/bounded_kani_corpus.rs, tests/it/kani_batching.rs, tests/it/vacuity_primitives.rs, tests/it/cover_last.rs"
review_set: subset
relationships: []
---

# SR-12221: IR-863 gap analysis

## Summary

Ticket: IR-863. PR: agent-ix/quire-contract-codegen#426 at 8aa1f25. This checks the three ticket
acceptance criteria against `git diff origin/main...HEAD`. Plan completion: not assessed.

## Method

- AC1 (no `Trace:`/`Tracing:` doc lines remain): grep over the full in-scope file set. On main
  there are 310 doc-line forms (the same as on 836b298). At HEAD there are 0. One inline `//`
  comment remains (FND-001).
- AC2 (ids kept): I compared, per file, the multiset of ids removed from `Trace:` doc lines with
  the ids added in new `#[trace]` attributes. Removed: 651. Added: 653. The +2 is one two-line
  `Trace:` continuation (`FR-017-CON-1, FR-017-CON-2`) that my line tokenizer counted only on the
  added side. The -1 is `TC-025` on production fn `negotiate_kani_obligations`, dropped by
  agreement because tests bind TC-025. Every other file matches exactly, and the per-function
  check found no id moved between tests.
- AC3 (matrix): I re-ran `quire matrix` (quire 0.36.2, engine 0.50.2,
  QSPEC_DIR=/home/peter/dev/quire-specification) on origin/main and on HEAD. Both outputs are
  byte-identical to the coder's ir863-matrix-before.txt and ir863-matrix-after2.txt. Per
  criterion, the status and the binder file multiset are unchanged: 812 criteria, 445 tagged,
  14 tagged-by-ignored-test, 45 method-without-symbol, 308 untagged, 1141 binders on both sides.
- Out-of-scope files: kani_argument_order.rs, kani_generation_claims.rs and kani_witness_join.rs
  were edited in 6b06292 and restored in 8aa1f25. Their net diff against main is empty.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | An inline `// Trace: TC-023.` comment remains inside the body of `tc_023_proof_graph_artifact_validates_against_its_published_schema`. It is not a doc line, so AC1 is met as worded. But it is the only `Trace:` token left in scope, and it is redundant, because the enclosing fn already carries `#[trace("TC-023", "FR-028-AC-1")]`. Any grep audit of the deprecated form (AC1's own check) flags it. Drop the `Trace: TC-023.` prefix and keep the prose. The same pattern exists in other lanes (src/routed/contract_package_tests.rs:809, tests/it/oracle_generation.rs:554), which are outside this PR. | tests/it/bounded_kani_corpus.rs:751 |

## Verdict

AC1: met (no doc-line forms; one redundant inline comment, FND-001). AC2: met; the only id not
carried is the agreed TC-025 on a production fn. AC3: met; the matrix is unchanged per criterion.
Mergeable. FND-001 is low.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1: tests/it/bounded_kani_corpus.rs now says `// The old struct-order encoder wrote` without the redundant trace prefix; the canonical enclosing test attribute and all comment prose remain. |

## Disposition round 1 verification

The original review and finding above are preserved verbatim. The candidate removes the
redundant inline prefix and preserves both prose lines and the enclosing
`#[trace("TC-023", "FR-028-AC-1")]` attribute. No deprecated trace doc line remains in
the 34 changed Rust files. Per-function trace ID multisets match merged main after #408,
except the agreed TC-025 removal from the production function.

A fresh `quire matrix --scope /home/peter/dev/worktrees/ir863 --format json` using
`QSPEC_DIR=/home/peter/dev/quire-specification` was compared independently with the coder's
base and head JSON. Every criterion status and binder identity (path, qualified function
name and kind, ignoring moved line positions) matches: 812 criteria, 445 tagged, 14
tagged-by-ignored-test, 45 method-without-symbol, 308 untagged, 1141 binders. These are static
trace claims, not proof execution evidence. The three excluded integration files remain
unchanged. Source normalization confirms this delta introduces no runtime code or hollow
implementation/test body.

This is a disposition of the existing IR-863 delta review, not a new whole-repository gap
audit. The unchanged 327 baseline assurance deficits recorded in SR-12730 remain routed
to IR-726; they are not resolved by this conversion or hidden by the unchanged matrix.
The broad baseline audit is not repeated. Optional semantic review was not requested and
was not performed. Plan completion: not assessed.

Round 1 verdict: the original low finding is fixed; no new delta findings. IR-863's three
conversion acceptance checks remain met. Ready for the lead's aggregate gate after
review-artifact custody; no whole-repository assurance PASS is claimed.
