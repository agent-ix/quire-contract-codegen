---
id: SR-1495
title: "IR-461 scope-boundary analysis: a second state-frame public entry against AD-004"
type: SpecReview
analysis: scope-boundary
review_set: subset
scope: "agent-ix/quire-contract-codegen@b094c9cf5685022a41f7723f12d318f4c7643997; spec/kani/functional/FR-015-bounded-kani-obligations.md:278-312, spec/core/functional/interface-001-codegen-api.md:109-112, spec/assurance/AD-004-cg-crate-layout.md:1061"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: reviews
---

# SR-1495: IR-461 scope-boundary analysis

## Summary

Ticket: IR-461, PR #275 at b094c9c. This analysis asks whether the new entry stays inside FR-015's
boundary and AD-004's allocation, and whether it adds behaviour the stated need (QSL-20) does not
require.

The ticket text (untrusted, Linear IR-461) asks for per-construct dispositions "as the scalar lane
does". The scalar lane does this through `negotiate_kani_obligations` with `ObligationItem` and
`ObligationDisposition`. On origin/main, AD-004 (lines 397-400 and step 4d, lines 742-743) makes
`negotiate_kani_obligations` "the one public generation entry" and turns
`generate_state_frame_obligations` into a frame arm of `ObligationItem`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The PR adds a third public generation entry, `generate_state_frame_dispositions`. AD-004's target is one entry, with the state-frame lane as an `ObligationItem` frame arm at step 4d. The PR records this only as a one-line note in AD-004's gap table ("lives in the same file"), which leaves the conflict open and gives step 4d another public entry to delete. Per-clause dispositions across a package are what negotiate's frame arm would produce. Fix: specify the need as the planned `ObligationItem` frame arm, which gives request-ordered `ObligationRecord`s per clause. Otherwise amend AD-004's target explicitly, with rationale; that is an architecture decision for the owner, not a gap-table note. | spec/core/functional/interface-001-codegen-api.md:109-112, spec/assurance/AD-004-cg-crate-layout.md:1061, spec/kani/functional/FR-015-bounded-kani-obligations.md:278-285 |
| FND-002 | medium | Author decision (1), "a malformed request is a refused record, unlike negotiate", is justified as "QSL-20 needs the whole package". negotiate's `Rejected` outcome already returns every item's record in request order (outcome.rs:377-382) and withholds only harness bytes. So the disposition need does not require departing from negotiate. The divergence adds a second request-validity rule for the same crate. State why QSL-20 needs harnesses for siblings of a malformed request, or keep negotiate's rule. | spec/kani/functional/FR-015-bounded-kani-obligations.md:303-307, spec/core/functional/interface-001-codegen-api.md:112 |

## Verdict

Request changes on FND-001. Which way to fix it, a frame arm of negotiate or an AD-004 amendment,
needs the owner if the author keeps a separate entry. QSL-20's private text matters to one question
only: whether QSL-20 consumes a new four-way enum, or whether `ObligationDisposition` (with the
`NoFiniteEncoding` reason) is enough. If it is enough, the frame-arm route needs nothing from QSL.

## New findings (disposition pass 1)

Reviewed at 0eaed13b80be44d62145ef0970b949418a3ccacb.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | Planned FR-015-AC-38 (IR-489) already gives a V2 clause claim over the same postcondition `state_clause` node a `postcondition` harness through negotiate. The `StateFrame` `contract` role is a second planned arm for that node, with the same `kind` and a different harness. The spec does not say how the two relate: whether the StateFrame contract role is retired when AC-38 lands, and whether `DuplicateItem` applies across the two arms. | spec/kani/functional/FR-015-bounded-kani-obligations.md:284-289 |

## New findings (disposition pass 3)

Reviewed at f287f534a1b80b1bd76aaa09dacbeab0c05f3804.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | interface-001 lists `generate_state_frame_role` as a codegen API operation, a new public entry beside `negotiate_kani_obligations`. Unlike `generate_state_frame_obligations`, it carries no note that AD-004 step 4d retires it, and nothing says it may be crate-internal (`pub(crate)`, tested in-crate). AD-004's target is one public generation entry. Add the step-4d retirement note, or state that the function is not public. | spec/core/functional/interface-001-codegen-api.md:109-112, spec/kani/functional/FR-015-bounded-kani-obligations.md:291-298 |

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 0eaed13b80be44d62145ef0970b949418a3ccacb: "It adds one `ObligationItem` arm, `StateFrame`, which is the frame arm AD-004 step 4d plans, and no public entry"; the AD-004 gap-table edit is reverted |
| FND-002 | fixed | 0eaed13b80be44d62145ef0970b949418a3ccacb: an invalid item now follows negotiate: "record it `invalid_request` and … return every item's record with no harness bytes" |
| FND-003 | fixed | 9ffcba22c884118b6961c3f5964d607dea50baf1: the arms coexist; `DuplicateItem` spans one arm and role; whether the contract role retires is left to IR-489 |
