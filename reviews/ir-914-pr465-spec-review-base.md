---
id: SR-13317
title: spec-review/base review of CG ScalarLimits root export
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-017
  type: reviews
---

## Summary

Ticket: IR-914. Reviewed the added FR-017 behavior and AC-51 against the public type contract. The AC specifies codegen-to-request identity, while the FR also demands identity with the runtime owner.

## Verdict

CONDITIONAL. The added ids and Test (TC-027) reference are valid; no changed text adds a digest, pin, or copied type.

## Examined

FR-017 added root-export statement (examined): The codegen crate root SHALL publicly re-export the exact `quire_contract_runtime::exact::ScalarLimits` type of `KaniScalarNativeRequest::scalar_limits`, with no wrapper, constructor or default.
FR-017-AC-51 (examined): A consumer compile test names and constructs `quire_contract_codegen::ScalarLimits` using only codegen crate paths, and a typed accessor to `KaniScalarNativeRequest::scalar_limits` compiles with that same return type.
TC-027 (context only): existing Kani execution suite.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-51 omits an RT-type identity check, so a codegen-local wrapper used by both root and request can satisfy it while violating the FR. | spec/kani/functional/FR-017-kani-execution-evidence.md:831 |
