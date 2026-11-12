---
id: SR-17940486709564847047739
title: "IR-1448 PR #806 spec-review failure-domain"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen PR #806 at c2a8f9e0; IR-1448; spec/kani/functional/FR-055-option-clause-harnesses.md, spec/kani/matrix/TC-069-option-clause-harnesses.md, spec/kani/matrix/tests.md, spec/spec.md, docs/qualification/ir1448-kani-option-probe.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-055
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-069
    type: reviews
---

## Summary

Ticket: IR-1448. Looked for unstated option shapes, refusal precedence and identity confusion
in the FR-055 refusal set against quire-exact `ValueType` and CG's existing
`UnsupportedObligation` arms.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | The `PayloadNotScalar` family list is open: text, decimal, rational, float, quantity, enum, collection. quire-exact `ValueType` also has Timestamp, Uuid, Union, Population and Option; a record payload with an option leaf (`Option<R>`, `R` holding `Option<Int>`) is neither "itself the payload of an option" (`NestedOption`) nor listed. These shapes have no stated disposition. Make the reason closed ("any leaf other than Boolean or bounded integer") or list every family, and say which reason an inner option leaf takes. | spec/kani/functional/FR-055-option-clause-harnesses.md:97-104, FR-055-AC-7 |
| FND-002 | low | Precedence is unstated when an option clause also meets another refusal: an unsupported operator elsewhere in the body (`V2ClauseOperatorNotSupported`), a payload bound beyond `i64` (`DomainNotRepresentableInI64`, the draw is "one native `i64`"), or two option refusals in one clause (which node/reason is reported). | FR-055:68-75, 97-114 |

## Verdict

The absent case is modelled explicitly (canonical payload, no default, stop on unguarded read)
and identity/playback cover the canonical-absent rule. The open family list is the main
failure-domain gap.

## New findings (disposition pass 1)

Reviewed head a9adfb39.

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-003 | medium | The new precedence rule ("CG SHALL report the one at the lowest node id in the clause body, in the order FR-015-AC-42 already uses for unsupported operators") claims an existing order that does not exist. FR-015-AC-42 states no order. The code reports the first unsupported operator in a depth-first, left-to-right pre-order walk from the clause condition, following references: `unsupported_checked_clause_operator` pops a stack after `terms.extend(arguments.iter().rev())`, and `unsupported_clause_operator` uses a recursive `find_map`. That walk is not node-id order. FR-054-AC-17's "lowest node id" control would fail against the existing operator refusal unless the code changes. Fix: state traversal order (pre-order of the clause body) as the rule, or state that FR-054 changes AC-42's order, and align AC-17. | spec/kani/functional/FR-054-option-clause-harnesses.md:144-148, FR-054-AC-17; src/kani/generate/clause/v2.rs:510-575 |

## Dispositions

Round 1, reviewed head a9adfb39b95bb563fd3e4f64c8f277c9fe0cc0b3.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | "When an option operand's payload has any leaf whose family is not Boolean or a bounded integer, CG SHALL refuse the clause as `OptionNotLowered` with reason `PayloadNotScalar` ... The family list is closed by that rule". Timestamp, uuid, union and population are now listed, and a record-held option leaf is `NestedOption` (AC-11). |
| FND-002 | fixed | Precedence is now stated: lowest node, then shape before bound at the same node. An `i64`-overflowing bound "keeps its existing bound refusal". AC-17 covers it. The stated order does not match the code; see new finding FND-003. |

Round 2, reviewed head 581cb583d2fa823708471aad2e914cce32a672b5.

| FND | outcome | reason |
|---|---|---|
| FND-003 | fixed | "When a clause meets several refusals, CG SHALL report the first one met in a depth-first, left-to-right walk of the clause condition from its root that follows each reference once, which is the walk the V2 clause classifier already makes for the first unsupported operator and which this requirement does not change." This matches `unsupported_checked_clause_operator` (stack with `arguments.iter().rev()`, `visited` set). AC-17 is aligned: "met earlier in the walk"; "Mutant: reporting the last refusal met fails the first two." |
