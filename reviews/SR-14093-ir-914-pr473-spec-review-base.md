---
id: SR-14093
title: "IR-914 PR #473 Base specification review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #473 head 3674038a33306bbf34519f2720158e2f297cf521; IR-914; FR-014-AC-38"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
---

## Summary

Non-author review of the frozen PR #473 diff. Ticket: IR-914. Rust, spec matrix, test trace and generated LLVM probe path were inspected against FR-014-AC-38.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Matrix marks AC-38 covered while the criterion remains PLANNED (IR-489) | spec/oracle/matrix/tests.md:28; spec/oracle/functional/FR-014-exact-scalar-oracles.md:511 |

## Verdict

CONDITIONAL. The changed matrix row makes a completed-coverage claim for FR-014-AC-38, but the governing criterion still starts `PLANNED (IR-489)`. A reader cannot tell whether AC-38 itself is delivered or remains planned. Reconcile that status in the criterion while preserving the separate planned FR-015-AC-40 harness work. No new acceptance wording or tracking record was introduced by the matrix row.

## Coverage

```yaml
scope:
- id: FR-014-AC-38
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: PLANNED (IR-489). The Boolean condition in the third argument of a `state`/`state_clause`
    node's `quire.op.state.clause` body, addressed by the clause node id, derives
    a descriptor and generates an `ir_confirmed` oracle over its connectives, integer
    comparisons and integer add, subtract, multiply and negate, returning `Outcome<bool>`.
- id: FR-014-AC-7
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: context_only
  excerpt: Composite, collection, function, model, relation, state, temporal and protocol
    nodes are refused with their blocked or unsupported reason and never reported
    as generated.
- id: FR-014-AC-10
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: context_only
  excerpt: An operand that is neither a literal nor a reference is refused by CG with
    a typed reason, subject to FR-014-AC-38 field-read terms.
- id: TC-024
  path: spec/oracle/matrix/tests.md
  role: examined
  excerpt: ✅ Covered; the clause-identified descriptor and one-item claim map cover
    QSL-emitted referenced conditions and an admitted inline condition. Compiled runtime
    cases exercise declared field endpoints, a bounded operation parameter, connectives,
    comparisons and the four supported integer operations; non-clause and unsupported
    `pre(...)` controls refuse. FR-015-AC-40's harness embedding remains planned (IR-489).
bindings:
- test_id: skipped_v2_clause_consequent_has_a_measured_zero_native_probe
  ac_id: FR-014-AC-38
  trace: correct
```

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | On `e88c6d288e5c3658b164b508c9d202dbd4a5efce`, FR-014-AC-38 no longer begins `PLANNED (IR-489)`; the matrix marks AC-38 covered and separately states FR-015-AC-40 harness embedding remains planned. The criterion's substantive wording is unchanged. |

## Narrow rebase check

The rebased PR changes no previously reviewed Rust, test, or matrix blob. The only PR-own additional edit is the FR-014-AC-38 status correction. The new head is clean, the remote PR head matches, and scoped Quire validation and `git diff --check` pass. No new finding.
