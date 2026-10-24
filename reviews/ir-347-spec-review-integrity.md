---
id: SR-17007
title: "IR-347 CG #562 spec integrity analysis"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen#562; spec/assurance/AD-003-evidence-chain.md; spec/assurance/AD-004-cg-crate-layout.md"
review_set: subset
---

## Summary

Ticket: IR-347. Structural and cross-reference integrity of the two changed AD files at exact head `036539b64a2fd89f99365f80fc2802447ee33e2a`. `quire validate --scope .` passed on both touched files, with only module-registry warnings (DuplicateArchetype, DuplicateInverseEdge, semantic.inline-data-schema) that are unrelated to the diff. `quoin validate --repo . --json` returned `findings: []`. `git diff --check` was clean. Every id the diff cites resolves in `spec/kani/functional/FR-015-bounded-kani-obligations.md`: FR-015-AC-91, AC-96 to AC-102, AC-109, AC-110 and AC-111. Every path it cites as current exists under `src/kani/generate/lower/`. The table rows keep their column counts. Step 2f historical counts (item-count table, step 2f inventory) are kept as history and marked so.

## Examined units

- AD-003 seam bullet cross-references (examined).
- AD-004 module-map, renderer, interim-exception rows: column structure and cited ids (examined).
- AD-004 step 2f item map bullet: historical marking (examined).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

**PASS.** The diff introduces no structural or cross-reference defect.
