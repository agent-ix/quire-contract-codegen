---
id: "SR-2280"
title: "CG IR-664 code review (rust-review folded in): i128 consumer adaptation, combined IR and QSL lock move, checked-fixture rekey"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@code/ir-664-i128-consumers (frozen candidate, no PR yet; reviewed revision recorded in the IR-664 Linear marker only, per this repository's no-SHA rule); diff against current main: Cargo.lock, src/kani/abi.rs, src/kani/generate/{clause,outcome,v1_bundle}.rs, src/oracle/{boolean_v1,equality/mod,scalar/mod}.rs, src/replay/{frame,function,state_clause,witness}.rs, src/strategy/bound/generation.rs, tests/checked_package_support/rekey.rs, tests/{composite_equality,exact_function,exact_scalar,state_frame}_support/*, tests/it/{bound_strategy_generation,composite_equality_generation,exact_function_agreement,exact_function_generation,exact_scalar_generation,kani_generation,kani_obligations_state_clause_replay,kani_obligations_state_frame,oracle_arithmetic,oracle_generation,skeleton_spine,terminal_map}.rs"
---

# SR-2280: CG IR-664 code review

## Summary

Ticket: IR-664. Static review only: no cargo build or test was run by the reviewer. The author
runs `make ci` and the real `make kani` lane.

Measured diff against current CG `main`: the branch is 11 commits ahead and 0 behind (its merge
base is `main` itself). It changes 30 files, +1193/-471. The scope the author described
("11 files, +659/-229") is only the tip commit. The other ten commits are the IR integer
adaptation (src/kani, src/oracle/boolean_v1.rs, src/strategy/bound/generation.rs), the QSL
replay API adaptation (src/replay/*, src/oracle/equality/mod.rs test, src/replay/witness.rs),
fixture widening, and the state-clause test's move to QSL-emitted operation shapes.

Checked and clean:

- Lock: exactly one quire-contract-ir revision (two crates, both the merged IR-680 revision on IR
  `main`) and one quire-spec-language revision (all eight qsl-* crates, a revision on QSL `main`).
  `Cargo.toml` is unchanged and keeps `branch = "main"`. The only other lock change is
  quire-contract-model's own new dependency edges (num-bigint, num-integer, num-traits,
  thiserror), all already in the lock at one version. `scripts/check_one_copy.awk` passes. The
  QSL move is forced, not drift: qsl-package and qsl-replay depend on quire-contract-model, so one
  IR revision in the graph needs a QSL revision built against the widened IR.
- i128 adaptation (src/kani/abi.rs:52, clause.rs:260, v1_bundle.rs:411/434, boolean_v1.rs:587/598,
  generation.rs:341/372): every narrowing is `i64::try_from` and fails closed with a typed
  refusal. There is no `as` cast, no saturation and no panic. `integer_literal_source` handles
  `i64::MIN` correctly over i128.
- rekey.rs: termination is bounded by `2 * nodes` attempts, a byte-progress set and 1 GiB/256M
  work caps, all checked before each read. Substitution touches only objects whose `domain` is
  the checked-node domain and whose digest matches exactly. Identity projection and package id
  are rebuilt from the substituted wire on every pass (the same algorithm `wire()` used on main).
  Only `StaleNodeKey` with both a locus and an `expected_node_id` is repaired. Every other
  refusal, including a stale key with no expected key, is returned unchanged. Dependencies are
  re-sorted as a `Vec`, which keeps duplicates. BTreeMap/BTreeSet make it deterministic. The
  first attempt reproduces main's `wire()` bytes exactly. Panics are confined to test-fixture
  invariants. It adds no FR-092 key algorithm and no global registry: resolution is per artifact
  through `FixtureIds`.
- Tests: no assertion was deleted or weakened. Every changed assertion compares against the
  admitted key through `FixtureIds`/`FixtureOracles`. kani_obligations_state_frame.rs loses only
  the `fixture_declaring` helper. Its one caller (TC-035 shape test) now uses QSL-emitted
  packages with a declared parameter or result, and asserts the same
  `UnsupportedOperationShape`. The AC6 IEEE negative control still uses its own unregistered
  environment key. terminal_map.rs adds the new `UnsupportedWitnessValue` row. skeleton_spine
  adds limit-forwarding and declared-domain forwarding assertions.
- Declared non-application fixture nodes get a `SourceOwner`. Relationship nodes get a model
  owner backed by registered model evidence.
- No conflict markers, local paths or new SHAs outside Cargo.lock. Heavy tests keep their
  existing `#[ignore = "kani lane"]` gates. The new tests are light.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The branch goes beyond the described scope and beyond the i128 adaptation. It is 11 commits, 30 files, +1193/-471, not "11 files, +659/-229". It carries the QSL replay API adaptation that ticket IR-666 owns (items 1 and 2: re-pin QSL and pass the trailing `ReplayLimits`), and IR-666 says "Do NOT fold IR-664 in". It also changes public production API. `replay_falsification` gains a `ReplayLimits` parameter. `ReplayInputs.stage_limits` changes from `StageLimits` to `BTreeMap<String, u64>`, and the struct gains pub `declared_domains` and `replay_limits`. `SpineReplayError` gains `UnsupportedWitnessValue`. `StateClauseReplay`, whose fields were all pub, gains a private field, so callers outside the crate can no longer build or destructure it, and gets a `replay_limits()` accessor. Function witness entries are now sorted by node id. The coupling is forced by the lock graph, so this is not a code defect, but it is undeclared. Failure scenario: an IR-666 coder repeats or conflicts with this work, and the driver lane, which pins CG by revision, breaks on the `ReplayInputs`/`StateClauseReplay` literals with no PR text telling it why. Fix: the PR body lists every API change above, and IR-666 is re-scoped in Linear (items 1 and 2 delivered here; items 3 and 4 remain) | src/replay/function.rs:117, src/replay/function.rs:276-291, src/replay/function.rs:55, src/replay/state_clause.rs:356-370, src/replay/function.rs:197 |
| FND-002 | low | The comment in `decoded_state` still says the Boolean arm "keeps this match free of a wildcard", but the same commit adds a `_ =>` arm (frame.rs:378). function.rs:188 adds a matching wildcard. `WitnessValue` is an exhaustive enum in qsl-replay, so a wildcard silently routes any variant QSL adds later into the refusal, with no compile-time prompt to decide. Name the refused variants, or at least correct the stale comment | src/replay/frame.rs:366-384, src/replay/function.rs:185-193 |
| FND-003 | low | After the rekey, builder doc comments are stale. `add_dependency` says identity "is fixed before this method ever runs", and the composite/exact-scalar builders still derive a builder-side application digest that the reader replaces through `StaleNodeKey`. Readers are told the builder's digest is the identity when `FixtureIds` may now map it | tests/exact_scalar_support/package.rs:995-1003, tests/composite_equality_support/package.rs:633-645 |

## Verdict

The code changes are correct and fail closed. The rekey helper is sound: it is bounded,
deterministic, repairs only exact typed keys that the reader names, and does not mask negative
fixtures. The lock satisfies one-IR/one-QSL, and its diff is pins plus IR's own new edges. Not
merge-ready as presented, because the candidate's real scope (FND-001) is undeclared and overlaps
IR-666. Declaring it and re-scoping IR-666 is a ticket and PR-text fix, not a code change.
FND-002 and FND-003 are cheap cleanups.

## Dispositions

Round 1, reviewed at the branch's second frozen head (fix commit caeb25e; 12 commits, 40 files,
+1488/-495 against current `main`; revision recorded in the IR-664 Linear marker). Static
re-check: no build was run. Since the round-0 head, the lock and `rekey.rs` are byte-unchanged.
The lock still holds one IR revision and one QSL revision. No test was deleted. No production
behaviour beyond the i128/QSL adaptation was added: the Boolean-oracle literal guard was removed,
but IR's `check_kind` refuses any literal outside its named type, and `TypedExpression` is built
only by `check_expression`, so that guard was unreachable. No IR-666 terminal-map or F-row code
is present. The AC3/AC15 origin-order fix sorts by the reader-resolved id, which is the order the
generator uses.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | deferred | The PR body draft now declares every public API change this finding named, the forced QSL coupling, IR-666 items 1-2 being delivered here, and the driver impact (verified against the draft text). Re-scoping the IR-666 ticket is a planner action on IR-666, whose description still lists those items. That must happen before IR-666 is picked up; it is not a change to this branch |
| FND-002 | fixed | caeb25e |
| FND-003 | fixed | caeb25e |
