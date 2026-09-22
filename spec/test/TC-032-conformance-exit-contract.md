---
id: TC-032
title: "Verify the generation-conformance producer's own exit contract"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-006
    type: verifies
---
# TC-032: Verify the generation-conformance producer's own exit contract

## Description

Verify that the bounded generation-conformance producer's process exit status is the declared
function of the rows it published — 0 when every row passed, 1 when any failed, 2 when none failed
and one was vacuous — that the status is classified from the published bytes rather than from a row
collection held beside them, that exactly one exit path outside the test module carries the
classifier's value, and that the compiled binary itself exits 0 against the real corpus.

## Test Procedure

1. Classify published row sets of each shape through the producer's own serialization: all passing;
   one failing beside a pass; one vacuous beside a pass; a failing and a vacuous row together; and
   no rows at all.
2. Classify a published line whose `outcome` is not a string, and require the run to abort rather
   than return a code.
3. Scan the producer's own source: require exactly one call reaching the process exit status before
   the test module, require that call's argument to be the classifier's value rather than a literal,
   and require no top-level `fn` to be declared after the test module, where that scan cannot see it.
4. Build the plain example binary unconditionally, execute it against the real bounded corpus, and
   assert the process's own exit status.

## Expected Results

Step 1 yields 0, 1, 2, 1 and 0 respectively — the fourth because a failing row outranks a vacuous
one, and the fifth under the same rule as any other run with nothing failing and nothing vacuous
rather than through a case of its own. Step 2 panics with a message naming the missing string
`outcome`. Step 3 finds exactly one exit path and it carries the classifier's value. Step 4 exits 0.

Step 3 is a textual census and is stated as one: it catches a constant substituted for the
classifier's value and a second exit path added before or after the test module, and it does not
catch an early `return` in `main`. Step 4 is an end-to-end observation of a passing corpus and not a
non-empty one: a run that published no rows exits 0 as well, and the ten-row floor asserted over the
emitted JSONL under TC-009 is what refuses that.

These execute in the producer file's own `#[cfg(test)]` module, which `[[example]] test = true`
makes `cargo test` build and run; they are not reached by the `cargo run` invocation SUITE-001
declares, which is the producing lane rather than the verifying one.
