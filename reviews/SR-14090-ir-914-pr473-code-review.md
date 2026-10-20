---
id: SR-14090
title: "IR-914 PR #473 Code review"
type: SpecReview
analysis: code-review
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
| FND-001 | low | No findings (placeholder) | src/core/ir/clause.rs, src/oracle/clause.rs, src/oracle/scalar/mod.rs, src/lib.rs, tests/it/kani_obligations_state_frame.rs, tests/state_frame_support/native_twin.rs |

## Verdict

PASS. The checked-package reader derives clause identity and bounded inputs; the scalar operation renderer is reused; generated functions and claim map preserve the clause node. No vendored file, hash/digest/pin tracking, duplicated operation catalog or weakened CI lane was found. The tests-first commit precedes implementation, new tests use `#[trace("FR-014-AC-38")]`, and the native LLVM regression measures zero on the skipped consequent token. The pre-PR full CI and real Kani results were reported by the author; this reviewer did not repeat full gates.

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

## Narrow rebase check

PR #473 new clean remote head `e88c6d288e5c3658b164b508c9d202dbd4a5efce` rebases the reviewed patch onto CG #472. All previously reviewed Rust and test file blobs and the changed matrix blob are byte-identical to the frozen review head. The only PR-own added change is the FR-014-AC-38 planned-status correction. `git diff --check` and scoped Quire validation pass. No new finding.
