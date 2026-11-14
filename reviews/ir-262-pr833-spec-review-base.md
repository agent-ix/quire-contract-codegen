---
id: SR-17940486709564847047984
title: "IR-262 PR #833 spec-review base"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #833 at 36cb1232, git diff origin/main...HEAD; IR-262; spec/kani/functional/FR-049-enum-scalar-harnesses.md (Status), spec/kani/matrix/TC-063-enum-scalar-harnesses.md (Status), spec/kani/matrix/tests.md (FR-049 row), docs/qualification/ir262-enum-scalar-kani-measurements.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-049
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-063
    type: reviews
---

## Summary

Ticket: IR-262. This is a base spec-review of the status edits. The PR changes only the Status
sections; no AC or Behavior text changed. I checked each status claim against the code at
36cb1232, the committed lock (QSL 434869e8, quire-exact b81e5950), the IR-262 ticket's acceptance
criteria and an installed-Kani re-run (see the code-review SR).

The diff adds no revision pins, hashes or digests to the spec prose. The measurements doc names
quire-exact `b81e5950` as provenance of the measurement. It is not an instruction to pin.

## Verdict

**FAIL: one high finding and one medium.** The verdict table in the measurements doc is accurate:
I reproduced Verified, Falsified and Incomplete-Falsified through the routed path. The two
PARTIAL items are disclosed, but neither is attributed correctly, and the AC wording is not met
where the Status says it is.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The owner-replay PARTIAL is not honest as written. (1) It contradicts FR-049 Behavior, which says that if the owner replay route is unavailable "the item is `unsupported` with no harness". The Status says the route is unavailable, yet lists AC-1/AC-5 harnesses as Supported and passing. (2) It names the wrong owner. QSL's function replay route (`qsl_replay::replay`) accepts `WitnessValue::Enum` (qsl-replay `execute/argument.rs:773-796`, QSL FR-098-AC-10). The block is CG's adapter (`src/replay/function.rs:256-260` refuses Enum), not quire-spec-language. (3) AC-5 appears under both "Tagged and passing" and "Not done". The `tests.md` row says "AC-1 to AC-10 tagged", so the matrix counts AC-5 and AC-10 as covered while their replay clauses are unmet. Fix: wire the replay (code-review FND-001), or follow the SHALL, or amend Behavior with the owner. Then list AC-5 and AC-10 as partial, not passing, and name a ticket id for any upstream gap that remains. | spec/kani/functional/FR-049-enum-scalar-harnesses.md (Status; Behavior "Symbolic ABI and witness roundtrip"); spec/kani/matrix/tests.md:20; spec/kani/matrix/TC-063-enum-scalar-harnesses.md:73 |
| FND-002 | medium | The AC-11 tool-versions/target deferral names "the CG runner evidence" as owner. That is this repository, not an upstream owner, and no ticket id is given. The IR-262 ticket's own AC reads "The run evidence shall record the Kani, Rust and solver versions, the unwind bounds and every option", so the ticket AC is unmet. The measurements doc records versions by hand and gives Rust only as "rustc stable". Org direction (EA-49, QSL-406, IR-441) moves away from recording tool versions in evidence, so this needs a ruling rather than a silent deferral. Fix: get a planner ruling, then either file a CG runner ticket (all families) and cite it in the Status, or amend FR-049-AC-11 and the ticket AC to drop tool versions and target. | spec/kani/functional/FR-049-enum-scalar-harnesses.md (Status, AC-11); docs/qualification/ir262-enum-scalar-kani-measurements.md:3 |

## Examined scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-049 Status | examined | PARTIAL (IR-262) ... Not done, with owner: owner replay ... AC-11 tool versions and target |
| FR-049 Behavior, witness roundtrip | examined | submit it through the existing owner replay route ... If that route is unavailable the item is `unsupported` with no harness. |
| FR-049-AC-5 | examined | replay through the owner route reproduces the disagreement. |
| FR-049-AC-11 | examined | Run evidence records ... tool versions, target, limits, raw checks and cover ... |
| TC-063 Status | examined | PARTIAL (IR-262): every step runs ... except the owner replay ... and AC-11's tool versions and target |
| tests.md FR-049 row | examined | PARTIAL (IR-262): AC-1 to AC-10 tagged ... |
| measurements doc | examined | verdict table, options vector, re-run note |
| IR-262 ticket AC | context_only | The run evidence shall record the Kani, Rust and solver versions, the unwind bounds and every option. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | The AC-11 amendment is not consistent. (1) TC-063 step 9 still says "Inspect the run record for ... full argv with unwind options, tool versions, target, limits ..." (AC-11), so the verification procedure still demands what the AC dropped. (2) FR-049-AC-11's "target" (a compilation target) was rewritten to "target harness", a different fact; the planner ruling (QSL-406, EA-49) covers tool versions only, and the Status line says only "Tool versions are not required". (3) The IR-262 ticket AC still reads "The run evidence shall record the Kani, Rust and solver versions". Fix: amend TC-063 step 9 to match AC-11; either keep "target" or record the ruling that drops it; the planner updates the ticket AC text. | spec/kani/matrix/TC-063-enum-scalar-harnesses.md:59-62; spec/kani/functional/FR-049-enum-scalar-harnesses.md:160,214-217 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Status now records the owner route as available (`replay_enum_witness` through `qsl_replay::replay`), the "Not done" list is gone, AC-5/AC-10 are backed by owner-replay tests, and tests.md/TC-063 agree. |
| FND-002 | fixed | Planner ruled tool versions are not required (QSL-406, EA-49); FR-049-AC-11 drops "tool versions" and the Status cites the ruling. The incomplete propagation of that amendment is FND-003. |
| FND-003 | fixed | Round 2 (7776d34d): TC-063 step 9 now reads "full argv with unwind options, target, limits, checks, cover, playback, exit and terminal outcome (AC-11); tool versions are not required". FR-049-AC-11 and its Status line put back "target" in place of "target harness". The IR-262 ticket AC now reads "record the target, the unwind bounds, every option and the outcome (tool versions are not recorded; QSL-406, EA-49)". |
