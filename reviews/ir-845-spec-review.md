---
id: SR-12140
title: "IR-845 spec review: FR-021 QSL application API lines become boundaries"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@8771d9ffb11e5570f3bd803ee3d5d1b7dfef0ada (PR #420, diff vs origin/peter/ir-833-qsl-inspection-boundaries a4ad399); spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md, spec/oracle/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: references
---

## Summary

Ticket: IR-845. Decision applied (terra:QSL:plan, relayed by titan:kani:plan): option 2. QSL exports no callable application API through qsl-replay, so every FR-021 line that targeted a QSL-owned application API becomes a boundary in the IR-833 style (cite IR-845 and FR-014-AC-47). The author went further than the lines the ticket lists and converted every `PLANNED (QSL-358/IR-349 ...)` clause in AC-1, 5, 7, 9, 10, 14, 16, 17, 19, 20 and 26, plus AC-8, AC-31 and AC-32, the description lines :41/:66/:126/:237, Dependencies and Status, and TC-031 steps 6, 7, 11 and 12 and Expected Results.

Completeness was checked by grepping FR-021, TC-031 and the oracle tests.md at the frozen head for `QSL-358`, `IR-349`, `eventual`, `owner API`, `when that API` and `when available`. There is one hit left: FR-021-AC-3 (FND-001). `quire matrix --scope .` at the head was run: AC-8, AC-16, AC-18, AC-31 and AC-32 report `method-without-symbol` (Inspection), AC-30 is `untagged` (PLANNED IR-511, pre-existing), and every other FR-021 AC is `tagged`. Nothing changed in the bindings of a Test AC.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-021 :38-46 | examined | QSL exports no callable application API through qsl-replay (IR-845), and QSL arch-lint T12-A bars CG from qsl-eval, so CG claims no QSL application API. |
| FR-021 :66-69 | examined | Dynamic `Evaluation.location`/`.losses` behavior is outside CG's boundary: CG claims no QSL application API (IR-845). |
| FR-021 :126-136 | examined | Concrete callable Rust is outside this output: CG claims no QSL application API (IR-845) |
| FR-021 :228-242 | examined | Concrete oracle emission and its owner result variants are outside CG's boundary (IR-845) |
| FR-021-AC-1 | examined | Concrete generated disposition is outside CG's boundary (IR-845). |
| FR-021-AC-3 | examined | The owner API origin mapping is verified when that API is available. PLANNED (QSL-358/IR-349 for origin mapping). |
| FR-021-AC-5 | examined | owner-path execution of that order is outside CG's boundary (IR-845). |
| FR-021-AC-7 | examined | Running a concrete oracle against a QSL evaluator is outside CG's boundary (IR-845) |
| FR-021-AC-8 | examined | The schedule marks only linked-package admission as applicable and never marks a kernel-mode package applicable. Concrete mode admission through a QSL application API is outside CG's boundary (IR-845). |
| FR-021-AC-9 | examined | The schedule preserves a `function.call` charge point and an incomplete stop before body work; the incomplete outcome at a denied charge is not applied; owner-path execution of it is outside CG's boundary (IR-845). |
| FR-021-AC-10 | examined | No meter charge is admitted for it; a runtime charge assertion against a QSL API is outside CG's boundary (IR-845). |
| FR-021-AC-14 | examined | Concrete owner-path emission is outside CG's boundary (IR-845). |
| FR-021-AC-16 | examined | No oracle makes a claim that those fields are populated: CG claims no QSL application API (IR-845). |
| FR-021-AC-17 | examined | the cross-check against an origin published by a QSL-owned checker is outside CG's boundary (IR-845). |
| FR-021-AC-19 | examined | no concrete owner-path oracle exists to check (IR-845). |
| FR-021-AC-20 | examined | Forwarding a QSL application API's owner result variants is outside CG's boundary (IR-845). |
| FR-021-AC-26 | examined | A denied left-operand charge stops the flat execution before the right operand charge; ... owner-path execution matching an equivalent shallow body is outside CG's boundary (IR-845). |
| FR-021-AC-31 | examined | No test name, trace tag, test output line or doc comment of the 100,000-node case, no TC-031 step and no oracle-matrix row claims, or presents as evidence, that a generated oracle compiled, ran on a 512 KiB stack or agreed with a shallow equivalent against a QSL application API |
| FR-021 mutation rows AC-4, AC-8, AC-16, AC-19, AC-20, AC-31 | examined | Count schedule-source compilation as execution evidence, or present an owner-path run as evidence without a QSL application API. |
| FR-021 Dependencies, Status, Out of Scope | examined | No QSL application API: QSL exports no callable application API through qsl-replay (QSL decision, IR-845), so this requirement targets none. |
| TC-031 Description, steps 6, 7, 11, Expected Results | examined | Inspect (AC-31, FR-014-AC-47): QSL exports no callable application API through qsl-replay, so CG emits and runs no concrete owner-path oracle (IR-845). |
| spec/oracle/matrix/tests.md FR-021-AC-31 row | examined | Covered by inspection; no 100,000-node test name, trace tag, output text, doc comment, TC-031 step or matrix row claims or presents as evidence a generated-oracle run against a QSL application API |
| spec/oracle/matrix/tests.md TC-031 registry row | examined | FR-021-AC-1, ..., FR-021-AC-24, FR-021-AC-34 |
| FR-014-AC-47 | context_only | No test name, trace tag, test output line or doc comment of every oracle-agreement corpus of FR-014, FR-018 and FR-021 ... claims, or presents as evidence, agreement with the QSL value authority |
| tests/it/exact_function_generation.rs:696-698 | context_only | /// Trace: FR-021-AC-7, FR-021-AC-25, FR-021-AC-27, FR-021-AC-28, FR-021-AC-29, TC-031. fn tc_031_ac7_admitted_hundred_thousand_call_nodes_generate_on_a_512_kib_stack |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-021-AC-3 still reads "The owner API origin mapping is verified when that API is available. PLANNED (QSL-358/IR-349 for origin mapping)." Under the IR-845 decision that API will not exist, so this clause is a permanent PLANNED leg naming a target the rest of FR-021 now says CG never claims, and it contradicts the oracle matrix row that lists AC-3 as Covered. Convert it like AC-17: the cross-check against a QSL-owned origin mapping is outside CG's boundary (IR-845). | spec/oracle/functional/FR-021-function-application-oracles.md:250, spec/oracle/matrix/tests.md:41 |
| FND-002 | low | FR-021-AC-9 now reads "the incomplete outcome at a denied charge is not applied". An outcome is not applied; the charge is. The old text was "returns the ... incomplete outcome at a denied charge without applying it" (the charge). Reword the schedule remainder to "a denied `function.call` charge branches to the incomplete stop and the denied charge is not applied", so the test intent matches the AC-9 mutation row. | spec/oracle/functional/FR-021-function-application-oracles.md:256, spec/oracle/functional/FR-021-function-application-oracles.md:302 |
| FND-003 | low | FR-021-AC-26's first sentence still states an execution claim ("A denied left-operand charge stops the flat execution before the right operand charge") after the word "eventual" was dropped. CG now runs no flat execution, so the sentence reads as a CG-owned runtime claim with no executor. The testable remainder is the second sentence (the schedule's branch/stop edges). Phrase the first sentence as a property of the schedule's edges. | spec/oracle/functional/FR-021-function-application-oracles.md:273, spec/oracle/matrix/TC-031-function-application-oracles.md:85-91 |
| FND-004 | low | The TC-031 registry row in the oracle tests.md lists the ACs TC-031 covers but omits FR-021-AC-31 and FR-021-AC-32. Both now have their own matrix rows with verification "TC-031 (Inspection)", and TC-031 steps 11 and 12 perform them. AC-25 to AC-30 and AC-33 are also missing (pre-existing). Add at least AC-31 and AC-32 to the row, and note AC-31's inspection status and AC-32's planned status in its summary. | spec/oracle/matrix/tests.md:100, spec/oracle/matrix/tests.md:49-50 |

## Verdict

CHANGES REQUESTED (one medium, three low). Apart from AC-3, the conversion is complete and true to the decision. Each converted Test criterion (AC-1, 5, 7, 9, 10, 14, 17, 19, 20, 26) keeps a real, testable schedule or claim-map remainder, and its binding in `quire matrix` is unchanged. AC-8 and AC-16 stay Inspection, which is correct. AC-31 is now an Inspection boundary performed by TC-031 step 11. Its oracle-matrix row is Covered, and that is consistent: the only 100,000-node test, `tc_031_ac7_admitted_hundred_thousand_call_nodes_generate_on_a_512_kib_stack`, claims generation and schedule compilation, not execution. The mutation rows for AC-4, 8, 16, 19, 20 and 31 match their reworded ACs. Nothing contradicts #416's boundary text: AC-18 and FR-014-AC-47 are untouched. FR-014-AC-47 is about the QSL value authority and AC-31 is about the QSL application API, so the two boundaries are distinct and do not overlap. Criterion-strength and scope-boundary findings are in SR-12141 and SR-12142.
