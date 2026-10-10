---
id: SR-6307
title: IR-494 failure-domain review
type: SpecReview
analysis: failure-domain
scope: agent-ix/quire-contract-codegen@d9955f18171edc58c4f3db1f8efbb715277b465f; spec/oracle/functional/FR-035-caller-text-admission.md, spec/oracle/matrix/TC-050-caller-text-admission.md
review_set: subset
---

## Summary

Ticket: IR-494. The changed contract covers invalid UTF-8 before payload construction, profile-length and metering refusal, unsupported symbolic classes, mutation falsification, and non-vacuous proof for all 21 values. There is no callback, user-supplied executable logic, or graph walk in this diff. The Kani execution/refusal ownership gap is recorded in SR-6301.

## Reviewed units

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-035-AC-2 | examined | Invalid UTF-8 bytes make `TextPayload::from_utf8` return `InvalidUtf8` before admission and leave the admission meter unchanged; `admit_text` itself is never claimed to accept those bytes. |
| FR-035-AC-3 | examined | For each profile, `Text[1,1]` admits or refuses according to the profile length of the retained sequence, including `e` plus U+0301 under NFC and NFD; a length refusal occurs before `text.result-retain`, and denial of each reached charge point yields the direct runtime stop, charges and consumed counters. |
| FR-035-AC-4 | examined | For each of the six profiles and both `Text[0,8]` and `Text[1,1]`, a real Kani harness symbolically ranges over all 21 sequences of zero to two scalars from the four-scalar alphabet, constructs each `TextPayload` by `from_utf8`, compares the generated admission oracle with a separate direct runtime call including outcome and metering, and ends with one satisfied cover after its assertions. The proof identity names this finite class and its type. |
| FR-035-AC-5 | examined | In a copy of the generated oracle with its admission call or declared `TextType` altered so the oracle accepts a payload the direct runtime call refuses, real Kani falsifies the admission agreement assertion with concrete playback; restoring the call verifies over the same symbolic class. |
| FR-035-AC-6 | examined | A request outside the declared finite proof class has a typed unsupported or requires-bound disposition and no harness, and no caller-ingress claim is marked `ir_confirmed` or assigned a checked-package node id. The six attempted text-operand `numeric.convert` nodes remain Contract IR refusals and do not count as passing caller-admission coverage. |
| TC-050 | examined | Request an unbounded or unsupported payload class. Check its typed disposition, absent harness and absence of `ir_confirmed` or checked node identity. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

No additional findings in this method.
