---
id: SR-12202
title: "Scope-boundary review of IR-598 per-family corpus retirement (CG #423 delta)"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen; PR #423 delta f22c59d9..1163a768 (IR-598): spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/assurance/AD-001-codegen-architecture.md, spec/assurance/AD-004-cg-crate-layout.md, spec/core/functional/interface-001-codegen-api.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/spec.md, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, spec/kani/matrix/tests.md"
review_set: subset
date: 2026-10-17
---

## Summary

Ticket: IR-598. Checked the slice gates and boundaries (IR-858/4b, 4h, IR-268, IR-805 and its upstream owners, the no-wrapper rule) against the planner rulings on IR-598 and against AD-004 4b/4g/4h. Graph ownership (QSL-819, then IR-852 and IR-851, then IR-805) and the IR-854 division handoff are correctly outside CG. The collection slice's gate set is inconsistent with AC-109's own reasoning and with AC-95's end state, and its IR-268 gate cannot be evaluated from the spec.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-109 gates on 4b 'because the V2 path must already render through HarnessSpec and render.rs for the deletion to leave the family served'. AC-110 gates on 4b only, yet collection.rs renders its own template until step 4h (AD-004 :1043-1049). AC-95's end state requires every proof-eligible case to render through HarnessSpec, and also says AC-95 holds once AC-109..111 hold. AC-95 could therefore be claimed with 4h unlanded. Either gate AC-110 (or AC-95) on 4h, or narrow AC-95's HarnessSpec clause. | FR-015-AC-110, FR-015-AC-95, FR-015-AC-109, AD-004 (spec/kani/functional/FR-015-bounded-kani-obligations.md:801) |
| FND-002 | medium | The gate 'AC-96, -97 and -99 to -102 being backed (IR-268)' cannot be evaluated. IR-268 is Done, collection.rs and tagged tests exist (including the kani-lane tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies), yet FR-015:788-793 still mark those rows PLANNED, and tests.md says 'no symbolic collection or graph harness exists'. Name the concrete evidence that opens the gate, or update the stale status markers. | FR-015-AC-110, FR-015-AC-96, FR-015-AC-99, FR-015-AC-102 (spec/kani/functional/FR-015-bounded-kani-obligations.md:801) |
| FND-003 | low | The no-wrapper rule has no checkable form. A name search catches only the names it lists, so an adapter under a new name, for example one that builds a V2 request from `CheckedArithmeticRequest`, `CollectionQuery` or `GraphRequest`, passes. Add an inspection: each slice adds no interface-001 operation, and no src/ item converts a finite-input request type into a V2 generator call. | FR-015-AC-95, FR-015-AC-109, FR-015-AC-110 (spec/kani/functional/FR-015-bounded-kani-obligations.md:786) |

## Verdict

The boundaries are mostly right: CG owns no population input, division moves to IR-854, and every slice is gated on IR-858. Fix the 4h gap and give the IR-268 gate and the no-wrapper rule a checkable form.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified at 455b1b1 (round 1) |
| FND-002 | fixed | Verified at 455b1b1 (round 1) |
| FND-003 | fixed | Verified at 455b1b1 (round 1) |

## Refresh assessment

**PASS for refreshed spec delivery.** IR858 gates every implementation slice and owns step4b only. Collection additionally waits for step4h; graph/final waits for AC109+110 and IR805, with merged IR851/852 accessors still needing CG integration and QSL820 real-package population emission after QSL819. Live tracker confirms IR851/852/QSL819 Done and QSL820/IR858/IR805 In Progress. The owner ruling retains holds on 4c–4e/4f/5/6 and the later last-caller note leaves 4f ownership with its owner. No upstream authority is restated locally and no arithmetic replay bridge is introduced. No new finding or fix round; every prior finding retains its terminal fixed disposition. Original findings, verdict and dispositions above are historical and remain verbatim.

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
- id: FR-015-retirement-gates
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'FR-015-AC-96 to FR-015-AC-102 retain the collection implementation and
    partial

    statuses recorded below; graph clauses remain PLANNED. This retirement plan

    adds no proof credit or current backend results.

    FR-015-AC-109 to FR-015-AC-111 state the slices and FR-015-AC-95 their end

    state. QSL-353 establishes that

    QSL-emitted packages read through Contract IR; it does not supply the symbolic

    corpus or the graph population input. The graph slice also waits for IR-805: CG
    integration of the merged IR-851/IR-852

    accessors and QSL-820 real-package population emission (after QSL-819). Retirement
    of each'
- id: AD-004-last-caller
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "it until 4g (`bounded_kani_corpus.rs`), so 4f leaves it in place. The\
    \ items of\n     `census_validation.rs` (`validate_dependencies`, `KaniDiagnostic`,\
    \ `KaniErrorCode`) are removed once their last caller is gone: the V1 bundle path\n\
    \     (removed at 4f) and the corpus (removed at 4g), or earlier when the V2 census\
    \ input replaces them. It is not in `kani/census.rs`"
- id: AD-004-4g
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "- 4g: the corpus, retired per family in three slices with no compatibility\
    \ wrapper and no second\n     public generator (IR-598, FR-015-AC-109 to AC-111);\
    \ step 4b (IR-858) gates every slice. The\n     arithmetic slice deletes the `Arithmetic`\
    \ arm and `definedness_arithmetic.rs`; V2 does not\n     serve divide or remainder\
    \ (div settles `Unsupported(OperationNotRendered)`, rem stays a\n     refused\
    \ identity) until IR-854, which also owns `mod`. The collection slice, gated also\
    \ on\n     step 4h, deletes the `Collection` arm and `bounded_collections.rs`.\
    \ The graph slice, gated\n     on IR-805, deletes t"
```
