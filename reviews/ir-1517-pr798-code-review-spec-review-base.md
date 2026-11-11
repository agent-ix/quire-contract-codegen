---
id: SR-17940486709564847047700
title: "IR-1517 PR #798 spec-review base (status and evidence edits)"
type: SpecReview
analysis: base
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen PR #798 at 581e0797; IR-1517; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-1517. The PR edits only status and evidence text:

- the FR-015-AC-133 row: PLANNED becomes Implemented, the implementation sentence is restated, and evidence tests are named;
- the FR-015-AC-48 row's PARTIAL note;
- the AC-48 closure paragraph;
- TC-025 step 14 and step 52;
- the tests.md AC-38..49 row.

The AC-133 normative text is unchanged. Each of its claims holds in the code: `V2ClauseRead.element`, the Boolean pair as 0 and 1, the `selected` diagnostic, and nested refusal as `UnsupportedFieldType` on the quantified field and `UnsupportedObjectState` elsewhere.

`make spec` passes with 6 warnings, the same as main; the line-945 AC-128 warning predates this PR. `quire matrix` lists AC-133 and AC-134 as tagged.

AC-48 stays PARTIAL, which is correct. Condition (4) remains open because FR-024-AC-71..75 are PLANNED under IR-1520.

## Verdict

The status edits are accurate, but TC-025 step 52 now describes a fixture, case values and a mutant baseline that differ from what was built and tested. Not mergeable until FND-001 is fixed in this PR. FND-002 and FND-003 are low wording fixes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-025 step 52 contradicts itself. A parenthetical says the tests use `z_items`, element `Small` 0..=3, "in place of the `items`/`a_unread` fixture below". The rest of the step keeps the unbuilt fixture and values: author `items` with element Int[0, 9]; (b) "element maximum 9 changed to 8"; (c) "from Int[0, 9]"; (e2) "quantified `items`"; and "assert before each case that the record lists both reads", which the test asserts once. Mutant M1 is still described as "what the code does today". The step no longer states what is tested. Rewrite it to the executed fixture and values, with a nonzero element minimum per SR-17940486709564847047699 FND-001. Mark M1 as historical, and record that M3 is covered by the AC-134 test and control (a). | spec/kani/matrix/TC-025-bounded-kani-obligations.md:910-929 |
| FND-002 | low | The FR-015-AC-48 row stays PARTIAL, but its note now lists only closed gaps (AC-133, AC-134, AC-135, AC-136). It no longer names what keeps it PARTIAL: closure condition (4), the postcondition and frame replay identity checks of IR-1520. Add one clause naming condition (4) and IR-1520. | spec/kani/functional/FR-015-bounded-kani-obligations.md:869 |
| FND-003 | low | The closure paragraph now says "IR-1520 closes condition (4)". Condition (4) also lists the invariant path, FR-024-AC-65 and AC-66, which are already Delivered by IR-1519. IR-1520 owns only the postcondition path (FR-015-AC-137..142, FR-024-AC-71..72) and the frame path (FR-024-AC-73..75). Say that IR-1519 closed the invariant part. | spec/kani/functional/FR-015-bounded-kani-obligations.md:959 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-015-AC-133 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:948 | Implemented (IR-1517; ...). The field's read domain is its inclusive cardinality range together with its element domain, an inclusive integer range or the Boolean pair. |
| FR-015-AC-48 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:869 | PARTIAL ... The element-domain gap is closed: FR-015-AC-133 is implemented ... |
| FR-015-AC-48 closure | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:959 | IR-1517 and IR-1518 closed conditions (2) and (3), and IR-1520 closes condition (4) |
| FR-015-AC-134 | context_only | spec/kani/functional/FR-015-bounded-kani-obligations.md:949 | Implemented (IR-1507, CG #788) |
| TC-025 step 52 | examined | spec/kani/matrix/TC-025-bounded-kani-obligations.md:910 | Whole-object collection clause, FR-015-AC-133 |
| TC-025 step 14 | examined | spec/kani/matrix/TC-025-bounded-kani-obligations.md:574 | steps 52 (implemented, IR-1517) and 53 (implemented, IR-1518) |
| tests.md AC-38..49 row | examined | spec/kani/matrix/tests.md:46 | AC-133 (IR-1517), AC-134 (IR-1507) and AC-135 and AC-136 (IR-1518) are all implemented |
| FR-024-AC-65 | context_only | spec/replay/functional/FR-024-counterexample-envelope-intake.md:606 | Delivered (IR-1519) |

## Dispositions

Round 1, fix head 488e61d8.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | TC-025 step 52 is rewritten to the executed fixture (element_document, z_items an ordered set 0..=3 of Int[1, 9]) and to cases (a)-(h), (e) and (e2) as the tests run them. No old items/Int[0, 9]/a_unread Int[0, 5] or 'what the code does today' text remains. M3 is attributed to the AC-134 test. The reviewer re-ran M5 and M6 and both fail 52(a) as stated. |
| FND-002 | fixed | The FR-015-AC-48 row now reads 'What keeps FR-015-AC-48 PARTIAL is closure condition (4) below: the replay identity checks owned by IR-1520 (the postcondition and frame paths).' |
| FND-003 | fixed | The closure paragraph now says the invariant path (FR-024-AC-65, AC-66) is delivered by IR-1519 and that IR-1520 closes only the postcondition path (FR-015-AC-137..142 with FR-024-AC-71, AC-72) and the frame path (FR-024-AC-73..75). This matches FR-024's statuses. |
