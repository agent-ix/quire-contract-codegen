---
id: "SR-639"
title: "CG PR 206 code review: QSL-337 call_site dependencies and frame replay in src"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@16828648175edf3190ad799e975a944a5a8ab518; Cargo.lock, src/frame_replay.rs, src/spine_replay.rs, src/lib.rs, src/kani_witness_join.rs, tests/it/skeleton_spine.rs, tests/it/kani_obligations_state_frame.rs, tests/state_frame_support/native_twin.rs"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
---

# SR-639: CG PR 206 code review

## Summary

Ticket: IR-92 (slice 3; IR-290 end to end, QSL-337 adoption). PR:
agent-ix/quire-contract-codegen#206, head 1682864, base main 8fcb51f. This is code-review with the
rust-review lane folded in, scoped to `git diff origin/main...HEAD`.

## Method

I read the whole diff and the touched modules at the head. I read QSL's `qsl-replay/src/call_site.rs`
and `DependencyInput::new` in `qsl-replay/src/spine.rs` at QSL origin/main 7d0497be to check what
CG hands QSL and which refusals arrive where. I diffed `Cargo.lock`. I grepped `src/`, `tests/` and
`spec/` for every user of the removed `DependencyLock.sources`, `DuplicateDependency` and
`qsl_replay::spine`. I ran `make ci` at the head with a private scratchpad TRUSTED_HOME, and ran
mutation probes on the dependency wire and the dependency input.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The only default-suite test of `FrameReplay::replay` asserts the settlement and nothing else. It does not check `category() == Violation` or that `found()` names the `account`/`audit` field write, which the Kani-lane test does at lines 1027-1033. The granted leg of the same test goes through `Twin::replay`, which calls `qsl_replay::replay_frame` directly and not `FrameReplay::replay`. | tests/it/kani_obligations_state_frame.rs:806-821 |
| FND-002 | low | `FrameReplayInputs.run` is a whole `ReplayInputs`, and its `function` field is silently ignored. The field doc says so, but a caller can pass any function name, including an invalid identifier, and get no error. `ReplayPackage::new` rejects the same value. The lock fields could be split from the function selection, or the frame path could reject a function that does not match the operation. | src/frame_replay.rs:34-36, src/frame_replay.rs:115 |
| FND-003 | low | The `DependencyLockError::Input` doc says the variant covers "a library and the unit" sharing a source owner. `DependencyInput::new` never sees the unit. That case reaches the caller as `ReplayPackageError::CallSite(CallSiteRefusal::DependencyInput)` from `call_site`'s `check_unit_owner`. | src/spine_replay.rs:215-218 |
| FND-004 | low | `mod frame_replay` is tagged `// Implements: FR-016`. The requirement it implements is FR-015-AC-33, and interface-001 lists `FrameReplay::new` under FR-015. | src/lib.rs:33 |

## Verdict

Mergeable. All four findings are low, and none is a wrong result.

What is right:
- `Cargo.lock` changes only the 9 QSL git crates (qsl-attrs, qsl-cst, qsl-eval, qsl-forms,
  qsl-foundation, qsl-package, qsl-replay, qsl-semantics, quire-exact), each from 628d3785 to
  7d0497be. Nothing else moves. `qsl-replay` is still the only QSL dependency in Cargo.toml.
  No test or source file reaches `qsl_replay::spine`.
- `ReplayInputs::admit` sorts the selections, refuses a repeated identity, and builds one
  `SuppliedLibrary` per lock source. `ReplayPackage::new` and `FrameReplay::new` pass that input to
  `call_site`, and `ReplayInputs::wire` fills `package.dependencies` from the same sorted lock with
  the lock's recorded `package_id` and its single source. The two calls share one admitted lock, so
  the compiled and requested dependency sets cannot drift apart.
- The imported-dependency replay is a real oracle. `q(x) = u::big(x)` reproduces the violation
  at x=3 (with category `Violation` asserted) and settles `Inconclusive` at x=7. A dependency that
  was assumed rather than evaluated would give the same settlement at both points.
  Mutation probes are listed below.
- `FrameReplay::new` takes the payload's anchor, frame and frame occurrence from `call_site`'s
  `OperationSite`. It sets `clause_node = payload.frame` and `occurrence_key = payload.occurrence`.
  No identity is supplied by the test. The test-only `spine::compile` path is gone, and the
  envelope-disagrees test now tampers with the packet `FrameReplay` built.
- `FrameReplay::replay(self)` consumes the value, which is correct because `replay_frame` takes the
  wire by value and `ReplayRequestWire` is not `Clone`.
- No new `unwrap`, `expect`, `panic`, `as` cast or indexing in `src/`. `pair[0]`/`pair[1]` index a
  `windows(2)` slice. There is no compatibility shim for the old `call_site` signature and no new
  pin or digest record. State-clause replay is not touched.
- `DependencyLock.sources` has no other user. The two remaining `"sources"` strings in `tests/`
  are checked-package JSON fixtures, not this type.

Mutation probes at 1682864 were scratch edits in the review worktree, reverted after. See the
Mutations section.

## Mutations

Each probe was run with `cargo test --locked --test it skeleton_spine` and then reverted.

- M1: `ReplayInputs::wire` emits `dependencies: Vec::new()` (the dependency is dropped from the
  wire). Killed by 4 tests: the imported-dependency replay, the identity mismatch, the unselected
  refusal and the request-shape test.
- M2: each dependency entry carries the unit's source instead of the dependency's (wrong
  dependency source bytes). Killed by the same 4 tests.
- M3: `admit` hands `call_site` `DependencyInput::default()` (an empty dependency input). Killed by
  the imported-dependency replay and the identity-mismatch test.

The coder's own probe (pointing the envelope's `clause_node` at the anchor fails the frame request
test) is consistent with the test text. I did not re-run it.

## Gate

`make ci` at 1682864, with a private scratchpad TRUSTED_HOME, exited 0. fmt-check, spec, clippy
`-D warnings`, the MSRV test, deny, audit-unsafe, rustdoc and test all passed. Each test run gave
94 unit and 235 integration tests passed, with 8 ignored (the Kani lane).

On the Kani lane (MSRV toolchain, `target-codex-backends`, under the host lock), 3 ignored tests
passed: `tc_025_real_kani_frame_counterexamples_replay_natively_through_qsl`,
`tc_026_one_boolean_clause_goes_from_a_bound_package_through_kani_to_native_replay` and
`tc_026_real_falsification_decodes_against_the_persisted_schema_and_mutation_refuses`.
I did not re-run the full 8-test `make kani`.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | The fix round rewrapped the `ReplayPackage::new` doc comment by hand and left one line at 166 characters. The rest of the item is wrapped at the 100-character width `rustfmt.toml` sets for this module. rustfmt does not wrap comments, so `make fmt-check` passes. This is cosmetic only. | src/spine_replay.rs:381 |

## Dispositions

Round 1, reviewed at e0fc6407f5812faa593af53581f92f265b491f29 (fix commit e0fc640). CG main is still
8fcb51f, so the PR does not need a rebase.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | e0fc640 |
| FND-002 | fixed | e0fc640 |
| FND-003 | fixed | e0fc640 |
| FND-004 | fixed | e0fc640 |
| FND-005 | still-open | New this round. The doc line at src/spine_replay.rs:381 is 166 characters and needs rewrapping. Low and cosmetic, so it does not block merge. |

Evidence for this round:
- **FND-001.** The settle test is now `tc_025_frame_replay_settles_a_forbidden_and_a_granted_write`.
  Both legs go through `FrameReplay::replay`. The forbidden leg asserts `category() == Violation`
  and that `found()` names the field write `account`/`audit`. The granted leg asserts
  `Inconclusive`, `DisagreementCause::Verdicts{Violation, Success}` and `found().is_none()`.
- **FND-002.** `ReplayInputs` has no `function` field any more, and `ReplayPackage::new` takes
  `(inputs, function: &str)`. interface-001 matches the new signature. Nothing reads a function
  name on the frame path.
- **FND-003.** The `Input` doc now names only libraries that share an owner, and it points the
  case of a library sharing the unit's owner to `ReplayPackageError::CallSite`.
- **FND-004.** The tag on `mod frame_replay` now reads `Implements: FR-015-AC-33`.

Mutation probes at e0fc640. Each was run with `cargo test --locked --test it <filter>` and then
reverted.
- P1: `admit` without the sort. Killed by the FR-016-AC-15 request-shape test.
- P2: the dependency entry carries the unit's `package_id`. Killed by the AC-14 replay test and
  the AC-15 test.
- P4: `clause_node = payload.anchor`. Killed by the AC-33/34 envelope test, the AC-36 settle test
  and the envelope-disagrees test.
- P5: `DependencyInput::new(..).unwrap_or_default()`. Killed by the new FR-016-AC-19 test.

Gates at e0fc640:
- `make ci` with a private scratchpad TRUSTED_HOME exited 0. Each test run gave 94 unit and 238
  integration tests passed, with 8 ignored.
- Kani lane: `tc_025_real_kani_frame_counterexamples_replay_natively_through_qsl` and
  `tc_026_one_boolean_clause_goes_from_a_bound_package_through_kani_to_native_replay` both passed.
