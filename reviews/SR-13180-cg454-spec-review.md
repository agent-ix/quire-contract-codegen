---
id: "SR-13180"
title: "CG PR 454 spec review: FR-039-AC-5 moved to Analysis with IR reader evidence"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@af121888fa410c17f7a7a9f635f73de24ae949db; spec/core/functional/FR-039-contract-package-consumption.md (Behavior bullet 173-178, AC-5 at 188, new AC-5 analysis section 193-220), spec/core/matrix/TC-053-contract-package-consumption.md, spec/core/matrix/tests.md (diff origin/main...HEAD, merge base cfb38490)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-039
    type: reviews
---

# SR-13180: CG PR 454 spec review

## Summary

Ticket: IR-888 (parent IR-878). PR: agent-ix/quire-contract-codegen#454 at af12188.

The PR rewrites the FR-039 Behavior bullet on reader refusals, rewrites FR-039-AC-5, changes its
verification from Test to Analysis, adds a `### FR-039-AC-5` analysis section, removes TC-053
step 5, and moves the tests.md AC-5 row to Analysis.

The analysis argument was checked against CG source at the PR head. The by-type half is true.
`ContractPackageV1` (quire-contract-model at Cargo.lock 299a126) has one private field, `wire`.
Only the IR lowerer and `ContractPackageWireV1::read` construct it. CG production source has no
`ContractPackageV1` constructor, no deserialize of it, and no conversion from raw bytes. Every
production consumer takes `&ContractPackageV1`: `negotiate_contract_package`, `generate_routed`,
`generate_composite_equality_oracles`, `generate_exact_function_oracles`, and the crate-internal
scalar and Kani consumers. No CG path admits a package without the reader or lowerer. Every
`ContractPackageWireV1::read` call outside `#[cfg(test)]` matches only `Admitted`.

## Verdict

CHANGES REQUESTED, with two medium findings. The by-type argument is sound. However:

- The AC-5 statement was rewritten instead of kept, and the rewrite narrows two clauses.
- The analysis cites one production read call as conforming. That call actually discards the
  reader's refusal and replaces it with a CG backend refusal.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-039-AC-5 is not kept verbatim. The rewrite changes the trigger from an input class ("a malformed ... wire ... reaches no CG consumer") to a reader outcome ("When the FR-330 reader refuses ..."). It also changes "performs no second parse or checked-graph fallback" (unconditional) to "terminate ... before any ... second parse or checked-graph fallback". Under the new wording, a CG parse of the wire before or beside the reader call no longer violates AC-5. The decision recorded on IR-888 says the statement is kept with no weakening. Restore the original statement, or keep the "performs no second parse or checked-graph fallback" clause unconditional | spec/core/functional/FR-039-contract-package-consumption.md:188 |
| FND-002 | medium | The analysis (lines 202-205) cites the `ContractPackageWireV1::read` call in `src/oracle/function/mod.rs:1745` as matching only `Admitted`. It omits the call's behaviour on refusal: `checked_integer_function_application_is_supported` drops `Refused`/`Incomplete` (`_ => return false`). Its production caller `src/routed/capability.rs:867` then returns `KaniFormRefused { OperationNotRendered("quire.op.function.call") }`. That relabels a reader refusal or limit stop as a CG backend refusal, which contradicts AC-5's preservation clause. The analysis calls preservation "planned" without naming this known counterexample, and no ticket tracks it | spec/core/functional/FR-039-contract-package-consumption.md:202-208; src/oracle/function/mod.rs:1745-1752; src/routed/capability.rs:867-883 |
| FND-003 | low | The rewritten Behavior bullet drops the positive obligation "CG SHALL pass malformed ... inputs through Contract IR's single FR-330 reader". Only the Description's non-normative "does not define another package schema or reader" remains | spec/core/functional/FR-039-contract-package-consumption.md:173-178 |
