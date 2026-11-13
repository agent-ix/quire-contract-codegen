---
id: SR-17940486709564847047969
title: "IR-805 PR #827 spec-review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #827 at f1ef23ed; spec/kani/functional/FR-015-bounded-kani-obligations.md (graph status paragraph, Disposition sequence for a graph claim, AC-98, AC-100, AC-101, AC-105 to AC-108), spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md; IR-805"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-805, slice A. The PR changes the evidence-status text of FR-015-AC-98, -100, -105 to -108 and
the graph paragraphs of TC-025 and the test matrix. This review asks whether each status is honest given
the code and the executed Kani verdicts, and whether the status texts agree with each other. AC-98 and
AC-105 (PARTIAL, with the field-edge form in slice D) are stated honestly. The PARTIAL wording on AC-106
and the ordering of AC-107 before AC-108 are correct as structure. The defects are in what the credit
claims.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | FR-015-AC-106's PARTIAL text credits "the healthy half for the relationship-end form" to installed-Kani `Verified` runs "with the cover satisfied over a `{0,3}` unique and a `{0,3}` non-unique end". Both runs assert the tautology `reaches X or not reaches X`, and the cover is satisfied by the empty population (SR-17940486709564847047967 FND-001 and FND-002). So these runs back no healthy-case behaviour. The `{0,0}` and forced `{1,1}` runs do carry meaning: positive-length search and the self-loop. But the `Falsified` run is checked against a separately authored reachability, not `qsl_replay`. The status should say the `{0,3}` runs are tautological smoke runs that earn no healthy-case credit, or the healthy claim must be one that can fail. | spec/kani/functional/FR-015-bounded-kani-obligations.md (AC-106 row), spec/kani/matrix/tests.md (IR-805 row) |
| FND-002 | medium | The status texts contradict one another. (1) The FR-015 graph paragraph says "AC-100's graph clause, AC-107 and AC-108 are implemented for every branch a real package reaches", but the AC-108 row says PARTIAL. (2) "Disposition sequence for a graph claim" still says "Real-package graph proofs remain Planned pending CG integration and QSL-820 emission". (3) TC-025 declares the graph identity implemented and then keeps "the graph-family clause remains planned". (4) The tests.md AC-96 to AC-102 row still says "the graph clauses stay planned (IR-805)" for AC-99/100/101. (5) AC-101's graph clause "stays PLANNED" although it is implemented. | spec/kani/functional/FR-015-bounded-kani-obligations.md:163-166,212-215, spec/kani/matrix/TC-025-bounded-kani-obligations.md:210, spec/kani/matrix/tests.md:49 |
| FND-003 | medium | FR-015-AC-108 should say, branch by branch, which evidence backs it. Today it lumps the `requires_bound` and `Stale`/`Foreign`/`NotPopulation`/`Missing` branches under "covered only by the typed mapping in `graph.rs`", where no test exists, and AC-98 calls the same branches "unit-unreachable and unbacked". The text should mark each one Test (real package), Inspection only with the QSL refusal that makes it unreachable, or untested. The disposition sequence also never places `UnsupportedClaimShape` or `PopulationTypeMismatch`. The code runs population and end refusals before the shape check, so an unexpressible claim with no population reports `PopulationUnavailable`. The spec neither requires nor forbids that order. | spec/kani/functional/FR-015-bounded-kani-obligations.md:202-215 (AC-108 row), src/kani/generate/graph.rs:172-242 |
| FND-004 | medium | FR-015-AC-105 says only that "the harness asserts the admitted clause condition". It does not define what a precondition or postcondition graph claim means: no subject call, no sibling-precondition assumption, no totality or satisfiability split, all of which AC-99 defines for collections. The code accepts both kinds with invariant semantics (SR-17940486709564847047967 FND-003). | spec/kani/functional/FR-015-bounded-kani-obligations.md (AC-105 row) |
| FND-005 | low | FR-015-AC-105 requires that "`N = 0`, the maximum, and edge counts `lower` and `upper` are each drawn", but does not say how that is evidenced. `HarnessSpec` allows exactly one cover, and the one chosen is satisfied by `N = 0` alone. The criterion should name the cover, or the separate controls, that show each endpoint is reachable. | spec/kani/functional/FR-015-bounded-kani-obligations.md (AC-105 row), src/kani/generate/graph.rs:561-564 |

## Verdict

Not mergeable as is. AC-106's healthy-half credit rests on tautological runs. AC-107's IMPLEMENTED status
is untested for four of its clauses (SR-17940486709564847047968 FND-001). The FR, TC and matrix statuses
contradict each other. AC-108 should state its branch evidence explicitly. The answer to the brief's
question is yes: the AC text should say which branches have no real-package test.
