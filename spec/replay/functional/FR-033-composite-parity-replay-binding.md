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
  - target: ix://agent-ix/quire-contract-codegen/AD-003
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-070
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
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
construct a QSL-owned node-selected equality-parity claim over the original proving package. The
proposition remains the independent equality verdict and admitted occurrence-pair count owned by
[FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-71 and the native refinement
comparison owned by [FR-028](../../kani/functional/FR-028-bounded-proof-ceilings.md) AC-15. Equality
and inequality preserve the claimed operation; an inequality verdict is the negation of equality
with the same pair count. This is parity evidence about generated code, never a source-predicate
violation.

This is PLANNED IR-635 work, GATED for implementation on QSL-640's actual public API delivery. The
proposed (unmerged) QSL FR-358 specifies the node-selected parity arm, a shared `Refinement` enum
and staged settlement rules; QSL FR-070 specifies canonical values. This amended specification
replaces the earlier selected-Boolean-function proposal. The unmerged QSL change now contains
composite witness value and decode code, including exact composite integer leaves. Union value text
decodes there, but replay conversion remains unsupported because the checker has no union type
form; that admission gap remains gated in addition to parity API delivery. Merged QSL main still
exposes only Boolean/i64 witness values. The composite parity replay and verified-settlement
APIs remain absent from both sources. UNVERIFIED SOURCE: the unmerged value/decode implementation
has not been independently built or exercised here, and the parity contracts still require actual
QSL-640 code delivery. Specification publication or unmerged value code does not establish a
callable parity API or passing replay. QSpec FR-322 owns the checked-package artifact and node
identity, not a QSL node-selector requirement. This requirement names planned semantic obligations
without inventing upstream Rust signatures.

## Inputs

- The actual composite claim node and operation, original checked/emitted package and original
  source/dependency byte provision needed for QSL recompilation.
- The generated harness identity, symbolic leaf bindings, operand declarations and literal values
  from the same proving context; harness bounds and retained original execution limits.
- The falsified run's selected assertion playback, or the verified run's SUCCESS count and
  independent native-refinement evidence. The retained shadow comparison outcome includes its
  equality/inequality result and occurrence-pair count.
- For falsified parity replay, the driver's actual typed native observation of the same proved
  generated artifact over the reconstructed operands and original limits, with its pair count.
  Verified settlement takes no native-observation field; it retains actual refinement evidence,
  including explicit absence for `not_run`, without an extra native probe or invented observation. A
  refusal, incomplete outcome or execution fault remains that observation, rather than a fabricated
  Boolean.
- The CG-minted QSL `ObligationIdentity`, whose O-09 preimage is only the claimed composite node,
  its occurrence key from the recompiled package, obligation kind and one argument per distinct
  parameter-operand node id with its actual harness bounds ordered by `DomainKey` (ADR-021 TX-3).
  A parameter compared with itself contributes one argument, not one per operand appearance;
  literal operands contribute none.
- The full CG harness identity/evidence record owned by FR-015 AC-76, retaining its abstractions,
  size budget, static closure pair-node count and unexercised behaviours outside O-09. This record
  count is distinct from the runtime occurrence-pair count compared in F-6.
- The separate canonical proved-content identity and retained original context tying the native
  observation to the actual proved generated artifact and original limits. This content binding is
  not the O-09 digest and cannot be replaced by it. AD-002 R-6/R-7 and
  [AD-003](../../assurance/AD-003-evidence-chain.md) distinguish the roles; no tool/source tracking
  hash, digest label pin or additional tracking identity is introduced.
- Configured public replay encoded-input-byte, value-occurrence and work limits admitted by the
  upstream contract. Capture limits for launcher output remain separate inputs and evidence.

## Outputs

- A typed QSL-owned parity request for this node and its operand values, or a typed claim/setup
  refusal before settlement. For an admitted same claim, retained Disagreed produces Failed/CgDefect
  before operand admission or missing/fault native-observation handling, under FR-029 AC-24. A
  required unavailable capability yields no invented settlement or observation.
- A binding-checked QSL settlement/record for
  [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md), or a typed refusal with no
  settlement if the result belongs to another claim or a required upstream capability is
  unavailable. No fabricated terminal value or QSL catalog code crosses an unavailable seam.

## Behavior

- The builder shall submit the claimed node inside the original recompiled QSpec FR-322 package to
  the pending QSL-640 parity selector and preserve the package/source identity and node membership
  checks of AD-002 R-7.
- The builder shall retain the composite equality operation, each operand's actual declared domain
  and each literal operand's singleton domain, without substituting enclosing function parameters,
  changing source bytes or manufacturing a Boolean predicate/function.
- The builder shall reconstruct operands from the persisted original-node leaf bindings owned by
  [FR-025](../../kani/functional/FR-025-generated-subject-abi.md), preserving declaration
  identities, member identities, slots, presence, arity, order and multiplicity.
- The builder shall render QSL FR-070 witness values with the authoritative QSpec FR-181 canonical
  JCS encoding through its owning dependency, without copying schemas or other repositories'
  artifacts. Its transcript escaping replaces percent, semicolon, less-than and greater-than with
  `%25`, `%3B`, `%3C` and `%3E` respectively, using the upstream grammar.
- The canonical route shall preserve Boolean, arbitrary exact integer, exact IEEE float bits,
  rational, decimal, text, enum, quantity underlying exact scalar and reference values; it shall
  preserve option, record, tuple, union, sequence, set, bag and ordered-set structure. Population is
  not a value and no map family is added by this requirement.
- When the operand stage is reached, the builder shall validate canonical spelling, typed declared
  shape and admission through the owning QSL contract after same-valid-claim Disagreed precedence,
  including declared/unit/reference identities, enum/union members, required and optional slot
  presence, tuple arity, sequence order, set distinctness, bag multiplicity and
  ordered-set order/distinctness. Reference values require no invented object environment or
  dereferenced value; absent required environment retains QSL's typed no-value or refusal reading.
  Quantity conversion preserves its exact scalar and unit without rounding.
- When arbitrary public caller input arrives, the builder shall guard its encoded byte length
  against the configured replay-input limit, counting actual encoded bytes rather than a
  decoded-size estimate. It shall enforce checked occurrence/work accounting during traversal.
- The canonical value lifecycle shall use explicit heap stacks for decode, clone, equality, redacted
  Debug and drop, so admitted nesting consumes no proportional native stack. It shall refuse
  byte/count/work exhaustion with its typed resource reason, without an arbitrary nesting-depth cap.
  FR-017's stream capture ceiling is not this public-input guard.
- For falsified replay, the binding shall require the driver's measured native observation of the
  same proved artifact and the retained shadow result/count, tied to the node, operands, original
  limits and canonical proved-content identity. Playback alone and a freshly regenerated correct
  artifact supply no native observation of mutated proved content. This required observation for
  replay continuing beyond Disagreed does not mask retained same-valid-claim disagreement; the
  converter preserves FR-029 AC-24 before checking missing or failed native observation.
- The converter shall validate the result/run-to-claim binding before reading QSL's closed typed
  discriminants and catalog codes. A changed node, operation, operand, domain, source/package,
  limits, content identity or result identity yields a typed refusal with no settlement.
- The driver shall call QSL's exact equality parity arm between CG's public builder and converter.
  CG supplies no local exact evaluator, synthetic predicate or canned replay verdict.
- The builder shall project actual FR-028 refinement evidence into one shared `Refinement` for both
  falsified and verified requests: `Exhausted`, `NotExhausted`, `CeilingReached` or `Disagreed`.
  Exhaustive maps to `Exhausted`, sampled/not_run to `NotExhausted`, actual refinement ceiling to
  `CeilingReached`, and retained refinement_failed to `Disagreed`. Verified settlement carries no
  native observation, including no extra probe for `not_run`.
- The converter shall preserve the claim-validity refusals before applying any parity settlement
  row, including `Disagreed`; another package/node/occurrence, malformed bounds or O-09 mismatch
  cannot settle the actual claim.
- For a valid falsified claim, the converter shall consume QSL's first applicable outcome in the
  Falsified Settlement table below, retaining the actual native cause and resource stage.
- For a valid verified claim, the converter shall consume QSL's disagreement, refinement ceiling,
  zero/vacuous, exhausted-and-covered, then otherwise-Tested order. The ordinary CG backend's
  vacuous/inconclusive classification remains owned by FR-029; it is never relabelled a verified
  backend run to create a zero-check request.
- The builder shall submit the same node-selected claim and actual harness bounds for verified
  coverage without a CG-authored declared-bound list.
- The converter shall consume QSL-derived positions and source coverage.
- The converter shall retain QSL's uncovered reading for missing keys.
- The converter shall preserve QSL's typed key-naming refusal for unknown, duplicate or
  kind-mismatched bounds, an enum Variants bound with an undeclared variant, or a request
  DeclaredDomain over an enum position.
- The builder shall preserve literal singleton domains without deriving parameter positions.
- The converter shall consume full enum coverage only when QSL establishes that every
  source-declared variant is included.
- The converter shall retain uncovered/Tested for other non-Boolean leaf families (text, rational,
  decimal, IEEE, quantity and reference) until QSL has an authoritative bound kind describing the
  whole declared domain.
- The converter shall not promote one covered length/range/scale dimension to full coverage.
- The converter shall consume QSL's tightened-harness-bound coverage under ADR-021 TX-3 without
  promoting it to full declared-domain coverage.

## Falsified Settlement

When source/package, claimed-node membership, bound and O-09 identity admission succeeds, CG shall
preserve the following QSL first-match order for that valid claim. These are planned public facade
semantics, not implemented CG/QSL API claims.

| Row | Condition | Consumed settlement and retained evidence |
|---|---|---|
| F-1 | `Refinement::Disagreed` | Failed with CG-owned CgDefect, even if operand admission, native observation or exact comparison would otherwise agree/refuse/stop |
| F-2 | operand admission refusal | Inconclusive(ReplayRefused), retaining the actual QSL catalog code and operand index |
| F-3 | actual native observation is Incomplete or ExecutionFault | Failed/GeneratedFault retaining the actual native outcome and NativeCause; no fabricated completed result |
| F-4 | QSL exact evaluation reaches its accounting limit | Incomplete(ResourceExhausted), retaining ExactEvaluation stage, counter, configured limit and count reached |
| F-5 | `Refinement::CeilingReached` | Incomplete(ResourceExhausted), retaining RefinementCeiling stage and actual refinement ceiling evidence |
| F-6 | otherwise, exact versus retained shadow equality/inequality result and pair count | divergence Failed/CgDefect; agreement Inconclusive(ScalarAgrees) retaining the CompositeEquality claim and outcome; never Refuted |

A native `Completed` or `Refused` observation remains measured same-artifact evidence but is not the
settlement oracle: the exact-versus-shadow comparison owns F-6. `Equal` and `NotEqual` preserve the
actual claimed operator and the same admitted pair count. A retained independent refinement
disagreement cannot disappear merely because the replayed case agrees. The common identity refusal
is earlier because a mismatched request is not the same claim; FR-029 AC-24 remains intact for the
admitted claim.

The single-observation native failure in F-3 retains its actual cause and is separate from QSL's
exact-evaluation exhaustion (F-4) and an independent refinement ceiling (F-5). Backend Kani
wall/memory ceilings are outside these replay inputs: FR-028 AC-2/3 classify that backend run as
`KaniRunOutcome::Inconclusive`, and FR-029's ordinary map returns final `Incomplete(TimedOut)` or
`Incomplete(ResourceExhausted)`. FR-028 AC-24 governs the separate native refinement run. No stage
is collapsed into completed `NotExhausted` evidence or `Tested`; FR-029 AC-23 remains intact.

The terminal decision and its precedence remain owned by FR-029 AC-17 and AC-19 to AC-27; the source
strength and resource classification remain owned by FR-028 AC-17/24. Canonical decoding all value
families does not expand a generator's supported shadow shapes or machine ABI. A shape with no
lawful harness or upstream conversion remains explicitly typed unsupported/requires-bound under
FR-028 AC-20, with no fake harness, native value or compatibility fallback.

## Setup Refusal Precedence

Claim/setup checks and operand/native handling are separate stages:

1. Public transport's encoded byte limit guards input before parsing or canonical unescaping;
   checked occurrence/work limits guard every required decode. No valid claim or evidence is
   invented from malformed transport.
2. Original source/package, claimed-node membership/operation/occurrence, declared operand schema,
   harness-bound keys/kinds and O-09 identity, original limits and proved-content/context identity
   must be coherent. Adapter assertion/harness selection and the persisted leaf-binding schema
   must name that actual claim; another result/run cannot settle it. These claim-identity refusals
   precede Disagreed. A bound-schema refusal is not an operand value's admission failure.
3. The actual upstream node-parity/value/settlement capability must be available. Its absence is a
   distinct typed setup refusal, not a fabricated terminal value or QSL catalog code.
4. For the same admitted claim after delivery, retained Disagreed takes F-1/FR-029 AC-24 precedence
   before operand decode/admission and native-observation handling. A composite operand field `x`
   declared `Int[0, 9]` but supplied as `x: 12`, wrong operand shape/member/presence, missing native
   observation or native fault cannot mask that
   retained disagreement as a no-settlement setup refusal. The converter consumes Failed/CgDefect
   without fabricating an operand or native observation; the delivered owning representation is
   required by the QSL-640 code gate.
5. Without Disagreed, playback leaf arity/width/order, canonical syntax, typed value
   shape/member/presence and declared-domain validation occurs in the operand stage before exact
   evaluation. Operand admission refusal is
   F-2; native Incomplete/ExecutionFault is F-3. Missing required native observation remains a typed
   refusal with no settlement for a replay continuing beyond F-1, while actual Completed/Refused
   remains evidence. The same-artifact observation tie is required. Exact and refinement limits
   retain F-4/F-5 order. Verified settlement instead consumes coherent actual refinement evidence,
   with `not_run` explicitly absent and no native probe.

While the upstream gate holds, an incoherent claim reaches its own typed refusal and an otherwise
valid setup reaches unavailable capability. After delivery, operand/native failures do not erase
same-valid-claim Disagreed. QSL non-fault refusals retain their own catalog codes; executor faults
and CG-raised malformed playback/assumption-domain defects retain FR-029 failure ownership after
the disagreement row. A real backend or refinement stop retains its original FR-028 classification
and FR-029 resource terminal; it cannot become completed NotExhausted evidence or Tested.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-033-AC-1 | PLANNED/GATED. A public consumer builds the equality-parity request directly from the composite node and original proving context. CG submits the original source/package and node selector to QSL and accepts only its admitted same-package/node result under QSpec FR-322; another package/source/node, operation or domain refuses with no settlement. Parameter/parameter, parameter/literal and literal/literal claims preserve their operands and literal singleton domains without an enclosing-function predicate or synthetic source. | Test |
| FR-033-AC-2 | PLANNED/GATED. Canonical round trips preserve every listed leaf family, including beyond-i64 composite integer leaves as ExactInteger (while top-level Integer remains i64), IEEE NaN and signed-zero bits, normalized rational, retained decimal, delimiter-containing Unicode text, enum member, exact quantity/unit and reference identity, and every listed composite kind. Mixed nested values retain declaration/slot/member/presence/arity/order/multiplicity. No decoder claim depends on a currently supported Boolean/i64 harness alone. | Test |
| FR-033-AC-3 | PLANNED/GATED. Noncanonical JCS/tag/scalar/escape spelling, unknown declaration or unit/reference identity, wrong enum/union member, missing/extra or wrongly present record slot, wrong tuple arity, excess cardinality, out-of-domain leaf, duplicate set/ordered-set element under authoritative equality or altered canonical element order yields a typed operand/decode refusal before exact evaluation when that stage is reached, after common claim checks and same-valid-claim Disagreed precedence. An admitted claim whose operand field x is declared Int[0, 9] but supplied as x: 12 with Disagreed yields Failed/CgDefect, not an early operand refusal; bag duplicates and legal ordered-set order survive. | Test |
| FR-033-AC-4 | PLANNED/GATED. Public caller input exactly at the configured encoded-byte limit is admitted and one byte over refuses before parsing; occurrence/work exhaustion refuses with checked accounting. A 100,000-link admitted recursive value decodes, clones, compares, formats redacted Debug and drops on a 512 KiB native stack without a depth refusal. Its one-byte-too-small budget refuses truthfully; launcher capture limits alone cannot satisfy this check. | Test |
| FR-033-AC-5 | PLANNED/GATED. The retained actual assertion playback reconstructs operand values by the FR-025 original-node/leaf bindings; another harness or persisted binding schema refuses claim binding. For the same admitted claim, swapped playback leaf order, missing/extra playback leaf or wrong primitive width refuses reconstruction only after Disagreed precedence; retained Disagreed remains Failed/CgDefect. Literal payloads remain their singleton value and absent/null/present states remain distinct. Removing or changing a leaf binding changes reconstruction or refuses, rather than silently decoding another operand. | Test |
| FR-033-AC-6 | PLANNED/GATED. For falsified replay the driver executes the same proved generated artifact and supplies its actual native outcome/count; QSL receives this observation, the retained shadow result/count and the canonical content tie. Another artifact/context, fresh regeneration that removes the mutation or changed original limits refuses at claim binding. For the same admitted claim, missing observation cannot mask Disagreed/Failed/CgDefect; without Disagreed it refuses a replay continuing beyond F-1 before exact evaluation, with no fabricated observation. A canned observation or Kani transcript alone supplies no positive coverage. | Test |
| FR-033-AC-7 | PLANNED/GATED. For an identity-valid claim reaching row F-6, actual QSL exact equality/count divergence from a falsified shadow maps to CG-defect `Failed`; an assertion-only mutation with otherwise agreeing equality/count maps to `Inconclusive(ScalarAgrees)` retaining the CompositeEquality claim/outcome. Neither maps to `Refuted`. Independently mutating equality result and occurrence-pair count is detected, including no early exit after an unequal pair. | Test |
| FR-033-AC-8 | PLANNED/GATED. CG submits the original claim's operand declarations and literal singleton domains and consumes QSL-derived bound keys without a CG-authored declared-bound list. Omitting a bounded position or using an empty list with declared/unbounded/recursive keys leaves it uncovered; unknown/duplicate/kind-mismatched keys refuse. Exact declared coverage and a wider legal harness range cover; a tightened range/cardinality/depth does not silently cover under ADR-021 TX3. Enum Variants bounds cover only when every source-declared variant is included; a Variants bound naming an undeclared variant and a request DeclaredDomain over an enum position each preserve QSL's typed refusal naming the key; text/rational/decimal/IEEE/quantity/reference positions remain uncovered/Tested until an authoritative whole-domain bound kind exists, rather than promoting one dimension as full coverage. CG cannot supply an invented declared-bound list to promote the result. | Test |
| FR-033-AC-9 | PLANNED/GATED. A result or record from another node/run/operation/operand/domain/limits/content binding yields no settlement. QSL non-fault refusals retain their catalog codes and executor faults remain failures; unavailable upstream APIs produce the typed unavailable-capability refusal with no fake values, local QSL types, predicate route or fabricated catalog code. | Test |
| FR-033-AC-10 | PLANNED/GATED. Each lawful generated family exercised by the completed route has a real verify and falsify control with retained backend evidence and mutation-killed same-artifact replay; all canonical families have the AC-2/3 decode checks. Families awaiting shadow generation or upstream admission remain explicitly unsupported/gated and are reported as gaps, rather than counted as passing end-to-end coverage. | Test |
| FR-033-AC-11 | PLANNED/GATED (IR-635/QSL-640). CG computes O-09 from the exact claimed node, recompiled occurrence, kind and one argument per distinct parameter-operand node ID with actual harness bounds ascending by DomainKey. Self-comparison contributes the same parameter once; two distinct parameters contribute once each; literal-only comparison contributes no arguments. Changing one such member changes it; changing only abstractions, size budget, static closure pair-node count or unexercised behaviours does not. The record's pair-node count is distinct from F-6's runtime occurrence-pair count. Those extras remain in the full FR-015-AC-76 record, and changing proved artifact/context invalidates the separate canonical content tie even when O-09 remains equal. Existing function/frame preimages are unchanged. | Test |
| FR-033-AC-12 | PLANNED/GATED (IR-635/QSL-640). Both paths carry the same four-state Refinement. Claim/source/node/occurrence/bounds/O-09 refusal precedes Disagreed; for a valid claim Disagreed wins over operand refusal (including an operand field x declared Int[0, 9] but supplied as x: 12), missing native observation, native fault, exact limit, refinement ceiling and agreeing replay. No early operand/native setup refusal erases that disagreement and no missing evidence is fabricated. Completed/Refused native evidence does not replace exact-versus-shadow verdict/pair-count comparison, and the actual Eq/Ne operation survives. | Test |
| FR-033-AC-13 | PLANNED/GATED (IR-635/QSL-640). Without retained Disagreed, falsified admission refusal precedes native fault; native Incomplete/ExecutionFault yields GeneratedFault/Failed with actual NativeCause before exact evaluation; ExactEvaluation resource exhaustion precedes RefinementCeiling and each remains distinctly staged Incomplete(ResourceExhausted). Backend Kani timeout/memory Inconclusive outcomes never enter this facade and retain their ordinary final Incomplete mapping. No resource stage is fabricated or converted to Tested. | Test |

## Dependencies

[TC-048](../matrix/TC-048-composite-parity-replay-binding.md) defines planned public-consumer
scenarios. No executable coverage is claimed by that document. FR-025 owns emitted binding
semantics; FR-028 owns shadow independence, refinement class, domain containment and execution
ceilings; FR-029 owns the exhaustive strength projection, terminal decision and record category. QSL
owns canonical value types/admission, original-package node selection, exact evaluation,
declared-bound completeness and the legal settlement/cause vocabulary. QSpec FR-181 remains the
encoding authority. No schema, binary or source file is vendored from those owners.

Implementation remains blocked on actual QSL-640 delivery of this parity arm, every required
canonical conversion/lifecycle operation, shared
`Exhausted`/`NotExhausted`/`CeilingReached`/`Disagreed` refinement evidence, same-claim
observation/content binding and legal parity-agreement record. The proposed discriminant names
describe semantics; compilation against the delivered owning API must establish their real
representation. Spec publication alone establishes none of these.