---
id: SR-17940486709564847047968
title: "IR-805 PR #827 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #827 at f1ef23ed; IR-805 slice A; FR-015-AC-98, -100 (graph clause), -101 (graph clause), -105, -106, -107, -108 in the computed matrix (quire 0.36.2, quire matrix --scope .); tests/it/kani_obligations_symbolic_graphs.rs, src/kani/generate/graph.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-805, slice A. This is a planless gap analysis; plan completion was not assessed.
`quire matrix --scope .` on the PR head shows FR-015-AC-98, -105, -106, -107 and -108 all `tagged`, with
tests in `tests/it/kani_obligations_symbolic_graphs.rs`. Being tagged is not the same as being covered.
Each criterion's clauses were compared with what its tagged tests actually assert:

- AC-98: relationship-end harness generation is asserted, the `reaches_field` refusal is asserted, and the
  claim of no caller bound is supported. Its PARTIAL status matches the evidence.
- AC-105: the relationship-end domains `{0,1}`, `{0,3}` unique and non-unique, `{2,3}`, `{0,0}` and
  `{1,1}`, and population extents 0, 1, 3 and 4 are asserted on the rendered text. PARTIAL matches.
- AC-106: see the code-review artifact SR-17940486709564847047967, FND-001, FND-002 and FND-004. The
  status wording is covered in the spec-review artifact SR-17940486709564847047969.
- AC-107 and AC-108: findings below.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | FR-015-AC-107 is marked IMPLEMENTED, but four of its clauses have no test. (1) One node under two clauses gives two items with different identities. (2) A field-edge and a relationship-end reachability give separate items. (3) A collection claim settles `unsupported` naming the missing claim. (4) A clause with no reachability does the same. The tagged tests only cover paths inside a single relationship-end clause, the two-position rule and a refused sibling. | tests/it/kani_obligations_symbolic_graphs.rs:278-418, spec/kani/functional/FR-015-bounded-kani-obligations.md (AC-107 row) |
| FND-002 | medium | The FR-015-AC-108 status says "every branch reachable through a QSL-emitted, IR-admitted package is implemented and tagged". Only `PopulationUnavailable` and `NotReachesApplication` have AC-108-tagged tests. `UnsupportedClaimShape` is untested even though a real package reaches it: two distinct reachability applications in one condition, or a non-Boolean combinator around the application. `PopulationTypeMismatch`, `HarnessSymbolCollision`, `DomainNotRepresentable` and the edge `requires_bound` are also untested. The memory and unwind refusals the status cites are tagged AC-98, not AC-108. And `graph.rs` has no unit tests, so "covered only by the typed mapping in `graph.rs`" means inspection, not a test. | src/kani/generate/graph.rs:172-242,268-291, tests/it/kani_obligations_symbolic_graphs.rs:267-313,490-535 |
| FND-003 | medium | The claim that the `requires_bound` (unbounded population or end) branches and the `Stale`/`Foreign`/`NotPopulation`/`Missing` selections cannot be reached through a real package rests on prose and a fixture comment ("refused by QSL's checker before IR admission"). No test shows QSL refusing them. AC-97 set the precedent of a recorded QSL-rejection test (`qsl_rejects_a_missing_upper_bound_with_a_positive_minimum`) for exactly this kind of unreachability claim. | tests/it/kani_obligations_symbolic_graphs.rs:35-36, spec/kani/functional/FR-015-bounded-kani-obligations.md (AC-98, AC-108 rows) |
| FND-004 | medium | FR-015-AC-101's graph clause is still marked "stays PLANNED (IR-805)", yet slice A implements the graph unwind, memory and representational refusals with no domain reduction (graph.rs:375-411), and a test asserts them (mis-tagged AC-98). The matrix shows no graph evidence for AC-101, while AC-98 picks up credit for a clause it does not own. | src/kani/generate/graph.rs:362-411, tests/it/kani_obligations_symbolic_graphs.rs:490-535 |

## Verdict

Not mergeable as is. The AC-107 IMPLEMENTED status is not backed for four of its clauses. AC-108's claim
that every reachable branch is tagged is false for `UnsupportedClaimShape` and several others. The
unreachability statements need either QSL-refusal tests or a plain "Inspection only" label.
