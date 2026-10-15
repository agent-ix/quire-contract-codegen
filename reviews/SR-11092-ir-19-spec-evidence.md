---
id: SR-11092
title: "IR-19 claimed-module proof gate evidence review"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-041-claimed-module-proof-gate.md, spec/kani/matrix/TC-056-claimed-module-proof-gate.md, spec/kani/matrix/tests.md, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md, spec/spec.md"
review_set: subset
---

## Summary

Ticket: IR-19. Each new AC names Test or Test and Inspection, and TC-056 describes concrete negative controls plus positive real-Kani runs. The matrix marks the controls Planned.

## Reviewed units

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-041-AC-1 | examined | A nonempty reviewed declaration is checked against the current compiled-module census and the merge-base declaration. An absent compiled module, a removed claim, and an empty list each name the claim or condition and fail; adding a compiled claim does not erase an older one. |
| FR-041-AC-2 | examined | With two claimed modules sharing one `src/lib.rs`, a SUCCESS location inside only the first and an UNREACHABLE check inside the second reports the second `unreached` and fails. A SUCCESS check inside each passes this census; a SATISFIED cover, unknown location or non-SUCCESS status cannot supply the missing check. |
| FR-041-AC-3 | examined | The current build's transitive expectation/subject reachability yields the shared-helper intersection; the gate fails when the checked-in generated evidence omits or adds a helper, and records an explicit empty set when none is shared. For an exact operation, the expectation uses QSpec vectors or a model with no `quire-exact` call. |
| FR-041-AC-4 | examined | Each claimed module and computed shared helper has a positive unmodified real-Kani proof and a source mutation that compiles, executes, and reports a failed proposition check. Deleting a control or making its mutant verify, fail to compile, time out, skip execution, or merely trip the dirty-tree guard fails the gate; evidence names the exact diff, command and failed check from this run. |
| FR-041-AC-5 | examined | A stubbed executor, predetermined outcome or build-only run produces no qualifying module evidence and cannot pass the gate. |
| NFR-006-AC-19 | examined | On a clean current candidate, `make kani-gate` runs FR-041's module census and real mutation controls in its one required lane, and a failure of any claim or shared-helper control makes the gate exit non-zero. An optional mutation target or saved earlier result cannot substitute. |
| TC-056 step 1 | examined | Check in a reviewed nonempty claim declaration for the generated modules the gate says it |
| TC-056 step 2 | examined | Run the unmodified proofs with the installed Kani backend and read the typed per-check |
| TC-056 step 3 | examined | From the current build, compute each expectation's and subject's transitive reachable |
| TC-056 step 4 | examined | Verify each unmodified claimed proof first. For each claimed module and each computed shared |
| TC-056 step 5 | examined | Invoke `make kani-gate` on a clean current candidate. Confirm it includes the module census |
| FR-041 description | context_only | When the repository claims proof coverage for a generated-harness module, its existing `make kani-gate` SHALL accept that claim only after a real Kani run discharges a proposition in the module and an executed mutation inside the module falsifies a proof check. |
| Kani matrix FR-041 row | examined | Planned (IR-19); the current real-Kani gate has no reviewed claim census, per-module SUCCESS floor, build-derived shared-helper control or current-run mutations per claim. |
| Kani matrix NFR-006 row | examined | Planned (IR-19); module and shared-helper controls must run inside the one required `make kani-gate` lane. |
| Kani matrix TC-056 row | examined | Planned (IR-19); current-run real Kani proof and mutation evidence is not yet implemented. |
| spec/spec.md trace row | examined | Real-Kani lane gating names NFR-006 and planned FR-041, with TC-045 and planned TC-056. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS: the proposed evidence methods fit the observable gate behavior; no stored verdict or stand-in executor can satisfy positive proof credit.

The review used CG candidate 2a4d503 over main 28a1d18 and the committed QSL ADR-011 §2.3. It reviewed the specification only; implementation and real-Kani results remain planned.
