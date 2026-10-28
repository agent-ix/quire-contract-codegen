---
id: SR-619010
title: "IR-1086 frame replay acceptance gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen#622; IR-1086; FR-024-AC-30; TC-035; src/replay/frame.rs; tests/it/kani_obligations_state_frame.rs; tests/state_frame_support/native_twin.rs"
review_set: subset
---

## Summary

Scoped matrix, tests, and implementation review of CG #622 at 5bd6d62df149057dac0a3ef99a048deaf80aa018. The computed matrix tags FR-024-AC-30 to the new bound-recheck test, the node-identity test, and one ignored installed-Kani test.

## Examined units

- FR-024-AC-30 (examined): FrameReplay::new recomputes the component-wise minimum of requested and proving-run bounds even for a Playback parsed under wider limits; the installed backend and QSL-emitted harness obligations remain planned under IR-624.
- TC-035 (examined): Parse complete retained playback once and submit a parsed playback over either effective bound.
- tests/it/kani_obligations_state_frame.rs:2075-2126 (examined): The new test narrows requested bytes and values after parsing wider.
- tests/state_frame_support/native_twin.rs:980-1034 (examined): The helper fixes run.decode_limits at 1 MiB / 1024 values and caps parsing by those limits.
- src/replay/frame.rs:572-583 and src/replay/witness.rs:231-280 (examined): Production checks both bounds and harness identity before decoding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new test cannot pass Playback parsed above the proving-run bound because its helper caps parsing by that bound; deleting both run.decode_limits minima from FrameReplay::new would still pass the test, leaving that half of AC-30 unverified. | tests/state_frame_support/native_twin.rs:1001; tests/it/kani_obligations_state_frame.rs:2075 |

## Verdict

CONDITIONAL until a direct test sets the proving-run bound below parse limits and verifies FrameReplay::new refuses the already-parsed Playback. The implementation itself performs this check. FR-024-AC-30 as a whole remains planned under IR-624: the installed Kani case is ignored and the unchanged node-identity case currently fails. The three wider module failures occur before the changed replay boundary.

## Coverage

Plan completion: not assessed. Scoped PR diff only. Quoin matrix computed; semantic review of the changed acceptance behavior was included in this assigned review.
