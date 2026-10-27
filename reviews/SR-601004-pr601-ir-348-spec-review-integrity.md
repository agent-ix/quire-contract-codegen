---
id: SR-601004
title: "IR-348 PR #601 spec-review/integrity (cross-spec consistency after V1 deletion)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; PR #601 head 64992f5be8aebb7d036fba5491fa9ef61af70d29 against merge-base 46dbaccacbd96f55654fc4b331a9d42883e70a2c; unchanged spec files that describe code or tests this PR changes: spec/kani/functional/FR-041-claimed-module-proof-gate.md, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md, spec/kani/matrix/TC-056-claimed-module-proof-gate.md, spec/kani/matrix/tests.md, spec/replay/functional/FR-032-routed-scalar-replay-binding.md, spec/kani/matrix/TC-036-generated-subject-abi.md; IR-348"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-041
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/NFR-006
    type: reviews
---

## Summary

Ticket: IR-348. This integrity lens grepped `spec/` for every surface the PR deletes or changes:

- the V1 bundle, `BoundClause`, `kani_witness_join` and `kani_argument_order`;
- the claim hold;
- the Kani filters.

It then checked each hit against the code at head. Retirement prose that is already marked
historical (FR-015, FR-019, FR-031, TC-030, AD-004) is consistent. Three groups of statements are
not: they describe the pre-PR state as current.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Four places still say the FR-041 V1 claim hold is "PLANNED", "not implemented at this spec head", and that the claim test takes the V1 bundle source from `kani_generation::guard_sources()[0]`: FR-041 Status, the NFR-006-AC-19 note, the TC-056 procedure preamble, and the FR-041 and TC-056 rows of the kani matrix. This PR implements the hold (`kani_claims.json` hold row, `cg_gates.rs::claim_counts`, `validate_claim_holds`) and points `guard_sources()` at the V2 scalar harness. | spec/kani/functional/FR-041-claimed-module-proof-gate.md:96-110; spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:209-219; spec/kani/matrix/TC-056-claimed-module-proof-gate.md:22-32; spec/kani/matrix/tests.md:80, 127 |
| FND-002 | medium | NFR-006 Scope still says the lane is selected by "the filters `kani_obligations`, `skeleton_spine`, `kani_witness_join`, ...", and that it writes bundles "through `src/publication` (`kani_witness_join`)". The PR removed `kani_witness_join` from `KANI_FILTERS` and from `EXPECTED_KANI_FILTERS`, and deleted the module. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:90-100 |
| FND-003 | low | FR-032's traceability table cites `tests/it/kani_witness_join.rs` and `kani_argument_order.rs` as tests that currently "exercise the function schema". Both are deleted by this PR. TC-036 likewise cites `kani_argument_order.rs` as the V1 emission-order evidence. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:404; spec/kani/matrix/TC-036-generated-subject-abi.md:84 |

## Verdict

Changes requested (medium). These are status and scope statements only. No AC text needs to
change, and no item is wrongly marked PASS. All three groups lag the code, though, and readers will
trip on them:

- **FND-001:** move the hold from PLANNED to implemented-but-Open. FR-041 AC-1 to AC-5 remain
  unfulfilled for the held row.
- **FND-002:** drop `kani_witness_join` from the NFR-006 scope.
- **FND-003:** mark the FR-032 and TC-036 citations historical.

## Coverage

```yaml
scope:
- {id: FR-041, path: spec/kani/functional/FR-041-claimed-module-proof-gate.md, role: examined, excerpt: "Open; the V1-sourced module claim hold is PLANNED under IR-348's immediate V1 deletion code slice and is not implemented at this spec head. The current test tests/it/kani_generation_claims.rs takes kani_generation::guard_sources()[0], which is a V1 bundle source"}
- {id: NFR-006-AC-19, path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md, role: examined, excerpt: "The IR-348 immediate V1 deletion code slice must implement the hold in or before its deletion PR; at this spec head it remains PLANNED."}
- {id: NFR-006, path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md, role: examined, excerpt: "selected by the filters kani_obligations, skeleton_spine, kani_witness_join, bounded_kani_corpus, kani_generation and kani_batching"}
- {id: NFR-006-AC-4, path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md, role: examined, excerpt: "OPEN during IR-348 migration: the kani_generation filter remains and must select V2 tests after V1-only tests retire. Original criterion retained"}
- {id: TC-056, path: spec/kani/matrix/TC-056-claimed-module-proof-gate.md, role: examined, excerpt: "The hold is PLANNED at this spec head. The held V1 row remains Open, and all non-held controls stay required."}
- {id: FR-032, path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md, role: examined, excerpt: "tests/it/kani_witness_join.rs, kani_argument_order.rs and skeleton_spine.rs exercise the function schema"}
- {id: TC-036, path: spec/kani/matrix/TC-036-generated-subject-abi.md, role: examined, excerpt: "harnesses of the V1 BoundClause arm (tests/it/kani_argument_order.rs, traced to"}
- {id: FR-015, path: spec/kani/matrix/tests.md, role: context_only, excerpt: "AC-54/56 retired as V1 bundle-only under the 2026-10-27 IR-348 ruling."}
- {id: spec.md:subsystem-registry, path: spec/spec.md, role: examined, excerpt: "IR-348 retires the V1 bundle, clause arm, V1-only readers and public operation immediately at AD-004 step 4f ... No compatibility entry remains."}
```

## Dispositions

Re-checked at exact head 5380318cb380788ba0512e8765d2faa651078965, including the conflict resolution of merge 3bb40d0c with main's #600 FR-041-AC-6 to AC-10 text.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-041 Status, the NFR-006-AC-19 note, TC-056 (preamble and Status) and the kani matrix FR-041 and TC-056 rows now say IR-348 implements the bounded hold, the claim names are compared with the merge base, and guard_sources()[0] selects a V2 scalar harness with no FR-041 proof credit. The merge kept main's IR-1047/IR-1049 additional-claim text, and no AC row changed against main. |
| FND-002 | fixed | NFR-006 Scope now lists "kani_obligations, skeleton_spine, bounded_kani_corpus, kani_generation and kani_batching", and the src/publication (kani_witness_join) clause is gone. |
| FND-003 | fixed | FR-032:404 now says "The removed kani_witness_join.rs and kani_argument_order.rs were V1-only historical controls", and TC-036:84 marks kani_argument_order.rs "removed under IR-348". |
