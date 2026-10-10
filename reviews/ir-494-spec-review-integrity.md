---
id: SR-6304
title: IR-494 integrity review
type: SpecReview
analysis: integrity
scope: agent-ix/quire-contract-codegen@d9955f18171edc58c4f3db1f8efbb715277b465f; spec/oracle/functional/FR-035-caller-text-admission.md, spec/oracle/matrix/TC-050-caller-text-admission.md, spec/decisions/ADR-006-caller-text-admission-boundary.md
review_set: subset
---

## Summary

Ticket: IR-494. FR-035 maps to StR-001 and TC-050; all six criteria have a Planned matrix row, and the finite 21-value class equals 1 + 4 + 16. No additional completeness, contradiction, atomicity, or testability defect emerged beyond SR-6300 and SR-6301.

## Reviewed units

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-035 | examined | When a caller requests runtime Text admission, the code generator shall emit an oracle and a separately bounded Kani obligation for a caller-owned `TextPayload` and declared `TextType`. |
| FR-035-AC-1 | examined | A generated caller-ingress oracle accepts `&TextPayload`, the declared `TextType` and `&mut Meter`, calls `rt::admit_text` once, and returns the runtime outcome, retained text, original runtime provenance, charge sequence and consumed counters unchanged for all six profiles over representative empty, ASCII, combining, composed and supplementary payloads. |
| FR-035-AC-2 | examined | Invalid UTF-8 bytes make `TextPayload::from_utf8` return `InvalidUtf8` before admission and leave the admission meter unchanged; `admit_text` itself is never claimed to accept those bytes. |
| FR-035-AC-3 | examined | For each profile, `Text[1,1]` admits or refuses according to the profile length of the retained sequence, including `e` plus U+0301 under NFC and NFD; a length refusal occurs before `text.result-retain`, and denial of each reached charge point yields the direct runtime stop, charges and consumed counters. |
| FR-035-AC-4 | examined | For each of the six profiles and both `Text[0,8]` and `Text[1,1]`, a real Kani harness symbolically ranges over all 21 sequences of zero to two scalars from the four-scalar alphabet, constructs each `TextPayload` by `from_utf8`, compares the generated admission oracle with a separate direct runtime call including outcome and metering, and ends with one satisfied cover after its assertions. The proof identity names this finite class and its type. |
| FR-035-AC-5 | examined | In a copy of the generated oracle with its admission call or declared `TextType` altered so the oracle accepts a payload the direct runtime call refuses, real Kani falsifies the admission agreement assertion with concrete playback; restoring the call verifies over the same symbolic class. |
| FR-035-AC-6 | examined | A request outside the declared finite proof class has a typed unsupported or requires-bound disposition and no harness, and no caller-ingress claim is marked `ir_confirmed` or assigned a checked-package node id. The six attempted text-operand `numeric.convert` nodes remain Contract IR refusals and do not count as passing caller-admission coverage. |
| TC-050 | examined | Generate twelve real-Kani harnesses: every profile crossed with both `Text[0,8]` and `Text[1,1]`. |
| ADR-006 | examined | The caller Text-admission oracle and its proof form a separate, explicitly caller-declared ingress route under FR-035. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

No additional findings in this method.
