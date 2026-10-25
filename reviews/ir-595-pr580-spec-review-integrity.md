---
id: SR-580002
title: "IR-595 PR 580 checked-closure facade re-point integrity analysis"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@08c68e3ac71b47e1fedae1c017891ac150053a89; spec/oracle/functional/FR-021-function-application-oracles.md, spec/assurance/AD-004-cg-crate-layout.md, spec/oracle/matrix/TC-031-function-application-oracles.md"
---

## Summary

Ticket: IR-595. PR: quire-contract-codegen#580 at 08c68e3a. This pass checks completeness,
consistency and AC-to-TC coverage of the re-pointed FR-021 criteria and the new Behavior bullet,
AD-004 and TC-031 step 12b.

## Examined scope

- FR-021-AC-32 to TC-031 12b: examined. 12b covers the named entries and keeps
  `qsl_replay::replay` in the separate Kani witness test.
- FR-021-AC-35 to TC-031 12b: examined. The single acquisition and the seven inputs are
  covered. The added acquisition-failure clause is not (FND-001).
- FR-021-AC-36 to TC-031 12b: examined. The owner/wire-id derivation, the lookup and the
  registration with `selection.signature()` and `Admission::Accepted` are covered.
- FR-021-AC-37 to TC-031 12b: examined. The handle mismatch, the lookup-refusal retention and
  `WrongAcquisition` at register and Begin are covered. The CG-side result type is unstated
  (FND-002).
- New FR-021 Behavior bullet: examined against the Outputs claim map and AC-37 (FND-002).
- AD-004 seam paragraph against FR-021 Description and Status: examined. They agree on the
  PLANNED state, the #1483 candidate status and the oracle/replay role split.
- Unchanged ACs (AC-1 to AC-31, AC-33, AC-34): context only. The diff leaves them
  byte-identical.
- `quire validate --scope .` over the three files: one `ac:non-canonical-shape` warning at
  FR-021 line 330 (AC-36), recorded in SR-580003. No errors.

## Verdict

CHANGES REQUESTED. The re-pointed criteria agree with AD-004 and the Status paragraph, and
TC-031 12b traces each AC. Two medium gaps remain. AC-35 adds an acquisition-failure obligation
with no TC step. The facade outcomes that are not accepted have no named CG disposition.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-35 newly asserts "Acquisition failure remains `AdmittedRefusal` and yields no handle", with verification Test (TC-031). TC-031 step 12b has no acquisition-failure vector: no malformed source or limit refusal through `acquire_checked_closure`, and no check that the item gets no handle, registry or charge. The strengthening is untraced. | spec/oracle/functional/FR-021-function-application-oracles.md:329; spec/oracle/matrix/TC-031-function-application-oracles.md:144-200 |
| FND-002 | medium | The new Behavior bullet requires `Ok(Admission::Accepted)` before Begin. It does not say which CG claim-map disposition an item gets when registration returns `Ok(SignatureMismatch \| DuplicateBinding \| CallableNotAdmitted \| RegistrySealed)` or `Err(SelectionRefusal)`, or when Begin returns `ApplicationBeginRefusal::Call(CallFailure)`. AC-37 says such a refusal "retains its exact facade type" but names no CG output field or variant to carry it, while the Outputs section lists only `ExactFunctionRefusal`-typed refusals. Two implementers would map these differently. | spec/oracle/functional/FR-021-function-application-oracles.md:277-289,331; spec/oracle/functional/FR-021-function-application-oracles.md:157-162 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | The new claim-map variants embed `CallableLookupRefusal`, `SelectionRefusal`, `ApplicationBeginRefusal` (wrapping `quire_eval::CallFailure`) and `quire_eval::Admission` as fields of `ExactFunctionRefusal`. That enum derives `Serialize` with `#[serde(tag = "code")]` (src/oracle/function/mod.rs:1268-1270), and the claim map is written as JSON (mod.rs:671, 720, 3076). None of those facade or leaf types derives `Serialize` (#1483 application.rs:189, 246, 254; quire-eval registry.rs:30, result.rs:94). The spec does not say how these causes are encoded in the emitted claim map: stable code strings, a CG-owned mapping, or not serialized. Implementers would diverge, and an ad hoc CG mirror enum would be a shadow vocabulary of QSL causes. | spec/oracle/functional/FR-021-function-application-oracles.md:163-172 |

## Dispositions

Round 1, checked against fix-round head 1249c63ad436f113d26c402bd15ef429f1346dd2.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | TC-031 12b adds malformed-source and insufficient stage/replay-limit acquisition vectors asserting the exact AdmittedRefusal, no handle, no registry and no charge. |
| FND-002 | fixed | Outputs names FacadeLookup, RegistrationNotAccepted, SelectionRefused and ApplicationBeginRefused as ClaimDisposition::Refused variants. The Behavior bullets and AC-37 route each result to them, and TC-031 exercises every Admission variant (it matches quire-eval main's five-variant enum) plus ApplicationBeginRefusal::Call. The serialization gap this opens is recorded as new FND-003. |
