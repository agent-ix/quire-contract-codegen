---
id: ADR-006
title: "Separate caller Text admission from checked expression oracles"
type: ADR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/ADR-001
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-035
    type: relates_to
---
# ADR-006: Separate caller Text admission from checked expression oracles

## Status

Proposed. The caller admission oracle and Kani route are unimplemented;
[FR-035](../oracle/functional/FR-035-caller-text-admission.md) and
[TC-050](../oracle/matrix/TC-050-caller-text-admission.md) are Planned, not
proof evidence.

## Context

[FR-014](../oracle/functional/FR-014-exact-scalar-oracles.md) binds every
generated scalar expression oracle to an admitted `CheckedPackageV2` node and
its catalogued operation identity. The current source also has a
`TextAdmission` descriptor and an emitted call to `rt::admit_text`, but its
derivation maps a text-result `quire.op.numeric.convert` node to that
descriptor. The catalogued conversion requires an `exact_numeric` operand;
Contract IR rejects each of the six attempted text-operand nodes before
generation. There is no QSL Text-admission expression to supply such a node.
By contrast, catalogued text comparison nodes are admitted and retain their
FR-014 oracle and runtime agreement evidence.

Contract Runtime's exact text API accepts `&TextPayload`, `&TextType` and
`&mut Meter` for admission. `TextPayload` has private fields; its
`from_utf8` constructor validates bytes before the runtime admission call,
while `from_source_literal` represents a separately decoded source literal.
CG's present symbolic argument ABI has only `bool` and `i64`, so it cannot
turn a checked node into a symbolic `TextPayload` proof. [ADR-001](./ADR-001-overlapping-generators-and-input-models.md)
Q3 currently says every generator has one `CheckedPackageV2` input path. That
statement is correct for checked-expression generation but cannot describe
caller ingress, where no term node exists.

## Decision

Scope ADR-001 Q3 to generators of checked-package expression and contract
claims. The caller Text-admission oracle and its proof form a separate,
explicitly caller-declared ingress route under FR-035. That route has no
checked node id, `ir_confirmed` provenance or QSL term identity. It does not
enter FR-015's per-obligation checked-package Kani claim model or FR-025's
Boolean/`i64` subject ABI; its own finite symbolic input and identity are
required by FR-035.

The generated admission oracle calls Contract Runtime `exact::admit_text` on
a publicly constructed `TextPayload`. The proof harness selects zero to two
scalars from `{e, U+0301, U+00E9, U+1F600}`, encodes them as at most eight
UTF-8 bytes and calls `TextPayload::from_utf8` before the oracle. The class
contains 21 valid runtime-provenance payloads. Kani compares the generated
call with a direct runtime call under the same `TextType` and equal fresh
meters, covers a valid payload after the assertions, and must falsify a
changed admission call. Native tests separately exercise malformed UTF-8 at
the constructor boundary and injected meter denials. The proof is explicitly
limited to that finite class and declared profile/bounds; source-literal
provenance, arbitrary UTF-8 strings and text-to-number parsing remain outside
this claim.

The unreachable `TextAdmission` mapping from `numeric.convert`, and its
checked-node emission/derivation path, shall be removed in the code follow-up.
The six refused nodes remain as negative boundary evidence. The admitted
text-comparison route remains under FR-014 and TC-024.

## Consequences

- FR-014-AC-2 names text comparison among its checked expression families;
  it no longer counts caller Text admission as a per-node oracle.
- FR-035 and TC-050 own the missing admission oracle and bounded proof, with
  Planned coverage until real runtime and Kani controls run.
- FR-015 and FR-025 continue to govern their admitted-package obligations;
  the caller proof has a distinct identity and unsupported-class refusal.
- QSL-694's text-to-number parsing exploration is related and does not block
  caller `TextPayload` admission.

## Alternatives Considered

- Make `numeric.convert` accept Text. Its catalogued operand is exact numeric,
  and neither QSL nor QSpec currently defines a Text admission term; this
  would give a caller boundary a false checked-expression identity.
- Count a fixed text literal or the six refused conversion nodes as proof.
  Neither ranges over caller payloads or exercises an admitted generated
  admission call under real Kani.
- Drop admission from the MVP claim entirely. That would leave the existing
  caller-ingress need unverified rather than assigning it a truthful owner.
