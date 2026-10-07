---
id: "SR-3021"
title: "CG IR-655 spec-ears-analysis: FR-034-AC-51..54 and the stage-2 allocation prose"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen@spec/ir-655-stage2-temporal-oracle (frozen head, no PR yet; reviewed revision recorded in the IR-655 Linear marker only, per this repository's no-SHA rule); spec/kani/functional/FR-034-caller-death-ownership.md (Stage-2 live-birth and producer-operation observations section, FR-034-AC-51..54)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
---

# SR-3021: CG IR-655 EARS analysis

## Summary

Ticket: IR-655. The FR-034 requirement statement is unchanged. The PR adds four AC rows and a
prose section. Most of the prose uses "shall" with a named actor: the fixture operation, the
fixture controller, the operation, the oracle, the author. It also contains declarative sentences
without "shall", including the schedule claim that SR-3020 FND-005 reviews. The AC rows follow the
repository's existing declarative AC style, as AC-24 and AC-31..40 do. Reviewed here: atomicity,
named actor and wording that a reader could misread.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-51 bundles at least eight obligations: release after a completed sample, the child's own acknowledgement, both identities and pins live, Completed retained, both pins dead before escalation, I/M settlement before escalation, ordinary accounting continuing, and three rejection clauses. A Test can satisfy some of them and still bind the whole row. The obligations should be split or enumerated so each has its own predicate. | spec/kani/functional/FR-034-caller-death-ownership.md:1583 |
| FND-002 | low | The AC-54 sentence "No backend marker alone supplies no ordering evidence" is a double negative. It can be read as "the absence of a marker supplies no evidence" or as "no marker supplies none". The prose says "No-marker alone shall not establish gate/termination order"; the AC should say the same. | spec/kani/functional/FR-034-caller-death-ownership.md:1586; spec/kani/functional/FR-034-caller-death-ownership.md:1146-1147 |
| FND-003 | low | AC-52 and AC-53 are passive ("is preceded by a producer-bound record") and do not name the actor that produces the record or performs the operation, O or the fixture operation. The binding is only in the prose (FR-034:1126-1133). | spec/kani/functional/FR-034-caller-death-ownership.md:1584-1585 |

## Verdict

Conditional. No requirement statement changed. AC-51 is compound, and two AC rows have wording or
actor defects. AC-53's and AC-54's conditions are otherwise single and testable as written.

## Dispositions

Round 1. Branch head "Strengthen stage-two mutation and temporal predicates"; its revision is
recorded in the Linear marker only.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | AC-51 is now one obligation: release-dependent birth established by the child's own acknowledgement after a completed ordinary sample (FR-034:1605). The death predicate moved to AC-55, and the construction preconditions are checked separately (FR-034:1122-1125, step 28a). |
| FND-002 | fixed | The AC-54 double negative is removed. The AC now says the producer-order predicate rejects early-close and omitted-confirmation controls (FR-034:1608), and the prose says "Absence of a backend marker shall not prove order" (1164-1165). |
| FND-003 | fixed | AC-52, AC-53 and AC-54 now name O as the actor that produces the record (FR-034:1606-1608). |

### Verdict (disposition pass 1)

Clean: no open EARS findings at the round-1 head. AC-55 and AC-56 were checked: each is a single
conditioned obligation with a named actor or a named Analysis subject.
