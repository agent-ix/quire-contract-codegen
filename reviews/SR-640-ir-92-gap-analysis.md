---
id: "SR-640"
title: "CG PR 206 gap analysis: FR-016-AC-14 and FR-015-AC-33 against tests and code"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@16828648175edf3190ad799e975a944a5a8ab518; src/frame_replay.rs, src/spine_replay.rs, tests/it/skeleton_spine.rs, tests/it/kani_obligations_state_frame.rs, spec/functional/complete-v1/FR-015-bounded-kani-obligations.md, spec/functional/complete-v1/FR-016-witness-native-replay.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

# SR-640: CG PR 206 gap analysis

## Summary

Ticket: IR-92. PR: agent-ix/quire-contract-codegen#206, head 1682864. This analysis is planless and
scoped to the PR diff: the two new ACs, the tests tagged to them, the production code they claim,
and the code the PR added with no owning AC. Plan completion: not assessed.

## Method

For each new AC I split the statement into its clauses and looked for a test assertion that fails
when that clause is false. I checked `Trace:` tags against the clauses. I listed every new public
item and error variant and looked for its owning AC and a test. I ran mutation probes (in SR-639)
to confirm that the AC-14 tests are not tautological.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-015-AC-33's last clause is "an operation the unit names no frame for is refused". The test uses `withdraw`, which the `test/bank` domain package does not declare at all. So the test exercises the "operation does not resolve" branch of `UnknownOperation`. It does not exercise the case the AC names: a declared operation that no clause of the unit names, for which QSL emits no frame. Both branches return the same variant, so the AC wording and the test disagree only in intent. A declared `Account` operation without a clause would match the AC exactly. | tests/it/kani_obligations_state_frame.rs:829-844 |
| FND-002 | low | `DependencyLockError::Input` is a new public error variant, and interface-001 lists it as an output of `ReplayPackage::new`. No test reaches it, for example two lock selections whose sources share an owner, or a selection with an empty version. Only `Duplicate` is tested. | src/spine_replay.rs:215-219 |

## Verdict

Coverage is honest for both new ACs, with the two low gaps above.

FR-016-AC-14, clause by clause:
- Compiled together with the dependency's lock source: `tc_026_a_unit_importing_a_locked_dependency_replays_to_a_reproduced_verdict`
  compiles a unit that imports `test/units`. The `Import` refusal test proves compilation needs the
  lock source.
- `package.dependencies` has one entry per selection with the lock's `package_id`: `wire.dependencies.len() == 1`
  together with `tc_026_a_lock_recording_another_dependency_identity_is_refused`
  (`DependencyIdentityMismatch`). Entry order and shape are also asserted by
  `tc_026_the_request_package_reference_carries_the_lock_dependencies`.
- `replay` settles through the imported function: x=3 gives reproduced + `Violation`, and x=7 gives
  `Inconclusive`.
- The three refusals (call-site import, identity mismatch, unselected) each have their own test
  and assert the variant.

FR-015-AC-33: the default-suite test asserts `clause_node == payload.frame`,
`occurrence_key == payload.occurrence`, `occurrence.node() == frame` and `anchor != frame`, and that
the forbidden write reproduces. The identities come from `call_site`, and there is no test-side
compile left. QSL's `replay_frame` checks the payload against the recompiled package, so a wrong
anchor or frame would be refused. The coder's probe (pointing `clause_node` at the anchor fails the
test) agrees with this reading. The refusal clause is FND-001.

Code with no owning AC: none. `ProvidedDocument` and `FrameReplayError::{Name, Transcript, Envelope}`
are part of the FR-015-AC-33 builder. State-clause postcondition replay is absent, with no stub,
which matches the PR's stated scope (QSL-336 is not landed).
