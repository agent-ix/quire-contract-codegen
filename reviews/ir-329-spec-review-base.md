---
id: SR-5420
title: IR-329 PR 344 base review
type: SpecReview
analysis: base
review_set: subset
scope: agent-ix/quire-contract-codegen@513ae51d0eab9f6e6b5c396b3b2c695fae793993; spec/kani/functional/FR-015-bounded-kani-obligations.md,
  spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, spec/spec.md, spec/core/functional/interface-001-codegen-api.md,
  spec/assurance/AD-001-codegen-architecture.md
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: reviews
- target: ix://agent-ix/quire-contract-codegen/TC-023
  type: references
---

# SR-5420: IR-329 PR 344 base review

## Summary

Clean: the thirteen new criteria use valid IDs, name a verification method, state concrete outcomes and adverse cases, and the change accurately labels their computed trace bindings as untagged. The base checklist records the untagged state as a planned implementation gap, without claiming code completion.

## Examined units

| ID | Role | Excerpt |
| --- | --- | --- |
| FR-015-AC-82 | examined | While the bounded-profile classifier remains, a request for distinct named constructs returns one `CapabilityEntry` per requested construct in request order, preserving each selected profile disposition (`Supported`, `Refused` or `Inconclusive`). |
| FR-015-AC-83 | examined | While the bounded-profile classifier remains, an empty, duplicate or absent construct is a typed `KaniOutcome` refusal rather than a partial disposition census. |
| FR-015-AC-84 | examined | While the finite-input arithmetic lowerer remains, a profile/dispatch-admitted add, subtract, multiply, divide or remainder returns the exact checked `i128` result only inside the request's inclusive result range. |
| FR-015-AC-85 | examined | While the finite-input arithmetic lowerer remains, an inverted result range is `InvalidInput`; zero division, checked overflow and a result outside the range are typed refusals, with no arithmetic result or generated artifact. |
| FR-015-AC-86 | examined | While the finite-graph lowerer remains, a profile/dispatch-admitted request follows only the selected reference field, finds a positive-length path in sorted depth-first order, and checks the expansion bound before entering each new object; a target edge is found without expanding its target. |
| FR-015-AC-87 | examined | While the finite-graph lowerer remains, a zero expansion bound or unknown endpoint is `InvalidInput`, and exhausting the bound is `ResourceExhausted`, with no graph result or generated artifact. |
| FR-015-AC-88 | examined | While the finite-collection lowerer remains, a profile/dispatch-admitted `ForAllNonNegative` or `ExistsEqual` query retains input order and duplicates and returns its truth value and examined-item count; a decisive item ends that count, otherwise every item counts. |
| FR-015-AC-89 | examined | While the finite-collection lowerer remains, an input longer than `max_items` is `ResourceExhausted`, with no query result or generated artifact. |
| FR-015-AC-90 | examined | While the bounded-corpus emitter remains, one admitted arithmetic, graph or collection case emits all four artifacts (oracle, finite strategy, Kani harness and proof-dependency graph) from the same selected profile, validated finite input and family lowering. |
| FR-015-AC-91 | examined | While the bounded-corpus emitter remains, its generated outcome is `Proved` when the family lowering's Boolean oracle is true and `Counterexample` when false; neither is an observed Kani run or native replay verdict. |
| FR-015-AC-92 | examined | While the bounded-corpus emitter remains, a declared dependency census with an empty or repeated identity, inconsistent kind/state/path or a kind other than `Required` refuses as typed `InvalidInput` with no artifact or case-identity claim; a valid census is normalized into the emitted proof graph. |
| FR-015-AC-93 | examined | While the bounded-corpus emitter remains, a profile/input mismatch, family-lowering refusal, proof-graph serialization refusal or duplicate case identity returns its typed refusal with the selected profile revision, emits no artifact and claims no new case identity. |
| FR-015-AC-94 | examined | While the bounded-corpus emitter remains, equal case inputs produce equal artifact paths and contents regardless of emission order, and reordering the same finite graph's objects or references does not change them; changing a request field, selected profile, finite input, ceilings or normalized dependency census changes the case identity and artifact paths. |
| TC-023 | examined | Verify the current bounded-profile classifier, three CG-owned finite-input lowerers and four-artifact corpus emitter against FR-015-AC-82 through FR-015-AC-94. |
| AD-004 | context_only | It returns no `KaniOutcome` at generation time: a verdict comes only from a run. |
| ADR-001 | context_only | FR-015 is the Kani backend’s one generator; `CheckedPackageV2` is the one input model. |
| spec/spec.md Kani ownership map | examined | The current Kani module and public-surface ownership map is: |
| interface_001 finite-input operations | examined | status: live finite-input auxiliary under FR-015-AC-82 and AC-83; AD-004 step 4g governs retirement of the hand-built corpus path |
| AD-001 Current state | examined | The bounded-profile classifier, three finite-input lowerers and corpus emitter have interim ownership under FR-015-AC-82 to FR-015-AC-94; |
| FR-015-AC-52 | context_only | The context of every outcome and refusal the corpus generator returns (proved, counterexample, `kani_corpus_dependency_invalid`, `kani_corpus_identity_collision`, a lowering refusal and `kani_profile_input_mismatch`) is the profile selection's revision. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean: the thirteen new criteria use valid IDs, name a verification method, state concrete outcomes and adverse cases, and the change accurately labels their computed trace bindings as untagged. The base checklist records the untagged state as a planned implementation gap, without claiming code completion.
