---
id: SR-791005
title: "IR-1515 PR #791 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #791 at 995a89f7 (git diff origin/main...HEAD); IR-1515; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-1515. PR #791 (spec only, one commit 995a89f7 on origin/main a52a000e) adds FR-015-AC-133, AC-134 and AC-135, three FR-015 requirement bullets, TC-025 steps 52 and 53 with mutants M1..M8, the AC-48 row tail, the TC-025 step-14 tail and the tests.md AC-48 row. Method: gap-analysis. Reviewed read-only in /home/peter/dev/worktrees/IR-1515-review against the merged CG #788 code (src/kani/identity.rs, src/kani/generate/collection.rs, src/kani/generate/collection/operation.rs, src/kani/generate/negotiate.rs, src/kani/generate/outcome.rs), AD-003 E-1, qsl-replay at 68acd90, and the SR-788001..003 rulings on IR-1507.

## Verdict

**CHANGES REQUIRED (status).** This is a planless, spec-only diff with no production code. `git diff origin/main...HEAD` touches only the three spec markdown files, so there is no code-review lane. Against the code at origin/main: AC-133 is correctly PLANNED, because `operation_selected_reads` drops `element` (collection.rs:660-664). AC-135 is correctly PLANNED, because `classify_v2_clause` has no NodeKey check (negotiate.rs:910-1011). `quire matrix` shows AC-133..135 untagged, as expected. AC-134 is already implemented and tested.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-134 is marked 'PLANNED (IR-1517)', and AC-48's tail lists 'which drawn fields count as reads' as an open gap until IR-1517 lands. The merged code already does this. `lower` builds `fields` from every `model_object_fields` entry, read or not (src/kani/generate/collection/operation.rs:162-183). `operation_selected_reads` maps every one of them into `state_reads` (collection.rs:653-672), and the record doc says 'A whole-object collection clause names every object field it draws' (identity.rs:430). The AC-48-tagged test `whole_object_claim_is_carried_and_every_record_member_is_cross_checked` (tests/it/kani_collection_operation.rs:1146-1203) admits a control record that includes the unread `a_unread`, and refuses `unread.record.state_reads.remove(1)` with `StateReads`. Steps 52(e) and 52(f), and mutant M3, would pass or fail against today's code. Mark AC-134 implemented, or PARTIAL with only the extra-undeclared-read case outstanding. Tag the existing test to AC-134 (or name it as the evidence), drop AC-134 from IR-1517's code scope, and correct the AC-48 tail, the step-14 tail and the tests.md row so they say only AC-133 and AC-135 are open. | spec/kani/functional/FR-015-bounded-kani-obligations.md:946 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-015-AC-134 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:946 | PLANNED (IR-1517; spec IR-1515; refs FR-015-AC-48, FR-015-AC-133 and FR-015-AC-77). A claim whose record lists a set of state-field reads, named by declaring node id and field name, that differs from the set of fields the whole-object collection clause draws is refused with `V2ClauseClaimMismatch` naming `StateReads` before harness generation. Every drawn field counts as a read, whether or not the clause reads it, because its declared domain bounds the drawn state and so changes the proof [...] |
| FR-015-AC-133 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:945 | PLANNED (IR-1517; spec IR-1515; refs FR-015-AC-48, FR-015-AC-77 and FR-015-AC-80). A claim whose record gives a sequence-valued field of a whole-object collection clause a cardinality range or an element domain, at any nesting level, that differs from the independently selected one is refused with `V2ClauseClaimMismatch` naming `StateReads` before harness generation. The field's read domain is its inclusive cardinality range together with its element domain: an inclusive integer range, the Boolean pair, or, for a nested sequence, the same pair applied recursively. [...] |
| FR-015-AC-135 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:947 | PLANNED (IR-1518; spec IR-1515; refs FR-015-AC-48 and AD-003 E-1). A V2 clause claim whose `ObligationIdentity` `NodeKey` differs from the `NodeKey` the request supplies for the selected clause node is refused with `UnsupportedObligation::V2ClauseNodeKeyMismatch`, which names the selected clause node and carries neither key nor CG's selection, before any record comparison and before harness generation. [...] CG derives no `NodeKey` from a `CheckedNodeId` (AD-003 E-1); the request supplies QSL's issued key with each selected clause node. Only the `NodeKey` is compared: a differing model or producer-assigned ordinal alone is admitted and carried unchanged |
| FR-015-AC-48 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:866 | [...] PARTIAL (specified by IR-489; code IR-1507): [...] Two gaps stay open and are specified by FR-015-AC-133 and FR-015-AC-134 (read domain of a sequence-valued state field, and which drawn fields count as reads; code IR-1517) and FR-015-AC-135 (the carried `NodeKey` must be the clause node's issued key; code IR-1518). Until those land, the code compares a sequence field's inclusive cardinality range only and does not check the `NodeKey`. |
| tests.md-FR-015-V2 | examined | spec/kani/matrix/tests.md:46 | AC-48 is implemented by IR-1507 for both routes and stays partial under IR-1515, which specifies AC-133 and AC-134 (code IR-1517) and AC-135 (code IR-1518) as planned |
