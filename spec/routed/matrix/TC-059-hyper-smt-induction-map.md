---
id: TC-059
title: "Map QSL-encoded HP-2 induction solver results"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-045
    type: verifies
---
# TC-059: Map QSL-encoded HP-2 induction solver results

## Description

Verify CG's solver submission and typed method outcome over QSL-owned
canonical HP-2 product queries, without implementing a second encoder.

## Test Procedure

1. Supply one QSL-admitted HP-2 safety product with two ordered components,
   a negated-body automaton state and a `μ_U`-admitted joint step. Ask QSL's
   hyper query API for base and step bytes at depth 1. Capture the bytes at
   the selected solver boundary and assert byte equality and exact
   product/item/depth association (FR-045-AC-1).
2. Return `unsat` for both queries; assert one `Inductive { depth: 1 }`
   method outcome and QSL's FR-182-AC-7 settlement. Swap one query's
   product identity or depth and assert no proof (FR-045-AC-1, AC-2).
3. Drive base `sat` with a reconstructable product path, step `sat` after
   base `unsat`, solver unknown, QSL encoding refusal, a governed resource
   stop and a missing response. Assert the distinct candidate,
   induction-not-closed or typed inconclusive outcome; none claims a proof
   or terminal refutation before QSL replay (FR-045-AC-2).

## Expected Results

Only the two matching `unsat` queries produce `Inductive`. Every adverse
branch retains its product, item and depth and remains eligible only for
QSL-owned replay or inconclusive settlement.
