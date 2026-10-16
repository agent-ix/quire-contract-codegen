---
id: SR-11780
title: IR-814 base spec review of the FR-033 AC-14 to AC-17 composite equality remainder
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen@216b447a1f9475534df11056df849eb667203beb (PR 399, diff vs origin/main 007417d); spec/replay/functional/FR-033-composite-parity-replay-binding.md,
  spec/replay/matrix/TC-048-composite-parity-replay-binding.md, spec/replay/matrix/tests.md, spec/assurance/AD-003-evidence-chain.md
review_set: subset
---

## Summary

Ticket: IR-814. Base checklist over the four-file spec diff. It adds FR-033-AC-14 to AC-17
(PLANNED, IR-814), rewrites the AC-1 and AC-11 status headers and AD-003 E-1, adds TC-048 rows,
and reattributes proving-record/native integration to IR-806. Each delivered or undelivered claim
was checked against CG origin/main 007417d, the Contract IR model at the locked revision 9be7387,
IR FR-038, and QSL main FR-358. Review only; no edits.

## Verdict

**NOT MERGEABLE**: one high finding and one medium finding in the inline-operand criteria, and one
low finding.

What holds:

- The delivered/undelivered split is true. `tests/it/composite_original_eq.rs` covers Eq
  parameter/parameter (`f`), Eq literal/literal (`h`), Eq parameter/literal (`g`), Ne
  parameter/parameter (`n`, :391), Ne parameter/literal (`m`, :441), operator mismatch both ways
  (:480) and Eq self-comparison (`same`, :502, traced to FR-033-AC-11). No Ne literal/literal or Ne
  self-comparison function or test exists. The SOURCE fixture (:24-31) has no such function.
- AC-14 and AC-15 are singular, testable and consistent with QSL FR-358 step 6: a composite literal
  is `graph_child` with empty bounds, and a parameter is `graph_child` with its Node bounds. They
  are also consistent with AC-1 (operator carried, `OperatorMismatch`), AC-9 (binding) and AC-11
  (no deduplication).
- AC ids are appended, not renumbered. `git log --all -S FR-033-AC-14` finds only 216b447.
  Open PRs #381, #396 and #397 touch none of the four files. FR-014/021/031 and TC-024/031/044 are
  untouched.
- Gates: `quire validate --scope . "spec/**/*.md"` exits 0; the warnings are older ones in
  untouched FR-017 and FR-014. `quire coverage --strict` goes from 214 unbacked / 0 contradicted at
  origin/main to 218 / 0 at 216b447. The only movement is FR-033-AC-14 to AC-17, each "no backing
  symbol". `quire matrix` differs only by those four untagged rows.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-033-AC-16 can never be exercised. Its condition, "Where the owning operand projection yields an inline integer literal operand of an Eq or Ne application", cannot occur. IR's `composite_application_operands` (quire-contract-model at the CG-locked 9be7387, composite_operands.rs:405-417) refuses every top-level inline literal as `UnsupportedOperand` (`InlineInteger` / `InlineNonInteger`) and only ever returns `GraphChild` (:481). IR FR-038 (:3215-3222) says this is defensive only: the catalog's `structural_kind` excludes Integer, so reader admission already refuses an integer operand of `structural.eq`/`structural.ne`, and no admitted fixture exists. A test author cannot build the input, so the AC passes vacuously forever. TC-048 step 1's "where the owning operand projection yields one" clause has the same problem. Remove AC-16, or restate it as the observable behaviour: an inline integer operand reaches CG only as IR's typed `InlineInteger` refusal, passed through `CompositeBuildError::Operands` with no report. The pre-existing AC-11 sentence "inline integer literals use application/occurrence/position and singleton Range" describes the same dead branch. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:376, spec/replay/matrix/TC-048-composite-parity-replay-binding.md:55-58 |
| FND-002 | medium | FR-033-AC-17 is marked PLANNED for behaviour main already produces. An inline non-integer operand is refused by IR as `UnsupportedOperand{InlineNonInteger}`, and CG maps that to `CompositeBuildError::Operands` before any QSL call (src/replay/composite_builder.rs:251-254), so it gets a typed refusal with no report or terminal value. QSL FR-358 step 6 also owns this check (`ScalarIdentity::NotInlineLiteral`) for any request that reaches it. As worded ("a typed inline-operand refusal"), AC-17 invites a new CG-owned variant that duplicates both. Restate AC-17 as passing IR's typed refusal through `Operands` with no report, and back it with a test rather than a new refusal. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:377 |
| FND-003 | low | AC-11's header now says "full proving-record/native integration is owned by IR-806 through AC-5, AC-6, AC-9 and AC-10". The proving record AC-11 refers to (Population-keyed metadata in the full CG proving record, FR-015-AC-76 evidence) is not stated by any of AC-5, AC-6, AC-9 or AC-10. AC-9 is also not in IR-806's described scope (FR-033 AC-2..6/8/10). The IR-814 ticket description, which is untrusted data, still lists "full proving-record / native integration" in IR-814's own scope. Name the criterion that owns the proving record, or drop "proving-record" from the IR-806 attribution, and align the IR-814 ticket. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:371 |

## Examined units

```yaml
scope:
- id: FR-033-AC-14
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: "PLANNED (IR-814). When the original application node is a `structural.ne` whose two operands are both composite literals and the requested operator is NotEqual, the verified request carries NotEqual and an O-09 of two positional `graph_child` arguments, each the literal's own node with empty Bounds and no parameter position."
- id: FR-033-AC-15
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: "PLANNED (IR-814). When the original application node is a `structural.ne` comparing one parameter with itself and the requested operator is NotEqual, the verified request carries NotEqual and an O-09 of two positional `graph_child` arguments with the same node identity, each retaining the drawn Node-keyed bounds naming that parameter and neither deduplicated."
- id: FR-033-AC-16
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: "PLANNED (IR-814). Where the owning operand projection yields an inline integer literal operand of an Eq or Ne application, the O-09 carries one argument at that operand's position, in operand order, identified by the application node, occurrence and position, with a singleton `Range` domain of the literal's exact value."
- id: FR-033-AC-17
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: "PLANNED (IR-814). If an inline literal operand has a type other than integer, then the builder refuses with a typed inline-operand refusal before any QSL invocation, with no report or terminal value."
- id: FR-033-AC-1
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: "PARTIAL (IR-635 verified Eq graph-child constructor controls and Ne parameter/parameter and parameter/literal controls, including operator mismatch refusal; Ne literal/literal is AC-14 and inline operands are AC-16 and AC-17, all PLANNED under IR-814; falsified and generated-artifact routes planned, IR-806)."
- id: FR-033-AC-11
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: "PARTIAL (IR-635 Eq graph-child positional O-09 controls and Ne parameter/parameter and parameter/literal controls; Eq self-comparison is covered; Ne literal/literal (AC-14), Ne self-comparison (AC-15) and inline operands (AC-16, AC-17) are PLANNED under IR-814, and full proving-record/native integration is owned by IR-806 through AC-5, AC-6, AC-9 and AC-10)."
- id: FR-033-AC-9
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: context_only
  excerpt: "PARTIAL (IR-666 report binding covered; IR-635 Eq-only verified source/package/node/occurrence prechecks and genuine invocation covered; generated-artifact authentication and falsified invocation planned, IR-806)."
- id: AD-003
  path: spec/assurance/AD-003-evidence-chain.md
  role: examined
  excerpt: "E-1 composite extension (PARTIAL: IR-635 delivered the Eq graph-child positional O-09 and Ne parameter/parameter and parameter/literal controls of FR-033-AC-11; Ne literal/literal, Ne self-comparison and inline literals (FR-033-AC-14 to AC-17) are PLANNED under IR-814; UNRUN CG CONSUMER, QSL-640)."
- id: TC-048
  path: spec/replay/matrix/TC-048-composite-parity-replay-binding.md
  role: examined
  excerpt: "For inline operands (FR-033-AC-16, FR-033-AC-17), where the owning operand projection yields one, require a singleton Range argument at the application/occurrence/position for an integer literal, and a typed refusal with no report for any other inline literal type."
bindings: []
```
