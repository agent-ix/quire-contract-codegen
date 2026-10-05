---
id: SR-1566
title: "integrity review of IR-629 process-provider BackendKind spec (quire-contract-codegen#287)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@43fbf2f6083663e680e1ebeeeeefe11b288fdef5; spec/routed/functional/FR-019-capability-settlement.md, spec/routed/functional/FR-022-routed-generation.md, spec/decisions/ADR-002-backend-adapter-boundary.md, spec/routed/matrix/TC-046-process-provider-settlement.md, spec/routed/matrix/tests.md, spec/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/ADR-002
    type: reviews
---
# Integrity review: IR-629 process-provider BackendKind (PR #287)

## Summary

Ticket: IR-629. Consistency of the new FR-019 section with ADR-002, FR-022, TC-046, both matrices
and the code at the head. `BackendKind` (src/routed/capability.rs:423) has one variant, `Kani`.
Exhaustive matches over it: `index`, `identity`, `negotiate_arm` (capability.rs), and
`GenerationContexts::has` and the `generate_routed` dispatch (src/routed/generate.rs:76, 239);
`ALL: [Self; 1]` must grow; `KindOutput` (generate.rs:85) has one variant per kind by construction
but is not a match over `BackendKind`; `from_identity` is a search through `identity()`, not a
match. No adapter trait (FR-026) and no execution or terminal-record match over `BackendKind`
exists in the code today. Routed matrix and `spec/tests.md` agree on AC-11..14 / TC-046 Planned.

## Verdict

Changes requested (one medium, two low). The ADR-002 amendment and the FR-022 out-of-scope note
are correct and minimal in what they say; the defects are what they leave stale or ungated.

## Examined

- ADR-002 Q4 and the IR-629 amendment, ADR-002 Consequences (examined)
- FR-022 Out of Scope note (examined)
- FR-019-AC-11 and TC-046 Status (examined)
- FR-019 Dependencies (examined)
- spec/routed/matrix/tests.md rows 19 and 47, spec/tests.md Routed row (examined, clean)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-019-AC-11 says only that it "waits on open question 2", and TC-046's Status names only questions 1 and 2, but adding the variant does not compile until `GenerationContexts::has` and the `generate_routed` dispatch (generate.rs:76, 239) gain arms and `KindOutput` a variant, which FR-022 explicitly states none of (open question 3). AC-11..14 are therefore also gated on question 3, and nothing says so | spec/routed/functional/FR-019-capability-settlement.md:151; spec/routed/matrix/TC-046-process-provider-settlement.md:48; spec/routed/functional/FR-022-routed-generation.md:258-261 |
| FND-002 | low | ADR-002 Consequences still reads "FR-019 and FR-022 gain rows only when a second backend is added", and Q4 step 5 says FR-022 gains the kind's rows; the amendment adds FR-019 rows before any variant exists and FR-022 gains none, so the unchanged text now contradicts the amendment | spec/decisions/ADR-002-backend-adapter-boundary.md:87,100 |
| FND-003 | low | FR-019 Dependencies names "quire-driver's pre-negotiation conversion (IR-609) ... filed by the driver lane at merge"; IR-609 already exists (In Progress, "Provider, plugin host and cache") and its description mentions neither the pre-negotiation conversion nor `BackendKind`, so the pointer and "filed at merge" do not agree with the tracker | spec/routed/functional/FR-019-capability-settlement.md:161-162 |

## Dispositions

Round 1, reviewed at 48a7a96eca9d7dbc66a3c7a808f042151da2607c. New defects found this round are recorded in SR-1570.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 48a7a96 |
| FND-002 | fixed | 48a7a96 |
| FND-003 | fixed | 48a7a96 |
