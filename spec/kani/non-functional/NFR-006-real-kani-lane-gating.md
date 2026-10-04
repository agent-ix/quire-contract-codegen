---
id: NFR-006
title: "A change that can alter what real Kani proves runs the real-Kani lane before it merges"
type: NFR
quality_attribute: reliability
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: constrains
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: constrains
  - target: ix://agent-ix/quire-contract-codegen/FR-031
    type: constrains
---
# NFR-006: A change that can alter what real Kani proves runs the real-Kani lane before it merges

## Statement

- The repository shall define the **Kani-touching set** as the changed paths matching
  `src/kani/**`, `src/oracle/**`, `src/routed/**`, `src/replay/**`, `src/core/**`,
  `tests/it/kani_*.rs`, `tests/it/skeleton_spine.rs`, `tests/it/bounded_kani_corpus.rs` and
  `Cargo.lock`.
- The repository shall provide `make kani-scope`, which reads the paths a change touches against the
  merge base with `origin/main` and prints `required` and the matching paths when any is in the
  Kani-touching set, and `not required` otherwise.
- The repository shall provide `make kani-gate`, which runs the whole `make kani` lane once, serially,
  while holding the host-wide lock `/tmp/agent-e-heavy-build.lock`, and prints one evidence line.
- The gate shall exit non-zero unless the lane's tests all passed, the number that ran equals the
  number the lane's filters list for `--ignored`, and that number is greater than zero.
- The gate's evidence line shall have the form `kani-gate: result=<passed|failed> ran=<n>
  expected=<n> elapsed=<s>s kani=<version> tree=<clean|dirty> head=<commit>`, with `head` the
  commit the lane ran on.
- If the working tree is dirty, then the gate shall print `tree=dirty` and exit non-zero.
- If the `cargo-kani` launcher is absent, then the gate shall run no test, print `kani-gate: not run:
  launcher absent` and exit non-zero.
- When a change touches a path in the Kani-touching set, the change's author shall run `make
  kani-gate` on the change's final head and put its evidence line in the pull request body.
- When a pull request touches a path in the Kani-touching set, the merger shall not merge it until
  its body carries an evidence line with `result=passed`, `tree=clean` and a `head` equal to the pull
  request's head commit.
- If a pull request touches a path in the Kani-touching set and the lane did not run, then the
  pull request body shall carry `kani-gate: not run: <reason>`.
- If a pull request body carries `kani-gate: not run: <reason>`, then the body shall not describe the
  change as verified by the real-Kani lane.
- When `make kani-scope` prints `not required`, the pull request body may carry the line `kani-gate:
  not required` in place of an evidence line.
- When a version tag is about to be pushed, the person pushing it shall run `make kani-gate` on the tagged
  commit and record the evidence line in the release ticket.
- The `make ci` target shall not depend on `kani-gate` or `kani`.
- The set of tests marked `#[ignore = "kani lane: ..."]` shall equal the set `make kani` selects.

## Scope

Applies to the real-Kani lane: the `#[ignore]`d tests of `tests/it` that run `cargo kani`, selected by
the filters `kani_obligations`, `skeleton_spine`, `kani_witness_join`, `bounded_kani_corpus`,
`kani_generation` and `kani_batching` (FR-015, FR-017, FR-031, TC-025, TC-026, TC-027, TC-043,
TC-044). The statement fixes the gate's behaviour and the local targets that implement it. It does
not name a hosting system: the CI workflow at `.github/workflows/ci.yml` is out of this requirement,
and a required status check or a scheduled run on that or any other system is a separate owner
decision (Open decisions).

## Rationale

The default `cargo test` skips every real-Kani test, so a change to what Kani is asked to prove can
leave `make ci` green. The defects this repository has merged from exactly that behaviour were found
by running the lane, not by `make ci`:

- A cover placed before an assertion is satisfied by the very valuation that falsifies the assertion;
  Kani prints one playback per distinct valuation, so a violation classified as a failure with no
  counterexample (IR-451, FR-015-AC-7). The control that shows it is a real-Kani fixture.
- The V1 bundle and corpus harnesses verified with no non-vacuity cover until FR-015-AC-53 to
  FR-015-AC-58 added one; the run outcomes of those harnesses are real-Kani outcomes.
- Overflow in the bundle oracle is a failing Kani check only under real Kani
  (`kani_bundle_oracle_overflow_is_a_failing_check_not_a_wrapped_value`, FR-031-AC-20).
- The witness the replay reads is Kani's own printed block (IR-29, TC-026); a decoder that returns a
  fixed pair passes every default test.

**Chosen rule.** The lane is required, in full, for a change that touches an emitter, the runner, the
replay or the lockfile, and is not charged to any other change. Real-Kani cost is measured and
large: the reviews in this repository record two oracle tests at 111 s (SR-1459), the skeleton spine
at 157 to 398 s (SR-1423, SR-1424), three scalar tests at 908 s (SR-647) and six modules at 1056 s
(SR-690); the IR-604 ticket reports 2277 to 2789 s for the full lane serial in recent gates (a
figure the ticket states and this repository does not record). A required lane that runs at every
change would charge a spec-only or review-only change that cost for no evidence, and `make ci` is
run twice per change; so the rule keys on paths and `make ci` stays as it is. The set is a path
list, so `make kani-scope` decides it from the diff with no judgement. The lane is run in full, not as
a per-path subset, because one emitter change can reach harness kinds the path does not name: the cover rule
spans seven harness kinds (FR-015-AC-53 to FR-015-AC-58), each emitted from its own function.
Narrowing the lane per path needs a per-module cost this repository does not measure (Open decisions).

**Rejected: a scheduled full lane as the gate.** A nightly or weekly run on `main` costs a change
nothing, but it finds a defect after the merge, with several merges between the last green run and
the red one, and it holds `main` red for everyone while the cause is isolated; it gives a pull
request no evidence at all; and it needs a runner with Kani, which the repository's one workflow does
not have (it is `workflow_dispatch` only and installs no Kani), while the host's heavy-build lock
serialises lanes on one machine already shared with agents. A scheduled run is not required here;
the pre-tag run is the backstop for drift that no path in the set shows, such as a Kani or toolchain
upgrade.

**Rejected: adding the lane to `make ci`.** `make ci` is the gate for every change, spec and review
files included, and runs twice per change; the lane's cost would dominate it, contend for the lock
and push agents to skip `make ci`.

**Why a log line and not an assertion.** A pull request that says "the Kani lane passed" is a claim.
The evidence line is the lane's own output, names the commit it ran on (the one identity that binds a
run to the content it proved, as the repository's hash rule allows), and carries `ran` against
`expected` so a filter that matches nothing, which libtest reports as success, reads as `failed`. The
enforcement is the merger's inspection of that line: nothing in the repository enforces it mechanically
until a hosting decision is made (Open decisions), and this requirement says so rather than implying
a branch protection.

**Why a skip is stated.** A change that touched the set and ran no lane would otherwise read the same
as one that passed. `not run: <reason>` keeps the gap visible in the pull request, and the merger
decides on it; this requirement does not say who may accept it.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Merged pull requests touching the Kani-touching set with no `result=passed` line at their head | 0 | 0 | inspection |
| Lane runs reported `passed` with `ran` different from `expected` or `ran` of 0 | 0 | 0 | contract-testing |
| `#[ignore = "kani lane: ..."]` tests the lane does not select | 0 | 0 | contract-testing |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-006-AC-1 | `make kani-scope` prints `required` and the path for each of `src/kani/generate/negotiate.rs`, `src/oracle/boolean_v1.rs`, `src/routed/generate.rs`, `src/replay/witness.rs`, `src/core/identity.rs`, `tests/it/kani_batching.rs` and `Cargo.lock`. | Test (TC-045) |
| NFR-006-AC-2 | `make kani-scope` prints `not required` for a change touching only `spec/kani/functional/FR-017-kani-execution-evidence.md`, `reviews/REV-018-bound-coverage-observations.md`, `src/strategy/mod.rs`, `Makefile` and `.github/workflows/ci.yml`. | Test (TC-045) |
| NFR-006-AC-3 | `make kani-gate` runs the lane's tests with `--test-threads=1` as the one cargo test process under `flock /tmp/agent-e-heavy-build.lock`, and does not start the tests while another process holds that lock. | Test (TC-045) |
| NFR-006-AC-4 | `make kani-gate` exits non-zero and prints `result=failed` when the lane's filters list zero tests, when fewer tests run than are listed, and when any test fails; it exits zero and prints `result=passed` only when every listed test ran and passed and the count is above zero. | Test (TC-045) |
| NFR-006-AC-5 | The evidence line of a passing run has the fields `result`, `ran`, `expected`, `elapsed`, `kani`, `tree` and `head` in that order, with `ran` equal to `expected`, `tree=clean` and `head` the commit the lane ran on. | Test (TC-045) |
| NFR-006-AC-6 | With a dirty working tree `make kani-gate` prints `tree=dirty`, never `result=passed`, and exits non-zero. | Test (TC-045) |
| NFR-006-AC-7 | With no `cargo-kani` launcher `make kani-gate` starts no test, prints `kani-gate: not run: launcher absent` and exits non-zero. | Test (TC-045) |
| NFR-006-AC-8 | The tests marked `#[ignore = "kani lane: ..."]` in `tests/it` and the tests `cargo test --test it -- --ignored --list` lists under the `make kani` filters are the same set. | Test (TC-045) |
| NFR-006-AC-9 | `make -n ci` expands to no `kani` or `kani-gate` command. | Test (TC-045) |
| NFR-006-AC-10 | The body of a merged pull request that touched the Kani-touching set carries a `kani-gate` line with `result=passed`, `tree=clean` and a `head` equal to the pull request's last commit. | Inspection |
| NFR-006-AC-11 | The body of a pull request that touched the Kani-touching set and ran no lane carries `kani-gate: not run: <reason>` and does not state that the real-Kani lane verified the change. | Inspection |
| NFR-006-AC-12 | The release ticket of a pushed version tag carries a `kani-gate` line with `result=passed` and a `head` equal to the tagged commit. | Inspection |

## Open decisions

- Whether a required status check or a scheduled run on a hosting system enforces this mechanically.
  That is a workflow change and an owner decision; this requirement leaves `.github/` untouched.
- Whether a per-path subset of the lane can stand for the whole lane for a runner-only or a replay-only
  change. It needs a per-module cost the repository does not record.
- Whether `src/core/**` and `Cargo.lock` belong in the set. They are in it because harness names and
  the QSL, Contract IR and Contract Runtime locks reach every real-Kani run; the cost is a lane run for
  every dependency bump. The `Makefile` is not in it, so an edit to the `kani` target alone escapes
  the gate.

## Verification

TC-045 drives the two targets with stand-in launcher and test-runner executables and a fixture list of
changed paths, and compares the ignored tests with the lane's selection. The pull-request criteria are
inspection of the pull request body.

## Dependencies

- **Upstream**: [FR-015](../functional/FR-015-bounded-kani-obligations.md),
  [FR-017](../functional/FR-017-kani-execution-evidence.md),
  [FR-031](../../oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md).
- **Downstream**: [TC-045](../matrix/TC-045-real-kani-lane-gate.md).
