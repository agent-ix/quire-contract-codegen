---
id: SR-17940486709564847047721
title: "IR-1512 PR #801 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #801 at 4ca035e6 (base main cd660b50); IR-1512; src/replay/state_clause.rs, src/replay/state_clause/invariant.rs, tests/it/state_clause_invariant_replay.rs, tests/it/terminal_map.rs, tests/state_frame_support/native_twin.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
---

## Summary

Ticket: IR-1512. The PR adds `InvariantReplayInputs::unread_values: Vec<(String, i64)>` and a
`complete_snapshot` step in `StateClauseReplay::new_invariant` that binds every declared field of
the model object: drawn fields from the playback, each unread declared field from the supplied
value. Two new error variants (`UnreadValueForReadField`, `UnreadFieldNotRanged`) map to
`ReplaySettlement::CgDefect`. The transcript and `render_witness` input stay the playback
bindings (`drawn`), not the completed values.

Gates run in an own worktree at 4ca035e6, as one batched job under `flock /tmp/cargo-slot.lock`:
`cargo fmt --check` clean; `cargo clippy --locked --all-targets -- -D warnings` clean;
`cargo test --locked --test it -- state_clause_invariant_replay terminal_map layout` 52 passed
(11 invariant replay, 32 terminal_map, 8 layout incl. L-1/L-2); `cargo test --locked --lib` 540
passed, 7 ignored.

Reviewer mutation controls in `complete_snapshot` (each reverted; tree clean afterwards), run
with `--test it -- state_clause_invariant_replay`:

- M1, iterate `declared.fields()` in reverse (snapshot not in declared order): survives, 11/11.
  Expected: the snapshot object is a name-keyed map and FR-024-AC-69 states that it asserts no
  order. Declared order is not observable on the wire, so no finding.
- M2, check order changed to duplicate, then undeclared, then drawn/read: survives, 11/11
  (precedence among several refusals is not specified; see the spec-review SR).
- M3, lower bound clamped to 0 (`minimum.max(0)..=maximum`): survives, 11/11 (FND-001).
- M4, an empty `unread_values` returns the drawn-only snapshot (the pre-PR behaviour): killed by
  `tc_035_each_unread_value_refusal_is_a_distinct_variant_before_any_document`, at the
  executor-uncalled assertion of `refuse` (line 477).
- M5, lower endpoint exclusive (`minimum + 1`): killed by the same test (endpoint `0` accepted,
  line 512).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | No test supplies an unread field with a negative lower bound; a range check clamping the minimum at 0 (M3) passes every test, so "endpoints accepted, beyond refused" is shown only for `audit: Int[0, 1000]` | tests/it/state_clause_invariant_replay.rs:510-527 |
| FND-002 | low | AC-70's no-i64-range cases include a string field and an unbounded integer; the test covers only a Boolean field and a range wider than i64, though `Twin::with_unbounded_audit_clause` and `emitted_package_with_unread_string_audit` exist | tests/it/state_clause_invariant_replay.rs:555-586 |
| FND-003 | low | The closure parameter `drawn` shadows the outer slice `drawn` in `drawn.iter().any(\|(drawn, _)\| drawn == name)` | src/replay/state_clause/invariant.rs:292 |
| FND-004 | low | `clause_read_fields` parses the clause again with `read_clause` after `ensure_declaring_object` already did, and maps the unreachable `None` to `HarnessMismatch{Object}`; the read names could be returned from the first parse | src/replay/state_clause/invariant.rs:257-269 |

## Verdict

No blocking defect. Checks with no defect found: the executor-uncalled claim is asserted for every
AC-70 refusal by `refuse`, whose `Ok` branch calls `replay_through` with a flagging executor, so a
refusal that reached a built replay fails the test (M4 shows it biting). Range semantics read the
accessor range through `replay_range`, so a field without an i64 range gives
`UnreadFieldNotRanged` and an omitted one gives `MissingField` (both tested for the Boolean and
the wide case). The transcript is built from `drawn`, asserted exactly as
`<<<assertion|deposit|BalanceNonnegative|balance=-1>>>`. All other `InvariantReplayInputs`
constructions go through `Twin::invariant_inputs`, which sets `unread_values: Vec::new()`, and the
existing invariant tests over `with_bank_invariants` (every field read) pass unchanged, which is
the regression. Both new variants map to `CgDefect` and are in the `terminal_map` list.
No `unwrap`, `panic!` or `unsafe` in `src`; no hash, digest or pin change.
