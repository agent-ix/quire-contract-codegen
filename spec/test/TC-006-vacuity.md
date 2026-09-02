---
id: TC-006
title: "Distinguish vacuity and unexecuted control flow"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-004
    type: verifies
---
# TC-006: Distinguish vacuity and unexecuted control flow

## Description

Verify coverage export and runtime counts classify consequent execution, vacuity, rejection, discard,
ordinary unexecuted flow, and test outcome distinctly per requirement.

## Test Procedure

Run always-false, always-true, mixed, rejected, discarded, and unexecuted fixtures; export LLVM
coverage and combine it with source maps and runtime reports.

## Expected Results

An unobserved consequent cannot claim exercised coverage, vacuity is explicit, and unrelated
unexecuted control flow receives a distinct classification with complete tool/source-map identity.
