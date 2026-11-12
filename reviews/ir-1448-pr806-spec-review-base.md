---
id: SR-17940486709564847047735
title: "IR-1448 PR #806 spec-review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #806 at c2a8f9e0; IR-1448; spec/kani/functional/FR-055-option-clause-harnesses.md, spec/kani/matrix/TC-069-option-clause-harnesses.md, spec/kani/matrix/tests.md, spec/spec.md, docs/qualification/ir1448-kani-option-probe.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-055
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-069
    type: reviews
---

## Summary

Ticket: IR-1448. New FR-055 (option presence and value lowering in V2 clause Kani harnesses,
AC-1 to AC-10), TC-069, a registry row in spec/spec.md, a PLANNED matrix row in
spec/kani/matrix/tests.md, and a hand-written Kani probe record. Checked against CG main
(src/kani/generate/clause/v2.rs, src/kani/generate/outcome.rs), FR-014, FR-015, FR-017, the
quire-exact kernel (origin/main 0d29394), the IR definedness checker
(quire-contract-model expression.rs check_unwrap) and the concurrent IR-1447 spec worktree.
`quire validate` over spec/plan/reviews: the 6 pre-existing warnings only, none in FR-055 or
TC-069. `quire matrix`: FR-055-AC-1..10 resolve, all untagged (expected for PLANNED).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | FR-055 contradicts FR-015-AC-42 and does not amend it. Today a V2 clause holding `quire.op.option.present`/`.value` is refused by `unsupported_checked_clause_operator` as `V2ClauseOperatorNotSupported { node_id, operation_identity }` (FR-015-AC-42), since `supported_clause_operator` admits only connectives, comparisons and add/sub/mul/neg. FR-055 lowers those two operators but leaves AC-42 saying every operator outside that list is `unsupported` (IR-1447 amends AC-42 for its control ops; this PR does not). `OptionOperationNotLowered` also overlaps `V2ClauseOperatorNotSupported` for any other option operator (e.g. the catalogued `quire.op.option.some`) with no precedence, and the "SHALL NOT map to" list omits `V2ClauseOperatorNotSupported`, the one refusal it actually collides with. | spec/kani/functional/FR-055-option-clause-harnesses.md:23-31, 109-114; spec/kani/functional/FR-015-bounded-kani-obligations.md (AC-42); src/kani/generate/clause/v2.rs:126-138, 510-575 |
| FND-002 | medium | The Description says option reads are "refused today as ... `NoFiniteEncoding`, FR-015-AC-62". FR-015-AC-62 is the `StateFrame` unsupported-family record, not the V2 clause route; the V2 clause refusal today is `V2ClauseOperatorNotSupported` (FND-001). The FR-055-AC-7 mutant "mapping a reason to `NoFiniteEncoding`" therefore guards against the wrong regression; the realistic one is falling back to `V2ClauseOperatorNotSupported`. | FR-055:23-25, FR-055-AC-7 |
| FND-003 | high | FR-055-AC-4 is not testable on the inputs the AC section mandates ("All controls use admitted source-backed V2 inputs"). The unguarded claim `result = (option.value(x) <= 1000)` fails definedness: IR `check_unwrap` returns `undefined(..., DefinednessObligationKind::OptionPresence, "option presence is not established")` unless a `Present` fact guards it, and QSL delegates definedness to IR (QSL ADR-010 OBS-010). quire-exact documents `Undefined::NoneValue` as "Only direct kernel evaluation of an unlinked expression can meet it." So no admitted package carries the AC-4 clause; the control needs a different input (a direct oracle/harness fixture outside admission, stated as such) or AC-4 must be reframed (defensive stop on a non-admitted path, or refusal at admission). The AC-5 "evaluate value before the guard" mutant is unaffected. | FR-055-AC-4, FR-055:85-88, TC-069 step 4; quire-contract-model src/expression.rs check_unwrap; quire-exact src/outcome.rs:106-108 |
| FND-004 | medium | The semantic authority is mis-cited. FR-055 says the absent-read outcome is "the outcome the quire-exact option kernel reports" and "FR-014 owns option oracle semantics; this requirement does not restate them". quire-exact main has no option.present/value operation (only `OptionValue::payload()` and the `Undefined::NoneValue` reason); the evaluator that yields NoneValue is QSL's (QSpec TC-917 step 5). FR-014 has no option operation text at all. So the oracle side (how CG's clause oracle evaluates the two operators, and that `Undefined::NoneValue` classifies to the thin `ClauseStop::Undefined`) has no owning requirement. The value itself (absent -> Undefined) is consistent with QSL. | FR-055:85-88, 147 |
| FND-005 | medium | `field?` scope contradicts the `NullablePresence` refusal. The Description lowers "an option, `Option<T>` or an optional field `field?`", but every `field?` slot admits three states (quire-exact `FieldValue::{Present, Absent, Null}`, `Presence::Optional`), so every optional field "needs the third null state" and is refused; the trigger "needs the null state" is undefined, and TC-069 step 7's fixture ("a field needing `null`") cannot be built unambiguously. FR-015-AC-70 (IR-264) draws `present`/`absent`/`null` for the same field shape; FR-055 should either say `field?` is out of scope (refused as `NullablePresence` whenever read) or draw the null state as AC-70 does, and the Inputs (checked type `Option<T>` only) should match. | FR-055:23, 48-50, 106-107, FR-055-AC-7; TC-069 step 7 |
| FND-006 | low | Numbering leaves a permanent gap: IR-1447 is authored as FR-015-AC-152..163 and TC-025 step 57 (worktree IR-1447-spec), not FR-054/TC-068, so FR-055/TC-069 skip numbers nothing will claim. No collision. Both PRs insert a row at the top of the spec/kani/matrix/tests.md table (trivial rebase conflict). | spec/kani/matrix/tests.md:13 |

## Verdict

Status hygiene is right: FR-055, TC-069, the tests.md row and the spec.md row are PLANNED with
owner IR-1448, no AC claims implementation, and the probe is labelled non-CG evidence. The new
refusal `OptionNotLowered { node_id, reason }` as one more `UnsupportedObligation` arm fits the
existing family shape (beside `NoFiniteEncoding`, `PathGuardNotLowered`, `V2ClauseOperatorNotSupported`),
`requires_bound` for an unbounded-integer payload is kept (FR-015-AC-61 / AC-8), and
composite equality (FR-015-AC-69..71) is left untouched. Not mergeable: FND-001 (unamended
FR-015-AC-42 conflict) and FND-003 (AC-4 untestable on admitted inputs) are high.

## New findings (disposition pass 1)

Reviewed head a9adfb39 (FR-055/TC-069 renumbered to FR-054/TC-068).

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-007 | medium | FR-054-AC-23 can't pass at this head, and the FR-015-AC-42 row will conflict with IR-1447. AC-23 requires that FR-015-AC-42 says the amendment "stands beside the control-operator exception of FR-015-AC-152 to FR-015-AC-163". Those ACs do not exist on main or on this branch: IR-1447 is still uncommitted in worktree IR-1447-spec, with no pushed branch and no PR. The appended "Amended by IR-1448" sentence does not mention them. IR-1447 also edits the same AC-42 table row, inserting a control-op exception inside the parenthetical, while this PR appends at the row end. Whichever lands second must hand-merge the row. Fix: either drop the AC-152..163 clause from AC-23 (IR-1447 then adds the cross-reference when it lands), or state the merge order. | spec/kani/functional/FR-054-option-clause-harnesses.md:193; spec/kani/functional/FR-015-bounded-kani-obligations.md:863 |
| FND-008 | low | The FR-054 Status says "the implementation ticket is owned by the IR code lead and is created by the lead". Per the lead, IR-1448 itself owns the code PR and no new ticket is created. Status should read PLANNED (IR-1448), with the implementation under IR-1448. | spec/kani/functional/FR-054-option-clause-harnesses.md:206-207 |
| FND-009 | low | The probe record still says "the quire-exact option kernel and its meter were not used ... (the kernel outcome is the authority, FR-054)". FR-054 now states that quire-exact has no option operation and that QSL/IR own definedness, with CG mapping `NoneValue` to `ClauseStop::Undefined`. The probe text contradicts the corrected authority. | docs/qualification/ir1448-kani-option-probe.md:57-59 |

## Dispositions

Round 1, reviewed head a9adfb39b95bb563fd3e4f64c8f277c9fe0cc0b3.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | FR-015-AC-42 gains "Amended by IR-1448: `quire.op.option.present` and `quire.op.option.value` over an `Option<T>` operand are not refused here; FR-054 lowers them or refuses them with `OptionNotLowered`, and every other `quire.op.option.*` operator is refused with `OptionNotLowered` (reason `OptionOperationNotLowered`) and not as `V2ClauseOperatorNotSupported`." FR-054 adds the `option.some` rule, adds `V2ClauseOperatorNotSupported` to the SHALL NOT map list and adds a precedence rule. The coordination gap with IR-1447 is new finding FND-007. |
| FND-002 | fixed | The Description now reads "refused today as `UnsupportedObligation::V2ClauseOperatorNotSupported` (FR-015-AC-42)". The AC-14 mutant collapses the reasons into `V2ClauseOperatorNotSupported`. |
| FND-003 | fixed | AC-5 now runs "On a direct oracle fixture outside admission", AC-22 compiles the generated oracle in a scratch crate, and the AC preamble excepts AC-5. Checked against the code: the Semantic authority text ("admitted only where a `Present` fact guards it ... IR `OptionPresence` definedness"; NoneValue "only a direct evaluation of an unlinked expression can meet") matches IR `check_unwrap` and quire-exact `outcome.rs`. The only other path `check_unwrap` admits is the literal `some(...)` operand, which is never absent. |
| FND-004 | fixed | The new "Semantic authority" section: QSL/IR own definedness, quire-exact owns NoneValue, FR-014 states no option operation, and FR-054 owns the presence/payload read and the NoneValue to `ClauseStop::Undefined` mapping. Dependencies no longer claim FR-014 ownership. The stale probe text is new finding FND-009. |
| FND-005 | fixed | "An optional record field `field?` is not lowered ... reading one is refused as `NullablePresence`". The Inputs are restricted to `Option<T>`. AC-12 adds a named fixture and a mutant. |
| FND-006 | fixed | Renumbered to FR-054/TC-068, which IR-1447 does not use (IR-1447 is authored as FR-015-AC-152..163). tests.md and spec.md rows are updated. A trivial table-row rebase remains possible. |

Round 2, reviewed head 581cb583d2fa823708471aad2e914cce32a672b5.

| FND | outcome | reason |
|---|---|---|
| FND-007 | fixed | FR-054-AC-23 now reads "FR-015-AC-42 states that `quire.op.option.present` and `quire.op.option.value` are lowered by FR-054 and that every other `quire.op.option.*` operator is refused with `OptionNotLowered` and not as `V2ClauseOperatorNotSupported`." and no longer cites FR-015-AC-152..163. The AC-42 amendment is one sentence at the row end: "Amended by IR-1448: `quire.op.option.present` and `quire.op.option.value` are lowered or refused by FR-054, and every other `quire.op.option.*` operator is refused as `OptionNotLowered`, not here." IR-1447 still edits the same row (inside the parenthetical). That is now only a textual rebase for whichever lands second. |
| FND-008 | fixed | "PLANNED (IR-1448 specifies; IR-1448 also owns the implementing code PR; no new ticket)." |
| FND-009 | fixed | "no quire-exact kernel or meter was used, and `option.value` of an absent option is modelled as `Undefined` by assumption (FR-054 owns that CG rule; QSL and the Contract IR own option definedness and quire-exact owns `Undefined::NoneValue`)." |
