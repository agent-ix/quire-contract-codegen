---
id: SR-16035
title: "IR-958 CG #550 gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #550; FR-024-AC-41..45; TC-035 steps 34..36; changed src/replay and tests/it paths"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-024", type: references }
---

## Summary

Independent diff-scoped gap analysis of CG PR #550 at exact head def4d9f87d3908607ec637c455b9e50809948cfc on base b68193997a15fd37ea3d25132c39f83138ed54dc. The changed tests exercise typed replay targets and a selected call site's occurrence, but they do not exercise packet-sourced function or parity occurrences.

## Verdict

FAIL — FR-024-AC-43/44 have no criterion-bound test or packet-to-request evidence, despite the source now setting an occurrence field. No final cohort or aggregate gate result is claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new function request test is tagged AC-41 and compares the emitted key only to `package.site.site.declaration`. It supplies no admitted packet, so replacing an admitted packet key with a different selected-site key would leave the test green. TC-035 step 34 and AC-43 remain unbacked. | src/replay/function.rs:1251-1273 |
| FND-002 | high | The changed parity fixture constructs a request key from `ParityPreimage`, and no changed test supplies an operator or value parity packet or distinguishes two occurrences of one node. TC-035 step 35 and AC-44 remain unbacked, including the nonzero expression ordinal and typed readback case. | tests/it/composite_parity_converter.rs:168-224 |
| FND-003 | medium | No changed test exercises the AC-45 missing-`occurrence_key` request reader with and without byte provision, or asserts refusal before replay. The new missing-target packet test exercises AC-42 instead. Keep AC-45 planned until the QSL reader path is tested from CG's request. | tests/it/kani_obligations_state_clause_replay.rs:564-618 |

## Coverage

FR-024-AC-41 through AC-45 and TC-035 steps 34 through 36 were examined. AC-41 frame/state-clause packet shape and AC-42 target refusal have changed tests; AC-43/44/45 do not have changed criterion tags or the specified scenarios. Matrix computation was attempted; the command did not return a usable scoped result within the review window, so this is a trace-tag and source/test inspection, not a whole-repository matrix verdict. Plan completion: not assessed. Optional repository-wide semantic audit was not requested.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | The misleading AC-41 tag was removed from the site-key test at 0aa469e6, and its doc now states packet provenance is separate. The test still has no admitted packet and does not back AC-43; #550 explicitly excludes IR-968 completion and holds merge for the packet-forwarding slice. |
| FND-002 | deferred | No parity test changed at 0aa469e6. AC-44's own-packet, distinct-occurrence and typed-readback scenarios remain planned under the stated IR-968 exclusion and merge hold. |
| FND-003 | deferred | No AC-45 test changed at 0aa469e6. The PR description explicitly leaves QSL reader missing-key refusal evidence to a separate focused slice and holds merge; the criterion remains unbacked. |
| FND-003 | fixed | At 7daf5f0a the new AC-45-tagged test takes a real function request, confirms its byte provision is present, removes the occurrence key, and calls QSL's typed `ReplayRequest::decode` both with and without byte provision. Both cases assert `MissingOccurrenceKey`, `invalid_runtime_input`, `missing-member`, and `/replay/occurrence_key`. It calls no replay executor and produces no verdict. This is a source/test disposition; execution awaits the final QSL lock. |
