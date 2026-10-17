---
id: SR-12025
title: "IR-835 code review (rust lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@f48e60ed913bc7f17f5d010219ff2784479547c3; Cargo.lock, tests/it/kani_obligations.rs, tests/it/kani_obligations_state_frame.rs, src/kani/generate/frame.rs (read as context)"
review_set: subset
---

## Summary

Ticket: IR-835, PR quire-contract-codegen#411, head f48e60e, base origin/main 9446ca9. The rust-review lane is folded into this file. The diff is a Cargo.lock bump and two test fixtures. The lock moves `quire-contract-ir` and `quire-contract-model` from 38f3581 to 299a126, the eight `qsl-*` crates from 23f9f34 to 2a316c0, and `quire-semantic-value` from ad6e975 to 7ad17ad. The fixtures give hand-authored `state`/`frame` nodes a `deletes_scoped: []` member and a field-entry `scope`. No `src/` file changes.

The bump makes the IR reader admit a `deletes_scoped` member that it refused before IR-816. CG's frame-harness generator never reads that member.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| Cargo.lock | examined | 11 `source` lines change: qsl-attrs, qsl-cst, qsl-eval, qsl-forms, qsl-foundation, qsl-package, qsl-replay, qsl-semantics 23f9f34 -> 2a316c0; quire-contract-ir, quire-contract-model 38f3581 -> 299a126; quire-semantic-value ad6e975 -> 7ad17ad |
| tests/it/kani_obligations.rs:530 | examined | `json!({"term": "frame", "modifies": [], "creates": [], "deletes": [], "deletes_scoped": []})` |
| tests/it/kani_obligations_state_frame.rs:386-414 | examined | `field_entry(&object, "receiver", name)`; `field_entry(&other_object, "every-object", "balance")`; `"deletes_scoped": []` |
| src/kani/generate/frame.rs:726-771 | context_only | `frame_grants` refuses non-empty `creates`/`deletes`, relationship and foreign-field grants; reads `kind`, `declaration`, `name` only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `frame_grants` never reads `deletes_scoped` (or `scope`). IR 299a126 now admits a frame with `deletes_scoped: [{"kind": "receiver"}]` (`deletes self`) or `parameter` entries, and QSL main emits them (TC-1043 vectors W1, W2, W6). CG then generates a supported frame harness for an operation that may delete its receiver. FR-015-AC-29 requires a frame that deletes to be refused with a typed reason and no harness. Reproduced by mutation: with `deletes_scoped: [{"kind": "receiver"}]` on every shape's frame, all 59 `kani_obligations_state_frame` tests still pass, including the supported-harness tests. Before this bump, IR refused such a frame body as an unknown member, so CG never saw one. The bump makes the path reachable. | src/kani/generate/frame.rs:734-745, Cargo.lock:1451 |

## Verdict

Changes requested: FND-001.

Lock (verified):

- `git diff origin/main...HEAD -- Cargo.lock` touches only the 11 `source` lines listed above. No dependency lists, versions or checksums change.
- `cargo metadata` and `cargo tree --locked` resolve.
- `cargo tree -d` lists no duplicated agent-ix crate. There is one quire-contract-ir source (299a126), and quire-contract-model shares it.
- `make deny` passes: advisories, bans, licenses and sources ok, and `scripts/check_one_copy.awk` passes.
- Each new rev is an ancestor of its repo's `main`. IR 299a126 contains IR-816 (6b80801). QSL 2a316c0's own lock carries quire-contract-model at 6b80801.
- quire-semantic-value ad6e975..7ad17ad is e622ca7 (DuplicateClause, a code change) plus 7ad17ad (spec only).
- The revs resolve through cargo's git fetch, so no hash was edited by hand.

Two filament-core-data sources are still in Cargo.lock, unchanged by this PR:

- `agent-ix-extraction-frontend` 0.2.1 and `agent-ix-semantic-ir` 0.1.1 come from `?rev=fd261ca1549f79cd8512312ab759895540e9a159`.
- `agent-ix-semantic-schema` 0.3.0 comes from `?tag=semantic-schema-v0.3.0#6156f9e0e891f7d508ce7b137ba1f6eb849a099e`, reached through `quire-rs` 0.50.1 (rev 8ab0d50) <- agent-ix-extraction-frontend <- qsl-semantics.

check_one_copy passes because it counts entries per crate, and these are three different crates. IR-819 tracks this, waiting on quire-rs. It is outside this PR's scope and not a finding here.

Fixtures (read against FR-040 and QSpec FR-340):

- `kani_obligations.rs:530`, a frame with four empty members, matches FR-040-AC-17 ("a frame of four empty members admits").
- In `kani_obligations_state_frame.rs`, own fields at `receiver` and the foreign `OTHER_OBJECT.balance` at `every-object` are both admitted shapes. `every-object` is the only meaningful scope for another type's field. `receiver` for the receiver's own fields matches the single-receiver frame harness.
- The `Deletes` shape keeps a type-wide `deletes: [object]` with `deletes_scoped: []`, which FR-340 says is type-wide.
- `cargo fmt --check` passes.
- The rust lane has nothing further: there is no new panic, unsafe, conversion or lock surface.

Tests:

- `cargo test -j2 --locked --no-fail-fast`, unsandboxed, `QSPEC_DIR` = qspec-gates at 2fb2473.
  - `it`: 594 passed, 0 failed, 34 ignored. This includes the ticket's tc_031_ac33 (4) and tc_024_ac44 (2) tests.
  - doctest: 1 passed.
  - lib: 246 passed, 32 failed.
- The 32 lib failures are environmental (`apparmor_restrict_unprivileged_userns=1`, `MemoryMechanismUnavailable` / namespace handshake): 29 `kani::run::execute::batch_tests::*`, `tc_027_concurrent_runs_in_one_target_directory_keep_their_own_reports`, `tc_027_execution_reads_only_the_report_its_own_run_exported`, and `kani::run::namespace::tests::completed_monitor_cleanup_kills_an_orphan_and_its_fork_after_the_last_sample`. They are not evidence either way. No Kani execution result is claimed.
