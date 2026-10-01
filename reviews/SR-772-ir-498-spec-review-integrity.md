---
id: "SR-772"
title: "CG PR 226 spec review (integrity, with EARS): FR-018 member type node clause"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@ea3a8985efa9e2381cadbde6075486ed3a76b951; spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/matrix/tests.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-018
    type: references
---

# SR-772: CG PR 226 spec review (integrity, with EARS)

## Summary

Ticket: IR-498. This review applies spec-review with the integrity sub-analysis. The EARS
sub-analysis is folded in, because the change is one new paragraph. The other sub-analyses do
not apply: no domain object, relationship or dependency edge changed.

Units examined:

- **The new FR-018 §Inputs paragraph** (lines 100-105).
- **The FR-018-AC-14 statement**, read for context: bounded_domain nodes read by name.
- **The FR-018 Mutations table.**
- **The tests.md FR-018-AC-2 row edit.**
- **SHA check.** No SHA remains in any spec or matrix line the PR touches.
- **Gate.** `make spec` exits 0. It reports only the 3 grammar warnings that origin/main
  already has (FR-017:159, and FR-014:278 twice), none for FR-018.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | medium | The new clause adds generator behaviour that no criterion owns. A member type may be a `bounded_domain` node, it is read from its own bound only, and a non-scalar base is refused. There is no acceptance criterion, no row in the Mutations table ("a criterion without one is not written"), and no matrix or tests.md row. "Spec first" therefore produced prose that nothing traces. The change needs an AC (for example FR-018-AC-16) with its mutation (reading the union of sibling bounds) and a matrix row whose status is honest (Partially covered, text base only). | spec/oracle/functional/FR-018-composite-equality-oracles.md:100-105; spec/oracle/matrix/tests.md:28-31 |
| FND-002 | low | The clause is normative but has no `shall` and no EARS shape, so the grammar lint does not see it. It also leaves out the refusal the code implements: a `bounded_domain` member type over a non-`scalar_type` base, such as QSL's `collection_bounds` over a collection node, is refused as unsupported. And it does not say what happens when a bound's form does not fit its base (see SR-770 FND-001). | spec/oracle/functional/FR-018-composite-equality-oracles.md:100-105 |

## Verdict

The matrix edit is correct and SHA-free. The FR-018 addition is a sensible description of
the encoding, but as written it is not a requirement anything can trace. Fix FND-001 in this
PR. FND-002 can be fixed in the same edit.

## New findings (disposition pass 1)

Reviewed at 9e564e7.

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-003 | medium | The new tests.md FR-018-AC-16 row justifies the untested float-base refusal with "Contract IR refuses an equality over any IEEE-bearing type at admission, before this generator runs". Measurement contradicts this. IR 968ba9b8 admits both a tuple equality with a QSL-shaped `float_rounding` member (scratch probe) and the corpus's `E_NESTED_IEEE`, whose `OperatorIneligible` refusal comes from CG's own `check_equality` (FR-018-AC-6). The matrix status rests on a false claim. The new Behavior clause also conflicts with AC-6 for QSL-shaped floats (see SR-770 FND-004). | spec/oracle/matrix/tests.md:30; spec/oracle/functional/FR-018-composite-equality-oracles.md:145-151 |

## Dispositions

| FND | outcome | sha/reason |
|-----|---------|------------|
| FND-001 | fixed | 9e564e7: FR-018-AC-16, its Mutations row, a tests.md row (Partially covered), the TC-029 trace list and TC-029 step 8 were added. |
| FND-002 | fixed | 9e564e7: two EARS Behavior clauses (When ... shall / If ... then ... shall) now state the non-scalar-base refusal and the form-mismatch refusal. |
| FND-003 | fixed | 82b669f: the tests.md AC-16 row no longer claims IR refuses IEEE equality at admission. It states the real residual (the form-mismatch refusal is exercised over an integer base only). The Behavior clause moves float to the `float_rounding` form and defers to AC-6. AC-16, its Mutations row and TC-029 step 8 cover the float case. |
