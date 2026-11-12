---
id: TC-068
title: "Verify option presence and value clause harnesses"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-054
    type: verifies
---
# TC-068: Verify option presence and value clause harnesses

## Description

Verify [FR-054](../functional/FR-054-option-clause-harnesses.md) through admitted V2 clause
packages over option operands, public CG generation and, one run at a time, installed Kani
through CG's runner. This case is planned. The scratch probe in
`docs/qualification/ir1448-kani-option-probe.md` measured the encoding only. Every produced-result
mutant fails the step "produced result agrees with the independent checked condition".

## Test Procedure

1. Generate the harness for `x: Option<Int[0, 1000]>` and inspect the source: one presence `bool`,
   one `i64`, no allocation. Mutate by rendering the value in an enum arm; the inspection must
   fail (FR-054-AC-1).
2. Enumerate the admitted values on the host and assert 1002 draws, each once. Mutate by leaving
   the absent payload free (the absent value appears twice, at payload 1) and by widening the
   present bound (payload 1001 appears); each must fail (FR-054-AC-2).
3. Run the claim `result = option.present(x)` healthy and require `Verified` with the cover
   satisfied. Mutate the oracle's presence read to its negation and require `Falsified` at
   presence 0 and presence 1, payload 0 (FR-054-AC-3).
4. Run `result = (option.present(x) implies option.value(x) <= 1000)` healthy, then the subject
   with consequent `x.value < 1000`; require `Falsified` with presence 1, payload 1000
   (FR-054-AC-4).
5. On a direct oracle fixture outside admission (an unguarded `result = (option.value(x) <= 1000)`
   built without the IR definedness check), require the stop at presence 0, payload 0. Mutate to
   a default payload 0 and to an assumed presence; each must make the fixture verify and so fail
   (FR-054-AC-5).
6. Require the guarded claim of step 4 to complete at the absent draw. Mutate the oracle to read
   `option.value` before the presence test and require the failure at presence 0, payload 0
   (FR-054-AC-6). Run the subject `present and value <= 1000` and require `Falsified` at presence 0,
   payload 0 (FR-054-AC-7).
7. Repeat step 4 for `y: Option<R>`, `R` a record of `n: Int[0, 1000]` and a Boolean, with the
   subject `n < 1000` falsified at presence 1, `n` 1000 (FR-054-AC-8). Enumerate 2003 draws and
   mutate the absent Boolean to free; the absent value must appear twice (FR-054-AC-9).
8. Request one clause per refusal: a payload leaf of each family of FR-054-AC-10, an option of an
   option and a record payload holding an option leaf, a record field declared `field?`, and a
   clause applying `quire.op.option.some`. Assert the typed reason, the node and no harness, and
   that none is `NoFiniteEncoding`, `StateFrameRefused` or `V2ClauseOperatorNotSupported`
   (FR-054-AC-10 to FR-054-AC-14). Mutate by returning `V2ClauseOperatorNotSupported`, by
   classifying the record-held option leaf as `PayloadNotScalar`, and by lowering `field?` as a
   two-state option; each must fail.
9. Request a refused clause beside a supported sibling and assert the sibling keeps its harness
   (FR-054-AC-15). Request an option of an unbounded integer and assert `requires_bound`; mutate
   to `PayloadNotScalar` and require failure (FR-054-AC-16).
10. Request a clause with an unsupported operator at a lower node id than an option refusal, a
    clause with two option refusals, and a clause whose payload is both non-scalar and unbounded;
    assert the reported refusal is the lowest node id, and the shape reason first (FR-054-AC-17).
11. Compare the identity record before and after changing the absent rule to a free payload and
    assert they differ (FR-054-AC-18). Decode a present and an absent playback to one option
    witness value each (FR-054-AC-19) and refuse an absent playback with payload 1
    (FR-054-AC-20).
12. Run steps 3, 4, 6 and 7 through the two-phase runner, one run at a time, recording verdict and
    wall time and the memory only where the environment supplies a source (FR-054-AC-21). Compile
    the generated oracle of steps 5 and 6 in a scratch crate and observe each verdict
    (FR-054-AC-22).
13. Inspect FR-015-AC-42 for the amendment (FR-054-AC-23).

## Expected Results

Healthy clauses verify with a satisfied cover; every mutant fails at its named step with its
distinguishing input; every refusal keeps its own reason and no harness.

## Status

PLANNED (IR-1448): no FR-054 criterion is implemented or tagged.
