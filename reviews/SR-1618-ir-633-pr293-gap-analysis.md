---
id: SR-1618
title: IR-633 CG PR 293 gap analysis
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@213ab63ee5b3551fe7610cde5df0a079e51d421d; src/lib.rs, src/routed/capability.rs, tests/it/capability_settlement.rs; IR-633"
review_set: subset
---

## Summary

Reviewed CG PR 293 at the frozen head. Ticket: IR-633.

## Verdict

**CONDITIONAL** — the CG type carries both origins, but the new test does not exercise the registry-to-driver projection required by FR-019-AC-15.

## Scope

- FR-019-AC-15 (examined): PLANNED (IR-633). CG's `BackendDescriptor` has a typed `ProviderOrigin` projecting exactly QSL layer R's `Linked` and `Process` meanings (QSL FR-288, ADR-029 PV-1); layer R owns that vocabulary. The driver projects each descriptor from `Registry::descriptors()` once into CG's descriptor, copying `id` and advertised pairs unchanged and mapping `origin()` exhaustively, `Linked` to `Linked` and `Process` to `Process`, with no wildcard or origin inference from identity, manifest or provider bytes or a side map. Two registry descriptors identical except for origin produce CG descriptors i
- TC-030 (examined): Registry-origin projection, both variants, no inference or direct QSL dependency.
- FR-022 (context_only): The CG dependency boundary requires an in-process consumer descriptor.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-019-AC-15 is marked tagged, but no test crosses Registry::descriptors() through the driver projection for either origin. | tests/it/capability_settlement.rs:25 |

## Coverage

Plan completion: not assessed. The computed `quoin matrix --repo . --json` reports FR-019-AC-15 tagged to the new `tc_030_backend_descriptor_keeps_origin_independent_of_identity` test, with no run evidence. No new production stub, direct `qsl-route` dependency, wire origin field, or compatibility layer was found in the diff.
