---
id: SR-202610310129
title: "IR-1206 gap-analysis review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-1206; routed package sharing and once-per-package regression controls"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-030
    type: reviews
---

## Summary

Independent review of PR #693's three changed files: src/routed/capability.rs, src/routed/contract_package_tests.rs and tests/it/capability_settlement.rs. All four checked-package fields of KaniForm now use canonical immutable shared ownership. Existing instance-address cache keys remain unchanged.

## Verdict

PASS: no findings in the scoped correction. Parent-owned focused receipts show 33 unit tests and 53 capability-settlement integration tests passed, workspace all-target Clippy completed with warnings denied, and formatting passed. Reviewer inspected source and outputs; no Cargo command or proof was run by this reviewer.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined scope

- FR-019-AC-25 (examined), spec/routed/functional/FR-019-capability-settlement.md: OPEN for surviving typed V2 forms; deprecated `V1BoundClause` roles retire under the 2026-10-27 owner ruling on IR-348. Original criterion retained for traceability: The normative Kani form set is the complete V1 proof set defined by IR-269 and FR-015: scalar families Boolean, integer, rational, decimal, IEEE float, text, enum and quantity; record and tuple composite equality; function application; precondition, postcondition and invariant `BoundClause` roles; and bounded collection and graph forms. V1 `ClauseKind` has no frame variant. State before/after and frame obligations are typed V2 `St
- FR-019-AC-27 (examined), spec/routed/functional/FR-019-capability-settlement.md: With one Kani candidate, a typed form outside the normative set in FR-019-AC-25 settles `unsupported`, warned, with `unsupported_projection`/`unsupported-requested-capability` naming its typed tag/form and Kani; no artifact is emitted and no generator is invoked. A malformed in-set form settles with the applicable typed FR-015 validation refusal.
- FR-019-AC-43 (examined), spec/routed/functional/FR-019-capability-settlement.md: Two driver items selecting distinct `expression` occurrences of one checked `integer.add` node and different direct `numeric.narrow` occurrences in one admitted ContractPackage settle independently under the same Kani descriptor. Each preflight result is classified from its own selected result context; changing the selected narrow changes that item's typed result identity or refusal without changing the other. A node-only classifier or package-wide choice of either bound fails this control.
- TC-030 (examined), spec/routed/matrix/TC-030-capability-settlement.md: Use the public negotiation API with typed IR forms and one Kani descriptor.

## Review evidence

The borrowed caches derive their keys from the CheckedPackageV2 allocation, using package.as_ref() and ordinary Arc dereference coercion, rather than the address of an Arc handle, package name or lock. Cloning RequestItem/KaniForm now keeps that allocation alive and shared for the entire borrowed envelope. Independent Arc allocations remain distinct; allocation reuse cannot collide while those immutable package references remain live. Eq/PartialEq remain value equality, independent of the address-only ephemeral cache key. No mutation, unsafe code, new mirror type, dependency, compatibility alias or schema is introduced.

The function control clones the first request twice, varies one callee and includes an independently admitted source with overlapping call IDs and different graph contents. It asserts exact once-per-allocation graph visits, supported/invalid settlement outcomes and bounded indexed lookup work. The outside-form control uses explicit Arc clones for four shared items plus another admitted package and asserts both graph and scalar derivation visits equal the sum of the two graphs. The independent classifier fixture extends the same named source fixture without changing its package name. The existing test support and production counters are used without a new bypass or timing threshold. Remaining direct constructors intentionally create their own admitted allocations; cloning an already constructed form shares its package through the standard Arc Clone implementation across all four variants.

Arc ownership is an intentional public source API change authorized by the ticket. All direct in-repo constructors compile in the parent gates, and no compatibility path is retained. No CI workflow, manifest, test registration or gate is weakened. Repository AGENTS.md delegates to CLAUDE.md; its conventions and relevant Rust API, ownership, boundary and test guidance were examined. No active repository AssuranceProfile was found.

Plan completion: not assessed

The computed whole-repository matrix baseline is 555 tagged, 362 untagged, 93 method-without-symbol, 12 tagged-by-ignored-test. FR-019-AC-25 is an inspection criterion with seven binders; AC-27 has three tagged binders and AC-43 one. The scoped package-sharing correction introduces no new semantic branch or obligation gap: the two restored visit-count controls distinguish the original per-clone regression from package-name coalescing, while integration controls preserve refusal and supported behavior. Historical matrix gaps remain context; this review does not claim that the entire normative Kani form inventory is implemented or that all repository criteria have evidence.
