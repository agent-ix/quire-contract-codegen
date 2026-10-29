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
  - target: ix://agent-ix/quire-contract-codegen/FR-041
    type: constrains
---
# NFR-006: A change that can alter what real Kani proves runs the real-Kani lane before it merges

## Statement

- The repository shall define the **Kani-touching set** as the changed paths matching
  `src/kani/**`, `src/oracle/**`, `src/routed/**`, `src/replay/**`, `src/core/**`,
  `src/publication/**`, `tests/it/kani_*.rs`, `tests/it/skeleton_spine.rs`,
  `tests/it/bounded_kani_corpus.rs`, `tests/it/scratch_crate.rs`,
  `tests/exact_scalar_support/package.rs`, `tests/checked_package_support/base.rs`,
  `tests/checked_package_support/rekey.rs`,
  `tests/state_frame_support/**`, `schemas/kani-*.schema.json`,
  `schemas/generated-rust-kani-*.schema.json`, `Cargo.toml` and
  `Cargo.lock`.
- The `make kani-scope` target shall classify changed paths supplied for the candidate. For a
  rename, the caller shall supply both the old and new paths.
- When any changed path is in the Kani-touching set, `make kani-scope` shall print `required` and the
  matching paths.
- When no changed path is in the Kani-touching set, `make kani-scope` shall print `not required`.
- The `make kani-gate` target shall run the recipe of the `make kani` target once, so that the
  toolchain, flags, target directory and the six filters have one definition that both targets use.
- The `make kani` recipe shall hold the lock named by `KANI_LOCK`, whose default is
  `/tmp/agent-e-heavy-build.lock`, and shall run the lane's tests with `--test-threads=1`.
- The `make kani-gate` target shall exit non-zero unless the lane's tests all passed, the number that
  ran equals the number the lane's filters list for `--ignored`, and that number is greater than zero.
- The `make kani-gate` target shall include [FR-041](../functional/FR-041-claimed-module-proof-gate.md)'s
  current-build claimed-module census and real mutation controls in its one required lane, and
  the target shall exit non-zero if any claimed module or shared helper fails that
  requirement. Optional targets and stored verdicts shall not satisfy the controls. This obligation
  remains Open until there is a genuine current-build claim source.
- The real-Kani lane shall include genuine current-build module claim and mutation
  controls when they exist. A retired fixed inventory or saved result grants no proof credit.
- The `make kani-gate` target shall print one run-result line of the form `kani-gate:
  result=<passed|failed> ran=<n> expected=<n> elapsed=<s>s kani=<version> tree=<clean|dirty>`,
  with `kani` the version the launcher reports. The line describes the run; it is not a
  transferable approval or a substitute for running the gate on the candidate under review.
- The `make kani-gate` target shall inspect the working tree before the build and after the run.
  If either inspection finds a dirty tree, it shall print `tree=dirty`, never report a pass,
  and exit non-zero. It shall not read or compare a Git revision for this check.
- If the `cargo-kani` launcher is absent, then `make kani-gate` shall run no test, print `kani-gate:
  not run: launcher absent` and exit non-zero.
- When a change touches a path in the Kani-touching set, the author shall run the actual full
  `make kani-gate` on a clean, stable candidate immediately before opening its pull request.
- When merging a pull request that touches the Kani-touching set, the merger shall verify that
  the candidate checked out for review contains the pull request's current changes.
- When the current Kani-touching candidate is ready to merge, the merger shall run the actual
  full `make kani-gate` on that clean, stable candidate. A copied pull-request status line does
  not satisfy this obligation.
- If a required lane cannot run or fails, the pull request shall remain unmerged. It cannot be
  described as verified by the real-Kani lane or waived by a written reason.
- When `make kani-scope` prints `not required`, no real-Kani run is required for that change.
- When preparing to push a version tag, the person pushing it shall run the actual full
  `make kani-gate` on the clean, stable candidate to be tagged. A copied release-ticket line is
  insufficient.
- The `make ci` target shall not depend on `kani-gate` or `kani`.
- The Kani-touching set shall contain the target of every `include!`, `include_str!`,
  `include_bytes!` and `#[path]` named in a `tests/it` file that holds a lane test, and in each file
  those name in turn.
- The set of tests marked `#[ignore = "kani lane: ..."]` shall equal the set of `#[ignore]`d tests
  that the `make kani` filters select.

## Scope

Applies to the real-Kani lane: the `#[ignore]`d tests of `tests/it` that run `cargo kani`, selected by
the filters `kani_obligations`, `skeleton_spine`, `bounded_kani_corpus`,
`kani_generation` and `kani_batching` (FR-015, FR-017, FR-031, TC-025, TC-026, TC-027, TC-043,
TC-044). The statement fixes the gate's behaviour and the local targets that implement it. It does
not name a hosting system: the CI workflow at `.github/workflows/ci.yml` is out of this requirement,
and a required status check or a scheduled run on that or any other system is a separate owner
decision (Open decisions).

The Kani-touching set is the code the lane reaches: the lane's tests call generation entry points of
`src/kani`, `src/oracle` and `src/routed`, decode and replay through `src/replay`, and reach their harness fixtures through
`tests/it/scratch_crate.rs` (which copies `Cargo.lock` into every scratch crate) and the `#[path]`
modules `tests/exact_scalar_support/package.rs` and `tests/state_frame_support/**`, the first of
which `include!`s `tests/checked_package_support/base.rs` and `#[path]`-includes
`tests/checked_package_support/rekey.rs`. The rule is by file, not by test: a lane
file's `include_str!` of `tests/state_frame_support/subject.rs` compiles the subject into the real-Kani
crate, and `tests/it/bounded_kani_corpus.rs` includes
`schemas/kani-corpus-proof-graph-v1.schema.json` for its corpus proof-graph controls. The set
therefore holds that surviving Kani schema, though no schema goes through Kani. The retired
`schemas/kani-proof-graph-v2.schema.json` and
`schemas/generated-rust-kani-v2.schema.json` paths remain `required`-classified by
`make kani-scope` if reintroduced, as NFR-006-AC-1 requires. The oracle and coverage schemas
are out because no lane file includes them (these are the include targets
of the lane files beyond the `#[path]` modules listed). `src/core/**` is
in the set because harness names and identity come from it. `src/strategy/**` and `src/evidence/**`
are out because no module of `src/kani`, `src/oracle`, `src/routed`, `src/replay`, `src/core` or
`src/publication` imports them and no lane test calls their entry points (as currently declared). A later change that makes the lane reach them extends the set.

## Rationale

The default `cargo test` skips every real-Kani test, so a change to what Kani is asked to prove can
leave `make ci` green. Two merged defects of that kind were found by running the lane, not by
`make ci`:

- A cover placed before an assertion is satisfied by the very valuation that falsifies the assertion;
  Kani prints one playback per distinct valuation, so a violation classified as a failure with no
  counterexample (IR-451, #260, FR-015-AC-7). The control that shows it is a real-Kani fixture.
- The V1 bundle and corpus harnesses carried no non-vacuity cover until #262 and #264 added one
  (FR-015-AC-53 to FR-015-AC-58); their real-Kani outcomes are the evidence.

Two more properties are shown only by real Kani and are not defects that merged: overflow in the
bundle oracle is a failing Kani check, not a wrapped value (FR-031-AC-20, and for divide and
remainder FR-031-AC-24), and the witness the replay
reads is Kani's own printed block (IR-29, TC-026), which a synthetic transcript does not exercise.

**Chosen rule.** The lane is required, in full, for a change that touches the set, and is not
charged to any other change. Real-Kani cost is large. This repository records only partial
timings: two oracle tests at 111 s (SR-1459); the `skeleton_spine` lane at 157 s on an unmodified head
(SR-1423); three scalar tests at 908 s (SR-647); nine tests over four modules at 1056 s (SR-690),
when the lane was smaller than it is now; and the IR-604 ticket's statement that the quire-integration
exemplar takes about 530 s serial. The lane now holds 27 tests across eight files; the full lane
ran in 1,308 s on the IR-617 pre-PR candidate. A required lane at every change would charge a spec-only or review-only change
that cost for no evidence, and `make ci` runs twice per change; so the rule keys on paths and `make
ci` stays as it is. The set is a path list, so `make kani-scope` decides it from the diff with no
judgement. The lane is run in full, not as a per-path subset, because one emitter change can reach
harness kinds the path does not name: the cover rule spans seven harness kinds (FR-015-AC-53 to
FR-015-AC-58), each emitted from its own function. Narrowing the lane per path needs a per-module cost
this repository does not measure (Open decisions).

**Rejected: a scheduled full lane as the gate.** A nightly or weekly run on `main` costs a change
nothing, but it finds a defect after the merge, with several merges between the last green run and
the red one, and it holds `main` red for everyone while the cause is isolated; it gives a pull
request no evidence at all; and it needs a runner with Kani, which neither workflow in the repository
has (`ci.yml` is `workflow_dispatch` only and installs no Kani; `cla.yml` checks the CLA), while the
host's heavy-build lock serialises lanes on one machine already shared with agents. A scheduled run is
not required here; the pre-tag run is the backstop for drift that no path in the set shows, such as a
Kani or toolchain upgrade, and for a merge that combined two passing heads.

**Rejected: adding the lane to `make ci`.** `make ci` is the gate for every change, spec and review
files included, and runs twice per change; the lane's cost would dominate it, contend for the lock
and push agents to skip `make ci`.

**What the run result is.** The gate prints its own counts, version and clean-tree status so the
operator can inspect the completed process and its exit status. A copied line in a pull request or
release ticket is not evidence that the current candidate was tested. The author runs the lane before
opening a Kani-touching pull request; the merger runs it again on the current candidate. The
clean-tree check is local to each run. The run result grants no later candidate proof credit.
The `ran`/`expected` check makes a filter that matches nothing, which libtest reports as success,
read as `failed`.

**One rule, no merge on a skip.** A pull request in the set merges only after the actual full gate
passes on the clean current candidate. A launcher-absent or skipped run cannot make it mergeable.
Whether the owner may ever waive the lane for one change, and who may record that, is an owner
decision (Open decisions); until the owner rules, the answer is no.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Merged pull requests touching the Kani-touching set without an actual full gate passing on the current candidate immediately before merge | 0 | 0 | inspection of the run and candidate |
| Lane runs reported `passed` with `ran` different from `expected` or `ran` of 0 | 0 | 0 | contract-testing |
| `#[ignore = "kani lane: ..."]` tests the lane does not select | 0 | 0 | contract-testing |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-006-AC-1 | `make kani-scope` prints `required` and the path for each of `src/kani/generate/negotiate.rs`, `src/oracle/boolean_v1.rs`, `src/routed/generate.rs`, `src/replay/witness.rs`, `src/core/identity.rs`, `src/publication/mod.rs`, `tests/it/kani_batching.rs`, `tests/it/skeleton_spine.rs`, `tests/it/bounded_kani_corpus.rs`, `tests/it/kani_obligations_state_frame.rs`, `tests/it/scratch_crate.rs`, `tests/exact_scalar_support/package.rs`, `tests/checked_package_support/base.rs`, `tests/checked_package_support/rekey.rs`, `tests/state_frame_support/model.rs`, `schemas/kani-proof-graph-v2.schema.json`, `schemas/generated-rust-kani-v2.schema.json`, `Cargo.toml` and `Cargo.lock`, each fed alone. | Test (TC-045) |
| NFR-006-AC-2 | `make kani-scope` prints `not required` for each of `spec/kani/functional/FR-017-kani-execution-evidence.md`, `reviews/REV-018-bound-coverage-observations.md`, `src/strategy/mod.rs`, `src/evidence/mod.rs`, `Makefile` and `.github/workflows/ci.yml`, each fed alone. | Test (TC-045) |
| NFR-006-AC-3 | Given both old and new paths for a rename from `src/kani/old.rs` to `src/strategy/new.rs`, or from `src/strategy/old.rs` to `src/kani/new.rs`, `make kani-scope` prints `required` for each. | Test (TC-045) |
| NFR-006-AC-4 | OPEN during IR-348 migration: the `kani_generation` filter remains and must select V2 tests after V1-only tests retire. Original criterion retained: `make -n kani` and `make -n kani-gate` expand to the same `cargo test` command line, and that line holds each of the six filters `kani_obligations`, `skeleton_spine`, `kani_witness_join`, `bounded_kani_corpus`, `kani_generation` and `kani_batching`. | Test (TC-045) |
| NFR-006-AC-5 | `make kani-gate` starts the lane's `cargo test` once with `--test-threads=1`, holds the lock named by `KANI_LOCK` while it runs, and does not start it while another process holds that lock. | Test (TC-045) |
| NFR-006-AC-6 | `make kani-gate` exits non-zero and prints `result=failed` when the lane's filters list zero tests, when fewer tests run than are listed, and when any test fails; it exits zero and prints `result=passed` only when every listed test ran and passed and the count is above zero. | Test (TC-045) |
| NFR-006-AC-7 | The passing run-result line has the fields `result`, `ran`, `expected`, `elapsed`, `kani`, `tree` in that order, with `ran` equal to `expected`, a non-empty `kani` equal to the version the launcher reports, and `tree=clean`. A run whose launcher reports no version prints `result=failed`. | Test (TC-045) |
| NFR-006-AC-8 | When the tree is clean before the build but dirty after the run, `make kani-gate` prints `tree=dirty`, never `result=passed`, and exits non-zero without reading or comparing a Git revision. | Test (TC-045) |
| NFR-006-AC-9 | With a dirty working tree before the build, `make kani-gate` prints `tree=dirty`, never `result=passed`, and exits non-zero. | Test (TC-045) |
| NFR-006-AC-10 | With no `cargo-kani` launcher `make kani-gate` starts no test, prints `kani-gate: not run: launcher absent` and exits non-zero. | Test (TC-045) |
| NFR-006-AC-11 | The tests marked `#[ignore = "kani lane: ..."]` in `tests/it` are exactly the `#[ignore]`d tests of `tests/it` whose path holds one of the `make kani` filters. | Test (TC-045) |
| NFR-006-AC-12 | `make -n ci` expands to no `kani` or `kani-gate` command. | Test (TC-045) |
| NFR-006-AC-13 | For a Kani-touching pull request, the author runs the full gate on its clean candidate before opening it, and the merger runs it again on the clean current candidate before merge; both actual processes exit successfully with all expected tests run. A copied body line alone cannot satisfy either run. | Inspection |
| NFR-006-AC-15 | A Kani-touching pull request whose required lane did not run or failed remains unmerged and does not claim real-Kani verification; a written explanation does not waive the gate. | Inspection |
| NFR-006-AC-16 | A pull request whose scope is `not required` has no mandatory real-Kani run under this requirement. | Inspection |
| NFR-006-AC-17 | Before a version tag is pushed, the full gate actually passes on the clean candidate to be tagged; a copied release-ticket line cannot substitute for that run. | Inspection |
| NFR-006-AC-18 | Every `include!`, `include_str!`, `include_bytes!` and `#[path]` target named in a file of `tests/it` that holds a lane test, and in each file those name in turn, matches a pattern of the Kani-touching set. | Test (TC-045) |
| NFR-006-AC-19 | On a clean current candidate, `make kani-gate` runs FR-041's current-build module census and real mutation controls in its one required lane, and a failure of any claim or shared-helper control makes the gate exit non-zero. An optional mutation target, retired fixed inventory or saved earlier result cannot substitute. Open until a genuine current-build claim source exists. | Test |

NFR-006-AC-8 retains the post-run dirty-tree refusal without a Git revision comparison.
NFR-006-AC-14 is retired under IR-1105 because it compared Git revisions.
No equivalent revision or digest gate replaces AC-14. The actual current-candidate Kani runs remain required.

NFR-006-AC-19 remains Open. IR-1105 retires the frozen V1 claim inventory and
its hold; neither provides module proof credit. The original FR-003@4 transition
and a distinct V2 implication claim each require genuine current-build module
claims and installed-Kani positive and falsifying controls. IR-1047 Acceptance 4
remains unmet.

## Open decisions

- Whether the owner may ever waive the lane for one change, and who may record the waiver. The default,
  until the owner rules, is that the pull request does not merge.
- Whether a required status check or a scheduled run on a hosting system enforces this mechanically.
  That is a workflow change and an owner decision; this requirement leaves `.github/` untouched.
- Whether a per-path subset of the lane can stand for the whole lane for a runner-only or a replay-only
  change. It needs a per-module cost the repository does not record.
- Whether `src/core/**`, `Cargo.toml` and `Cargo.lock` belong in the set. They are in it because
  harness names and identity come from `src/core`, and the QSL, Contract IR and Contract Runtime
  versions reach every real-Kani run; the cost is a lane run for every dependency change. The
  `Makefile` is not in it, so an edit to the `kani` target alone escapes the gate.

## Verification

TC-045 drives the two targets with stand-in launcher and `cargo` executables, a fixture list of changed
paths, a temporary lock path and a source walk of the `#[ignore]`d tests. The pull-request and
release criteria inspect the actual gate processes and clean candidate.

## Dependencies

- **Upstream**: [FR-015](../functional/FR-015-bounded-kani-obligations.md),
  [FR-017](../functional/FR-017-kani-execution-evidence.md),
  [FR-031](../../oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md).
- **Downstream**: [TC-045](../matrix/TC-045-real-kani-lane-gate.md).
