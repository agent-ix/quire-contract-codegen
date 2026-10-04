---
id: TC-039
title: "Verify bounded proof ceilings, their inconclusive reasons and the proof subject"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: verifies
---
# TC-039: Verify bounded proof ceilings, their inconclusive reasons and the proof subject

## Description

Verify that every harness identity records its ceilings, that a run exceeding either ceiling settles
with its own inconclusive reason, that evidence records the bounds and ceilings used, that every
identity, refusal and evidence names its family, and that the proof subject and a narrowed bound are
recorded as FR-028 states.

## Test Procedure

1. Generate one harness at two memory ceilings and at two wall-clock ceilings.
2. Run a harness whose run cannot finish inside a small wall-clock ceiling.
3. Run a harness whose run exceeds a small memory ceiling.
4. Read the evidence of a verified run and of each run above.
5. Read the family named in a harness identity, a generation refusal and an execution evidence.
6. Generate a family with no supplied shadow.
7. Generate a family with a supplied shadow.
8. Narrow one argument inside its declared domain.
9. Narrow one argument outside its declared domain.
10. Run a batch of N harnesses sharing one wall-clock budget T with a report in which one member's
    entry reads Failure, no checks, exit status `timeout` beside finished members; a stand-in that
    is still running at N times T; a budget whose product overflows; and a T above 4294967295
    seconds whose product with N fits, and a `Duration::MAX` request, reading each launch's
    arguments and outer bound.

## Expected Results

1. Each ceiling change changes the identity (FR-028-AC-1).
2. The run is stopped and is `inconclusive` with the timed-out reason naming the ceiling, never
   verified or falsified (FR-028-AC-2).
3. The run is stopped and is `inconclusive` with the memory-exhausted reason naming the ceiling
   (FR-028-AC-3).
4. Each evidence records every argument's bounds and both ceilings (FR-028-AC-4).
5. Each names the family (FR-028-AC-5).
6. The harness records proof subject `production` (FR-028-AC-6).
7. The result holds a `bounded_shadow` harness together with its refinement obligation
   (FR-028-AC-7).
8. The identity and the evidence record the declared domain, the narrowed bound and that the
   harness covers only the narrowed bound (FR-028-AC-8).
9. The obligation is refused with a typed reason and no harness (FR-028-AC-9).
10. The timed-out member is `inconclusive` timed-out naming T and the finished members keep their
    results; the second batch is killed, refused as timed out and no member is classified; the
    third never elapses and does not panic; each oversized T launches without `--harness-timeout`
    and the batch runs, its outer bound elapsing at N times T where the product fits and never where
    it does not (FR-028-AC-12).

## Status

Planned. At this revision the run is held to a caller-declared wall-clock budget, no memory ceiling
is set, and no identity records a ceiling, a family or a proof subject.
