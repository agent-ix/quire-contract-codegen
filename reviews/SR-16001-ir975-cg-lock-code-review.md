---
id: SR-16001
title: IR-975 CG lock adoption code and Rust review
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #546; Cargo.lock; src/routed/contract_package_tests.rs; tests/checked_package_support/base.rs and rekey.rs; tests/composite_equality_support/package.rs; tests/exact_scalar_support/package.rs; tests/it/exact_function_generation.rs and exact_scalar_generation.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-014", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-021", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-039", type: references }
---

## Summary

Independent diff-scoped code and Rust review of CG PR #546 at `dffb2483cf838c6719704cf8d74bbcdc36997df6` on base `2cdce220567878a20170088606e049ed67cf9cc2`. `Cargo.lock` changes only the `quire-contract-ir` and `quire-contract-model` entries from `8810495e` to merged IR revision `bbc581f5`; each has one entry, with no manifest or other cohort change. The built owner `quire check-lock Cargo.lock` passes. The fixture rewrite keeps `identity_projection` in the package-id hash input while omitting it from the wire, derives the projection from current graph nodes, and obtains extension definitions from the QSpec checkout selected by `QSPEC_DIR`. The same-key collision test hashes a local copy, leaving the transmitted selection-only wire intact. No compatibility reader, source copy, production CG behavior change, unsafe block, or new Cargo dependency appears in this diff.

## Verdict

CONDITIONAL — one low-severity stale fixture comment describes the removed duplicate projection. The author-reported seven focused RED/GREEN controls and wider confined suites are recorded as author evidence, not as reviewer-run gates. Four routed package tests remain red at QSL `IdentityDivergedFromIr` under locked QSL `485ccaa3`, tracked separately as QSL-1009; this review does not certify aggregate readiness.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The 129-function fixture comment still says each node repeats an identity-projection record, though this PR removes that wire member; its 1 MiB size explanation is stale. | tests/it/exact_function_generation.rs:65 |

## Coverage

The changed Rust is test support and tests. The public fixture still admits through `CheckedPackageV2::read`; `refresh_identity` constructs `CheckedPackageIdentityPreimageRefV2` from typed selection data and current typed graph nodes. The published QSpec checkout at `b3f3c803` has a selection-only `identity_preimage` and extension definitions selected by definition reference. FR-014-AC-44, FR-021-AC-33, FR-021-AC-34 and FR-039-AC-3 retain tagged tests. The owner check-lock result and parsed old/new Cargo.lock each show one Contract IR and one Contract Model revision, both at the same commit. No full test, Kani, or aggregate gate was run by this reviewer. The author's private raw archive returned HTTP 401 to this reviewer, so RED/GREEN log internals were not independently replayed; source, scope, and summary were inspected.

## Examined scope and bindings

```yaml
scope:
  - {id: FR-014-AC-44, path: "spec/oracle/functional/FR-014-exact-scalar-oracles.md", role: examined, excerpt: "For each IR-admitted direct request naming a `temporal`/`formula`, `temporal`/`fairness` or `expression`/`case` node, the scalar claim map has exactly one entry with that requested node id and `ExactScalarRefusal::Unsupported`."}
  - {id: FR-021-AC-33, path: "spec/oracle/functional/FR-021-function-application-oracles.md", role: examined, excerpt: "With an IR-admitted `temporal`/`formula`, `temporal`/`fairness` or `expression`/`case` node used directly as an item's call-node id, the claim-map entry retains that id and is `Refused { refusal: ExactFunctionRefusal::Unsupported }`."}
  - {id: FR-021-AC-34, path: "spec/oracle/functional/FR-021-function-application-oracles.md", role: examined, excerpt: "Given one authentic strict-reader-admitted `CheckedPackageV2` of less than 1,048,576 bytes containing 129 distinct functions with two bound Integer parameters each, 128 acyclic `quire.op.function.call` body-root edges forming one chain and a terminating admitted Scalar body, the public function generator schedules all 129 functions and all 128 call edges from that same checked graph."}
  - {id: FR-039-AC-3, path: "spec/core/functional/FR-039-contract-package-consumption.md", role: examined, excerpt: "A selected dependency function reference resolves only through the reader-validated selected package identity and target key. A missing, wrong-identity or stale selected dependency refuses before CG; an identically keyed local node or extra unselected dependency cannot redirect generation."}
findings:
  - fnd: FND-001
    severity: low
    confidence: high
    method: code-review
    check: other
    artifact_id: FR-021-AC-34
    related_ids: []
    path: tests/it/exact_function_generation.rs
    lines: "65"
    excerpt: "Given one authentic strict-reader-admitted `CheckedPackageV2` of less than 1,048,576 bytes containing 129 distinct functions with two bound Integer parameters each, 128 acyclic `quire.op.function.call` body-root edges forming one chain and a terminating admitted Scalar body, the public function generator schedules all 129 functions and all 128 call edges from that same checked graph."
    finding: "The fixture comment still says it avoids repeating each node's `identity-projection` record. The new wire has no `identity_projection` member, so the size explanation is stale. Update the comment to describe graph and source-map records only."
bindings:
  - {test_id: tc_024_ac44_direct_temporal_formula_is_unsupported, ac_id: FR-014-AC-44, trace: correct}
  - {test_id: tc_031_ac33_direct_fairness_call_node_is_unsupported, ac_id: FR-021-AC-33, trace: correct}
  - {test_id: tc_031_ac34_public_strict_reader_chain_generates_compilable_flat_schedule, ac_id: FR-021-AC-34, trace: correct}
  - {test_id: checked_reader_refuses_forged_same_key_local_function, ac_id: FR-039-AC-3, trace: correct}
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | PR #546 head `3e1fa85a3a76f71b14ae9e63ff7dd41fd3b32b0b` changes the two stale comment lines to describe only shared graph-node and source-map records. The test body, product code, manifest, and lock are unchanged from the reviewed head. |

## Final PR-head recheck

PASS at exact head `3e1fa85a3a76f71b14ae9e63ff7dd41fd3b32b0b` on base `2cdce220567878a20170088606e049ed67cf9cc2`. The complete `dffb2483..3e1fa85a` diff touches only the comment in `tests/it/exact_function_generation.rs:63-66`; no new code or test behavior was introduced. The originally examined lock and fixture behavior carries. The private raw RED/GREEN archive still returns HTTP 401, and no accessible replacement logs were posted at disposition time, so the review continues to distinguish author-reported focused results from independently observed evidence. The QSL-1009 package-unit failure and CG #543 merge hold remain outside this finding's disposition.
