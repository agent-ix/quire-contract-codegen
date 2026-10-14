---
id: SR-8830
title: "IR-453 PR 382 code review with Rust lane"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@8c6f5a67542c80f05601999bdeddcc0727b1eb94; spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, src/kani/generate/corpus/bounded_kani_corpus.rs, src/replay/function.rs, src/replay/witness.rs"
---

## Summary

Ticket: IR-453, PR #382. Code review and the Rust lane found production panic tokens introduced in the corpus module. Oracle-strength review found the source semantics can be wrong while both new tests pass. The actual Kani lane was also run; its falsification playback is classified from the Kani report plus concrete-playback console block, checked against the generated harness identity, and replayed against the case's compiled QSL source.

## Examined Units

- NFR-005-AC-1 (examined): no panic tokens in non-test source under `src/`.
- FR-015-AC-57 (examined): installed Kani falsification carries playback.
- TC-023 (examined): source-backed corpus replay procedure and expected outcomes.
- `render_graph_qsl`, `render_collection_qsl`, `compile_qsl_package`, `tc_023_each_corpus_case_compiles_its_retained_qsl_source`, `false_cases`, `KaniTemp::run`, `classify_kani_run`, `tc_023_real_corpus_playback_replays_through_its_qsl_source`, `ReplayPackage::harness_identity`, `replay_harness_counterexample_through`, and `decode_playback` (examined).

## Verdict

FAIL. The new production source adds `expect` and `unwrap` calls in `src/kani/generate/corpus/bounded_kani_corpus.rs`, which the existing NFR-005-AC-1 source scan detects. The focused scan test fails at this head. The real Kani playback and QSL replay test passed, establishing that the returned transcript comes from a falsified generated harness and is joined to its expected harness and QSL package. The false-only real-Kani cases do not verify that the QSL renderer preserves true graph and collection results.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | New production `expect`/`unwrap` calls in `render_graph_qsl` and `compile_qsl_package` violate NFR-005-AC-1; the source scan fails with both tokens in this file. | src/kani/generate/corpus/bounded_kani_corpus.rs:822, 995-996 |
| FND-002 | medium | Both new tests pass if every generated graph and collection QSL predicate is hard-coded false; true cases are only compiled, never semantically checked. | src/kani/generate/corpus/bounded_kani_corpus.rs:789-855, 1197-1287, 1289-1309 |

## Coverage

`cargo test tc_042_ac1_src_holds_no_panic_token -- --nocapture` failed and reported `src/kani/generate/corpus/bounded_kani_corpus.rs: ["expect", "unwrap"]`. `cargo test tc_023_real_corpus_playback_replays_through_its_qsl_source -- --ignored --nocapture` passed (1 test). The latter invokes `cargo kani -Z concrete-playback -Z unstable-options --export-json ... --harness <case symbol> --concrete-playback print`; `classify_kani_run` requires the exported report to show a failed property and obtains the transcript from console output. The decoder then checks the playback's generated harness symbol before QSL replay. Oracle check: `false_cases()` supplies only unreachable graph and false collection cases; `tc_023_each_corpus_case_compiles_its_retained_qsl_source` compiles true cases but does not evaluate them, so a constant-false QSL graph/collection mutant survives both tests. No full suite was run.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Production graph rendering and dependency-name construction now propagate errors instead of panicking. |
| FND-002 | fixed | The focused test evaluates retained QSL for every corpus case and compares it with the case's Boolean claim. |
