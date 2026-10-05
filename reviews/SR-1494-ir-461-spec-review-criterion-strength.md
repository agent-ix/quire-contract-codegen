---
id: SR-1494
title: "IR-461 criterion-strength analysis of FR-015-AC-59 to AC-65"
type: SpecReview
analysis: criterion-strength
review_set: subset
scope: "agent-ix/quire-contract-codegen@b094c9cf5685022a41f7723f12d318f4c7643997; spec/kani/functional/FR-015-bounded-kani-obligations.md:376-382, spec/kani/matrix/TC-025-bounded-kani-obligations.md:179-207"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

# SR-1494: IR-461 criterion-strength analysis

## Summary

Ticket: IR-461, PR #275 at b094c9c. I judged by hand whether each of FR-015-AC-59 to AC-65 can fail.
The skill's Jev client does not exist on this machine (the skill says no client exists in quoin),
so no calibrated Jev judgement was made.

AC-59 fails if the first-refusal short-circuit comes back: a batch whose first request is refused
must yield N records, and TC-025 step 19 says so explicitly. AC-60's byte-identity can fail. AC-61
and AC-62 can fail. AC-64 can fail. AC-65 can fail unless the batch is built by calling the
single-clause entry, in which case it is true by construction. That is acceptable, because AC-61 to
AC-63 pin the mapping independently.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-63 and TC-025 step 23 require a test request whose generated source "does not parse as Rust" (`InvalidGeneratedSyntax`). `validate_request` (frame.rs:400-424) has already checked the paths and field identifiers, and module symbols come from hex digests, so the spec does not show how a public request reaches this ground without fault injection. If it cannot, the TC step cannot be executed. Say how it is reached, or exclude it from the test the way `RecordSerialization` is implicitly excluded. | spec/kani/functional/FR-015-bounded-kani-obligations.md:380, spec/kani/matrix/TC-025-bounded-kani-obligations.md:198-202 |

## Verdict

Approve as far as criterion strength goes. No AC passes vacuously as written. The weakness of the
"alone" comparison is recorded in SR-1491 FND-002.

## New findings (disposition pass 1)

Reviewed at 0eaed13b80be44d62145ef0970b949418a3ccacb.

Re-judged at 0eaed13:

- AC-59 still fails if the first-refusal short-circuit comes back (N items give N records; TC step 19).
- AC-66 iterates every variant and every `NotLowered` arm. Building the unreachable variants directly against the mapping function is an acceptable test of a pure mapping, and the compiler enforces the no-wildcard part.
- AC-61 to AC-64 can each fail.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | AC-66 and TC-025 step 26 say no public request reaches `ResourceLimitExceeded`. The ceiling is 1 MiB (`MAX_GENERATED_SOURCE_BYTES`, core/artifact.rs:20) and `state_fields` is an unbounded caller slice, so a large enough field list reaches it. Building it directly is still fine; the statement is wrong. | spec/kani/functional/FR-015-bounded-kani-obligations.md:420, spec/kani/matrix/TC-025-bounded-kani-obligations.md:207-211 |

## New findings (disposition pass 2)

Reviewed at 9ffcba22c884118b6961c3f5964d607dea50baf1.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | AC-67 and TC-025 step 27 require the engine to return `MalformedClause` for "a clause with an operand node absent from the graph". No admitted package can carry such a node. The IR reader refuses an unresolved reference as `invalid_semantic_graph` (quire-contract-model at dec8ade, checked_package/v2/mod.rs:1725 and :2081-2109). Test fixtures build `CheckedPackageV2` only through that reader (tests/exact_scalar_support/package.rs:779-795). And `lower_clause` runs before `ClauseShape::read` (frame.rs:332-333), so a dangling closure would surface as `NotLowered` first. A test cannot build the input, and a correct engine cannot be shown to pass. Drop that case from AC-67 and step 27, or verify it by a unit test of the internal graph `follow`. The non-identifier graph field name case is fine. | spec/kani/functional/FR-015-bounded-kani-obligations.md:464, spec/kani/matrix/TC-025-bounded-kani-obligations.md:217-220 |

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 0eaed13b80be44d62145ef0970b949418a3ccacb: AC-66 now says the variants no public request reaches, including `InvalidGeneratedSyntax`, "are built directly" against the mapping |
| FND-002 | fixed | 9ffcba22c884118b6961c3f5964d607dea50baf1: AC-66 now says `ResourceLimitExceeded` is built directly or reached with a large state-field list |
| FND-003 | fixed | f287f534a1b80b1bd76aaa09dacbeab0c05f3804: the dangling-operand case is removed from AC-67, the Behavior bullet and step 27; the prose records it as unreachable on an admitted package |
