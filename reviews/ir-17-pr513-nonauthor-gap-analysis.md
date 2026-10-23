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

## Dispositions

Round 1, independently inspected at published head 82051284537d9468f03b8a158a3e4f3605309a3f. Original findings and the original-head verdict above remain unchanged.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | The tagged native regression exercises the previously missing pure Boolean subtree inside a metered root. It checks all result budgets, exact retain-only charge counts and skipped arithmetic; old code fails six shapes and the fixed code passes the wrapper requiring all seven. |

### Narrow evidence readback

Compared cd202f1b8fda0c23428c19aa47c40ce3769ff76c to 82051284537d9468f03b8a158a3e4f3605309a3f. The nonauthored product delta is the Boolean renderer guard and the two scalar ownership descriptions, plus the new native regression. No normative requirement changed. Unit-metadata files and integration coverage are excluded as before and remain with the official independent reviewer.

Read the actual fix-round-3 handoff, pure-connective-red.log, pure-connective-green.log/JSON and focused-summary.json. The red run compiled old production and failed six of seven shapes at budget 1 with Completed(false); the pure comparison leaf passed. The restored green native wrapper passed and requires all generated checks. The new cases cover Not, short And/Or, total And/Or, implication and the comparison leaf; each checks every result budget through its required retain count, exact consumed/admitted charges and no unreachable arithmetic charge. Existing retention, first-stop, unmetered byte-identity and unmetered comparison controls have author-run zero-exit evidence, as do format/lint/spec. These are inspected author-run results; the reviewer ran no Cargo, tests, Kani or backend.

The frozen branch's copied review artifacts matched the prior scratchpad versions before this append. The scratchpad copies additionally remove trailing presentation whitespace for final diff-check custody; findings, original structured data and original verdict text are preserved.

**Current scoped verdict: PASS** for the two nonauthored product findings and the associated gap at this exact new head; no new finding from this narrow delta. This does not accept the excluded authored slice, inherited repository assurance gaps, or final make ci/Kani outcomes. Parent owns the final gates after all independent dispositions.

```yaml
{
  "round": 1,
  "head": "82051284537d9468f03b8a158a3e4f3605309a3f",
  "dispositions": [
    {
      "fnd": "FND-001",
      "outcome": "fixed",
      "after_excerpt": "fn tc_044_pure_connective_subtrees_retain_within_a_metered_root() {\n    let mut modules = Vec::new();\n    let mut checks = String::new();\n    for (index, (label, flag, expected_retains)) in [\n        (\"not\", true, 2_u64),\n        (\"short_and\", false, 2),\n        (\"short_or\", false, 2),\n        (\"total_and\", true, 3),\n        (\"total_or\", true, 4),\n        (\"implication\", true, 3),\n        (\"comparison_leaf\", true, 1),\n    ]",
      "reason": "The tagged native regression exercises the previously missing pure Boolean subtree inside a metered root. It checks all result budgets, exact retain-only charge counts and skipped arithmetic; old code fails six shapes and the fixed code passes the wrapper requiring all seven."
    }
  ]
}
```

## Round 5 merged-cohort readback

Independent narrow readback at clean published head 3de23f5793b2cf4e301de1713cbc70a106bfa434, reconciled with CG main 43d5a6fdc3743835f90b1d6b720cb9195a7db1dd. Earlier original findings and round-1 fixed outcomes remain unchanged; no finding is reopened and no new finding is added. The authored UnitTable files/integration slice remains excluded.

Compared the fixture delta from the last accepted product head 82051284537d9468f03b8a158a3e4f3605309a3f. All equality production source before the main test module is byte-identical; the previously fixed Boolean renderer and scalar ownership text are also unchanged. No Cargo.toml or normative spec delta is present. The Makefile/stdout-capture work brought in by reconciliation is identical to current main, not an IR-17 gate-script change.

The large source generator remains unchanged: 50,000 optional records for the deep control and 65,536 records for work accounting. Only finite fixture compiler intake is enlarged via public StageLimits::with_named: actual source bytes, bytes+1 tokens, and 8*records+32 syntax nodes, with matching public ReplayLimits input bytes. Production defaults, the resolution work limit, both 512KiB stacks, domain/proof bounds and the default/exact/one-byte-less publication assertions remain. The work test's at-limit output is inspected with the source allowance already chosen for that work test; the separate publication-bound controls are preserved.

Cycle controls now supply the genuine recursive nominal owner context, retaining repeated-node and healthy-sibling assertions and adding subprocess diagnostics. The renderer isolation fixture initializes the genuine package identity and prepares an owner-admitted Quantity plan that actually reaches render_item -> UnsupportedValueType; the deliberately injected whole-call generation error remains separately identified. These private seams do not become public composite Quantity support.

The deep provenance assertion now locates the module that reconstructs the actual Chain0 owner declaration rather than assuming genuine keys sort first; all-module subject checks remain. Independent native reconstruction recompiles the same retained source with the same finite intake allowances outside the unchanged runtime worker, and the scratch Cargo.lock is seeded from the executing root cohort.

Inspected actual merged-cohort handoff/final-summary.json and four raw logs under qualification-logs/ir17-main-landing/fix-round-5-merged. Each exact filter matched one test and passed; deep native control ran for about 202 seconds. Logs resolve QSL 485ccaa3 (merged QSL-960), not the former local PR head. The actual root lock and recorded resolution retain one coherent first-party copy; reviewer ran only scripts/check_one_copy.awk, source/metadata comparisons and diff checks. All 268 metadata package name/version/source/manifest-path tuples before and after the last IR lock alignment compare identical, justifying the absence of another expensive rerun for that alignment. Author-run fmt/lint/Quire results are zero exit. Local PR-head qualification is kept distinct from this merged-cohort evidence.

**Current scoped verdict: PASS at 3de23f5793b2cf4e301de1713cbc70a106bfa434.** No reviewer Cargo, Kani, backend, branch edit or push. Whole-repository matrix/assurance inventory and the excluded UnitTable slice are not newly accepted; no full-matrix recomputation or original finding disposition was needed for this unchanged-normative fixture delta. Final aggregate and canonical Kani gates remain parent-owned and unrun in this round.
