---
id: SR-17940486709564847047611
title: "IR-304 slice (a) PR #772 gap analysis and spec conformance"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #772 (head 90aec995, base 6bc424e9); IR-304 slice (a); FR-022-AC-26, AC-28, AC-31, AC-36, AC-37, AC-38; FR-015 path_guard_not_lowered; TC-033; spec/routed/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-033
    type: reviews
---

## Summary

Ticket: IR-304 (slice (a) only). Compared the merged FR-022 text ("Per-occurrence requirement
records", "Frames", "Where the checks run", "Per-item precedence", "Scoped evidence status"
slice (a)/(b) split, Behavior bullets), FR-015's `path_guard_not_lowered` code, TC-033 and
`spec/routed/matrix/tests.md` against the code and the tagged tests. `quire matrix` resolves
every new tag: AC-26 (requirement_records.rs:452), AC-28 (:507), AC-31 (:82), AC-36 (:864),
AC-37 (:652, :970, routed_generation.rs:2600), AC-38 (:805); AC-27 and AC-32 to AC-35 stay
untagged (slice (b)).

Public names match the spec: `RequirementOccurrence`, `RequirementGuard`,
`RequirementRecordInput`, `RequirementRecords::new`, `RoutedRequirement`,
`RoutedGenerationItem::with_requirement`, `KaniGenerationContext.requirement_records`,
`RequirementRecordMismatch` (seven variants), `RoutedGenerationError::RequirementRecord`,
`UnsupportedObligation::PathGuardNotLowered`, `PathGuardRefusal::GuardLoweringNotImplemented`
(wire `path_guard_not_lowered` / `guard_lowering_not_implemented`, asserted), and
`ObligationItem::ScalarClaim.path_condition`. `GuardContextMissing` is correctly absent: the
FR-022 status text assigns it to slice (b). The golden
`tests/fixtures/kani-scalar/direct-increment-record.json` was committed first (1a70b8aa, tests
only; `src/` identical to base) and its test passes when built at that commit, so it was
captured from the unchanged generator. The fixture migration covers every file the status
text lists; each migrated test only gains the requirement and records, with no assertion
removed or weakened.

## Verdict

**FAIL for the PR subset until the medium findings are fixed in this PR.** The code conforms
to slice (a); the gaps are one missing AC case, one over-claimed tag, and spec status text
that this PR makes false.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-022-AC-37's plain-read case requires both `RecordMissing` (OwnResult) and `OccurrenceNotSelected` (NotApplicable) to precede `UnauthenticatedOperationView`. The tagged test asserts only the OwnResult arm and ends with a bare comment, `// A NotApplicable item carrying a requirement refuses before the view check.`, with no code: a stub. | tests/it/routed_generation.rs:2596-2690 |
| FND-002 | medium | FR-022-AC-31 is tagged on the golden-record test alone. The criterion's main claim (two routed RR-16 identities differing in `requirement_occurrence`, serialized as node/role/ordinal) is slice (b) and unimplemented, yet `quire matrix` now reports AC-31 `tagged`. This overstates coverage; leave AC-31 untagged (or cite it from TC-033 status as golden-only) until slice (b). | tests/it/requirement_records.rs:80-85 |
| FND-003 | medium | Status text is stale after this PR: FR-022 "Scoped evidence status" says AC-26/27/28 and AC-31 to AC-38 are Planned and that routed items have no requirement-record intake; TC-033's status says the same; tests.md row for AC-26..28/31..38 says "await code" and the TC-033 row says "IR-304 cases planned". This PR lands and tags AC-26, AC-28, AC-36, AC-38 and AC-37 except its two-guard case, so the status text must be updated here (it describes this PR's own evidence, so it belongs in this PR). | spec/routed/functional/FR-022-routed-generation.md:832-836; spec/routed/matrix/TC-033-routed-generation.md:331-336; spec/routed/matrix/tests.md:25,64 |
| FND-004 | low | The FR-022-AC-36-tagged test does not assert the criterion's third clause (the same node routed `OwnResult` with its own requirement is accepted and reaches the slice (a) guard handling). The behaviour is exercised by the AC-38 test's `f` item but not bound to AC-36. | tests/it/requirement_records.rs:862-965 |
| FND-005 | low | FR-022 specifies `RequirementRecord { request_index, occurrence, mismatch }` with `mismatch` "one `RequirementRecordMismatch`"; the code boxes it (`Box<RequirementRecordMismatch>`) for clippy's large-variant lint. Harmless, but the spec's public-API text and the type differ; note the box in FR-022 in this PR. | src/routed/generate.rs:650-659; spec/routed/functional/FR-022-routed-generation.md:230-234 |
| FND-006 | low | The slice (a) status text says every item with a non-empty path condition settles `path_guard_not_lowered`, while "Per-item precedence" keeps FR-014 and `operation_not_rendered` first; the code also keeps `unsatisfiable_bound` and `caller_declared_operation` ahead of the guard refusal, which neither list places. The code's choice is reasonable; the spec should state it. | src/kani/generate/negotiate.rs:1343-1380; spec/routed/functional/FR-022-routed-generation.md:319-337,843-846 |

## Coverage

Plan completion: not assessed. PR-diff gap analysis of IR-304 slice (a), not a full repository
sweep. Focused branch tests pass; no `make ci` and no real Kani proofs were run (by
instruction). The one failing lib test (`production_refinement_live_solver_handoff_reaches_qsl_checker`,
`QUIRE_CVC5_BIN` unset) fails identically at the base-equivalent commit.
