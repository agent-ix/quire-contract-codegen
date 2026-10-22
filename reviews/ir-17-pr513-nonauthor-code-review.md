---
id: SR-15389
title: "IR-17 PR513 supplementary nonauthor code-review"
type: SpecReview
analysis: code-review
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
| FND-001 | medium | The subtree arithmetic fast path bypasses retain_boolean for completed BooleanNot/connective children of an otherwise metered oracle. For ShortCircuitAnd(Not(flag), Eq(Add(x,0),x)) at flag=true, the root is metered because its skipped right subtree contains arithmetic; Not(flag) is emitted by plain_node with no retain and the outer short circuit retains only once. AC-26 requires two retains. With result_units=1, current source completes false instead of returning Incomplete at the outer retain. | src/oracle/boolean_v1.rs:1184 |
| FND-002 | low | The module introduction and emitted scalar header still call quantity/enumeration operators runtime-owned, but this migration emits direct quire_semantic_value::quantity/enumeration calls. This leaves the newly migrated ownership documentation inaccurate. | src/oracle/scalar/mod.rs:3634 |

## Examined scope

- FR-031-AC-26 (examined), spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md: Each completed connective, including unary BooleanNot, calls `quire_exact::retain_boolean` exactly once and charges `boolean.result-retain` once, including a result decided by the left; a child stop charges no connective result-retain point.

- Renderer::outcome_bool (examined), src/oracle/boolean_v1.rs: if !self.holds_arithmetic() {
            self.emit("rt::Outcome::Completed(")?;
            self.plain_node(expression)?;
            return self.emit(")");
        }

- SourceBuilder::finish (examined), src/oracle/scalar/mod.rs: source.push_str("use quire_semantic_value::quantity as quantity_owner;\n");

- SOURCE_HEADER (examined), src/oracle/scalar/mod.rs: // Each function calls its `quire-exact` operator or a runtime-owned residue
// operator with the caller's `Meter`; no charge amount appears in this source.

- TypeClosure::FinishUnion (examined), src/oracle/equality/resolution.rs: if payload != member.payload() {
                        return Err(mismatch().into());
                    }

- generate_composite_equality_oracles (examined), src/oracle/equality/mod.rs: if expected_package.domain.as_ref() != actual_package.domain().as_str()
            || expected_package.digest.as_ref() != actual_package.hex()

- admitted::require_context (examined), src/oracle/equality/admitted.rs: admitted.ok_or_else(|| CompositeEqualityRefusal::MissingAdmittedContext {
        type_node_id: declaration.clone(),
    })

- OriginalCompositeEqRequest::admitted_context (examined), src/replay/composite_builder.rs: pub fn admitted_context(&self) -> &AdmittedContext {
        self.admitted.context()
    }

- OracleOutput::QuantityWithUnit (examined), src/oracle/scalar/mod.rs: "(rt::Outcome<rt::Quantity>, quantity_owner::IdentifiedUnit)".to_owned()

- SCALAR_PROOF_SCHEMA (examined), src/kani/run/scalar.rs: const SCALAR_PROOF_SCHEMA: &str = "agent-ix/cg-kani-scalar-proof-content/v3";

- scalar_observer_manifest (examined), src/core/profile.rs: pub(crate) fn scalar_observer_manifest() -> String {

- tc_044_native_connectives_retain_only_completed_results (examined), tests/it/oracle_arithmetic.rs: "not" => ("false", "3", "0", 0, 2),

- quantity_paired_result_units_survive_stops_and_feed_following_operations (examined), tests/exact_scalar_support/agreement_cases.rs: assert!(matches!(stopped.0, Outcome::Incomplete(_)));

- tc_029_ac18_render_value_type_refuses_quantity_and_reference_with_a_typed_error (context_only), src/oracle/equality/mod.rs: RenderError::UnsupportedValueType { family: "quantity" }

## Evidence and limits

The supplied focused-final-summary.json identifies this exact source head and records zero exit status for scalar native agreement, record/tuple native agreement, six host commands, Union controls, panic-source, scalar-family generation, Boolean retention, formatting, lint and spec validation. I inspected the source, summary and representative raw native logs; these are author-run checks, not reviewer execution. The shared scalar runner requires all declared native cases to run; the original 19 cases remain plus the paired-Quantity control. No earlier host-only result is relabeled as a Kani proof, a third QSL value-authority leg or final aggregate gate.

The prior eight source findings in SR-15370 have corresponding fixes: QSV paired Quantity envelopes and stop propagation; exhaustive Union rejection in record/tuple fixtures; enum_owner comparator expectations; updated Boolean metering contracts; current owner/dependency wording; corrected EnvironmentFailure documentation; retained historical disposition; and corrected local enum-source provenance. The new Boolean finding below is an omitted path beyond those earlier fixes.

## Verdict

**FAIL** for this scoped method at the exact frozen head. Resolve the listed findings and obtain independent disposition before the final aggregate/Kani gates.

## Reviewer data

```yaml
{
  "clean": false,
  "scope": [
    {
      "id": "FR-031-AC-26",
      "path": "spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md",
      "role": "examined",
      "excerpt": "Each completed connective, including unary BooleanNot, calls `quire_exact::retain_boolean` exactly once and charges `boolean.result-retain` once, including a result decided by the left; a child stop charges no connective result-retain point."
    },
    {
      "id": "Renderer::outcome_bool",
      "path": "src/oracle/boolean_v1.rs",
      "role": "examined",
      "excerpt": "if !self.holds_arithmetic() {\n            self.emit(\"rt::Outcome::Completed(\")?;\n            self.plain_node(expression)?;\n            return self.emit(\")\");\n        }"
    },
    {
      "id": "SourceBuilder::finish",
      "path": "src/oracle/scalar/mod.rs",
      "role": "examined",
      "excerpt": "source.push_str(\"use quire_semantic_value::quantity as quantity_owner;\\n\");"
    },
    {
      "id": "SOURCE_HEADER",
      "path": "src/oracle/scalar/mod.rs",
      "role": "examined",
      "excerpt": "// Each function calls its `quire-exact` operator or a runtime-owned residue\n// operator with the caller's `Meter`; no charge amount appears in this source."
    },
    {
      "id": "TypeClosure::FinishUnion",
      "path": "src/oracle/equality/resolution.rs",
      "role": "examined",
      "excerpt": "if payload != member.payload() {\n                        return Err(mismatch().into());\n                    }"
    },
    {
      "id": "generate_composite_equality_oracles",
      "path": "src/oracle/equality/mod.rs",
      "role": "examined",
      "excerpt": "if expected_package.domain.as_ref() != actual_package.domain().as_str()\n            || expected_package.digest.as_ref() != actual_package.hex()"
    },
    {
      "id": "admitted::require_context",
      "path": "src/oracle/equality/admitted.rs",
      "role": "examined",
      "excerpt": "admitted.ok_or_else(|| CompositeEqualityRefusal::MissingAdmittedContext {\n        type_node_id: declaration.clone(),\n    })"
    },
    {
      "id": "OriginalCompositeEqRequest::admitted_context",
      "path": "src/replay/composite_builder.rs",
      "role": "examined",
      "excerpt": "pub fn admitted_context(&self) -> &AdmittedContext {\n        self.admitted.context()\n    }"
    },
    {
      "id": "OracleOutput::QuantityWithUnit",
      "path": "src/oracle/scalar/mod.rs",
      "role": "examined",
      "excerpt": "\"(rt::Outcome<rt::Quantity>, quantity_owner::IdentifiedUnit)\".to_owned()"
    },
    {
      "id": "SCALAR_PROOF_SCHEMA",
      "path": "src/kani/run/scalar.rs",
      "role": "examined",
      "excerpt": "const SCALAR_PROOF_SCHEMA: &str = \"agent-ix/cg-kani-scalar-proof-content/v3\";"
    },
    {
      "id": "scalar_observer_manifest",
      "path": "src/core/profile.rs",
      "role": "examined",
      "excerpt": "pub(crate) fn scalar_observer_manifest() -> String {"
    },
    {
      "id": "tc_044_native_connectives_retain_only_completed_results",
      "path": "tests/it/oracle_arithmetic.rs",
      "role": "examined",
      "excerpt": "\"not\" => (\"false\", \"3\", \"0\", 0, 2),"
    },
    {
      "id": "quantity_paired_result_units_survive_stops_and_feed_following_operations",
      "path": "tests/exact_scalar_support/agreement_cases.rs",
      "role": "examined",
      "excerpt": "assert!(matches!(stopped.0, Outcome::Incomplete(_)));"
    },
    {
      "id": "tc_029_ac18_render_value_type_refuses_quantity_and_reference_with_a_typed_error",
      "path": "src/oracle/equality/mod.rs",
      "role": "context_only",
      "excerpt": "RenderError::UnsupportedValueType { family: \"quantity\" }"
    }
  ],
  "findings": [
    {
      "fnd": "FND-001",
      "severity": "medium",
      "confidence": "high",
      "method": "code-review",
      "check": "code-bug",
      "artifact_id": "FR-031-AC-26",
      "related_ids": [
        "TC-044"
      ],
      "path": "src/oracle/boolean_v1.rs",
      "lines": "1184-1188",
      "excerpt": "Each completed connective, including unary BooleanNot, calls `quire_exact::retain_boolean` exactly once and charges `boolean.result-retain` once, including a result decided by the left; a child stop charges no connective result-retain point.",
      "finding": "The subtree arithmetic fast path bypasses retain_boolean for completed BooleanNot/connective children of an otherwise metered oracle. For ShortCircuitAnd(Not(flag), Eq(Add(x,0),x)) at flag=true, the root is metered because its skipped right subtree contains arithmetic; Not(flag) is emitted by plain_node with no retain and the outer short circuit retains only once. AC-26 requires two retains. With result_units=1, current source completes false instead of returning Incomplete at the outer retain.",
      "requested_change": "Within an already metered oracle, route BooleanNot and connective subtrees through the metered renderer even when those subtrees contain no arithmetic. Preserve the whole arithmetic-free oracle's bool ABI. Add native budget=1 Incomplete and budget=2 Completed(false)/two-retain controls with skipped right arithmetic."
    },
    {
      "fnd": "FND-002",
      "severity": "low",
      "confidence": "high",
      "method": "code-review",
      "check": "other",
      "artifact_id": "SOURCE_HEADER",
      "related_ids": [
        "FR-014"
      ],
      "path": "src/oracle/scalar/mod.rs",
      "lines": "3634-3635",
      "excerpt": "// Each function calls its `quire-exact` operator or a runtime-owned residue\n// operator with the caller's `Meter`; no charge amount appears in this source.",
      "finding": "The module introduction and emitted scalar header still call quantity/enumeration operators runtime-owned, but this migration emits direct quire_semantic_value::quantity/enumeration calls. This leaves the newly migrated ownership documentation inaccurate.",
      "requested_change": "Name quire-semantic-value as the quantity/enum operator owner in both the module introduction (43-44) and emitted SOURCE_HEADER."
    }
  ]
}
```

