---
id: SR-8808
title: "IR-765 spec-review/base review of PR 378"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/core/functional/FR-039-contract-package-consumption.md, spec/core/matrix/TC-054-exact-local-protocol-target.md, spec/core/matrix/TC-053-contract-package-consumption.md, spec/core/matrix/tests.md"
review_set: subset
---

## Summary

The changed criterion, TC split, matrix and upstream carrier agree on local typed target admission and located reader refusals. Reviewed exact PR head 3ac051c14e3ae3db0f15ffe1a3c89a2256b858d8; QSpec context at 34384b8.

## Verdict

**PASS** — no defect found in this method.

## Examined scope

- FR-039-AC-4 (examined) — spec/core/functional/FR-039-contract-package-consumption.md: For a genuinely admitted FR-330 package, CG binds a `protocol`/`protocol_reference` carrier's exact `body.target` to the local `protocol` node, retaining its typed key, requested identity and source occurrences; changing that local target and recomputing the admitted carrier and package identities changes the selected target and package identity. A matching key only in a selected dependency does not bind locally. An absent local target, a present non-`protocol` target and a target with unequal `semantic_type` are refused by the FR-322 reader before CG with the respective `missing_declaration`/
- TC-054 (examined) — spec/core/matrix/TC-054-exact-local-protocol-target.md: Verify [FR-039-AC-4](../functional/FR-039-contract-package-consumption.md) through the production CG entry using genuine QSL-admitted, IR-lowered FR-330 packages. The QSpec FR-322-AC-66 carrier and FR-330-AC-4 single graph determine the target; the FR-322 reader owns target admission and located refusals.
- TC-053 (examined) — spec/core/matrix/TC-053-contract-package-consumption.md: Verify [FR-039](../functional/FR-039-contract-package-consumption.md) through the production negotiation and generation entries using genuine QSL-admitted, IR-lowered FR-330 packages. The controls distinguish package decisions from decisions the original checked graph or detached lowering records would make.
- FR-039-AC-4 matrix row (examined) — spec/core/matrix/tests.md: | FR-039 | FR-039-AC-4 | TC-054 | 🚧 Planned; exact local protocol-target binding and adverse reader controls are not implemented |
- FR-322-AC-66 (context_only) — spec/objects/interfaces/FR-322-checked-package-artifact.md: The worked package with a `protocol`/`protocol_reference` carrier to its local `protocol_clause` target is admitted with one graph collection; its body target, sole dependency and semantic type bind that exact protocol node and its recomputed key changes when the target changes. With the body target and sole dependency changed together and identities recomputed, a missing local target refuses `missing_declaration`/`missing-name` even when a selected dependency contains that key, and a present non-protocol target or unequal semantic type refuses `ill_typed`/`operator-ineligible`. A non-referenc
- FR-330-AC-4 (context_only) — spec/objects/interfaces/FR-330-contract-package.md: A package has exactly one semantic node collection ordered by the FR-322 typed node key; each node occurs once, local references resolve to compatible nodes, dependency references retain FR-322 selection semantics, a `protocol`/`protocol_reference` carrier binds by its body target to a local `protocol` node under FR-322-AC-66, and recursion is represented only by permitted keyed references.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
