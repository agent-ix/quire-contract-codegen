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
  - target: ix://agent-ix/quire-spec-language/FR-357
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

Imported contexts remain PLANNED: the Eq constructor returns typed `ImportedContextUnsupported`
until CG retains admitted dependency packages from the original compile.

## Description

When the driver submits a composite equality `bounded_shadow` item for settlement, CG shall
construct a QSL-owned node-selected equality-parity claim over the original proving package. The
proposition remains the independent equality verdict and admitted occurrence-pair count owned by
[FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-71 and the native refinement
comparison owned by [FR-028](../../kani/functional/FR-028-bounded-proof-ceilings.md) AC-15. Equality
and inequality preserve the claimed operation; an inequality verdict is the negation of equality
with the same pair count. This is parity evidence about generated code, never a source-predicate
violation.

IR-666 delivers the public CG report converters and direct QSL facade tests of F-1 to F-7 and
V-1 to V-5. IR-635's first slice adds the public `OriginalCompositeEqContext` and immutable
request/report types for admitted graph-child `structural.eq`: retained source-lock checks,
original node/function/occurrence membership, exact recompiled package/context equality,
positional O-09 construction and genuine public verified-shadow invocation through the converter.
The caller's canonical proved-content identity and original limits are retained binding inputs;
this slice does not independently authenticate a generated artifact or produce backend/refinement
evidence. The typed `structural.ne` accessor is still missing under IR-690. Falsified same-artifact
native execution, playback reconstruction, generated-family and proof-strength controls remain
planned. QSL's public composite witness, parity and verified-shadow interfaces are delivered.
The literal constructor controls use admitted one-field `Integer` records. A QSL-emitted
`Int[0, 9]` record literal inserts a `quire.op.numeric.narrow` conversion child (IR-691), and
`sequence[]` inserts an expression-constructor descendant. IR-651 refuses expression-bearing
literal graphs as `NonliteralGraphValue`; bounded-field and collection-constructor literals remain
gated and are not counted as positive coverage.
The builder shall use the owning `replay_composite_parity(wire, claim, falsified, replay_limits)`
and `settle_verified_shadow(wire, claim, verified, replay_limits)` interfaces, with public
`ReplayLimits` last. The claim retains original exact-evaluation `ScalarLimits`; transport
`ReplayLimits` remain a separate entry argument, not an invented `CompositeIdentity` member.
QSpec FR-322 owns the original checked-package artifact and node/occurrence membership. QSL FR-070
owns the canonical composite `WitnessValue` forms, including exact nested integer leaves; a
successful canonical decode does not establish source-type admission or lawful generated coverage.
Union text decode remains distinct from unsupported checker union admission. The completed CG
route requires actual original-artifact authentication, typed dependency integration and real
generated-family verification/falsification controls before CODE delivery.

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
- The CG-minted QSL `ObligationIdentity`, whose O-09 preimage contains only `arguments`, the
  claimed application `node`, `obligation_kind` and `occurrence_key` from the recompiled package.
  There is one argument per operand position in operand order, including repeated operands and
  literal operands. Parameter arguments use the actual `graph_child` node at that position and
  only its drawn `DomainKey::Node` harness bounds naming that parameter; repeated positions remain
  distinct argument entries, not a distinct node set. Population-keyed metadata remains in the
  full CG proving record, outside the admitted QSL composite claim and its O-09 arguments. A QSL
  request carrying a `DomainKey::Population` harness bound receives typed `HarnessUnknownKey`
  refusal during `prepare`, before the O-09 identity tie; its binding-checked `Refused` report maps
  to `Inconclusive(ReplayRefused)`. A CG precheck refusal still returns no report or terminal value. Composite literals use their own actual `graph_child` node with empty `bounds` because
  their value is bound by that content-addressed node. An inline integer literal uses its
  application node/occurrence/position and singleton `range`; other inline literal types refuse.
  Within each Bounds argument, keys sort by canonical encoded key bytes, not `DomainKey` Ord;
  duplicate keys within that argument refuse. Repetition across positional arguments is preserved.
- The full CG harness identity/evidence record owned by FR-015 AC-76, retaining its abstractions,
  size budget, static closure pair-node count and unexercised behaviours outside O-09. This record
  count is distinct from the runtime occurrence-pair count compared in F-7.
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
  [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md). CG setup failure before
  invoking QSL, missing report or wrong-claim report returns a typed refusal without a terminal
  value. A QSL common-step `Refused` result is a report with a terminal value after its full claim
  binding passes. No fabricated QSL catalog code crosses an unavailable seam.

## Behavior

- The builder shall submit the claimed node inside the original recompiled QSpec FR-322 package to
  owning public QSL parity selector and preserve the original retained source/dependency bytes,
  package/context identity and claimed node/occurrence membership checks of AD-002 R-7.
- The builder shall construct O-09 through QSL's shared public typed `parity_obligation` using
  `ParityPreimage`, positional `ParityArgument`, `Domain::Range` or `Domain::Bounds` and
  `BoundEntries`.
- When the encoder refuses, the builder shall retain typed `IdentityEncodeError`/owning identity
  refusal rather than a fabricated digest.
- The canonical four-member preimage shall gain no top-level operator, native observation, content
  identity, refinement or counter member; operation validation remains in node membership and the
  full claim.
- For each parameter argument, the builder shall include only drawn `DomainKey::Node` bounds naming
  that parameter, including its actual collection/depth bounds under declared-domain substitution.
- The builder shall retain Population-keyed metadata only in the full CG proving record, outside
  the admitted QSL composite claim and its O-09 arguments.
- When a QSL request carries a `DomainKey::Population` harness bound, the converter shall retain
  the binding-checked `HarnessUnknownKey` refusal report and its `Inconclusive(ReplayRefused)`
  terminal value. A CG pre-invocation refusal produces no report or terminal value.
- The builder shall sort keys within each Bounds argument by canonical encoded key bytes.
- When a Bounds argument contains duplicate keys, the builder shall refuse that argument.
- The builder shall preserve repeated positional arguments as separate entries.

Encoded-key ordering example: path `[0,0]` precedes `[0]`, and `[10]` precedes `[2]`.

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
- The converter shall retain the actual sent `CompositeIdentity::new` over the request obligation,
  `CompositeParityClaim` and complete `CompositeEvidence` before calling the owning entry.
- The converter shall validate the result/run-to-claim binding before reading QSL's closed typed
  discriminants and catalog codes. It shall compare `report.claim()` with
  `CompositeIdentity::new(obligation, &claim, evidence)` over exactly what CG sent: obligation
  identity; node; occurrence; operator; obligation kind; harness bounds; exact-evaluation limits;
  canonical proved-content identity; and the full observation. Falsified observation includes both
  operands, retained shadow verdict and pair count, actual native observation and cause, and
  refinement. Verified observation includes SUCCESS count and refinement. A missing report or a
  change to any one member yields a typed CG refusal with no terminal value. CG's own failed
  source/package, harness, playback or content prechecks likewise return no report or terminal
  value. Once the full report binds, any QSL `Refused` result is a typed report with a terminal
  value: non-fault refusals retain their QSL code in
  `Inconclusive(ReplayRefused)`, while `ReplayRefusal::Fault` and
  `ReplayRefusal::Admission(AdmissionFailure::Fault)` yield `Failed`. QSL's `prepare` refusals
  precede F-1. A refusal raised during operand admission follows F-1 and F-2; a refusal raised
  during exact comparison follows admission. Retained Disagreed therefore wins over those later
  refusals. CG shall not synthesize an invariant fault or a QSL report to exercise this mapping.
- The converter shall expose `composite_parity_terminal_value` for a falsified
  `CompositeParityReport` and `verified_shadow_terminal_value` for a `VerifiedShadowReport`.
  Each operation shall take the retained sent `CompositeIdentity` and an optional genuine report
  and return a binding-checked settlement with private result storage, a `result()` accessor and
  a `terminal_value()` derived from that QSL result.
- When the report is absent or `report.claim()` differs from the retained sent identity, the
  converter shall return `CompositeReportError::MissingReport` or
  `CompositeReportError::ClaimMismatch`, respectively, with no terminal value. The separate IR-635
  builder owns pre-invocation source, artifact, harness and playback refusals.
- The comparison shall use the retained sent claim, not a later-mutated request.
  When a valid request changes before sending, the converter shall retain the identity of the
  changed request actually sent and compare the genuine returned report against that identity.
- When the genuine report equals the actual sent identity, the converter shall pass this binding
  check regardless of differences from an earlier unsent request. Independent source/artifact
  prechecks still apply.
  Echo equality does not authenticate the proved artifact: independent retained
  AD-002 R-6/R-7 source/package/context, canonical proved-content and original-limit ties shall
  still establish that the measured native observation belongs to the actual proved generated
  artifact. O-09, canonical proved-content identity and refinement certificate identity remain
  separate roles; none shall substitute for another or create a new tracking digest.
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

The actual generated-family and source coverage remains a CODE gate. Real Text-leaf profile
selection and nested set/bag single-encoding performance remain open upstream work; API availability
and rational-leaf substitution do not close those checks. Unbounded collection/recursive coverage
exercised only through bound derivation/coverage seams does not establish source-function coverage
where source grammar requires explicit maxima. Families lacking lawful generated shadow or owning
admission shall remain explicitly unsupported/gated, with no synthetic proof or claim of closure.

## Falsified Settlement

When source/package, claimed-node membership, bound and O-09 identity admission succeeds, CG shall
preserve the following QSL first-match order for that valid claim. QSL FR-358's F-1 to F-7 rows
and `qsl-replay` facade were merged in QSL #645. IR-666's public converter preserves their
bound results; IR-635's original-artifact builder and production invocation remain planned.

| Row | Condition | Consumed settlement and retained evidence |
|---|---|---|
| F-1 | `Refinement::Disagreed` | Failed with CG-owned CgDefect, even if operand admission, native observation or exact comparison would otherwise agree/refuse/stop |
| F-2 | actual native observation is Incomplete or ExecutionFault | `GeneratedFault { native }`/Failed retaining the actual native outcome and NativeCause; QSL owns the no-admission/no-exact-evaluation ordering |
| F-3 | operand admission refusal | `RefusedInput`, Inconclusive(ReplayRefused), retaining the actual QSL catalog code and operand index |
| F-4 | request accounting limit reached during operand admission | `Incomplete { stage: Admission }`/Incomplete(ResourceExhausted), retaining the counter, configured limit and count reached; QSL owns the no-exact-evaluation ordering |
| F-5 | QSL exact evaluation reaches its accounting limit | `Incomplete { stage: ExactEvaluation }`/Incomplete(ResourceExhausted), retaining QSL's counter, configured limit and count reached |
| F-6 | `Refinement::CeilingReached` | `Incomplete { stage: RefinementCeiling }`/Incomplete(ResourceExhausted), retaining actual refinement ceiling evidence |
| F-7 | otherwise, exact versus retained shadow equality/inequality result and pair count | divergence Failed/CgDefect; agreement Inconclusive(ScalarAgrees) retaining the CompositeEquality claim and outcome; never Refuted |

A native `Completed` or `Refused` observation remains measured same-artifact evidence but is not the
settlement oracle: the exact-versus-shadow comparison owns F-7. `Equal` and `NotEqual` preserve the
actual claimed operator and the same admitted pair count. A retained independent refinement
disagreement cannot disappear merely because the replayed case agrees. The common identity refusal
is earlier because a mismatched request is not the same claim; FR-029 AC-24 remains intact for the
admitted claim.

The single-observation native failure in F-2 retains its actual cause and precedes operand refusal
and both limit stages. Request accounting exhaustion during admission is F-4 and never claims exact
evaluation ran; QSL exact-evaluation exhaustion is F-5 and precedes the independent refinement
ceiling at F-6. Backend Kani
wall/memory ceilings are outside these replay inputs: FR-028 AC-2/3 classify that backend run as
`KaniRunOutcome::Inconclusive`, and FR-029's ordinary map returns final `Incomplete(TimedOut)` or
`Incomplete(ResourceExhausted)`. FR-028 AC-24 governs the separate native refinement run. No stage
is collapsed into completed `NotExhausted` evidence or `Tested`; FR-029 AC-23 remains intact.

The terminal decision and its precedence remain owned by FR-029 AC-17 and AC-19 to AC-28; the source
strength and resource classification remain owned by FR-028 AC-17/24. Canonical decoding all value
families does not expand a generator's supported shadow shapes or machine ABI. A shape with no
lawful harness or upstream conversion remains explicitly typed unsupported/requires-bound under
FR-028 AC-20, with no fake harness, native value or compatibility fallback.

## Setup Refusal Precedence

Claim/setup checks and operand/native handling are separate stages:

1. Public transport's encoded byte limit guards input before parsing or canonical unescaping;
   checked occurrence/work limits guard every required decode. No valid claim or evidence is
   invented from malformed transport.
2. CG's original source/package, claimed-node membership/operation/occurrence, declared operand schema,
   harness-bound keys/kinds and O-09 identity, original limits and proved-content/context identity
   must be coherent. Adapter assertion/harness selection and the persisted leaf-binding schema
   must name that actual claim; another result/run cannot settle it. These claim-identity refusals
   precede Disagreed and return no report or terminal value. A bound-schema refusal is not an
   operand value's admission failure. If CG sends a request instead, QSL's `prepare` may return
   `CompositeParityResult::Refused`; after full identity binding this report has a terminal value
   and precedes F-1. A later operand-admission or exact-comparison fault may also produce
   `Refused`, but only after F-1 (and F-2); Disagreed suppresses it. The terminal value of any
   binding-valid `Refused` report is ReplayRefused with QSL's code for a non-fault, or Failed for
   `Fault` and `Admission(Fault)`.
3. QSL #645 has delivered the node-parity/value/settlement facade, and IR-666 exposes the
   public CG report converters. IR-635 still owns the original-artifact builder and production
   invocation. A future pre-invocation refusal from that builder carries no QSL report or
   terminal value; the converter never fabricates a QSL catalog code.
4. For the same admitted claim after CG consumer delivery, retained Disagreed takes F-1/FR-029 AC-24 precedence
   before operand decode/admission and native-observation handling. A composite operand field `x`
   declared `Int[0, 9]` but supplied as `x: 12`, wrong operand shape/member/presence, missing native
   observation or native fault cannot mask that
   retained disagreement as a no-settlement setup refusal. The converter consumes Failed/CgDefect
   without fabricating an operand or native observation; CG code must use QSL #645's delivered
   owning representation.
5. Without Disagreed, playback leaf arity/width/order, canonical syntax, typed value
   shape/member/presence and declared-domain validation occurs in the operand stage before exact
   evaluation. Native Incomplete/ExecutionFault is F-2 before operand admission refusal at F-3;
   request admission exhaustion is F-4. Missing required native observation remains a typed
   refusal with no settlement for a replay continuing beyond F-1, while actual Completed/Refused
   remains evidence. The same-artifact observation tie is required. Exact and refinement limits
   retain F-5/F-6 order. Verified settlement instead consumes coherent actual refinement evidence,
   with `not_run` explicitly absent and no native probe.

The public converter binds reports from direct QSL facade calls and the Eq-only verified
constructor. Falsified production construction and same-artifact authentication remain IR-635
work. On a valid direct claim, operand/native
failures do not erase same-valid-claim Disagreed. QSL non-fault refusals retain their own catalog
codes; executor faults
and CG-raised malformed playback/assumption-domain defects retain FR-029 failure ownership after
the disagreement row. A real backend or refinement stop retains its original FR-028 classification
and FR-029 resource terminal; it cannot become completed NotExhausted evidence or Tested.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-033-AC-1 | PARTIAL (IR-635 Eq-only verified constructor controls; Ne accessor IR-690, falsified and generated-artifact routes planned). A public consumer builds the equality-parity request directly from the composite node and original proving context. CG submits the original source/package and node selector to QSL and accepts only its admitted same-package/node result under QSpec FR-322. A mismatch caught by CG before invocation returns no report or terminal value; a QSL common-step mismatch returns a binding-checked `Refused` report with its terminal value. Parameter/parameter, parameter/literal and literal/literal claims preserve their operands and literal singleton domains without an enclosing-function predicate or synthetic source. Original recompiled node/occurrence and each positional graph child or inline literal must belong to the actual retained proving context; no enclosing-function substitution satisfies this. | Test |
| FR-033-AC-2 | PLANNED/GATED. Canonical round trips preserve every listed leaf family, including beyond-i64 composite integer leaves as ExactInteger (while top-level Integer remains i64), IEEE NaN and signed-zero bits, normalized rational, retained decimal, delimiter-containing Unicode text, enum member, exact quantity/unit and reference identity, and every listed composite kind. Mixed nested values retain declaration/slot/member/presence/arity/order/multiplicity. No decoder claim depends on a currently supported Boolean/i64 harness alone. | Test |
| FR-033-AC-3 | PLANNED/GATED. Noncanonical JCS/tag/scalar/escape spelling, unknown declaration or unit/reference identity, wrong enum/union member, missing/extra or wrongly present record slot, wrong tuple arity, excess cardinality, out-of-domain leaf, duplicate set/ordered-set element under authoritative equality or altered canonical element order yields a typed operand/decode refusal before exact evaluation when that stage is reached, after common claim checks and same-valid-claim Disagreed precedence. An admitted claim whose operand field x is declared Int[0, 9] but supplied as x: 12 with Disagreed yields Failed/CgDefect, not an early operand refusal; bag duplicates and legal ordered-set order survive. | Test |
| FR-033-AC-4 | PLANNED/GATED. Public caller input exactly at the configured encoded-byte limit is admitted and one byte over refuses before parsing; occurrence/work exhaustion refuses with checked accounting. A 100,000-link admitted recursive value decodes, clones, compares, formats redacted Debug and drops on a 512 KiB native stack without a depth refusal. Its one-byte-too-small budget refuses truthfully; launcher capture limits alone cannot satisfy this check. | Test |
| FR-033-AC-5 | PLANNED/GATED. The retained actual assertion playback reconstructs operand values by the FR-025 original-node/leaf bindings; another harness or persisted binding schema refuses claim binding. For the same admitted claim, swapped playback leaf order, missing/extra playback leaf or wrong primitive width refuses reconstruction only after Disagreed precedence; retained Disagreed remains Failed/CgDefect. Literal payloads remain their singleton value and absent/null/present states remain distinct. Removing or changing a leaf binding changes reconstruction or refuses, rather than silently decoding another operand. | Test |
| FR-033-AC-6 | PLANNED/GATED. For falsified replay the driver executes the same proved generated artifact and supplies its actual native outcome/count; QSL receives this observation, the retained shadow result/count and the canonical content tie. Another artifact/context, fresh regeneration that removes the mutation or changed original limits refuses at claim binding. For the same admitted claim, missing observation cannot mask Disagreed/Failed/CgDefect; without Disagreed it refuses a replay continuing beyond F-1 before exact evaluation, with no fabricated observation. A canned observation or Kani transcript alone supplies no positive coverage. Matching an echoed content identity is not artifact authentication; independently retained R-6/R-7 source/package/context and original limits remain required. | Test |
| FR-033-AC-7 | PARTIAL (IR-666 direct public F-7 converter controls; IR-635 original-artifact path planned). For an identity-valid claim reaching row F-7, actual QSL exact equality/count divergence from a falsified shadow maps to CG-defect `Failed`; an assertion-only mutation with otherwise agreeing equality/count maps to `Inconclusive(ScalarAgrees)` retaining the CompositeEquality claim/outcome. Neither maps to `Refuted`. Independently mutating equality result and occurrence-pair count is detected, including no early exit after an unequal pair. | Test |
| FR-033-AC-8 | PLANNED/GATED. CG submits the original claim's operand declarations and literal singleton domains and consumes QSL-derived bound keys without a CG-authored declared-bound list. Omitting a bounded position or using an empty list with declared/unbounded/recursive keys leaves it uncovered; unknown/duplicate/kind-mismatched keys refuse. Exact declared coverage and a wider legal harness range cover; a tightened range/cardinality/depth does not silently cover under ADR-021 TX3. Enum Variants bounds cover only when every source-declared variant is included; a Variants bound naming an undeclared variant and a request DeclaredDomain over an enum position each preserve QSL's typed refusal naming the key; text/rational/decimal/IEEE/quantity/reference positions remain uncovered/Tested until an authoritative whole-domain bound kind exists, rather than promoting one dimension as full coverage. CG cannot supply an invented declared-bound list to promote the result. Composite literal graph children have empty Bounds and no free position; this cannot remove declared/drawn bounds of a parameter position or promote source-unbounded derivation-only coverage. | Test |
| FR-033-AC-9 | PARTIAL (IR-666 report binding covered; IR-635 Eq-only verified source/package/node/occurrence prechecks and genuine invocation covered; generated-artifact authentication and falsified invocation planned). CG compares `report.claim()` with `CompositeIdentity::new` over the sent obligation, node, occurrence, operator, obligation kind, harness bounds, limits, content identity and complete observation: falsified operands, shadow verdict/count, native observation/cause and refinement, or verified SUCCESS count and refinement. Changing each CG-owned retained sent-identity member independently with a genuine report held fixed, or omitting the report, yields a typed CG refusal with no terminal value; a valid report binds. The Eq-only verified constructor returns no report or terminal value on its precheck refusal; remaining production prechecks are planned. A binding-valid QSL `Refused` report retains its QSL code and maps non-fault to Inconclusive(ReplayRefused), or maps `Fault`/`Admission(Fault)` to Failed; a `prepare` refusal precedes Disagreed, while an admission or exact-comparison refusal follows it. No predicate route, fabricated report or invented catalog code is used. Before every outcome projection, compare against the actual retained sent identity, never a later-mutated request; a genuine public report compared against an independently changed CG-owned retained sent-identity member refuses. A valid request changed before send binds when its genuine report equals that actual changed sent identity; differences from an earlier unsent identity do not cause binding refusal. This is not artifact authentication. Public ReplayLimits remains a separate last facade argument, not a ScalarLimits/full-claim member. | Test |
| FR-033-AC-10 | PLANNED/GATED. Each lawful generated family exercised by the completed route has a real verify and falsify control with retained backend evidence and mutation-killed same-artifact replay; all canonical families have the AC-2/3 decode checks. Families awaiting shadow generation or upstream admission remain explicitly unsupported/gated and are reported as gaps, rather than counted as passing end-to-end coverage. Open real Text-profile, nested set/bag performance and source-unbounded coverage limitations remain explicit; API presence or rational substitution supplies no generated-family acceptance. | Test |
| FR-033-AC-11 | PARTIAL (IR-635 Eq-only graph-child positional O-09 constructor; Ne accessor IR-690, inline operands and full proving-record/native integration planned). CG computes O-09 through the owning typed parity_obligation from the exact claimed application node, recompiled occurrence, kind and one argument per operand position in operand order. Self-comparison contributes two graph_child arguments for the same actual parameter, each retaining only its actual Node-keyed bounds naming that parameter; Population-keyed metadata remains only in the full CG proving record, outside the admitted QSL composite claim and O-09. A QSL request carrying a Population-keyed harness bound produces typed HarnessUnknownKey during prepare before identity_tie; its binding-checked Refused report maps to Inconclusive(ReplayRefused), while CG pre-invocation refusal has no report or terminal value; distinct parameters each contribute their positional argument. Composite literal operands contribute their own graph_child node with empty Bounds; inline integer literals use application/occurrence/position and singleton Range, and unsupported inline types refuse. Bounds sort by canonical encoded key bytes, not DomainKey Ord, with duplicate-key refusal within each argument; repeated parameter positions are not deduplicated. Changing a preimage member changes identity or refuses original membership; changing only abstractions, size budget, static closure pair-node count or unexercised behaviours does not. That record count is distinct from F-7 runtime occurrence-pair count. Full FR-015-AC-76 evidence and independent original proved-content/context binding remain authoritative even when O-09 matches. No operator/native/content/refinement/counter top-level preimage member or tracking digest is added; existing function/frame preimages remain unchanged. | Test |
| FR-033-AC-12 | PARTIAL (IR-666 direct QSL F-1 precedence controls; IR-635 Eq-only verified constructor prechecks and public invocation covered; falsified and cross-outcome integration planned). Both paths carry the same four-state Refinement. Claim/source/node/occurrence/bounds/O-09 refusal precedes Disagreed; for a valid claim Disagreed wins over operand refusal (including an operand field x declared Int[0, 9] but supplied as x: 12), missing native observation, native fault, exact limit, refinement ceiling and agreeing replay. No early operand/native setup refusal erases that disagreement and no missing evidence is fabricated. Completed/Refused native evidence does not replace exact-versus-shadow verdict/pair-count comparison, and the actual Eq/Ne operation survives. | Test |
| FR-033-AC-13 | PARTIAL (IR-666 direct public F-2 to F-6 controls; IR-635 must reach the same rows from a CG-built request over the original proving context). On a valid falsified claim without Disagreed, a native Incomplete or ExecutionFault wins over an invalid operand, too-small admission limit, too-small exact limit and CeilingReached: CG observes GeneratedFault/Failed retaining its NativeCause. With a Completed native, an operand outside its declared domain yields RefusedInput/ReplayRefused even with CeilingReached; a request limit reached during admission yields Incomplete/Admission with its counter, and an exact limit reached after admission yields Incomplete/ExactEvaluation even with CeilingReached; only with sufficient limits does CeilingReached yield Incomplete/RefinementCeiling. CG asserts these report results and terminal values; QSL FR-358 owns the internal rule that F-2 skips admission and exact evaluation and F-4 skips exact evaluation. All three limits are ResourceExhausted but keep distinct stages. Backend Kani timeout/memory Inconclusive outcomes never enter this facade. No resource stage becomes Tested. | Test |

## Dependencies

[TC-048](../matrix/TC-048-composite-parity-replay-binding.md) records the direct public-converter
controls and the remaining IR-635 production scenarios. FR-025 owns emitted binding
semantics; FR-028 owns shadow independence, refinement class, domain containment and execution
ceilings; FR-029 owns the exhaustive strength projection, terminal decision and record category. QSL
owns canonical value types/admission, original-package node selection, exact evaluation,
declared-bound completeness and the legal settlement/cause vocabulary. QSpec FR-181 remains the
encoding authority. No schema, binary or source file is vendored from those owners.

Implementation shall consume the delivered owning public interfaces and retain every required
canonical conversion/lifecycle, refinement, same-claim observation/content and legal cause check.
The Eq verified constructor controls use supplied verified evidence and establish no generated
backend proof or native observation. Actual CG driver authentication, generated-family proofs and
tests remain PLANNED/UNRUN CODE gates;
public API availability and specification publication establish none of that evidence.
