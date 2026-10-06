---
id: SR-1703
title: "IR-643 PR 301 spec-review/failure-domain review"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen@431616130b06d5621d8795f845b9b75659b8908c; spec/oracle/functional/FR-018-composite-equality-oracles.md:150-170,332-333; spec/oracle/matrix/TC-029-composite-equality-oracles.md:182-204"
review_set: subset
---
# IR-643 PR 301 spec-review/failure-domain review

## Summary

Ticket: IR-643. Reviewed only FR-018-AC-24/25 and TC-029 steps 16/17 at the frozen PR head. The exact QSpec fixture was read at ece259a038b5e3592ffa97b102fa9e3357ba1434; existing CG resolver code was read only to ground the findings.

## Examined scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-018-AC-24 | examined | Type reconstruction of QSpec's `positive-recursive-records.json` List (through `Option<List>`) and Tree (through `Sequence<Tree>` bounded `[0,3]`) closes each back-edge at the in-progress record key, generates equality items built over both record types, and terminates; a synthetic unclosed `Option` self-cycle and a `Sequence` self-cycle each refuse as `TypeResolutionCycle` naming the repeated type node, without a panic, stack overflow or generated symbol. PLANNED (IR-643). |
| FR-018-AC-25 | examined | The per-item type-resolution counter charges one unit for each root type-node entry or followed type-node reference, including a repeated or memoized reference, across both operands and conversion targets, independently of Contract IR lowering; an acyclic chain reaches its 65,537th entry and refuses only its item as `TypeResolutionWorkExhausted { limit: 65_536, consumed: 65_537 }` before further descent and without call-stack exhaustion or generated symbols, while a sibling needing at most 65,536 entries still generates. PLANNED (IR-643). |
| TC-029-step-16 | examined | Recursive records (FR-018-AC-24, planned, IR-643). Read the authoritative QSpec `proposals/checked-package-v2/fixtures/positive-recursive-records.json` through Contract IR's strict reader, without copying it into this repository. Build equality expression items over the fixture's List (`Option<List>`) and Tree (`Sequence<Tree>[0,3]`) record declarations in a valid test package; the fixture itself supplies type nodes, not equality expressions. Assert both items generate, their back-edges resolve to each record's `NodeKey`, and the call terminates. At the equality module's type-resolution seam,  |
| TC-029-step-17 | examined | Type-resolution work (FR-018-AC-25, planned, IR-643). At that same seam, use an acyclic chain of 65,537 type-node entries with a valid terminating leaf and a healthy sibling. Assert the first item refuses `TypeResolutionWorkExhausted { limit: 65_536, consumed: 65_537 }`, emits no symbol and leaves the sibling generated; an exactly 65,536-entry chain does not receive the work refusal. Count root entries and followed member, option and collection references, including both operands and conversion targets and a repeated reference to a previously resolved node, in a short mixed-shape case against  |

## Verdict

PASS: The new text covers positive recursive record closure, unclosed Option/Sequence cycles, typed refusal, the first failed work entry, stack safety, and sibling isolation. The separate fixture and boundary conflicts are recorded by base and integrity review.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
