---
id: SR-791002
title: "IR-1515 PR #791 spec-review/ears"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen PR #791 at 995a89f7 (git diff origin/main...HEAD); IR-1515; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-1515. PR #791 (spec only, one commit 995a89f7 on origin/main a52a000e) adds FR-015-AC-133, AC-134 and AC-135, three FR-015 requirement bullets, TC-025 steps 52 and 53 with mutants M1..M8, the AC-48 row tail, the TC-025 step-14 tail and the tests.md AC-48 row. Method: spec-review/ears. Reviewed read-only in /home/peter/dev/worktrees/IR-1515-review against the merged CG #788 code (src/kani/identity.rs, src/kani/generate/collection.rs, src/kani/generate/collection/operation.rs, src/kani/generate/negotiate.rs, src/kani/generate/outcome.rs), AD-003 E-1, qsl-replay at 68acd90, and the SR-788001..003 rulings on IR-1507.

## Verdict

**Minor.** The three new FR-015 requirement bullets are ubiquitous EARS, each with the system subject ('The generator') and exactly one `shall`. AC-133 and AC-134 each state one refusal outcome as a direct assertion. Their trailing rationale sentences ('Every drawn field counts as a read ... because ...', the definition of the read domain) define terms rather than add obligations. `quire validate` reports no new grammar warning on the three files. AC-135 bundles three obligations.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-135 bundles three separately verifiable obligations in one criterion. (1) A differing NodeKey refuses with `V2ClauseNodeKeyMismatch` naming only the clause node. (2) The check runs before any record comparison. (3) A differing model or ordinal alone is admitted and carried. (3) is an admission behaviour with its own case (53(c)) and mutant (M7), and SR-791001 FND-002 disputes it. Folding it into the refusal AC means a test cannot be tagged to one without the other. Split (3) into its own criterion or a stated non-requirement, and keep (1) and (2) together if wanted. | spec/kani/functional/FR-015-bounded-kani-obligations.md:947 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-015-bullets | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:508-513 | The generator shall refuse a V2 clause claim whose carried `NodeKey` is not the key the request supplies for the selected clause node (FR-015-AC-135). / The generator shall compare a sequence-valued field's cardinality range and element domain against the independently selected claim (FR-015-AC-133). / The generator shall compare the full set of fields a whole-object collection clause draws, read or not, against the claim's state-field reads (FR-015-AC-134). |
| FR-015-AC-133 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:945 | PLANNED (IR-1517; spec IR-1515; refs FR-015-AC-48, FR-015-AC-77 and FR-015-AC-80). A claim whose record gives a sequence-valued field of a whole-object collection clause a cardinality range or an element domain, at any nesting level, that differs from the independently selected one is refused with `V2ClauseClaimMismatch` naming `StateReads` before harness generation. The field's read domain is its inclusive cardinality range together with its element domain: an inclusive integer range, the Boolean pair, or, for a nested sequence, the same pair applied recursively. [...] |
| FR-015-AC-134 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:946 | PLANNED (IR-1517; spec IR-1515; refs FR-015-AC-48, FR-015-AC-133 and FR-015-AC-77). A claim whose record lists a set of state-field reads, named by declaring node id and field name, that differs from the set of fields the whole-object collection clause draws is refused with `V2ClauseClaimMismatch` naming `StateReads` before harness generation. Every drawn field counts as a read, whether or not the clause reads it, because its declared domain bounds the drawn state and so changes the proof [...] |
| FR-015-AC-135 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:947 | PLANNED (IR-1518; spec IR-1515; refs FR-015-AC-48 and AD-003 E-1). A V2 clause claim whose `ObligationIdentity` `NodeKey` differs from the `NodeKey` the request supplies for the selected clause node is refused with `UnsupportedObligation::V2ClauseNodeKeyMismatch`, which names the selected clause node and carries neither key nor CG's selection, before any record comparison and before harness generation. [...] CG derives no `NodeKey` from a `CheckedNodeId` (AD-003 E-1); the request supplies QSL's issued key with each selected clause node. Only the `NodeKey` is compared: a differing model or producer-assigned ordinal alone is admitted and carried unchanged |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1, head e6e8dfa7: verified in the diff 995a89f7..e6e8dfa7 |
