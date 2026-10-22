---
id: SR-15428
title: "integrity review of IR-968 CG PR 522 occurrence-key amendment"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; spec/replay/functional/FR-024-counterexample-envelope-intake.md; spec/replay/matrix/TC-035-counterexample-envelope-intake.md; IR-968"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-035
    type: references
---
# SR-15428: integrity review of IR-968

## Summary

Reviewed only the IR-968 amendment dd80e6ec..c447bd0f on CG #522 against merged QSpec FR-323-AC-22, QSL FR-071-AC-15 and FR-098-AC-22 through AC-26. Cross-artifact consistency and one-to-one TC-035 evidence mapping. The inherited frame and state-clause request exclusions in FR-024-AC-41 remain intact.

## Scope

- FR-024-AC-43 (examined): A function-contract replay request built from an admitted packet carries exactly that packet's function `declaration` occurrence key, including node, role and ordinal. Its `function` target and `obligation_identity` remain unchanged. CG obtains the key from the admitted packet, without minting or defaulting one or deriving it from `obligation_identity`.
- FR-024-AC-44 (examined): An operator-parity replay request carries its own packet's scalar application occurrence key, and a value-parity replay request carries its own packet's function `declaration` occurrence key. Typed request serialization and readback preserve each complete key, including distinct admitted occurrences of one node and an `expression` occurrence with a nonzero ordinal; neither key is replaced by a node-only identity or another claim's key. The packet's `obligation_identity` remains unchanged.
- FR-024-AC-45 (examined): A `function` replay request missing `occurrence_key` refuses through QSL's request reader as `invalid_runtime_input`/`missing-member` at `/replay/occurrence_key` before byte provision and recompile; the same request with a byte-provision entry also removed reports the same refusal. Neither refusal invokes replay or becomes a contract verdict.
- TC-035-step-29 (examined): From an admitted function-contract counterexample packet, build a `function` replay request using its genuine function `declaration` occurrence. Compare the complete packet and request keys (node, role and ordinal), `function` target and `obligation_identity` before and after typed request serialization and readback. Replay a falsified Boolean predicate through QSL and record its reproduced violation. Inspect construction for a locally minted or default key.
- TC-035-step-30 (examined): Build operator-parity and value-parity requests from their admitted packets and compare each request's complete key with its own packet's key after typed readback. Include an admitted scalar `expression` occurrence with a nonzero ordinal and two authentic occurrences of one node with different ordinals; check that the request retains the chosen occurrence and its `obligation_identity`. The value-parity request uses its packet's function `declaration` occurrence. Recheck that frame and state-clause requests from steps 6 and 10 have neither `arguments` nor `occurrence_key` while their envelope a
- TC-035-step-31 (examined): Remove `occurrence_key` from an otherwise valid serialized `function` request, then also remove one byte-provision entry. Decode both with QSL's public request reader and record the code, path and phase of each refusal; confirm no replay executor is called.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS. No amendment-specific defects found. TC-035 steps 29 through 31 and expected results 34 through 36 directly exercise full packet-key provenance, typed readback, false-predicate replay, parity occurrence distinction, missing-key precedence and the retained frame/state request shape. Implementation status is explicitly planned; this review claims no CG code coverage.
