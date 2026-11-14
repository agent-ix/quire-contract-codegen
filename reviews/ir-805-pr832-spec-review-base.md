---
id: SR-17940486709564847047981
title: "IR-805 PR #832 spec-review base"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #832 at ec0646b1, git diff origin/main...HEAD; IR-805 slice D; spec/kani/functional/FR-015-bounded-kani-obligations.md (AC-98, AC-105, AC-106, AC-107, AC-108 and prose), spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-805 slice D. Base spec-review of the status edits for the field-edge
`quire.op.model.reaches_field` form. I read every changed sentence with a word diff against
origin/main and checked each status claim against the committed `Cargo.lock` (IR c8dca8c6,
QSL 434869e8). On that lock the IR accessor `reaches_population` refuses any application whose
identity is not `quire.op.model.reaches`
(`quire-contract-model/src/checked_package/v2/relationship_end.rs:71`), so every field-edge claim
settles `PopulationAccessorRefused { NotReachesApplication }` before CG's field-edge code runs.

`quire validate` on the three files: no errors. The only warning (AC-128 obligation shape, line
973) predates this PR. The diff adds no revision pins, hashes or digests to the prose. The `#616`
and `#654` mentions sit on rows that were re-flowed, not added.

## Verdict

**PASS with findings (no high).** The headline honesty requirement is met:

- AC-98, AC-105 and AC-106 are left PARTIAL, not delivered.
- AC-106 says the installed-Kani verdicts were measured against QSL-1904 composed onto the
  checkouts the run used, before it reached QSL main, and are PARTIAL until re-run.
- The prose says the field-edge generation tests pin the typed refusal on QSL main.
- AC-106 records the wrong-field clause as evidenced only by the multiplicity claim, and gives the
  reason: the harness draws one edge.
- AC-106 lists the field-edge domain oracle, the Set/Bag/OrderedSet runs and the `{0,3}` run as
  not delivered.

Two status sentences still describe behaviour of a composition that is not the committed one, and
one evidence citation points at a test that is not tagged to the AC it supports.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-98's "Remaining gap" sentence says a package without the population link "settles `unsupported` naming `PopulationUnavailable`, and that is every package QSL main emitted". AC-108 likewise cites `field_edge_domains_use_the_population_and_the_member_type_multiplicity` as the test for "`reaches_field` with no population link settles `PopulationUnavailable`". On the committed lock that never happens. IR c8dca8c6 refuses `reaches_field` in `reaches_application`, so the observed reason is `PopulationAccessorRefused { NotReachesApplication }`. `PopulationUnavailable` for a field edge only appears with an IR that has #1020/#1021, and that IR does not build with QSL main. Scenario: a reader takes AC-108's `PopulationUnavailable` branch as tested evidence, but no test run from this tree produces it. Fix: say that on the committed lock the refusal is `NotReachesApplication`, and that `PopulationUnavailable` is the expected reason once IR with the field-edge accessor is pinned (untested here). | spec/kani/functional/FR-015-bounded-kani-obligations.md:943, :953 |
| FND-002 | low | AC-106 says the verdicts were measured "against QSL's population link composed onto the QSL and IR checkouts the run used". It does not say that the six `real_kani_*` controls bound to AC-106 cannot pass on the committed lock: `harness()` panics because the generator refuses. The matrix still credits AC-106 with them. "PARTIAL until re-run" covers the status, but a reader running `cargo test --lib kani_graph_field_private -- --ignored` on this tree gets six failures with no warning. Fix: add one clause saying they fail by design on a lock without QSL-1904 and the field-edge IR accessor. | spec/kani/functional/FR-015-bounded-kani-obligations.md:951; tests/it/kani_graph_field_private.rs:6-9 |
| FND-003 | low | AC-108 lists `field_edge_domains_use_the_population_and_the_member_type_multiplicity` "and its siblings" under "Test (... AC-108-tagged)". That test carries `#[trace]` for AC-98 and AC-105 only, so it is not AC-108-tagged and the computed matrix does not bind it to AC-108. Fix: tag it to AC-108 too, or move the citation out of the AC-108-tagged list. | spec/kani/functional/FR-015-bounded-kani-obligations.md:953; tests/it/kani_obligations_symbolic_graphs.rs:376-379 |

## Examined scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-015-AC-98 | examined | Slice D adds the field-edge form ... Remaining gap: a package whose `reaches_field` application carries no population link settles `unsupported` naming `PopulationUnavailable` ... |
| FR-015-AC-105 | examined | slice D renders from the member type through the same draw ... on QSL main they pin the typed refusal instead and the field-edge draw is PARTIAL. |
| FR-015-AC-106 | examined | Slice D (IR-805), field-edge form, installed Kani 0.68.0 ... PARTIAL until they are re-run against QSL main ... Not delivered: the field-edge domain conformance and completeness oracle ... |
| FR-015-AC-107 | examined | Slice D: a field-edge reachability is located like any other ... refused by location before any population or edge input is read |
| FR-015-AC-108 | examined | a `reaches_field` with no population link settles `PopulationUnavailable` ... an IR whose `reaches_population` predates the field-edge form refuses `NotReachesApplication` ... |
| FR-015 prose (lines 116, 128, 149, 164, 213) | examined | graph clauses are PARTIAL: the relationship-end form ... and the field-edge form ... are both implemented |
| TC-025 | examined | Slice D delivers the field-edge healthy and mutant controls (... they run only against a QSL that links `reaches_field` to its population, QSL-1904). |
| tests.md | examined | the field-edge installed-Kani and replay controls ... ran against QSL's `reaches_field` population link (QSL-1904) before it reached QSL main, so they are PARTIAL until re-run |

## Dispositions

Round 1 (head 91b274b2). `quire validate` of FR-015: no new warnings. The spec diff adds no revision
pins or hashes.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AC-98 and AC-108 now say that with the Contract IR in the lock the claim is refused `NotReachesApplication`. With an IR that accepts the form and a QSL without the link, the reason is `PopulationUnavailable`, "which those tests do not yet pin". No revision pins. |
| FND-002 | fixed | AC-106 now says: "On a package without QSL's population link these controls stop at generation with a message naming the missing link (QSL-1904), not a run result." |
| FND-003 | still-open | The AC-108 row still lists `field_edge_domains_use_the_population_and_the_member_type_multiplicity` "and its siblings" under "Test (... AC-108-tagged)". The test still carries `#[trace]` only for AC-98 and AC-105 (tests/it/kani_obligations_symbolic_graphs.rs:383-386), so the matrix does not bind it to AC-108. Fix: one `#[trace("TC-025", "FR-015-AC-108")]` line, or move the citation out of the AC-108-tagged list. |
