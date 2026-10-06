---
id: SR-1721
title: "IR-241 PR 302 portability fix gap analysis"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@1a556e1b8cab6e9c0b7da812f52237cbbefd068d; src/kani/run/execute.rs, src/kani/run/launch.rs, src/kani/run/memory.rs, src/kani/run/namespace.rs; spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/functional/FR-017-kani-execution-evidence.md (context)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: references
---

# SR-1721: IR-241 PR 302 portability fix gap analysis

## Summary

Ticket: IR-241. PR: quire-contract-codegen#302. Reviewer model: claude-opus-5-5. The audit was
planless and scoped to the PR diff and to the criteria its changed tests bind. Those are
FR-028-AC-2, -3, -4, -12 and -21, and FR-017-AC-14, -18, -19, -21 and -22. This is not a
repo-wide sweep, and it does not re-review merged #295. Neither the PR nor this audit claims
IR-241 or the Kani MVP complete. The caller-death guardian stays IR-639.

`quire matrix --scope . --format tsv` (quire 0.36.1) shows all ten criteria `tagged`. The
unsupported-platform clause of FR-028-AC-21 has two new or kept binders:
`execute.rs:1872` (non-Linux only, single and batch, with a no-dispatch marker) and
`launch.rs:657` (portable, unavailable procfs, no spawn).

The PR moves 15 batching and report tests from `execute_kani_obligation(s)` to a test-only
`report_fixture`. The fixture runs the same command, report and classification helpers through
`run_launcher`, which is `run_monitored` with no namespace owner and no memory observer. It does
this on every platform, Linux included. Two coverage regressions on Linux follow from that.
They are recorded below.

## Method

- Matrix rows were read for the ten criteria above. Each changed test was traced to the entry
  point it now calls: production `execute_kani_obligation(s)` through `start` and
  `run_bounded_launcher`, or `report_fixture::single`/`groups` through `run_launcher`.
- For every assertion that moved, the check was what source change would now make it fail
  that would have failed before.
- `grep` confirmed which public `KaniExecutionEvidence` fields still have a default-lane
  assertion: `launcher_path` and `solver` appear only at `execute.rs:2337,2341`, on the private
  `ReportedExecution`.
- The ignored real-Kani tests in `tests/it/kani_batching.rs` and `tests/it/kani_obligations.rs`
  were read as context for what the `make kani` lane still covers.
- No stub, `todo!`, `dbg!` or placeholder return was found in the diff.

## Verdict

**CHANGES REQUESTED.** The portability objective is met: FR-028-AC-21's unsupported-platform
refusal is bound and runs on macOS. On Linux, however, the production bounded launcher lost
default-lane coverage for paths it exercised before this PR. Those are the namespace-owned
output-limit stop, the deadline-free (`Duration::MAX`) namespace startup, and the mapping of
the public evidence fields. The PR body says the 17 Linux-only tests "still run on Linux". It
does not say that the 15 migrated tests no longer exercise the bounded runner on Linux.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-017-AC-22's public evidence fields are no longer asserted on `KaniExecutionEvidence` in the default lane. `launcher_path`, `solver`, `kind` and `checks` are checked only on the private `ReportedExecution`, so a wrong field mapping in `ReportedExecution::with_memory` passes every default test on Linux and macOS. | src/kani/run/execute.rs:282-298, src/kani/run/execute.rs:2319-2361 |
| FND-002 | medium | On Linux, no test now drives the namespace-owned bounded launcher through an output-over-limit stop (FR-017-AC-14) or a `Duration::MAX` request (FR-028-AC-12). The tests that did, `tc_043_an_over_limit_stream_refuses_a_single_run_and_a_batch_by_its_member_count` and `tc_043_a_duration_max_batch_runs_without_a_member_timeout_and_the_maximum_runs_with_one`, now use the unowned `report_fixture` path on every platform. | src/kani/run/execute.rs:2738-2770, src/kani/run/execute.rs:2218-2240, src/kani/run/execute.rs:886-975, src/kani/run/launch.rs:237-348 |

### Failure scenarios

- FND-001: edit `execute.rs:289` to `launcher_path: self.harness_path` (both are `String`, so
  it compiles). Every default-lane test still passes on Linux and macOS, because
  `tc_043_a_member_evidence_carries_its_own_fields_and_the_batch_invocation` asserts
  `ReportedExecution`. The ignored Kani lane asserts `harness_path`, `unwind` and `arguments`,
  but not `launcher_path` or `solver`. Before #302 the same test asserted the public evidence
  produced by `execute_kani_obligations`. Fix: on Linux, assert the member fields on the
  public evidence returned by the production entry (for example in the existing N-to-1 count
  test, or in a Linux twin of the member-evidence test). Keep the portable fixture assertion
  for macOS.
- FND-002: in `run_monitored`, make the namespace branch return `OutputOverLimit` before
  `owner.cleanup()` confirms teardown. Alternatively, make `NamespaceOwner::dispatch` refuse or
  panic when `deadline` is `None`, which is the `Duration::MAX` case, since `start` computes
  `Instant::now().checked_add(Duration::MAX) == None`. Either way, no default test fails. The
  `launch.rs` over-limit tests use `run_launcher_with_timeout` with no namespace, and the 25
  Kani proofs have finite T and small output. Before #302 both moved tests drove these exact
  paths through bwrap on Linux. Related moves with the same Linux effect are the own-report and
  stale-report tests (FR-017-AC-19, `execute.rs:1130,1203`) and the no-report batch rules
  (FR-017-AC-21, `execute.rs:2252`). Fix: keep the portable fixture tests, and on Linux run the
  same stand-ins through `execute_kani_obligation(s)` as well, for example with a
  `cfg`-selected `StandIn` entry or Linux-only twins. At minimum, cover the single and batch
  over-limit case and the `Duration::MAX` case, and state the split in the PR body.
