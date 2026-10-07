---
id: SR-3101
title: "IR-689 spec review (EARS conformance): FR-034 AC-78..AC-82 and the observation transport prose"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen branch spec/ir689-stage2-observation (frozen head named in the Linear marker); spec/kani/functional/FR-034-caller-death-ownership.md section 'Feature-only stage-2 observation transport' (lines 1453-1549) and the five new AC rows FR-034-AC-78..AC-82 (lines 1902-1906); compared with the surrounding FR-034 prose and AC conventions"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
---

## Summary

Ticket: IR-689. This pass checks the new normative sentences for EARS form: a named actor, "shall", one obligation per statement, and a trigger or state clause where one applies. It also checks each new AC for atomicity.

The section opens with a correct optional-feature clause ("Where guardian-test-support is enabled", FR-034:1455) and closes with its complement ("When guardian-test-support is disabled", FR-034:1541). Most sentences name C, L or O and use "shall". The FR-034 AC table convention is declarative, PLANNED/UNRUN-prefixed rows, and the new rows follow it. AC-82 is atomic.

The defects are compound criteria and a few sentences with no actor or with "must". The Linear range reservation describes AC-78..AC-82 as "five atomic planned obligations"; that claim is re-measured here, not assumed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-034-AC-79 bundles at least five independently failing obligations in one criterion: (1) five closed event kinds; (2) each emitted at most once; (3) payloads taken at the actual boundary; (4) producer order; and (5) ordinary samples not suppressed. It also names five separate adverse controls: missing, duplicate, partial, wrong-order and fabricated-operation. A partial implementation, for example once-only frames done but boundary-payload provenance owed, cannot be reported per obligation. One Test/Analysis tag on AC-79 would claim all of them. | spec/kani/functional/FR-034-caller-death-ownership.md:1903 |
| FND-002 | low | Three other new criteria each bundle two to four obligations. AC-78 has post-Armed acceptance, exclusive O-only writer, failure controls, no child mapping and no public API. AC-80 has the carrier source restriction and before-lease-close retention. AC-81 has the pre-exposure charge, typed failure with unchanged cleanup, and no new cap, ACK or pause. | spec/kani/functional/FR-034-caller-death-ownership.md:1902; spec/kani/functional/FR-034-caller-death-ownership.md:1904; spec/kani/functional/FR-034-caller-death-ownership.md:1905 |
| FND-003 | low | Several requirement-position sentences name no system actor or use "must". "each scenario shall require its applicable events explicitly" makes a scenario the actor; the verifier or harness is meant. "Source feasibility and bounds must be established before fixture CODE delivery" has no actor and uses "must". "lack of this property shall block CODE delivery" makes an absence the actor. None of these can be assigned to C, L, O or the CG verification harness as written. | spec/kani/functional/FR-034-caller-death-ownership.md:1503; spec/kani/functional/FR-034-caller-death-ownership.md:1546; spec/kani/functional/FR-034-caller-death-ownership.md:1519 |

## Verdict

**Changes requested (medium).** Split AC-79 so that each obligation has its own falsifiable row: event set and once-only, boundary payload, producer order, and ordinary-sample preservation. Split AC-78 and AC-81 likewise. Give the three actorless sentences a named actor (the CG verification harness or the CODE author) and use shall.

Examined with no EARS finding: FR-034:1455-1467 (pipe custody), 1469-1477 (reader binding), 1490-1491 and 1501-1504 (record emission), 1525-1531 (completion carrier), 1541-1545 (feature-off absence), and the AC-82 row.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | The sixteen rewritten or new rows AC-78..AC-93 are obligation-shaped "shall" statements. make spec now emits sixteen new `ac:non-canonical-shape` warnings ("an acceptance criterion is a verification statement"). The surrounding FR-034 rows, including AC-51..AC-77, use the declarative verification form. The exception sentence also uses the permissive "may add" in requirement position. The gate still passes, but the slice adds avoidable validator noise and departs from the file's convention. | spec/kani/functional/FR-034-caller-death-ownership.md:1946-1961; spec/kani/functional/FR-034-caller-death-ownership.md:1460 |

## Dispositions

Round 1 was re-checked at the branch's round-1 fix head (subject 'Resolve IR689 observation selection and independent evidence obligations'; head named in the Linear marker only). The check was static and read-only, and make spec passes.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fix commit 'Resolve IR689 observation selection and independent evidence obligations': the former AC-79 is split. AC-79 is fixed-frame grammar, AC-85 is exactly-once presence, AC-86 is boundary provenance, AC-87 is producer order and AC-88 is schedule preservation. Each has its own TC step (29b, 29h-29k) and its own Expected row. |
| FND-002 | fixed | fix commit 'Resolve IR689 observation selection and independent evidence obligations': AC-78 is split into AC-78, AC-83 and AC-84, and AC-80 into AC-80 and AC-89. AC-81 keeps only the pre-exposure charge, with failure noninterference in AC-90 and no-ACK/pause in AC-88. |
| FND-003 | fixed | fix commit 'Resolve IR689 observation selection and independent evidence obligations': "The verifier shall require the events applicable to its selected scenario" (FR-034:1524-1525). Frame fit is now "The CODE author shall prove ...; the CODE reviewer shall reject ..." (FR-034:1533-1534). The actorless "must be established" sentence is removed. |
