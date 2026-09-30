---
id: "SR-033"
title: "IR-301 code review and Rust review: routed scalar harness execution"
type: SpecReview
schema_version: "1.0"
scope: "agent-ix/quire-contract-codegen; src/kani_execution.rs, src/lib.rs, tests/it/kani_obligations.rs, tests/it/kani_witness_join.rs, spec/functional/complete-v1/FR-017-pinned-kani-execution-evidence.md, spec/functional/complete-v1/FR-022-routed-generation.md, spec/test/complete-v1/TC-027-pinned-kani-execution-evidence.md, spec/test-matrix.md"
relationships: []
---

## Summary

Ticket: IR-301 ("CG Kani execution cannot run a routed scalar harness
(harness type mismatch)"). PR #181, branch `ir-301-exec-scalar-harness`,
reviewed at head `5f8dd9f`, base `origin/main` (`edade59`).

Code review + Rust review (this crate is Rust; folded into this same
artifact per those skills' own Output contract) of the diff
`git diff origin/main...HEAD`, scoped to the PR's changed files only.

The change introduces `KaniExecutableHarness<'a>` (`Contract(&KaniObligationHarness)` |
`Scalar(&KaniScalarObligationHarness)`), a `From` impl for each variant, and a private
`HarnessView` that projects the seven facts `execute_kani_obligation` and
`kani_launch_command` read (pins, identity digest, source artifact, oracle-source digest,
runtime revision, unwind, solver, options) so both harness kinds run through one code path.
`KaniExecutionRequest.harness` changed from `&KaniObligationHarness` to
`KaniExecutableHarness<'_>`; `KaniExecutionEvidence.kind` changed from `ObligationKind` to
`Option<ObligationKind>` (`None` for a scalar harness, which carries no contract role). All
call sites in `tests/it/kani_obligations.rs` and `tests/it/kani_witness_join.rs` were updated
to `harness.into()` / `(&harness).into()`.

Field-by-field check of `HarnessView::view()` against `KaniObligationIdentity` and
`ScalarObligationIdentity` (`src/kani_obligations.rs`) confirms every field is sourced
correctly for both variants (pins, solver, unwind, options, runtime_revision map 1:1;
`identity.oracle_digest` vs `identity.oracle_sha256` are correctly distinguished per kind).
No panics or `unwrap()`/`expect()` were introduced in `src/`. No new `unsafe`. No lock or
async surface touched.

Spec: FR-017-AC-11 added (new criterion, not a rewrite of an existing one), FR-017 Inputs/
Outputs/Behavior updated to describe the two-kind view, FR-022 gets a one-line note that
`execute_kani_obligation` accepts `KaniScalarObligationHarness` directly, TC-027 gets a new
paragraph describing the routed-scalar-harness test scenario, and `spec/test-matrix.md`
extends the AC range TC-027 covers. All read consistently with the code; the new AC and test
matrix row match what the tests actually assert.

Tests added (`tests/it/kani_obligations.rs`): a routed `x + 1` over `Int[0, 9]` harness built
with `generate_routed`, crate assembled the way the driver does (`oracle_artifacts`
`Cargo.toml` + harness `rust.contents` as `src/lib.rs`) — (1) identity pin drift refuses
before the backend is measured, (2) installed-backend drift refuses the same way, (3) cover
classification matches a contract harness's, and (4) an `#[ignore]`d kani-lane test runs it
against real Kani 0.67.0 under a 60s budget, asserting evidence names the scalar identity,
oracle digest, harness digest, pins and unwind, and that a crate missing the harness source
is refused with `HarnessNotInCrate`. This matches IR-301's stated acceptance directly. Per
the reviewer's brief, the ignored real-Kani test asserting only wiring (not a `Verified`
verdict, since CBMC did not conclude within budget on the measuring host) is expected here —
a follow-up branch (IR-303) already strengthens that assertion and was measured passing
separately; not flagged as a finding on this PR.

Gates run (all from the PR's own worktree, worktree's existing `target` dir, no new /tmp
target dirs):

| Gate | Command | Exit | Log |
| --- | --- | --- | --- |
| fmt | `cargo fmt --check` | 0 | `/tmp/claude-1000/-home-peter-dev/1c021523-1315-469a-97b0-cc0762694a67/scratchpad/fmt.log` |
| lint | `make lint` (`cargo clippy --locked --all-targets -- -D warnings`) | 0 | `/tmp/claude-1000/-home-peter-dev/1c021523-1315-469a-97b0-cc0762694a67/scratchpad/lint.log` |
| it::kani_obligations | `cargo test --locked --test it kani_obligations` | 0 (24 passed, 2 ignored, 229 filtered) | `/tmp/claude-1000/-home-peter-dev/1c021523-1315-469a-97b0-cc0762694a67/scratchpad/test-kani_obligations.log` |
| it::kani_witness_join | `cargo test --locked --test it kani_witness_join` | 0 (0 passed, 1 ignored, 254 filtered) | `/tmp/claude-1000/-home-peter-dev/1c021523-1315-469a-97b0-cc0762694a67/scratchpad/test-kani_witness_join.log` |
| lib::kani_execution | `cargo test --locked --lib kani_execution` | 0 (11 passed) | `/tmp/claude-1000/-home-peter-dev/1c021523-1315-469a-97b0-cc0762694a67/scratchpad/test-lib-kani_execution.log` |
| spec drift check (not a gate; verifying PR body's claim) | `make spec` | 2 | `/tmp/claude-1000/-home-peter-dev/1c021523-1315-469a-97b0-cc0762694a67/scratchpad/make-spec.log` — the only findings are AP-001/MP-001 frontmatter-schema drift and one FR-014 EARS warning at line 267, none in a file this PR touches; matches the PR body's claim of pre-existing main drift |

No `#[ignore]`d kani-lane tests were run, per the reviewer's scope.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | src/kani_execution.rs |

## Verdict

Clean. No blockers, no should-fix findings, no nits beyond the required placeholder row. The
diff is sound: the `KaniExecutableHarness`/`HarnessView` refactor is a faithful, single-path
generalization with no branching on kind anywhere except the two small `view()` match arms
that read structurally-identical fields from two distinct identity types; every call site was
updated; the new tests exercise pin drift, backend drift, cover classification and (under the
ignored real-Kani lane) an actual run, matching IR-301's stated acceptance criteria; the spec
and test-matrix updates are consistent with the code and the tests. Gates (fmt, lint, the two
targeted `it` test modules, the lib unit tests) all pass. Mergeable as-is.
