---
id: SR-1534
title: "Base spec review of quire-contract-codegen PR #283 (composite-equality bounded shadow and refinement)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@1f3025ea99c3fe1fa804ed43fce116e339f9c96d; spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/TC-039-bounded-proof-ceilings.md, spec/oracle/matrix/TC-029-composite-equality-oracles.md, spec/kani/matrix/tests.md, spec/oracle/matrix/tests.md, spec/assurance/AD-001-codegen-architecture.md"
review_set: subset
---
# Base spec review of PR #283

## Summary

Ticket: IR-241 (also IR-264). This is the base checklist review of the PR diff against CG `main`
e3d3ab4. The review set is base plus five analyses: integrity (SR-1535), EARS conformance
(SR-1536), criterion strength (SR-1537), scope boundary (SR-1538) and failure domain (SR-1539).
Every claim below was measured against CG `main`, Contract Runtime `main` 9597b43, QSpec FR-149
(quire-specification origin/main 2b2dd28) and Kani 0.68.0 with CBMC 6.11.0 on this host. The PR
text, the ticket text and the author's report were treated as untrusted claims.

The structure checks pass. Ids are well-formed and collision-free. `make spec` gives the same 6
warnings at base and at head, all in FR-024, with no errors. The strict coverage delta is 21 new
planned rows and none newly backed (`quire coverage`: 320/459 at base, 320/480 at head). The
matrix rows, the Traces To lists and the TC steps agree one for one.

The design is ADR-003's accepted Q1/Q2/Q3: a bounded shadow, a separate refinement obligation, and
no stubs. It is not a different decision.

## Measurements re-run by this review

- **Shadow probe (reproduced).** This review wrote its own shadow over a record
  `{ a: Bool, b: Option<Bool>, c: Int[-3,3]? }`. The shadow is straight-line Rust over `bool`,
  `Option<bool>`, `i64` and a three-state slot. It was checked against a closed-form FR-149
  expectation for verdict and pair count, with the `NotEqual` negation and one cover. Under Kani
  0.68.0 it verified in 0.16 s of solver time with the cover satisfied, at a peak RSS of about
  130 MB. A seeded "absent equals null" mutant was `FAILED` (falsified) in 0.25 s. This reproduces
  the author's 0.13 s / 145 MB figure in kind, on a slightly larger record that has an optional
  integer slot.
- **`plan_equality`-only probe through the real `Value` (consistent, not stronger).** Contract
  Runtime 9597b43 with `--features exact`, two concrete `none` `Option<Boolean>` values,
  `unwind(4)`, a 12 GB address-space cap and a 300 s limit on the whole `cargo kani` (compile
  included). It gave no verification result when the limit ended it. This agrees with the
  author's "did not finish in 300 s concrete none/none" and does not reproduce the ticket's
  "about 2.5 s".
- **The ticket's 2.5 s harness.** Contract Runtime history shows commit 40fd43c, "IR-31: rescope
  to a plan/evaluate property test; Kani tracked as IR-241", which abandoned the Kani attempt
  ("every harness attempt exceeded 8-16 GB"). No `plan_equality` Kani harness exists on Contract
  Runtime `main`. The ticket's statement that IR-31 "landed a plan_equality-level harness" is
  therefore not supported by the repository. The author's careful wording ("not shown to
  contradict") is appropriate.
- **Not reproduced:** the `check_equality` + `evaluate` probe at roughly 4 GB and still growing, and
  the symbolic 600 s `plan_equality` run. Both are long and memory-heavy, and the host's heavy-build
  lock was held by another lane's `make kani`. Neither is needed to support the design: the
  concrete `plan_equality` run already fails to finish.
- **Interplay.** There is no conflict with FR-031 (IR-602). FR-029 is the consumer FND-001 names,
  and IR-459's code PR (FR-024 and FR-029) is in flight. FR-028-AC-21's `MemoryExhausted` variant is
  what FR-029-AC-3 waits on, and the PR records that. IR-349 and IR-583 are handled by targeting the
  generated oracle's signature, with the change recorded as an Open Question. FR-018-AC-22 and
  FR-028-AC-15 still name Contract Runtime types (`Outcome`, `Meter`, `equality.pair`), so they will
  need rewriting if IR-583 changes the seam.

## Scope examined

- FR-028-AC-13 to FR-028-AC-23, FR-015-AC-69 to FR-015-AC-76, FR-018-AC-21 and AC-22, each with its Behavior statement and, where present, its mutation row (examined).
- TC-025 steps 29 to 36, TC-039 steps 11 to 21 and Expected Results 11 to 21, TC-029 steps 13 and 14 and the added Expected Results paragraph (examined).
- spec/kani/matrix/tests.md and spec/oracle/matrix/tests.md (examined).
- FR-028 Rationale, Open Questions and Checklist, and the AD-001 Risks bullet (examined).
- FR-029 (terminal map), FR-025, FR-018-AC-2, AC-7, AC-8 and AC-9, ADR-003 (context_only).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Proof strength never reaches the terminal value QSL consumes. FR-029, unchanged by this PR, maps `verified` with n SUCCESS checks to `Proved { success_checks: n }` without reading the proof subject or the refinement result. So a verified `bounded_shadow` harness, including one whose refinement settled `refinement_failed`, reaches QSL as `Proved`. FR-028-AC-17's guarantee that no `bounded_shadow` result reads as a production proof holds only inside CG's own enum. A falsified shadow harness has a second problem: FR-029 requires a replay settlement for every falsified outcome, but no replay path exists for a shadow counterexample, because FR-025 has no binding for a composite leaf (FR-028 Open Question 5). The PR must state how proof strength reaches FR-029 and the terminal record, at minimum as an Open Question and a Checklist item with FR-029 named downstream, so that a shadow result cannot be read as `Proved` over the production code. | FR-028-AC-17; spec/kani/functional/FR-029-run-outcome-terminal-record.md terminal table ("verified ... Proved"); FR-015-AC-75 |
| FND-002 | medium | The behaviours the proof does not exercise are mapped to native criteria that are unbacked today. FR-015-AC-76 maps `meter_limits` to FR-018-AC-9, `ill_typed_operands` to FR-018-AC-8 and `foreign_reference` to FR-018-AC-7, and all three are 🚧 Planned in spec/oracle/matrix/tests.md. It maps `charge_amounts_other_than_pairs` to FR-018-AC-2, which is ⚠️ partially covered: its agreement with QSL is delegated, and its runtime leg compares against the same runtime the oracle calls. AC-76 asks only that the identity name a covering criterion, not that the criterion be backed. Each exclusion therefore reads as covered when nothing covers it. The identity should record each covering criterion's matrix status, or the exclusions should stay open until those criteria are backed. | FR-015-AC-76; spec/oracle/matrix/tests.md (FR-018-AC-4, AC-5, AC-7 through AC-9 Planned; FR-018-AC-2 Partial) |
| FND-003 | medium | FR-015-AC-69 to AC-76 carry no mutation rows. FR-015 has never had a mutations table, but this PR applies the mutation record to FR-028 (11 rows) and FR-018 (2 rows) and not to the family's own eight criteria. Those are the criteria a test would most easily satisfy vacuously: AC-71's expectation, AC-72's budget and AC-73's refusal table. Add a mutation table for AC-69 to AC-76, or state why FR-015 is exempt. | spec/kani/functional/FR-015-bounded-kani-obligations.md FR-015-AC-69 to FR-015-AC-76 |

## Verdict

Changes requested. The PR is careful, in scope and faithful to ADR-003, and its measured claims
reproduce where this review could run them. Blocking before merge: FND-001 here and SR-1537
FND-001 (FR-015-AC-75 requires two equivalent mutants to be caught). The medium findings in this
file and in SR-1535, SR-1537, SR-1538 and SR-1539 should be fixed in the same round.

On the PR's size: one PR is reviewable, and this review covered it in one pass. A split would make
things worse, not better. FR-028-AC-13 to AC-23 are not truly family-agnostic (SR-1535 FND-002),
so merging FR-028 first would merge a contract whose only consumer is not yet in the tree, while
FR-015-AC-75 and FR-028-AC-22 cross-reference each other. Recommendation: keep it as one PR.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | FR-029-AC-17, the FR-029 Behavior refusal bullet, interface-001 `run_terminal_value` and TC-040 step 16 list three `shadow_proved_*` strengths plus `refinement_failed`. They omit `shadow_proved_refinement_inconclusive`, which FR-028-AC-17 and AC-24 add in the same commit. FR-029's first bullet keeps such a result from being `Proved`, but no row maps it, so a verified shadow harness whose refinement hit a ceiling has no defined output. FR-029's own "total map" rule therefore fails for that pair. Add the strength to FR-029-AC-17, the refusal bullet, interface-001 and TC-040 step 16 ("the four `shadow_proved_*` strengths"). | FR-029-AC-17; spec/kani/matrix/TC-040-run-outcome-terminal-record.md step 16; spec/core/functional/interface-001-codegen-api.md run_terminal_value; FR-028-AC-17; FR-028-AC-24 |
| FND-005 | low | The PR #283 description is stale. It says FR-028-AC-13 to AC-23, "TC-039 steps 11 to 21" and "TC-029 steps 13 and 14", and it does not mention FR-029-AC-17, FR-028-AC-24, FR-018-AC-23, TC-039 step 22, TC-029 step 15 or TC-040 step 16. Measured: 24 new planned rows (`quire coverage` goes from 320/459 to 320/483: FR-028 +12, FR-015 +8, FR-018 +3, FR-029 +1). The author should update the PR body. | PR #283 description |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed 3e0bc7c | FR-029-AC-17 and the FR-029 Behavior now map a verified outcome to `Proved` only when its strength is `production_proved`. A shadow strength or `refinement_failed` is refused with `TerminalPairError::NonProductionProof`, a falsified shadow harness with `ShadowCounterexample`, and neither gets a terminal value. The exception to one-record-per-item is stated, with an Open Question for QSL. interface-001, TC-040 step 16 and the matrix are updated. FR-030 needs no change: by its stated precedence, FR-029 governs every run this repository executed, and shadow harnesses are such runs. Gap: the fourth shadow strength is not covered, see New FND-004. |
| FND-002 | fixed 3e0bc7c | FR-015-AC-76 records each covering criterion's backing state (`backed` or `unbacked`) against the matrix, adds `declaration_reconstruction` covered by FR-018-AC-23, and does not offer an `unbacked` result as closing a family row. The Checklist carries backing them. |
| FND-003 | fixed 3e0bc7c | FR-015 gains a table 'Mutations FR-015-AC-69 to FR-015-AC-76 detect', with one non-vacuous row per criterion. |
| FND-004 | fixed 5833419 | FR-029 now has one strength table of six rows and a single wildcard-free match over FR-028-AC-17's closed set. `production_proved` maps to `Proved`. `shadow_proved_refinement_exhaustive`, `_sampled`, `_not_run`, `_inconclusive` and `refinement_failed` map to `NonProductionProof` with no terminal value. FR-029-AC-17 requires a test that the enumerated set equals FR-028-AC-17's, and it has a mutation row. interface-001 `run_terminal_value` and TC-040 step 16 list all six. These are the same six strengths FR-028-AC-17 names, including `inconclusive` from AC-24. |
| FND-005 | fixed 5833419 | The PR body now says 24 new planned ACs (FR-028 +12, FR-015 +8, FR-018 +3, FR-029 +1) and gives the TC steps. This matches the measured 320/459 to 320/483 rows. |
