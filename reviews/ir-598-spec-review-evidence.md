---
id: SR-12203
title: "Evidence review of IR-598 per-family corpus retirement (CG #423 delta)"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen; PR #423 delta f22c59d9..1163a768 (IR-598): spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/assurance/AD-001-codegen-architecture.md, spec/assurance/AD-004-cg-crate-layout.md, spec/core/functional/interface-001-codegen-api.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/spec.md, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, spec/kani/matrix/tests.md"
review_set: subset
date: 2026-10-17
---

## Summary

Ticket: IR-598. Checked that the verification method and evidence fit FR-015-AC-109/110/111 and the TC-023 retirement procedure. 'Test (TC-023)' fits the absence searches and the refusal checks. The 'green before deletion' preconditions are temporal and rely on #[ignore] kani-lane tests, so no post-merge test can check them as written.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | 'Green before the deletion' (AC-110, and AC-109's 'green before the arm is deleted', TC-023) is a temporal precondition that no post-merge test can check. The real-Kani tests are #[ignore] kani-lane tests (for example kani_obligations_symbolic_domains.rs:2077), so default CI never runs them. Specify the evidence: the named kani-lane test is run through `make kani` or `make kani-gate` on the slice PR's head, and its result is recorded on the ticket. Use the method Test + Inspection. | FR-015-AC-110, FR-015-AC-109, TC-023 (spec/kani/functional/FR-015-bounded-kani-obligations.md:800-801) |

## Verdict

The search-based absence checks and the refusal tests are appropriate Test evidence. The real-Kani preconditions need an evidence form that someone can audit.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified at 455b1b1 (round 1) |

## Refresh assessment

**PASS for refreshed spec delivery.** The verification cells of AC109..111 remain Test and Inspection, and TC023 retains recorded slice-head make kani/make kani-gate backend evidence for the collection and graph gates. The advisor reports authored Test, mismatch=false and inconclusive=false for each of AC109..111; the catalog also recommends Inspection for AC109. Judgment: Test plus Inspection still fits the future public dispositions, deletion checks and temporal prerequisites. No rerun is implied by historical IR268 results or the original remainder inference; this refresh supplies no new proof credit. No new finding or fix round; every prior finding retains its terminal fixed disposition. Original findings, verdict and dispositions above are historical and remain verbatim.

The frozen candidate changes Markdown only. No source, test, manifest, workflow or source-embedded input changes. No Cargo, Kani or measurements ran; no current proof result is claimed. No applicable AssuranceProfile exists in spec/.

### Refresh scope

```yaml
scope:
- id: FR-015-AC-109
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'PLANNED (IR-598 arithmetic slice; gated on IR-858, AD-004 step 4b, merged
    first, because the V2 scalar path must already render through `HarnessSpec` and
    `render.rs` for the deletion to leave the family served). After this slice no
    source under `src/` contains the corpus `Arithmetic` request variant and arm,
    `BoundedCorpusFamily::DefinednessArithmetic`, `prepare_checked_arithmetic`, `checked_method`,
    `render_arithmetic_*` or `lower/definedness_arithmetic.rs` and their re-exports,
    and no test in `tests/` drives them. The slice deletes only what the arithmetic
    family owns: the corpus replay (`Bo'
- id: FR-015-AC-110
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-598 collection slice; gated on IR-858, AD-004 step 4b, and
    on AD-004 step 4h, because the collection family renders through `HarnessSpec`
    and `render.rs` only after 4h). The gate opens when the AC-99 installed-backend
    controls are green, with their tagged tests present (`tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies`
    and the symbolic-domain tests tagged FR-015-AC-96 and FR-015-AC-99 in `tests/it/kani_obligations_symbolic_domains.rs`).
    Evidence for a real-Kani precondition is a recorded `make kani` or `make kani-gate`
    run on the slice PR's head that is green
- id: FR-015-AC-111
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-598 graph and final slice; gated on IR-858, AD-004 step 4b,
    on FR-015-AC-109 and FR-015-AC-110 having landed, and on IR-805, whose graph claims
    need CG integration of the merged IR-851/IR-852 accessors and QSL-820 real-package
    population emission (after QSL-819)). Graph claims are served by FR-015-AC-98
    and FR-015-AC-105 to FR-015-AC-108, and FR-015-AC-106's real-Kani graph replay
    is the replacement for the corpus graph replay; Evidence for a real-Kani precondition
    is a recorded `make kani` or `make kani-gate` run on the slice PR's head that
    is green, cited in the PR (the real-Kani
- id: TC-023-retirement
  path: spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md
  role: examined
  excerpt: 'Retirement procedure (IR-598), per slice, only after IR-858 (step 4b)
    has landed. Arithmetic (FR-015-AC-109): require a div claim to settle `Unsupported(OperationNotRendered)`
    with no harness and a rem claim to stay a refused identity; search `src/` and
    `tests/` for each deleted arithmetic name, and confirm the corpus replay is still
    present; confirm the AC-84/85 divide and remainder loss is recorded as a Rust
    checked-arithmetic literal check and points to IR-854. Collection (FR-015-AC-110):
    require step 4h landed and a recorded `make kani` or `make kani-gate` run on the
    slice PR''s head with t'
```
