---
id: SR-35031
title: "IR-348 PR 570 L-6 gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; PR #570 AD-004 L-6, FR-016, FR-017 and FR-032 trace"
review_set: subset
---
# IR-348 PR 570 L-6 gap analysis

## Summary

The computed matrix binds the affected FR-016, FR-017 and FR-032 criteria to tests. The AD-004 L-6 source boundary still has one unmet path in state-clause replay.

## Verdict

**FAIL** — the raw Kani check-clause reader in replay is an implementation gap against L-6.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AD-004 L-6 requires Kani console wording to be read only in `kani/output`; state-clause replay checks a raw Kani assertion suffix. | spec/assurance/AD-004-cg-crate-layout.md:923; src/replay/state_clause.rs:550 |

## Coverage

Plan completion: not assessed. `quire matrix --scope . --format json` at the frozen head reported 927 criteria: 557 tagged, 301 untagged, 16 tagged only by ignored tests and 53 methods without a symbol. The untagged and ignored-only counts are repository-wide existing debt and are not attributed to this PR. The affected FR-016-AC-1/6/25, FR-017-AC-5/12 and FR-032-AC-2/3 are all tagged (3/6/4, 6/9 and 7/10 binders respectively). Reverse trace of the changed public parser and decoder APIs found FR-016, FR-017 and FR-032 ownership; AD-004 L-6 owns the architectural boundary. No changed source or test stub was found. Quoin evidence-backed matrix was not used; the static Quire matrix was used. Optional semantic review was not requested.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | At 209563808da36b89aafe6e29da468b0e89ab65a6, the `src/replay/state_clause.rs` call to `parsed.matches_generated_postcondition(&expected)` consumes a parsed playback; the output reader alone checks the raw assertion suffix. The tagged FR criteria retain their test bindings. |
