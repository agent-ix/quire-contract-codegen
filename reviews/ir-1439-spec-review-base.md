---
id: SR-2026110283896
title: "IR-1439 PR #765 FR-050 quantity Kani row specification review"
type: SpecReview
analysis: base
date: 2026-11-10
scope: "agent-ix/quire-contract-codegen; PR #765 @ 51d9a3f9; IR-1439; spec/kani/functional/FR-050-quantity-scalar-harnesses.md, spec/kani/matrix/TC-064-quantity-scalar-harnesses.md, spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/functional/FR-025-generated-subject-abi.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/matrix/tests.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-050
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-064
    type: reviews
---

## Summary

Ticket: IR-1439. Spec-only PR adding FR-050 and TC-064 (planned Kani row for quantity `add`/`sub`/`mul`/`div`/`pow`/comparisons) plus one-line amendments in FR-015, FR-025, FR-028, spec/spec.md and spec/kani/matrix/tests.md. Head 51d9a3f9. `quire validate` on every changed file passes (one pre-existing FR-015 line 927 grammar warning). Checked against FR-014 quantity oracle criteria (AC-6, AC-48..AC-52, AC-58..AC-65), TC-030's capability-settlement rows, the FR-046 Rational precedent, QSV `quantity.rs` and sibling PR #764 (FR-049, enum).

## Verdict

CHANGES REQUIRED. FR-050's operation list, oracle reuse (#750-#754 compound and carried forms via FR-014-AC-52/58/59), its planned status and the IR-1443 gate are stated consistently in FR-050, TC-064, tests.md and spec.md. The FR-015 refusals row is placed in the composite-equality shadow table and contradicts the live TC-030 settlement; the FR-014-AC-51 supersession is mis-cited; no witness/replay contract backs the playback the mutant controls require; and the shared lines conflict textually with #764.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new FR-015 row sits in the composite-equality shadow refusal table ('Shapes the composite-equality shadow does not support'), but a quantity scalar operation is not a shape that shadow reaches. It also asserts a disposition (`unsupported` naming the carrier, with no typed reason name, unlike every other row) that contradicts the live settlement for the same claim: TC-030's Quantity scalar row settles every quantity scalar `expression` claim as `OperationNotRendered` (`ScalarLoweringRefusal::NoRenderer`, Planned IR-269), which this PR does not amend. Move the carrier-absent disposition to the capability-settlement surface (TC-030 row / FR-022) with a typed reason, or state that `OperationNotRendered` stays until IR-1443 lands. | spec/kani/functional/FR-015-bounded-kani-obligations.md:767 |
| FND-002 | medium | FR-050 says 'AC-51 is superseded for non-conversion quantity operations' and calls itself the row 'that FR-014-AC-51 leaves uncredited', but FR-014-AC-51 governs only `quire.op.quantity.convert`; non-conversion quantity settlement is TC-030's Quantity scalar row. The supersession should name the TC-030 row (and FR-022 settlement), and leave AC-51 unchanged since conversion stays out of scope. | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:105 |
| FND-003 | medium | AC-8 requires mutants rejected 'with actual solver playback', but FR-050 has no counterexample decoding/replay criterion for symbolic quantity magnitudes (physical bindings, roles, canonical magnitude, domain checks, owner replay), unlike the FR-046-AC-9 / FR-025-AC-10 Rational precedent. FR-025-AC-7 now refers to 'the planned FR-050 row', yet FR-025 gains no FR-050 ABI row or prerequisite sentence like the Rational one. Add a witness/replay AC (or defer it explicitly to IR-1443 and FR-025). | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:100 |
| FND-004 | medium | Sibling PR #764 (IR-1438, FR-049 enum) edits the same lines: a three-way `git merge-tree` of the two heads conflicts in FR-025-AC-7, the FR-028 ShadowFamilyNotBuilt bullet, spec/kani/matrix/tests.md (top row) and spec/spec.md (table row after FR-046); FR-015 auto-merges. The two PRs also use different patterns for the FR-015 shadow table: #764 amends the ShadowFamilyNotBuilt row to carve enum leaves out under FR-049, while #765 keeps quantity leaves ShadowFamilyNotBuilt and adds a separate scalar-operation row. Whichever lands second must rebase and the table should use one pattern. Not edited here. | spec/kani/functional/FR-015-bounded-kani-obligations.md:767 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-050 | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:23 | When an admitted request selects a reader-admitted `quantity` scalar operation (`add`, `sub`, `mul`, `div`, `pow` or a comparison) over operands whose unit and magnitude domains are finite, and the quire-exact quantity carrier dependency K-EXACT-quantity (ticket IR-1443) is available, CG SHALL generate a bounded Kani harness that proves the generated quantity oracle of [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) agrees with the native quantity expression over every admitted symbolic magnitude tuple. This is the quantity row of the IR-269 scalar renderer that FR-014-AC-51 l |
| FR-050-AC-1 | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:93 | Each admitted quantity `add`, `sub`, `mul`, `div`, `pow` and comparison node selects a harness that names the operation identity, operand forms and units from the admitted node, and compound-unit and carried-result forms admitted by FR-014 (#750-#754) are accepted without a new operand form. A conversion node is not selected here and settles as FR-014-AC-51 states. |
| FR-050-AC-3 | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:95 | The generated oracle and an independent native expression over public owner quantity operations agree, from matching entry state and limits, on completed value, identified result unit, typed outcome, ordered charges, denied next charge and consumed counters, including zero, exact and short budgets. The reference neither runs the generated oracle twice nor bridges through Runtime. |
| FR-050-AC-6 | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:98 | When K-EXACT-quantity (IR-1443) is unavailable, the item is `unsupported` naming that carrier dependency with no harness and no Verified claim. Unsupported operand form, absent native expression and a quantity leaf of a composite each yield their own named disposition, with total per-item accounting under FR-039 and an unchanged supported sibling. |
| FR-050-AC-8 | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:100 | Mutant controls are each rejected by a failed assertion with actual solver playback rather than a passed proof: a wrong operator (`add` for `sub`), swapped operand order in `sub`, `div` or a comparison, a result unit differing from the owner's identified unit, an altered `pow` exponent, a dropped charge or changed charge order, a stopped outcome reported completed, a clipped magnitude endpoint and an outcome-based assume. |
| FR-050-deps | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:105 | - [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) owns quantity oracle semantics (AC-6, AC-48..AC-52, AC-59..AC-65); its native or oracle agreement is not this proof, and AC-51 is superseded for non-conversion quantity operations only when this row is delivered. |
| FR-015-quantity-row | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:767 | / a quantity scalar operation selected under planned [FR-050](./FR-050-quantity-scalar-harnesses.md) whose K-EXACT-quantity carrier (IR-1443) is absent / `unsupported` naming the carrier dependency; a quantity leaf of a composite stays `ShadowFamilyNotBuilt` above / |
| FR-025-AC-7 | examined | spec/kani/functional/FR-025-generated-subject-abi.md:177 | An argument with no applicable row (decimal, IEEE, text, enum, composite, collection, function, quantity outside the planned FR-050 row, or Rational outside the planned FR-046 row) is accounted `unsupported` with a typed reason naming its family/capability, and no harness is emitted. The planned Rational row does not authorize a harness without its lossless decoding and public replay prerequisites. |
| FR-028-quantity-bullet | examined | spec/kani/functional/FR-028-bounded-proof-ceilings.md:357 | - A shadow or a production harness for each leaf and member family `ShadowFamilyNotBuilt` names: text, enum, rational, decimal, quantity, float, sequence, set, bag and ordered set; the scalar families are their own rows and PRs (quantity scalar operations: planned FR-050). |
| tests.md-FR-050-row | examined | spec/kani/matrix/tests.md:13 | / FR-050 / FR-050-AC-1 through FR-050-AC-9 / TC-064 / 🚧 PLANNED (IR-1439): no quantity scalar Kani harness; needs K-EXACT-quantity (IR-1443). / |
| spec.md-FR-050-row | examined | spec/spec.md:92 | / Quantity scalar operation harnesses / [FR-050](kani/functional/FR-050-quantity-scalar-harnesses.md) planned bounded quantity operation proof / [TC-064](kani/matrix/TC-064-quantity-scalar-harnesses.md) planned generated-proof and mutant controls / |
| FR-015-shadow-row | context_only | spec/kani/functional/FR-015-bounded-kani-obligations.md:764 | / a leaf or member of family text, enum, rational, decimal, quantity, float, sequence, set, bag or ordered set / `unsupported`, `ShadowFamilyNotBuilt` naming the family and the node; the equality oracle of FR-018 still generates for it / |
| TC-030-quantity-row | context_only | spec/routed/matrix/TC-030-capability-settlement.md:401 | / Quantity scalar `expression` claim, IR-confirmed generated claim, including identity `quire.op.quantity.convert`, read through source-authenticated V1 handoff / Planned (IR-269) / Quantity scalar operation forms; the conversion's settlement is FR-014-AC-51. / `OperationNotRendered` (`classify_claim` maps `ScalarLoweringRefusal::NoRenderer`). / |
| FR-014-AC-51 | context_only | spec/oracle/functional/FR-014-exact-scalar-oracles.md:691 | After admitted V2 or source-authenticated V1 operation views are available, a selected `quire.op.quantity.convert` claim settles through capability settlement like any other quantity operation, as `OperationNotRendered` (Planned, IR-269), because the Kani scalar renderer builds no quantity harness; the conversion is credited by its oracle only and is uncredited for Kani. The settlement is not a refusal by name, and a quantity harness is IR-269's work, not this requirement's. The owning settlement row is TC-030's Quantity scalar row, which names `quire.op.quantity.convert` and cites this criter |
| FR-046-AC-9 | context_only | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md:159 | Counterexample decoding validates the two physical I64 entries per original parameter, exact width/comment/order, schema roles, required physical-entry and byte limits, canonical components, original domains and selected declaration/application binding before public owner replay. Grouping yields one typed owner Rational value per parameter through the existing canonical serialization seam; missing, duplicate, foreign, role-swapped, domain-conflicting and malformed component schemas or entries retain typed refusals with no replayed failure. Out-of-domain, duplicate-zero, noncanonical, wrong-dec |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | TC-064 step 5 asserts `OperationNotRendered` 'naming the missing prerequisite' unconditionally, while FR-050-AC-6 says only 'where a reason is carried'. TC-030's settlement maps `ScalarLoweringRefusal::NoRenderer` and carries no prerequisite name, so the test step either cannot pass or asserts an unspecified field. Align step 5 with AC-6 or specify where the prerequisite is recorded. | spec/kani/matrix/TC-064-quantity-scalar-harnesses.md:39 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | fixed in f255c11d |
| FND-002 | fixed | fixed in f255c11d |
| FND-003 | fixed | fixed in f255c11d |
| FND-004 | deferred | The FR-015 pattern divergence is gone (#765 no longer touches FR-015). Textual conflicts with open #764 (head 1479c253) remain in FR-025, FR-028, kani/matrix/tests.md and spec.md per `git merge-tree`; they are inherent to two open siblings and are resolved by rebasing whichever PR lands second, not by a change to this PR. |
| FND-005 | fixed | fixed in e738e884/66c9558c |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | FR-050's Description (added in f255c11d) ends 'absent either, the item is refused (AC-6)', but AC-6 says the item remains settled as `OperationNotRendered`, and FR-014-AC-51 states that settlement 'is not a refusal by name'. Say 'settled as AC-6 states' rather than 'refused'. | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:39 |
