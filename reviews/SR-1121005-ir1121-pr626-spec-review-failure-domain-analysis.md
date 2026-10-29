---
id: SR-1121005
title: "IR-1121 CG PR 626 spec-review-failure-domain-analysis"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-047-v2-function-proof-producer.md; spec/kani/matrix/TC-061-v2-function-proof-producer.md; spec/kani/matrix/tests.md; spec/spec.md; spec/tests.md"
review_set: subset
---

## Summary

Ticket: IR-1121. Reviewed PR #626 at 09d3cd428219279c1cde7308e809dc48cae5d98b against bd24ad40e009025e632d058fa0570fc1c6ca1a96. Method: spec-review-failure-domain-analysis. No applicable AssuranceProfile found under spec/.

## Examined criteria

- FR-047-AC-1 (role: examined; excerpt): An admitted current-V2 function claim selected at one exact occurrence produces one executable Kani harness and persisted identity. The harness calls the generated FR-021 owner-path oracle and the installed backend verifies a healthy subject with a satisfied cover; a separately admitted body-only mutant at the corresponding authored claim position is falsified with its own identity and retained in-domain assertion playback. A schedule-only or test-built identity is not evidence. PLANNED (IR-1121
- FR-047-AC-2 (role: examined; excerpt): For two distinct claim occurrences on one node, both outputs retain their own occurrence, original request index and selected declaration; their persisted identities and replay contexts do not cross-bind. A function name or equal body cannot merge them. PLANNED (IR-1121).
- FR-047-AC-3 (role: examined; excerpt): Unknown claim or declaration, ambiguous occurrence or declaration, and a claim whose selected form is not a function each yield distinguishable typed selection refusals before oracle emission, Kani execution or playback decoding; an unrelated admitted sibling is unchanged. PLANNED (IR-1121).
- FR-047-AC-4 (role: examined; excerpt): Changing only the admitted package, source owner or source bytes, dependency selection, request index, function declaration, claim occurrence, obligation kind, parameter identity/domain, generated artifact or proof ceiling produces a named typed mismatch or a distinct admitted proof identity; the harness and replay package never bind across those changes. The canonical proof-content identity may bind exact proved content; no tracking hash or source-version pin is introduced. PLANNED (IR-1121).
- FR-047-AC-5 (role: examined; excerpt): The retained context builds `ReplayPackage::new` from the original `ReplayInputs` and binds the emitted `KaniObligationIdentity` with `with_kani_identity`. The same package and identity admit CG's typed `replay_counterexample` for a falsifying Kani transcript; wrong package, source, dependency, request or identity is refused with its typed cause before native replay. PLANNED (IR-1121).
- FR-047-AC-6 (role: examined; excerpt): A transcript one byte above the smaller caller/capture byte ceiling, or one decoded value above the smaller value-count ceiling, is refused before native replay. An in-bound transcript decodes values only against the emitted identity's complete ordered domains; missing, extra and out-of-domain values do not become a reproduced failure. PLANNED (IR-1121).
- FR-047-AC-7 (role: examined; excerpt): The producer uses one admitted checked package and owner-selected callable for generation and proving, and preserves FR-021's `Undefined`, `Refused`, `Incomplete` and checked-invariant distinctions. The FR-042 wrong-completed-result and genuine environment-construction guards remain their own separately planned producer tests; this criterion does not count an invented fault as coverage of either guard. PLANNED (IR-1121; FR-021/FR-042 prerequisites).
- FR-047 Description, Inputs, Outputs, Behavior and Dependencies (role: examined).
- TC-061 procedure, expected results and Planned status (role: examined).
- FR-021, FR-016, FR-039 and FR-042 (role: context_only).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The draft names ReplayPackage::with_kani_identity as if available and claims mismatched package/identity is refused before decode. Current ReplayPackage has no such method and replay_counterexample consumes the V1-clause-bearing KaniObligationIdentity without checking package identity before decoding. Specify the CG-owned V2 identity/package binding API and typed mismatch point, including how it replaces the mandatory V1 ClauseRef for this producer; keep it Planned. | spec/kani/functional/FR-047-v2-function-proof-producer.md:126 |

| FND-002 | medium | FR-047 permits a function declared by an explicitly selected dependency, but the named ReplayPackage::new(ReplayInputs, function) calls QSL call_site for a function selected in the unit source only. Define a dependency-owner function-site selection and its source/lock join, or state this branch as blocked on an upstream API; the existing constructor cannot admit the promised dependency case. | spec/kani/functional/FR-047-v2-function-proof-producer.md:49 |

## Verdict

CHANGES REQUESTED. Scope limited to the changed spec and its declared owner boundaries.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The original claim that with_kani_identity was absent was wrong; the real defect was using its mandatory V1 ClauseRef for a V2 function. The revised Outputs defines distinct planned V2FunctionProofIdentity, bind_v2_function_replay and BoundV2FunctionReplay with predecode mismatch refusal. |
| FND-002 | fixed | The revised Inputs and Behavior explicitly hold dependency-owned selection on QSL IR-1122, require the actual declaring package and occurrence from the original lock, and forbid unit-site substitution. |

Disposition checked at PR #626 head dddc4e2961e470ad743a0172253c41f8d5288288.
