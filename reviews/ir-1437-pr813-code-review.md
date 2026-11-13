---
id: SR-17940486709564847047753
title: "Code review — CG PR 813 make kani-mutations target"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; Makefile, scripts/cg_gates.rs, scripts/kani_mutations.rs, scripts/kani-mutants.json, tests/it/kani_mutations.rs, tests/it/main.rs, spec/kani/functional/FR-055-kani-mutations-target.md, spec/kani/matrix/TC-069-kani-mutations-target.md; ticket IR-1437"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-055", type: references }
---

## Summary

Code review with the Rust lane (rust-review folded in) of PR #813 at 0eeade84, scoped to `git diff origin/main...HEAD`. Ticket: IR-1437. Read by code (Kani not run): the owning tc_027 proof tests in `tests/it/kani_obligations.rs`, the Kani launcher (`src/kani/run/launch.rs`, `src/kani/run/execute.rs`, `src/kani/run/scalar.rs`), the playback reader (`src/kani/output/playback.rs`), and the mutant anchor sites in `src/oracle/scalar/render.rs` and `src/kani/generate/scalar.rs`. Those two files belong to another lane and were only read.

## Verdict

**NOT MERGEABLE.** The control flow is sound and matches FR-055. Pass, fail, malformed and unavailable are each distinct. The floor of 3 gives exit 2 before anything runs. The per-mutant timeout and the total budget are enforced by a process-group kill. The working tree is never written: the mutant goes into a temp-dir copy, so a panic or kill cannot touch the tree. The target is outside `make ci`. The lock is `flock $(KANI_LOCK)`, the same as `make kani`. There are no pins or hashes, and nothing is copied from RT (RT's target is Python). The stand-in tests check the gate's control flow honestly. But two faults mean the shipped declaration can never pass against real Kani: the rejection marker never reaches the owning proof's output (FND-001), and one mutant edits test-only code (FND-002).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `VERIFICATION:- FAILED` never appears in the owning tc_027 tests' output: Kani stdout is piped and parsed (launch.rs:357-358), and a falsified proof surfaces only as the `.expect` panic Debug of `ProofNotVerified(evidence)`. That evidence has no raw stream for a failed property (execute.rs:291-298), and its counterexample is only the playback block. A correctly rejected mutant is therefore `fail`, so the target can never exit 0 | scripts/kani_mutations.rs:19, scripts/kani_mutations.rs:203, tests/it/kani_obligations.rs:2546 |
| FND-002 | high | Mutant 3 anchors the `assert!(sound, ...)` template in `#[cfg(test)] fn render_scalar_reference` (src/kani/generate/scalar.rs:1019-1095). That reference renderer is compiled out of the library the `it` binary links. The production harness assert is the AST `Statement::Assert` at scalar.rs:781, so the mutant cannot affect the owning proof and always survives | scripts/kani-mutants.json:15-18 |
| FND-003 | medium | Every mutant does a full cold build. The scratch copy skips all `target*` dirs and KANI_MUTANT_TEST has no `--target-dir`, so the whole crate plus the `it` binary is built inside /tmp/cg-kani-mutations-*/target (GBs on the shared disk). The child runs in its own process group, so Ctrl-C stops make and the gate but leaves cargo and Kani running orphaned. On SIGINT or SIGKILL, `Scratch::drop` never runs and the multi-GB scratch dir leaks | scripts/kani_mutations.rs:228-249, scripts/kani_mutations.rs:284, Makefile:132 |
| FND-004 | low | Error returns after spawn (`child.try_wait()?`, `fs::read(&log_path)?`) leave the function without killing the child's process group, which orphans the run | scripts/kani_mutations.rs:252-253 |
| FND-005 | low | `remove_file` plus `write` only protects against a symlinked mutant file. A declared file under a symlinked directory (copy_tree keeps the link) writes through the link, possibly into the working tree. No such symlink exists today; refusing a target path whose parent canonicalises outside the scratch dir would close it | scripts/kani_mutations.rs:230-236, scripts/kani_mutations.rs:290-291 |
| FND-006 | low | The AC-6 test is named "nothing runs", and the AC-9 test asserts nothing ran, but neither checks that the runner was never invoked (for example with a sentinel file the stand-in runner writes). They only check the printed rows | tests/it/kani_mutations.rs:266-283, tests/it/kani_mutations.rs:296-308 |

## Coverage

- Rust lane: rust-style checks (error enum, exit-code mapping, process-group kill, Drop cleanup, no unsafe, no unwrap on fallible I/O in the gate) were applied to scripts/kani_mutations.rs and scripts/cg_gates.rs.
- Anchors: each of the three anchors occurs exactly once in current src. Mutants 1 and 2 are in production `render_integer_arithmetic` code; mutant 3 is test-only (FND-002).
- Focused tests: `cargo test --locked --test it kani_mutations` run via locked-build.sh with a throwaway target dir (see report). Kani and make ci were not run.

## Dispositions

Round 1, checked against fix commit 64d0a8b2 (`git diff 12c98cd8..64d0a8b2`, main merge excluded). Focused tests: 12 kani_mutations tests plus the scalar unit test `only_a_falsified_proof_emits_the_kani_rejection_verdict` passed. Kani and make ci were not run.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | `KaniScalarNativeError::falsified_verdict` returns the marker only for `ProofNotVerified` with a `Falsified` outcome. The tc_027 tests print it with `println!` before panicking, and `--nocapture` sends it to the log the gate reads |
| FND-002 | fixed | Mutant 3 now anchors the production `Statement::Assert { condition: expr("sound"...` in `render_scalar` (scalar.rs:781, which negotiate.rs:405 calls). It occurs exactly once. A new syn test refuses any anchor inside cfg(test) code |
| FND-003 | fixed | One scratch root per run with a shared `CARGO_TARGET_DIR`, so only the first mutant builds cold. SIGINT, SIGTERM and SIGHUP set an atomic, the gate kills the group, and Drop removes the scratch; a test covers this. SIGKILL can still leak the scratch, which no handler can prevent |
| FND-004 | fixed | `inject_and_run` now kills the group on every result other than `Exited`, I/O errors included |
| FND-005 | fixed | `has_symlink_component` refuses, as `malformed`, a mutant path that passes through a symlink |
| FND-006 | still-open | The AC-9 test now asserts the runner was never invoked. The AC-6 test (`without_cargo_kani_every_mutant_is_unavailable_and_nothing_runs`) still asserts only the printed rows and never checks `sandbox.invocations()` is empty |
