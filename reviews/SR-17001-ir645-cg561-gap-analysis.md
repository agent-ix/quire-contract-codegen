---
id: SR-17001
title: "IR-645 CG #561 incremental gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen#561; FR-030, FR-015-AC-52, TC-041; src/kani/terminal.rs, generate/corpus/bounded_kani_corpus.rs; tests/it/terminal_map.rs, bounded_kani_corpus.rs"
review_set: subset
---

## Summary

Ticket: IR-645. Incremental planless analysis of the changed FR-030 to TC-041 to implementation chain at exact head `0a6d44b71da57f51b9216305655a648096e22cf6`. Plan completion: not assessed. The `Proved(3)` and `u32` overflow tests bind to changed mapper behavior, but the three IR constructor refusal controls stop at `is_err()`.

## Examined criteria

- FR-030-AC-1 (examined): valid pair cardinality and overflow refusal.
- FR-030-AC-4 (examined): positive retained count, zero-check refusal, and overflow refusal.
- FR-030-AC-5 (examined): vacuous proof and refusal of another Inconclusive cause.
- FR-030-AC-8 (examined): admitted Unavailable causes and refusal of another cause.
- FR-030-AC-18 (examined): basis and certification source.
- FR-015-AC-52 (context_only): the generated corpus case is not a run verdict.
- TC-041 (examined): procedures 1-7 and expected results 1-7.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-041 now requires inspection of typed refusals for a zero-check Proved, an invalid Inconclusive cause, and an invalid Unavailable cause. The three tagged tests assert only `.is_err()`, so a wrong IR error cause or generic error would pass. Assert the expected `KaniOutcomeError` variant/code for all three controls, preserving the already exact over-range `TerminalPairError` assertion. | tests/it/terminal_map.rs:1045, tests/it/terminal_map.rs:1077-1080, tests/it/terminal_map.rs:1235-1241 |
| FND-002 | high | The changed corpus success path records a positive `success_checks` value before any Kani execution, and a public terminal mapper can certify it as Checks(1). This is the code-review FND-001 semantic gap; the IR-1002 follow-up does not make the new proof-count claim an observed transcript count. | src/kani/generate/corpus/bounded_kani_corpus.rs:507-515, src/kani/terminal.rs:192-212 |

## Verdict

**FAIL** for a reachable false proof count and incomplete refusal oracles. The changed FR-030 mapping itself uses the retained count with a checked conversion. Existing broader repository coverage was not claimed by this diff-scoped audit.

## Coverage

Plan completion: not assessed. The changed tests exercise Proved(3) and overflow; the three constructor-refusal controls require exact causes to back TC-041's expected results. No aggregate gate was run by this reviewer.

## Dispositions

Disposition pass 1 at exact head `b7dedc2776608b5f84941af3ff609d9a2b78433a` (parent `0a6d44b71da57f51b9216305655a648096e22cf6`).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | TC-041 now checks `requested_kind`, `requested_code`, and `rejected_success_checks` for zero-check Proved, both invalid Inconclusive codes, and invalid Unavailable. The prior `.is_err()`-only assertions are gone; exact over-range `TerminalPairError` remains. |
| FND-002 | fixed | Corpus success returns `BoundedCorpusExpectation` instead of a `KaniOutcome`; its predicted check count cannot be passed to the terminal mapper. The public export, replay use, and TC-023 controls use expectation semantics. See SR-17000 FND-001. |

## Narrow delta coverage

**PASS** for the amended FR-015-AC-52/91/95 and TC-023 assertions on the changed path. TC-023's source guard checks the public case field and both removed pre-run Kani constructors; the compiled type boundary is the primary prevention. Plan completion: not assessed. This is an incremental PR review, not a full-repository coverage claim.

## Review ID correction

This artifact supersedes the same-method IR-645 CG #561 reviewer record originally labeled with a colliding SR id. The finding and disposition substance is unchanged; only the review ids and cross-references were reissued.

## Narrow gate-fix delta, pass 4

At exact head `fc896ff69dd767101e608d712f60f6055faca3bc`, the FR-017-AC-42 cancellation/timeout control still uses the real launch, pidfd, memory and capture path, and no trace tag was added or removed. The test-only confirmation poll's unbounded wait creates the observability defect recorded as SR-17000 FND-003: a failed cleanup can hang the test rather than produce the required fault evidence. No spec text changed. Plan completion: not assessed; no full-repository coverage claim.

## Narrow gate-fix disposition, pass 5

**PASS** at exact head `3355357e9be50ba3dfa305202704dc14046be1fe`. The test-only pidfd confirmation now has a finite 30-second wait, so the same AC-42 control can return a typed unconfirmed-teardown error if cleanup fails. The real process observation and the ordinary production five-second confirmation are unchanged. SR-17000 FND-003 is fixed. Plan completion: not assessed; no full-repository coverage claim.
