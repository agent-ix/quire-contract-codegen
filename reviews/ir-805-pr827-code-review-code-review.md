---
id: SR-17940486709564847047967
title: "IR-805 PR #827 code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #827 at f1ef23ed, git diff origin/main...HEAD; IR-805 slice A; src/kani/generate/graph.rs, src/kani/generate/negotiate.rs, src/kani/generate/outcome.rs, src/kani/identity.rs, src/kani/run/harness.rs, src/lib.rs, tests/it/kani_obligations_symbolic_graphs.rs, tests/it/layout.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-805 (slice A, relationship-end graph claims). This is the code-review method with the
rust-review lane folded in. It covers the new `graph.rs` lowerer and renderer, how negotiation wires it in,
the new outcome and identity types, the runner's `Graph` arm and the new tagged test file. I did not
re-run Kani or the suite: the lead runs the whole suite, and every finding below follows from reading the
source.

Checked and found sound:

- No caller bound and no inferred population. The extent comes only from `select_population` on the typed
  `reaches_population` selector, whose minimum is always 0 per `CheckedSelectedPopulation`. The edge
  bounds, order and uniqueness come only from `relationship_end`. `request.unwind` and the memory ceiling
  only refuse; they never shrink a domain.
- Native carriage. Every slot is a `KaniPrimitiveType::I64` input. The oracle takes `i64` and `&[i64; N]`.
  No `rt::Integer` appears, and a test asserts this.
- The AC-107 location check runs before any AC-108 input: a missing path, a non-`state_clause` node or a
  non-reachability node returns `MissingClaim` before `reaches_population` is called (graph.rs:129-171).
  So a path naming a non-reaches application never yields `NotReachesApplication`.
- The closure starts from the selected edges, not from the identity relation, and uses a correct
  Floyd-Warshall order. Dead rows and dead edges are guarded by `count` and `degrees`, and every index is
  in bounds.
- `#[trace("TC","FR-AC")]` attributes are on every new test. One tag is wrong (FND-007).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | The two Kani controls credited as AC-106's required healthy cases, the `{0,3}` unique and `{0,3}` non-unique ends, assert tautologies: `reaches(self,self,kids) or not reaches(self,self,kids)`, which lowers to `(reachable) \|\| (!(reachable))`. The assertion holds whatever the closure, degree, target or uniqueness encoding is, so `Verified` is evidence of nothing beyond the harness compiling. Worse, slice A only accepts conditions that are a Boolean function of one application (`lower_condition`). A `{0,3}` end admits both reachable and unreachable graphs, so under this restriction every claim that verifies over such an end must be a tautology. Slice A therefore cannot produce a non-vacuous healthy control for the case AC-106 names. That needs a multi-application claim such as transitivity, which the lowerer refuses as `UnsupportedClaimShape`. | tests/it/kani_obligations_symbolic_graphs.rs:602-626, src/kani/generate/graph.rs:242,294-327 |
| FND-002 | high | The cover `object_count == 0 \|\| object_count == {n}` is satisfied by the empty population, which is always feasible. It witnesses no non-empty graph, no maximum population and no edge count `lower`/`upper`. If the assumptions are unsatisfiable for every `object_count > 0` (for example a unique end whose `lower` exceeds the population maximum), the harness still reports `Verified` with its cover satisfied, which is a vacuous proof. This defeats the intent of `HarnessSpec`'s `LiteralTrueCover` guard without tripping it. | src/kani/generate/graph.rs:561-564, src/kani/generate/spec.rs:474-480 |
| FND-003 | medium | Precondition and postcondition graph claims are accepted and rendered with invariant semantics. The harness never invokes the subject (`subject_path` appears only in the identity), assumes no sibling preconditions, and asserts a precondition universally. The resulting `Verified`/`Falsified` verdict means nothing for an operation clause. The collection family routes these kinds through the AC-38/39/43 operation-clause harness and refuses them otherwise. A test (`operation_clauses_draw_the_same_symbolic_graph_for_two_reference_operands`) locks in the current behaviour. | src/kani/generate/graph.rs:80-93,172,468-541, tests/it/kani_obligations_symbolic_graphs.rs:420-446 |
| FND-004 | medium | The Falsified control's "native check" is a separately authored DFS reachability written in the test, not `qsl_replay` over the authored clause. AC-106 explicitly forbids a separately authored reachability. The test is tagged FR-015-AC-106 and the PR body cites it as independently checked, but it cannot count as AC-106's native-oracle evidence. The playback decoder also depends on the console format (`split("vec![").skip(2)`) and on a hard-coded `3 + row * 4` layout. | tests/it/kani_obligations_symbolic_graphs.rs:646-715 |
| FND-005 | low | When the population maximum cannot be parsed into `u64`, the refusal reports invented values (`population_maximum: u64::MAX, edge_upper: 0`) instead of the authored maximum. The refusal also fires before the relationship-end read, so an unbounded edge with an unparseable population reports `DomainNotRepresentable` rather than `requires_bound`. | src/kani/generate/graph.rs:194-202 |
| FND-006 | low | The AC-100 identity test builds every changed variant from a different QSL source, so each one also changes the package lock. The test therefore cannot show that the identity records the population extent or the edge multiplicity in their own right; the package digest alone would make every `assert_ne!` pass. No variant changes the edge multiplicity alone, and nothing asserts that the ordered option vectors are recorded or equal to the launched arguments. | tests/it/kani_obligations_symbolic_graphs.rs:448-488 |
| FND-007 | low | `a_domain_beyond_the_unwind_or_the_memory_ceiling_refuses_without_reducing_it` is tagged FR-015-AC-98, but it asserts FR-015-AC-101's graph clause: a population or edge domain over the ceiling settles `unsupported` naming it, with no reduction. AC-101's graph clause is still marked PLANNED and has no tagged evidence as a result. | tests/it/kani_obligations_symbolic_graphs.rs:490-535 |

## Verdict

Not mergeable as is. The generator wiring is careful: no caller bound, native `i64` slots, correct
location-first ordering and a correct positive-length closure. But the installed-Kani evidence that the PR
and spec credit to AC-106 is vacuous. FND-001 covers the tautological claims and FND-002 the always-true
cover. FND-003 is an unguarded semantic gap for operation clauses. Fixing FND-002 means using a cover that
needs a non-empty, maximal draw, for example `object_count == {n}`. FND-001 needs either a non-tautological
healthy claim (which requires multi-application conditions) or a spec status that gives these runs no
healthy-case credit.

## Dispositions

| FND | outcome | reason |
|---|---|---|
| FND-001 | deferred | Multi-reachability (e.g. transitivity) claims are not built in slice A and stay PLANNED in FR-015-AC-106. At b12cd890 the AC-106 text, the TC-025 and tests.md rows and the test doc comments label the {0,3} runs tautological smoke runs with no healthy-case credit. The overclaim is withdrawn; the missing capability is stated as a named gap. |
| FND-002 | fixed | fixed at b12cd890: vec![Cover { |
| FND-003 | fixed | fixed at b12cd890: if kind != ObligationKind::Invariant { |
| FND-004 | deferred | qsl_replay agreement is slice B and stays PLANNED in AC-106. At b12cd890 the test comment and the AC-106 text both say the DFS check is NOT qsl_replay and is not this criterion's native-oracle evidence. The fragile playback decoding (split("vec![").skip(2), 3 + row * 4) is unchanged; that is acceptable for a smoke-level falsification check that earns no oracle credit. |
| FND-005 | fixed | fixed at b12cd890: .map_err(\|_\| Reason::DomainNotRepresentable { |
| FND-006 | fixed | fixed at b12cd890: let other_multiplicity = emitted(&narrowed.package, &[0]); |
| FND-007 | fixed | fixed at b12cd890: #[trace("TC-025", "FR-015-AC-101")] |
