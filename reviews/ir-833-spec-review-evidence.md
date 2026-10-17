---
id: SR-12087
title: "IR-833 evidence review: verification methods and matrix rows for the converted criteria"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen@843588258cc236867caedfc34ef5683ea7b77076 (PR #416); spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md, spec/oracle/matrix/tests.md, tests/it/exact_function_agreement.rs, tests/exact_function_support/agreement_cases.rs, src/oracle/function/mod.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: references
---

## Summary

Ticket: IR-833. Checked each converted criterion's verification method against what it now states, the matching oracle tests.md row, and the trace tags `quire matrix --scope .` binds to it at the frozen head. FR-014-AC-37 (Test, TC-024; untagged, matrix row 🚧 Planned) and FR-018-AC-2 (Test, TC-029; tagged by the composite-equality agreement cases, matrix row ⚠️ Partially covered) keep a Test method whose remaining claim is the direct-runtime agreement, and their matrix rows match. The FR-021 rows are where method, matrix and tags diverge.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-014-AC-37 | examined | ... each oracle's outcome, admitted charges and consumed counters equal direct runtime execution. This criterion claims no agreement with the QSL value authority ... \| Test (TC-024) |
| FR-018-AC-2 | examined | ... equal to those of `quire_contract_runtime::exact::TypeEnvironment::check_equality` plus `CheckedEquality::evaluate` invoked directly ... \| Test (TC-029) |
| FR-021-AC-2 | examined | No test name, trace tag, test output line or doc comment of the function-application corpus, no TC-031 step and no oracle-matrix row names agreement ... with a direct application through a QSL-owned package. \| Inspection (TC-031) |
| FR-021-AC-3 | examined | ... any independent application a test builds for comparison is constructed from the request, never read back from generated code. ... \| Test (TC-031) |
| FR-021-AC-4 | examined | The schedule records ordered arguments without inventing a refusal result. Whether an eventual oracle's ... input refusals equal the QSL-owned ones is outside CG's boundary ... \| Test (TC-031) |
| FR-021-AC-18 | examined | No test name, trace tag, test output line or doc comment of the function-application corpus, no TC-031 step and no oracle-matrix row names agreement in outcome, refusal or charge order with the QSL-owned application authority. \| Inspection (TC-031) |
| tests.md FR-021-AC-2 row | examined | TC-031 (Inspection) \| ✅ Covered by inspection; no function-application test name, trace tag, output text, doc comment, TC-031 step or matrix row names agreement with a QSL-owned application |
| tests.md FR-021-AC-4 row | examined | ⚠️ Partially covered; ... only `InputRefusal::WrongValueKind` is asserted -- `::DanglingReference` is structurally unreachable ... |
| tests.md FR-021-AC-18 row | examined | TC-031 (Inspection) \| ✅ Covered by inspection; boundary: agreement with the QSL-owned application authority is outside CG's boundary ... |
| tc_031_ac2_generated_oracle_agrees_with_direct_runtime_call | examined | Trace: FR-021-AC-2, TC-031. The generated oracle's `Outcome<Value>` and admitted charge sequence equal a direct `CheckedPackage::call` on an independently assembled package |
| tc_031_ac4_input_refusal_surfaces_unchanged_before_any_charge | examined | Trace: FR-021-AC-4, TC-031. `InputRefusal::Arity` and `::WrongValueKind` are returned by the generated oracle unchanged, before any charge |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Trace: `tc_031_ac2_generated_oracle_agrees_with_direct_runtime_call` (and the module-level `Trace: FR-021-AC-2, FR-021-AC-4, FR-021-AC-5, FR-021-AC-9` test) still bind FR-021-AC-2, and `quire matrix` lists both as AC-2's binders. They assert generated-oracle versus direct Contract Runtime agreement; AC-2 is now an Inspection that no test can verify. The binding is wrong, and the native agreement those tests assert has no criterion to bind to (see SR-12085 FND-001). The spec should say where that historical evidence now traces (or that it is untraced), and the retag is a code follow-up. | spec/oracle/functional/FR-021-function-application-oracles.md:249, tests/exact_function_support/agreement_cases.rs:197-203, tests/it/exact_function_agreement.rs:34 |
| FND-002 | medium | FR-021-AC-2 and FR-021-AC-18 are marked "Inspection (TC-031)" and "✅ Covered by inspection", but no TC-031 step performs that inspection (AC-16, the other TC-031 Inspection, is done in step 10). Read literally, the inspection also fails today: `tests/it/exact_function_agreement.rs:14`, `tests/exact_function_support/agreement_cases.rs:14` and `src/oracle/function/mod.rs:127` carry doc comments naming "three-way agreement with the QSL authority" for AC-18 (to disclaim it). Add an inspection step to TC-031 that states what it scans and what counts as naming agreement, and either mark the rows ⚠️ until the stale doc comments are rewritten or state that disclaimers are permitted. | spec/oracle/matrix/tests.md:42, spec/oracle/matrix/tests.md:46, spec/oracle/matrix/TC-031-function-application-oracles.md:107-117, tests/it/exact_function_agreement.rs:14-19 |
| FND-003 | medium | After the FR-021-AC-4 split the Test half is only "the schedule records ordered arguments without inventing a refusal result", yet the tests.md row's partial-coverage reason (only `WrongValueKind` asserted, `DanglingReference` unreachable), the mutation row ("Collapse distinct owner input refusals into one generic cause") and the bound test `tc_031_ac4_input_refusal_surfaces_unchanged_before_any_charge` all concern refusal forwarding, which the criterion no longer states. CG's own "before any execution charge" ordering, which is not a QSL-agreement claim, was also dropped. Either restate the CG-side forwarding property (refusals reach the caller unchanged and before any charge) as the Test half, or rewrite the matrix row, mutation row and binding to the schedule clause. | spec/oracle/functional/FR-021-function-application-oracles.md:251, spec/oracle/functional/FR-021-function-application-oracles.md:293, spec/oracle/matrix/tests.md:43, tests/exact_function_support/agreement_cases.rs:272-290 |

## Verdict

CHANGES REQUESTED (three medium). Keeping FR-014-AC-37 and FR-018-AC-2 as Test with a boundary sentence instead of new ids is sound on the evidence side: what each still claims (agreement with direct runtime execution) is testable, the existing composite-equality bindings assert exactly that, and the boundary sentence makes no claim of its own a Test would need to verify. That it relies on FR-014-AC-47 for enforcement is a scope question (SR-12088 FND-001). FR-021-AC-3 stays Test; its rewording keeps it falsifiable (the existing AC-3 binding in `exact_function_generation.rs` checks the request-derived identity) without presupposing a QSL comparison. The FR-021-AC-1..AC-3 matrix row split correctly removes AC-2 from the ✅ Test row.

## Dispositions
