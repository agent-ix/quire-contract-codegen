---
id: SR-7026
title: "IR-295 PR351 failure-domain review"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen@a84869ca87712c57309c2d2fea4ee13deb0bd306; spec/routed/functional/FR-019-capability-settlement.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/routed/matrix/tests.md; IR-295"
review_set: subset
---

## Summary

Checked refusal paths around form admission, typed selectors, obligation roles, artifact emission and distinction from out-of-V1 refusal. A V2 StateFrame frame-role failure path is in normative scope but absent from the status register.

## Verdict

**CONDITIONAL — one in-scope V2 refusal path is unspecified in the complete Planned map.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | A valid planned StateFrame frame-role request whose frame grants every field has a source-defined typed refusal but no TC-030 planned selector/cause test. | spec/routed/matrix/TC-030-capability-settlement.md:201-202; spec/kani/functional/FR-015-bounded-kani-obligations.md:634,639; src/kani/generate/frame.rs:191-194 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 5b7e3505d0f5ba359033beaff23f1f3d7ed2d00f |

FND-001 after_excerpt:

```text
V2 `StateFrame` frame role for a `postcondition` whose frame grants every state field | Planned (IR-269) | All-fields-granted frame form; FR-015-AC-63 and AC-68 name this refusal shape. | `StateFrameRefused { NothingForbidden }` (`generate_state_frame_role`, FR-015-AC-63 and AC-68).
```
