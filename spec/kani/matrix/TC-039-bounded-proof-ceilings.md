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
recorded as FR-028 states. Verify the shadow contract (steps 11 to 22): a shadow's independence from
the code it stands for, its refinement obligation and the class and strength of its run, the
abstractions it owes evidence for, its refused shapes, the memory ceiling and the seeded mutants it
must catch.

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

### The shadow contract (planned, IR-241)

Steps 11 to 22 use a stand-in family, not composite equality: a two-leaf shadow (two bounded
integers compared for equality) with a stand-in production subject that the test can mutate, so
that the machinery of FR-028-AC-13 to FR-028-AC-24 is asserted without the composite family's
generator. The composite family's own run is TC-025 steps 29 to 36.

11. Parse the source of the stand-in shadow harness and read its imports, stubs and assumptions;
    generate it again with a mutated generator that imports a runtime type, one that emits a
    `kani::stub`, and one that assumes the two drawn values are equal.
12. Read the result's refinement obligation: the production subject's symbol, the shadow, the
    bounded domain, the abstraction function and the comparison's observables; locate the
    abstraction function's module and the imports of the shadow, the expectation and the harness
    (only `core`, `alloc`, `std` and `kani`).
13. Run the refinement over the stand-in subject unmutated, over one that charges one extra
    `equality.pair`, over one that refuses a legal pair, over one that returns the other
    verdict, and over one whose environment refuses to construct a value the shadow draws, reading
    each case outcome.
14. Run the refinement over a domain of exactly the case cap, one case larger, and one whose case
    count overflows `u128`, twice each with equal inputs and one run with another seed, reading the
    class, the case count, the cases run, the seed, that a sampled run executed the boundary cases
    (one leaf at a time, plus all-lower and all-upper, not a cross product) and exactly the
    request's sample size of seeded cases, and that the same seed repeats them.
15. Read the proof strength of a verified `production` harness; of a verified `bounded_shadow`
    harness with an exhaustive refinement, a sampled one, none and one stopped at a ceiling; and of
    a verified shadow harness beside a refinement disagreement.
16. Read the identity's abstractions, unexercised behaviours and shared helpers with their
    `value_bearing` or `order_only` marks; generate a shadow holding an abstraction no evidence
    discharges; mutate each `value_bearing` shared helper (a bound, a presence) and run the shadow
    harness, the refinement and the family's independent check together; permute an `order_only`
    helper and read that no check is required to fail.
17. Request a refinement domain narrower than the shadow's drawn domain on one argument.
18. Request items whose shape is unbounded, has no shadow, or exceeds the size budget, beside a
    supported item.
19. Run the launcher stand-in of step 3 as a process tree whose child, not the launcher, exceeds the
    memory ceiling, under each stand-in mechanism (one that observes resident memory, one that
    only limits it); read the evidence of a run that finishes under the ceiling for the mechanism
    and, where observed, the peak; run on a stand-in platform with no mechanism; run a batch
    whose launcher's child exceeds the ceiling.
20. Run the seeded mutants of the stand-in: a shadow that always answers `true`, a production
    subject that never compares one leaf, a mutated `value_bearing` shared helper that the
    production subject reads independently, and one it reads through the same helper.
21. Read the evidence of a shadow run and of a refinement run for the Kani, Rust and solver
    versions, the unwind bound and the option vector.
22. Run the refinement with a stand-in subject that outruns its wall-clock ceiling and one that
    exceeds its memory ceiling; read the refinement evidence (class, cases, seed, ceilings,
    mechanism, outcome, versions) beside the shadow run's.

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
11. The unmutated shadow source has no runtime path, no stub and no assumption beyond the domain
    bounds, and each mutated generator fails the check it breaks (FR-028-AC-13).
12. The obligation names the subject, shadow, domain, abstraction function and comparison; the
    abstraction function's module is imported by neither the shadow, the expectation nor the
    harness (FR-028-AC-14); the shadow source names no crate but `core`, `alloc`, `std` and `kani`
    (FR-028-AC-13).
13. The unmutated subject agrees on every case; the extra charge, the refused legal pair, the
    other verdict and the construction refusal are each a recorded disagreement carrying the case and both results, none
    skipped (FR-028-AC-15).
14. The domain of exactly the cap is `exhaustive` with every case run; the larger and the
    overflowing domains are `sampled` with the boundary cases plus exactly the sample size of
    seeded cases, the seed and the cases run recorded, and equal inputs run equal cases
    (FR-028-AC-16).
15. The proof strengths read as FR-028-AC-17 states, and no `bounded_shadow` result reads
    `production_proved`.
16. The identity lists each abstraction with its evidence, each unexercised behaviour and each
    shared helper with its mark; the undischarged abstraction is refused naming it with no
    harness; each `value_bearing` helper's mutation fails the shadow harness, the refinement or
    the independent check, and an `order_only` permutation is exempt (FR-028-AC-18).
17. The narrower refinement domain is refused naming the argument, with no harness (FR-028-AC-19).
18. Each of the three shapes is refused with its disposition and reason, none emits a harness or a
    `production` fallback, and the supported item settles as it does alone (FR-028-AC-20).
19. The child's excess kills the whole tree and settles `inconclusive` `memory_exhausted` naming the
    ceiling, distinct from `timed_out`; each evidence names the mechanism and, where observed, the
    peak;
    the platform with no means refuses the run before the backend starts; the batch is killed
    whole and refused as memory-exhausted with no member classified (FR-028-AC-21).
20. The always-true shadow is `Falsified`, the never-comparing subject settles `refinement_failed`,
    the independently read helper mutant settles `refinement_failed` and the same-helper mutant is
    caught by the independent check; the unmutated stand-in verifies and agrees (FR-028-AC-22).
21. Each evidence records the versions, the unwind bound and the complete option vector of the run
    that produced it (FR-028-AC-23).
22. Each stopped refinement is stopped at its ceiling, records it, and yields
    `shadow_proved_refinement_inconclusive`; the refinement evidence is recorded beside the shadow
    run's (FR-028-AC-24).

## Status

Planned, except step 10 (FR-028-AC-12, the batch wall-clock rule), which is implemented (IR-277)
and run by the `tc_043_*` batch tests listed in TC-043, tagged to both cases. At this revision the
run is held to a caller-declared wall-clock budget, no memory ceiling is set, and no identity
records a ceiling, a family or a proof subject.

Steps 11 to 22 (FR-028-AC-13 to FR-028-AC-24, IR-241) are planned: no shadow, refinement obligation,
proof strength or `MemoryExhausted` reason exists in `src/`, and the evidence records no tool
version (measured: no `--version` read and no version field in `src/kani`). They are
asserted against a stand-in family and a stand-in production subject, so the machinery is tested
before the first real family (TC-025 steps 29 to 36).
