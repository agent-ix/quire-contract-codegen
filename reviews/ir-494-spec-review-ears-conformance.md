---
id: SR-6303
title: IR-494 ears-conformance review
type: SpecReview
analysis: ears-conformance
scope: agent-ix/quire-contract-codegen@d9955f18171edc58c4f3db1f8efbb715277b465f; spec/oracle/functional/FR-035-caller-text-admission.md
review_set: subset
---

## Summary

Ticket: IR-494. FR-035 Description and Behavior were checked for EARS grammar and semantic trigger/response clarity. Quire reported 92/93 documents grammar-clean; its two EARS warnings are on unchanged FR-017 line 174, outside this PR. No changed requirement statement has an EARS defect.

## Reviewed units

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-035 Description | examined | When a caller requests runtime Text admission, the code generator shall emit an oracle and a separately bounded Kani obligation for a caller-owned `TextPayload` and declared `TextType`. |
| FR-035 Behavior | examined | When given a valid `TextPayload` and `TextType`, the emitted oracle shall call `rt::admit_text(&payload, &text_type, meter)` and return its `Outcome<Text>` unchanged. |
| FR-035 Behavior | examined | When generating a Kani obligation, the generator shall draw a symbolic length in `0..=2` and symbolic alphabet indices for both positions, encode the selected scalars as UTF-8, and construct the payload through `TextPayload::from_utf8`. |
| FR-035 Behavior | examined | If the caller requests a proof for a payload class or profile/bound encoding outside that supported class, then the generator shall refuse it with a typed reason and emit no proof artifact. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

No additional findings in this method.
