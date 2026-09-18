---
id: FR-008
title: "Admit bound clauses for numeric strategy generation"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
  - target: ix://agent-ix/quire-contract-ir/FR-012
    type: depends_on
  - target: ix://agent-ix/quire-contract-ir/FR-013
    type: depends_on
  - target: ix://agent-ix/quire-contract-ir/FR-014
    type: depends_on
  - target: ix://agent-ix/quire-contract-ir/FR-015
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-034
    type: references
---
# FR-008: Admit bound clauses for numeric strategy generation

## Description

When a caller requests strategies for one executable clause of a public IR `BoundPackage`, the
generator shall admit the clause only when bound oracle generation admits it and the clause has the
single-comparison shape this slice can construct. The generator derives every generated value's
domain from the clause's own integer declarations and refuses every other clause with a located
diagnostic.

This refines [FR-002](../FR-002-tristate-proptest.md) for bound numeric and state-scalar clauses
(agent-ix/quire-contract-codegen#3 under epic agent-ix/quire-spec-language#83). The supported clause
set is the bounded-integer comparison grammar that [FR-001](../FR-001-deterministic-oracles.md)
FR-001-AC-8 adds (agent-ix/quire-contract-codegen#4). SL lowers a model field such as
ConfigVersion's `versionNumber` (0..=1000) into a `ValueDeclaration` whose `SymbolName` is a
deterministic field alias
([quire-spec-language FR-034](ix://agent-ix/quire-spec-language/FR-034)) and whose type is an
integer declaration ([quire-contract-ir FR-013](ix://agent-ix/quire-contract-ir/FR-013)).

## Terms

- **Domain**: the inclusive range `minimum..=maximum` of a read's integer declaration. It is not the
  IR's `IntegerType.domain` field, which quire-contract-ir FR-013 calls the signed or unsigned
  representation.
- **Read**: one value-reference operand, identified by its declaration `SymbolName` and its
  observation (`current`, `pre`, or `post`, per quire-contract-ir FR-014).
- **Primary read**: the left operand when it is a read; otherwise the right operand.
- **Partner read**: the other operand when both operands are reads.

## Inputs

- A public `quire_contract_ir::BoundPackage` and one full `ClauseRef` naming a clause in it.
- The clause's `quire_contract_ir::ClauseKind` (quire-contract-ir FR-012), declaration
  environment, and typed expression (quire-contract-ir FR-014).
- The caller's attestation context.

## Outputs

- A domain census listing, for each read: declaration name, declaration kind (`Input`/`State`),
  observation, and inclusive minimum and maximum.
- Either an admitted relation handed to [FR-009](./FR-009-constructive-correlated-populations.md) and
  [FR-010](./FR-010-domain-boundary-campaigns.md), or a `StrategyDiagnostic`.

## Behavior

- The generator shall take each read's domain from its `ValueDeclaration`'s `IntegerType` minimum and
  maximum.
- The generator shall not accept a caller-supplied range for a bound clause.
- The generator shall submit the clause to the same admission that `generate_bound_oracles` applies
  before applying any strategy rule.
- If bound oracle admission refuses the clause, then the generator shall refuse with
  `UnsupportedClause`, preserving the oracle's lower-level code, terminal state, and exact IR source
  span, except that an explicitly recognized unsupported comparison operand described below shall be
  normalized to `UnsupportedRelation` at that operand's locus.
- If the clause kind is not `Precondition`, `Postcondition`, or `Invariant`, then the generator shall
  refuse with `UnsupportedClauseKind`.
- The generator shall admit an oracle-admitted clause only when its root is exactly one `Compare` node
  whose operands are each an integer read or an `IntegerLiteral`, with at least one read operand.
- If a `Compare` operand is a Boolean or `Text` read, a Boolean or `Text` literal, or any node other
  than an integer read or `IntegerLiteral`, then the generator shall refuse with
  `UnsupportedRelation`.
- If both `Compare` operands are literals, then the generator shall refuse with
  `UnsupportedRelation`.
- If an oracle-admitted clause root is a Boolean literal, reference, negation, or connective, then the
  generator shall refuse with `UnsupportedRelation`, because constructive generation over Boolean
  combinations of comparisons is not specified in this slice.
- If both operands are the same read, then the generator shall refuse with `UnsupportedRelation`.
- If the clause reads one declaration through `Current` and also through `Pre` or `Post`, then the
  generator shall refuse with `UnsupportedRelation`, because this slice does not define how those
  observations alias at the execution points quire-contract-ir FR-014 permits them together.
- If the `ClauseRef` does not name a clause in the package, then the generator shall refuse with
  `UnknownClause` and terminal state `invalid-input`.
- The generator shall report `UnsupportedClauseKind` and `UnsupportedRelation` with terminal state
  `unsupported`.
- The generator shall check refusals in this order and report only the first that applies:
  `UnknownClause`, `UnsupportedClause`, `UnsupportedClauseKind`, `UnsupportedRelation`.
- Every refusal diagnostic shall carry the full `ClauseRef`.
- An `UnsupportedClause` refusal shall carry the span the oracle diagnostic carries, as
  interface-001 `diagnostics.fields` requires it for expression failures after
  agent-ix/quire-contract-codegen#4.
- An `UnsupportedRelation` refusal shall carry the exact IR `SourceSpan` of the first offending node
  in authored preorder, matching the codegen#4 refusal locus rule.
- `UnknownClause`, `UnsupportedClauseKind`, `EmptyPopulation`, and `UnsupportedCampaignConstraint`
  are not expression failures and shall carry no span.
- The generator shall report every new refusal as a variant of the existing `StrategyErrorCode` in
  `StrategyDiagnostic`.
- The generator shall treat the two reads of one comparison as sharing one domain, because
  quire-contract-ir FR-014 makes ordering and equality operands compatible only when their complete
  integer types are equal.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-008-CON-1 | The generator SHALL read domain bounds only through the public `quire_contract_ir` API at the pinned revision, with no private decoder or local copy of the IR wire shape. The pin is `04eb6f8`; quire-spec-language pins `690bde7`, and quire-contract-ir FR-012 through FR-015 and FR-023 are unchanged between them. | Interface | Test (TC-017) |
| FR-008-CON-2 | The generator SHALL decide strategy admission before rendering strategy, population, census, or runner source; bound-oracle admission renders only its private candidate while applying the shared admission path, and a refusal returns no source artifact or partial bundle. | Integrity | Test (TC-017) |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-008-AC-1 | Given ConfigVersion `VersionUnchanged`, in a fixture package shaped as quire-spec-language FR-034 projects it, the census lists the `versionNumber` field's declaration, by its SL field-alias `SymbolName`, as a `State` declaration read at `Post` (primary) and `Pre` (partner) with inclusive domain 0..=1000 taken from its integer declaration. | Test (TC-017) |
| FR-008-AC-2 | Given SL's `integer-healthy` invariant `amount < 7`, the census lists `amount` with domain 0..=1000, and the relation is admitted with operator `Less`, `amount` as primary read, and literal 7. | Test (TC-017) |
| FR-008-AC-3 | A clause that bound oracle generation refuses (arithmetic, negation, or a definedness obligation) is refused with `UnsupportedClause` carrying the oracle's code and span; a Boolean connective over two comparisons, a Boolean equality `flag == true`, a literal-only comparison, an `x == x` comparison, and a `Current`/`Pre` mix of one declaration are each refused with `UnsupportedRelation`. | Test (TC-017) |
| FR-008-AC-4 | An `Assertion` or `Case` clause is refused with `UnsupportedClauseKind`, and an absent `ClauseRef` is refused with `UnknownClause` and `invalid-input`; every refusal carries the full `ClauseRef`, and neither carries a span. | Test (TC-017) |
| FR-008-AC-5 | A clause that triggers several refusals reports only the first in the stated order, and no refusal leaves any artifact. | Test (TC-017) |

## Dependencies

- **Upstream**: [StR-001](../../stakeholder/StR-001-traceable-generation.md);
  [FR-001](../FR-001-deterministic-oracles.md) FR-001-AC-8 bounded-integer oracle grammar
  (agent-ix/quire-contract-codegen#4, merged by PR #29 at `e0be330`);
  [quire-contract-ir FR-012, FR-013, FR-014, FR-015](ix://agent-ix/quire-contract-ir/FR-014) at
  `04eb6f8`; [quire-spec-language FR-034](ix://agent-ix/quire-spec-language/FR-034) field aliases;
  [FR-002](../FR-002-tristate-proptest.md).
- **Downstream**: [FR-009](./FR-009-constructive-correlated-populations.md),
  [FR-010](./FR-010-domain-boundary-campaigns.md),
  [TC-017](../../test/strategies/TC-017-bound-domain-admission.md).
