---
id: FR-008
title: "Admit checked V2 clauses for numeric strategy generation"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
  - target: ix://agent-ix/quire-contract-ir/FR-038
    type: depends_on
  - target: ix://agent-ix/quire-spec-language/FR-034
    type: references
---
# FR-008: Admit checked V2 clauses for numeric strategy generation

## Description

For a request selecting one claim of an admitted `CheckedPackageV2`, the generator SHALL use Contract
IR's public clause-context integer-comparison operand accessor (IR FR-038-AC-202 through AC-209) to
admit a direct comparison and derive its exact domains. The generator SHALL return one complete
strategy bundle or one typed, located refusal; it SHALL NOT generate a partial bundle. This is the
V2 input contract for the constructive population and campaign behavior of FR-009 through FR-012.
The IR accessor and its public consumer test are planned under IR-703. CG also needs a V2
comparison oracle for all six admitted operators; the current `generate_exact_scalar_oracles`
operation set supports integer ordering but has no integer equality/inequality descriptor. Neither
that generator nor the existing V1 bound oracle discharges this planned comparison-oracle path.

## Terms

- **Selected claim**: the checked clause id and authentic `CheckedOccurrence` supplied with the
  request. The comparison node id alone does not identify a clause or an observation.
- **Read**: one accessor operand with typed `StateField` or `OperationInput` provenance, exact
  selected declaration identity, authored ordinal, observation and inclusive i128 endpoints.
- **Primary read**: operand zero when it is a read; otherwise operand one. The partner read is the
  other operand when both are reads. Neither definition reorders the comparison.
- **Domain**: a read's exact inclusive model-member or operation-parameter range. The generator's
  emitted case values remain i64; an i128 endpoint is not implicitly narrowed to i64.

## Inputs

- `&CheckedPackageV2`, one checked clause id, one authentic claim occurrence, and the existing
  population and campaign policy.
- The IR accessor's contextual comparison and positional operands. No caller-supplied range,
  V1 `BoundPackage`, `ClauseRef`, private body decoder or local wire schema is an input.

## Outputs

- A census of each read's typed declaration identity, `State` or `Input` kind, observation and
  exact inclusive domain, together with an admitted ordered relation for FR-009 and FR-010; or
- one `StrategyDiagnostic` with a stable refusal kind, selected clause and claim, IR structural
  condition/operand path, and reached child id when IR supplies one. An expression-child id does
  not imply a unique source occurrence; a separate source-map lookup may enumerate regions.

## Behavior

- The generator SHALL select one claim, call IR's clause-context accessor with that clause id and
  occurrence, and preserve its closed comparison operator and operand ordinals. It SHALL admit only
  a direct condition reference to binary integer.eq/ne/lt/le/gt/ge with one or two eligible reads.
  It SHALL preserve the authored left/right order for noncommutative comparisons.
- The generator SHALL map typed `StateField` provenance to `State` and typed `OperationInput` to
  `Input`, without inspecting a name to infer the kind. It SHALL preserve IR's `Current`, `Post`,
  and `Pre` observations. Direct state-field reads in invariant/precondition are `Current`, direct
  state-field reads in postcondition are `Post`, a postcondition `state.pre` state-field read is
  `Pre`, and an operation input is `Current`.
- The generator SHALL use the accessor's exact i128 bounds for each read and exact i128 literal.
  It SHALL check each lower endpoint, upper endpoint and literal against i64 before conversion or
  rendering. A value outside i64 SHALL refuse as `UnsupportedRelation` at its operand path, with
  no truncation, wrapping, clamping or widened generated domain.
- For two reads, the generator SHALL compare their complete inclusive ranges before constructing
  the one-domain relation. Unequal ranges SHALL refuse as `UnsupportedRelation` at operand one,
  even if both ranges fit i64. Equality of ranges is a CG admission rule, not a claimed IR V2
  static-type invariant. The same-read and mixed `Current` with `Pre`/`Post` exclusions remain.
- An eligible read/literal pair SHALL use the read's domain and keep the literal in its authored
  position. Two literals SHALL refuse. Alias reads, including
  `let s=pre(self) in s.versionNumber`, nested projections, result slots, unrelated parameters,
  arithmetic operands, Boolean connectives and noninteger operands SHALL refuse by IR's typed
  ineligibility or CG's `UnsupportedRelation`; no alias expansion is performed.
- The generator SHALL preflight the selected claim only. An IR accessor refusal SHALL map to a
  distinct `StrategyDiagnostic` refusal kind with the IR cause and structural locus retained.
  CG SHALL supply a V2 comparison oracle over the accessor's six closed operators and ordered
  operands for the selected claim before rendering the strategy runner. This comparison-oracle
  path is planned and SHALL preserve any typed selected-claim refusal and its locus. Another
  claim's refusal in the same package SHALL NOT poison an eligible selected claim. A failure
  that prevents construction of the selected claim's complete oracle SHALL fail its request
  without artifacts. Successful generation SHALL still be atomic for the selected request's
  entire bundle.
- Refusal precedence SHALL be: unknown selected clause, absent or inauthentic claim, accessor's
  remaining first-defect order (unsupported clause kind, missing comparison, operator, arity,
  child and operand defects), CG relation/domain refusal, then selected-comparison-oracle
  construction refusal, followed by rendering errors. The IR accessor owns the order within its
  own checks. Each request reports its first refusal only.
- A postcondition directly authored as `self.versionNumber = pre(self.versionNumber)` SHALL be
  eligible when QSL emits and IR admits the ConfigVersion V2 package and both reads have the same
  0..=1000 range and QSL lowers `=` to `quire.op.integer.eq`; the left `Post` and right `Pre`
  entries share the record.project id while their contextual observations remain distinct. A synthetic graph fixture SHALL NOT substitute for
  this QSL-produced positive control.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-008-CON-1 | The generator SHALL read V2 operand identity, provenance, observation, operator and bounds only through IR's public typed accessor. A private decoder or copied schema is forbidden. | Interface | Inspection |
| FR-008-CON-2 | Admission SHALL finish before any strategy, population, census or runner source is published; a refusal returns no artifact or partial bundle. | Integrity | Test |
| FR-008-CON-3 | The generator SHALL check each read endpoint for i64 representability before constructing the shared-domain relation. | Integrity | Test |
| FR-008-CON-4 | The generator SHALL check each comparison literal for i64 representability before conversion. | Integrity | Test |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-008-AC-1 | A fresh QSL-produced, IR-admitted ConfigVersion V2 postcondition with direct `self.versionNumber = pre(self.versionNumber)` yields two `State` census entries in authored order, `Post` then `Pre`, each 0..=1000, with the same project id, `quire.op.integer.eq`, and the selected clause and authentic claim retained. The let-bound alias form yields a typed operand refusal and no bundle. | Test |
| FR-008-AC-2 | An IR-admitted V2 `amount < 7` invariant with a typed current read of range 0..=1000 yields operator `Less`, primary `amount`, literal 7 and that inclusive census domain. Reversing the two authored operands preserves the reversed positions and corresponding operator semantics. | Test |
| FR-008-AC-3 | A valid selected comparison generates despite a comparison-oracle-refused sibling claim in the same package; requesting the refused claim returns its typed selected-claim cause and locus with no bundle. An oracle-construction failure for the selected claim returns no bundle. Each of integer.eq/ne/lt/le/gt/ge has an executable selected-claim comparison oracle whose result agrees with the ordered relation and generated tags. Arithmetic, alias, Boolean and literal-only shapes refuse at their own structural loci. | Test |
| FR-008-AC-4 | Unknown clause, absent claim, unsupported clause kind and malformed comparison report distinct typed causes in the stated first-defect order. Diagnostics carry selected clause and claim, structural path and reached child id when available, without claiming a unique child source region. | Test |
| FR-008-AC-5 | A request with several applicable defects returns the first only. Every refusal returns zero artifacts; a successful request returns the complete source, census and runner bundle. | Test |
| FR-008-AC-6 | Either read endpoint or a literal at i64::MIN - 1, i64::MAX + 1, i128::MIN or i128::MAX refuses `UnsupportedRelation` at that operand without narrowing. Two individually i64-representable reads with unequal ranges also refuse; equal ranges admit when other rules hold. | Test |

## Dependencies

- **Upstream**: [IR FR-038](ix://agent-ix/quire-contract-ir/FR-038) AC-202 through AC-209 (planned IR-703
  implementation), and [QSL FR-034](ix://agent-ix/quire-spec-language/FR-034).
- **Downstream**: [FR-009](./FR-009-constructive-correlated-populations.md),
  [FR-010](./FR-010-domain-boundary-campaigns.md),
  [TC-017](../matrix/TC-017-bound-domain-admission.md).
