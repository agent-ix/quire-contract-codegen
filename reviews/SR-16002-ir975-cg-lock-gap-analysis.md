---
id: SR-16002
title: IR-975 CG lock adoption gap analysis
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #546; lock adoption and changed CheckedPackage V2 test fixtures"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-014", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-021", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-039", type: references }
---

## Summary

Planless, diff-scoped gap analysis of CG PR #546 at `dffb2483cf838c6719704cf8d74bbcdc36997df6`. The change adopts the single-body Contract IR wire through one coherent lock revision and updates only test fixture construction and assertions. No CG production consumer path changes. The computed matrix still binds the directly affected scalar, function and selected-dependency criteria. The companion code/Rust review SR-16001 records a stale comment, not a missing behavior or test.

## Verdict

PASS for changed criterion-to-test bindings and reverse-code scope. This is a PR-diff claim, not a full-repository or aggregate-gate PASS. QSL-1009's four `IdentityDivergedFromIr` package-unit failures remain an explicit external dependency.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No additional gaps found (placeholder) | - |

## Coverage

`quoin matrix --repo <PR head> --json` reports FR-014-AC-44 tagged by two direct unsupported-root tests, FR-021-AC-33 by five direct/body-root tests, FR-021-AC-34 by the strict-reader 129-function chain and rekey controls, and FR-039-AC-3 by selected target and forged same-key tests. The changed fixture hash path uses current graph nodes and the owned IR borrowed preimage type; the transmitted selection object has no derived projection. The size control asserts absence of the removed member. No new product source, shim, stub, or unowned behavior was introduced. The QSL divergence is documented as a known dependency, not hidden as a passing gate.

Plan completion: not assessed

Reverse scope: changed files only. The optional semantic audit was not requested. No aggregate gate was run by the reviewer.

## Examined scope and bindings

```yaml
scope:
  - {id: FR-014-AC-44, path: "spec/oracle/functional/FR-014-exact-scalar-oracles.md", role: examined, excerpt: "For each IR-admitted direct request naming a `temporal`/`formula`, `temporal`/`fairness` or `expression`/`case` node, the scalar claim map has exactly one entry with that requested node id and `ExactScalarRefusal::Unsupported`."}
  - {id: FR-021-AC-33, path: "spec/oracle/functional/FR-021-function-application-oracles.md", role: examined, excerpt: "With an IR-admitted `temporal`/`formula`, `temporal`/`fairness` or `expression`/`case` node used directly as an item's call-node id, the claim-map entry retains that id and is `Refused { refusal: ExactFunctionRefusal::Unsupported }`."}
  - {id: FR-021-AC-34, path: "spec/oracle/functional/FR-021-function-application-oracles.md", role: examined, excerpt: "Given one authentic strict-reader-admitted `CheckedPackageV2` of less than 1,048,576 bytes containing 129 distinct functions with two bound Integer parameters each, 128 acyclic `quire.op.function.call` body-root edges forming one chain and a terminating admitted Scalar body, the public function generator schedules all 129 functions and all 128 call edges from that same checked graph."}
  - {id: FR-039-AC-3, path: "spec/core/functional/FR-039-contract-package-consumption.md", role: examined, excerpt: "A selected dependency function reference resolves only through the reader-validated selected package identity and target key. A missing, wrong-identity or stale selected dependency refuses before CG; an identically keyed local node or extra unselected dependency cannot redirect generation."}
findings: []
bindings:
  - {test_id: tc_024_ac44_direct_temporal_formula_is_unsupported, ac_id: FR-014-AC-44, trace: correct}
  - {test_id: tc_031_ac33_direct_fairness_call_node_is_unsupported, ac_id: FR-021-AC-33, trace: correct}
  - {test_id: tc_031_ac34_public_strict_reader_chain_generates_compilable_flat_schedule, ac_id: FR-021-AC-34, trace: correct}
  - {test_id: checked_reader_refuses_forged_same_key_local_function, ac_id: FR-039-AC-3, trace: correct}
```

## Final PR-head recheck

PASS carries to exact PR #546 head `3e1fa85a3a76f71b14ae9e63ff7dd41fd3b32b0b` on unchanged base `2cdce220567878a20170088606e049ed67cf9cc2`. The only post-review delta edits the 129-function fixture comment; the four examined criterion bindings, test bodies, lock, and production code remain byte-identical to the reviewed head. This is no new aggregate-gate result, and QSL-1009 remains a separate dependency.
