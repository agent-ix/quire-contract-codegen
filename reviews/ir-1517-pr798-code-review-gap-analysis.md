---
id: SR-17940486709564847047699
title: "IR-1517 PR #798 gap-analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen PR #798 at 581e0797; IR-1517; FR-015-AC-133, FR-015-AC-134, TC-025 step 52, tests/it/kani_collection_operation.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-1517. This is a planless gap-analysis: Plan completion: not assessed.

`quire matrix` at the head lists two criteria as `tagged`:

- FR-015-AC-133: tests/it/kani_collection_operation.rs:1271, :1327 and :1386.
- FR-015-AC-134: :1151. The retag of `whole_object_claim_is_carried_and_every_record_member_is_cross_checked` is done.

The new tests cover AC-133 cases (a)-(d), (e), (e2) and (f). They also cover a missing element domain on the sequence field and an invented element domain on a scalar field.

Reviewer mutation controls, each applied alone and run against the `kani_collection_operation` module:

- Boolean element computed as 0..=2: killed by `boolean_element_domain_is_the_pair_zero_and_one`.
- Claim comparison ignoring the element minimum (stripped on both sides in `mismatch`): killed by `sequence_field_read_domain_is_cardinality_range_plus_element_domain`, case (f).
- Element domain filled from the cardinality range: survives the Int test and is killed only by the Boolean test.
- Selected element minimum hard-coded to 0 in `element_domain`'s Integer arm: **survives** all 21 tests.

The coder did not run spec mutant M3 (select only the fields the clause reads). That omission is acceptable. Under M3 the selection drops `a_unread`, so both of these refuse:

- the AC-134-tagged test's omitted-`a_unread` case;
- AC-133 control (a), whose hand-written record lists the unread `a_unread`.

The coder adapted the fixture to `z_items`, element `Small` 0..=3, with cardinality 0..=3. The spec's `items` fixture is element Int[0, 9] with cardinality 0..3. The adapted values make the element and cardinality ranges identical, and both start at 0, which weakens discrimination (FND-001, FND-002). The step-52 text mismatch is recorded in the spec-review (SR-17940486709564847047700).

## Verdict

AC-133 is tagged and most of its cases fail under the matching mutant, but one realistic mutant (a selected element minimum fixed at 0) survives the whole module. Not mergeable until FND-001 is fixed in this PR. FND-002 is low and would be fixed by the same fixture change.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No test gives a sequence element a nonzero minimum: `z_items` uses `Small` 0..=3 and the Boolean pair is 0..=1. A mutant hard-coding the selected element minimum to 0 in `element_domain` survives all 21 tests of the module (reviewer run). The AC requires the inclusive element range to be cross-checked, and case (f) only checks the claim side, against a selected minimum that is 0 anyway. Add a fixture or case whose element minimum is nonzero, e.g. Int[1, 9], and assert the selected diagnostic's element minimum. | tests/it/kani_collection_operation.rs:1269-1318; src/kani/generate/collection.rs:687-690 |
| FND-002 | low | The adapted fixture's element domain (0..=3) equals its cardinality range (0..=3). A mutant that writes the cardinality range into `element` passes the Int test, including the assertion on the `selected` diagnostic, and is killed only by the Boolean test. The spec's `items` fixture (cardinality 0..3, element Int[0, 9]) was distinct on purpose. Use distinct ranges. | tests/it/kani_collection_operation.rs:1077-1112 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-015-AC-133 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:948 | A claim whose record gives a one-level sequence-valued field of a whole-object collection clause a cardinality range or an element domain that differs from the independently selected one is refused with V2ClauseClaimMismatch naming StateReads before harness generation. |
| FR-015-AC-134 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:949 | A claim whose record lists a set of state-field reads ... that differs from the set of fields the whole-object collection clause draws is refused with V2ClauseClaimMismatch naming StateReads before harness generation. |
| TC-025 step 52 | examined | spec/kani/matrix/TC-025-bounded-kani-obligations.md:910 | Whole-object collection clause, FR-015-AC-133 ... cases (a)-(f), mutants M1-M3 |
