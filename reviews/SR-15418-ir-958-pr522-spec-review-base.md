---
id: SR-15418
title: "Base spec review of IR-958 function and frame witness selection"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/replay/functional/FR-024-counterexample-envelope-intake.md (FR-024-AC-4, FR-024-AC-41, FR-024-AC-42); spec/replay/matrix/TC-035-counterexample-envelope-intake.md"
review_set: base
---

## Summary

Reviewed CG PR #522 at b08104af5e5c48c04849408cccd99a007c3b1391 against the base checklist, merged QSpec FR-323 selection contract, QSL ADR-017 PF-4, and the QSL `OperationSite` actually consumed by CG. The new function and frame criteria have distinct negative cases and TC-035 steps; Q-6 correctly leaves state-clause allocation open before shared packet-type implementation. One source-of-selection mismatch needs correction. No applicable AssuranceProfile with required review selection was found. FR-024-AC-41 and FR-024-AC-42 have no implementation trace yet, as expected for this spec-first PR; they must gain tagged tests in the code PR before acceptance.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-024 requires CG to fill `frame_operation` using the declaring object and operation “returned for that frame by QSL's typed call site,” and AC-41 says the pair comes from its `OperationSite`. The current `OperationSite` exposes only `anchor`, `frame`, `frame_occurrence`, and `clauses`; it returns neither the owner nor the operation identifier. CG currently holds a typed `OperationName` input and checks it through `call_site` before constructing `FrameOperation`. State a realizable, authoritative source for `{object, operation}` (for example, the validated typed operation selection), including how the declaring owner is obtained for inherited names, or require the QSL site API to return that identity. TC-035 step 6 should verify the chosen source and owner resolution. | FR-024 Description and AC-41; TC-035 step 6; `src/replay/frame.rs:589-607`; QSL `qsl-replay/src/call_site.rs:174-188` | wrong-requirement |

## Verdict

CONDITIONAL. The draft is scoped to function and frame packets and does not silently decide Q-6 for state-clause replay. Correct FND-001 before this spec becomes the implementation contract. The changed FR-024 and TC-035 files validate with Quire; the two new criteria are not yet test-tagged because their implementation has not landed.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | At 359ed408566c841a0dc88b656f94aff9479f5545, FR-024 requires QSL's validated typed call-site result to supply the declaring owner and operation after inherited resolution, and CG refuses packet construction if the pair is unavailable. It records that the current `OperationSite` lacks the pair and QSL must expose it before CG implements this path. TC-035 step 6 compares the packet with that validated result, including an inherited selection whose selected owner differs from its declaring owner, and tests refusal when QSL supplies no pair. |

## Disposition verdict

PASS at 359ed408566c841a0dc88b656f94aff9479f5545. FND-001 is fixed in the spec and TC. The QSL typed call-site extension and Q-6 allocation remain explicit prerequisites to the later implementation; this spec-only PR makes no implementation claim.
