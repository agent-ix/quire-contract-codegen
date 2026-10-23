---
id: SR-16038
title: 'evidence review of IR-968 CG PR #551'
type: SpecReview
analysis: evidence
scope: agent-ix/quire-contract-codegen; spec/replay/functional/FR-024-counterexample-envelope-intake.md;
  spec/replay/matrix/TC-035-counterexample-envelope-intake.md; IR-968
review_set: subset
---

## Summary

Reviewed CG PR #551 exact head 2ece78aa for IR-968 using the evidence lens. 1 finding(s).

## Examined scope

- FR-024-AC-43 (examined): A function-contract adapter supplies a completed `WitnessPacket<NoPayload>` carrying its decoded witness and retained proving-run members. CG admits it through `WitnessEnvelope::reconstruct` before request construction. The request builder accepts the admitted envelope and request-only members, and copies exactly its function `declaration` occurrence key, `function` target and opaque `obligation_identity`; it accepts no bare key or selected-site substitute. Removing the producer packet, or replacing its key with a different admitted function occurrence while leaving the selected claim fixed, p
- FR-024-AC-44 (examined): An input-arm parity claim producer supplies a completed `WitnessPacket<NoPayload>` before its request builder. CG admits it through `WitnessEnvelope::reconstruct`. The operator-parity request copies that packet's checked scalar application occurrence key, and the value-parity request copies its function `declaration` occurrence key, target and opaque obligation identity. Typed request serialization and readback preserve each full key, including two admitted occurrences of one node and an `expression` occurrence with a nonzero ordinal. No request builder mints a key from `ParityPreimage` or tak
- FR-024-AC-46 (examined): Before constructing a function-contract, operator-parity or value-parity request, CG compares the admitted packet's `function` target, complete occurrence key and opaque obligation identity with the independently selected claim from the admitted package and proving-run identity or original checked parity claim, never from that packet. A wrong target, role, ordinal, node or obligation identity returns a typed claim-binding refusal naming the mismatched member, with no request or replay. A different valid occurrence of one node is accepted only for its own selected claim; the other claim's packe
- FR-024 (examined): The generator shall fill every common required `WitnessPacket` member from the proving run's envelope members and the obligation identity, and shall invent none.
- TC-035 (examined): 32. For each of function contract, operator parity and value parity, hold an independently
    selected claim fixed and replace one producer packet member at a time: `function` target,
    occurrence node, role, ordinal or opaque obligation identity. Use a second otherwise valid
    packet when testing the different ordinal of the same node. Record the typed claim-binding
    refusal and confirm no request builder or replay executor runs. Remove the packet occurrence
    key and check QSL packet admission refuses without a CG default. Check that a packet
    assembled inside `request_for` from

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Steps 29-32 mutate target, occurrence and obligation but do not swap source arm or package identity. An implementation that accepts an Input packet on the backend function path, or another package packet with the same selected claim, would pass these controls. | TC-035; spec/replay/matrix/TC-035-counterexample-envelope-intake.md:125-151 |

## Verdict

Findings require a same-PR spec correction before merge.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Exact head 1ceb2a21 now states the refusal and TC-035 exercises the negative case. |

### Fixed evidence

- FND-001 after excerpt: 32. For each of function contract, operator parity and value parity, hold an independently
    selected claim fixed and replace one producer packet member at a time: `function` target,
    occurrence node, role, ordinal, subject `clause_node`, `package_id`, source arm or opaque
    obligation identity. Use a second otherwise valid
    packet when testing the different ordinal of the same node. Record the typed claim-binding
    refusal and confirm no request builder or replay executor runs. Remove the packet occurrence
    key and check QSL packet admission refuses without a CG default. Check 
