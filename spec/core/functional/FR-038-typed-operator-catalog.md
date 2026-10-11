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

## Behavior

- The typed source shall cover the identities the V2 exact-scalar oracle can
  currently derive, including its family-specific rounding and comparison
  distinctions. The scalar Kani projection shall select only integer add,
  subtract, multiply and negate. The state/frame Kani projection shall select
  only integer `lt`, `le`, `gt`, `ge`, `eq` and `ne`.
- For a shared identity, the V2 consumers shall take identity, family, arity,
  operand order and applicable rendering selection from the same typed entry.
  The oracle shall continue to check the checked node's own identity and mode
  against its derived descriptor before rendering. A missing, mismatched or
  unsupported combination shall produce the consumer's existing refusal or
  unsupported outcome, never a default operation.
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
| FR-038-AC-1 | A source inventory enumerates every identity derivable by the V2 exact-scalar oracle and every identity selected by V2 scalar Kani and state/frame Kani. For the four shared integer arithmetic identities and the four shared integer ordering identities, the consumers resolve the same typed identity, family, arity and ordered operands; integer `eq` and `ne` are available only to the state/frame selection until another V2 consumer is specified. No second string-to-operator match for a shared identity remains in those consumers. | Inspection |
| FR-038-AC-2 | The typed source distinguishes integer, rational, decimal, IEEE, text, enum and quantity operations and their applicable result and mode forms. A descriptor/checked-node mismatch in identity or required mode, an unknown identity, an unsupported family, and an invalid operand shape cannot select a different operation or silently default; each follows the owning consumer's typed refusal or unsupported path. | Test |
| FR-038-AC-3 | Focused V2 oracle, scalar Kani and state/frame tests distinguish the wrong identity, family, operand arity or order, comparison symbol and applicable runtime callee from the correct rendering. The existing admission and generated-artifact behavior for supported identities remains equivalent. | Test |
| FR-038-AC-4 | The four scalar Kani harnesses still compare their embedded oracle result with a separately rendered native `i128` arithmetic expression. A mutation of the production oracle's selected operation or callee makes the real Kani proof fail under an in-domain non-vacuous control; routing the native expression through that callee fails review. | Test |
| FR-038-AC-5 | The V1 `strategy/bound` relation evaluator retains its own host comparison semantics until the V2 strategy replacement specified by [AD-004](../../assurance/AD-004-cg-crate-layout.md) and [FR-008](../../strategy/functional/FR-008-bound-domain-strategy-admission.md). The hand-built bounded corpus is excluded from this catalog migration and its duplicate `checked_method` mapping is removed by [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) and AD-004 step 4g retirement, subject to that step's proof controls. Neither path gains a compatibility forwarding layer. | Inspection |

## Dependencies

Contract IR's checked reader and the upstream verification catalog own admitted
operation identities. [AD-004](../../assurance/AD-004-cg-crate-layout.md) places
the CG typed source with V2 node access, or has CG use an upstream typed decoder
if one becomes available. The V2 strategy comparison accessor and its admission
gate remain [FR-008](../../strategy/functional/FR-008-bound-domain-strategy-admission.md)
work. Corpus retirement remains [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
work and is not a prerequisite for the enduring V2 consumer catalog.
