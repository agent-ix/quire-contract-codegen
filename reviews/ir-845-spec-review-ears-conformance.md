---
id: SR-12791
title: IR-845 spec-review/ears-conformance review
type: SpecReview
analysis: ears-conformance
scope: 'agent-ix/quire-contract-codegen; PR #420 own spec delta against #416'
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-021
  type: references
---

## Summary

Ticket: IR-845. PASS. Examined the affected Description and Constraints requirement statements, including deletion of the future concrete-emitter obligation and retention of the generator no-compatibility obligation. Their remaining shall responses name the actor and specify concrete forbidden surfaces. Boundary assertions are declarative scope statements rather than executable event obligations. No new EARS defect was found.

The deterministic engine scanned spec/**/*.md: 107/109 documents grammar-clean; three advisory warnings occur only in unchanged FR-017 and FR-014, with none in FR-021. Semantic judgment found no event/state substitution, vague response or wrong unwanted-condition pattern in the affected FR-021 statements. Acceptance-criteria table prose is outside the EARS statement lens.

## Reviewed scope

| Unit | Role | Path | Excerpt |
| --- | --- | --- | --- |
| FR-021 Description boundary | examined | spec/oracle/functional/FR-021-function-application-oracles.md | The current Contract Runtime `exact` implementation and its emitted host closures are migration inputs, not this requirement's target ABI. Contract Runtime [FR-275](ix://agent-ix/quire-contract-runtime/FR-275) deletes that port; QSL exports no callable application API through qsl-replay (IR-845), and QSL arch-lint T12-A bars CG from qsl-eval, so CG claims no QSL application API. Concrete Rust oracle emission, compilation and execution against a QSL application API are outside CG's boundary until a concrete need is filed and QSL specifies that API (IR-845; FR-014-AC-47). The generator shall not |
| FR-021 no-panic constraint | examined | spec/oracle/functional/FR-021-function-application-oracles.md | The schedule crate shall contain no `unwrap`, `expect`, panicking index, `unreachable!`, `panic!`, `todo!` or `unimplemented!`, no literal charge amount and no literal `Outcome`/`Value` standing in for an owner result. |
| FR-021 no-runtime-port constraint | examined | spec/oracle/functional/FR-021-function-application-oracles.md | The generator shall not emit or retain `quire_contract_runtime::exact` paths, nor an alias, wrapper or compatibility layer for them. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for this method and scoped delta. The two original IR-853 deferrals remain deferred.
