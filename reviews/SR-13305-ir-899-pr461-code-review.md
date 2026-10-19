---
id: SR-13305
title: "IR-899 PR 461 code review (with Rust lane): first-party lock cohort refresh"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@d40e60dbca7cfcf2328462684e483f8ace134e75 vs origin/main merge-base 58f447801773268b0178ef45c44fb0fc68dffbba; Cargo.lock, tests/it/kani_obligations_symbolic_domains.rs"
review_set: subset
---

## Summary

Ticket: IR-899. PR agent-ix/quire-contract-codegen#461. The Rust lane is folded into this file. The diff is `Cargo.lock` (66+/63-) and one fixture line plus a comment in `tests/it/kani_obligations_symbolic_domains.rs`. `Cargo.toml` is unchanged, so every first-party crate still tracks `branch = "main"` and the lock alone carries the move.

Measured by the reviewer at the head, in a fresh worktree with its own target dir:

- `make deny` passes, including `scripts/check_one_copy.awk`: every agent-ix git crate has exactly one `Cargo.lock` entry. `cargo check --locked --all-targets` passes.
- The 20 selected symbolic collection tests pass; the real-Kani test `tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies` stays ignored (kani lane, not run by the author or the reviewer).
- Full `cargo test --locked --no-fail-fast --tests` (with `QSPEC_DIR`, run outside the tool sandbox): 300 unit and 622 integration tests pass. 51 unit and 2 integration tests fail, all in Kani process-namespace execution (`kani::run::execute`, `kani::run::namespace`, `kani_terminal_private`, `kani_scalar_native_private`, and two `kani_obligations` terminal-value tests). Each fails at namespace startup ("bwrap: setting up uid map: Permission denied" or "namespace startup refused: Broken pipe"), and this host sets `kernel.apparmor_restrict_unprivileged_userns=1`. The PR changes no code on that path, so these are environmental and are not attributed to it; the author's all-pass count (351 unit, 624 integration) was not reproduced on this host.
- Every "new" revision in the PR body table matches `Cargo.lock`, and each equals its upstream `main` at review time: quire-contract-ir 9ff7d489, quire-exact c2cd5f56, quire-semantic-value f3dba824, filament-core-data 3173a8f7. QSL a44e86da contains ed2831ea (QSL-903 #1042, 4 behind) and bd29deea (QSL-904 #1043, the planner's updated target, 3 behind). Every "old" revision matches origin/main's lock.
- QSL main has moved past a44e86da (34ec80da and then 1bcc1584 during the review, 6+ commits, all after the PR commit). A reviewer probe (`cargo update -p qsl-replay` to 1bcc1584, lock discarded afterwards) still passes `cargo check --all-targets`, so the move does not strand CG.
- Contract runtime scope limit: a reviewer probe with the runtime at current main 62a02532 fails to compile CG (10 lib errors, 14 lib-test errors: unresolved `quire_contract_runtime::exact::{Integer, Outcome, Refusal, ScalarLimits, IeeeWidth, ...}` re-exports). Keeping the runtime at 3cc88c48 is therefore required for this PR, and the overlap with open PR #397 (IR-17, "consume exact-kernel types from quire-exact") is real.
- filament-core-data resolves at two revisions: the two `rev=3173a8f7` crates and the `tag=semantic-schema-v0.3.0` (6156f9e0) schema crate, which `quire-rs` 8ab0d509 depends on. `quire-rs` itself is `rev`-pinned by filament's `agent-ix-extraction-frontend`, so CG cannot move it. The one-copy gate counts crates, not repositories, so it passes; this is IR-819 as the PR says.
- The fixture changes `profile v = "quire.value.complete/v1"` to `"quire.model.complete/v1"`, with a `//` comment naming the reason. QSL 2350bf83 (#964, "Gate the model constructs on the model profile selection") is in the bumped range and explains the change. The checked-package evidence still supports only `quire.value.complete/v1`, and the package is still `Admitted`, which backs the PR's claim that the emitted package needs only the value feature. All callers of `source_unit_with_body` are `#[trace("FR-015-AC-9x", "TC-025")]` tests.
- No compatibility layer, vendored file, `[patch]`, path dependency, new pin, hash or SHA record is added. No `/// Trace:` lines.

Driver IR-773 fit: quire-driver depends on CG, QSL, IR, quire-exact and quire-semantic-value on `branch = "main"`, and its lock holds quire-contract-runtime at 3cc88c48, the same revision this PR keeps. A targeted driver update to this cohort compiles; see FND-003 for the plain `cargo update` hazard.

## Examined criteria

- IR-899 deliverable (examined): "move CG once to a coherent published Contract IR + QSL + quire-exact lock cohort at current heads (QSL at or after ed2831ea), one revision per dependency, adapt any measured API changes with tagged tests, no compat layer".
- IR-899 planner update (examined): "the single CG lock move lands on QSL at or after bd29deea ... coherent with IR and quire-exact heads, one revision per dependency."
- Cargo.lock first-party sources (examined): one entry per agent-ix crate; filament-core-data at two revisions (IR-819).
- tests/it/kani_obligations_symbolic_domains.rs:168-176 (examined): `// \`deref(input)\` is object navigation, admitted under QSL's model layer.` and `profile v = \"quire.model.complete/v1\";`
- PR body revision table and scope statements (examined).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The PR body says "Contract runtime remains at `3cc88c48` while its current main is `b941966b`". Runtime main was already 62a02532 (IR-882 #135, committed 2026-10-19T17:18Z) before the PR commit (17:53Z). The conclusion still holds, because CG also fails to compile against 62a02532, but the cited head is wrong. Fix: cite 62a02532 (or the head at update time). | PR #461 body |
| FND-002 | low | `Cargo.lock` also moves 19 crates.io packages to new versions, which the PR body does not mention while it calls the change "this confined lock plus fixture PR" and lists only first-party revisions. Examples: cc 1.5.1 to 1.9.0, syn 3.0.6 to 3.0.9, serde/serde_core/serde_derive 1.0.228 to 1.0.229 (serde_derive moves from syn 2 to syn 3), libc 0.2.189 to 0.2.191, uuid 1.26.1 to 1.28.0, zerocopy 0.8.59 to 0.8.63. `make deny` and the reviewer's check and tests pass, so nothing is broken, but the scope statement is inaccurate. Fix: say in the body that third-party crates were refreshed too, or limit the move with `cargo update -p` for the first-party crates. | Cargo.lock:212-2630 |
| FND-003 | low | The merge-order note tells the driver to adopt QSL's bind API after this merge, but not that CG at this head only compiles with quire-contract-runtime 3cc88c48. quire-driver depends on CG on `branch = "main"` and resolves the runtime itself, so a plain `cargo update` in the driver picks runtime main 62a02532 and CG then fails to compile (reviewer probe: 10 lib errors on removed `quire_contract_runtime::exact` re-exports). Fix: in the handoff to draco:qsl-driver:plan and the PR body, say the driver must update selectively (`cargo update -p` for the QSL, IR, quire-exact, quire-semantic-value, filament and CG crates) and keep the runtime at 3cc88c48 until PR #397 lands. | PR #461 body, Merge order |

## Verdict

**PASS: mergeable after the lead's real-Kani run.** The lock move is coherent: one revision per first-party crate, every moved revision is that repository's `main` at review time, QSL is past both ticket targets, and nothing is vendored, patched, shimmed or pinned. The single fixture change is justified by an upstream behaviour change in the bumped range and leaves the test assertions untouched. The three findings are low and are corrections to the PR body or handoff, not to the code. The ticket requires real Kani before merge; the author deferred it to the lead and the reviewer did not run it.

## Dispositions

No disposition pass has run yet. Rows are appended here, one per finding per round.
