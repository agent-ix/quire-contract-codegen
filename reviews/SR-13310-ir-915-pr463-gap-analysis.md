---
id: SR-13310
title: "IR-915 PR 463 gap analysis: excluded accounting-only CG positions"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@fc288843; FR-039-AC-9, FR-019-AC-53 vs spec/core/matrix/tests.md, spec/routed/matrix/tests.md, TC-053, TC-030; src/core/contract_package.rs, src/routed/capability.rs, src/routed/generate.rs"
review_set: subset
---

## Summary

Ticket: IR-915. PR agent-ix/quire-contract-codegen#463. This is a planless gap analysis; plan
completion was not assessed. The PR changes no code and no tags.

`quire matrix --scope .` (quire 0.36.2) reports FR-039-AC-9 and FR-019-AC-53 as `untagged`,
with no binder. That matches the 🚧 Planned (IR-915) rows. No CG code yet has an
excluded-position kind: `ContractRequestPosition` has only `DriverItem` and `ContextRoot`. So no
existing tag could truthfully claim either criterion, and none does.

No production code is introduced, so there is no orphan code. The test plans (TC-053 step 7 and
the TC-030 paragraph) cover each criterion's main path and its count and correspondence adverse
cases. The test-plan gaps behind SR-13307 FND-002, FND-003 and FND-004 are recorded there, not
repeated here.

## Examined criteria

- FR-039-AC-9 (examined): untagged. The Planned row is truthful, and TC-053 step 7 covers the main path plus the remove, duplicate and change-`node_id` refusals.
- FR-019-AC-53 (examined): untagged. The Planned row is truthful, and the TC-030 paragraph covers the two-item settle and the one-item and three-item `EnvelopeItemCount` refusals.
- FR-019-AC-48 through AC-52 (context_only): their ✅ Covered row is unchanged and still accurate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

**PASS.** The matrix states the coverage truthfully: both new criteria are Planned and untagged,
and each has a TC procedure. The weaker spots in the test plans are SR-13307 findings.
