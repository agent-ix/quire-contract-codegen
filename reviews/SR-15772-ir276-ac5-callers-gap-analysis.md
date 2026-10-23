---
id: SR-15772
title: IR-276 CG caller pass gap analysis
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-276 AC-5 caller pass; src/evidence/v2_llvm.rs; src/kani/generate/clause/v2.rs; src/kani/generate/negotiate.rs; tests/clause_admission_support/mod.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-004", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-015", type: references }
---

## Summary

Planless, diff-scoped gap analysis of the same candidate head `7f0df7c996ca870834e73c8c6dabe9a9e7544852`. The changed production behavior is request-local lookup reuse. IR-276 AC-5 is a ticket criterion, so the computed CG matrix does not mint it; source-level counted work tests and branch history supply its evidence. Existing tagged public tests still bind the clause oracle and LLVM analyzer criteria. The companion code/Rust review SR-15771 records the architecture-boundary defect introduced by the test fixture placement.

## Verdict

PASS for the changed behavior's criterion-to-test and reverse-code checks; this does not assert completion of every AC-5 caller across CG or a full-repository gap audit. The code-review finding remains open.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No additional gaps found (placeholder) | - |

## Coverage

`quire matrix --scope . --format json` computed FR-004-AC-16 with five binders, AC-18 with one, AC-23 with five, AC-24 with five, and FR-014-AC-38 with thirteen at this head. The new evidence tests bind AC-16, AC-18, AC-23 and AC-24, while the unchanged public tests preserve the wider criteria. FR-015-AC-42 is still planned and has no binder; this PR neither claims nor implements that behavior. The changed Kani test compares full records and generated source against public/single-item baselines, including typed refusal, while the counted work test checks K=2,4,8 for three admitted clause populations. The LLVM test checks source/report bytes, population order, and refusal precedence. No stub, empty success, or unsupported new behavior was found in the changed production paths.

Plan completion: not assessed

Reverse scope: changed files only. The optional semantic audit was not requested. No aggregate gate was run by the reviewer.

## Examined scope and bindings

```yaml
scope:
  - id: IR-276-AC-5
    path: src/kani/generate/negotiate.rs
    role: examined
    excerpt: "No per-node or per-obligation path in IR or CG does a linear `find` or `filter` over all graph nodes, all declarations, or the whole source map."
  - id: FR-004-AC-16
    path: spec/evidence/functional/FR-004-vacuity-evidence.md
    role: examined
    excerpt: "The expected clause population and implication counts of the V2 analyzer equal those independently derived from the admitted `CheckedPackageV2` clause bodies."
  - id: FR-004-AC-18
    path: spec/evidence/functional/FR-004-vacuity-evidence.md
    role: examined
    excerpt: "A V2 clause whose generated source map declares an implication count unequal to the count derived from its body yields a structured binding non-success naming the clause and no classification, and is never treated as implication-free."
  - id: FR-004-AC-23
    path: spec/evidence/functional/FR-004-vacuity-evidence.md
    role: examined
    excerpt: "Over a V2 clause population, removing one clause, one consequent probe, one evaluation probe, or every clause from the generated artifacts, each alone, prevents successful analysis."
  - id: FR-004-AC-24
    path: spec/evidence/functional/FR-004-vacuity-evidence.md
    role: examined
    excerpt: "Over a V2 clause population, each malformed input class of FR-004-AC-5 yields a non-success outcome with no invented classification."
  - id: FR-014-AC-38
    path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
    role: context_only
    excerpt: "The Boolean condition in the third argument of a `state`/`state_clause` node's `quire.op.state.clause` body, addressed by the clause node id, derives a descriptor and generates an `ir_confirmed` oracle."
findings: []
bindings:
  - { test_id: request_view_keeps_complete_population_and_report_bytes_under_input_permutation, ac_id: FR-004-AC-16, trace: correct }
  - { test_id: request_view_keeps_population_and_artifact_refusal_precedence_without_partial_rows, ac_id: FR-004-AC-18, trace: correct }
  - { test_id: request_view_keeps_population_and_artifact_refusal_precedence_without_partial_rows, ac_id: FR-004-AC-24, trace: correct }
```

## Final PR-head recheck

At PR #545 head `7b5a4ab7e2bdbb9e965941d07820ac5900c7e412`, the new admission-only fixture preserves the QSL-emitted, strict-IR-admitted test input path without adding production behavior or new trace claims. The changed Kani measurement test reads its source and domain from that fixture directly. The five examined FR-004 bindings and the counted IR-276 work checks retain the same assertions; the production patch from candidate `7f0df7c9` is byte-identical. No new test/criterion or reverse-code gap was introduced by the fixture repair. Scoped PASS carries; SR-15771's FND-001 is fixed.

## Incremental collection-cache gap recheck

Scoped PASS at PR #545 head `87445bd4f50d6ab5961b2606e64c5e45e213051f`, base `b68193997a15fd37ea3d25132c39f83138ed54dc`. The previous nine reviewed commits have equal `git range-diff` entries, and the original SR-15771/15772 blobs remain byte-identical. CG #547's merged graph API supplies the package-bound, first-match owner index; this PR's new production behavior is only request-local reuse of that graph. The two new tests exercise actual QSL-emitted, strict-IR-admitted collection items, count the real graph constructor across K and R, and compare complete ordered records/harnesses against singleton outputs, including a typed refusal and two-package interleaving. They create no new FR trace tag: IR-276 AC-5 is a ticket criterion outside the computed CG matrix. Existing collection behavior criteria remain covered by their prior tagged tests; no stub or new unowned behavior was found in this incremental slice. The private runtime archive could not be read from this host (HTTP 401); this reviewer checked the committed counters, test assertions and test-first history, not an independently run gate. Plan completion: not assessed. No repository-wide AC-5 completion claim.
