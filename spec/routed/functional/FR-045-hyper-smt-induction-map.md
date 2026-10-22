---
id: FR-045
title: "Run QSL-encoded HP-2 induction queries and map solver outcomes"
type: FR
relationships:
  - target: ix://agent-ix/quire-spec-language/FR-315
    type: depends_on
  - target: ix://agent-ix/quire-spec-language/FR-176
    type: depends_on
  - target: ix://agent-ix/quire-spec-language/FR-182
    type: references
---
# FR-045: Run QSL-encoded HP-2 induction queries and map solver outcomes

## Description

When QSL supplies canonical SMT queries for an admitted HP-2 safety item's
`HyperProduct`, CG's SMT backend SHALL submit those queries to its selected
solver and return a typed method outcome to QSL's hyper settlement. QSL
FR-315 owns transition-relation encoding, bounded unrolling and induction
base/step query construction. CG SHALL NOT independently encode the product,
its `μ_U` joint-step guard, automaton, definedness or fairness semantics.

## Inputs

- One QSL-admitted HP-2 safety item and its exact FR-176 `HyperProduct`
  selection: the ordered component states, negated-body automaton state,
  initial tuples and joint transitions admitted by `μ_U`.
- Caller-selected induction depth `k >= 1`, QSL-owned canonical base and
  step SMT-LIB query bytes for that product and depth, and the caller's
  solver and execution limits. Query construction uses the future QSL-owned
  hyper form of FR-315's `InductionBase` and `InductionStep`; this contract
  does not define that form.

## Outputs

One typed HP-2 SMT method outcome retaining the selected product identity,
item identity, depth and per-query solver dispositions:

| Outcome | Condition | QSL handoff |
| --- | --- | --- |
| `Inductive { depth: k }` | Both QSL-encoded base and step queries are `unsat` at `k` | Candidate for FR-182 V-3's `proved`, `decisive-witness`, `Uncertified` settlement; certificate checking remains QSL-owned |
| `CandidateViolation { depth: k, witness }` | A base query is `sat` and a concrete product path is reconstructed | QSL replay decides whether the witness is decisive; solver satisfiability alone never settles `refuted` |
| `InductionNotClosed { depth: k }` | Base is `unsat` and step is `sat`, with no decisive base witness | FR-182 V-6 inconclusive |
| `Inconclusive { depth: k, cause }` | Encoding is refused, solver says unknown, a result cannot be reconstructed, or the run reaches its governed execution limit | QSL receives the typed cause; no proof or counterexample is claimed |

## Behavior

CG SHALL preserve the QSL query bytes when submitting them and keep each
solver response paired with its own query kind, product and depth. It SHALL
report `Inductive` only after both required queries return `unsat`, with no
solver unknown, resource stop or unexamined query. A `sat` base result SHALL
not be a terminal violation without QSL's product-path replay. CG SHALL treat
a `sat` step as no proof and hand QSL the method and depth with the outcome;
QSL FR-127 and FR-182 own terminal settlement and QSL-908 owns certificate
rows. This contract chooses no Kani or SMT memory ceiling.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-045-AC-1 | For an admitted HP-2 safety product and `k = 1`, CG requests QSL's base and step queries for the same product and depth, submits their canonical bytes unchanged to the selected solver, and maps `unsat`/`unsat` to `Inductive { depth: 1 }` with product and item identity retained. QSL FR-182-AC-7 then settles that outcome as `proved`, `decisive-witness`, `Uncertified`; CG does not construct the query or assign certification. | Test (TC-059) |
| FR-045-AC-2 | With a base `sat` response, CG returns only a reconstructable product-path candidate for QSL replay; with base `unsat` and step `sat`, it returns `InductionNotClosed { depth: k }`; an unknown, encoding refusal, resource stop, wrong-query association or missing response yields a typed inconclusive outcome and never `Inductive` or terminal `Refuted`. The identities and `k` survive every branch. | Test (TC-059) |

## Dependencies

- QSL FR-176 owns the self-composition product and its `μ_U` semantics.
- QSL FR-315 must gain a QSL-owned HP-2 product query form before this CG
  producer can run; its current `ModelSubject`/temporal-item form does not
  encode a hyper product.
- QSL FR-127 and FR-182 own terminal mapping; QSL-991 consumes the
  `Inductive { depth: 1 }` row. QSL-908 owns certificate settlement.

## Status

PLANNED (IR-974). No HP-2 SMT producer or QSL hyper-product query form exists
in the current tree. This specification is the CG solver/map side only.
