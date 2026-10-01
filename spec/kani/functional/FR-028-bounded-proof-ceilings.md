---
id: FR-028
title: "Bound every Kani proof so it finishes, and record what it proved"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: references
  - target: ix://agent-ix/quire-contract-codegen/ADR-003
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-011
    type: references
---
# FR-028: Bound every Kani proof so it finishes, and record what it proved

## Description

The code generator shall bound every Kani harness so that its run can finish: each harness identity
records a memory ceiling and a wall-clock ceiling, a run that exceeds either settles `inconclusive`
with its own reason, and the evidence records the bounds and ceilings the run used. Proof coverage
is maximised per family. A family proves the production code where its harness verifies within the
ceilings, and a bounded shadow together with a refinement obligation where it does not
([ADR-003](../../decisions/ADR-003-kani-tractability.md)).

## Inputs

- An [FR-015](./FR-015-bounded-kani-obligations.md) generation request, carrying a memory ceiling in
  bytes and a wall-clock ceiling in seconds as request-level values, and optionally a narrowed bound
  for an argument.
- For a family whose production harness cannot verify within its ceilings, the bounded shadow model
  of that family and its refinement obligation, derived independently of the code under proof (QSL
  ADR-011 §2.3). Contract Runtime co-owns them for its kernel types.

## Outputs

- Harness identities that record the ceilings, the proof subject, the family and, for each narrowed
  argument, both its declared domain and its narrowed bound.
- Execution evidence ([FR-017](./FR-017-kani-execution-evidence.md)) that records the bounds
  and ceilings the run used.

## Behavior

- The generator shall record the request's memory ceiling and wall-clock ceiling in every harness
  identity.
- When a harness runs, the generator shall hold the backend process tree to the memory ceiling and
  the wall-clock ceiling its identity records.
- If a run exceeds its wall-clock ceiling, then the generator shall stop it and classify it
  `inconclusive` with the timed-out reason, naming the ceiling.
- If a run exceeds its memory ceiling, then the generator shall stop it and classify it
  `inconclusive` with the memory-exhausted reason, naming the ceiling.
- The generator shall never classify a run that exceeded a ceiling as verified or falsified.
- The generator shall record in each execution evidence the bounds of every symbolic argument and
  the two ceilings the run was held to.
- The generator shall name the family in every harness identity, refusal and execution evidence, so
  that coverage is reportable per family.
- The generator shall record each harness identity's proof subject as `production` or
  `bounded_shadow`.
- Where no shadow is supplied for a family, the generator shall emit its harness over the production
  code.
- Where a shadow is supplied for a family, the generator shall emit the harness over the shadow
  together with its refinement obligation in the same result.
- Where a request narrows an argument's bound inside its declared domain, the generator shall record
  the declared domain, the narrowed bound and that the harness covers only the narrowed bound, in
  the harness identity and in the evidence.
- If a narrowed bound is not inside the argument's declared domain, then the generator shall refuse
  the obligation with a typed reason and emit no harness.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-028-AC-1 | Every harness identity records the request's memory ceiling and wall-clock ceiling, and changing either ceiling changes the identity. | Test (TC-039) |
| FR-028-AC-2 | A run that exceeds its wall-clock ceiling is stopped and classified `inconclusive` with the timed-out reason naming the ceiling, and never verified or falsified. | Test (TC-039) |
| FR-028-AC-3 | A run that exceeds its memory ceiling is stopped and classified `inconclusive` with the memory-exhausted reason naming the ceiling, distinct from the timed-out reason, and never verified or falsified. | Test (TC-039) |
| FR-028-AC-4 | Every execution evidence records the bounds of each symbolic argument and the two ceilings the run was held to. | Test (TC-039) |
| FR-028-AC-5 | Every harness identity, generation refusal and execution evidence names its family. | Test (TC-039) |
| FR-028-AC-6 | A harness of a family with no supplied shadow records proof subject `production`. | Test (TC-039) |
| FR-028-AC-7 | A family with a supplied shadow yields a harness with proof subject `bounded_shadow` and its refinement obligation in the same result, and never the shadow harness alone. | Test (TC-039) |
| FR-028-AC-8 | An argument narrowed inside its declared domain records the declared domain, the narrowed bound and that the harness covers only the narrowed bound, in the harness identity and in the evidence. | Test (TC-039) |
| FR-028-AC-9 | A narrowing outside the argument's declared domain is refused with a typed reason and no harness. | Test (TC-039) |

## Dependencies

- **Upstream**: [FR-015](./FR-015-bounded-kani-obligations.md), whose harnesses these ceilings
  bound; the Contract Runtime shadow model and refinement obligation for its kernel types.
- **Downstream**: [TC-039](../matrix/TC-039-bounded-proof-ceilings.md),
  [FR-017](./FR-017-kani-execution-evidence.md), which runs a harness under these ceilings;
  [FR-029](./FR-029-run-outcome-terminal-record.md), which maps the two inconclusive reasons.
