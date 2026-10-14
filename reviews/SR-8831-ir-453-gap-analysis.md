---
id: SR-8831
title: "IR-453 PR 382 gap analysis"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@8c6f5a67542c80f05601999bdeddcc0727b1eb94; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/core/non-functional/NFR-005-no-generation-panics.md, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, src/kani/generate/corpus/bounded_kani_corpus.rs, src/replay/function.rs, src/replay/witness.rs"
---

## Summary

Ticket: IR-453. Planless; plan completion not assessed. The computed matrix keeps FR-015-AC-57 tagged only by ignored real-Kani tests and FR-015-AC-82 through AC-93 and AC-95 untagged, consistent with TC-023's existing status note. The new Kani replay test did pass when explicitly run; the NFR-005-AC-1 source scan fails due to newly added panic tokens. Oracle-strength review found false-only replay cases leave true-result QSL semantics unchecked.

## Verdict

FAIL. NFR-005-AC-1's executable source scan detects new `expect` and `unwrap` calls in the changed corpus source. The false-only Kani-to-QSL replay test does not establish the generated QSL semantics for true graph or collection cases.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The changed corpus implementation adds production panic tokens that make the NFR-005-AC-1 scan fail. The failure is observable in `tc_042_ac1_src_holds_no_panic_token`. | src/kani/generate/corpus/bounded_kani_corpus.rs:822, 995-996 |
| FND-002 | medium | Both new tests pass if every generated graph and collection QSL predicate is hard-coded false; true cases are only compiled, not semantically checked. | src/kani/generate/corpus/bounded_kani_corpus.rs:789-855, 1197-1287, 1289-1309 |

## Coverage

Plan completion: not assessed. `quoin matrix --repo . --json` reports FR-015-AC-57 as `tagged-by-ignored-test`; AC-82 through AC-93 and AC-95 are untagged; AC-94 is tagged. These rows were not changed in this PR. The new Kani replay behavior is exercised only by `tc_023_real_corpus_playback_replays_through_its_qsl_source`, marked ignored for the Kani lane. `KaniTemp::run` combines that process's exported report and output; `classify_kani_run` takes a transcript only after the report has a failed property, and `decode_playback` checks the transcript harness. Replay behavior is limited to the changed corpus path. Its explicit run passed. The compile-only test includes reachable graph and true collection cases but never evaluates their QSL predicate. The real-Kani test only replays unreachable graph and false collection inputs; replacing both QSL renderers with constant `false` preserves those test outcomes while breaking every true case. No reverse-gap inventory of unrelated repository modules was made; this review's reverse-gap scope was limited to the changed replay behavior.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Production graph rendering and dependency-name construction now propagate errors instead of panicking. |
| FND-002 | fixed | The focused test evaluates retained QSL for every corpus case and compares it with the case's Boolean claim. |
