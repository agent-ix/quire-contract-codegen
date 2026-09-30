---
id: "SR-635"
title: "IR-92 slice 2 spec review: interface-001 witness operations and TC-026 schema refusal"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@92ec95afe85b8af23e3123105d9e7120a5b452ff; spec/interface/interface-001-codegen-api.md, spec/test/complete-v1/TC-026-witness-native-replay.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-026
    type: reviews
---

# SR-635: IR-92 slice 2 spec review

## Summary

Ticket: IR-92 (slice 2). PR: agent-ix/quire-contract-codegen#204, head 92ec95a. Method:
spec-review, with the integrity check folded in. `make spec` passed as part of `make ci`.

## Method

I read the interface-001 operation edits (witness_schema removed, decode_falsification
rewritten, the features table) and the TC-026 edit. I compared them with the public exports in
`src/lib.rs`, with the `decode_falsification` signature, and with FR-016-AC-8.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | This PR changed the `WitnessValue` that `replay_falsification` takes from IR's type to qsl-replay's. Its interface-001 entry still says "named WitnessValue list", while `decode_falsification`, two entries above, now says "named qsl-replay WitnessValue list". The two operations chain, so the unqualified name reads as if they took different types. | spec/interface/interface-001-codegen-api.md:150 |
| FND-002 | low | TC-026's procedure still says to "build a harness's witness schema from its persisted obligation arguments ... and compare the schema's order". The schema is no longer a public product, because witness_schema was removed and the schema is now internal to `decode_falsification`. The step can be run only through a private unit test. The procedure could say that the order is observed through the names `decode_falsification` gives each value. | spec/test/complete-v1/TC-026-witness-native-replay.md:19-21 |

## Verdict

Approve with low findings. Removing `witness_schema` from both the operations list and the
features table matches `src/lib.rs`. The new `decode_falsification` output (`DecodeFailure` code,
source_id, context) matches the struct. Dropping `InvalidInput` from TC-026 is correct, because
`DecodeFailure` carries no outcome kind. FR-016-AC-8's text still holds without change: the
schema, the naming by position and the typed non-argument refusal all remain.
