---
id: SR-514006
title: "IR-914 PR #571 spec-review/ears-conformance review"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; PR #571 head d4b7e842fd28bb9c21e413cc3a807d1fa83782f2 from main 1944268f; spec/evidence/functional/FR-004-vacuity-evidence.md, spec/strategy/functional/FR-002-tristate-proptest.md, spec/strategy/functional/FR-011-numeric-harness-campaigns.md; IR-914"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-004
    type: reviews
---

## Summary

EARS review of the requirement statements PR #571 adds or edits in FR-002, FR-004 and FR-011.
Ticket: IR-914. `quire validate --summary` reports 5/5 files grammar-clean with 0 engine grammar
findings; one semantic finding: an unwanted-input rule stated as a participle clause with no
named response.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-011 "Comparisons shall use the exact typed fields ... rejecting malformed or noncanonical text instead of normalizing it" states an unwanted-input behaviour as a trailing participle with no `If ... then` form and no named result in FR-011 (FR-004-AC-10 names one only for coverage binding) | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:137-140 |

## Verdict

PASS with one low note. The new FR-011 bullets and the canonical-rendering section name CG or the
runner as subject, use one `shall` per obligation, and give concrete, checkable responses (exact
field grammar, no truncation, case folding or new digest). The edited FR-002 and FR-004 text is
input/open-question prose and criteria, not new EARS statements.

## Coverage

```yaml
scope:
- id: FR-011
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "Comparisons shall use the exact typed fields before rendering or the exact canonical strings after rendering, rejecting malformed or noncanonical text instead of normalizing it."
- id: FR-011-rendering
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "For a selected QSL `RequirementItem` whose occurrence was accepted by `export_bindings`, CG shall render its `OccurrenceKey` as `qocc2_<node>_<role-byte-count>_<role-hex>_<ordinal>`."
- id: FR-002
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "The requirement identity is each selected QSL `RequirementItem` occurrence accepted by `export_bindings`; the revision identity is the admitted package's `package_id`."
- id: FR-004
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: "Each selected QSL `RequirementItem` accepted by `export_bindings` supplies its requirement occurrence; the admitted package's `package_id` supplies the revision."
```

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | At head 73e48d3c FR-011 reads "If an identity text comparison boundary receives malformed or noncanonical text, then CG shall return `InvalidIdentityRendering` before bundle publication, verdict construction, or report mutation; CG shall not normalize it." FR-011-AC-11 verifies it. |
| FND-002 | fixed | Head a540b38b. FR-002 Behavior now has Planned If-then refusals with no artifact: `HarnessErrorCode::OwnerIdentityMismatch` for a differing process `request.package_id`, `HarnessErrorCode::UnboundModelElement` naming the absent element, and `HarnessErrorCode::InvalidIdentityRendering` for malformed or noncanonical text, without normalization. These back FR-002-AC-22 and AC-23. The diff from dd07a018 is those ten Behavior lines only. |

## New findings (disposition pass 2)

Narrow review at head dd07a018e5d30f64062640bba5eeb86a1a95b2a4 of the one new addition, FR-002-AC-23 with its TC-004 control and matrix row. AC-23 is PLANNED, testable through the new TC-004 corruption control, and narrows nothing.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | FR-002-AC-23 (and AC-22) have no FR-002 Behavior statement behind them: the only If-then for malformed identity text is FR-011's, which returns `StrategyErrorCode::InvalidIdentityRendering`; FR-002 lists the `HarnessErrorCode::` variants only in Outputs, so the harness refusal is an AC with no requirement statement in its own FR | spec/strategy/functional/FR-002-tristate-proptest.md:43-46; spec/strategy/functional/FR-002-tristate-proptest.md:160 |
