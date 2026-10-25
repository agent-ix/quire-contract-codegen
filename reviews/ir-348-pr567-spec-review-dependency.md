---
id: SR-35017
title: "Dependency review of AD-004 L-12 correction"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-contract-codegen; spec/assurance/AD-004-cg-crate-layout.md"
review_set: subset
---

## Summary

Reviewed step 4c to 4f order against the live routed enum at PR #567 head 1340120353d0b47773d308a3772846b2232d60ae. The V2 generator prerequisite is present, but no step owns a replacement routed form or its admission before deleting the public V1 form. Ticket: IR-348. No applicable AssuranceProfile was found in the repository spec tree.

## Verdict

**CONDITIONAL** — one medium missing-owner finding.

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| AD-004-step-4c | examined | The precondition and contract families take the V2 input and render through HarnessSpec; the V1 arm remains until 4f. |
| AD-004-step-4e | examined | The one public generator entry exists before QSL moves the exemplar control. |
| AD-004-step-4f | examined | Remove routed/capability.rs's public KaniForm::V1BoundClause, its KaniFormName identity and V1 clause preflight when the V2 contract-family form replaces that route. |
| AD-004-L-12 | examined | After step 4f no BoundPackage or BoundClause name remains in kani/, routed/ or replay/. |
| AD-004-IR-364-gate | context_only | IR-364 owns the V2 strategy replacement; its gate does not delete V1 Kani readers. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Step 4c specifies only a V2 input for Kani precondition and contract families. The live public routed KaniForm has V1BoundClause but no V2 clause or contract-family form. Step 4f now requires that replacement before removing V1BoundClause, yet neither 4c, 4d nor 4e assigns creation of the routed form, identity, preflight and tests. Assign that prerequisite to a step and require its route to be live before 4f deletes V1. | spec/assurance/AD-004-cg-crate-layout.md:1059 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At e72d3f432af5582de0711729600bba2ba83bc927, step 4e owns the public V2 routed contract-clause form, identity, preflight and tests. It depends on the step 4c lowering, requires a preceding FR-019 spec change retaining AC-25/26/33 obligations, and gates the QSL control move and step 4f's V1 deletion. |
