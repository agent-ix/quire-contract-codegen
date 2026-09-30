---
id: "SR-620"
title: "IR-433 spec review: FR-030 and TC-041, the Contract IR outcome to QSL terminal value map"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@a57aa9108f0d49227ec8d818605acaf7bb895b53; spec/functional/complete-v1/FR-030-ir-outcome-terminal-map.md, spec/test/complete-v1/TC-041-ir-outcome-terminal-map.md, spec/test-matrix.md, spec/index.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-030
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-041
    type: reviews
---

# SR-620: IR-433 spec review

## Summary

Ticket: IR-433 (the map itself is tracked under IR-358). PR:
agent-ix/quire-contract-codegen#196, head a57aa91. Method: spec-review. The integrity, EARS and
cross-repo consistency checks are folded into this file.

## Method

I read FR-030, TC-041, the matrix and index edits, and CG's FR-029. I checked them against the
types they name, at the locked commits:

- `quire_contract_ir::kani::KaniOutcome` and `KaniOutcomeKind`: IR 54f9a48,
  `src/kani/outcome.rs`.
- `qsl_replay::TerminalValue`, `ProofRefusalCause`, `UnavailabilityCause`, `IncompleteCause` and
  `TerminalRecord`: QSL 966e7d2, `qsl-replay/src/proof_result.rs`.

I also read IR's FR-031 on IR `origin/main`, including its retirement of FR-031-AC-5.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The `Proved` row needs `success_checks: n` with `n` at least one, but the declared input carries no check count. A `KaniOutcome` has only `kind`, `code`, `source_id` and `context`, `proved_from_checks` discards the count, and FR-030 forbids reading `source_id`/`context`. So the row and FR-030-AC-4 cannot be implemented as written. | spec/functional/complete-v1/FR-030-ir-outcome-terminal-map.md:33-34, :48, :71 |
| FND-002 | medium | The `Unavailable` row maps to `Unsupported(UnavailabilityCause)` without naming a variant, and QSL has two: `SolverAbsent` and `BackendAbsent`. So the map is not a function as specified, and FR-030-AC-4 checks only the outer `Unsupported`. IR FR-031 names the causes `kani_solver_absent` / `kani_backend_absent`. | spec/functional/complete-v1/FR-030-ir-outcome-terminal-map.md:53, :71 |
| FND-003 | medium | FR-030 says it maps outcomes "the Kani boundary produces at negotiation and input validation". CG FR-029 says an item settled at negotiation "has no run, no artifact and no terminal value", and IR FR-031 says no `KaniOutcome` exists for such a construct. FR-030 also does not say when it applies rather than FR-029's run-outcome map, so one run could get two terminal values. | spec/functional/complete-v1/FR-030-ir-outcome-terminal-map.md:27-29 |
| FND-004 | medium | The TC-041 test-case row was inserted into the four-column Functional Requirement Coverage table as a six-column row, and it is missing from the Test Case Summary table where TC-040 sits. `quire validate` does not flag this. | spec/test-matrix.md:75, spec/test-matrix.md:172 |
| FND-005 | low | FR-030-AC-1 is compound, and its verification method is wrong for half of it. Totality is a compile-time property, and the "no wildcard arm" half is checked by TC-041 step 6, "Inspect the map's source", which is an Inspection, not a Test. | spec/functional/complete-v1/FR-030-ir-outcome-terminal-map.md:68, spec/test/complete-v1/TC-041-ir-outcome-terminal-map.md:24 |
| FND-006 | low | The Behavior bullet forbids reading "the outcome's message", but `KaniOutcome` has no message field. | spec/functional/complete-v1/FR-030-ir-outcome-terminal-map.md:62 |
| FND-007 | low | The title and Description say the map "preserves the outcome's refusal cause". Only the kind-level cause survives: `TerminalRecord` holds `item` and `value` only, so the stable `code` (for example `kani_dispatch_unowned` vs `kani_capability_missing`, both `Refused`) is dropped. IR's retired `KaniProviderRecord` carried it as `cause`. | spec/functional/complete-v1/FR-030-ir-outcome-terminal-map.md:21-23 |

### FND-001 detail

`KaniOutcome::proved_from_checks(success_checks, …)` returns `Self::proved(source_id, context)`
for any non-zero count, so the count is lost. IR's FR-031 Behavior says it exposes "`Proved` with
its SUCCESS check count", but IR's code at 54f9a48 does not carry the count. That is a cross-repo
gap for the IR lane.

Fix: either add the SUCCESS-check count as an explicit second input to FR-030, from the run's
transcript as FR-029 does, and make AC-4 name a concrete `n`. Or make FR-030 depend on an IR
change that adds the count to `KaniOutcome`, and file that change on IR.

### FND-002 detail

Fix: key the row on the cause `code`. `kani_solver_absent` maps to `Unsupported(SolverAbsent)` and
`kani_backend_absent` maps to `Unsupported(BackendAbsent)`. State the result for any other
`Unavailable` code so the map stays total, and add an AC for each. Note that IR at 54f9a48 emits
neither code, so the IR lane needs to know about this too.

### FND-003 detail

Fix: define the input domain as the outcomes produced for a supported item after negotiation:
run-time refusals, input validation and limits. Say that items settled at negotiation reach
neither map. Say that FR-029 and FR-030 take disjoint inputs, or which of the two a run's
terminal record comes from.

### FND-004 detail

Fix: move line 75 to follow the TC-040 row, at line 172 in the Test Case Summary table.

## Verdict

Request changes on FND-001: as written, the `Proved` row cannot be computed from the requirement's
own input. FND-002 to FND-004 belong in the same round.

What is right:

- The kind column is exhaustive. It lists all 10 `KaniOutcomeKind` variants at IR 54f9a48
  (`Proved`, `Counterexample`, `Refused`, `InvalidInput`, `IncompleteInput`, `Unavailable`,
  `TimedOut`, `ResourceExhausted`, `Cancelled` and `Inconclusive`) and splits `Inconclusive` by
  cause.
- The rows agree with QSL: the three refusal kinds go to `Declined` with their own
  `ProofRefusalCause`, the three limit kinds go to `Incomplete` with their own `IncompleteCause`,
  `Counterexample` goes to `Refuted`, and no outcome maps to `Tested`. Vacuous proof to
  `Proved { success_checks: 0 }` matches both FR-029 and QSL's `category()`.
- This agrees with IR's retirement. IR FR-031 retires AC-5 without reusing the id, says the map
  is owned by CG under IR-358, and states no row of its own. FR-030 references
  `ix://agent-ix/quire-contract-ir/FR-031` and uses QSL's type instead of defining its own.
- No id collides: FR-030 and TC-041 are new, and the index ranges (FR-028 to FR-030, TC-039 to
  TC-041) are correct. The EARS statements have a subject and "shall". Status is honestly
  Planned.
- `quire validate` with the Makefile globs exits 0.
