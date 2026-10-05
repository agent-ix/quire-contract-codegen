---
id: SR-1491
title: "IR-461 spec review (base checklist): FR-015-AC-59 to AC-65 state and frame dispositions"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@b094c9cf5685022a41f7723f12d318f4c7643997; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md, spec/core/functional/interface-001-codegen-api.md, spec/assurance/AD-004-cg-crate-layout.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

# SR-1491: IR-461 spec review (base checklist)

## Summary

Ticket: IR-461. PR: agent-ix/quire-contract-codegen#275, head b094c9c, diffed against
origin/main 7345463. This is the base checklist for the spec-review set. The sub-analyses each have
their own document: integrity SR-1492, EARS SR-1493, criterion-strength SR-1494 and
scope-boundary SR-1495.

What I measured on origin/main:

- `generate_state_frame_obligations` is in `src/kani/generate/frame.rs:327`. Its signature is
  `(request: &StateFrameRequest<'_>) -> Result<StateFrameObligations, StateFrameRefusal>`, and it
  returns at the first `?`. Its doc says "nothing is partially returned".
- `StateFrameRefusal` (`frame.rs:146`) has 17 variants.
- `MAX_OBLIGATION_ITEMS = 256` is at `src/kani/generate/outcome.rs:20`.
- FR-015-AC-23 is about `ObligationDisposition` items only. FR-015-AC-26 to AC-36 are about the
  single clause.

The PR's statements about the current code are therefore correct. The ticket text quotes
`src/state_frame.rs:~425` at 8fcb51f, which is stale, and the PR correctly cites `frame.rs`.

Other checks:

- `quire validate --scope <head> "spec/**/*.md"` exits 0. It adds no warning over main.
- `quire coverage --strict`, main against head: 301/422 becomes 301/429 rows backed. That is
  exactly the seven new unbacked planned rows, FR-015-AC-59 to AC-65. Every other difference is a
  line shift. Both runs exit 1, for reasons that predate this PR.
- The tests.md row, the TC-025 coverage list and TC-025 steps 19 to 25 agree with the seven ACs.
- The PR mentions no compatibility layer. It discloses no research or private-repo internals.
  QSL-20 appears only as a ticket id.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-015-AC-62 and TC-025 step 22 list four condition shapes that are outside the supported form: negation, literal comparison, two fields, and two reads of one side. They omit "a read through another parameter". FR-015-AC-29 names that shape for the single-clause entry (`ConditionNotSupported`, `frame.rs:662-716`), and it appears in no disposition AC of the batch. | spec/kani/functional/FR-015-bounded-kani-obligations.md:379, spec/kani/matrix/TC-025-bounded-kani-obligations.md:195 |
| FND-002 | low | FR-015-AC-59 ("the disposition it has alone") and FR-015-AC-63 ("recorded as they are alone") never define "alone". It could mean a batch of one, or the single-clause entry's result mapped through the disposition table. AC-59 also compares only the disposition, not the reason or the harness. | spec/kani/functional/FR-015-bounded-kani-obligations.md:376, spec/kani/functional/FR-015-bounded-kani-obligations.md:380 |

## Verdict

Request changes. The base checklist turns up only the two low findings above. The blocking
findings are in integrity (SR-1492, FND-001 to FND-003) and scope-boundary (SR-1495, FND-001).
Together they say that the new disposition vocabulary and the new public entry conflict with
`ObligationDisposition`, with planned FR-015-AC-40 and with AD-004's one-entry target.

## New findings (disposition pass 1)

Reviewed at 0eaed13b80be44d62145ef0970b949418a3ccacb.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | A `StateFrame` item carries its own subject path, and the request also has a `subject_path`. The spec does not say which one an item harness calls. An unparsable *item* subject path is `InvalidPath`, which the table maps to `InvalidStatePath`, a reason named for the state path. AC-64 and TC-025 step 24 test only an unparsable state path. | spec/kani/functional/FR-015-bounded-kani-obligations.md:284-288, spec/kani/functional/FR-015-bounded-kani-obligations.md:312 |
| FND-004 | low | "Both roles of a clause get the same record reason" means a refusal that only concerns the frame also marks the `contract` item as not generated, even though its operation-contract harness is renderable. The frame-only refusals are `NothingForbidden`, `FrameEffectUnsupported`, `UnknownStateField` on a granted field, and a frame-render ceiling. State that this is accepted, or settle the contract role independently. | spec/kani/functional/FR-015-bounded-kani-obligations.md:294-297 |
| FND-005 | low | The prose changes the single-clause engine: a non-identifier field name read from the graph, and an operand that cannot be followed, both become `MalformedClause`. Today they are `InvalidField` (frame.rs:610, :709) and `ConditionNotSupported` (frame.rs:663-665, :713). No AC or TC step asserts what the engine returns for these, and FR-015-AC-29 is unchanged. | spec/kani/functional/FR-015-bounded-kani-obligations.md:319-323 |

## New findings (disposition pass 2)

Reviewed at 9ffcba22c884118b6961c3f5964d607dea50baf1.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | The `BoundNotResolved` split has no row for a clause field whose member exists but whose `value` is not a reference, so there is no `value.target`. The rows cover: target is an unbounded type; target is a non-`integer_range` or non-`i64` bound; member absent. `state_domains` reads `member.value.target` (frame.rs:790), so that shape reaches `BoundNotResolved` today. AC-63 and TC-025 step 23 also omit the member-absent case that the table puts under `StateFrameRefused`. | spec/kani/functional/FR-015-bounded-kani-obligations.md:321-322, spec/kani/functional/FR-015-bounded-kani-obligations.md:460 |

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 0eaed13b80be44d62145ef0970b949418a3ccacb: AC-63 now lists "reads through a parameter other than `self`" among the `StateFrameRefused` shapes |
| FND-002 | fixed | 0eaed13b80be44d62145ef0970b949418a3ccacb: AC-59 now says "each later item's record (disposition and reason) equals the record the same item has in a request holding that item only" |
| FND-003 | fixed | 9ffcba22c884118b6961c3f5964d607dea50baf1: the item's subject path is the one its harness calls; the request's is validated first and not read; an unparsable item subject path is `InvalidStatePath` (AC-64, step 24) |
| FND-004 | fixed | 9ffcba22c884118b6961c3f5964d607dea50baf1: the roles are independent (AC-68, step 28); see SR-1492 FND-008 for the oracle this opens |
| FND-005 | fixed | 9ffcba22c884118b6961c3f5964d607dea50baf1: AC-67 and step 27 assert the engine's `MalformedClause`; see SR-1494 FND-003 for the absent-operand case |
| FND-006 | fixed | f287f534a1b80b1bd76aaa09dacbeab0c05f3804: the `BoundNotResolved` row now covers a member whose value is not a reference; AC-63 and step 23 name member-absent and value-not-a-reference |
