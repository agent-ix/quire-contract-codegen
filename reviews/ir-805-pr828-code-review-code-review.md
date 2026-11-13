---
id: SR-17940486709564847047970
title: "IR-805 PR #828 code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #828 at 123b2daf, git diff origin/main...HEAD; IR-805 slice B; src/replay/state_clause/graph.rs, src/replay/state_clause.rs, src/kani/generate/graph.rs, src/kani/identity.rs, src/lib.rs, tests/it/kani_graph_private.rs, tests/it/kani_obligations_symbolic_graphs.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-805 (slice B: semantic mutants for FR-015-AC-106 with `qsl_replay` over the authored clause).
This is the code-review method with the rust-review lane folded in. I ran
`cargo test --lib kani_graph_private -- --include-ignored --test-threads=1` once, inside
`flock /tmp/cargo-slot.lock` and a 24G scope, at 123b2daf. All 5 tests passed: the four installed-Kani
runs and the non-Kani replay test, in 9.4 s. I did not run `make ci` or the whole `--test it`; the lead runs
those.

Checked and found sound:

- Verdict disagreement (brief question 1). Each source mutant changes the generated helper, and the
  authored clause stays the same. Slice A's healthy controls `verified("empty-0-0", ...)` and
  `verified("forced-self-loop", ...)` verify those same two claims. So when Kani falsifies a mutant and QSL
  says the authored clause holds on the decoded graph, the counterexample is spurious. That is the expected
  signal that the mutant was detected, and asserting `Inconclusive` with `DisagreementCause::Verdicts` and
  `Category::Success` is the right assertion. The negated claim is a claim mutant, not a harness mutant, and
  it correctly asserts `ReproducedWithEvaluatedWitness` with a violation. Whether the spec text says this
  honestly is covered in SR-17940486709564847047972.
- Decoding (brief question 3). Slots are read in the generated draw order: `object_count`, `start`,
  `target`, then each row's `degree_{row}` and `edge_{row}_{col}` (graph.rs:475-512).
  `decode_playback` checks the arity (witness.rs:363), so `slots[index]` cannot go out of bounds. The domain
  checks match the harness assumptions: count in `[1,N]`, `start < count`, a degree in `[lower,upper]` only
  for active rows, targets below the count, and uniqueness. The decoder also checks that the assertion
  message is the generated graph assertion, so the playback is "Falsified at the clause assertion".
- No invented values. The snapshot holds `count` objects with no fields, a closed `complete: true`
  population (the population is `extent: closed` and the count is the full draw), and one `complete: true`
  entry for the selected relationship that contains exactly the drawn edges. The other relationships are
  left out rather than invented. The `n{i}` keys are labels, not model values. The anchor and the
  population, type and relationship addresses come from the caller.
- Every new test carries `#[trace("TC-025", "FR-015-AC-106")]`. The ignore reason uses the established
  "real Kani qualification:" convention (same as `src/routed/mod.rs`'s private scalar Kani tests).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | The "wrong-role" mutant does not follow another role. It replaces `(edge as i64) < degrees[row]` with `(edge as i64) < 0`, so the closure follows no edge at all: a constant-false traversal. On its fixture (forced `{1,1}` loop, population 1) it draws the same graph `[[0]]` and gets the same verdict as the rejected-self-loop mutant, so it detects nothing the self-loop mutant does not already detect. TC-025 says a "substituted traversal mutant" does not qualify. The module doc admits it is an approximation, but the test name `real_kani_wrong_role_mutant_...` and the status text count it as AC-106's wrong-role mutant. | tests/it/kani_graph_private.rs:12-14,464-481 |
| FND-002 | medium | No test covers any refusal branch of `decode_falsification`: `PlaybackAlreadySupplied`, provenance mismatch, `HarnessMismatch` (clause), `AssertionMismatch`, `GraphWitnessNotInDomain` (count 0 or above N, start out of range, degree outside `[lower,upper]`, target out of range, repeated target on a unique end) and `GraphStartIsNotTarget`. The non-Kani test's doc says "a witness the harness could not have drawn is refused before QSL is asked", but its body never triggers a refusal. It sets `inputs.witness` directly and asserts that the clause holds. | src/replay/state_clause/graph.rs:132-243, tests/it/kani_graph_private.rs:483-525 |
| FND-003 | medium | `GraphReplayInputs::witness` is a public field (re-exported from `lib.rs`), and `new_graph_invariant` checks none of the `D(g)` facts that `decode_falsification` enforces (`start < objects`, every target below `objects`, uniqueness, degree bounds). A public caller can therefore replay a graph the harness never drew, or could not draw, and get a QSL verdict such as a reproduced violation that is credited as replay of a Kani counterexample. The non-Kani test depends on this bypass. Either validate the witness again in `new_graph_invariant`, or make the field private and add a crate-private test seam. | src/replay/state_clause/graph.rs:88-89,263-275, src/lib.rs:321 |
| FND-004 | low | Two replay paths have no test, with Kani or without: `forward == false` (the instance source and target are swapped) and a non-unique end, which emits duplicate `Instance` rows that QSL may or may not accept for a complete relationship entry. Every run in this PR uses forward, unique ends. | src/replay/state_clause/graph.rs:313-333 |
| FND-005 | low | The decode-time domain check takes `N`, `lower`, `upper` and `unique` from the generated harness's `identity`, not from the authored package's `select_population` and `relationship_end` projections. A defect in generator identity would therefore be admitted as "original-domain". This is acceptable for the semantic mutants, which never change the identity, but it is not the independent `D(g)` that AC-106 describes, and the doc comment should not suggest otherwise. | src/replay/state_clause/graph.rs:154-155,212-228 |

## Verdict

Not mergeable as is. The replay plumbing is sound and invents nothing, and the mutant tests assert the right
outcome: a verdict disagreement is how a harness mutant is detected, and all five tests pass under
installed Kani at 123b2daf. The blockers are FND-001, a "wrong-role" mutant that is a constant-false
traversal adding no detection power, credited as the AC-106 wrong-role mutant; FND-002, a refusal surface
with no tests behind a doc comment that claims refusal coverage; and FND-003, a public bypass of the decode
domain checks. FND-004 and FND-005 are low.

## Dispositions

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | fixed at 6b3940be: the wrong-role test is deleted and the module doc reads "There is NO wrong-role mutant. ... AC-106's \"another field or role\" clause is NOT evidenced." |
| FND-002 | fixed | fixed at 6b3940be: `decode_falsification_refuses_every_playback_the_harness_could_not_draw` drives hand-authored playbacks through every refusal (PlaybackAlreadySupplied, empty/over-N/negative count, start out of range, degree above upper and below lower, target out of range, repeated unique target, GraphStartIsNotTarget, AssertionMismatch, provenance Decode, HarnessMismatch); the false refusal claim in the non-Kani doc is removed. |
| FND-003 | fixed | fixed at 6b3940be: `witness: Option<GraphWitness>,` is private, set only by `decode_falsification` or the `#[cfg(test)] pub(crate) fn with_witness` seam; `GraphReplayInputs::new` and `fn witness(&self)` replace the public field. |
| FND-004 | fixed | fixed at 6b3940be: `the_reverse_direction_and_a_non_unique_end_reach_the_snapshot` asserts swapped source/target for `forward: false`, the same violation both ways, and two repeated `litter` instances admitted by QSL with the clause holding. |
| FND-005 | fixed | fixed at 6b3940be: doc now says the domain "is read from the harness identity ... this check does not re-derive it from the package, so it is a consistency check of the playback against the harness, not an independent domain oracle." |
