---
id: FR-035
title: "Generate and prove caller TextPayload admission"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: references
  - target: ix://agent-ix/quire-contract-codegen/ADR-006
    type: references
---
# FR-035: Generate and prove caller TextPayload admission

## Description

When a caller requests runtime Text admission, the code generator shall emit an
oracle and a separately bounded Kani obligation for a caller-owned
`TextPayload` and declared `TextType`. This is a caller-ingress claim, not a
claim about an expression node in an admitted `CheckedPackageV2`. The oracle
calls Contract Runtime's own `exact::admit_text` API and
returns its outcome and metering unchanged. Its evidence identifies the
caller-declared profile, inclusive length bounds, payload class and proof
configuration, without `ir_confirmed` provenance, a checked node id or a QSL
term identity.

[FR-014](./FR-014-exact-scalar-oracles.md) continues to own generated oracles
for admitted, catalogued text comparison nodes. A comparison's existing
agreement evidence does not establish that its operands were admitted by this
route. `quire.op.numeric.convert` has an `exact_numeric` operand and is never
the identity of caller Text admission. [ADR-006](../../decisions/ADR-006-caller-text-admission-boundary.md)
records the architecture boundary.

## Inputs

- A caller-supplied `TextType::new(min, max, profile)` with nonempty inclusive
  bounds and one of the six `TextProfile` values.
- A runtime `TextPayload` made by the public `TextPayload::from_utf8(&[u8])`
  constructor, and a caller-supplied `Meter`. Invalid UTF-8 bytes are refused
  by that constructor before a `TextPayload` exists or admission is called.
- For proof, a finite symbolic source class: zero, one or two Unicode scalars
  chosen independently from `e`, U+0301 COMBINING ACUTE ACCENT, U+00E9 LATIN
  SMALL LETTER E WITH ACUTE, and U+1F600 GRINNING FACE. The harness encodes
  the selected scalars as UTF-8 (at most eight bytes), calls the public
  `from_utf8` constructor, then passes the resulting payload to the emitted
  admission oracle. The two selected indices and the length are symbolic;
  this is a 21-member class, not one fixed literal.

## Outputs

- A deterministic generated admission oracle and its caller-ingress claim
  identity, separate from FR-014's per-node claim map. The identity accompanies
  the generated harness through [FR-017](../../kani/functional/FR-017-kani-execution-evidence.md)
  execution evidence, so a Kani outcome identifies the profile, bounds and
  finite payload class it actually concerns.
- For each supported profile and declared bounded type in the proof request,
  one Kani harness whose identity records that profile, type bounds, the exact
  symbolic source class, solver/options and its non-vacuity cover.
- A typed refusal with no generated harness when a requested payload class,
  bound or profile has no finite supported encoding. Such a refusal is never
  reported as a proof.

## Behavior

- When given a valid `TextPayload` and `TextType`, the emitted oracle shall
  call `rt::admit_text(&payload, &text_type, meter)` and return its `Outcome<Text>`
  unchanged. It shall neither pre-normalize the payload nor substitute an
  unmetered local bound check.
- When `TextPayload::from_utf8` rejects invalid bytes, the caller-ingress
  route shall expose that constructor refusal before it can call admission;
  the meter shall not be charged by admission. The generated oracle takes a
  `TextPayload`, so this refusal is not an `admit_text` outcome.
- When an admitted payload violates the declared profile length after any
  required normalization, the oracle shall return the runtime's refusal at
  its actual charge point; it shall not report completion or charge
  `text.result-retain`. An injected denial of each reached charge point shall
  preserve the runtime's stop and consumed counters.
- When generating a Kani obligation, the generator shall draw a symbolic
  length in `0..=2` and symbolic alphabet indices for both positions, encode
  the selected scalars as UTF-8, and construct the payload through
  `TextPayload::from_utf8`. It shall compare the emitted oracle with an
  independent direct call to `rt::admit_text` on that payload and type,
  including completion/refusal, retained text and provenance on completion,
  admitted charges and consumed counters. It shall have exactly one cover
  after the assertions, reached by a valid constructed payload.
- The supported proof class shall include all six profiles, a permissive type
  `Text[0,8]` and a restrictive type `Text[1,1]`. The proof claim shall name
  the exact class and type. No claim shall imply proof for arbitrary UTF-8
  bytes, more than two scalars, source-literal provenance, or a QSL term.
- If the caller requests a proof for a payload class or profile/bound encoding
  outside that supported class, then the generator shall refuse it with a
  typed reason and emit no proof artifact.
- When testing invalid UTF-8 constructor input, the test shall exercise the
  caller boundary; it shall not create a symbolic `TextPayload` by bypassing
  private fields.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-035-AC-1 | A generated caller-ingress oracle accepts `&TextPayload`, the declared `TextType` and `&mut Meter`, calls `rt::admit_text` once, and returns the runtime outcome, retained text, original runtime provenance, charge sequence and consumed counters unchanged for all six profiles over representative empty, ASCII, combining, composed and supplementary payloads. | Test |
| FR-035-AC-2 | Invalid UTF-8 bytes make `TextPayload::from_utf8` return `InvalidUtf8` before admission and leave the admission meter unchanged; `admit_text` itself is never claimed to accept those bytes. | Test |
| FR-035-AC-3 | For each profile, `Text[1,1]` admits or refuses according to the profile length of the retained sequence, including `e` plus U+0301 under NFC and NFD; a length refusal occurs before `text.result-retain`, and denial of each reached charge point yields the direct runtime stop, charges and consumed counters. | Test |
| FR-035-AC-4 | For each of the six profiles and both `Text[0,8]` and `Text[1,1]`, a real Kani harness symbolically ranges over all 21 sequences of zero to two scalars from the four-scalar alphabet, constructs each `TextPayload` by `from_utf8`, compares the generated admission oracle with a separate direct runtime call including outcome and metering, and ends with one satisfied cover after its assertions. The proof identity names this finite class and its type. | Test |
| FR-035-AC-5 | In a copy of the generated oracle with its admission call or declared `TextType` altered so the oracle accepts a payload the direct runtime call refuses, real Kani falsifies the admission agreement assertion with concrete playback; restoring the call verifies over the same symbolic class. | Test |
| FR-035-AC-6 | A request outside the declared finite proof class has a typed unsupported or requires-bound disposition and no harness, and no caller-ingress claim is marked `ir_confirmed` or assigned a checked-package node id. The six attempted text-operand `numeric.convert` nodes remain Contract IR refusals and do not count as passing caller-admission coverage. | Test |

## Dependencies

- **Upstream**: Contract Runtime's own `exact` text and accounting API, which
  defines and exports `admit_text`, `TextPayload`, `TextType` and `Meter`.
- **Execution**: [FR-017](../../kani/functional/FR-017-kani-execution-evidence.md)
  classifies and retains evidence for the generated caller harness.
- **Related**: [FR-014](./FR-014-exact-scalar-oracles.md) for checked-package
  text comparison. QSL-694 explores text-to-number parsing inside a term; it
  does not gate this caller-ingress operation.
- **Verification**: [TC-050](../matrix/TC-050-caller-text-admission.md).
