---
id: SR-667001
title: "CG #667 digest API deletion code and Rust review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR #667 8cc398a5; Cargo.lock, src/kani/{identity,generate/collection,run/execute}.rs, src/replay/{composite_builder,frame,function,state_clause}.rs, tests/exact_function_support/package.rs, tests/it/{composite_original_eq,composite_parity_converter,exact_function_generation,kani_obligations_state_clause_replay,kani_obligations_state_frame}.rs, tests/state_frame_support/native_twin.rs; IR-1181, IR-1133"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-017", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-024", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-033", type: references }
---

## Summary

Reviewed the exact PR #667 head 8cc398a5 against origin/main with the Rust idiom and boundary checks. Examined FR-017-AC-61, FR-024-AC-52/53, FR-033-AC-19/20 and the retired FR-017-AC-13 row. Inspected the changed source and tests, QSL lock, retained source-byte custody, typed refusals, duplicate document identities, and absence of digest substitutes. The author's focused fmt, check, Clippy, Kani marker, composite, converter, state-clause and exact-function test results were read from the PR description; this reviewer did not rerun builds. The broad state-frame run reports two unrelated TC-025 failures outside the changed TC-035 tests.

## Verdict

**CONDITIONAL** at 8cc398a5: the implementation preserves the examined product behavior, but six live tests still trace the retired FR-017-AC-13 while its replacement AC-61 has no binder. Fix this trace assignment on the original PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Six live Kani controls tag retired FR-017-AC-13 and none tag replacement FR-017-AC-61, so the required collection proof evidence has no current trace binding. Retag only the tests that actually exercise AC-61 and remove retired credit. | src/kani/run/execute.rs:1352; src/kani/classify.rs:418; tests/it/kani_obligations_symbolic_domains.rs:2302 |

## Coverage

- FR-017-AC-61: examined; its seal, byte-identical crate occurrence, assertion line, three entry points, batch atomicity and report-location controls were inspected. The matrix reports zero binders.
- FR-017-AC-13: examined; retired in place and earns no credit, but the matrix still reports six live test binders.
- FR-024-AC-52/53: examined; state documents use self-declared four-label identities, duplicate labels refuse before request construction, and canonical bytes come from the existing encoder. The matrix reports four and three binders respectively.
- FR-033-AC-19/20: context only; both explicitly remain Planned, and this PR does not claim implementation credit. The independent original source argument compares full identity and raw bytes.
- Plan completion: not assessed.

## Dispositions

Disposition pass 1 at head 0ba810c587f7d850603c421a4ff075032f0cae29. The only delta from the reviewed head changes six `#[trace]` bindings and one adjacent doc comment; product code is unchanged. Each test exercises the replacement's zero-check, bound-assertion, three-entry, or installed-Kani collection controls. Source/tests contain no FR-017-AC-13 trace tag.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | All six tests now trace FR-017-AC-61 where their assertions back that criterion; the retired AC-13 has no source/test tags. |
