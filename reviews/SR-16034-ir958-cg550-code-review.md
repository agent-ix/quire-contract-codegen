---
id: SR-16034
title: "IR-958 CG #550 code and Rust review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #550; Cargo.toml; src/replay/{composite_builder,frame,function,obligation,state_clause}.rs; tests/it/{composite_parity_converter,kani_obligations_state_clause_replay,kani_obligations_state_frame,kani_obligations_symbolic_domains,skeleton_spine,terminal_map}.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-024", type: references }
---

## Summary

Independent diff-scoped code and Rust review of CG PR #550 at exact head def4d9f87d3908607ec637c455b9e50809948cfc on base b68193997a15fd37ea3d25132c39f83138ed54dc. The frame and state-clause replay-target adaptations use QSL's typed site fields, and the Category import uses its semantic-value owner. Function and parity request construction do not meet the packet-source rule in FR-024-AC-43/44.

## Verdict

FAIL — two high packet-provenance findings. The draft Cargo.lock and final dependency cohort were not present for compilation or aggregate gate review; no gate verdict is claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Function replay takes its occurrence from `CallSite.site.declaration`, while the request API receives no admitted `WitnessPacket`. A caller with packet occurrence B and a selected call site occurrence A can emit A unchanged; the request cannot prove or enforce AC-43's exact packet provenance. Forward the admitted packet's key at the request boundary and refuse a mismatch; preserve the existing function target and obligation identity. | src/replay/function.rs:657-670 |
| FND-002 | high | Original composite parity takes caller node and occurrence, converts them into `ParityPreimage`, then constructs a new `OccurrenceKey` for the request. No packet enters this path. With two admitted application occurrences of one node, the request can carry the caller's other occurrence, and the required nonzero expression ordinal/own-packet identity is not guaranteed. Carry the packet's scalar-application key for operator parity; value parity needs its own function declaration packet key. | src/replay/composite_builder.rs:255-338 |

## Coverage

Examined FR-024-AC-41 through AC-45 and TC-035 steps 34 through 36 against the 12 changed paths. Frame and state-clause typed target construction under AC-41 and missing/conflicting packet target refusal under AC-42 were checked. AC-45's QSL reader behavior was not changed here. The independent gap-analysis is SR-16035. Plan completion: not assessed. Optional repository-wide semantic audit was not requested.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | The 0aa469e6 draft narrows #550 to IR-958/965 and IR-979, calls the selected-site key provisional, and explicitly holds merge and IR-968 completion for a packet-intake/key-forwarding slice. The request API still has no admitted packet; AC-43 remains unsatisfied. Keeping the provisional key is acceptable only while the branch is a held draft. |
| FND-002 | deferred | The same explicit IR-968 exclusion and hold applies. Composite parity still constructs the request key from caller-supplied node/occurrence via ParityPreimage; AC-44's own-packet key and distinct-occurrence case remain unsatisfied. |
