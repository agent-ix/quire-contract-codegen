---
id: SR-6301
title: "IR-494 caller Text admission architecture evaluation"
type: SpecReview
analysis: architecture-evaluation
scope: "agent-ix/quire-contract-codegen@d9955f18171edc58c4f3db1f8efbb715277b465f; PR #346 ADR-001 Q3, ADR-006, FR-035, TC-050; context: FR-017 Kani execution contract"
review_set: subset
---

## Summary

Ticket: IR-494. The proposal assigns caller `TextPayload` construction and admission outside checked expression provenance, and the 21-value symbolic construction can enter the public runtime API. The execution and evidence handoff for this third harness kind has no assigned owner: FR-017 accepts only contract and scalar harnesses, while FR-035 and TC-050 require real Kani proof and playback.

## Reviewed units

| Unit | Role | Excerpt |
| --- | --- | --- |
| ADR-001 Q3 | examined | Every checked-expression or contract-claim generator reads an admitted `quire.checked-package/v2` package through Contract IR's `CheckedPackageV2`. |
| ADR-006 Decision | examined | It does not enter FR-015's per-obligation checked-package Kani claim model or FR-025's Boolean/`i64` subject ABI; its own finite symbolic input and identity are required by FR-035. |
| ADR-006 Status | examined | The caller admission oracle and Kani route are unimplemented; FR-035 and TC-050 are Planned, not proof evidence. |
| FR-014-AC-2 | context_only | Text comparison descriptors for admitted checked-package expression nodes each generate a function calling the matching runtime `exact` operator. |
| FR-035 Description | examined | When a caller requests runtime Text admission, the code generator shall emit an oracle and a separately bounded Kani obligation for a caller-owned `TextPayload` and declared `TextType`. |
| FR-035 Outputs | examined | For each supported profile and declared bounded type in the proof request, one Kani harness whose identity records that profile, type bounds, the exact symbolic source class, solver/options and its non-vacuity cover. |
| FR-035-AC-4 | examined | For each of the six profiles and both `Text[0,8]` and `Text[1,1]`, a real Kani harness symbolically ranges over all 21 sequences of zero to two scalars from the four-scalar alphabet, constructs each `TextPayload` by `from_utf8`, compares the generated admission oracle with a separate direct runtime call including outcome and metering, and ends with one satisfied cover after its assertions. The proof identity names this finite class and its type. |
| FR-035-AC-5 | examined | In a copy of the generated oracle with its admission call or declared `TextType` altered so the oracle accepts a payload the direct runtime call refuses, real Kani falsifies the admission agreement assertion with concrete playback; restoring the call verifies over the same symbolic class. |
| TC-050 Steps 3-4 | examined | Generate twelve real-Kani harnesses; mutate only the generated admission call or its declared type in a copy and rerun real Kani. |
| FR-017 Inputs | context_only | One harness of either kind this requirement generates: an FR-015 contract harness or an FR-022/FR-014 exact-scalar harness. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new caller admission harness is excluded from FR-015's checked-package path, but no requirement assigns it to a Kani execution and evidence path. FR-017 accepts only `KaniObligationHarness` and `KaniScalarObligationHarness`; FR-035 specifies a distinct identity and TC-050 tests a one-off backend run. A caller proof request can therefore generate a harness with no defined production classification for verified, cover-unsatisfied, falsified playback, or infrastructure refusal. Define whether FR-017 accepts this third kind or which separate executor owns those outcomes and claim linkage. | spec/decisions/ADR-006-caller-text-admission-boundary.md:50; spec/oracle/functional/FR-035-caller-text-admission.md:17,53,101; spec/oracle/matrix/TC-050-caller-text-admission.md:32; spec/kani/functional/FR-017-kani-execution-evidence.md:21,44 |

## Verdict

The ingress and checked-expression responsibilities have clear owners. The production proof execution boundary needs an explicit owner before implementation can claim verified caller admission evidence.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | c55c6ca6984a2d19c79c99243bdedc986653eb33 |

After excerpt (FR-017 Behavior): "When executing an FR-035 caller Text-admission harness, the generator shall retain its typed caller-ingress identity from the selected harness in the execution evidence, bind the outcome and any playback to that identity and harness path, and carry `None` for contract obligation kind." FR-017-AC-26 and TC-027/TC-050 keep this execution route Planned until implementation.
