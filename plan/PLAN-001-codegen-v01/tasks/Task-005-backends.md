---
id: Task-005
title: "Harness proof and vacuity backends"
type: Task
status: in_progress
track: B
priority: P0
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-003
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-004
    type: references
---
# Task-005: Harness, proof, and vacuity backends

## Scope

Implement tri-state harness/proptest generation, Kani lowering and proof dependencies, and LLVM-based
vacuity evidence after deterministic oracle semantics exist.

## Current slice

Issue #3 is being reconciled directly from the shared-assurance `main` revision. The harness
generator owns the pre-state snapshot, subject invocation ordering, post-state evaluation, runtime
`Verdict`, proptest execution loop, explicit-discard path, and complete campaign accounting boundary.
Its accepted-case floor and discard ceiling are request inputs bound into generation identity.
Strategy generation directly shapes bounded ranges, finite memberships, enums, and supported
correlated relations; only explicitly residual constraints may use rejection.

The bounded Kani adapter reuses the exact oracle predicates, emits distinct
framing/binding/contract/harness regions, derives a complete dependency graph, and records proof
execution as `not_run`. Its next reviewed increment generalizes direct Boolean bindings to bounded
`i64` current-input/current-state/pre-state arguments and post-state results. Inclusive assumptions
come only from checked IR `IntegerType` domains; post-state results carry the same domain in the
ensures contract. The v2 adapter and graph retain the complete typed ABI, bounds, cargo-kani 0.67.0
identity/options and concrete-playback configuration while preserving v1 artifacts historically.

## PR #22 round 8 repair delta

Seed the mixed positive fixture and pin its exact counters. Preserve every framework abort as
`Exhausted`, retaining the reason, accounting, and optional policy failure. Preserve the
policy-level discard check because supplied reports may contain prior discards, and prove that
path with a prepopulated-report fixture. Preserve the loop/failure checks and every generated-source
size guard. Correct the accounting unit and add a source-limit conformance case.
The campaign-outcome producer integration remains a separate completion item; generated-crate
execution supplies the focused campaign controls in this revision.

Issue #5's reviewed spec is being repaired in a bounded coordinator-authorized slice: generated
entry probes, a typed-IR implication census, strict LLVM reading, and measured classification
primitives. Native fixtures use actual generator outputs. Aggregate analysis and coverage
attestations await IR #50's bound population and a native producer/run-result contract; no private
binding or counter model is introduced. REV-014 and REV-015 record the repair and residual work.

## Guards

Next bounded dispatch is proposed in REV-017 atop published PR #27 `cd345e1`: implement
complete bound observation only after API approval, retaining unqualified provenance. A
runtime-owned campaign transport and an authorized native producer/shared verification join
are separate ownership gates; no private counters or attestation/receipt framework fills them.
Every generated-source guard remains unchanged.

The numeric/state Kani increment specified by FR-003 and TC-014 is complete. It consumes issue #4's
executable-oracle analyzer and rendered predicates directly, derives its bounds only from checked IR
domains, and proves the healthy Boolean/`i64` multi-result and ConfigVersion-style fixtures with the
pinned Kani backend. It does not consume, change, or duplicate issue #3's model-domain strategy
interface; agent E retains ownership of that branch and file set. Native replay of printed
counterexamples remains the downstream quire-spec-language IT-010 boundary.

- Current `main` is the branch base; superseded bespoke-assurance PRs are not revived or restacked.
  It carries the reviewed PR #22, #26, #23 and #25 heads, so this integration adds bound-package
  oracle generation and bound coverage analysis on top of them rather than restacking them.
- Unsupported state, constraint, or shrinking semantics fail with a structured diagnostic rather
  than falling back to an unreported filter.
- Vacuity is partial and closes no full FR-004 row. Boolean and numeric/state Kani are complete for
  FR-003; Task-005 stays `in_progress` because the independently owned issue #3 strategy slice and
  remaining vacuity scope are not claimed by this increment.

## Numeric/state Kani completion evidence

The issue #2 increment generates deterministic zero/one/multiple-result Boolean/`i64` subject ABIs,
exact inclusive IR-domain assumptions, v2 proof graphs and attestations, and source-spanned explicit
refusals. The local SUITE-008 run exercises cargo-kani 0.67.0 with the recorded executable digest and
exact options: healthy mixed and ConfigVersion-style identity subjects prove, while changed-state and
strict-comparison subjects print concrete counterexamples. TC-003, TC-005, and TC-014 are complete;
the FR-003 portion of TC-007 is complete, while TC-007 overall stays planned for FR-005 parity.
