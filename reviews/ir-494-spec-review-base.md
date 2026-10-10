---
id: SR-6300
title: "IR-494 caller Text admission specification review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@d9955f18171edc58c4f3db1f8efbb715277b465f; PR #346 spec/decisions/ADR-001, ADR-006; spec/oracle/functional/FR-014, FR-035; spec/oracle/matrix/TC-024, TC-050, tests.md; spec/kani/matrix/tests.md; spec/spec.md"
review_set: subset
---

## Summary

Ticket: IR-494. The caller ingress boundary, the finite 21-payload class, twelve profile/bound harness combinations, and Planned matrix status are internally consistent with the inspected code. The new requirement misidentifies Contract Runtime's exact API as a re-export from `quire-exact`; Contract Runtime defines and exports its own text and accounting types.

## Reviewed units

| Unit | Role | Excerpt |
| --- | --- | --- |
| ADR-001 Q3 | examined | Every checked-expression or contract-claim generator reads an admitted `quire.checked-package/v2` package through Contract IR's `CheckedPackageV2`. |
| ADR-006 Decision | examined | The caller Text-admission oracle and its proof form a separate, explicitly caller-declared ingress route under FR-035. |
| FR-014-AC-2 | examined | Integer (add, subtract, multiply, negate, truncating/floor/Euclidean division, modulo), rational, ordering, decimal, IEEE arithmetic/comparison/width conversion, text comparison, enum comparison, and quantity arithmetic/comparison/conversion descriptors for admitted checked-package expression nodes each generate a function calling the matching runtime `exact` operator. |
| FR-035 Description | examined | The oracle calls `quire-exact::admit_text` through Contract Runtime's `exact` API and returns its outcome and metering unchanged. |
| FR-035 Dependencies | examined | Contract Runtime's `exact` text and accounting API, which re-exports `quire-exact::admit_text`, `TextPayload`, `TextType` and `Meter`. |
| FR-035-AC-1 | examined | A generated caller-ingress oracle accepts `&TextPayload`, the declared `TextType` and `&mut Meter`, calls `rt::admit_text` once, and returns the runtime outcome, retained text, original runtime provenance, charge sequence and consumed counters unchanged for all six profiles over representative empty, ASCII, combining, composed and supplementary payloads. |
| FR-035-AC-2 | examined | Invalid UTF-8 bytes make `TextPayload::from_utf8` return `InvalidUtf8` before admission and leave the admission meter unchanged; `admit_text` itself is never claimed to accept those bytes. |
| FR-035-AC-3 | examined | For each profile, `Text[1,1]` admits or refuses according to the profile length of the retained sequence, including `e` plus U+0301 under NFC and NFD; a length refusal occurs before `text.result-retain`, and denial of each reached charge point yields the direct runtime stop, charges and consumed counters. |
| FR-035-AC-4 | examined | For each of the six profiles and both `Text[0,8]` and `Text[1,1]`, a real Kani harness symbolically ranges over all 21 sequences of zero to two scalars from the four-scalar alphabet, constructs each `TextPayload` by `from_utf8`, compares the generated admission oracle with a separate direct runtime call including outcome and metering, and ends with one satisfied cover after its assertions. The proof identity names this finite class and its type. |
| FR-035-AC-5 | examined | In a copy of the generated oracle with its admission call or declared `TextType` altered so the oracle accepts a payload the direct runtime call refuses, real Kani falsifies the admission agreement assertion with concrete playback; restoring the call verifies over the same symbolic class. |
| FR-035-AC-6 | examined | A request outside the declared finite proof class has a typed unsupported or requires-bound disposition and no harness, and no caller-ingress claim is marked `ir_confirmed` or assigned a checked-package node id. The six attempted text-operand `numeric.convert` nodes remain Contract IR refusals and do not count as passing caller-admission coverage. |
| TC-024 Step 6 | examined | Keep the six `TextAdmission` fixtures in the refused corpus and check that Contract IR rejects each attempted text-operand `numeric.convert` node. |
| TC-050 Steps 1-5 | examined | Generate twelve real-Kani harnesses: every profile crossed with both `Text[0,8]` and `Text[1,1]`. |
| TM-003 FR-035 row | examined | Planned (IR-494); no caller-ingress generated oracle, bounded `TextPayload` Kani harness, real-Kani mutation control or runtime agreement test exists. |
| Kani matrix FR-035 row | examined | Planned (IR-494); caller `TextPayload` has no symbolic Kani input or generated admission harness. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-035 says Contract Runtime re-exports `quire-exact` text admission and types, but its `exact` module defines its own `text` and `accounting` modules and exports those symbols. This names the wrong API owner and would send an implementation toward incompatible types; describe the Contract Runtime API directly. | spec/oracle/functional/FR-035-caller-text-admission.md:21,107; /home/peter/dev/quire-contract-runtime/src/exact/mod.rs:86,89,151; /home/peter/dev/quire-contract-runtime/src/exact/text.rs:363 |

## Verdict

One medium requirement accuracy finding. The checked-expression and caller ingress distinction, 1 + 4 + 16 = 21 payload count, six profiles times two bounds, constructor refusal boundary, and Planned evidence labels need no correction in this diff.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | c55c6ca6984a2d19c79c99243bdedc986653eb33 |

After excerpt (FR-035 Dependencies): "Contract Runtime's own `exact` text and accounting API, which defines and exports `admit_text`, `TextPayload`, `TextType` and `Meter`." FR-035 Description also names Contract Runtime's own `exact::admit_text` API.
