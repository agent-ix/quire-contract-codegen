---
id: SR-791003
title: "IR-1515 PR #791 spec-review/integrity"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen PR #791 at 995a89f7 (git diff origin/main...HEAD); IR-1515; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-1515. PR #791 (spec only, one commit 995a89f7 on origin/main a52a000e) adds FR-015-AC-133, AC-134 and AC-135, three FR-015 requirement bullets, TC-025 steps 52 and 53 with mutants M1..M8, the AC-48 row tail, the TC-025 step-14 tail and the tests.md AC-48 row. Method: spec-review/integrity. Reviewed read-only in /home/peter/dev/worktrees/IR-1515-review against the merged CG #788 code (src/kani/identity.rs, src/kani/generate/collection.rs, src/kani/generate/collection/operation.rs, src/kani/generate/negotiate.rs, src/kani/generate/outcome.rs), AD-003 E-1, qsl-replay at 68acd90, and the SR-788001..003 rulings on IR-1507.

## Verdict

**Minor.** Numbering is free and consistent. FR-015 ends at AC-132 on main. No open PR adds FR-015-AC-133..135 (checked #790, #762, #583, #553, #535, #523, #501, #474, #413 and #381; #790 adds FR-024 criteria only). The FR bullets, the AC rows, TC-025 steps 52 and 53, the step-14 tail and the tests.md AC-48 row reference each other consistently. `quire validate` is clean on all three files (only the pre-existing AC-128 non-canonical-shape warning, line 942, shifted from 936). `make spec` gives 6 warnings, the same set as main. `quire matrix` lists AC-133, AC-134 and AC-135 as untagged, which is expected for planned criteria. IR-1517 and IR-1518 exist in Backlog. The status of AC-134 is wrong, which is a code-versus-spec finding (SR-791005 FND-001). One tracker-reference defect.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-135 names IR-1518 as its code ticket, but IR-1518's title reads '... (FR-015-AC-134)' and its description says 'Code ticket for the planned criterion FR-015-AC-134'. Only the later comment corrects this to AC-135. AC-134 belongs to IR-1517, so anyone looking up the AC-to-ticket mapping by title or description gets the wrong criterion. Fix the IR-1518 title and description to FR-015-AC-135. This is a tracker edit, not a spec edit. | spec/kani/functional/FR-015-bounded-kani-obligations.md:947 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-015-AC-133 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:945 | PLANNED (IR-1517; spec IR-1515; refs FR-015-AC-48, FR-015-AC-77 and FR-015-AC-80). A claim whose record gives a sequence-valued field of a whole-object collection clause a cardinality range or an element domain, at any nesting level, that differs from the independently selected one is refused with `V2ClauseClaimMismatch` naming `StateReads` before harness generation. The field's read domain is its inclusive cardinality range together with its element domain: an inclusive integer range, the Boolean pair, or, for a nested sequence, the same pair applied recursively. [...] |
| FR-015-AC-134 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:946 | PLANNED (IR-1517; spec IR-1515; refs FR-015-AC-48, FR-015-AC-133 and FR-015-AC-77). A claim whose record lists a set of state-field reads, named by declaring node id and field name, that differs from the set of fields the whole-object collection clause draws is refused with `V2ClauseClaimMismatch` naming `StateReads` before harness generation. Every drawn field counts as a read, whether or not the clause reads it, because its declared domain bounds the drawn state and so changes the proof [...] |
| FR-015-AC-135 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:947 | PLANNED (IR-1518; spec IR-1515; refs FR-015-AC-48 and AD-003 E-1). A V2 clause claim whose `ObligationIdentity` `NodeKey` differs from the `NodeKey` the request supplies for the selected clause node is refused with `UnsupportedObligation::V2ClauseNodeKeyMismatch`, which names the selected clause node and carries neither key nor CG's selection, before any record comparison and before harness generation. [...] CG derives no `NodeKey` from a `CheckedNodeId` (AD-003 E-1); the request supplies QSL's issued key with each selected clause node. Only the `NodeKey` is compared: a differing model or producer-assigned ordinal alone is admitted and carried unchanged |
| FR-015-AC-48 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:866 | [...] PARTIAL (specified by IR-489; code IR-1507): [...] Two gaps stay open and are specified by FR-015-AC-133 and FR-015-AC-134 (read domain of a sequence-valued state field, and which drawn fields count as reads; code IR-1517) and FR-015-AC-135 (the carried `NodeKey` must be the clause node's issued key; code IR-1518). Until those land, the code compares a sequence field's inclusive cardinality range only and does not check the `NodeKey`. |
| TC-025-step-14-tail | examined | spec/kani/matrix/TC-025-bounded-kani-obligations.md:576-577 | The sequence-field read domain, the unread drawn fields and the NodeKey binding are specified by steps 52 and 53 below (planned). |
| tests.md-FR-015-V2 | examined | spec/kani/matrix/tests.md:46 | AC-48 is implemented by IR-1507 for both routes and stays partial under IR-1515, which specifies AC-133 and AC-134 (code IR-1517) and AC-135 (code IR-1518) as planned |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | IR-1517 is the code ticket for AC-133. Its title still reads '... sequence element domain in whole-object collection reads (FR-015-AC-133)'; that part is fine. Its description still says 'carry the element domain (recursively for nested sequences), keep every drawn field, read or not, as a read'. AC-133 now covers one-level sequences only, says nested sequences stay refused, and treats AC-134 as implemented. Comments 2d784625 and 7feea176 correct the scope, but a coder reading the description builds the recursive variant. Fix: edit the IR-1517 description. This is a tracker edit. | spec/kani/functional/FR-015-bounded-kani-obligations.md:945 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | Round 1, head e6e8dfa7: IR-1518's title now reads (FR-015-AC-135, AC-136), but its description still says 'Code ticket for the planned criterion FR-015-AC-134'. It also still says 'The request carries the QSL-issued NodeKey ... CG derives no key from a CheckedNodeId', which contradicts the rewritten AC-135. Only comment 89393a00 corrects it. Fix: edit the IR-1518 description. This is a tracker edit; no spec change is needed. |
| FND-001 | fixed | Round 2, head 60c745ef: verified in the diff e6e8dfa7..60c745ef (tracker edits verified via linear issue view) |
| FND-002 | fixed | Round 2, head 60c745ef: verified in the diff e6e8dfa7..60c745ef (tracker edits verified via linear issue view) |
