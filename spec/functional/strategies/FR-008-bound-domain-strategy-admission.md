---
id: FR-008
title: "Derive integer strategy domains from bound clauses"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-008: Derive integer strategy domains from bound clauses

## Description

When a caller requests strategies for one executable clause of a public IR `BoundPackage`, the
generator shall derive every generated value's domain from the clause's own integer declarations. The
generator admits only the clause forms listed below and refuses every other form with a structured
diagnostic that names the clause and source locus.

This is the numeric/state slice of [FR-002](../FR-002-tristate-proptest.md) for
agent-ix/quire-contract-codegen#3 under epic agent-ix/quire-spec-language#83. Domains come from the
model, not from caller-supplied constraints: SL lowers a model scalar such as ConfigVersion's
`VersionNumber` (0..=1000) into `IntegerType { domain, minimum, maximum, overflow }` on each
`ValueDeclaration`.

## Inputs

- A public `quire_contract_ir::BoundPackage` and one full `ClauseRef` naming a clause in it.
- The clause's `DeclarationEnvironment` values, `ExecutionPoint` anchor, and `TypedExpression`.
- The caller's attestation context.

## Outputs

- A per-declaration domain census: declaration name, kind (`Input`/`State`), observations read
  (`Current`/`Pre`/`Post`), inclusive minimum and maximum.
- Either an admitted relation handed to [FR-009](./FR-009-constructive-correlated-populations.md) and
  [FR-010](./FR-010-domain-boundary-campaigns.md), or a `StrategyDiagnostic`.

## Behavior

- The generator shall take each read value's domain from its `ValueDeclaration`'s `IntegerType`
  minimum and maximum and shall not accept a caller-supplied range for a bound clause.
- The generator shall admit a clause only when all of these hold:
  - its root is exactly one `Compare` node with any of the six `ComparisonOperator`s;
  - each operand is a `ValueReference` to an integer declaration or an `IntegerLiteral`;
  - at least one operand is a `ValueReference`;
  - the clause carries no definedness obligations;
  - every read declaration uses `OverflowPolicy::Reject`.
- The generator shall admit `ValueReference` observations `Current`, `Pre`, and `Post`. It shall
  treat the `Pre` and `Post` reads of one `State` declaration as two correlated generated values that
  share that declaration's domain.
- If the clause contains any other node, including `Numeric`, `NumericNegate`, Boolean connectives
  over numeric compares, `deref`, or `reaches`, then the generator shall refuse with
  `UnsupportedRelation`.
- If the clause carries a definedness obligation, then the generator shall refuse with
  `UnsupportedObligations`, matching the oracle refusal, and shall not generate a population that
  assumes the obligation holds.
- If a read declaration uses `OverflowPolicy::Saturate`, then the generator shall refuse with
  `UnsupportedOverflowPolicy`.
- If a read declaration is not an integer and not a Boolean, then the generator shall refuse with
  `UnsupportedDeclarationType`.
- If the `ClauseRef` does not name a clause in the package, then the generator shall refuse with
  `UnknownClause` and terminal state `invalid-input`.
- Every refusal diagnostic shall carry the full `ClauseRef` and the source span of the first
  offending node or declaration.
- The generator shall not admit a clause that [FR-001](../FR-001-deterministic-oracles.md)'s bound
  oracle generation refuses. Strategy admission may be stricter than oracle admission, never looser.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-008-CON-1 | Domain bounds SHALL be read only through the public `quire_contract_ir` API at the pinned revision; no private decoder or local copy of the IR wire shape. | Interface | Inspection |
| FR-008-CON-2 | The generator SHALL decide admission before rendering any source; a refused clause yields no partial bundle. | Integrity | Integration Test |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-008-AC-1 | Given ConfigVersion `VersionUnchanged`, the census lists one `State` declaration read at `Pre` and `Post` with inclusive domain 0..=1000 taken from its `IntegerType`. | Test (TC-017) |
| FR-008-AC-2 | Given SL's `integer-healthy` clause `amount < 7`, the census lists `amount` with domain 0..=1000 and the relation is admitted with operator `Less` against literal 7. | Test (TC-017) |
| FR-008-AC-3 | A clause containing `Numeric`, `NumericNegate`, a Boolean connective over numeric compares, `deref`, or `reaches` is refused with `UnsupportedRelation`, the full `ClauseRef`, and the offending node's span; no bundle is emitted. | Test (TC-017) |
| FR-008-AC-4 | A clause with a non-empty obligation list is refused with `UnsupportedObligations`; a `Saturate` declaration is refused with `UnsupportedOverflowPolicy`; an unknown `ClauseRef` is refused with `UnknownClause` and terminal state `invalid-input`. | Test (TC-017) |
| FR-008-AC-5 | For every clause in the admission test corpus that bound oracle generation refuses, strategy generation also refuses. | Test (TC-017) |

## Dependencies

- **Upstream**: [FR-001](../FR-001-deterministic-oracles.md) bound oracle admission (codegen#4 numeric
  slice), [FR-002](../FR-002-tristate-proptest.md) shaped strategies.
- **Downstream**: [FR-009](./FR-009-constructive-correlated-populations.md),
  [FR-010](./FR-010-domain-boundary-campaigns.md), [TC-017](../../test/strategies/TC-017-bound-domain-admission.md).
