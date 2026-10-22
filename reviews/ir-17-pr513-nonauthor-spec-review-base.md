---
id: SR-15391
title: "IR-17 PR513 supplementary nonauthor spec-review-base"
type: SpecReview
analysis: base
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

- FR-018-AC-33 (examined), spec/oracle/functional/FR-018-composite-equality-oracles.md: A genuine root-facade compilation of authentic retained source, original dependencies and admitted request limits has equal compiled/context/original package identities and exact full recompiled/original CheckedPackageV2 equality before its context is exposed. An explicitly supplied different context package refuses generation and emitted environment construction before classification or lookup, including primitive-only requests, publishes no prepared value, and cannot evaluate with a completed Boolean or meter charge.

- FR-018-AC-34 (examined), spec/oracle/functional/FR-018-composite-equality-oracles.md: The actual generated ABI executes ordered and unordered enum comparisons over all declared cases, including a case absent from every selected literal and an enum parameter with no member literals. Exact shape variants and canonical positions come from genuine facade/QSV values. Same-spelled cases from distinct declarations retain distinct identities; matching and mismatching declaration comparisons preserve the owning equality disposition.

- FR-018-AC-35 (examined), spec/oracle/functional/FR-018-composite-equality-oracles.md: Unknown declaration, unknown case and a directly or transitively imported declaration produce the original typed Unknown/Foreign details at consumed and generated lookup boundaries. Malformed source/dependency input, QSL stage or above-reader resource limits, and internal Fault remain Package, Resource and Fault at the source-retaining boundary; a subsequent IR read stop remains the separate PackageReadLimit, with no partial context or successful prepared wrapper. The current ImportedContextUnsupported guard remains until original admitted dependency packages are retained.

- FR-018-AC-36 (examined), spec/oracle/functional/FR-018-composite-equality-oracles.md: Emitted source uses only WireNodeId and case identifiers for lookup and contains no NodeKey constructor/decoder or generated nominal preimage/hash. Only the prepared wrapper's private genuine index reaches check_equality. A private environment-only fault fixture still satisfies AC-8, every original charge-stop case still satisfies AC-9, and a wrong item/descriptor prepared wrapper refuses before charging. Record/tuple/recursive/collection cases remain required by AC-2 and AC-21 through AC-30; an absent owner lookup capability is recorded as unresolved migration work, not as accepted Unsupporte

- FR-018-AC-37 (examined), spec/oracle/functional/FR-018-composite-equality-oracles.md: Equality's generated manifest and native scratch consumers use the one profile authority's direct exact/QSV/replay dependencies, while ordinary Boolean/integer FR-014 scalar proof manifests and oracle bodies acquire no replay source-compilation/context leg. Native generated enum acceptance is reported separately from original TC025/TC027 proof qualification; neither result substitutes for the other.

- FR-018-AC-38 (examined), spec/oracle/functional/FR-018-composite-equality-oracles.md: With `None`, standalone generation and emitted preparation accept successfully reconstructed non-nominal primitive and nested Option/Collection closures; their index is empty only after complete zero-enum resolution. Existing admitted-IR conversion-form and quantity-family negatives retain respectively exact FormMismatch("conversion") and Unsupported("quantity"), with no replacement ContextPackageMismatch or MissingAdmittedContext, no symbols for the refused item and unchanged independent siblings. This does not claim matching QSL source emission or source authentication for those IR fixtures.

- FR-018-AC-39 (examined), spec/oracle/functional/FR-018-composite-equality-oracles.md: `None` at the first reached nominal identity returns typed MissingAdmittedContext naming that declaration. Cover left and right source operands, left and right conversion targets from primitive sources, nested Option and Collection enum types, and eventual record/tuple field and recursive closures through their genuine owner capability. No key, default/partial index, omitted nominal declaration or Unsupported substitute is produced. Retain the existing charged-entry/cycle/error precedence and unaffected sibling results; the separate missing record/tuple owner lookup remains outstanding.

- FR-018-AC-40 (examined), spec/oracle/functional/FR-018-composite-equality-oracles.md: Some(wrong-package context) fails the whole standalone call after existing setting/accounting checks and before item classification, for primitive-only, nominal and empty requests. Emitted context-free and nominal constructors both reject the same supplied mismatch before reconstruction. None and Some(wrong) have distinct outcomes; no failure publishes partial artifacts or a prepared wrapper.

- FR-018-AC-41 (examined), spec/oracle/functional/FR-018-composite-equality-oracles.md: A reached enum still reconstructs every owner-admitted case, including unused cases and an enum parameter with no member literal, using the supplied genuine keys and canonical positions. Failed nominal admission/lookup/traversal cannot yield an empty-index success. A successfully resolved zero-enum closure may use an empty index; merely inspecting an outer primitive/Option/Collection tag does not establish zero enums.

- FR-018-AC-42 (examined), spec/oracle/functional/FR-018-composite-equality-oracles.md: Generated oracle invocations under Some or valid context-free None preserve private package/item/descriptor/index binding, both check_type guards, same-index check_equality and evaluate. Private wrong-binding, present-context mismatch, missing nominal context and environment-only faults refuse without charge. The native OriginalCompositeEqContext route retains every mandatory authentic source/lock/package/whole-IR comparison and cannot use None to bypass them. No scalar proof manifest/body acquires replay compilation.

- FR-018-AC-43 (examined), spec/oracle/functional/FR-018-composite-equality-oracles.md: For genuine source with a: Int[0,9] and b: Integer, the admitted left parameter names its integer_range and the right parameter names canonical base Integer. Direct source/target reconstruction preserves Int[0,9] versus unbounded Integer despite the sibling range: a caller descriptor converting the left to Integer compares with the unbounded right under the original equality rule, including right values outside [0,9]; an unbounded Integer source cannot acquire a narrowing conversion by borrowing that sibling interval. Claim/native descriptor checks remain independent of generated output. If so

- FR-018-AC-44 (examined), spec/oracle/functional/FR-018-composite-equality-oracles.md: Genuine Union equality generation preserves the facade declaration key, settled node, authoritative member bindings and declared member/payload order across a nested Union closure, using the specified lookup and verified sealing in both generator and emitted preparation. In-process owner-API controls admit values of every member, including record and nested-Union payloads; distinct same-shaped unions retain incompatible identities. Missing/wrong capability, Unknown/Foreign/WrongKind lookup, name/form disagreement and seal faults retain their typed causes, no successful prepared wrapper and no

- FR-031-AC-18 (examined), spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md: PLANNED (IR-602). A divide or remainder node over a `saturate` integer type is refused with `UnsupportedSaturatingArithmetic` at the node's span, with no artifact and no raw `/` or `%` in any output, for the `saturate` rows S-1 to S-4 of TC-044 step 11, in both consumers; the diagnostic message names the missing saturating operation and the refusal has terminal state `unsupported`. On the tree before the change each row is refused with `UnsupportedIntegerDivision` instead. This criterion calls no kernel and can be implemented independently of kernel calls, while the interim code still exists f

- FR-031-AC-22 (examined), spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md: PLANNED (IR-602). The generated source of a native oracle with an arithmetic node aliases `quire_exact` as `rt`, takes `&mut rt::Meter`, returns `rt::Outcome<bool>`, and contains no `quire_contract_runtime::exact` path; a generated crate that compiles and runs it, with divide and remainder nodes and with add, subtract and multiply nodes in the same oracle (the construction Z-3, `y <= 0 && x % (y + 11) <= 10`, is one such oracle), resolves one `quire-exact`; CG's own `Cargo.toml` depends on `quire-exact` directly from `agent-ix/quire-exact`, and CG's lock holds one `quire-exact` entry (`make de

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
      "id": "FR-018-AC-33",
      "path": "spec/oracle/functional/FR-018-composite-equality-oracles.md",
      "role": "examined",
      "excerpt": "A genuine root-facade compilation of authentic retained source, original dependencies and admitted request limits has equal compiled/context/original package identities and exact full recompiled/original CheckedPackageV2 equality before its context is exposed. An explicitly supplied different context package refuses generation and emitted environment construction before classification or lookup, including primitive-only requests, publishes no prepared value, and cannot evaluate with a completed Boolean or meter charge."
    },
    {
      "id": "FR-018-AC-34",
      "path": "spec/oracle/functional/FR-018-composite-equality-oracles.md",
      "role": "examined",
      "excerpt": "The actual generated ABI executes ordered and unordered enum comparisons over all declared cases, including a case absent from every selected literal and an enum parameter with no member literals. Exact shape variants and canonical positions come from genuine facade/QSV values. Same-spelled cases from distinct declarations retain distinct identities; matching and mismatching declaration comparisons preserve the owning equality disposition."
    },
    {
      "id": "FR-018-AC-35",
      "path": "spec/oracle/functional/FR-018-composite-equality-oracles.md",
      "role": "examined",
      "excerpt": "Unknown declaration, unknown case and a directly or transitively imported declaration produce the original typed Unknown/Foreign details at consumed and generated lookup boundaries. Malformed source/dependency input, QSL stage or above-reader resource limits, and internal Fault remain Package, Resource and Fault at the source-retaining boundary; a subsequent IR read stop remains the separate PackageReadLimit, with no partial context or successful prepared wrapper. The current ImportedContextUnsupported guard remains until original admitted dependency packages are retained."
    },
    {
      "id": "FR-018-AC-36",
      "path": "spec/oracle/functional/FR-018-composite-equality-oracles.md",
      "role": "examined",
      "excerpt": "Emitted source uses only WireNodeId and case identifiers for lookup and contains no NodeKey constructor/decoder or generated nominal preimage/hash. Only the prepared wrapper's private genuine index reaches check_equality. A private environment-only fault fixture still satisfies AC-8, every original charge-stop case still satisfies AC-9, and a wrong item/descriptor prepared wrapper refuses before charging. Record/tuple/recursive/collection cases remain required by AC-2 and AC-21 through AC-30; an absent owner lookup capability is recorded as unresolved migration work, not as accepted Unsupporte"
    },
    {
      "id": "FR-018-AC-37",
      "path": "spec/oracle/functional/FR-018-composite-equality-oracles.md",
      "role": "examined",
      "excerpt": "Equality's generated manifest and native scratch consumers use the one profile authority's direct exact/QSV/replay dependencies, while ordinary Boolean/integer FR-014 scalar proof manifests and oracle bodies acquire no replay source-compilation/context leg. Native generated enum acceptance is reported separately from original TC025/TC027 proof qualification; neither result substitutes for the other."
    },
    {
      "id": "FR-018-AC-38",
      "path": "spec/oracle/functional/FR-018-composite-equality-oracles.md",
      "role": "examined",
      "excerpt": "With `None`, standalone generation and emitted preparation accept successfully reconstructed non-nominal primitive and nested Option/Collection closures; their index is empty only after complete zero-enum resolution. Existing admitted-IR conversion-form and quantity-family negatives retain respectively exact FormMismatch(\"conversion\") and Unsupported(\"quantity\"), with no replacement ContextPackageMismatch or MissingAdmittedContext, no symbols for the refused item and unchanged independent siblings. This does not claim matching QSL source emission or source authentication for those IR fixtures."
    },
    {
      "id": "FR-018-AC-39",
      "path": "spec/oracle/functional/FR-018-composite-equality-oracles.md",
      "role": "examined",
      "excerpt": "`None` at the first reached nominal identity returns typed MissingAdmittedContext naming that declaration. Cover left and right source operands, left and right conversion targets from primitive sources, nested Option and Collection enum types, and eventual record/tuple field and recursive closures through their genuine owner capability. No key, default/partial index, omitted nominal declaration or Unsupported substitute is produced. Retain the existing charged-entry/cycle/error precedence and unaffected sibling results; the separate missing record/tuple owner lookup remains outstanding."
    },
    {
      "id": "FR-018-AC-40",
      "path": "spec/oracle/functional/FR-018-composite-equality-oracles.md",
      "role": "examined",
      "excerpt": "Some(wrong-package context) fails the whole standalone call after existing setting/accounting checks and before item classification, for primitive-only, nominal and empty requests. Emitted context-free and nominal constructors both reject the same supplied mismatch before reconstruction. None and Some(wrong) have distinct outcomes; no failure publishes partial artifacts or a prepared wrapper."
    },
    {
      "id": "FR-018-AC-41",
      "path": "spec/oracle/functional/FR-018-composite-equality-oracles.md",
      "role": "examined",
      "excerpt": "A reached enum still reconstructs every owner-admitted case, including unused cases and an enum parameter with no member literal, using the supplied genuine keys and canonical positions. Failed nominal admission/lookup/traversal cannot yield an empty-index success. A successfully resolved zero-enum closure may use an empty index; merely inspecting an outer primitive/Option/Collection tag does not establish zero enums."
    },
    {
      "id": "FR-018-AC-42",
      "path": "spec/oracle/functional/FR-018-composite-equality-oracles.md",
      "role": "examined",
      "excerpt": "Generated oracle invocations under Some or valid context-free None preserve private package/item/descriptor/index binding, both check_type guards, same-index check_equality and evaluate. Private wrong-binding, present-context mismatch, missing nominal context and environment-only faults refuse without charge. The native OriginalCompositeEqContext route retains every mandatory authentic source/lock/package/whole-IR comparison and cannot use None to bypass them. No scalar proof manifest/body acquires replay compilation."
    },
    {
      "id": "FR-018-AC-43",
      "path": "spec/oracle/functional/FR-018-composite-equality-oracles.md",
      "role": "examined",
      "excerpt": "For genuine source with a: Int[0,9] and b: Integer, the admitted left parameter names its integer_range and the right parameter names canonical base Integer. Direct source/target reconstruction preserves Int[0,9] versus unbounded Integer despite the sibling range: a caller descriptor converting the left to Integer compares with the unbounded right under the original equality rule, including right values outside [0,9]; an unbounded Integer source cannot acquire a narrowing conversion by borrowing that sibling interval. Claim/native descriptor checks remain independent of generated output. If so"
    },
    {
      "id": "FR-018-AC-44",
      "path": "spec/oracle/functional/FR-018-composite-equality-oracles.md",
      "role": "examined",
      "excerpt": "Genuine Union equality generation preserves the facade declaration key, settled node, authoritative member bindings and declared member/payload order across a nested Union closure, using the specified lookup and verified sealing in both generator and emitted preparation. In-process owner-API controls admit values of every member, including record and nested-Union payloads; distinct same-shaped unions retain incompatible identities. Missing/wrong capability, Unknown/Foreign/WrongKind lookup, name/form disagreement and seal faults retain their typed causes, no successful prepared wrapper and no "
    },
    {
      "id": "FR-031-AC-18",
      "path": "spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md",
      "role": "examined",
      "excerpt": "PLANNED (IR-602). A divide or remainder node over a `saturate` integer type is refused with `UnsupportedSaturatingArithmetic` at the node's span, with no artifact and no raw `/` or `%` in any output, for the `saturate` rows S-1 to S-4 of TC-044 step 11, in both consumers; the diagnostic message names the missing saturating operation and the refusal has terminal state `unsupported`. On the tree before the change each row is refused with `UnsupportedIntegerDivision` instead. This criterion calls no kernel and can be implemented independently of kernel calls, while the interim code still exists f"
    },
    {
      "id": "FR-031-AC-22",
      "path": "spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md",
      "role": "examined",
      "excerpt": "PLANNED (IR-602). The generated source of a native oracle with an arithmetic node aliases `quire_exact` as `rt`, takes `&mut rt::Meter`, returns `rt::Outcome<bool>`, and contains no `quire_contract_runtime::exact` path; a generated crate that compiles and runs it, with divide and remainder nodes and with add, subtract and multiply nodes in the same oracle (the construction Z-3, `y <= 0 && x % (y + 11) <= 10`, is one such oracle), resolves one `quire-exact`; CG's own `Cargo.toml` depends on `quire-exact` directly from `agent-ix/quire-exact`, and CG's lock holds one `quire-exact` entry (`make de"
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

## Narrow readback

Published head 82051284537d9468f03b8a158a3e4f3605309a3f was compared with cd202f1b8fda0c23428c19aa47c40ce3769ff76c. There is no normative spec delta; the original scoped PASS is carried forward. Source/gap dispositions are recorded separately in SR-15389/SR-15390. The excluded authored unit metadata slice remains with the official independent reviewer. No new findings, Cargo or backend execution. These scratchpad artifacts normalize trailing presentation whitespace before custody; original findings and structured review data remain unchanged.
