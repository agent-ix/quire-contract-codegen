---
id: SR-17940486709564847047988
title: "IR-854 PR #835 spec-review base"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #835 at f585f1a4, git diff origin/main...HEAD; IR-854; spec/kani/functional/FR-057-integer-division-scalar-harnesses.md, spec/kani/matrix/TC-071-integer-division-scalar-harnesses.md, spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/tests.md, spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, spec/core/functional/FR-038-typed-operator-catalog.md, spec/kani/matrix/tests.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-057
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-071
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
---

## Summary

Ticket: IR-854. This review covers the final head f585f1a4. An earlier pass on 5a01ae6d was not
posted. The planner rulings are taken as binding scope: `rem` is inside IR-854 through
FR-014-AC-81, IR-1541 owns the native plan, the `divisor_constant` extension is optional, and
IR-854 is an IR-269 MVP row.

Examined: FR-057 Description, Behavior and AC-1 to AC-12; TC-071 steps 1 to 11; FR-014-AC-81 and
the FR-014 interval and refused-list edits; the TC-024 remainder section; both matrix rows; the
FR-015 note and the AC-57 and AC-109 amendments; the TC-023 note; the FR-038-AC-4 amendment; the
spec.md row.

Checked against the real code and specs, and found correct:

- quire-exact origin/main (b81e595) has no `QuotientRemainder` and no `DivisionPairOutOfDomain`.
  Only `reviews/ir-601-code-review.md` mentions them, as removed.
- `NativeIntegerOperation` and `NativeIntegerOperationKind` have only Add, Subtract, Multiply and
  Negate. `evaluate_native_integer_division` and `evaluate_native_integer_modulo` are plan-free
  and have the stated signatures. `native_div_rem` is `checked_div` plus `wrapping_rem` plus sign
  corrections. The native refusal is a sound reading of AD-006's checked-plan rule.
- QSL origin/main `lowering.rs:4474-4489` lowers `div` and `rem` with one `IntegerDivision` law
  and `mod` with none. CG `derive.rs:214-215` derives only `div` and `mod`, and `rem` falls to
  `OperationNotDerivable`. `render.rs:348-370` and `lower.rs:1096-1195` match the described
  oracle and inferred intervals. `LawNotDerivable`, `BoundNotI64`, `ResultUnreachable` and
  `NoRenderer` exist.
- The `div` window table and the division-free admission are correct for both divisor signs. The
  `rem` shift (`e`, or `e - |b|`) lands on the unique member of each window. The FR-014-AC-81
  inferred `rem` intervals are sound. The AC-4 and AC-81 vectors match the laws.
- The expected side is independent of the oracle, so AC-3 and AC-4 are testable, not
  tautological. The row does not depend on IR-602. AC-12 and TC-071 step 11 exclude tool
  versions.
- `make spec` exits 0 on a clean export of f585f1a4. The one FR-014 grammar warning (line 336,
  trigger "before") is outside this diff.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | The stated control cannot kill the AC-2 mutants. The control "evaluates the rendered assertion over the real oracle at the input that distinguishes each mutant". Every listed mutant only widens the assertion: it accepts any `Refused`, the other `DivisionOutOfDomain` member, `Incomplete`, or `Completed` at `b == 0`. A widened assertion still passes every outcome the real oracle produces. The scalar meter is unlimited (`scalar.rs:688`: "`Incomplete` cannot occur"), so no input separates the `Incomplete` mutant. TC-071 step 2 repeats this. Feed each mutated assertion the wrong outcome directly (another refusal cause or member, an `Incomplete`, a `Completed` at zero), or restate the mutants as narrowings that the real oracle's outcome falsifies. | spec/kani/functional/FR-057-integer-division-scalar-harnesses.md:294, spec/kani/matrix/TC-071-integer-division-scalar-harnesses.md:41-51 |
| FND-002 | medium | The `rem` and `mod` expected sides require `i128::rem_euclid(a, b)`. With a symbolic divisor that is a 128-bit divider in CBMC. The Kani guidance says to check a remainder as `a - q * b`, and that wide-width division need not be tractable; the 2026-11-11 measurements found 128-bit arithmetic equivalence infeasible. The `div` side shows that a division-free form is possible. Permit a narrower form, such as `i64` `rem_euclid` with `b == -1` taken as `0`, or a relational check of the remainder. Otherwise expect the symbolic `rem` and `mod` shapes to measure not tractable on every route. | spec/kani/functional/FR-057-integer-division-scalar-harnesses.md:195-205 |
| FND-003 | medium | AC-7 holds vacuously. It requires `Verified` only "on each route and shape measured tractable". If every measured shape times out, AC-7 passes with no `Verified` run. Planner ruling (1) makes IR-854's VERIFIED criterion blocked by IR-1541, which means a VERIFIED result is expected. Require at least one measured `Verified` shape per operation once IR-1541 lands, for example the constant divisor at the smallest width on the native route. | spec/kani/functional/FR-057-integer-division-scalar-harnesses.md:299 |
| FND-004 | medium | The AC-8 replay check does not cover the expected-side mutants. "The kernel value or outcome differs from the mutant's" fits the oracle mutants: profile swaps, `rem` as `modulo` or as the quotient member, `mod` as a remainder member, and completing at zero. For the wrong-profile window table and the removed zero arm, the oracle is healthy. The kernel outcome equals the oracle's, and the relational expected side has no outcome of its own to compare. TC-071 step 8 repeats this. For those two mutants, state the check, for example: the kernel outcome satisfies the healthy mapping and violates the mutated one. | spec/kani/functional/FR-057-integer-division-scalar-harnesses.md:300, spec/kani/matrix/TC-071-integer-division-scalar-harnesses.md:90-108 |
| FND-005 | medium | Tool versions are required in the measurement evidence. Behavior requires each measurement record to carry "Kani, CBMC and solver versions", and AC-9 requires "each field filled". That contradicts QSL-406 and EA-49, which delete recorded tool versions, and FR-056, where tool versions in a row are optional. It also contradicts FR-057's own AC-12 and TC-071 step 11. Drop tool versions from the required record, or mark them optional as FR-056 does. | spec/kani/functional/FR-057-integer-division-scalar-harnesses.md:255-259, spec/kani/functional/FR-057-integer-division-scalar-harnesses.md:301 |

## Verdict

Not mergeable. FND-001 (mutants the stated control cannot kill) must be fixed in this PR.
FND-002 to FND-005 are text fixes in the same PR. The design is sound: the relational `div`
expected side, the windowed `rem` and `mod` expectation, the zero-divisor and `MIN / -1`
dispositions, the AD-006 native refusal, and FR-014-AC-81. The upstream facts the spec relies on
check out.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-006 | medium | It is not clear which table holds native-route rows. FR-057 sends every `TimedOut` or `memory_exhausted` shape, on each route, to "the shipped scalar table that FR-056-AC-15 consumes". FR-056-AC-15, which this PR does not amend, names that table `TractabilityTable::shipped_general_route_scalar`, equal to `ir1530-general-route-scalar-table.json`, and says of native rows "a `native` row would [refuse]; none ships". Once FR-057 lands, native division rows go into a table named and specified as general-route only, and FR-056-AC-15's "none ships" is false. QE FR-409 Behavior 20 also writes measured native rows, keyed by divisor shape, into "the tractability table that IR-1526's seam loads". That gives the same table a second writer and a key field FR-056 lacks unless the conditional `divisor_constant` extension is added. Name the table that holds native rows (rename the shipped table, or add a native one), amend FR-056-AC-15 to match, and say whether the QE FR-409 rows or the CG FR-057 rows are the ones that ship. | spec/kani/functional/FR-057-integer-division-scalar-harnesses.md:275, spec/kani/functional/FR-056-kani-tractability-refusal.md:140 |

## Dispositions

Round 1, checked at b720c916 against the diff f585f1a4..b720c916.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | AC-2 and TC-071 step 2 now accept the real oracle's three outcomes with the healthy assertion, then feed constructed outcomes (another refusal cause, the other member, `Incomplete`, `Completed` at zero) to both assertions. The healthy assertion rejects each one; each mutant accepts the one it wrongly admits. This kills every widening mutant. |
| FND-002 | fixed | "No expected side SHALL divide at 128 bits." `e` is now `i64::rem_euclid(a, b)` widened to `i128`, with a `b == -1` arm that gives `0`. That arm is correct: `i64::MIN rem_euclid -1` is the only overflowing case, and every `a rem_euclid -1` is `0`. `b == i64::MIN` is safe, because std wraps the `\|b\|` correction. The window shift `e - \|b\|` is over `i128`, so `\|i64::MIN\| = 2^63` is exact. The 64-bit divider's tractability is measured, which the text now says. |
| FND-003 | fixed | AC-7 now requires that, once IR-1541 lands, at least one native-route shape per operation and profile measures `Verified`, at minimum the 8-bit constant divisor, and that timeouts alone do not satisfy it. TC-071 step 7 asserts this. |
| FND-004 | fixed | AC-8 and TC-071 step 8 split the replay check. For oracle mutants, the kernel outcome differs from the mutant oracle's. For the two expected-side mutants, the kernel outcome satisfies the healthy mapping and violates the mutated one. |
| FND-005 | fixed | The measurement record now lists only generator and kernel versions, host and date as required. Kani, CBMC and solver versions are "optional, as in FR-056 rows. No record or criterion requires them (QSL-406, EA-49)". AC-9 says "each required field" and "Tool versions are optional and not checked". |

Round 2, checked at 965c1373 against the diff b720c916..965c1373.

| FND | outcome | reason |
|---|---|---|
| FND-006 | fixed | FR-056 Description and AC-15 now make the shipped scalar table CG's single record for both routes. Each row is distinguished by its required `route` field, and "none ships" is replaced by the per-route rule. FR-057 Behavior and AC-9 say quire-exact records FR-409 rows as TC-957 evidence and ships no table, and that the IR-854 coder's CG PR copies the native rows in. The author kept `route` required instead of optional, which agrees with FR-056's malformed-row rule (FR-056-AC-3) and is the better choice. Keeping the historical `general_route` names is stated explicitly. |
