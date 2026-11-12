---
id: SR-17940486709564847047744
title: "IR-1447 PR #808 spec-review integrity"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen PR #808 at 56f1887d; IR-1447; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md, docs/qualification/ir1447-kani-control-probe.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-1447. Integrity and consistency of FR-015-AC-152..163, the three new FR-015 prose
bullets, the AC-42 edit and TC-025 step 57, at head 56f1887d. Step 57 reaches every new AC;
all rows are PLANNED with owner IR-1447; no AC id collides with an open PR; citations resolve
in `quire matrix`.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | The refusal family is not exhaustive over position and nesting. The third FR-015 bullet says any `let`/`if`/`case` "shape the control lowering does not encode" is refused with one closed control reason, but the four reasons cover only value families. Lowering is stated per position: `let` only in the clause-condition context (AC-152), `if` only as an operand of a comparison or arithmetic (AC-153), `case` with no context at all (AC-154), `if` inside a collection element (AC-155). A Boolean `if` at the clause root or under `and`/`implies`, a `let` in scalar position, and nested controls (`if` in a `let` body, a `case` arm holding an `if`) get no lowering and no reason. On nesting: the spec need not fix a depth cap, and leaving the measured depth to the delivering ticket is acceptable, but it must say what happens. Either "lowers compositionally at any position whose operands are native scalar, bounded only by the existing `MAX_CONDITION_TERMS` and source-size ceilings and their existing refusals" with one nested fixture, or a fifth closed reason (e.g. `ControlPositionNotSupported`) for positions and nesting outside AC-152..155. | spec/kani/functional/FR-015-bounded-kani-obligations.md:446-457, 978-981; src/kani/generate/clause/v2.rs:510-556 |
| FND-002 | medium | No precedence is defined. Not among the four reasons (nested controls with two failing shapes), not against `V2ClauseOperatorNotSupported` (a clause holding both an unsupported operator and a failing control), and not against FR-054 `OptionNotLowered` (a `let` binding an option value, an `if` with option branches). The classifier is a first-met DFS walk, and FR-054-AC-17 states that rule for options (first refusal met in the walk wins, mutant: reporting the last). Add the same for controls, plus an FR-054-AC-14-style mutant: collapsing a control reason into `V2ClauseOperatorNotSupported` fails AC-160..163. | spec/kani/functional/FR-015-bounded-kani-obligations.md:988-991; FR-054-AC-14, FR-054-AC-17 (main) |
| FND-003 | medium | The "or an operand that is not a union" branch of FR-015-AC-162 cannot be tested. The catalog's `quire.op.control.case` operand family is `union`, and Contract IR strict admission refuses any other operand, so no admitted clause reaches CG with one. In particular a QSL `enum` is the separate catalog family `enum`/`ordered_enum` and is never a `case` scrutinee. Drop the branch, or state it as a reader refusal (no CG item). Also say that "enum-shaped union" means a `union` with only arity-0 members and is not the FR-049 `enum` family. | spec/kani/functional/FR-015-bounded-kani-obligations.md:990; quire-verification-contracts contracts/checked-operation-catalog-v1.json (quire.op.control.case) |
| FND-004 | low | Wrong citation: "FR-038's `union_arms` constraint" (AC-154 and the probe record) resolves in this repo to CG's own FR-038 (spec/core/functional/FR-038-typed-operator-catalog.md), which has no `union_arms`. The rule is QSpec FR-440 / FR-322 "union_arms" (or quire-contract-ir's FR-038). Qualify the citation. | spec/kani/functional/FR-015-bounded-kani-obligations.md:982; docs/qualification/ir1447-kani-control-probe.md:64 |
| FND-005 | low | The non-scalar lists in FR-015-AC-160 and FR-015-AC-161 ("Text, Rational, Decimal, Quantity, Float, aggregate or collection") leave out families that can reach these positions: `enum`, `ordered_enum`, `string`, `uuid`, `timestamp`, `option`, `union`, `record`, `tuple`, `reference`. "aggregate" is not a catalog family. Make the rule "any family other than Integer or Boolean (for example ...)". | spec/kani/functional/FR-015-bounded-kani-obligations.md:988-989 |

## Verdict

Structure and links are sound. Exhaustiveness, precedence and the reachability of AC-162 need fixing before merge.

## New findings (disposition pass 1)

Reviewed head 92686c1e8bc308f2a2b825d4b2d4a096f604aa10.

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-006 | medium | A `case` scrutinee still has gaps. FR-015-AC-154 lowers a `case` only over a union operation parameter. FR-015-AC-164's compositional rule covers a construct "whose operands are native Integer or Boolean values", which a `case` never satisfies, because its operand 0 is a `union`. A `case` over a union state field, a union-valued literal or any other union term therefore has neither a lowering nor one of the four closed reasons. Either extend AC-154's draw to every union input that reaches the scrutinee, or add the shape to a closed reason (e.g. `CaseScrutineeNotParameter`). | spec/kani/functional/FR-015-bounded-kani-obligations.md (AC-154, AC-164) |
| FND-007 | medium | FR-015-AC-166 says the `let`-binding-an-option fixture is refused "with the reason of the first refusal met in the walk", but does not state which reason that is. Two readings are possible. If the control check runs when the walk visits the `let` node, before its operands, the result is `LetValueNotNativeScalar`. If the walk first meets an option operator in the bound value, it is FR-054's `OptionNotLowered`. A test cannot assert an unstated outcome. State the expected reason for the fixture, and whether a control's shape check happens on entering the node. | spec/kani/functional/FR-015-bounded-kani-obligations.md (AC-166) |

## Dispositions

Round 1, reviewed head 92686c1e8bc308f2a2b825d4b2d4a096f604aa10.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | AC-164: compositional lowering "at any position of the clause (a clause root, an operand of a connective or comparison, a `let` value or body, an `if` condition or branch, a `case` arm)", nested controls included. It is bounded by the existing `MAX_CONDITION_TERMS` and source-size refusals, and the delivering ticket owns any depth cap with its own typed reason. AC-165 is the nested `let`-over-`if` fixture with a failing mutant at 500. The case-scrutinee residue is new finding FND-006. |
| FND-002 | fixed | AC-166: the first refusal met in the depth-first, left-to-right walk wins, across the four control reasons, `V2ClauseOperatorNotSupported` and `OptionNotLowered`, with the last-met mutant. AC-167: none is reported as `NoFiniteEncoding`, `StateFrameRefused` or `V2ClauseOperatorNotSupported`, with the collapse mutant. The option-fixture ambiguity is new finding FND-007. |
| FND-003 | fixed | AC-162 is now `CaseUnionPayloadNotSupported` for a union with a payload member, and states that a non-union operand (a QSL `enum`, an integer or any other family) is refused at Contract IR admission and never reaches CG. |
| FND-004 | fixed | AC-154 cites "QSpec FR-440 `union_arms`, which Contract IR FR-038 enforces at admission". The probe record cites QSpec FR-440/FR-322 and quire-contract-ir FR-038. |
| FND-005 | fixed | AC-160 now reads "any value family other than native Integer or Boolean (for example enum, ordered_enum, string, uuid, timestamp, Rational, Decimal, Quantity, Float, option, union, record, tuple, reference or collection)". AC-161 and AC-163 refer to that list. |

## Dispositions (round 2)

Round 2, reviewed head ebd98b5841e41b0e6c138862ea3d63ba2455fdcb.

| FND | outcome | reason |
|---|---|---|
| FND-006 | fixed | FR-015-AC-171: a `case` whose scrutinee is not an operation parameter of an arity-0 `union` (a union state field, a union literal or any other union term) is refused `CaseScrutineeNotParameter`. AC-167, the refusal bullet and TC-025 step 57 now count five reasons. |
| FND-007 | fixed | AC-166: "a control's shape check runs on entering its node, before its operands are walked". A `let` bound to an option-typed parameter is `LetValueNotNativeScalar`, and a `let` bound to `quire.op.option.value` of an Integer result passes the control check and is decided by FR-054. This matches the pre-order walk of `unsupported_checked_clause_operator` (v2.rs:510-556). |
