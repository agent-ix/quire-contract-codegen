---
id: SR-1133
title: "EARS conformance review of quire-contract-codegen PR 247 (IR-553 function-path obligation identity)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen@f04c7a88081180ddf8170e79a9ea7005f55ae953; spec/replay/functional/FR-016-witness-native-replay.md (Behavior, new bullet)"
review_set: subset
---

## Summary

Ticket: IR-553. EARS conformance of the one requirement statement the PR adds to FR-016's
Behavior section. Every other FR-016 Behavior bullet is an EARS "shall" statement; the ACs are
outside EARS scope and are direct assertions as the repository requires.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new FR-016 Behavior bullet is not an EARS statement (no "the generator shall"), and it embeds as-of-base state ("The transcript digest it carries at this base is a placeholder") that belongs in TC-026 Status or AD-002, not in a requirement | spec/replay/functional/FR-016-witness-native-replay.md:114 |

## Verdict

One low wording defect; restate as "The generator shall fill the function path's request
`obligation_identity` with the O-09 function-contract obligation identity" and drop the
base-state sentence.

## Dispositions

Round 1, reviewed at 146b5d6306cf1195b82885c3f2705f116978f01c.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 146b5d6 |
