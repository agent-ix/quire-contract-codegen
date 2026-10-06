---
id: FR-033
title: "Bind composite equality parity to the original node and QSL settlement"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/AD-002
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-070
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-322
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-358
    type: references
  - target: ix://agent-ix/quire-specification/FR-181
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-013
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-014
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-021
    type: references
---
# FR-033: Bind composite equality parity to the original node and QSL settlement

## Description

When the driver submits a composite equality `bounded_shadow` item for settlement, CG shall
construct a QSL-owned node-selected equality-parity claim over the original proving package.
The proposition remains the independent equality verdict and admitted occurrence-pair count
owned by [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-71 and the native
refinement comparison owned by [FR-028](../../kani/functional/FR-028-bounded-proof-ceilings.md)
AC-15. Equality and inequality preserve the claimed operation; an inequality verdict is the
negation of equality with the same pair count. This is parity evidence about generated code,
never a source-predicate violation.

This is PLANNED IR-635 work, GATED for implementation on QSL-640's actual public API delivery.
The proposed QSL FR-070 canonical values, FR-322 node selector and FR-358 parity settlement are
upstream obligations, not assertions that the public facade implements them. A specification-only
upstream change does not release the code gate. At authoring, the published FR-358 selected-Boolean-function
predicate route does not meet this contract; its refinement boolean also cannot distinguish a
completed comparison from a resource ceiling. This requirement records the agreed future shape
without fabricating Rust signatures, QSL causes or a working replay.

## Inputs

- The actual composite claim node and operation, original checked/emitted package and original
  source/dependency byte provision needed for QSL recompilation.
- The generated harness identity, symbolic leaf bindings, operand declarations and literal
  values from the same proving context; harness bounds and retained original execution limits.
- The falsified run's selected assertion playback, or the verified run's SUCCESS count and
  independent native-refinement evidence. The retained shadow comparison outcome includes its
  equality/inequality result and occurrence-pair count.
- For falsified parity replay, the driver's actual typed native observation of the same proved
  generated artifact over the reconstructed operands and original limits, with its pair count.
  Verified settlement takes no native-observation field; it retains actual refinement evidence,
  including explicit absence for `not_run`, without an extra native probe or invented observation. A refusal, incomplete
  outcome or execution fault remains that observation, rather than a fabricated Boolean.
- The CG-minted QSL `ObligationIdentity` and canonical proof-content identity binding this actual
  node/operation/operand-domain claim, original limits and the proved artifact. AD-002 R-6/R-7,
  QSL ADR-013 O-09 and ADR-014 B-4 govern their authority.
- Configured public replay encoded-input-byte, value-occurrence and work limits admitted by the
  upstream contract. Capture limits for launcher output remain separate inputs and evidence.

## Outputs

- A typed QSL-owned parity request for this node and its operand values, or a typed setup refusal
  before QSL evaluation.
- A binding-checked QSL settlement/record for [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md),
  or a typed refusal with no settlement if the result belongs to another claim or a required
  upstream capability is unavailable. No fabricated terminal value or QSL catalog code crosses
  an unavailable seam.

## Behavior

- The builder shall select the claimed FR-322 node inside the original recompiled package and
  preserve the package/source identity and node membership checks of AD-002 R-7.
- The builder shall retain the composite equality operation, each operand's actual declared
  domain and each literal operand's singleton domain, without substituting enclosing function
  parameters, changing source bytes or manufacturing a Boolean predicate/function.
- The builder shall reconstruct operands from the persisted original-node leaf bindings owned
  by [FR-025](../../kani/functional/FR-025-generated-subject-abi.md), preserving declaration
  identities, member identities, slots, presence, arity, order and multiplicity.
- The builder shall render QSL FR-070 witness values with the authoritative QSpec FR-181 canonical
  JCS encoding through its owning dependency, without copying schemas or other repositories'
  artifacts. Its transcript escaping replaces percent, semicolon, less-than and greater-than
  with `%25`, `%3B`, `%3C` and `%3E` respectively, using the upstream grammar.
- The canonical route shall preserve Boolean, arbitrary exact integer, exact IEEE float bits,
  rational, decimal, text, enum, quantity underlying exact scalar and reference values; it shall
  preserve option, record, tuple, union, sequence, set, bag and ordered-set structure. Population
  is not a value and no map family is added by this requirement.
- The builder shall validate canonical spelling, typed declared shape and admission through the
  owning QSL contract, including declared/unit/reference identities, enum/union members,
  required and optional slot presence, tuple arity, sequence order, set distinctness, bag
  multiplicity and ordered-set order/distinctness. Reference values require no invented object
  environment or dereferenced value; absent required environment retains QSL's typed no-value
  or refusal reading. Quantity conversion preserves its exact scalar and unit without rounding.
- When arbitrary public caller input arrives, the builder shall guard its encoded byte length
  against the configured replay-input limit, counting actual encoded bytes rather than a
  decoded-size estimate. It shall enforce checked occurrence/work accounting during traversal.
- The canonical value lifecycle shall use explicit heap stacks for decode, clone, equality,
  redacted Debug and drop, so admitted nesting consumes no proportional native stack. It shall
  refuse byte/count/work exhaustion with its typed resource reason, without an arbitrary
  nesting-depth cap. FR-017's stream capture ceiling is not this public-input guard.
- For falsified replay, the binding shall require the driver's measured native observation of the
  same proved artifact and the retained shadow result/count, tied to the node, operands, original
  limits and canonical
  proved-content identity. Playback alone and a freshly regenerated correct artifact supply
  no native observation of mutated proved content.
- The converter shall validate the result/run-to-claim binding before reading QSL's closed typed
  discriminants and catalog codes. A changed node, operation, operand, domain, source/package,
  limits, content identity or result identity yields a typed refusal with no settlement.
- The driver shall call QSL's exact equality parity arm between CG's public builder and converter.
  CG supplies no local exact evaluator, synthetic predicate or canned replay verdict.
- For a falsified shadow, QSL shall compare its exact equality result/count on the admitted
  operands with the retained shadow comparison under the claimed operation. Divergence becomes
  `Failed` with CG's own `CgDefect` cause; agreement becomes QSL's named parity-agreement
  `Inconclusive`. Neither becomes `Refuted` or a reproduced source violation. The actual native
  observation remains bound evidence, including a native/shadow disagreement.
- For a verified shadow, CG shall pass the same node-selected claim, SUCCESS count, harness
  bounds and the closed refinement evidence projected from FR-028's strengths, with no native
  observation field or extra probe to fill `not_run`; QSL derives
  declared operand-bound keys and coverage itself from the recompiled claim. A missing key is
  uncovered; unknown/duplicate keys and bound-kind mismatches refuse. Literal domains are their
  singleton values, not the enclosing type's full range. ADR-021 TX3 owns tightened-bound
  coverage; TX2 names the separate bounded-shadow route.

The terminal decision and its precedence remain owned by FR-029 AC-17; the source strength and
resource classification remain owned by FR-028 AC-17/24. Canonical decoding all value families
does not expand a generator's supported shadow shapes or machine ABI. A shape with no lawful
harness or upstream conversion remains explicitly typed unsupported/requires-bound under
FR-028 AC-20, with no fake harness, native value or compatibility fallback.

## Setup Refusal Precedence

1. Original claim, source/package, node membership, operand/domain schema, original limits and
   proved-content identity coherence.
2. Arbitrary public input's encoded byte limit, before parsing or canonical unescaping.
3. Adapter assertion/harness selection and leaf arity/width/order; canonical syntax, shape,
   identity, presence, member and declared-domain validation, with checked count/work limits.
4. Availability of the actual upstream node-parity/value/settlement APIs.
5. For falsified parity replay, required measured native observation and its same-claim/proved-content tie, before evaluation; for verified settlement, coherent actual refinement evidence, with `not_run` explicitly absent.

Missing upstream capability is a distinct setup refusal and supplies no invented QSL code. An
otherwise malformed request reaches its earlier typed refusal even while the code gate holds.
QSL non-fault refusals retain their own catalog codes; executor faults and CG-raised malformed
playback/assumption-domain defects retain the failure ownership of FR-029. A real run/refinement
memory or wall-clock stop remains resource-inconclusive under FR-028 and cannot enter a
completed `NotExhausted` comparison path.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-033-AC-1 | PLANNED/GATED. A public consumer builds the equality-parity request directly from the composite node and original proving context. QSL recompiles the original source/package and validates FR-322 node membership; another package/source/node, operation or domain refuses with no settlement. Parameter/parameter, parameter/literal and literal/literal claims preserve their operands and literal singleton domains without an enclosing-function predicate or synthetic source. | Test |
| FR-033-AC-2 | PLANNED/GATED. Canonical round trips preserve every listed leaf family, including beyond-i64 integer, IEEE NaN and signed-zero bits, normalized rational, retained decimal, delimiter-containing Unicode text, enum member, exact quantity/unit and reference identity, and every listed composite kind. Mixed nested values retain declaration/slot/member/presence/arity/order/multiplicity. No decoder claim depends on a currently supported Boolean/i64 harness alone. | Test |
| FR-033-AC-3 | PLANNED/GATED. Noncanonical JCS/tag/scalar/escape spelling, unknown declaration or unit/reference identity, wrong enum/union member, missing/extra or wrongly present record slot, wrong tuple arity, excess cardinality, out-of-domain leaf, duplicate set/ordered-set element under authoritative equality or altered canonical element order yields a typed refusal before evaluation; bag duplicates and legal ordered-set order survive. | Test |
| FR-033-AC-4 | PLANNED/GATED. Public caller input exactly at the configured encoded-byte limit is admitted and one byte over refuses before parsing; occurrence/work exhaustion refuses with checked accounting. A 100,000-link admitted recursive value decodes, clones, compares, formats redacted Debug and drops on a 512 KiB native stack without a depth refusal. Its one-byte-too-small budget refuses truthfully; launcher capture limits alone cannot satisfy this check. | Test |
| FR-033-AC-5 | PLANNED/GATED. The retained actual assertion playback reconstructs operand values by the FR-025 original-node/leaf bindings; another harness, swapped leaf order, missing/extra leaf or wrong primitive width refuses. Literal payloads remain their singleton value and absent/null/present states remain distinct. Removing or changing a leaf binding changes reconstruction or refuses, rather than silently decoding another operand. | Test |
| FR-033-AC-6 | PLANNED/GATED. For falsified replay the driver executes the same proved generated artifact and supplies its actual native outcome/count; QSL receives this observation, the retained shadow result/count and the canonical content tie. Missing observation, another artifact, fresh regeneration that removes the mutation or changed limits refuses before parity evaluation. A canned observation or Kani transcript alone supplies no positive coverage. | Test |
| FR-033-AC-7 | PLANNED/GATED. Actual QSL exact equality/count divergence from a falsified shadow maps to CG-defect `Failed`; an assertion-only mutation with otherwise agreeing equality/count maps to named parity-agreement `Inconclusive`. Neither maps to `Refuted`. Independently mutating equality result and occurrence-pair count is detected, including no early exit after an unequal pair. | Test |
| FR-033-AC-8 | PLANNED/GATED. QSL derives bound keys from the original claim's operand declarations and literal singleton domains. Omitting a bounded position or using an empty list with declared/unbounded/recursive keys leaves it uncovered; unknown/duplicate/kind-mismatched keys refuse. Exact declared coverage and a wider legal harness range cover; a tightened range/cardinality/depth does not silently cover under ADR-021 TX3. CG cannot supply an invented declared-bound list to promote the result. | Test |
| FR-033-AC-9 | PLANNED/GATED. A result or record from another node/run/operation/operand/domain/limits/content binding yields no settlement. QSL non-fault refusals retain their catalog codes and executor faults remain failures; unavailable upstream APIs produce the typed unavailable-capability refusal with no fake values, local QSL types, predicate route or fabricated catalog code. | Test |
| FR-033-AC-10 | PLANNED/GATED. Each lawful generated family exercised by the completed route has a real verify and falsify control with retained backend evidence and mutation-killed same-artifact replay; all canonical families have the AC-2/3 decode checks. Families awaiting shadow generation or upstream admission remain explicitly unsupported/gated and are reported as gaps, rather than counted as passing end-to-end coverage. | Test |

## Dependencies

[TC-048](../matrix/TC-048-composite-parity-replay-binding.md) defines planned public-consumer
scenarios. No executable coverage is claimed by that document. FR-025 owns emitted binding
semantics; FR-028 owns shadow independence, refinement class, domain containment and execution
ceilings; FR-029 owns the exhaustive strength projection, terminal decision and record category.
QSL owns canonical value types/admission, original-package node selection, exact evaluation,
declared-bound completeness and the legal settlement/cause vocabulary. QSpec FR-181 remains the
encoding authority. No schema, binary or source file is vendored from those owners.

Implementation remains blocked on actual QSL-640 delivery of this parity arm, every required
canonical conversion/lifecycle operation, closed `Exhausted`/`NotExhausted`/`CeilingReached`
refinement evidence, same-claim observation/content binding and legal parity-agreement record.
The proposed discriminant names describe semantics; compilation against the delivered owning
API must establish their real representation. Spec publication alone establishes none of these.
