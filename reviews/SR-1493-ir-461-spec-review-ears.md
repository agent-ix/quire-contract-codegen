---
id: SR-1493
title: "IR-461 EARS conformance of the FR-015 state and frame disposition statements"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen@b094c9cf5685022a41f7723f12d318f4c7643997; spec/kani/functional/FR-015-bounded-kani-obligations.md:278-312"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

# SR-1493: IR-461 EARS conformance

## Summary

Ticket: IR-461, PR #275 at b094c9c. This analysis checks the seven new Behavior statements for
FR-015-AC-59 to AC-65 against EARS.

`quire validate` raises no EARS warning on them. Each statement has a subject, "the generator".
AC-59 and AC-60 are event-driven (When). AC-61 to AC-64 are unwanted-behaviour (If/then). AC-65 is
ubiquitous.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Two statements are compound. AC-59 says the generator "shall return exactly one record per request ... and shall settle every request". AC-63 says it "shall record it `refused` ... and shall not reject the batch". Each joins two independently failable obligations with two `shall`s, so a partial implementation cannot be marked partial. | spec/kani/functional/FR-015-bounded-kani-obligations.md:287-290, spec/kani/functional/FR-015-bounded-kani-obligations.md:303-307 |
| FND-002 | low | The AC-62 condition "outside the one-comparison shape (not a comparison of pre and post reads, two fields, or two reads of one side)" reads both ways. It can mean "not one of: a pre/post comparison, two fields, two reads", which would make two-field comparisons in-shape. It can also mean "not a pre/post comparison; or two fields; or two reads of one side". The AC-62 table row settles it (two fields is `no_finite_encoding`), but the statement does not. | spec/kani/functional/FR-015-bounded-kani-obligations.md:298-302 |

## Verdict

Approve as far as EARS goes. Both findings are wording fixes. The disposition names these
statements use are challenged in SR-1492.

## New findings (disposition pass 1)

Reviewed at 0eaed13b80be44d62145ef0970b949418a3ccacb.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The PR adds three `quire validate` EARS warnings that main does not have. Lines 334 and 336 are `ears:unclassifiable`/`missing-subject`: in the AC-61 and AC-62/63 bullets, `shall` wraps onto a continuation line away from "the generator". Line 340 is `ears:non-singular`: the AC-64 bullet has "shall record it `invalid_request` and shall return every item's record". Rewrap the two bullets and split the AC-64 one. | spec/kani/functional/FR-015-bounded-kani-obligations.md:333-342 |

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 0eaed13b80be44d62145ef0970b949418a3ccacb: AC-59 is now two statements (one record per item; settle every other item), and AC-63's "shall not reject the batch" is gone |
| FND-002 | fixed | 0eaed13b80be44d62145ef0970b949418a3ccacb: the parenthetical is gone; the table row names each condition shape explicitly |
| FND-003 | fixed | 9ffcba22c884118b6961c3f5964d607dea50baf1: `quire validate` raises no EARS warning on FR-015 at this head; the only warnings are FR-017:152, which predate the PR |
