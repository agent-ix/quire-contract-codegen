---
id: SR-12800
title: "IR-818 quantity-conversion specification code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR #415 own eight Markdown paths against 7f64cfb; ticket IR-818"
review_set: subset
---

## Summary

Ticket: IR-818. Frozen candidate 858e4a2d95d4cfa477b30f1235c7e6fcff2fec5f. This supplemental method covers code-review's language-independent duplication, integrity, specification/code boundary and evidence checks. The own payload is identical to the originally reviewed specification patch. No Rust, dependencies, tests or CI changes occur. No applicable AssuranceProfile exists under spec/.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for the scoped code-review method. The specification calls the existing conversion dependency through the current AD-004 owner; it introduces no local runtime implementation, copied API catalog, snapshot or additional dependency path. No coverage threshold, gate, warning policy or source behavior is weakened. The target's type-node provenance and call-time unit responsibilities are explicit. Refusal precedence and unreachable scalar-target/non-exact fixtures are separated from admitted compound-target fixtures.

This is a specification delivery. Existing source still implements the prior named conversion refusal; the PR body and TC-024 matrix explicitly identify conversion implementation as planned in the IR-818 code slice. This review grants no implementation or runtime agreement credit. `quire matrix --strict --format json` exits 1: 821 criteria comprise 445 tagged, 317 untagged, 14 ignored-only and 45 method-without-symbol. New AC-48 through AC-51 are untagged as expected for the planned slice. This is not a whole-repository coverage PASS, and no gap-analysis method is asserted.

Live Linear records were parsed with a YAML parser and compared with the prior verified receipt inventory: SR-12082's five, SR-12083's five and SR-12084's one findings all have latest fixed outcomes with nonempty after-excerpts. The final specification contains the corresponding entry-point outcomes, exact-only refusal ordering, omitted scalar-result vectors, correct runtime references, target-type provenance and admitted-fixture boundaries. Existing artifact text is preserved verbatim.

## Method selection

The existing base, integrity and EARS analyses are retained under SR-12082, SR-12083 and SR-12084; unchanged payload needs no repeat semantic cycle. No production code changed, so gap-analysis is not applicable. No Cargo.toml or Rust file changed, so rust-review and Cargo checks are not applicable. There are no added relationship edges or domain-object artifacts requiring dependency or object analysis. No Cargo/Kani, branch edit or push was performed. `git diff --check` passes.

## Scope

The following scope is shared verbatim with the private tracker comment. Excerpts are exact leading portions of long units, limited to 600 characters. All changed files and changed criteria were examined; implementation files provide context only. No test-to-criterion binding is credited by this method.

```yaml
scope:
- id: FR-014-AC-2
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Integer (add, subtract, multiply, negate, truncating/floor/Euclidean division,
    modulo), rational, ordering, decimal, IEEE arithmetic/comparison/width conversion,
    text comparison, enum comparison, quantity arithmetic/comparison and exact quantity
    conversion (FR-014-AC-48) descriptors for admitted checked-package expression
    nodes each generate a function calling the matching runtime `exact` operator.
    Entry-point dispositions for a `quire.op.quantity.convert` node: derivation and
    generation produce the oracle (FR-014-AC-48); the routed path, `negotiate_kani_obligations`
    and capability settlement '
- id: FR-014-AC-5
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Every claim-map entry carries the node id, IR id, package id, source map,
    claims, bounds and operation identity of its item, and, for a `quire.op.quantity.convert`
    item, the `type_argument` type node id (FR-014-AC-48).
- id: FR-014-AC-6
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Executing generated oracles on the QSpec TC-185, TC-186, TC-192 and TC-193
    vectors, and on these QSpec TC-187 vectors, yields outcomes, charges and consumed
    counters equal to direct runtime execution: U02 (dimension normalization through
    a product and a quotient, and a zero power), U03 (the dimension mismatch), U05
    (arithmetic and a power on `degC` and `degF`, refused `ill_typed`), U09 (a base
    dimension of the same declaration and shape as `L` under a different node key,
    refused `ill_typed`), U12 (compound-unit products, quotients and powers, with
    the `s` family), U13 and U21 (powers), U14 (th'
- id: FR-014-AC-18
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: '`derive_exact_scalar_items` returns, per node id in input order, the descriptor
    determined by the node''s `operation.identity`, `operation.mode`, `operation.laws`,
    operand forms and the one bound of the needed form on its result type; a node
    that is not an application, has no identity, names an identity outside the derivable
    set, has operand forms, a law or a mode that select no parameter, is refused with
    the matching `ClaimDerivationRefusal` and no descriptor; a node that is absent,
    does not lower or has a refused bound (missing, repeated or unreadable) is refused
    with the `ExactScalarRefusal`'
- id: FR-014-AC-48
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A `quire.op.quantity.convert` node whose `type_argument` names a quantity
    unit and whose `operation.mode` is `exact` derives and generates an `ir_confirmed`
    oracle, described by the public operation `quantity_conversion` (`ExactScalarOperation::QuantityConversion`,
    a unit-free variant: the target unit is a call-time argument `unit: &QuantityUnit`,
    and the generator does not check it against the `type_argument`, which the node
    carries only as a type node, Contract IR FR-038-AC-84 and FR-038-AC-213), with
    no checked bound, and the claim-map entry records the `type_argument` type node
    id (FR-014-'
- id: FR-014-AC-49
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'When the runtime refuses a `quire.op.quantity.convert` with `IllTyped`
    before any charge, the oracle stops as `ill_typed` with zero admitted charges
    and no change to any consumed counter, equal to direct `convert_quantity` execution
    (Contract Runtime FR-007-AC-21): `1 m` to `s` (U03-style `m` to `s`), U09 (a base
    dimension of the same declaration and shape as `L` under a different node key),
    `1 N_m` to `J` (U22) and compound `4 m^2` to `m`. An affine source or target converts
    and is not refused as `AffineUnitArithmetic`; only arithmetic on an affine unit
    is refused (FR-014-AC-6, U05).'
- id: FR-014-AC-50
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'No oracle generated for `quire.op.quantity.convert` returns a scalar:
    its result is always a `Quantity` in the supplied target unit, and a conversion
    node whose `operation.mode` is not `exact` is refused with `ClaimDerivationRefusal::ModeNotDerivable`,
    checked first, so a scalar `type_argument` with a non-`exact` mode is `ModeNotDerivable`;
    one whose mode is `exact` and whose `type_argument` is a scalar type (`Decimal`
    or `Integer`, QSpec TC-187 U07, U08, U16, U17, U18 and U29) or a compound unit
    (U25''s first step; IR-831) is refused with `OperandFormsNotDerivable`, each with
    no descriptor, no'
- id: FR-014-AC-51
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A selected `quire.op.quantity.convert` claim settles through capability
    settlement like any other quantity operation, as `OperationNotRendered` (Planned,
    IR-269), because the Kani scalar renderer builds no quantity harness; the conversion
    is credited by its oracle only and is uncredited for Kani. The settlement is not
    a refusal by name, and a quantity harness is IR-269's work, not this requirement's.
    The owning settlement row is TC-030's Quantity scalar row, which names `quire.op.quantity.convert`
    and cites this criterion.
- id: FR-021-AC-21
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'A scalar `Negate` body is refused as `UnsupportedOperator` with no schedule
    root, whether or not it reaches a `quire.op.quantity.convert` node (a conversion
    is not refused by name, because FR-021 has no quantity body, operand or argument
    support: the generator refuses a body that is one as `UnsupportedOperator` and
    the schedules as `ScheduleRefusal::UnsupportedOperation`; an earlier check that
    fires first is the accepted result: `Unsupported` for a direct unsupported form
    of the root, `NotExpression` and the other root checks, `UnsupportedOperandType`
    for a quantity-typed parameter or result, '
- id: FR-014-Behavior
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: "- If a requested node's `operation.identity` is `quire.op.quantity.convert`,\n\
    \  then the generator shall derive or accept a descriptor for it whose oracle\n\
    \  calls the Contract Runtime `convert_quantity` with `QuantityTarget::Exact`\n\
    \  (through the crate this repository's `Cargo.toml` names for scalar conversion\n\
    \  at its current AD-004 step, and through no second path), takes the target unit\n\
    \  as a call-time argument `unit: &QuantityUnit` of the generated oracle (as the\n\
    \  pre-IR-268 conversion oracle did), because the admitted node carries the target\n\
    \  only as a type node, `operation.member`'s `t"
- id: TC-024
  path: spec/oracle/matrix/TC-024-exact-scalar-oracles.md
  role: examined
  excerpt: "   (each carries the `exact` mode its unit type pins) and check each derives\
    \ an\n   `ir_confirmed` descriptor and generates an oracle that calls `convert_quantity`\n\
    \   with `QuantityTarget::Exact` and returns a `Quantity` in the unit passed as\
    \ the\n   call-time argument `unit: &QuantityUnit`, never a scalar and never checked\n\
    \   against the node's `type_argument` (FR-014-AC-48, FR-014-AC-50). Run the conversion\
    \ vectors\n   (U01, U04 `degC` to `K`, U09b, U10, U15, U22, U25's second step\
    \ and U26) and the `ill_typed` vectors (`1 m` to `s`, U09, `1 N_m` to `J`, compound\
    \ `4 m^2` to `m`),\n   and check ea"
- id: oracle-matrix-conversion
  path: spec/oracle/matrix/tests.md
  role: examined
  excerpt: '| FR-014 | FR-014-AC-48 through FR-014-AC-51 | TC-024 | 🚧 Planned (IR-818
    code slice); exact quantity conversion oracle over `convert_quantity` with `QuantityTarget::Exact`
    (Contract Runtime FR-007-AC-20 to AC-25), `ill_typed` with zero charges (FR-007-AC-21),
    no scalar result, and `OperationNotRendered` settlement (Planned, IR-269; owning
    row TC-030''s Quantity scalar row) |

    | FR-014 | FR-014-AC-4 | TC-024 | ⚠️ Partially covered; byte identity across repeated
    runs and request permutations and the claim-map order are asserted by regeneration;
    the criterion''s remaining clause has no test |

    | FR-'
- id: TC-030-quantity-row
  path: spec/routed/matrix/TC-030-capability-settlement.md
  role: examined
  excerpt: '| Quantity scalar `expression` claim, IR-confirmed generated claim, including
    identity `quire.op.quantity.convert` | Planned (IR-269) | Quantity scalar operation
    forms; the conversion''s settlement is FR-014-AC-51. | `OperationNotRendered`
    (`classify_claim` maps `ScalarLoweringRefusal::NoRenderer`). |

    | Record/tuple equality claim with a derivable IR-confirmed operation and bounded
    shadow shape | Planned (IR-264, IR-241) | FR-015-AC-69 through AC-76. | `OperationNotRendered`
    (`classify_claim` maps `ScalarLoweringRefusal::NoRenderer` for the non-integer
    renderer). |

    | Record/tuple equality claim'
- id: SR-12082
  path: reviews/ir-818-spec-review-base.md
  role: examined
  excerpt: '## Summary


    Ticket: IR-818. PR agent-ix/quire-contract-codegen#415 at 5abb404. Base checklist:
    ID format, AC

    testability, error paths and coverage for the reworded quantity-conversion requirement.
    The new AC

    ids FR-014-AC-48 to AC-51 are well-formed, unique and contiguous after AC-47.
    There are no hash or pin uses.

    Two completeness gaps leave a disposition unstated, and one AC is missing an RT
    vector.


    ## Findings


    | ID | Severity | Summary | Refs |

    | --- | --- | --- | --- |

    | FND-001 | medium | The Behavior bullet says the routed path, `negotiate_kani_obligations`,
    the function generator and '
- id: SR-12083
  path: reviews/ir-818-spec-review-integrity.md
  role: examined
  excerpt: '## Summary


    Ticket: IR-818. PR agent-ix/quire-contract-codegen#415, frozen head 5abb404, diff
    `origin/main...HEAD`

    (base 0e0f92e). Consistency check of the reworded FR-014 conversion text, the
    new FR-014-AC-48..51,

    FR-021-AC-21, TC-024 step 6 and the oracle test-matrix rows against each other,
    against Contract

    Runtime FR-007-AC-20..25 / TC-201 at RT 561691e, against the RT code at CG''s
    locked RT 3cc88c4, and

    against QSpec TC-187 (quire-specification origin/main 9a1d3cf).


    ## Findings


    | ID | Severity | Summary | Refs |

    | --- | --- | --- | --- |

    | FND-001 | high | FR-014-AC-6 puts U07, U08, U16'
- id: SR-12084
  path: reviews/ir-818-spec-review-ears-conformance.md
  role: examined
  excerpt: '## Summary


    Ticket: IR-818. PR agent-ix/quire-contract-codegen#415 at 5abb404. EARS check
    of the rewritten

    FR-014 Behavior bullet for `quire.op.quantity.convert`. The two main clauses are
    well-formed

    unwanted-behaviour statements: "If ... is `quire.op.quantity.convert`, then the
    generator shall ..."

    and "If the runtime returns `IllTyped` ..., then the oracle shall stop as `ill_typed`
    ...". The

    trailing sentences of the same bullet are not EARS.


    ## Findings


    | ID | Severity | Summary | Refs |

    | --- | --- | --- | --- |

    | FND-001 | low | The bullet ends with three declaratives that have no `shal'
- id: src/oracle/scalar/mod.rs
  path: src/oracle/scalar/mod.rs
  role: context_only
  excerpt: "pub const QUANTITY_CONVERSION_CAPABILITY: &str = \"quantity conversion\
    \ is not supported: the \\\n     runtime converts a quantity only to a scalar\
    \ target and has no conversion whose result is a \\\n     quantity (tracked as\
    \ IR-818)\";\n\n/// The operation identity of a quantity conversion.\npub(crate)\
    \ const QUANTITY_CONVERT_IDENTITY: &str = \"quire.op.quantity.convert\";\n\n///\
    \ The refusal for a node whose operation identity is a quantity conversion, whichever\
    \ entry\n/// point received it; `None` for every other identity.\nfn quantity_conversion_refusal(identity:\
    \ &str) -> Option<ExactScalarRefusal> {\n    (id"
- id: src/oracle/function/schedule.rs
  path: src/oracle/function/schedule.rs
  role: context_only
  excerpt: "QuantityConversionUnsupported {\n        /// The conversion body node.\n\
    \        body: CheckedNodeId,\n        /// The capability that is absent.\n  \
    \      capability: &'static str,\n    },\n    /// The schedule's ordinal or slot\
    \ counter cannot be represented.\n    CounterOverflow,\n    /// The graph map\
    \ key differs from the node's retained identity.\n    NodeIdentityMismatch {\n\
    \        /// Identity used as the graph-map key.\n        key: CheckedNodeId,\n\
    \        /// Identity retained by the graph node.\n        stored: CheckedNodeId,\n\
    \    },\n    /// A V2 body does not match the flat function schedule grammar"
```
