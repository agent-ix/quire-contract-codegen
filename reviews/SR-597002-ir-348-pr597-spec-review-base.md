---
id: SR-597002
title: "IR-348 PR #597 spec-review/base review (AD-004 immediate V1 Kani retirement)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #597 head 5e5245233fb8ab99000ce81563a6d66fb9ea5160 against origin/main 83721c07; spec/assurance/AD-004-cg-crate-layout.md (item map, generator fates, regression requirement, One input model, L-5, L-12, migration step 4 and 4a-4f, 4f criterion disposition, interim exceptions, risks, decisions); IR-348"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: reviews
---

## Summary

Ticket: IR-348. The PR edits only AD-004. It removes the V2-replacement and installed-control gates
on step 4f, so the V1 Kani bundle, the V1 clause arm and `KaniForm::V1BoundClause` can be deleted
now. The review test is the owner's rule that no acceptance criterion may be narrowed, and that
every criterion left unmet stays explicitly open. No AssuranceProfile applies: none exists under
`spec/`. The sub-analyses run are integrity (SR-597003) and dependency (SR-597004). EARS, object,
evidence, risk, failure-domain and scope lenses were not run, because the diff adds no requirement
statement, domain object or verification method.

`quire validate --scope . spec/assurance/AD-004-cg-crate-layout.md` passes at head, with module
registry warnings only.

Measured at head by trace tags (`grep` over `tests/` and `src/`):

- `tests/it/kani_generation.rs` holds 15 tests, and all of them drive the V1 bundle.
- The file is the sole tagged evidence for FR-005-AC-4, FR-015-AC-19, FR-015-AC-20,
  FR-015-AC-24, FR-015-AC-25, FR-015-AC-56 and FR-031-AC-20.
- `tests/it/capability_settlement.rs` tests tagged FR-019-AC-25, AC-26 and AC-33 build
  `KaniForm::V1BoundClause` (lines 2300-2414).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The 4f criterion disposition leaves out FR-019-AC-25, AC-26 and AC-33. Step 4f deletes `KaniForm::V1BoundClause` and its tests, and FR-019-AC-25's normative form set names the `BoundClause` precondition, postcondition and invariant roles, so it can no longer hold. The disposition does not record those criteria as open. | spec/assurance/AD-004-cg-crate-layout.md:1112-1122; spec/routed/functional/FR-019-capability-settlement.md; tests/it/capability_settlement.rs:2300-2414 |
| FND-002 | medium | The 4f disposition is incomplete. Deleting `kani_generation.rs` removes the only tagged evidence for FR-005-AC-4 and for FR-015-AC-19, AC-20, AC-24 and AC-56, but the disposition does not list them. It also leaves out SUITE-008, whose command filters on `kani_generation`. So the claim "none of these obligations is declared passed" does not cover all the evidence the PR loses. | spec/assurance/AD-004-cg-crate-layout.md:1112-1122; tests/it/kani_generation.rs; spec/core/matrix/suites.md:16 |
| FND-003 | medium | FR-015-AC-54 and AC-56, and the V1-bundle and V1-contract kinds in AC-53 and AC-58, apply only to `generate_kani_bundle`. Once 4f lands, no future evidence can back them. The AD still groups AC-53/54/58 with the V2 criteria as remaining "as written", and gives no owner disposition, unlike the FR-031 bundle rows. | spec/assurance/AD-004-cg-crate-layout.md:1113-1116; spec/kani/functional/FR-015 AC-53/54/56/58 |

## Verdict

Changes requested. The gate change itself is consistent with the stated rulings:

- No acceptance criterion text is edited, so none is narrowed in this PR.
- FR-015-AC-22 and AC-25 stay verbatim and are stated as open.
- L-5 is kept as an open obligation, not waived.
- FR-031 bundle rows, NFR-006-AC-1 and AC-4, `interface-001` and QI IT-011 are named as unresolved.

The defect is that the record of evidence loss is incomplete. FR-019's routed V1 forms and five
further criteria that only `kani_generation.rs` backs would lose their evidence without being listed
as open, which is the silent narrowing the rule forbids. V1-only criteria also need a recorded
disposition, not just "remain as written".

## Coverage

```yaml
scope:
- {id: AD-004:4f-disposition, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "This architecture decision changes the deletion gate, not any acceptance criterion. FR-015-AC-22/25 (V2 census), AC-40 to AC-49 (V2 contract clauses), and AC-53/54/58 (cover-last and emitting-entry guards) remain as written. FR-031-AC-11/18/19/20/21/23/24/25 and TC-044 step 10 still name bundle-specific arithmetic or refusal behavior"}
- {id: AD-004:4f, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "4f: immediate deletion of deprecated generate_kani_bundle, KaniArtifactBundle, ProofDependencyGraph, the V1 obligation arm, bundle validation, V1-only readers and callers, with the matching interface-001 edit."}
- {id: AD-004:L-5, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "Immediate V1 retirement may leave L-5 unmet until the QSL-owned exemplar migration and passing V2 run. L-5 is retained as an open obligation, not a deletion gate."}
- {id: AD-004:L-12, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "Step 4f removes these V1 paths and their V1-only tests immediately; the L-12 end-state remains absolute."}
- {id: FR-019-AC-25, path: spec/routed/functional/FR-019-capability-settlement.md, role: examined, excerpt: "The normative Kani form set is the complete V1 proof set ... precondition, postcondition and invariant BoundClause roles"}
- {id: FR-015-AC-54, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: examined, excerpt: "A V1 bundle harness (generate_kani_bundle ...) ends with exactly one kani::cover! after the call of its contract"}
- {id: FR-015-AC-56, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: examined, excerpt: "With the installed backend, a V1 bundle harness ... classifies Verified ... CoverUnsatisfied"}
- {id: FR-015-AC-22, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: examined, excerpt: "A declared proof-dependency census with an empty or duplicate identity ... is refused as a typed invalid input with no harness."}
- {id: FR-015-AC-25, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: examined, excerpt: "A valid declared census is folded into the harness identity ..."}
- {id: FR-031-AC-11, path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md, role: examined, excerpt: "generate_kani_bundle over the postcondition amount < 1000 implies amount + 1 <= 1000 ..."}
- {id: NFR-006-AC-4, path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md, role: examined, excerpt: "that line holds each of the six filters kani_obligations, skeleton_spine, kani_witness_join, bounded_kani_corpus, kani_generation and kani_batching."}
- {id: SUITE-008, path: spec/core/matrix/suites.md, role: examined, excerpt: "cargo test --locked --target-dir target-codex-backends --test it kani_generation -- --test-threads=1"}
```

## New findings (disposition pass 1)

Re-checked at head a6b88727da8e4d904b120f87a3cf698f766840ae against the previous head 5e524523.
FND-004 was in scope for the review pass and was missed there: that pass measured only
`tests/it/kani_generation.rs` and `capability_settlement.rs`, not the other tests that build the V1 arm.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | Step 4f also deletes the V1 clause arm, and these criteria's only tagged tests build `ObligationItem::BoundClause` on it: FR-015-AC-2, AC-8, AC-9, AC-10, AC-11, AC-12 and AC-13, plus the Kani-consumer half of FR-031-AC-10. They get no status change and no AD disposition row, and the matrix still marks them ✅ Covered. | tests/it/kani_obligations.rs:918, 1074, 1874, 2214; src/kani/generate/negotiate.rs:1910, 1934; tests/it/oracle_arithmetic.rs:1885-1995; spec/kani/matrix/tests.md:16-17; spec/oracle/matrix/tests.md:54; spec/assurance/AD-004-cg-crate-layout.md:1117-1128 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-019-AC-25/26/33 carry an OPEN prefix that retires the V1 `BoundClause` roles and keeps the surviving typed V2 obligations open, with the original text and verification unchanged. TC-030 records that the historical V1 tests do not count, and the AD 4f table has an FR-019 row owned by IR-348. |
| FND-002 | fixed | FR-005-AC-4 and FR-015-AC-19/20/24 are OPEN, and FR-015-AC-56 is RETIRED. SUITE-008 keeps its selector, with a note that V2 tests must replace the V1 ones. All of them now have rows in the AD 4f table. |
| FND-003 | still-open | AC-54/56 are correctly RETIRED as V1 bundle-only, and AC-58 is OPEN for surviving entries. But the AC-53 prefix lists surviving scalar, state-clause, frame-effect and corpus harnesses and leaves out the precondition kind, which survives through the V2 precondition harness (`src/kani/generate/clause/v2.rs`). The FR-015-AC-7 matrix row has the same gap. The surviving precondition cover-last obligation is therefore not stated as open. |
| FND-003 | fixed | Round 2, head 14e5808e: the FR-015-AC-53 prefix now reads "OPEN for surviving V2 precondition, scalar, state-clause, frame-effect and corpus harnesses". The FR-015-AC-7 matrix row and the AD 4f row name the V2 precondition family as well. The original criterion text and verification are unchanged. |
| FND-004 | still-open | Round 2, head 14e5808e: FR-015-AC-2 and AC-8 to AC-13 now carry OPEN prefixes, and their matrix rows are Open with no coverage carried forward. The AD 4f table has an FR-015-AC-2/8-13 and FR-031-AC-10 row, and FR-031-AC-10 has an OPEN prefix. One gap remains: spec/oracle/matrix/tests.md:54 still lists "FR-031-AC-7 through FR-031-AC-10" as ✅ Covered, while the new row at :55 marks FR-031-AC-10 🚧 Open. FR-031-AC-10 is therefore still asserted Covered, and its matrix status contradicts itself. |
| FND-004 | fixed | Round 3, head 954dfd9d: the Covered row at spec/oracle/matrix/tests.md:54 now reads "FR-031-AC-7 through FR-031-AC-9". FR-031-AC-10 appears only in its own 🚧 Open row at :55. No other matrix range covers it, and no other row changed. |
