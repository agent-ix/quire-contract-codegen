---
id: SR-15390
title: "IR-17 PR513 supplementary nonauthor gap-analysis"
type: SpecReview
analysis: gap-analysis
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
| FND-001 | medium | AC-26 has one tagged retention test, but all its connective/Not cases selected for metered rendering hold arithmetic below the connective under test. It omits an arithmetic-free BooleanNot/connective child inside a metered oracle, so the existing green native retention check does not detect the product defect documented by code-review FND-001. | tests/it/oracle_arithmetic.rs:1646 |

## Examined scope

- FR-031-AC-26 (examined), spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md: Each completed connective, including unary BooleanNot, calls `quire_exact::retain_boolean` exactly once and charges `boolean.result-retain` once, including a result decided by the left; a child stop charges no connective result-retain point.

- TypeClosure::FinishUnion (examined), src/oracle/equality/resolution.rs: if payload != member.payload() {
                        return Err(mismatch().into());
                    }

- tc_044_native_connectives_retain_only_completed_results (examined), tests/it/oracle_arithmetic.rs: "not" => ("false", "3", "0", 0, 2),

- quantity_paired_result_units_survive_stops_and_feed_following_operations (examined), tests/exact_scalar_support/agreement_cases.rs: assert!(matches!(stopped.0, Outcome::Incomplete(_)));

## Evidence and limits

Computed Quire matrices were generated for both detached trees. Base: 897 criteria, 524 tagged, 303 untagged, 18 tagged only by ignored tests, 52 method-without-symbol. Head: 899 criteria, 543 tagged, 288 untagged, 16 ignored-only, 52 method-without-symbol. AC-44/45 are new and tagged; AC-45 is excluded from this review's acceptance. Fifteen existing criteria gain tags, and FR-017-AC-27/30 move from ignored-only to tagged. No new untagged criterion appears. FR-031-AC-26 is newly tagged but its green control misses the arithmetic-free child case.

Inherited untagged/ignored obligations remain, including FR-005-AC-3 and FR-016-AC-25 already retained/deferred by SR-15371. This review does not reassign or close them, create tickets, or certify repository-wide coverage. Matrix diagnostics are the same seven-line module warnings at both trees.

Plan completion: not assessed
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
      "id": "TypeClosure::FinishUnion",
      "path": "src/oracle/equality/resolution.rs",
      "role": "examined",
      "excerpt": "if payload != member.payload() {\n                        return Err(mismatch().into());\n                    }"
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
    }
  ],
  "findings": [
    {
      "fnd": "FND-001",
      "severity": "medium",
      "confidence": "high",
      "method": "gap-analysis",
      "check": "coverage",
      "artifact_id": "FR-031-AC-26",
      "related_ids": [
        "TC-044"
      ],
      "path": "tests/it/oracle_arithmetic.rs",
      "lines": "1646-1700",
      "excerpt": "Each completed connective, including unary BooleanNot, calls `quire_exact::retain_boolean` exactly once and charges `boolean.result-retain` once, including a result decided by the left; a child stop charges no connective result-retain point.",
      "finding": "AC-26 has one tagged retention test, but all its connective/Not cases selected for metered rendering hold arithmetic below the connective under test. It omits an arithmetic-free BooleanNot/connective child inside a metered oracle, so the existing green native retention check does not detect the product defect documented by code-review FND-001.",
      "requested_change": "Within an already metered oracle, route BooleanNot and connective subtrees through the metered renderer even when those subtrees contain no arithmetic. Preserve the whole arithmetic-free oracle's bool ABI. Add native budget=1 Incomplete and budget=2 Completed(false)/two-retain controls with skipped right arithmetic."
    }
  ]
}
```

