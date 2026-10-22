---
id: SR-15413
title: "IR-951 PR 521 CG Rust gate code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@a78828a721be4f16750dd8663f5a9a2e50c95181; src/bin/cg_gates.rs; Makefile; CLAUDE.md; tests/it/kani_gate.rs; deleted scripts/kani_scope.sh, scripts/kani_gate.sh, scripts/test_qspec_preflight.sh; NFR-006-AC-1,4,5,6,7,8,9,10,11,12,14,18,19; TC-045; TC-056"
review_set: subset
---

## Summary

Ticket: IR-951. Reviewed CG PR 521 at exact head a78828a721be4f16750dd8663f5a9a2e50c95181 on base 5c19d08a. Compared the three original shell controls to the Rust `cg_gates` subcommands and their Make callers. Checked scope path patterns and no-renames diff direction, lock and trusted launcher, clean/stable tree, positive listed count, current-run claim reports, QSpec negative selftest, Rust bounds and process behavior. The new source uses no copied external artifact or compatibility layer. An independent focused `cargo test --locked --test it kani_gate::` passed 12/12; TC-045 validation and diffcheck passed. No full aggregate or real Kani gate ran in this review.

## Verdict

**CONDITIONAL** — one new behavioral test omits the criterion ID it directly verifies. Add `NFR-006-AC-1` to its existing `#[trace]`. The ported gate's examined pass/fail and evidence controls otherwise match the originals in the scoped tests; the final real Kani run belongs to the PR owner.

## Coverage

- NFR-006-AC-1,2,3,14 (examined): Rust scope table matches the old shell glob set, uses `git diff --no-renames`, and refuses stale Kani-reaching main changes.
- NFR-006-AC-4,5,6,7,8,9,10,19 (examined): Make shares the Kani test command; the gate serializes it under `flock`, requires a nonempty list and all expected tests, checks trusted Cargo/Kani identity and current reports, and refuses dirty or moving trees.
- NFR-006-AC-11,12,18 (examined): ignored lane selection and transitive include path test remain; CI workflow is unchanged and dry-run CI excludes the Kani gate.
- QSpec preflight (examined): the Rust selftest retains unset, empty, nonexistent, missing fixture and pre-Cargo refusal controls. `qspec-preflight` remains the `test` prerequisite.
- Rust idioms (examined): checked process status and UTF-8 handling, no added `unsafe`, production panic, unbounded request recursion, borrowed OS path conversion, or copied schema/fixture. The output reader streams Kani stdout linewise and the gate rejects incomplete evidence.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new red-case path-classification test cites only TC-045 although it asserts NFR-006-AC-1; its criterion binding is absent from that test. | tests/it/kani_gate.rs:185 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The new red scope test now cites `NFR-006-AC-1` beside `TC-045` at exact head 7b1492de. |

## Disposition verdict

**PASS** at exact head 7b1492dee014e41c0796de0fad5bb8f3776c2343 for the changed code and Rust test trace. The gate implementation is unchanged in this fix round.
