---
id: "SR-036"
title: "IR-303 gap analysis: routed scalar Kani harness verifies"
type: SpecReview
scope: "agent-ix/quire-contract-codegen; src/exact_scalar.rs, src/oracle.rs, tests/it/kani_obligations.rs, spec/test/complete-v1/TC-027-kani-execution-evidence.md"
relationships: []
---

# SR-036: IR-303 gap analysis

## Summary

Ticket: IR-303. Acceptance criterion: `tc_027_a_routed_scalar_harness_verifies` returns `Verified`
for the routed `x + 1` over `Int[0, 9]` harness under Kani within its budget; a harness
that violates its result bound returns a counterexample outcome (one negative case). The ticket's
"Do" section also asks to decide where a stack-size setting lives.

## Acceptance-criteria-to-tests trace

| Criterion | Backing test | Traced? |
| --- | --- | --- |
| Routed `x + 1` over `Int[0, 9]` harness returns `Verified` under Kani within budget | `tc_027_a_routed_scalar_harness_verifies` (tests/it/kani_obligations.rs:2521) | Yes — measured `Verified`, 288.87s, well inside the 600s `REAL_KANI_TIMEOUT` (tests/it/kani_obligations.rs:57) |
| A harness violating its result bound returns a counterexample outcome (negative case) | `tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified` (tests/it/kani_obligations.rs:2559) | Yes — measured `Falsified { .. }`, 216.44s |
| Decide where a stack-size setting lives | PR body: "No stack-size change needed: CBMC did not segfault at the default 8 MB soft stack" | Yes, as a documented negative decision — no code path was added or needed; verified no stack/ulimit/rlimit code exists in the diff |

Both `tc_027` tests carry `Trace: FR-017-AC-7, FR-017-AC-11, TC-027` doc comments, and
`spec/test/complete-v1/TC-027-kani-execution-evidence.md`'s routed-scalar paragraph was
updated in the same PR to describe both the verified and falsified runs, keeping FR-017-AC-11's
governing TC current with what the tests now actually assert (previously TC-027 explicitly
disclaimed the verdict: "The verdict is not asserted: CBMC did not conclude this harness within
ten minutes").

## Underspecified code / code with no owning requirement

None found in the diff. Every changed production line (the runtime move, the unwind
bump, the `manifest()` cbmc-flags addition) is a straight input to the two traced `tc_027`
tests.

The one gap this analysis surfaces is carried instead as a code-review finding (SR-035 FND-001):
the same cbmc-flags fix is not mirrored into `exact_function::manifest()` or
`composite_equality::manifest()`, which currently have no owning test either way because no
kani-lane test exists yet for those oracle kinds — there is no undertested acceptance criterion
here to flag under gap-analysis (nothing claims those crates verify under Kani today), so it is
recorded as a review finding rather than a matrix gap.

## Verdict

Complete against IR-303's stated acceptance criteria. Both required cases (positive verify,
negative falsify) are traced to tests that were actually run under real Kani and measured to pass.
No untraced or underspecified production code. No blockers.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No gaps found against IR-303's acceptance criteria (placeholder — this method's own finding, the manifest-flag asymmetry, is filed once under SR-035 to avoid duplicate reporting) | - |
