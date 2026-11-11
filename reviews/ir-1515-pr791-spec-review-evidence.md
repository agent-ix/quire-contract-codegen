---
id: SR-791004
title: "IR-1515 PR #791 spec-review/evidence"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen PR #791 at 995a89f7 (git diff origin/main...HEAD); IR-1515; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-1515. PR #791 (spec only, one commit 995a89f7 on origin/main a52a000e) adds FR-015-AC-133, AC-134 and AC-135, three FR-015 requirement bullets, TC-025 steps 52 and 53 with mutants M1..M8, the AC-48 row tail, the TC-025 step-14 tail and the tests.md AC-48 row. Method: spec-review/evidence. Reviewed read-only in /home/peter/dev/worktrees/IR-1515-review against the merged CG #788 code (src/kani/identity.rs, src/kani/generate/collection.rs, src/kani/generate/collection/operation.rs, src/kani/generate/negotiate.rs, src/kani/generate/outcome.rs), AD-003 E-1, qsl-replay at 68acd90, and the SR-788001..003 rulings on IR-1507.

## Verdict

**CHANGES REQUIRED.** `Test (TC-025)` is the right method for all three criteria, and the hand-written-record discipline (records never copied from a refusal, and a check that each record lists the intended fields) answers SR-788003 FND-003 / SR-788001 FND-003. Mutant check: M1 fails 52(b), and M3 fails 52(a) because the full control record differs from the read-only selection. M5 fails 53(b) and 53(d), M6 fails 53(a) and 53(b) because B is listed first, M7 fails 53(c), and M8 fails 53(d). Each names its site and failing step. But step 52's fixture cannot be admitted by the code at the site it names, so step 52 cannot run as written, and one mutant claim is not pinned.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Step 52's fixture cannot reach the record comparison. On the whole-object collection route, `field_domain` builds a sequence field's element with `scalar(element)?` (src/kani/generate/collection/operation.rs:95). `scalar` admits only Boolean and IntRange and refuses anything else with `UnsupportedObjectState` (:59-74). The field list is collected over every object field, read or not (:162-183). So the `grid` field (a sequence of sequences) refuses the whole clause as `CollectionClaimNotRendered`/`UnsupportedObjectState` while lowering. `classify_v2_clause` never computes a selection (negotiate.rs:953-968). As a result, control 52(a) ('admitted and a harness is generated') cannot pass, every case that depends on it is unrunnable, 52(c) and M2 have nothing to test, and AC-133's 'at any nesting level ... applied recursively' specifies a domain that this route refuses earlier. Either drop nesting from AC-133 and step 52, saying that a nested sequence is refused today as `UnsupportedObjectState`. Or specify nested-sequence admission as a prerequisite with its own ticket, and keep `grid` out of the control fixture until that lands. | spec/kani/matrix/TC-025-bounded-kani-obligations.md:910-929 |
| FND-002 | low | Step 52 says M4 ('adding a field the object does not declare to the selection') fails 52(a) and 52(f). 52(f) is the record with an extra `ghost` read and expects a refusal. Under M4 the selection is the three declared fields plus some field X. 52(f) is admitted, and so fails, only if X is `ghost` with the same declaring node and domain as the record. For any other X the sets still differ, 52(f) still refuses, and it stays green. Pin M4 to add `ghost` with the record's node and domain, or drop the 52(f) claim and rely on 52(a). | spec/kani/matrix/TC-025-bounded-kani-obligations.md:927-929 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| TC-025-step-52 | examined | spec/kani/matrix/TC-025-bounded-kani-obligations.md:910-929 | 52. Whole-object collection clause, FR-015-AC-133 and FR-015-AC-134. Author a fixture whose object declares three fields: `items`, a sequence of 0 to 3 elements each `Int[0, 9]`, read by the clause; `grid`, a sequence of 0 to 3 elements each a sequence of 0 to 2 elements each `Int[0, 9]`, not read by the clause; and `count`, `Int[0, 5]`, not read. [...] (a) control, the record with all three reads at their declared domains is admitted and a harness is generated; [...] (c) `grid` inner element maximum 9 changed to 8 [...] (f) the record with an extra `ghost` read the object does not declare: same refusal. [...] M2, comparing only the outermost element [...] fails 52(c) [...] M4, adding a field the object does not declare to the selection, fails 52(a) and 52(f) |
| TC-025-step-53 | examined | spec/kani/matrix/TC-025-bounded-kani-obligations.md:930-945 | 53. NodeKey binding, FR-015-AC-135. Author a package with two clauses A and B on different operations, request both with B listed first, each supplied with its own issued `NodeKey` (the keys written by hand in the test, not derived from the node ids by production code) [...] (c) A's record with `NodeKey_A`, ordinal 1 and a different owning model: admitted, identity carried unchanged; (d) [...] M5 [...] M6 [...] M7, comparing the whole `ObligationIdentity` instead of the `NodeKey`, fails 53(c); M8, running the record comparison first, fails 53(d) |
| FR-015-AC-133 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:945 | PLANNED (IR-1517; spec IR-1515; refs FR-015-AC-48, FR-015-AC-77 and FR-015-AC-80). A claim whose record gives a sequence-valued field of a whole-object collection clause a cardinality range or an element domain, at any nesting level, that differs from the independently selected one is refused with `V2ClauseClaimMismatch` naming `StateReads` before harness generation. The field's read domain is its inclusive cardinality range together with its element domain: an inclusive integer range, the Boolean pair, or, for a nested sequence, the same pair applied recursively. [...] |
| FR-015-AC-134 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:946 | PLANNED (IR-1517; spec IR-1515; refs FR-015-AC-48, FR-015-AC-133 and FR-015-AC-77). A claim whose record lists a set of state-field reads, named by declaring node id and field name, that differs from the set of fields the whole-object collection clause draws is refused with `V2ClauseClaimMismatch` naming `StateReads` before harness generation. Every drawn field counts as a read, whether or not the clause reads it, because its declared domain bounds the drawn state and so changes the proof [...] |
| FR-015-AC-135 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:947 | PLANNED (IR-1518; spec IR-1515; refs FR-015-AC-48 and AD-003 E-1). A V2 clause claim whose `ObligationIdentity` `NodeKey` differs from the `NodeKey` the request supplies for the selected clause node is refused with `UnsupportedObligation::V2ClauseNodeKeyMismatch`, which names the selected clause node and carries neither key nor CG's selection, before any record comparison and before harness generation. [...] CG derives no `NodeKey` from a `CheckedNodeId` (AD-003 E-1); the request supplies QSL's issued key with each selected clause node. Only the `NodeKey` is compared: a differing model or producer-assigned ordinal alone is admitted and carried unchanged |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | Step 52(e) gives the quantified field `items` an element that is itself a sequence, and expects `CollectionClaimNotRendered` with `UnsupportedObjectState`. `lower_collection_operation` runs `resolve_collection_field` on the quantified field before `operation::lower` (src/kani/generate/collection.rs:295-296). That calls `finite_element_domain` (:937), which refuses a non-scalar element with `CollectionDomainRefusal::UnsupportedFieldType` (:761), mapped to `CollectionUnsupportedReason::UnsupportedFieldType` (negotiate.rs:1117-1118). So 52(e) as written fails against today's code. `UnsupportedObjectState` comes only from `field_domain` (operation.rs:74, :95), which is reached only for a nested sequence on another drawn field. AC-133's sentence 'A sequence whose element is itself a sequence is refused today as UnsupportedObjectState while the clause is lowered' has the same gap. Fix: in 52(e), keep `items` one-level and add an unread drawn field whose element is a sequence, which refuses `UnsupportedObjectState`. Or expect `UnsupportedFieldType` for the quantified case. Then word AC-133 to match (quantified: UnsupportedFieldType; other drawn field: UnsupportedObjectState). | spec/kani/matrix/TC-025-bounded-kani-obligations.md:910-931 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1, head e6e8dfa7: verified in the diff 995a89f7..e6e8dfa7 |
| FND-002 | fixed | Round 1, head e6e8dfa7: verified in the diff 995a89f7..e6e8dfa7 |
| FND-003 | fixed | Round 2, head 60c745ef: verified in the diff e6e8dfa7..60c745ef (tracker edits verified via linear issue view) |
