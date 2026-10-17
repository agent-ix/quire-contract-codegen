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
