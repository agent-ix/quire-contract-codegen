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
