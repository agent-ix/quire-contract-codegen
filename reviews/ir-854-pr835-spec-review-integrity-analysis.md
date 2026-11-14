---
id: SR-17940486709564847047989
title: "IR-854 PR #835 spec-review integrity"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen PR #835 at f585f1a4, git diff origin/main...HEAD; IR-854; spec/kani/functional/FR-057-integer-division-scalar-harnesses.md, spec/kani/matrix/TC-071-integer-division-scalar-harnesses.md, spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/core/functional/FR-038-typed-operator-catalog.md, spec/routed/matrix/TC-033-routed-generation.md, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, spec/kani/matrix/tests.md, spec/oracle/matrix/tests.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-057
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-071
    type: reviews
---

## Summary

Ticket: IR-854. This analysis checks ids, cross-references, status records and consistency with
the code. FR-057, TC-071 and FR-014-AC-81 are new, unused ids. The `verifies` and `depends_on`
edges resolve. The two matrix rows, the spec.md row and the TC-071 step-to-AC mapping agree. The
FR-038-AC-4 count of seven entries is right. The Status cites IR-1541 and the IR-269 MVP ruling.
The FR-015-AC-57 and AC-109 amendments agree with FR-057. `make spec` exits 0.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | low | Fixtures that still treat `rem` as underivable are not amended. FR-014-AC-81 says the underivable-sibling fixtures that use `rem` "move to an identity that is still refused". TC-033 still routes `quire.op.integer.rem` as the underivable sibling and expects `no_derivable_claim` (lines 29, 157, 175, 235, 254). TC-023 still says "the separate `quire.op.integer.rem` test requires an `Underived`" (line 183). Both go stale once AC-81 lands. Add the same "amended by IR-854" note given to FR-015-AC-109, or name the replacement identity. | spec/routed/matrix/TC-033-routed-generation.md:157, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md:183 |
| FND-002 | low | FR-038-AC-1 still speaks of "the four integer arithmetic identities shared by scalar oracle and scalar Kani". After FR-057 there are seven, as the AC-4 amendment says. Add the same amendment note to AC-1. | spec/core/functional/FR-038-typed-operator-catalog.md:68 |
| FND-003 | low | FR-057 misdescribes the current code. It says `ScalarOperation` in `src/kani/generate/scalar.rs` names "only add, subtract, multiply and negate". It also has `Implies` (`scalar.rs:58-64`). | spec/kani/functional/FR-057-integer-division-scalar-harnesses.md:40-43 |
| FND-004 | low | The AC-6 native refusal has no named variant: "a typed native-plan refusal naming the node and the operation identity". Precedent rows name the `UnsupportedObligation` variant. Two implementers would pick different shapes, and the AC-6 test cannot assert the exact variant. Name it. | spec/kani/functional/FR-057-integer-division-scalar-harnesses.md:233-238, spec/kani/functional/FR-057-integer-division-scalar-harnesses.md:298 |
| FND-005 | low | AC-1 lists `V2ClauseNotTractable` among the refusals a claim can settle with through `generate_routed` and `generate_routed_native`. FR-057 Behavior and FR-056-AC-15 apply the shipped scalar table at run time, through the decided run wrappers, not at generation. A generation-time `V2ClauseNotTractable` for a scalar harness is not specified anywhere. Drop it from AC-1, or say which generation-time table produces it. | spec/kani/functional/FR-057-integer-division-scalar-harnesses.md:293 |

## Verdict

These integrity findings do not block a merge by themselves. All are short text fixes that should
ride in the same fix round as the base review's findings.

## Dispositions

Round 1, checked at b720c916 against the diff f585f1a4..b720c916.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | TC-033 line 30 and TC-023 line 184 carry "Amended by IR-854 (planned)" notes. The underivable-sibling node moves to an identity FR-014 still refuses, for example `quire.op.numeric.narrow`, and the `div`/`rem` refusal tests are superseded. |
| FND-002 | fixed | FR-038-AC-1 now says the shared integer arithmetic identities become seven when FR-057 lands, adding `div`, `rem` and `mod`. |
| FND-003 | fixed | FR-057 line 44 adds "(`ScalarOperation` also has `Implies`, for Boolean implication)". |
| FND-004 | fixed | The refusal is named `UnsupportedObligation::NativeDivisionPlanUnavailable { node_id, operation_identity }` in Routes, AC-1, AC-6 and TC-071 step 6. |
| FND-005 | fixed | AC-1 drops `V2ClauseNotTractable` from the generation dispositions. It says the shipped scalar table applies at run time through the decided run wrappers (FR-056-AC-15, FR-057-AC-9). |
