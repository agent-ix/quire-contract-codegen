---
id: FR-038
title: "Use one typed operator catalog for admitted V2 generation"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-031
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
---
# FR-038: Use one typed operator catalog for admitted V2 generation

## Description

When a V2 oracle or Kani generator selects an operation from an IR-admitted
checked-package claim, Contract Codegen shall resolve its catalogued identity
through one internal, closed typed operator source. That source shall describe
the operation family, operand arity and order, result kind, and applicable
rendering metadata without becoming an authority for IR admission or inventing
an identity absent from the upstream verification catalog. Each consumer shall
select only the operations it can render and shall retain its existing typed
  refusal or unsupported disposition for an admitted but unrenderable operation.
  For caller-supplied exact-scalar descriptors, it shall preserve
  [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md)'s distinct
  generated, `caller_declared` and typed-refused outcomes.

## Behavior

- The typed source shall cover the identities implemented by the V2
  exact-scalar oracle, including its family-specific rounding and comparison
  distinctions. [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md)
  additionally specifies derivation of integer `eq/ne` and Boolean
  `and/or/not/implies/eq/ne`; these are not implemented by the current
  `ExactScalarOperation` set.
- When those FR-014 scalar variants are implemented, the scalar emitter shall
  select them through this same typed source.
- The scalar Kani projection shall select only integer add, subtract,
  multiply and negate. The state/frame Kani projection shall select only
  integer `lt`, `le`, `gt`, `ge`, `eq` and `ne`.
- For a shared identity, the V2 consumers shall take identity, family, arity,
  operand order and applicable rendering selection from the same typed entry.
  The oracle shall continue to check the checked node's own identity, law and
  mode against its descriptor to determine operation provenance. A valid
  caller-supplied descriptor with a present but disagreeing identity, law or
  mode shall still generate the descriptor-named oracle as `caller_declared`
  with a typed blocked item. A missing node identity shall retain its typed
  refusal. An unknown derivation identity, an invalid operand shape or an
  operation unsupported by a Kani projection shall retain that consumer's
  specified refusal or unsupported outcome, never a default operation.
- The scalar Kani harness shall retain its independent native `i128`
  calculation over symbolic `i64` inputs. Its expected result shall not be
  computed by the runtime callee used in the embedded production oracle.
- The catalog shall be internal to CG. Contract IR and the upstream
  verification catalog continue to own checked-package admission and
  operation vocabulary. CG's public operations and serialized artifacts shall
  keep their specified shape and outcome semantics.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-038-AC-1 | A source inventory compares every identity in [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md)'s specified derivable set with the implemented scalar, scalar Kani and state/frame selections, recording the unimplemented scalar integer `eq/ne` and Boolean variants as open FR-014 work. For the four integer arithmetic identities shared by scalar oracle and scalar Kani, and the four integer ordering identities shared by scalar oracle and state/frame Kani, the implemented consumers resolve the same typed identity, family, arity and ordered operands; no second string-to-operator match remains for those shared identities. State/frame's implemented integer `eq/ne` does not discharge FR-014's specified scalar `eq/ne`. | Inspection |
| FR-038-AC-2 | The typed source distinguishes implemented integer, rational, decimal, IEEE, text, enum and quantity operations and their applicable result and mode forms. When FR-014's specified Boolean scalar variants are implemented, they use distinct Boolean entries in this same source before the scalar emitter selects them. A valid caller-supplied scalar descriptor with a present but disagreeing node identity, law or mode still generates its descriptor-named oracle as `caller_declared` with a typed blocked item under FR-014-AC-12 and AC-13. A missing node identity retains its typed refusal; an underivable identity, unsupported Kani family or invalid operand shape cannot silently default to another operation. | Test |
| FR-038-AC-3 | Focused V2 oracle, scalar Kani and state/frame tests distinguish the wrong identity, family, operand arity or order, comparison symbol and applicable runtime callee from the correct rendering. With the correct identity and arity, a wrong fixed result form or wrong member-selected result target, including a conversion target, is refused under FR-014's result/parameter checks rather than rendered as the right operator with the wrong result. Existing admission and generated-artifact behavior for supported identities remains equivalent. | Test |
| FR-038-AC-4 | The four scalar Kani harnesses still compare their embedded oracle result with a separately rendered native `i128` arithmetic expression. A mutation of the production oracle's selected operation or callee makes the real Kani proof fail under an in-domain non-vacuous control; routing the native expression through that callee fails review. | Test |
| FR-038-AC-5 | The V1 `strategy/bound` relation evaluator remains outside this V2 catalog migration until the V2 strategy replacement specified by [AD-004](../../assurance/AD-004-cg-crate-layout.md) and [FR-008](../../strategy/functional/FR-008-bound-domain-strategy-admission.md). The hand-built bounded corpus is also excluded, and its duplicate `checked_method` mapping is removed only by [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) and AD-004 step 4g retirement after that step's proof controls pass. Neither path gains a compatibility forwarding layer. | Inspection |
| FR-038-AC-6 | While the V1 `strategy/bound` relation evaluator remains live, focused tests discriminate all six comparison operators and a primary read on the left versus on the right of a noncommutative comparison; its host evaluation preserves the relation as authored. | Test |

## Dependencies

Contract IR's checked reader and the upstream verification catalog own admitted
operation identities. [AD-004](../../assurance/AD-004-cg-crate-layout.md) places
the CG typed source with V2 node access, or has CG use an upstream typed decoder
if one becomes available. The V2 strategy comparison accessor and its admission
gate remain [FR-008](../../strategy/functional/FR-008-bound-domain-strategy-admission.md)
work. Corpus retirement remains [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
work and is not a prerequisite for the enduring V2 consumer catalog.
