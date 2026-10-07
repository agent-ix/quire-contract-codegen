---
id: "SR-2923"
title: "IR-629 gap analysis: process-provider settlement matrix and coverage"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen branch spec/ir-629-input-contract (frozen head recorded in the IR-629 Linear review marker, not here); spec/routed/functional/FR-019-capability-settlement.md, spec/routed/functional/FR-022-routed-generation.md, spec/routed/functional/FR-026-backend-adapter-contract.md, spec/routed/matrix/TC-046-process-provider-settlement.md, spec/routed/matrix/tests.md, spec/tests.md; context src/routed/capability.rs, src/routed/generate.rs, tests/it/capability_settlement.rs"
---

# SR-2923: IR-629 gap analysis

## Summary

Ticket: IR-629. Planless. Plan completion: not assessed. This change touches spec only, so I
compared the computed matrix (`quire matrix --format tsv`, main against the branch) with the
hand-kept routed matrix and with the code that the amended criteria describe. No cargo, tests
or Kani were run.

Computed matrix delta against main: FR-019-AC-11 and AC-12 text changed and stay untagged.
FR-022-AC-1 text changed and stays method-without-symbol. FR-022-AC-4 and AC-5 text changed
and stay tagged to `tests/it/routed_generation.rs:251` and `:294`. FR-022-AC-17 and FR-026-AC-5
were added, untagged. No criterion lost a tag.

Units examined: FR-019-AC-1, FR-019-AC-11, FR-019-AC-12, FR-019-AC-13, FR-022-AC-4,
FR-022-AC-5, FR-022-AC-17, FR-026-AC-5, TC-046 steps 1 to 4 and Status, the routed matrix rows.

## Verdict

Not merge-ready from this lens. Two amended criteria keep a Covered status for clauses that no
test can exercise until the variant exists. TC-046 misses several truth-table cells. Gating
labels do not match what is actually gated.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Coverage is inflated. FR-022-AC-4 gained a `Process(id)` identity-mismatch branch, and FR-022-AC-5 gained "`Process` needs no generation context". The routed matrix still marks FR-022-AC-2 through AC-5 Covered by TC-033, and the computed matrix still shows both as tagged to tests that predate the variant. | spec/routed/functional/FR-022-routed-generation.md:255-256; spec/routed/matrix/tests.md:14 |
| FND-002 | low | FR-019-AC-1 ("every variant reaches an arm") stays Covered by TC-030, whose test iterates `BackendKind::ALL`. FR-019 now excludes `Process` from `ALL`, so that census can never reach the process arm, and nothing notes that the Covered status no longer spans every variant. | spec/routed/functional/FR-019-capability-settlement.md:118-120, 221; spec/routed/matrix/tests.md:12 |
| FND-003 | low | The TC-046 step 2 table omits these cells: bounded with empty `bounds` on an `unbounded`-only advertisement; unbounded, `finite_bound_available=false`, uncovered domain, on `bounded`-only; and both modes advertised, for both bounded and unbounded items. It also has no row for a named process backend. | spec/routed/matrix/TC-046-process-provider-settlement.md:33-41 |
| FND-004 | low | The gating labels are inaccurate. All of FR-019-AC-12 is "gated on QSL-654", but only the non-empty bounded-domain rows need `ProofBound.kind`. The unbounded rows and the invariance rows can be tested with CG's in-process types now. FR-022-AC-17 and FR-026-AC-5 are Planned in the matrix but lack the "PLANNED (IR-629)" prefix that AC-11 and AC-12 carry. | spec/routed/functional/FR-019-capability-settlement.md:230; spec/routed/functional/FR-022-routed-generation.md:268; spec/routed/functional/FR-026-backend-adapter-contract.md:62 |
| FND-005 | low | FR-026-AC-5 asserts that no adapter trait is implemented for `Process(id)`, which is a compile-time absence, yet its verification is Test (TC-046). TC-046 has no step that could fail if such an impl existed. | spec/routed/functional/FR-026-backend-adapter-contract.md:62; spec/routed/matrix/TC-046-process-provider-settlement.md:67-71 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | TC-046 has no row for a bounded item with an uncovered bounds[].kind on a provider advertising both modes, so a mutant that skips the domain check whenever 'unbounded' is advertised survives. | spec/routed/matrix/TC-046-process-provider-settlement.md:33-47 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | TC-046's two unbounded rows that advertise 'unbounded' leave finite_bound_available unset, so a mutant that gates 'supported' on it survives whichever value the test picks. | spec/routed/matrix/TC-046-process-provider-settlement.md:48-49 |

## Dispositions

Round 1, against the fix-round head recorded in the IR-629 Linear dispositions marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fix commit 'spec(CG): resolve IR-629 process review findings' |
| FND-002 | fixed | fix commit 'spec(CG): resolve IR-629 process review findings' |
| FND-003 | fixed | fix commit 'spec(CG): resolve IR-629 process review findings' |
| FND-004 | fixed | fix commit 'spec(CG): resolve IR-629 process review findings' |
| FND-005 | fixed | fix commit 'spec(CG): resolve IR-629 process review findings' |
| FND-006 | fixed | round 2: fix commit 'spec(CG): close IR-629 review disposition findings' |
| FND-007 | fixed | round 3: fix commit 'spec(CG): clarify routed-item constructor invariants' |
