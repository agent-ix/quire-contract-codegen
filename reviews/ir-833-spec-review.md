---
id: SR-12085
title: "IR-833 spec review: QSL value-authority lines become Inspection boundaries"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@843588258cc236867caedfc34ef5683ea7b77076 (PR #416, diff vs origin/peter/ir-818-quantity-convert-spec); spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/TC-029-composite-equality-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md, spec/oracle/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: references
---

## Summary

Ticket: IR-833. Ruling applied (titan:kani:plan): CG claims no agreement with the QSL value authority anywhere (T12-A allows CG only `qsl-replay`); each such CG criterion or TC line becomes an Inspection boundary in the form of FR-014-AC-47, naming IR-842 (generated oracle against QSL) or IR-669 (RT against QSL). FR-021's QSL-owned application API lines (:41, :66, :126, :237, AC-8/31/32, :329) are routed to IR-845 and are out of scope.

Completeness of the conversion was checked by grepping the whole `spec/` tree at the frozen head for `quire_spec_language`, `quire-spec-language`, `QSL`, `qsl`, `three-way`, `value authority`, `QSL-owned`, `agree3` and `authority leg`. Every line the ticket and the scope addition list is converted: FR-014-AC-37 and its Dependencies notes, TC-024 :17, :221-226 and Boolean step 1, FR-018 :47, AC-2 and Upstream :457, TC-029 title, :19 and step 4 (`quire_spec_language::value`), oracle tests.md :28/:29, :30/:31 (FR-018-AC-2), :44/:46 (FR-021-AC-18) and the TC-029 summary row title, FR-021 :50, :176, :233, AC-2, AC-4, AC-18 and the mutation rows :289/:305, plus the TC-031 step 11 owner-package comparison. No remaining QSL value-authority agreement claim was found elsewhere: the other hits are relationship targets, blocker issue numbers (quire-spec-language#120/#121), replay-facade codes (AD-002, FR-024, FR-029, FR-030, FR-032, FR-033), the IR-845 API lines, and FR-021-AC-7/AC-17 (owner evaluator and checker-origin legs, which are API-target lines of the IR-845 family, not value-authority agreement claims). FR-031-AC-11 and spec/tests.md:17 are QSL's exemplar running CG's Kani bundle, not a CG value-agreement claim; the author's drop of them is correct.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-014-AC-37 | examined | A differential corpus covering every node FR-014-AC-35 names compiles against the runtime alone, and each oracle's outcome, admitted charges and consumed counters equal direct runtime execution. This criterion claims no agreement with the QSL value authority: that boundary is owned by quire-integration IR-842 ... |
| FR-014 Dependencies (QSL value authority, qsl-replay-only constraint) | examined | generated-oracle against QSL agreement is owned by quire-integration IR-842, not by CG. FR-014-AC-47 keeps CG's own artifacts from claiming it. |
| FR-014-AC-47 | context_only | No test name, trace tag, test output line or doc comment of the TC-187 agreement cases, no TC-024 step and no oracle-matrix row for a TC-187 vector names agreement with the QSL value authority. |
| FR-018 Purpose :44-50 | examined | Agreement with the quire-spec-language value authority is outside CG's boundary and not claimed by CG (FR-014-AC-47); generated-oracle against QSL agreement is owned by quire-integration IR-842. |
| FR-018-AC-2 | examined | ... equal to those of `quire_contract_runtime::exact::TypeEnvironment::check_equality` plus `CheckedEquality::evaluate` ... This criterion claims no agreement with the QSL value authority: that boundary is owned by quire-integration IR-842 (FR-014-AC-47). |
| FR-021 :50 | examined | Runtime outcome and charge amounts are not decided here, and CG claims no agreement with the QSL value authority on them (FR-014-AC-47; owned by quire-integration IR-842) |
| FR-021-AC-2 | examined | No test name, trace tag, test output line or doc comment of the function-application corpus, no TC-031 step and no oracle-matrix row names agreement of an oracle's outcome, admitted charge sequence or consumed counters with a direct application through a QSL-owned package. |
| FR-021-AC-3 | examined | ... any independent application a test builds for comparison is constructed from the request, never read back from generated code. |
| FR-021-AC-4 | examined | The schedule records ordered arguments without inventing a refusal result. Whether an eventual oracle's ... input refusals equal the QSL-owned ones is outside CG's boundary ... |
| FR-021-AC-18 | examined | No test name, trace tag, test output line or doc comment of the function-application corpus, no TC-031 step and no oracle-matrix row names agreement in outcome, refusal or charge order with the QSL-owned application authority. |
| FR-021-AC-32 | context_only | owner-path emission waits for QSL-358/IR-349 and preserves AC-2 through AC-5, AC-8, AC-9, AC-14 and AC-16 through AC-20 against the QSL-owned API. |
| FR-031-AC-11 | context_only | IMPLEMENTED IN CG (IR-596); THE QSL EXEMPLAR RUN IS PENDING. The clause that runs QSL's IT-011 and IT-010-SC-05 against this branch ... |
| TC-024 Description, :221-226, Boolean step 1 | examined | Agreement with the QSL value authority is outside CG's boundary and not claimed by CG (FR-014-AC-47); generated-oracle against QSL agreement is owned by quire-integration IR-842. |
| TC-029 title, Description, step 4 | examined | No comparison with the QSL value authority runs here (FR-014-AC-47). |
| TC-031 step 11 | examined | Compare outcomes, typed input refusals, charge sequences and consumed counters with an independently constructed shallow equivalent; no comparison with a QSL-owned package runs here ... |
| spec/oracle/matrix/tests.md rows FR-014-AC-35..37, FR-018-AC-2, FR-021-AC-2, AC-4, AC-18, TC-029 summary | examined | agreement with the QSL value authority is outside CG's boundary, owned by quire-integration IR-842, and not claimed by CG (FR-014-AC-47) |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-021 no longer states any execution-agreement obligation for the function-application oracle. The old AC-2 was the one criterion requiring the oracle's outcome, admitted charges and consumed counters to equal an independent application; it is now a pure Inspection boundary. FR-014-AC-37 and FR-018-AC-2 kept their direct-runtime half as Test; FR-021 kept nothing, so AC-3's "any independent application a test builds for comparison" and TC-031 step 11's "independently constructed shallow equivalent" refer to a comparison no criterion requires. Either keep a Test half (the eventual oracle equals an independent application built from the request, against whichever target IR-845 settles) or record explicitly that the agreement obligation waits on IR-845. | spec/oracle/functional/FR-021-function-application-oracles.md:249, spec/oracle/functional/FR-021-function-application-oracles.md:250, spec/oracle/matrix/TC-031-function-application-oracles.md:118-130 |

## Verdict

CHANGES REQUESTED (one medium). Conversion is complete: every ticket line and the tests.md:28 addition is converted, none is missed in the spec/ tree, and FR-031-AC-11 is correctly dropped as QSL's exemplar run, not a CG value-agreement claim. The IR-845 API lines are untouched; nothing in the diff contradicts them (FR-021 :50 now says outcome and charges are "not decided here" rather than naming the QSL application authority as decider, which is consistent with :41 and :126). FR-021-AC-3's rewording is sound: it keeps the request-not-emitted-source independence rule without presupposing a QSL comparison. The FR-018 Upstream edge to the QSL value authority is correctly removed. The `spec/oracle/matrix/tests.md` edits merge cleanly with #412 (git merge-tree auto-merges the file; #412 touches the FR-035 and TC-050 rows only). Integrity, evidence and scope-boundary findings are in SR-12086, SR-12087 and SR-12088.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | round 1, head ee9da7736b18: FR-021-AC-2 is Test (TC-031) again, requiring the oracle's outcome, admitted charges and consumed counters to equal a direct application independently assembled from the request, with a QSL boundary sentence (IR-842, FR-014-AC-47, FR-021-AC-18). |
