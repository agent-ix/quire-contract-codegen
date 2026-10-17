---
id: "SR-11995"
title: "IR-808 code review: kani_gate stand-in tests run in their own sandbox"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@3254cf45489e87955a3a375fa660bf3623f47a9d; tests/it/kani_gate.rs, scripts/kani_gate.sh"
---

# SR-11995: IR-808 code review

## Summary

Ticket: IR-808. PR: agent-ix/quire-contract-codegen#408 (draft), head 3254cf4, base main
494937f. Scope is `git diff origin/main...origin/peter/ir-808-kani-gate-report-race`: one line of
`scripts/kani_gate.sh` and the sandbox wiring in `tests/it/kani_gate.rs`. Method: code-review with
the rust-review lane and a shell review of the script folded in.

## Method

I read the whole script, `scripts/kani_scope.sh`, the Makefile `kani-gate` target and the test
module, and traced each cwd-relative path the gate uses. Then I ran the `kani_gate::` module 8
times on main and 8 times on the PR head, and ran the full `--test it` binary once on the PR
head. After each set of runs I checked the checkout for `target-codex-backends/` and for git
status changes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The gate now finds `kani_scope.sh` from `$0`, but it still resolves `git`, the claim evidence directory and the cargo test command from cwd. If the gate is called through a symlink (for example `~/bin/kani-gate -> scripts/kani_gate.sh`), `dirname "$0"` is the link's own directory, so `kani_scope.sh` is not found. The gate then exits 1 with the misleading `source stale against origin/main ()`. That fails closed. If the gate is called by absolute path from another cwd, it now gets past the freshness step and checks the evidence of that other directory, where before it stopped at the helper lookup. `make kani-gate` cannot reach either case. | scripts/kani_gate.sh:69, scripts/kani_gate.sh:96 |
| FND-002 | low | The negative test's stdout oracle checks only the `result=failed ran=3 expected=3` prefix. It does not check `tree=clean` or the full result line, so it does not pin that the trusted-cargo identity check is the only reason for rejection. Example: a stub change makes the tree dirty and a later change drops the `test_command[0] == trusted_cargo` term. The gate still fails (tree=dirty), so the test still passes while FR-041-AC-5's identity guard is gone. | tests/it/kani_gate.rs:324 |

## Verdict

Approve. Both findings are low and neither blocks the merge.

- Root cause confirmed. On main 494937f, `kani_gate::gate_rejects_fake_cargo_and_kani_with_printed_proof_and_reports`
  failed at kani_gate.rs:312 in 2 of 8 module-only runs. Main also leaves a
  `target-codex-backends/` directory in the checkout. On the PR head the module passed 8 of 8
  times, and the full `--test it` run passed once (594 passed, 0 failed, 34 ignored). Neither left
  `target-codex-backends/` in the checkout or changed git status.
- `make kani-gate` behaves the same as before. The recipe is `$(BASH) scripts/kani_gate.sh ...`
  from the repo root, so `dirname -- "$0"` is `scripts` and line 69 expands to the old path. The
  `exec flock ... "$0"` re-exec keeps both `$0` and cwd. The real lane still checks
  `target-codex-backends/ir19-claim-crate` relative to cwd. That is CARGO_MANIFEST_DIR, which is
  where `real_kani_claims_have_discharge_and_falsifying_mutations` writes it.
- The negative test still proves its claim. The reports are now in the same cwd-relative
  directory the gate reads. The fake git reports a stable head and a clean tree, and the list and
  run grep lines both match. So every pass condition holds except the trusted-cargo identity
  check, and dropping that term would make the gate pass and the test fail.
- No other shared checkout paths are written. The scope tests, the `make -n` test and the include
  walk only read the checkout. The lock and markers are in the sandbox, and mktemp logs are
  removed by the EXIT trap.
- `dirname -- "$0"` is correct for relative calls (`scripts/...` and `./kani_gate.sh`), for paths
  with spaces (quoted) and for paths that start with `-` (`--`). The only weak case is a symlink
  (FND-001).
- The rust-review lane found nothing. `to_str().expect` on a CARGO_MANIFEST_DIR path is
  acceptable in tests, and the ps needle and the spawned argv are built from the same env value.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The new test `gate_invoked_through_a_symlink_finds_its_scope_script` carries `Trace: NFR-006-AC-8`. NFR-006-AC-8 covers a tree or head that moves between the before-build and after-run reads, and this test does not exercise that. What the test shows is that the freshness step is reached and passes, so the run starts. That is closest to NFR-006-AC-14's "it may run" clause, or else it is a robustness test with no criterion tag. As written, the matrix credits AC-8 with a test that cannot fail when AC-8 is violated. | tests/it/kani_gate.rs:340-341 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | e7df0f6: kani_gate.sh now sets `script_dir=$(dirname -- "$(/usr/bin/readlink -f -- "$0")")` and has a comment saying that only the sibling script is located by real path. The new symlink test fails against the 3254cf4 script (panic at kani_gate.rs:356) and passes at 5013686. `/usr/bin/readlink` follows the hard-coded `/usr/bin/readlink` at line 49 and `/usr/bin/bash` and `/usr/bin/python3` elsewhere, so it is consistent. Under `make kani-gate` the helper is now called by absolute path, which behaves the same because kani_scope.sh uses `$0` only in its usage message. |
| FND-002 | fixed | e7df0f6: the negative test now requires the full result line, from `kani-gate: result=failed ran=3 expected=3 elapsed=` through `s kani=Kani 1.0 tree=clean`. Red check: with `FAKE_MOVE_TREE=yes` the 3254cf4 oracle passes and the e7df0f6 oracle fails with "only the identity check may fail: ... tree=dirty". |
| FND-003 | fixed | Round 2, dfa4bb0: the symlink test is now tagged `Trace: TC-045.` only. That tag is honest. TC-045 is the case for the `make kani-scope` and `make kani-gate` targets, and it names `scripts/kani_scope.sh` and `scripts/kani_gate.sh`, so the tag places the test in that case without crediting any criterion it cannot fail. The criterion trace set in kani_gate.rs at 99d7000 is the same as on main and at 5013686: FR-041-AC-5, NFR-006-AC-1 through AC-12, AC-14, AC-18 and AC-19, TC-045 and TC-056. NFR-006-AC-8 stays bound through `gate_refuses_missing_launcher_dirty_or_moving_tree_and_stale_main` (kani_gate.rs:422), which runs the moving-head case. |


## Follow-up review (disposition pass 3)

Provisional local review only. Frozen candidate: 80378693a29f745dfb7dc1d6b4eddb07ce294a1c;
new delta: e2f6c1b..8037869, `src/kani/run/memory.rs`. The locally cached
`origin/main...HEAD` diff was inspected as context, including the gate script and tests;
remote freshness was not verified. No spec/plan or CI workflow changes appear in that diff.
No applicable AssuranceProfile was found under `spec/`.

All prior finding text and disposition rows above are preserved. SR-11995 FND-001,
FND-002 and FND-003 already have terminal `fixed` outcomes; no redundant outcome rows
are added. This round introduces no new findings. This does not renew historical test-run
claims: no Cargo or Kani command ran in this review. The leader reports that the required
systemd user scope was denied before Cargo launch; this reviewer did not retry it or run
unscoped commands. Formatting, clippy, execution tests and the aggregate gate remain UNRUN.

Tracker team-key validation, private comment dedup/publication/read-back and label state
are unavailable in this environment. No tracker or GitHub comment was posted. No remote
state, mergeability or merge approval is asserted. This scratch artifact requires custody
by the leader; the frozen branch was not edited.


### Code and Rust lane evidence

Provisional verdict: no new defect found in the reviewed delta; executable validation pending.
`NamespaceOwner::dispatch` claims the init identity and calls `bind_root` before writing the
startup-gate byte (`src/kani/run/namespace.rs:260-271`). Thus the bound init is still held
before backend dispatch, and its identity-checked RSS is an actual observation of the owned
root. `bind_root` propagates failed/mismatched identity reads before mutating the peak.
The fix stores that already measured value via `Some(previous.unwrap_or(0).max(bound))`.
Later successful `observe` calls retain the maximum; an unavailable read does not create
an invented zero. A measured zero is legitimate, as already modeled by `resident_bytes`.
No new panic, integer conversion, unsafe code, lock, wire schema, or spawned work is added.
The update uses the existing peak field and max rule; it adds no new helper or copied
upstream authority. Existing prepare-only behavior remains `None` until a tree is bound.

The inspected successful-evidence test asserts `peak_resident_bytes.is_some()` after a
real bounded launch (`src/kani/run/execute.rs:1660-1686`), and the prepare-only test asserts
`None` (`src/kani/run/memory.rs:769-775`). These are existing behavioral checks, not new
execution evidence. The prepare-only trace remains FR-028-AC-21, and the success test
also cites FR-028-AC-2/4 as context. The historical symlink test's TC-045-only tag is unchanged;
no claim is made that the pending scratch trace conversion is applied on this old base.

`git diff HEAD^ HEAD --check` passed. The full local delta preserves existing CI lanes.
