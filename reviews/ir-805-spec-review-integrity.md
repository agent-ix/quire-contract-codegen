---
id: "SR-12102"
title: "IR-805 spec-review/integrity review of reference-graph reachability Kani claims"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@edbe6aabdd756a864482c2b73b1c0c113b101894; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

# SR-12102: IR-805 spec-review/integrity review

## Summary

Ticket: IR-805. PR: agent-ix/quire-contract-codegen#417 at edbe6aabdd756a864482c2b73b1c0c113b101894.
Method: spec-review/integrity (completeness, internal consistency, atomicity).

## Scope

- FR-015 Inputs :204-210 (examined), FR-015 description :136-148 (examined), FR-015-AC-98 (examined).
- FR-015-AC-96, FR-015-AC-97, and the collection clauses of FR-015-AC-99 to FR-015-AC-102 (examined): unchanged from origin/main; IR-268 collection half (CG #393/#396) remains consistent.
- tests.md :94, :95 (examined): consistent.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The Inputs bullet says a graph item carries "the finite `model_population` binding and `Type` occurrence of each of its start and target types", i.e. a caller-named FR-053 selector, while :140-144 says IR has no accessor mapping a reaches claim to its population binding and "an item lacking either settles `unsupported`". It is undefined whether a caller-named selector suffices (-> Finite, harness) or the missing mapping always refuses. If the former, a caller can name any `Population<Order>[N]` unrelated to the claim, which is a caller-chosen bound like the forbidden `max_expansions`; AC-98 lists no refusal for a selection not bound to the claim. | spec/kani/functional/FR-015-bounded-kani-obligations.md:204, spec/kani/functional/FR-015-bounded-kani-obligations.md:140, spec/kani/functional/FR-015-bounded-kani-obligations.md:766 |

## Verdict

Revise before accept: settle which input authority binds a claim to its population.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | AC-108 says a population of `Unbounded` "is every claim QSL emits today" so it settles `requires_bound`, and the description says the population "must come from the claim and its admitted package". But the fix dropped the earlier statement that IR has no accessor mapping a reaches claim to its population: QSL requirement records are not on the V2 wire, a domain-package population record has no V2 node (QSL FR-094), and FR-053 builds `Unbounded` only from an upstream `Declaration` selection. With no stated source for that selection, CG gets `Unavailable`, which AC-108 maps to `unsupported`. So the spec gives two dispositions for today's claims; name the source of the claim's population selection (or the IR ticket that adds it), or say today's claims settle `unsupported` (`Unavailable`). | spec/kani/functional/FR-015-bounded-kani-obligations.md:143, spec/kani/functional/FR-015-bounded-kani-obligations.md:781 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | AC-108 says a graph claim "settles by the sequence in the Inputs text", but the three-step disposition sequence is in the Description (FR-015 :147-160), above `## Inputs` (:164); the Inputs section holds no sequence. Point the reference at the Description paragraph. | spec/kani/functional/FR-015-bounded-kani-obligations.md:788, spec/kani/functional/FR-015-bounded-kani-obligations.md:147 |
| FND-004 | low | AC-105 and AC-106 are tagged "PLANNED (IR-805; needs QSL-819 ...)" while the sequence (:147-160) and tests.md :95 say the Finite path runs only after both IR-851 and QSL-819. The two criteria omit IR-851. | spec/kani/functional/FR-015-bounded-kani-obligations.md:785, spec/kani/functional/FR-015-bounded-kani-obligations.md:786 |

## New findings (disposition pass 4)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | Regression from the bb65e1b citation split (reopens FND-004's concern). AC-98, AC-105 and AC-106 now say "needs IR-852 and QSL-819", but step 3 of the disposition sequence (:158-160) says a relationship-end claim stays `unsupported` until IR-851 too. AC-106 requires a relationship-end `Verified` control over `{0,3}` ends and AC-105 draws relationship-end edges, so their tags should also name IR-851 for the relationship-end form, as tests.md :95 does. | spec/kani/functional/FR-015-bounded-kani-obligations.md:779, spec/kani/functional/FR-015-bounded-kani-obligations.md:786, spec/kani/functional/FR-015-bounded-kani-obligations.md:787 |
| FND-006 | low | Step 2 of the sequence (:153-157) says "Once IR-852 supplies the mapping (IR-852 is itself blocked on QSL-819), QSL still gives a state-clause `reaches` no finite population ... every claim settles `requires_bound`". Because IR-852 lands only after QSL-819, step 2 cannot describe the state before QSL-819. After QSL-819 only a claim whose population has no authored bound is `Unbounded`. Restate step 2 as "a claim whose population has no finite bound settles `requires_bound`", and drop "every claim". | spec/kani/functional/FR-015-bounded-kani-obligations.md:153 |

## New findings (disposition pass 5)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | The FND-005 fix put IR-851 in front of QSL-819's gloss. The AC-98, AC-105 and AC-106 tags now read "needs IR-852 and QSL-819; relationship-end form also IR-851 (bounded domain population for reaches)", so the parenthetical describes IR-851, which is the relationship-end multiplicity ticket. Move the gloss back after QSL-819. | spec/kani/functional/FR-015-bounded-kani-obligations.md:781, spec/kani/functional/FR-015-bounded-kani-obligations.md:788, spec/kani/functional/FR-015-bounded-kani-obligations.md:789 |
| FND-008 | low | AC-108 ("`Unbounded` or `UnknownClosureOpenExtent`, which is every claim QSL emits until QSL-819") and TC-025 step (3) ("every QSL claim until QSL-819") contradict the reworded sequence. Before QSL-819 there is no selection, because IR-852 is blocked on QSL-819 and every claim is `Unavailable` (step 1). After QSL-819 and IR-852 land, only a claim with no finite authored bound is `Unbounded` (step 2). Replace the qualifier with "a claim whose population has no finite authored bound". | spec/kani/functional/FR-015-bounded-kani-obligations.md:791, spec/kani/matrix/TC-025-bounded-kani-obligations.md:85 |

## New findings (disposition pass 7)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-009 | low | AC-108 glosses `UnknownNode` as "the claim's node id is absent from the graph" and `NotReachesApplication` as "the path names no `quire.op.model.reaches` application". In IR FR-038 (:3202-3208), `relationship_end` takes the id of the reaches application at the path, not the claim's `state_clause` id. A path that names no reachability is already AC-107's `unsupported` "missing claim" refusal, so the same input has two different refusal names. Gloss the errors as "the application id at the claim's path is absent" or "not a reaches application", and state that AC-107's location refusal comes first. | spec/kani/functional/FR-015-bounded-kani-obligations.md:791 |

## New findings (disposition pass 8)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-010 | low | The FND-009 fix spliced AC-108's last sentence: "... gets AC-107's refusal and never `NotReachesApplication`, a field member of another shape and a claim form the encoding cannot express each settle `unsupported` naming the input." The field-member and claim-form refusals now read as part of the AC-107 ordering clause. End the ordering sentence after `NotReachesApplication` and restore those two items as their own sentence. | spec/kani/functional/FR-015-bounded-kani-obligations.md:791 |

## Dispositions

Reviewed at e1873b156ea042a0b76fedf2a804becf17ac89e0 (round 1; fix commits d088e9c, e1873b1).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | d088e9c..e1873b1: Inputs :212 now says the population is read from the admitted package, never named or sized by the caller, and the description forbids a caller-named binding. |

Round 2 reviewed at bff9ba20ffd6bc59bec6401cd61caaf449665946.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-002 | fixed | bff9ba2: FR-015 :147-160 states the sequence Unavailable->unsupported (IR-851), then Unbounded->requires_bound (QSL-819), then Finite; AC-108, TC-025 steps (2)-(3) and tests.md :95 match it. |

Round 3 reviewed at f7a398ec39cf8fe7186ef5cc9faf56090d5fbd20.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed | f7a398e: AC-108 now cites the Description paragraph "Disposition sequence for a graph claim" (:147), which exists. |
| FND-004 | fixed | f7a398e: AC-98, AC-105 and AC-106 now read "needs IR-851 and QSL-819", matching the sequence and tests.md :95. |

Round 4 note at bb65e1bad4c7a86cb71e4393f5c28c4cab872ea3 (fix d06fc2d..bb65e1b, citation split IR-851 / IR-852). No new outcome rows: FND-001 to FND-004 stay `fixed`. FND-002's fix holds, with step 1 now citing IR-852 for the mapping, and FND-003's reference still resolves to :147. Every citation in the Description, AC-98/105/106/108, TC-025 and tests.md :95 is a ticket (IR-851, IR-852, QSL-819); IR symbols appear only as explanation. New FND-005 and FND-006 are above.

Round 5 reviewed at 9a918744823279b4d9a2fcff0a38bd477d276074 (fix bb65e1b..9a91874).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-005 | fixed | 9a91874: AC-98, AC-105 and AC-106 tags now add "relationship-end form also IR-851", matching sequence step 3 and tests.md :95 (gloss placement is FND-007). |
| FND-006 | fixed | 9a91874: step 2 now says IR-852 lands only after QSL-819 and that, once both land, a claim with no finite authored bound is `Unbounded` and settles `requires_bound`; step 3 says a finite authored bound runs the Verified/Falsified path. |

Round 6 reviewed at 3bf7325c6ac1614314cbc17b0e67afc13fa5cc0b (fix 9a91874..3bf7325).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-007 | fixed | 3bf7325: AC-98, AC-105 and AC-106 tags read "needs IR-852 and QSL-819 (bounded domain population for reaches); relationship-end form also IR-851 (relationship-end multiplicity)". |
| FND-008 | fixed | 3bf7325: AC-108 and TC-025 step (3) now qualify `Unbounded` as "a claim whose population has no finite authored bound (after QSL-819 and IR-852)". |

Round 6 sweep: I grepped every added line of `git diff origin/main...3bf7325 -- spec/` for IR-851, IR-852, QSL-819, Unavailable, Unbounded, "every claim", requires_bound, QSL-804 and "blocked". Every hit agrees with the 3-step sequence. IR-852 is cited for the claim-to-population mapping (today `Unavailable`, settles `unsupported`). QSL-819 is cited for the finite authored bound, and `Unbounded` -> `requires_bound` applies only after QSL-819 and IR-852. IR-851 is cited only for relationship-end multiplicity. "Every claim" appears only for today's `unsupported` state. No new findings.

Round 7 note at 94718fd04fe90be72842cc6e840e9dd94afb2dd1 (fix 3bf7325..94718fd, IR-851 spec citation). The citation was verified against quire-contract-ir d1f2f83f (on origin/main). `CheckedPackageV2::relationship_end(&self, application: &CheckedNodeId) -> Result<CheckedRelationshipEnd, CheckedRelationshipEndError>` exists in FR-038 :3202-3213. Its nine fields (application, relationship, role, destination_type, lower, upper, ordered, unique, forward) and its closed errors (UnknownNode, NotReachesApplication, UnresolvedEnd) match FR-015 :140 exactly, and FR-038-AC-262 to FR-038-AC-268 are the PLANNED/UNRUN IR-851 rows. FND-001 to FND-008 stay `fixed`: the 3-step sequence, IR-852/QSL-819 citations and tests.md :95 are unchanged and consistent. New FND-009 is above.

Round 8 reviewed at 33854d0ef1a8c0733109cba68c0b7c7a84c18fab (fix 94718fd..33854d0).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-009 | fixed | 33854d0: AC-108 glosses `UnknownNode` as the id of the reaches application found at the claim's path (not the `state_clause` id), `NotReachesApplication` as that application id naming a non-reaches node, and states that AC-107's location refusal is checked first. |
