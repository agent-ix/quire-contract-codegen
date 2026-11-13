---
id: SR-17940486709564847047971
title: "IR-805 PR #828 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #828 at 123b2daf, git diff origin/main...HEAD; IR-805 slice B; FR-015-AC-106, TC-025; src/replay/state_clause/graph.rs, tests/it/kani_graph_private.rs, tests/it/kani_obligations_symbolic_graphs.rs, Makefile kani lane"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-805 slice B. Plan completion: not assessed. `quire matrix --scope .` at 123b2daf lists
FR-015-AC-106 as `tagged`. Its binders are the five `kani_graph_private.rs` tests (four ignored, one not)
and slice A's four ignored `verified` runs. For each AC-106 clause I compared the tagged evidence with the
criterion text. These clauses are covered by real evidence for the relationship-end form: the zero-length
mutant, the rejected-self-loop mutant and the negated claim, each `Falsified` under installed Kani, with the
decoded graph replayed through `qsl_replay` over the authored clause. I re-ran them and all pass. Still
pending, and honestly stated as pending: the field-edge form (slice D), the domain controls (slice C,
IR-939) and the non-tautological `{0,3}` healthy claim. AC-106 correctly stays PARTIAL.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | The AC-106 clause "one following another field or role" has no faithful evidence. The test tagged for it is a constant-false traversal (SR-17940486709564847047970 FND-001), and the fixture domain declares a second role, `empty`, which the harness never draws. The matrix shows AC-106 as `tagged`, so the gap is visible only in the prose. The test name should say what it does (for example "drops every edge"), or the test should not be credited as the role mutant. | tests/it/kani_graph_private.rs:464-481, spec/kani/functional/FR-015-bounded-kani-obligations.md:951 |
| FND-002 | medium | The PR removes the only graph `Falsified` control from the gate lane: slice A's `real_kani_unsatisfied_claim_is_falsified_with_an_in_domain_playback` in `tests/it/kani_obligations_symbolic_graphs.rs`, inside `make kani-gate`'s `--test it` filters. The replacements are lib tests (`#[path]` in `src/lib.rs`). `make kani-gate` (`--test it ... $(KANI_FILTERS)`) never runs them, and `make ci` skips ignored tests. So no gate reproduces AC-106's slice B evidence, although the AC-106 row calls these tests "Kani lane". The spec should either name the lib invocation as the qualification command or move the tests into the gate lane. | tests/it/kani_obligations_symbolic_graphs.rs:960-1033 (removed), Makefile:111-113, src/lib.rs:19-21 |
| FND-003 | low | The mutant pairs rely on healthy `Verified` controls from a different fixture: slice A's `document()` with six relationships, against slice B's two-relationship `domain_document()`. The unmutated slice B harness is never run on its own fixture. The QSL disagreement shows each counterexample is spurious, so detection holds, but no test shows that the healthy harness on this fixture verifies. | tests/it/kani_graph_private.rs:62-111, tests/it/kani_obligations_symbolic_graphs.rs:951-961 |

## Verdict

Not mergeable as is. The evidence is real and it re-ran green. However, one AC-106 mutant clause (another
role) is credited to a test that does not exercise it (FND-001), and the slice B evidence sits outside every
gate after the gate-lane `Falsified` control was deleted (FND-002). AC-106 is correctly kept PARTIAL.
