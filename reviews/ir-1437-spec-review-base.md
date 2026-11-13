---
id: SR-17940486709564847047748
title: "IR-1437 PR #810 kani-mutations target specification review (base)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #810; IR-1437; spec/kani/functional/FR-055-kani-mutations-target.md; spec/kani/matrix/TC-069-kani-mutations-target.md; spec/kani/non-functional/NFR-006-real-kani-lane-gating.md (AC-12, ci statement); spec/kani/matrix/TC-045-real-kani-lane-gate.md (step 12, expected results); spec/kani/matrix/tests.md; spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-055
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-069
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/NFR-006
    type: reviews
---

## Summary

Ticket: IR-1437. Base checklist over the PR #810 diff (`git diff origin/main...HEAD`, head
c0d6a62): new FR-055 (`make kani-mutations`), new TC-069, NFR-006 statement plus AC-12
amendment, TC-045 step 12 and expected-results amendment, matrix and registry rows. Review set
`subset`: base plus failure-domain, integrity, ears-conformance and dependency (separate SR
files). No `AssuranceProfile` exists under `spec/`, so no profile applies. Spec-only PR: no cargo
or `make ci` run.

ID formats, matrix row and registry row are well formed. Every FR-055 AC maps to a TC-069 step
(AC-1 step 1, AC-2 step 2, AC-3 step 3, AC-4 step 4, AC-5 and AC-6 step 5, AC-8 step 6, AC-7
step 7), and each AC states a condition that can be observed failing. No hash, digest, pin or
tracking-record antipattern was introduced. No content was copied from quire-contract-runtime:
no code, mutation table, protocol id or schema; FR-055 paraphrases one rationale sentence and
reuses the four outcome words by design.

## Verdict

**FAIL (not mergeable).** Two high findings: the spec does not bound or scope the compute it adds
to every `make ci` run, and the enumeration of "every generated harness's mutant" is undefined so
FR-055-AC-1 passes vacuously with zero mutants.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Cost is unbounded: FR-055 puts real Kani over every generated harness's mutant into every `make ci`, under the host lock, with no mutant-set bound, no per-mutant or total timeout value, and AC-6 makes `make ci` fail on any machine without `cargo-kani`; the spec does not address this owner decision | FR-055-AC-3, FR-055-AC-6, NFR-006 |
| FND-002 | high | "Every generated harness's mutant" has no defined source (CG declares no mutant table; its mutants are ignored lane tests in tests/it), and no AC covers zero mutants, so AC-1 exits zero vacuously when nothing runs | FR-055-AC-1, FR-055 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-055-AC-1 | examined | `make kani-mutations` exists. Run with every generated harness's mutant, it injects each mutant into a scratch copy of the source, runs the owning proof through Kani, and exits zero only when every mutant is `pass`. The working tree is byte-identical before and after. |
| FR-055-AC-2 | examined | The target prints one line per mutant run, naming the harness, the mutant and its outcome, followed by a total. A mutant that was not run does not appear as run. |
| FR-055-AC-3 | examined | `make -n ci` includes the `kani-mutations` target. |
| FR-055-AC-4 | examined | When a mutant still verifies, the target prints that mutant as `fail` and exits non-zero. |
| FR-055-AC-5 | examined | Absent/duplicate anchor, non-compiling, timed-out or inconclusive mutant each give an outcome other than `pass` and a non-zero exit. |
| FR-055-AC-6 | examined | With no `cargo-kani` on `PATH`, every mutant is `unavailable` and the target exits non-zero. |
| FR-055-AC-7 | examined | Deliberately weakening one generated harness makes its mutant verify; the target reports `fail`, non-zero; restoring returns zero. |
| FR-055-AC-8 | examined | NFR-006-AC-12 and TC-045 step 12 state that `make -n ci` expands to no `kani-gate` command and no `kani` lane command other than `kani-mutations`. |
| TC-069 | examined | Steps 1-7 map to FR-055-AC-1..AC-8; steps 1-6 use stand-in `cargo-kani`, step 7 installed Kani in the Kani lane. |
| NFR-006-AC-12 | examined | `make -n ci` expands to no `kani-gate` command and no `kani` lane command other than `kani-mutations`. |
| TC-045 step 12 | examined | run `make -n ci` and assert no line contains `kani-gate` and no line contains `kani` other than `kani-mutations`. |
| spec/kani/matrix/tests.md FR-055 row | examined | FR-055-AC-1 through FR-055-AC-8, TC-069, PLANNED (IR-1437). |
| quire-contract-runtime scripts/check_kani_mutations.py | context_only | Reference only: pass/fail/malformed/unavailable, MINIMUM_KANI_MUTATIONS = 3, three hand-declared mutations. |
