---
id: SR-11043
title: "IR-789/IR-790 PR 384 spec-review/failure-domain review"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen; IR-789, IR-790; spec/routed/functional/FR-019-capability-settlement.md, spec/routed/functional/FR-022-routed-generation.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/routed/matrix/TC-033-routed-generation.md, spec/routed/matrix/TC-046-process-provider-settlement.md"
review_set: subset
---

## Summary

Ticket: IR-789 and IR-790. Exact PR head 7eb090ce707f9f4b1cbb9375b2e7fdfd7ec15dd7, base 5d54a0b1ad9ff6e2073f4372ab44750cb40c6a12. The changed requirements name source identity, occurrence identity, context-root mismatch, and whole-call refusal behavior.

## Verdict

**PASS**.

## Examined scope

- FR-019-AC-25 (examined) — spec/routed/functional/FR-019-capability-settlement.md: The normative Kani form set is the complete V1 proof set defined by IR-269 and FR-015: scalar families Boolean, integer, rational, decimal, IEEE float, text, enum and quantity; record and tuple composite equality; function application; precondition, postcondition and invariant `BoundClause` roles; and bounded collection and graph forms. V1 `ClauseKind` has no frame variant. State before/after and frame obligations are typed V2 `StateFrame` items with the `contract` or `frame` role, included where FR-015 defines their typed lowering. Each item is identified by its typed IR tag/form, family, ope
- FR-019-AC-26 (examined) — spec/routed/functional/FR-019-capability-settlement.md: For any form in FR-019-AC-25 with one validated Kani candidate, a bounded extent settles `supported` exactly when the form's FR-015 lowering succeeds and the candidate advertises the bounded mode; negotiation creates no artifact as part of settlement.
- FR-019-AC-33 (examined) — spec/routed/functional/FR-019-capability-settlement.md: For a valid bounded typed form inside FR-019-AC-25 that matches one specific Planned subpath in TC-030's status register and does not match an Implemented subpath, negotiation returns `unsupported` with that subpath's single exact typed refusal cause, emits no artifact, and never settles `supported` before its renderer is implemented; this refusal remains distinct from the out-of-V1 refusal in AC-27.
- FR-019-AC-42 (examined) — spec/routed/functional/FR-019-capability-settlement.md: Two otherwise equal CG Process descriptors with absent and present-empty manifest `bounds` remain distinct typed values. Changing only between those forms leaves process settlement's disposition and request-validation precedence unchanged; run-limit defaults are not proof bounds.
- FR-019-AC-43 (examined) — spec/routed/functional/FR-019-capability-settlement.md: Two driver items selecting distinct `expression` occurrences of one checked `integer.add` node and different direct `numeric.narrow` occurrences in one admitted ContractPackage settle independently under the same Kani descriptor. Each preflight result is classified from its own selected result context; changing the selected narrow changes that item's typed result identity or refusal without changing the other. A node-only classifier or package-wide choice of either bound fails this control.
- FR-019-AC-44 (examined) — spec/routed/functional/FR-019-capability-settlement.md: For a V2 scalar claim, an absent `OwnResult`, an application occurrence absent from the admitted source, a conversion occurrence from another source or unrelated to the application, a missing conversion context root, or a ContractPackage source identity different from the checked source settles `invalid-request` with a typed form/context cause and no artifact or harness. Request and candidate/manifest defects retain their earlier precedence; for a valid candidate this selected-form refusal precedes extent and advertised-mode refusal. Neither a source message nor a caller maximum repairs the se
- FR-022-AC-23 (examined) — spec/routed/functional/FR-022-routed-generation.md: After FR-019 authenticates two `integer.add` driver items on one admitted ContractPackage with distinct selected application/direct-`numeric.narrow` pairs, routed generation retains the pair for each item in its claim and harness identity and asserts against its own selected result bound. The two generated harnesses have distinct identities and expected ranges; a fresh settlement of a changed selection changes or refuses only that item, without borrowing the other bound.
- FR-022-AC-24 (examined) — spec/routed/functional/FR-022-routed-generation.md: A routed scalar item's retained authenticated selection and its driver lowering position must name the same application/conversion pair in the same admitted ContractPackage. A changed pair, missing conversion context root or occurrence absent from that package refuses the whole call with a typed selection error before any harness. No package-wide bound search, source-text reparse or copied QSL context supplies a replacement.
- FR-022-AC-25 (examined) — spec/routed/functional/FR-022-routed-generation.md: The public CG API exposes the concrete `ContractScalarError` carried by `RoutedGenerationError::ContractScalar`, so an external driver can match `ContractScalarError::Oracle(OracleGenerationError::SourceTooLarge { bytes, limit, setting })` and distinguish the scalar source ceiling from request-accounting and other oracle failures without parsing text or naming a private module.
- TC-030 (context_only) — spec/routed/matrix/TC-030-capability-settlement.md:  Verify that every requested capability claim is settled by exactly one `negotiate_*` arm over a closed backend kind, that each FR-290 rule settles its own row, and that only an item settled `supported` routes a backend.
- TC-033 (context_only) — spec/routed/matrix/TC-033-routed-generation.md:  Verify that `generate_routed` runs each routed item's backend-kind generation arm, keyed by the driver's request index. It takes the routed backend and kind as given, refuses an inconsistent routing, and returns for Kani exactly what the FR-015 generator returns for the same items.
- TC-046 (context_only) — spec/routed/matrix/TC-046-process-provider-settlement.md:  Verify that `Process(BackendId)` settles from the FR-331 manifest's advertised (kind, mode) pairs and `domains`, against the requested item's capability kind and full `extent`. Manifest `bounds` are run-limit defaults and do not set admission maxima. Bounded `ProofBound.kind` is required by QSpec FR-331; QSL-654 owns the producer and admit-side `invalid-domains` enforcement. QSpec FR-290-AC-13 owns the domain-kind check. The test does not infer a kind from `DomainKey`. Verify the empty generation arm and absence of plugin calls (QSL ADR-029 PV-4).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Diff-scoped review of the two combined tickets and changed FR-019/FR-022/TC-030/TC-033/TC-046 content. No broad Cargo or Kani gate run by reviewer. Plan completion: not assessed.
