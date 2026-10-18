---
id: SR-12141
title: "IR-845 criterion-strength review: FR-021 remainders after the QSL application API boundary"
type: SpecReview
analysis: criterion-strength
scope: "agent-ix/quire-contract-codegen@8771d9ffb11e5570f3bd803ee3d5d1b7dfef0ada (PR #420, diff vs origin/peter/ir-833-qsl-inspection-boundaries a4ad399); spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: references
---

## Summary

Ticket: IR-845. This review asks whether each FR-021 criterion the PR touched can still fail once its QSL-owned-API clause becomes a boundary. The judgment is manual, from the spec text, the `quire matrix` bindings and the bound tests. Jev was not invoked.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-021-AC-1 | examined | Every requested `call` item receives exactly one scheduled or typed-refused disposition; a refused item contributes no instruction root, concrete symbol or location entry |
| FR-021-AC-2 | examined | For each function-application corpus vector, the oracle's outcome, admitted charge sequence and consumed counters equal a direct application independently assembled from the request with a fresh meter; today that application is the Contract Runtime `exact` port (historical), and CG names no QSL application API as its replacement (IR-845). |
| FR-021-AC-5 | examined | The schedule orders arity validation before per-argument validation, validates arguments in declared parameter order, then places `function.call` before body work |
| FR-021-AC-7 | examined | produces a flat SSA instruction table on a 512 KiB stack ... The schedule crate compiles. |
| FR-021-AC-10 | examined | refused as blocked on quire-spec-language#120, with no schedule root or concrete code for that function. No meter charge is admitted for it |
| FR-021-AC-14 | examined | The schedule crate declares `publish = false`, compiles without a Contract Runtime `exact` dependency, contains no charge amount or literal outcome/value |
| FR-021-AC-17 | examined | Each location record uses the request function binding and function-body ordering for its origin |
| FR-021-AC-19 | examined | The emitted schedule crate ... contains zero `.unwrap(`, `.expect(`, `unreachable!`, `panic!`, `todo!` and `unimplemented!` tokens |
| FR-021-AC-20 | examined | the target-neutral schedule preserves completed, undefined, refused, incomplete and checked-invariant result distinctions without folding one into another |
| FR-021-AC-32 | examined | Neither the schedule crate nor any concrete oracle requires `quire_contract_runtime::exact`, an alias, wrapper or compatibility layer; CG emits no owner-path oracle (IR-845). |
| FR-021 :236-242 | examined | AC-2 through AC-5, AC-8, AC-9, AC-14, AC-16, AC-17, AC-19 and AC-20 and their TC-031 vectors remain retained as target-neutral expected behavior; their present Contract Runtime `exact` witnesses are historical. |
| FR-021 Status :358-364 | examined | Its behavioral obligations are retained by AC-2 through AC-5, ... but their concrete execution and source assertions are outside CG's boundary (IR-845). |
| TC-031 step 12 | examined | Inspect the schedule crate for any `quire_contract_runtime::exact` dependency, alias, wrapper or compatibility surface. Assert none is required. CG emits no owner-path oracle (AC-32, IR-845). |
| src/oracle/function/mod.rs:151 | context_only | use quire_contract_runtime::exact as rt; |
| tests/it/exact_function_agreement.rs:42,86 | context_only | AC-2/4/5/9 and AC-17 tests compare the generated exact oracle with direct `rt` application and `PackageDeclarations::check` |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-021-AC-2 has a subject only while the historical Contract Runtime `exact` port exists. "The oracle's outcome, admitted charge sequence and consumed counters" need an executable concrete oracle. The PR now says CG emits none against a QSL API and names no replacement, while the Status section keeps AC-2's obligation and puts "their concrete execution" outside CG's boundary. Once Contract Runtime FR-275 deletes the port, AC-2 is either unfailable (no oracle to compare) or its bound tests (exact_function_agreement.rs) can no longer build. AC-4, AC-5's corpus half, AC-9's agreement vectors and AC-17's runtime-check test share this dependency. SR-12085 FND-001 restored AC-2 as Test "against whichever target IR-845 settles". IR-845 has now settled on no target, so FR-021 should say what AC-2 becomes after the port is deleted: retired to a boundary, re-homed as a schedule-level property, or held PLANNED under a named ticket. | spec/oracle/functional/FR-021-function-application-oracles.md:249, spec/oracle/functional/FR-021-function-application-oracles.md:236-242, spec/oracle/functional/FR-021-function-application-oracles.md:358-364 |
| FND-002 | low | FR-021-AC-32 covers "the schedule crate nor any concrete oracle", but TC-031 step 12 inspects only the schedule crate. The concrete oracle CG generates today, whose source the AC-8 test reads (`rt::CheckMode::Linked`), still names `quire_contract_runtime::exact` (src/oracle/function/mod.rs:151 and 2893). That is why AC-32 is Planned, but no step inspects it, so the Planned status has no procedure that would turn it Covered. Extend step 12 to the generator's emitted concrete oracle source. | spec/oracle/matrix/TC-031-function-application-oracles.md:126-128, spec/oracle/functional/FR-021-function-application-oracles.md:279 |

## Verdict

CHANGES REQUESTED (one medium, one low). None of the criteria whose PLANNED QSL-358/IR-349 clause became an IR-845 boundary is vacuous. AC-1, 5, 7, 10, 14, 19 and 20 each keep a schedule or claim-map property that a mutant fails, as their mutation rows state. AC-17 keeps "origin from the request binding", which a request-ordinal mutant fails. AC-9 and AC-26 keep their charge-point and branch/stop edges (see SR-12140 for wording). The weakness is in AC-2 (FND-001): the decision removes its only future target, and the spec does not say what replaces the historical port.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | IR-853 (planner ruling: cross-repo architecture; Peter, luna:IR and terra:QSL decide). At head 18832ac, AC-2, AC-4, the AC-5 corpus half, the AC-9 agreement vectors and the AC-17 runtime check each carry "PLANNED, blocked on IR-853: once Contract Runtime FR-275 deletes the `exact` port, ... target is decided by IR-853". Status adds that their text is testable as written while the port exists. No target is invented. |
| FND-002 | fixed | round 1, head 18832ac: TC-031 step 12 now inspects "the schedule crate and the concrete oracle CG emits today", and records that the current oracle still requires `exact`, so AC-32 stays Planned. |
