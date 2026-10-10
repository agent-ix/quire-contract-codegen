---
id: SR-4801
title: spec-review/base review of IR-338 spec diff
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen@f7edb3a089657ec40e63b5ddce997587dfa47da9; seven
  changed spec files
review_set: subset
---

## Summary

Reviewed the IR-338 spec-only diff at f7edb3a089657ec40e63b5ddce997587dfa47da9. Ticket: IR-338.

## Verdict

**CONDITIONAL** — The findings below need correction.

## Examined Units

- `FR-004-AC-1` (examined; `spec/evidence/functional/FR-004-vacuity-evidence.md`): An evaluated implication whose consequent is unobserved yields a vacuity finding even when every oracle return was true; the consuming coverage obligation denies success for that clause.
- `FR-004-AC-4` (examined; `spec/evidence/functional/FR-004-vacuity-evidence.md`): Successful analysis verifies the bound population, source/map, and native execution bindings.
- `FR-004-AC-8` (examined; `spec/evidence/functional/FR-004-vacuity-evidence.md`): Adverse coverage and non-success native execution cannot discharge the coverage obligation merely because a report serialized successfully.
- `FR-004-AC-9` (examined; `spec/evidence/functional/FR-004-vacuity-evidence.md`): Complete bound observations without a native campaign run are reported as observations only and never as a passed coverage result. Valid informational-only populations retain their references as no executable work.
- `FR-004-AC-10` (examined; `spec/evidence/functional/FR-004-vacuity-evidence.md`): PLANNED. A native-run coverage result binds the analyzed source map and generated source to the ones its producer used, matches their requirement and revision to the campaign, and reports the LLVM producer tool and version; a changed or missing run/map/source binding or producer identity yields non-success without coverage discharge.
- `FR-004-AC-11` (examined; `spec/evidence/functional/FR-004-vacuity-evidence.md`): PLANNED. The consuming report retains vacuity and native outcome as evidence for IR FR-045; an oracle-success run with an unobserved consequent denies the coverage obligation, and LLVM probe observations alone produce no completed-proof credit or separate eligible denominator.
- `FR-019-AC-3` (examined; `spec/routed/functional/FR-019-capability-settlement.md`): On the Kani arm, an unbounded extent against a `bounded`-only advertisement settles `requires-bound` when a finite bound is available, rather than `unsupported`; with no finite bound available it settles `unsupported`, warned, with `unsupported_projection`/`unbounded-extent`; and it never settles `supported`.
- `FR-019-AC-24` (examined; `spec/routed/functional/FR-019-capability-settlement.md`): PLANNED (IR-338). Given QSL FR-335's `value-validity` claim rooted at an unbounded `s: Set<Int[0,9999]>`, an empty candidate registry settles `unsupported` with a warning naming `value-validity` and emits no artifact or Kani outcome; one bounded-only Kani candidate with `finite_bound_available=true` settles `requires-bound` without substituting a bound or emitting an artifact; a separately requested `ProofBound::Cardinality{maximum: 8}` item keeps its own request index and bounded disposition without changing the unbounded item's result; and an unboundable added root with `finite_bound_availab
- `TC-006` (examined; `spec/evidence/matrix/TC-006-vacuity.md`): The planned aggregate case also consumes a native campaign whose oracle returns all succeed while
- `TC-030` (examined; `spec/routed/matrix/TC-030-capability-settlement.md`): For planned FR-019-AC-24, use the QSL FR-335 request for the `+` value-validity
- `interface-001` (examined; `spec/core/functional/interface-001-codegen-api.md`):     status: planned; public IR-owned bound population is available, native run-result contract and aggregate analysis integration remain pending
- `FR-004-matrix` (examined; `spec/evidence/matrix/tests.md`): | FR-004 | FR-004-AC-10, FR-004-AC-11 | TC-006 | 🚧 Planned; native run/source-map binding, LLVM producer identity and consuming obligation are not implemented |
- `FR-019-matrix` (examined; `spec/routed/matrix/tests.md`): | TC-030 | QSL FR-335 concrete collection control | Integration | P0 | FR-019-AC-24 | 🚧 Planned; QSL owns source fixture and quire-integration owns composed run |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The generic TC-030 row marks FR-019-AC-3 Covered, but Quire computes it untagged; the new Planned TC-030 row leaves the same test case with conflicting completion claims. Mark the generic row truthfully and add a real AC-3 trace only when its assertions cover both boundable and unboundable Kani branches. | spec/routed/matrix/tests.md:51 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 22a020a7511a6e251ed2f06099f3bd5f60f3de8c |
