---
id: SR-3061
title: "IR-687 spec review (EARS conformance): FR-034 AC-57..AC-65 and their Behavior prose"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen branch spec/ir687-prearm-negative-custody (frozen head named in the Linear marker); spec/kani/functional/FR-034-caller-death-ownership.md lines 272-362 (two new Behavior sections) and the nine new AC rows FR-034-AC-57..AC-65; compared with the surrounding FR-034 prose and AC conventions"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
---

## Summary

Ticket: IR-687. This pass checks the new normative sentences for EARS form: a named actor, "shall", one obligation per statement, and a trigger or state clause where one applies. It also checks each new AC for atomicity.

Most prose sentences name an actor (O, L or C) and use "shall", with When/While triggers. The FR-034 AC table convention is declarative, PLANNED/UNRUN-prefixed rows rather than shall statements, and the new rows follow it. The defects are compound criteria and permissive modals.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-034-AC-65 bundles at least six independently failing obligations in one criterion: first-established candidate selection, one irreversible transaction, inclusion of every due pre-send observation, owner retention of later facts, provisional publication until I termination plus separate M reap plus whole-chain settlement, and the absent-peak semantics. A partial implementation cannot be reported per obligation. One Test/Analysis tag on AC-65 would claim all six. | spec/kani/functional/FR-034-caller-death-ownership.md:1617 |
| FND-002 | low | Several other new criteria each bundle two or three obligations. AC-57 has an O→L zero-right rule, an L→C one-clone rule and a no-other-context rule. AC-58 has L authentication and C authentication. AC-60 has a positive permission list and a negative exclusion list. AC-63 has an ordering rule and an unconfirmed-precedence rule. | spec/kani/functional/FR-034-caller-death-ownership.md:1609; spec/kani/functional/FR-034-caller-death-ownership.md:1610; spec/kani/functional/FR-034-caller-death-ownership.md:1612; spec/kani/functional/FR-034-caller-death-ownership.md:1615 |
| FND-003 | low | The normative prose uses the permissive "may" twice ("O may publish the existing Committed / OperationalFailure", "an unsent frame may be retired"). It also uses non-shall declaratives in requirement position, for example "This ordering allocates the genuine-failure/later-stop pair; it does not weaken ...". The retirement "may" is the sentence behind the SR-3062 FND-001 contradiction. | spec/kani/functional/FR-034-caller-death-ownership.md:329-330; spec/kani/functional/FR-034-caller-death-ownership.md:332; spec/kani/functional/FR-034-caller-death-ownership.md:338 |

## Verdict

**Changes requested (medium).** Split AC-65 into atomic criteria. The reservation comment names it as the single later-transaction rule, but each of its six obligations needs its own falsifiable row. Replace the two "may" sentences with explicit shall or shall-not obligations.

Examined with no EARS finding: the triggers and actors at FR-034:274-279, 293-301 and 303-312, and the AC-59, AC-61, AC-62 and AC-64 rows.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | Two split criteria still bundle two obligations each. AC-66 bundles the same-bytes/one-clone route with the Test-oracle requirement. AC-72 bundles private retention until settlement with the no-normal-exit-while-pending rule. | spec/kani/functional/FR-034-caller-death-ownership.md:1744; spec/kani/functional/FR-034-caller-death-ownership.md:1750 |

## Dispositions

Round 1, re-checked at the branch's round-1 fix head (the commit after an ordinary main merge, subject 'Clarify negative startup publication and settlement contracts'; head named in the Linear marker only), against the newer published guardian review-source backup ref (the one whose head commit is 'Retain producer clock failure with borrowed outer setup custody'). Static, read-only; no build, test, Kani or replay run.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fix commit 'Clarify negative startup publication and settlement contracts': the former AC-65 is split into AC-65 and AC-70..AC-75, each one obligation |
| FND-002 | fixed | fix commit 'Clarify negative startup publication and settlement contracts': AC-57..AC-63 are now single obligations, with the remainder moved to AC-66..AC-69 |
| FND-003 | fixed | fix commit 'Clarify negative startup publication and settlement contracts': the new FR-034 sections contain no permissive may; COMMIT and retirement are explicit shall rules at FR-034:331-337 |
