---
id: SR-12142
title: "IR-845 scope-boundary review: FR-021-AC-32 and Contract Runtime FR-275"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen@8771d9ffb11e5570f3bd803ee3d5d1b7dfef0ada (PR #420, diff vs origin/peter/ir-833-qsl-inspection-boundaries a4ad399); spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/tests.md; read-only context agent-ix/quire-contract-runtime spec/exact/functional/FR-275-single-exact-kernel.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: references
---

## Summary

Ticket: IR-845. This review checks the boundary the PR draws (CG claims no QSL application API) against #416's boundary (FR-014-AC-47, QSL value authority, IR-842) and against Contract Runtime FR-275, which the new FR-021-AC-32 matrix row cites. Contract Runtime FR-275 exists at /home/peter/dev/quire-contract-runtime/spec/exact/functional/FR-275-single-exact-kernel.md ("Consume QSL's exact kernel and hold no ported QSL code"). It plans to delete the runtime's `quire_contract_runtime::exact` module, gated on IR-349, IR-583, IR-590 and RT target acceptance.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-021-AC-32 | examined | Neither the schedule crate nor any concrete oracle requires `quire_contract_runtime::exact`, an alias, wrapper or compatibility layer; CG emits no owner-path oracle (IR-845). |
| spec/oracle/matrix/tests.md FR-021-AC-32 row | examined | Planned; the existing oracle still witnesses the former Contract Runtime `exact` port until Contract Runtime FR-275 deletes it; CG emits no owner-path oracle (IR-845) |
| FR-021 Dependencies "No QSL application API" | examined | QSL exports no callable application API through qsl-replay (QSL decision, IR-845), so this requirement targets none. |
| FR-021-AC-18 | context_only | That agreement is not claimed by CG and is owned by quire-integration IR-842 (FR-014-AC-47). |
| FR-014-AC-47 | context_only | no TC-024, TC-029 or TC-031 step and no oracle-matrix row for those cases or corpora claims, or presents as evidence, agreement with the QSL value authority |
| Contract Runtime FR-275 Description | context_only | The runtime has no `quire_contract_runtime::exact` module ... and generated oracles and the code generator name the owning crates' own paths. ... Function application belongs to QSL's separate evaluation leaf: IR-583 ... IR-590 supplies the usable `no_std` plus `alloc` evaluator (QSL FR-262). |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The FR-021-AC-32 Planned row is accurate as fact: the generator still emits `quire_contract_runtime::exact` (src/oracle/function/mod.rs:151), and RT FR-275 plans the deletion. But it frames closure as waiting for RT ("until Contract Runtime FR-275 deletes it"), while AC-32 is a CG obligation that RT's deletion would break, not satisfy. RT FR-275 also states that "generated oracles and the code generator name the owning crates' own paths", meaning QSL's evaluation leaf (IR-583/IR-590) for function application. Under IR-845, CG targets no QSL application API, and T12-A bars CG from qsl-eval. So the two specs now disagree on what CG's function oracle names after the port is deleted. Name the CG-owned action that closes AC-32 (for example, removing the concrete exact-emitting function oracle, with a ticket), and record the FR-275 expectation as a cross-repo item for RT (a sibling-repo note or ticket, not an RT edit from this PR). | spec/oracle/matrix/tests.md:50, spec/oracle/functional/FR-021-function-application-oracles.md:279, spec/oracle/functional/FR-021-function-application-oracles.md:331-334 |

## Verdict

CHANGES REQUESTED (one medium). The new boundary is drawn the right way round. CG claims no QSL application API, it cites IR-845 and FR-014-AC-47, and it does not overlap #416's value-authority boundary. AC-18, FR-014-AC-47 and its matrix row are untouched and still true: no FR-021 line added here claims value-authority agreement. The only open boundary question is the hand-off with Contract Runtime FR-275 (FND-001).
