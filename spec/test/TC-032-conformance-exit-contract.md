---
id: TC-032
title: "Verify the generation-conformance exit status"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-006
    type: verifies
---
# TC-032: Verify the generation-conformance exit status

## Description

Verify that the generation-conformance program's exit status is the declared function of the rows
it printed, and that the compiled binary exits 0 against the real corpus.

## Test Procedure

1. Classify printed row sets of each shape: all passing; one failing beside a pass; one vacuous
   beside a pass; a failing and a vacuous row together; and no rows at all.
2. Classify a printed line whose `outcome` is not a string, and require the run to abort.
3. Build the example binary, run it against the real bounded corpus, and check its exit status.

## Expected Results

Step 1 yields 0, 1, 2, 1 and 0. Step 2 panics with a message naming the missing string `outcome`.
Step 3 exits 0.
