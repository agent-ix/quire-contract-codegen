---
id: SR-1940
title: "IR-652 lifecycle spec review (code-review)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@6b9cbd21f2db16abc6e6f777f11a372b689c9080; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/TC-049-caller-death-ownership.md (frozen candidate diff against main f4c37b253250671fdd990a635da963d013b5478c; PR not open)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-027
    type: reviews
---

# SR-1940: IR-652 lifecycle spec review, code-review

## Summary

Ticket: IR-652. The PR is not open; this reviews frozen candidate 6b9cbd21f2db16abc6e6f777f11a372b689c9080
on branch ir652-lifecycle-spec. Reviewer: claude-opus-5-5, session
8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run 5066cb1a-6ce7-4ef1-9919-36e3a3ff45a7. The diff changes
four spec files and no Rust, workflow or dependency file, so the code-review lane checks how the
amended criteria line up with the code and tests that already trace to them. The rust-review
checklist was read, but no Rust changed. One high finding.

## Method

Ran `git diff --stat f4c37b2...6b9cbd2` (four spec paths, 238+/122-). Grepped `src/` and `tests/`
for `Trace:` tags that name each changed or new criterion (FR-017-AC-19; FR-034-AC-2, 5, 7, 12,
14, 17, 23, 26, 31 to 34). Ran `quire matrix --format tsv` at the candidate head. Read
`src/kani/run/report_file.rs` and the four tests that trace FR-017-AC-19. Scope units examined:
FR-017-AC-19, and the tests `tc_027_execution_reads_only_the_report_its_own_run_exported`
(src/kani/run/execute.rs:1158), the tests at src/kani/run/execute.rs:1188 and 1231, and
`tc_027_the_report_is_read_bounded_and_refused_not_truncated` (src/kani/run/report_file.rs:85).
Changed FR-034 criteria have no tagged tests and compute `untagged`. No unsafe code, vendored
file, hash or pin is introduced.

## Verdict

**FAIL: one high finding.** No FR-034 criterion is falsely covered: each changed one computes
`untagged`. TC-027 and TC-049 claim no new executable coverage.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-017-AC-19 was rewritten to require pipe-to-memfd storage, a 16 MiB writer bound, concurrent kernel-backing charge, seals and all-owner-death reclamation. `quire matrix` still computes it `tagged`, through four tests that prove the superseded named-file allocation: a unique report file name, removal of the run's own file, and a read-bound refusal. So the computed matrix reports the new obligation as covered by tests that cannot fail on it. The disclaimer prose at TC-027:167 does not reach the computed matrix. Give the new obligation its own criterion id (leaving AC-19 for what those tests prove, or retiring it), or drop those tags in the same PR, so the new storage criterion computes `untagged`. | spec/kani/functional/FR-017-kani-execution-evidence.md:278, src/kani/run/execute.rs:1158, src/kani/run/execute.rs:1188, src/kani/run/execute.rs:1231, src/kani/run/report_file.rs:85, spec/kani/matrix/TC-027-kani-execution-evidence.md:167 |

## Dispositions

Round 1 reviewed a661f2f297f5e9860c25e9995226d07332426ab2 (previous 6b9cbd21f2db16abc6e6f777f11a372b689c9080; PR not open). Reviewer: claude-opus-5-5, session 8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run 9a4f4275-0799-4c05-867e-b01504c3095c.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | a661f2f297f5e9860c25e9995226d07332426ab2: Four stale FR-017-AC-19 Trace ids removed (execute.rs:1158/1188/1231, report_file.rs:85), test bodies and other ids unchanged; quire matrix at the fix head computes FR-017-AC-19 untagged. |
