---
id: "SR-1482"
title: "CG PR 269 spec review (integrity): depth removed from failed-record kind lists, version removed from the interface-001 dependency tuple"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@17f779983e55e025fde18969ad08ee51f59fbf22; spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/TC-029-composite-equality-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md, spec/core/functional/interface-001-codegen-api.md"
---

# SR-1482: CG PR 269 spec integrity review

## Summary

Ticket: IR-565 (primary), with IR-570 and IR-579. The spec diff makes seven one-line edits.

- FR-014's Behavior bullet and FR-014-AC-41 drop `depth` from the unrecognised-kind list.
- FR-018-AC-20 and FR-021-AC-23 drop `depth`. Both cite "the snake_case name FR-014 states", so
  they agree with FR-014.
- TC-024 step 3, TC-029 and TC-031 step 11 drop `depth` the same way.
- interface-001 `ReplayPackage::new` semantics drops `version` from the dependency-entry tuple.

Integrity checks run:

- Consistency: all six oracle documents list the same four kinds in the same order. The list
  matches IR 6fb6e97's `CheckedPackageLimit` without `Work` and `Bytes`.
- Completeness: no AC or TC row elsewhere in `spec/` still names `depth` as a lowering limit or
  `version` as a dependency-entry member.
- Atomicity: no AC was split or merged, no AC id changed, and the matrix rows in
  `spec/oracle/matrix/tests.md` for FR-014, FR-018 and FR-021 still cite the same ACs.
- Validation: `make spec` (quire 0.36.1) exits 0. Its warnings are in untouched documents
  (FR-017 line 152).
- interface-001: the merge resolution kept main's IR-465 / FR-016-AC-24 planned note on the
  `output` line and took the PR's version-free tuple on the `semantics` line.

## Verdict

Clean. The edits are the minimal ones the IR and QSL changes force, and they keep the six
oracle documents and interface-001 consistent with each other and with the code.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
