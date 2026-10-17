---
id: SR-12084
title: "ears-conformance review of IR-818 CG quantity-conversion spec (PR #415)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; spec/oracle/functional/FR-014-exact-scalar-oracles.md"
review_set: subset
---

## Summary

Ticket: IR-818. PR agent-ix/quire-contract-codegen#415 at 5abb404. EARS check of the rewritten
FR-014 Behavior bullet for `quire.op.quantity.convert`. The two main clauses are well-formed
unwanted-behaviour statements: "If ... is `quire.op.quantity.convert`, then the generator shall ..."
and "If the runtime returns `IllTyped` ..., then the oracle shall stop as `ill_typed` ...". The
trailing sentences of the same bullet are not EARS.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The bullet ends with three declaratives that have no `shall`: "The conversion is not an entry-point refusal: ... each treat it as an admitted quantity operation", "Compound-unit conversion targets are outside this statement (IR-831)" and "The composite-equality generator still refuses ...". They carry normative entry-point and exemption behaviour without EARS form | spec/oracle/functional/FR-014-exact-scalar-oracles.md:281-290 |

## Verdict

The core conversion and `ill_typed` statements conform to EARS. The trailing entry-point and exemption sentences should be split into their own `shall` statements, or moved to a non-normative note. Low severity, so this finding alone does not block the merge.

## Scope

Examined: FR-014 Behavior conversion bullet (:269-290). Context only: FR-014 Description :213-216.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | d685fc5: the entry-point, scalar/compound-target, mode and composite-equality clauses are now separate `shall` statements (an `If ... then ... shall` statement for each refusal) |
