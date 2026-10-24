---
id: SR-16281
title: IR-968 CG 552 gap analysis
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; CG #552; FR-024-AC-43, FR-024-AC-44, FR-024-AC-46; src/lib.rs, src/replay/composite_builder.rs, src/replay/function.rs, tests/it/composite_original_eq.rs, tests/it/skeleton_spine.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-024", type: references }
---

## Summary

Plan completion: not assessed. I compared the changed FR-024 packet paths with their trace-tagged tests and the merged FR-024/TC-035 criteria. The tests cover matching packets, several mutated members, and QSL readback, but the authentic second-occurrence substitution required by TC-035 is absent.

## Verdict

**CONDITIONAL** — FR-024-AC-43/44/46 trace credit is broader than the exercised packet-provenance cases. The missing authentic occurrence controls should be added in this PR before the tagged criteria are treated as backed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The tagged function and operator-parity tests mutate a packet made from the same selected claim; they never produce two independently valid QSL-admitted packets for distinct occurrences of one node and swap them while holding the selected claim fixed. The operator test's `u64::MAX` occurrence is a synthetic mismatch, and no valid nonzero `expression` occurrence is retained through request readback. This leaves the distinct-occurrence and producer-substitution controls in FR-024-AC-43/44/46 and TC-035 steps 29/30/32 unproved despite their trace tags. | src/replay/function.rs:1509; src/replay/function.rs:1565; tests/it/composite_original_eq.rs:710; tests/it/composite_original_eq.rs:803 |

## Review evidence

`tc_035_function_request_requires_its_producer_packet` starts with one package-created packet and changes its occurrence ordinal to 1. `tc_035_original_composite_request_requires_its_own_packet` likewise starts from one checked-claim packet and changes its occurrence to `u64::MAX`. Neither uses a second selected checked claim or demonstrates that both occurrences are independently valid for their own claims. The merged TC-035 step 30 asks for an admitted scalar `expression` occurrence with nonzero ordinal and two authentic occurrences of one node; step 32 asks for a second otherwise valid packet in the swap control. The code's typed equality checks appear capable of distinguishing them, but the required behavioral evidence is not present. Value-parity implementation is stacked separately in #553 and was outside this diff review.

No reverse gap was found for the new public packet APIs: FR-024-AC-43/44/46 own them. The source contains no new stub, fixture copy from another repository, or trace tag on a non-test function. The computed matrix was attempted on the stacked review head; this branch has the earlier FR-024 spec because #551 is not in its base, so I used the merged current-main FR-024/TC-035 text plus direct trace-tag inspection as the criterion authority. The author's focused test results were not independently rerun. Plan completion: not assessed.

## Examined scope and bindings

```yaml
scope:
  - id: FR-024-AC-43
    path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
    role: examined
    excerpt: "A function-contract adapter supplies a completed `WitnessPacket<NoPayload>` carrying its decoded witness and retained proving-run members. CG admits it through `WitnessEnvelope::reconstruct` before request construction and requires its `Witness` source arm."
  - id: FR-024-AC-44
    path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
    role: examined
    excerpt: "An input-arm parity claim producer supplies a completed `WitnessPacket<NoPayload>` before its request builder. CG admits it through `WitnessEnvelope::reconstruct` and requires its `Input` source arm."
  - id: FR-024-AC-46
    path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
    role: examined
    excerpt: "Before constructing a function-contract, operator-parity or value-parity request, CG compares the admitted packet's `function` target, complete occurrence key, subject `clause_node`, `package_id`, source arm and opaque obligation identity with the independently selected claim from the admitted package and proving-run identity or original checked parity claim, never from that packet."
```

## Same-reviewer disposition at a8fe44c3

**FND-001 resolved for CG #552; final verdict PASS.** The new `tc_035_original_composite_packets_bind_distinct_real_occurrences` compiles a QSL expression with repeated equality, selects one emitted structural-equality node with two actual occurrences, asserts their keys differ and the second ordinal is nonzero, creates a separate packet for each selected occurrence, admits and reads back each matching request, and rejects substitution of the second complete packet for the first selected claim as `PacketMismatch(Occurrence)`. This exercises the authentic occurrence and readback control absent at the reviewed head. I inspected the exact committed delta `f3016e97..a8fe44c3`; `git diff --check` passed. Value-parity implementation remains in separately stacked #553 and receives no credit from this disposition.

The author reports focused skeleton 31/31, composite 19/19, function packet 1/1, and Kani stdout 2/2 plus fmt, diff, and scoped Clippy passes. These are author-reported focused results; final-head aggregate gates remain pending.
