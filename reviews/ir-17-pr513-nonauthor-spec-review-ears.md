---
id: SR-15392
title: "IR-17 PR513 supplementary nonauthor spec-review-ears"
type: SpecReview
analysis: ears-conformance
review_set: subset
date: "2026-10-22"
scope: "agent-ix/quire-contract-codegen; IR-17 PR513 frozen cd202f1b against origin/main 0d7813df; nonauthored migration remainder, excluding authored unit-table slice"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-031
    type: reviews
---

## Summary

Ticket: IR-17. PR: agent-ix/quire-contract-codegen#513. Frozen reviewed head: cd202f1b8fda0c23428c19aa47c40ce3769ff76c; comparison base origin/main: 0d7813df955576edc867cfa90e2071bd56d844ff.

Independent supplementary review of the NONAUTHORED migration remainder. Excluded from acceptance: src/oracle/equality/units.rs, tests/it/admitted_quantity_units.rs, their registration and unit-table integration hunks, and the derived AC-45 metadata contract. Another independent reviewer owns those areas and the official whole-migration dispositions. The detached review worktree and frozen branch remain unmodified. No Cargo, build, test, Kani, namespace or host-security action was performed.

Applicable methods: Rust/code, computed matrix and reverse-gap checks, specification base and EARS for the changed nonauthored requirements. No applicable AssuranceProfile or enforced selection was found. Optional whole-repository semantic analysis and Jev criterion-strength analysis were not selected; this is not a whole-repository assurance PASS.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined scope

- FR-018-AC-44 (examined), spec/oracle/functional/FR-018-composite-equality-oracles.md: Genuine Union equality generation preserves the facade declaration key, settled node, authoritative member bindings and declared member/payload order across a nested Union closure, using the specified lookup and verified sealing in both generator and emitted preparation. In-process owner-API controls admit values of every member, including record and nested-Union payloads; distinct same-shaped unions retain incompatible identities. Missing/wrong capability, Unknown/Foreign/WrongKind lookup, name/form disagreement and seal faults retain their typed causes, no successful prepared wrapper and no 

- FR-031-AC-26 (examined), spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md: PLANNED (IR-602). The generated source of a native oracle holds no `quire_contract_runtime::operators` path and no `and_short_circuit`, `or_short_circuit` or `implies_short_circuit`; a short-circuit connective is an inline `match` over the left operand's `rt::Outcome` that returns the first stop and does not evaluate the right operand when the left decides, and a total connective evaluates both operands in order and returns the first stop. Each completed connective, including unary BooleanNot, calls `quire_exact::retain_boolean` exactly once and charges `boolean.result-retain` once, including 

## Evidence and limits

The supplied focused-final-summary.json identifies this exact source head and records zero exit status for scalar native agreement, record/tuple native agreement, six host commands, Union controls, panic-source, scalar-family generation, Boolean retention, formatting, lint and spec validation. I inspected the source, summary and representative raw native logs; these are author-run checks, not reviewer execution. The shared scalar runner requires all declared native cases to run; the original 19 cases remain plus the paired-Quantity control. No earlier host-only result is relabeled as a Kani proof, a third QSL value-authority leg or final aggregate gate.

The prior eight source findings in SR-15370 have corresponding fixes: QSV paired Quantity envelopes and stop propagation; exhaustive Union rejection in record/tuple fixtures; enum_owner comparator expectations; updated Boolean metering contracts; current owner/dependency wording; corrected EnvironmentFailure documentation; retained historical disposition; and corrected local enum-source provenance. The new Boolean finding below is an omitted path beyond those earlier fixes.

The nonauthored FR-018/TC-029 Union and FR-031/TC-044 ownership/metering edits were checked for actor, conditions, observable response, refusal and planned-evidence boundaries. Changed-document Quire validation with --summary returned zero: 4/4 documents grammar-clean, zero grammar findings. AC-45 and its procedure are context only, not independently accepted here. FR-031 retains the whole arithmetic-free bool ABI exception; within a metered oracle, its per-connective rule is explicit. The implementation and missing-test defects belong to SR-15389/15390, not a new normative ambiguity. No additional spec defect was found.

## Verdict

**PASS** for this scoped method at the exact frozen head. No new defect in the examined nonauthored normative text; this is not code or test acceptance.

## Reviewer data

```yaml
{
  "clean": true,
  "scope": [
    {
      "id": "FR-018-AC-44",
      "path": "spec/oracle/functional/FR-018-composite-equality-oracles.md",
      "role": "examined",
      "excerpt": "Genuine Union equality generation preserves the facade declaration key, settled node, authoritative member bindings and declared member/payload order across a nested Union closure, using the specified lookup and verified sealing in both generator and emitted preparation. In-process owner-API controls admit values of every member, including record and nested-Union payloads; distinct same-shaped unions retain incompatible identities. Missing/wrong capability, Unknown/Foreign/WrongKind lookup, name/form disagreement and seal faults retain their typed causes, no successful prepared wrapper and no "
    },
    {
      "id": "FR-031-AC-26",
      "path": "spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md",
      "role": "examined",
      "excerpt": "PLANNED (IR-602). The generated source of a native oracle holds no `quire_contract_runtime::operators` path and no `and_short_circuit`, `or_short_circuit` or `implies_short_circuit`; a short-circuit connective is an inline `match` over the left operand's `rt::Outcome` that returns the first stop and does not evaluate the right operand when the left decides, and a total connective evaluates both operands in order and returns the first stop. Each completed connective, including unary BooleanNot, calls `quire_exact::retain_boolean` exactly once and charges `boolean.result-retain` once, including "
    }
  ],
  "findings": []
}
```

