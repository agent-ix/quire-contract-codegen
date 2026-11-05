---
id: SR-2026110283868
title: IR-724 native comparison spec-review/base
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen; IR-724; tests/it/v2_integer_comparison_admission.rs;
  spec/strategy/functional/FR-008-bound-domain-strategy-admission.md; spec/strategy/matrix/TC-017-bound-domain-admission.md;
  spec/strategy/matrix/tests.md
review_set: base
---

## Summary

Ticket: IR-724. Full FR-008-AC-9 backing is established for its closed operator/provenance/observation variants. Complete typed projection equality preserves operator, operand identity, ordinal, provenance, observation and bounds; independent ordered comparisons check real generated census tags and ranges. Authentic negation, unbounded read, arithmetic and implicit numeric.narrow refusals preserve the full accessor error and selected key through admission and generation. Result::Err prevents an artifact bundle. AC-7/8 remain Planned; no shared FR-014 lowerer or exhaustive-valuation claim is made.

## Verdict

PASS. No defects found.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - | implementation-bug-despite-evidence |

## Coverage

No applicable AssuranceProfile exists in spec/. Independent source review used repository conventions and Rust testing/implementation guidance. The native Twin calls real QSL call_site then CheckedPackageV2::read; it is reused without Kani execution. No private IR body access or cross-case owner borrowing is added. No production, dependency or gate changes. Existing focused comparison, Clippy and specification validation logs were inspected; no Cargo, prover, coverage census or gate was run. Final scoped gate remains the lead’s work.

Base review checked status/trace consistency, unchanged criterion intent and links. Other specification lenses are not applicable: no requirement grammar, structure, domain object, relationship, boundary or verification-method change. Production gap analysis is not applicable to this tests/status-only diff.

```yaml
scope:
- id: FR-008-AC-9
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: A CG integration test calls IR's public clause-context accessor through
    its public API only, and the generator's census, ordered relation and diagnostics
    equal the accessor's operator, ordinals, provenance, observation, bounds and refusal
    fields for each closed operator, provenance and observation variant.
- id: FR-008-AC-7
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: context_only
  excerpt: PLANNED. Beyond the per-operator agreement stated in FR-008-AC-3, which
    this criterion does not repeat, for each of integer.eq, ne, lt, le, gt and ge
    the selected claim's V2 comparison oracle returns, for every valuation of a 1-
    to 4-member domain (read-versus-read and read-versus-literal, the literal at each
    member and in either authored position), the result of an independently evaluated
    ordered relation. For every such valuation, the FR-014 clause oracle's result
    for the same direct comparison is identical to the selected oracle's result and
    to that independent relation.
- id: FR-008-AC-8
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: context_only
  excerpt: PLANNED. The V2 comparison oracle takes only the accessor's typed result
    for the selected claim. Source inspection finds exactly one `lower_typed_integer_comparison`
    function under `src/oracle` used by both the selected strategy oracle and FR-014's
    clause oracle for all six operators and the ordered direct read/read and read/literal
    forms; neither path has a second CG comparison lowerer or evaluator module, and
    no V1 bound oracle, `BoundPackage` or V1-to-V2 adapter is an input to the V2 path.
- id: TC-017
  path: spec/strategy/matrix/TC-017-bound-domain-admission.md
  role: examined
  excerpt: 'FR-008-AC-9 is covered by

    `tests/it/v2_integer_comparison_admission.rs::v2_accessor_population_and_refusal_fields_agree`,

    traced to TC-017. It checks six operators, both closed read-provenance variants,
    all three'
- id: strategy-status
  path: spec/strategy/matrix/tests.md
  role: examined
  excerpt: "| FR-008 | FR-008-AC-9 | TC-017 | \u2705 Covered by `v2_accessor_population_and_refusal_fields_agree`:\
    \ six operators, both read provenances, all observations, census and typed refusal\
    \ retention |\n| FR-008 | FR-008-AC-8 | Inspection | \U0001F6A7 Planned (IR-1316;\
    \ IR-364): FR-014 does not call the shared lowerer |\n| FR-008 | FR-008-AC-10\
    \ | Inspection | \u2705 Covered by source inspection recorded in FR-008; no runtime\
    \ credit |"
- id: v2_accessor_population_and_refusal_fields_agree
  path: tests/it/v2_integer_comparison_admission.rs
  role: examined
  excerpt: '/// Public IR projections remain exact across every comparison, read provenance
    and observation.

    #[trace("FR-008-AC-9", "TC-017")]

    #[test]'
```
