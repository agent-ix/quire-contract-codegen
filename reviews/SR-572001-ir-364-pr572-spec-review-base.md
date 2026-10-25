---
id: SR-572001
title: "IR-364 PR #572 spec-review/base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #572 head bc7213ef556911eb71a19e02b83554c8dde4999f from base 1944268f; spec/strategy/functional/FR-011-numeric-harness-campaigns.md, spec/strategy/functional/FR-002-tristate-proptest.md, spec/strategy/matrix/TC-020-numeric-harness-campaigns.md, spec/strategy/matrix/TC-004-proptest-strategies.md, spec/strategy/matrix/tests.md; IR-364"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: reviews
---

## Summary

Non-author base spec review of draft PR #572 at frozen head bc7213ef, scoped to its diff. Ticket:
IR-364. Reviewed against the planner's execution-point decision recorded on IR-364 (comment
689a9b9e; options in 64d074bd), the QSL owner type (`qsl` `ClauseBinding`, whose
`execution_point: quire_contract_model::ExecutionPoint` is mandatory and kind-checked at QSL
`origin/main` `src/checking.rs` / `src/checking/bindings.rs`), CG FR-021's FR-485 authentication
pattern, and AD-004 L-12.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The authentication source is not an input: FR-011 and FR-002 compare the request point with "the accepted QSL binding point" and "the caller invocation context", but neither Inputs list names either as a distinct typed input, its owning type (QSL `ClauseBinding` / `quire_contract_model::ExecutionPoint`) or how CG obtains it; read literally, the request supplies both sides, so the fail-closed check can be satisfied by any caller value | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:49-51; spec/strategy/functional/FR-011-numeric-harness-campaigns.md:72-77; spec/strategy/functional/FR-002-tristate-proptest.md:21-26 |
| FND-002 | medium | Spec adds a branch the decision does not state: caller context is used "when the binding supplies none", i.e. a binding that exists but has no point; the decision says "when no binding exists", and QSL's accepted `ClauseBinding` always carries a point, so this is either an unreachable case the TCs must construct or an unauthorised fallback | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:72-74; spec/strategy/functional/FR-002-tristate-proptest.md:82-84 |
| FND-003 | medium | With two selected clauses at possibly different points, the point carried by a `Passed` verdict (both oracles true) and by a non-completed case is unspecified; FR-002 Behavior has no verdict-point statement at all and AC-19 covers only rejection and failure | spec/strategy/functional/FR-002-tristate-proptest.md:110; spec/strategy/functional/FR-002-tristate-proptest.md:82-86 |
| FND-004 | medium | "Corresponding runtime execution point" is undefined: the typed point is `quire_contract_model::ExecutionPoint` (variant plus `AnchorName`) while runtime `ExecutionPoint` is a string newtype; the rendering (variant only, or with anchor name) decides whether two invariants on different handlers stay distinct, and AC-6/AC-19 cannot be checked exactly without it | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:78-81; spec/strategy/functional/FR-011-numeric-harness-campaigns.md:128 |
| FND-005 | medium | Caller-context path has no kind/point compatibility check: QSL refuses a binding whose point does not fit its clause kind, but a caller-supplied `Handler` point for a `Precondition` would be accepted and recorded in `VerdictContext`; options note 64d074bd proposed CG validate kind/point compatibility, decision 689a9b9e is silent; planner decision needed, not author invention | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:72-77 |

## Verdict

NOT MERGEABLE (one high). What matches the decision: the request carries an explicit typed point
per selected clause; both selections are checked independently and may differ; missing and
mismatched points refuse with named variants and no artifact; clause kind is excluded as a source
in Inputs, Behavior and ACs; no default point and no compatibility layer appear. FR-011 OQ-1 is
removed correctly and OQ-2 (identity) stays open; FR-002 OQ-1 is reduced to identity without
narrowing AC-7 to AC-18. No `BoundPackage`/`BoundClause`/`ClauseRef` is named (FR-002-AC-9 and
AD-004 L-12 hold), and CG defines no binding type of its own, though FND-001 asks the spec to name
the QSL owner. Context, not findings: interface-001 line 327 still says verdicts use "the IR
ExecutionPoint's serialized name"; the PR defers it to a companion PR. Open PR #571 rewrites the
same FR-011 `construct_verdict` sentence, FR-002-AC-12 and FR-002 OQ-1, so whichever lands second
must be reconciled.

## New findings (disposition pass 1)

Checked at fix-round head fd1de474dd98225d6817e921f4d012e3c3ff9f5f.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | FR-011 and FR-002 Inputs say CG reads ClauseBinding.execution_point through the admitted V2 node's typed access, but quire-contract-ir origin/main has no V2 checked-package accessor returning an ExecutionPoint or binding (no ExecutionPoint in crates/quire-contract-model/src/checked_package/**), and QSL's export_bindings (FR-307, ADR-017 AR-4) exports (AbstractionKey, CheckedBinding) pairs with no execution point. No relationship or Dependencies entry names the upstream IR/QSL requirement that would add it, so this is decision option 2 without its upstream dependency. Also, quire_contract_model::ExecutionPoint is owned by quire-contract-ir, not QSL; QSL owns ClauseBinding. | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:49-51 |

## New findings (disposition pass 2)

Checked at fix-round head cc1917b557a633fc65a29bac65a546c94efbc19e.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | The new depends_on relationship in FR-011 and FR-002 targets ix://agent-ix/quire-contract-ir/IR-1013, but IR-1013 is a Linear ticket, not a spec artifact; quire-contract-ir origin/main has no artifact with that id, so the edge resolves nowhere and no other relationship in this repo targets a ticket id. The Dependencies prose already names IR-1013; drop the two relationship entries or retarget them to the Contract IR spec artifact once IR-1013 authors it. | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:26-27 |

## Dispositions

Round 1, checked against the repository at head fd1de474dd98225d6817e921f4d012e3c3ff9f5f.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Single source: FR-011 and FR-002 Inputs name the selected V2 node's accepted QSL ClauseBinding.execution_point; no request or caller point remains (grep at fd1de474), so the caller can no longer supply both sides. |
| FND-002 | fixed | Caller-context branch removed; MissingExecutionPoint now means the selected node has no accepted binding point. |
| FND-003 | fixed | FR-002 Behavior and AC-19 assign rejected, failed, passed (after completed postcondition) and non-completed points; TC-004 observes each. |
| FND-004 | fixed | Rendering <variant>:<utf8-byte-length>:<AnchorName> over a fixed four-variant set is injective; FR-011-AC-6 and TC-020 step 7 add the two-handler case. |
| FND-005 | fixed | Caller path removed; the only source is the accepted QSL binding, whose point QSL kind-checks. |
| FND-006 | fixed | Round 2 at cc1917b5: FR-011 and FR-002 Inputs mark the accessor Planned (IR-1013) and state the current V2 API lacks it; both Dependencies sections name IR-1013 as blocking; ownership now reads QSL owns ClauseBinding, Contract IR owns quire_contract_model::ExecutionPoint. Linear: IR-1013 blocks IR-364. |

## Coverage

```yaml
scope:
- id: FR-011
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "When generation is requested, the generator shall require the request's typed execution point to equal the selected clause's accepted QSL binding point when present, or the caller's typed invocation point when the binding supplies none. A missing source or missing request point shall yield `MissingExecutionPoint`; a mismatch shall yield `ExecutionPointMismatch`. Either refusal produces no artifact; the generator shall not infer a point from clause kind or silently substitute another point."
- id: FR-011-AC-6
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "For a selected clause with an accepted QSL binding point, the V2 request carries that exact typed point; for a selected clause without such a binding point, it carries the caller invocation context's exact typed point. The generated runner passes the corresponding runtime execution point to `construct_verdict` for each case, including an invariant at an initialization or handler anchor."
- id: FR-011-AC-7
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "A missing request point or a missing point in both the binding and caller context yields `MissingExecutionPoint`; a request point differing from an available accepted binding point or from the caller context when no binding point exists yields `ExecutionPointMismatch`. Each case produces no artifact. No point is inferred from `Invariant`, `Precondition`, or `Postcondition` kind."
- id: FR-002
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "Planned (IR-815): the V2 harness generator shall independently verify each selected clause's request point against that clause's accepted QSL binding point when supplied, or its caller invocation context when the binding supplies none. A missing source or request point shall return `MissingExecutionPoint`; a mismatch shall return `ExecutionPointMismatch`; both produce no artifact. Clause kind shall not supply a default point."
- id: FR-002-AC-12
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "PLANNED (IR-815). Repeated V2 harness generation for the same admitted package, selected clause identities, both explicit typed execution points and campaign policy is byte-identical."
- id: FR-002-AC-19
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "PLANNED (IR-815). A V2 harness request preserves each selected clause's exact typed execution point from its accepted QSL binding when supplied, or its caller invocation context otherwise; precondition and postcondition points may differ. A precondition rejection verdict uses the precondition's corresponding runtime point, and a postcondition failure verdict uses the postcondition's corresponding runtime point, without inferring either from clause kind."
- id: FR-002-AC-20
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "PLANNED (IR-815). For either selected clause, a missing request point or missing point in both the accepted binding and caller context yields `MissingExecutionPoint`; a request point mismatched with its available source yields `ExecutionPointMismatch`; each produces no harness artifact."
- id: TC-020
  path: spec/strategy/matrix/TC-020-numeric-harness-campaigns.md
  role: examined
  excerpt: "Independently omit the V2 request point; omit both the binding and caller context points; supply a request point different from the accepted QSL binding point; and, with no binding point, supply one different from the caller invocation point. Inspect each refusal and artifact output."
- id: TC-004
  path: spec/strategy/matrix/TC-004-proptest-strategies.md
  role: examined
  excerpt: "Independently for either selection, omit the request point, omit both the binding and caller points, mismatch the request point against the available binding, and mismatch it against the caller context when no binding point exists. Inspect every refusal and artifact output."
- id: FR-021
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: context_only
  excerpt: "If the FR-485 acquisition's checked owner package id or selected callable id differs from the requested admitted V2 package/function identity, the generator shall refuse that item as `OwnerIdentityMismatch`"
- id: AD-004
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: context_only
  excerpt: "L-12. `BoundPackage` and `BoundClause` are named nowhere in `kani/`, `routed/` or `replay/`, and after the last V1 reader is replaced nowhere."
```
