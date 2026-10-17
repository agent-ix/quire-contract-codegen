---
id: SR-12210
title: "Base spec review of IR-859 FR-015-AC-96..102 status refresh (CG #424)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #424 at 86d63d3 (IR-859), diff origin/peter/ir-805-graph-reachability-spec...HEAD: spec/kani/functional/FR-015-bounded-kani-obligations.md (IR-268 paragraph :95-98, FR-015-AC-96, -97, -99..-102 rows), spec/kani/matrix/tests.md :34 and :95, spec/kani/matrix/TC-025-bounded-kani-obligations.md :66-69"
review_set: base
date: 2026-10-17
---

## Summary

Ticket: IR-859. Checked the status flips against the planner rule on IR-859: an AC is marked implemented only where a tagged, passing test backs it, with the test cited, and AC-98 stays PLANNED. Every cited test exists at its cited line on 86d63d3, and the test file and `src/kani/generate/collection.rs` are byte-identical to merge fe20ecc. The `quire matrix` binders for AC-96, -97 and -99..-102 agree with the cited tags. AC-98 stays PLANNED and is untagged. The changed files validate. The cited Kani-lane test is `#[ignore]`d, and `make kani`/`make kani-gate` run it through `KANI_TEST ... --ignored` with the `kani_obligations_symbolic_domains` filter (Makefile:85-96).

Two medium defects remain. First, the AC-99 cell cites the IR-268 run on `a5221e1`. That commit is not in the repository, and IR-268's own comment says the integration test there exited 101 and was never rerun after the fix. The passing run is the close-out run on f43adca (the same tree as fe20ecc). Second, the AC-100 carve-out understates what is missing: the run evidence has no Kani, Rust or solver version field at all, and the carve-out wording names only the Rust version and the option vector.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The AC-99 cell cites "recorded IR-268 run on a5221e1 (Kani 0.68.0, CBMC 6.11.0, Cadical, unwind 8): healthy Verified 94 checks, mutant Falsified 93 checks". `a5221e1` is not a valid object in the repository. IR-268's 2026-10-16T05:41 comment says that on a5221e1 the outer integration test exited 101, and that no Kani rerun was made after the test-only fix 301e5896. The authoritative passing run is IR-268's close-out (2026-10-16T22:32): PR head f43adca, code 9c5f291, the same tree as merge fe20ecc. That run shows 1 passed, exit 0, 94 SUCCESS for the healthy claim and 93 SUCCESS / 1 FAILURE for the mutant, Kani 0.68.0, CBMC 6.11.0, CaDiCaL 3.0.0, unwind 8. Cite that run instead. | FR-015-AC-99 (spec/kani/functional/FR-015-bounded-kani-obligations.md:784) |
| FND-002 | medium | The AC-100 carve-out says only the recorded Rust version and option vector are unasserted. `KaniExecutionEvidence` (src/kani/run/execute.rs:246-283) has no Kani, Rust or solver version field: it records `arguments`, `unwind` and the solver kind, and nothing else of that kind. The Kani-lane test (:2080), which is tagged AC-99 and not AC-100, asserts only the solver kind and unwind 8, never a version. The Kani-version and solver-version parts are therefore unimplemented, not merely unasserted, and the row's "IMPLEMENTED for the collection family" then contradicts a collection clause that stays PLANNED. State the carve-out as: no run-evidence field for Kani, Rust or solver versions, and no test comparing the recorded `arguments` with `identity.options`; that clause stays PLANNED. Mirror this in tests.md :34 and :95 and in the TC-025 sentence. | FR-015-AC-100 (spec/kani/functional/FR-015-bounded-kani-obligations.md:785; spec/kani/matrix/tests.md:34, :95; spec/kani/matrix/TC-025-bounded-kani-obligations.md:69) |
| FND-003 | low | The AC-99 cell says "run by `make kani`/`make kani-gate`; kani lane: `make kani-gate`". The second clause repeats the first. Keep one. | FR-015-AC-99 (spec/kani/functional/FR-015-bounded-kani-obligations.md:784) |

## Verdict

The flips are mostly honest and well cited. The AC-96 and AC-97 rows match their tests clause by clause. The graph carve-outs on AC-100, -101 and -102 are correct, because every graph claim settles unsupported today. AC-98 is untouched and PLANNED. The AC-96 uniqueness-by-refusal note is disclosed honestly. Fix FND-001 and FND-002, which are spec-text changes only, before merge. Not mergeable at 86d63d3.
