---
id: "SR-038"
title: "IR-21 gap analysis: acceptance criteria to tests"
type: SpecReview
scope: "agent-ix/quire-contract-codegen; src/spine_replay.rs, src/kani_module_gate.rs, src/kani_transcript.rs, src/kani_execution.rs, tests/it/skeleton_spine.rs, spec/functional/complete-v1/FR-016-witness-native-replay.md, spec/functional/complete-v1/FR-023-claimed-module-proof-gate.md, spec/test/complete-v1/TC-026-witness-native-replay.md, spec/test/complete-v1/TC-034-claimed-module-proof-gate.md"
relationships: []
---

# SR-038: IR-21 gap analysis

## Summary

Ticket: IR-21. PR: agent-ix/quire-contract-codegen#184, head `d2987ef`. This is a manual check that
maps each IR-21 acceptance criterion, FR-023-AC-1 to AC-4 and FR-016-AC-9 to the tests that back
them, and maps the new code back to its owning requirement.

## Method

I read each criterion, found the tagged test (`Trace:` lines), and checked that the test's
assertions actually cover the criterion. I ran the default and prover lanes myself (logs in
SR-037).

| Criterion | Backing test | Result |
| --- | --- | --- |
| IR-21 AC-1: clause to IR to crate to kani proves to violation to counterexample to replay with same verdict | `tc_034_one_boolean_clause_goes_from_contract_through_kani_to_native_replay` (prover lane, passed 107.94s) | Met from the Contract IR onward; see FND-001 |
| IR-21 AC-2: checked-in claimed list, discharged check per module, mutation per module turns red, unreached fails | same test + `tests/fixtures/skeleton_spine/claimed-modules.txt` + `kani_module_gate` unit tests | Met |
| IR-21 AC-3: FB-07, real `replay`, no stub | `tc_026_*` default lane + prover lane | Met; src calls `qsl_replay::replay`, healthy twin excludes a constant verdict |
| IR-21 AC-4 (brief): QSL dev pin off stale rev | Cargo.toml / Cargo.lock at `20ba521` | Met |
| IR-21 ticket AC: evidence only for stages the input passes | none | Not addressed; see FND-001 |
| FR-023-AC-1..AC-4 | `tc_034_*` unit tests + prover lane | Met |
| FR-016-AC-9 | `tc_026_*` default lane | Mostly met; see FND-002 |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | IR-21's ticket AC ("counts as evidence for ADR-011 E1 to E4 only for the stages its input actually passes through") has no backing statement. TC-026/TC-034 are marked Covered with no note that the input starts at a hand-built Contract IR projection and that the replayed package is a hand-mirrored QSL twin. | spec/test/complete-v1/TC-034-claimed-module-proof-gate.md:40-43, spec/test/complete-v1/TC-026-witness-native-replay.md:42-46, tests/it/skeleton_spine.rs:292 |
| FND-002 | low | FR-016-AC-9 says `inconclusive` has "both verdicts named", but the tests assert only `disagreement().is_some()` (default lane) or only the settlement (prover lane), and never check the named verdicts. | tests/it/skeleton_spine.rs:240-241, tests/it/skeleton_spine.rs:388 |
| FND-003 | low | Trace tag mismatch: `kani_module_gate` is tagged `Implements: FR-017` in src/lib.rs, while its tests trace FR-023. | src/lib.rs:27 |

## Verdict

Every IR-21 functional criterion is backed by a test that passes on a real prover run. The gap is in
the evidence-scope criterion (FND-001), which needs a statement, not code.

## Dispositions

Round 1.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed 728e431 | TC-034 (new paragraph after step 3), TC-026 Status and the skeleton_spine module doc now state that the input is a hand-built BoundPackage (contract-to-IR and E1-E4 do not run), that the twin is hand-mirrored, and that the limits are stand-ins. |
| FND-002 | fixed 728e431 | Both inconclusive replays assert `(cause.proved(), cause.replayed()) == (violation, success)`, in the default lane and the prover lane. |
| FND-003 | fixed 728e431 | src/lib.rs:27 `// Implements: FR-023`. |
