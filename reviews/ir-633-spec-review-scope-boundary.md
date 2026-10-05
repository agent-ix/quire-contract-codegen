---
id: SR-1615
title: "IR-633 spec-review/scope-boundary review"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen@8cbf17816f5ef6e71abcda81da3306680a2b4a4d; spec/assurance/AD-001-codegen-architecture.md, spec/routed/functional/FR-019-capability-settlement.md, spec/routed/functional/FR-022-routed-generation.md, spec/routed/matrix/TC-030-capability-settlement.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-030
    type: reviews
---

# SR-1615: IR-633 spec-review/scope-boundary review

## Summary

Ticket: IR-633. PR: quire-contract-codegen#292 at 8cbf17816f5ef6e71abcda81da3306680a2b4a4d. Reviewed the four changed spec files against QSL ADR-029 PV-1/PV-4, pinned qsl-route descriptors, and CG/driver boundary code. 1 finding(s).

## Method

Inspected AD-001 boundary rows, FR-019 origin prose and AC-15, FR-022 type-location prose, and TC-030 planned origin check. Compared QSL descriptor API at 4403f2f0eee921b7ac9eba41dd1be1914c87835b and QSL ADR-029 PV-1/PV-4. Scope units: FR-019-AC-15, FR-019, AD-001, FR-022, TC-030 (all examined). The spec-only diff adds no source code, vendored file or hash pin.

## Verdict

**FAIL**: fix the finding before merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The spec assigns origin setting to the QSL registry builder, but QSL BackendDescriptor::new/admit accepts ProviderOrigin from its caller and ADR-029 PV-1 says the driver conversion sets it; Registry::register only stores/conflict-checks it. Name the driver registration conversion as the setter and QSL registry as the holder to avoid implementing origin inference inside QSL or CG. | spec/routed/functional/FR-019-capability-settlement.md:125, spec/assurance/AD-001-codegen-architecture.md:175 |
