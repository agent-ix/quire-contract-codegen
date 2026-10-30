---
id: FR-032
title: "Bind routed scalar lowering-parity witnesses to QSL replay"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/AD-002
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-357
    type: depends_on
  - target: ix://agent-ix/quire-spec-language/ADR-013
    type: references
---
# FR-032: Bind routed scalar lowering-parity witnesses to QSL replay

## Description

When the driver requests replay of a routed scalar counterexample, CG shall bind the request
directly to its `ScalarObligationIdentity`, preserving the arithmetic-parity proposition the
actual generated harness checked. This requirement owns a public request builder and a public
settlement converter; the driver owns the intervening QSL replay call. QSL owns the operator-level
exact evaluation, envelope arm, evidence and terminal causes.

This is planned IR-631 work. The QSL operator and full-report identity contracts are merged and measured below.
CG metadata retention, consumer conformance and the driver-observation contract remain CODE gates. No CG implementation or passing scalar-replay test is claimed.

## Scope

The slice is one routed exact-scalar Kani item, its retained assertion playback, its proved
generated oracle and its checked/emitted package. It includes typed setup refusals and conversion
to the settlement read by [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md).
It excludes driver orchestration and FR-331 serialization, source-predicate violation, a
function-level declared-parameter parity route, new operation-contract clause-kind generation,
moving existing replay calls, a local exact evaluator or direct exact-kernel dependency in CG,
copied QSL types, and conversion through `KaniObligationIdentity` or V1 `ReplayPackage`.

The proposed scalar plan is constrained by
[AD-002](../../assurance/AD-002-cg-qsl-replay-seam.md), including R-6 and R-7. Their scalar
contract fit is supplied by the merged operator contract; CG conformance remains planned. This document grants no exception to those
rules. Scalar lowering parity is never a source-property refutation.

## Inputs

- The `ScalarObligationIdentity` returned by
  [FR-022](../../routed/functional/FR-022-routed-generation.md), including the claimed node,
  catalogued operation identity, qualified harness name and ordered operand schema.
- The falsified run's retained assertion playback. The scalar builder owns its decode byte
  ceiling: the same 8 MiB per-harness ceiling owned by
  [FR-017](../../kani/functional/FR-017-kani-execution-evidence.md) AC-14 and the Kani capture
  adapter. Before adapter parsing, the builder checks the selected single-harness playback
  length against that ceiling. The grouped launcher limit remains 8 MiB times its harness
  count; that process bound does not authorize a larger selected scalar playback.
- The original scalar generation/proving context, retaining the renderer's own accounting
  limits alongside the generated source. CG's scalar generation authority records those
  original limits; the caller preserves that record through the proving run. The builder
  compares replay limits with that original context, not with a caller's new replay budget.
  This context producer is planned IR-631 work: `ScalarObligationIdentity` does not currently
  persist limits. The current renderer's limit fields are all `u64::MAX`; there is no missing-
  context default or inference from the current identity. The limits and generated source must be retained with the original proving context.
- The checked/emitted package used to derive this node's operand and result ranges, the proving
  context, and the original proving `call_site` package identity and source bytes needed by
  QSL's envelope. Locating the enclosing source function for compilation does not turn this
  scalar node into a function-contract obligation or predicate.
- The captured path and byte set of the **same proved generated oracle**. After building
  the plan, the driver executes that artifact natively over the plan's decoded operator operands
  with the harness's recorded limits and supplies its actual typed completed value, refusal,
  undefined/incomplete outcome or execution fault to the QSL request. Kani playback alone
  supplies no generated outcome. Executing a newly regenerated unmutated oracle is not an
  observation of the proved content.
- At conversion, the same scalar binding and falsified run, plus QSL's own scalar result or
  refusal from the driver's call, or the builder's typed setup refusal.

The driver is the native-observation authentication authority. CG supplies the retained
scalar generation/proving record and a checked plan-completion/conversion contract; the
builder makes no native execution or QSL replay call. The driver executes the **same proved
generated artifact** at the decoded operand vector and retained renderer limits, and authenticates
the typed outcome against that record's retained staged bytes, operands
and limits before completing the request. Missing evidence or a mismatch returns a typed
completion refusal, with no QSL evaluation or settlement. An unchecked caller echo of outcome
or a newly regenerated oracle does not meet this contract.

This boundary trusts the driver to execute and authenticate that artifact honestly. QSL does
not receive or execute the artifact; it compares exact semantics against the supplied typed
observation. CG's checked completion contract requires the driver's authenticated observation,
not a bare outcome. The driver compares its original generated artifacts, harness and record
byte for byte with the receipt's retained staged entries under FR-017-AC-45 to AC-49. The
producer/consumer wiring remains CODE-gated.

## Outputs

- A typed admitted scalar binding and QSL-owned replay plan for the operator-level
  lowering-parity arm, or a distinct typed setup refusal. The driver completes the plan with
  its actual native generated observation before calling QSL; it executes no regenerated oracle.
- A checked conversion to the existing settlement where representable, or a typed binding
  refusal with no settlement when a result or run belongs to another obligation. The new
  scalar terminal readings follow the measured QSL contract below; their CG settlement
  representation and checked conversion remain planned and gated on full-report identity.
  Existing predicate replay causes do not substitute for scalar readings.
- For setup refusal caused by a named unavailable capability below, a typed converter refusal with no
  settlement. The named missing capability is not relabelled a CG defect or assigned a
  fabricated QSL code. The caller cannot manufacture a terminal record from that refusal;
  attempted terminal mapping without settlement remains governed by
  [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-15.

## Behavior

- The public scalar builder shall accept `ScalarObligationIdentity` directly.
- The builder shall select and decode playback through the Kani output adapter against the
  scalar identity's own qualified harness and persisted operand order, with one eight-byte
  `i64` value per symbolic operand and no function-identity coercion.
- When producing an admitted request, the builder shall validate the claimed node and
  catalogued operation against the proving package and every operand against its own declared
  bounds in persisted symbolic call order.
- The builder shall carry the operator, decoded operands, operand ranges, result range and
  harness accounting limits to QSL without replacing them with a selected function's arguments.
- The builder shall bind the replay plan to the operand vector, limits, node, operation and
  retained staged artifact bytes required of the driver's native observation.
- If an identity, schema, expected staged-byte tie or required upstream capability cannot be
  validated, then the builder shall return a distinct typed setup refusal before replay.
- CG's scalar builder and converter shall leave exact evaluation and replay execution to QSL
  through the driver's public-facade call.
- When asked to return a settlement or derive its terminal value, the converter shall compare
  the QSL report's full claim identity with the exact retained claim sent by the driver, and
  validate the retained run-to-binding tie. This check applies to **every** outcome, including
  `Diverged`, `Agrees`, `GeneratedFault`, `RefusedInput`, exact `Incomplete` and other `Refused`.
  Equality of the structural obligation identity alone, or identity present only on agreement, is insufficient.
  An absent report or different report claim yields a typed binding refusal with no settlement.
  The report `claim()` contract is measured in merged QSL change #650; actual CG consumer
  conformance remains planned and untested.
- The converter shall read QSL's typed result/refusal discriminants and catalog codes rather
  than infer a verdict or refusal code from message text.
- The converter shall preserve QSL's distinction between generated/native-exact divergence
  (CG lowering fault), agreement despite the Kani falsification (harness-defect inconclusive),
  out-of-operand-domain input (refusal), and an out-of-result-domain exact result (expected
  refusal that is not itself a falsification).
- The scalar converter shall never return `ReplaySettlement::Reproduced` for this
  lowering-parity proposition.

The scalar generation proposition is owned solely by
[FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-37; its literal-operand rule
is AC-16. The replay projection refers to those criteria rather than redefining them. This
slice adds no refusal-cause equality, admitted-charge sequence or consumed-counter equality.
The original limits remain inputs so the driver and QSL execute the proving context.

The current profile comprises only `quire.op.integer.add`, `quire.op.integer.sub`,
`quire.op.integer.mul` and `quire.op.integer.negate` over the persisted `i64` operands.
These operations are mathematically defined for every admitted operand vector. An observed
`Undefined` therefore cannot agree with this profile; like `Incomplete` or an execution fault,
it does not conform to the current harness's `_ => false` assertion and cannot become
harness-defect agreement. A generic upstream undefined-agreement rule for other operators
does not expand this profile. Adding an operator or another parity projection requires its
own generation and replay acceptance criteria.

`run_terminal_value` and absent-settlement behavior remain owned by
[FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md). CG-only malformed/decode
and assumption-domain setup failures retain its `CgDefect` reading. Unsupported-capability
refusal returns no settlement. After successful full-claim comparison, QSL `Diverged` and
`GeneratedFault` read as `Failed`; `Agrees` reads as `Inconclusive(ScalarAgrees)`; `RefusedInput` reads as
`Inconclusive(ReplayRefused(invalid_runtime_input))`; exact `Incomplete` reads as
`Incomplete(ResourceExhausted)`; other QSL `Refused` follows the existing typed QSL refusal
mapping and preserves its catalog code for non-fault refusals. Source observation, owned by QSL: after context/site/identity verification, merged
`scalar.rs::compare` handles native `Incomplete` or `ExecutionFault`
as `GeneratedFault` before its operand-admission backstop. Thus a native fault plus an
out-of-range operand reaches `GeneratedFault`, not `RefusedInput`; CG setup still admits
operands before dispatch. This internal QSL ordering is attribution only, not a CG
acceptance obligation or a change to FR-357. Native `Incomplete` is not exact resource exhaustion. CG's scalar settlement representation must
preserve these distinctions; it must never yield `Reproduced`, `Refuted` or `Verified`.

## Scalar Obligation Identity

CG obtains the scalar claim identity from the admitted IR reader's FR-322 structural `NodeKey`/`CheckedNodeId` for the original application and retains the authentic expression occurrence from that read. CG typed generation supplies the scalar kind and positional operand identities, ranges and provenance; CG retains their complete checked claim body for comparison. The identity is compared as an exact string or structural value; CG does not derive an obligation hash or encode a parity preimage. QSL-1293 owns removal of the old shared mint and digest check. A different package, path, or producer ordinal yields a different structural identity; a body-only change under the same identity is refused by the retained-body comparison.
`operator_parity.rs` recompiles the original context, checks the application node/operator and
selected enclosing-function membership, authenticates the occurrence and positional operands,
then compares the claimed structural identity and complete retained claim body before scalar comparison. A mismatched identity or body refuses with the owning typed stale-dependency/revision-mismatch result; there are no claimed or recomputed digests. This is source inspection, not a passing-test claim. Actual
CG/QSL conformance remains planned. Function/clause carry-only contracts and function/frame
preimages remain unchanged.

The following CG-owned allocation defines the closed scalar wire vocabulary. It is a new
specification allocation, not a member already present in `ScalarObligationIdentity` or the
clause-role `ObligationKind`. QSL carries this string opaquely.

| Scalar family actually minted by this route | O-09 `obligation_kind` wire value | Authority/source |
|---|---|---|
| Integer arithmetic lowering parity (`add`, `subtract`, `multiply`, `negate`) | `operator_parity` | This FR-032 allocation; CG generation shall supply a typed family value and retain it in the proving record. Its typed implementation/accessor is CODE-gated and unrun. |

This is the sole current scalar-family value. CG shall not derive it from `operation_identity`,
reuse a precondition/postcondition/invariant/frame role, or accept a caller-chosen free string.
Future parity families require their own owning allocation; the composite family is specified
separately. Current CG has no typed scalar-kind source. Until generation implements and retains
this allocation, the route returns the named unavailable-capability refusal without minting an
identity.

**Scalar identity conformance.** For an admitted current-run scalar claim, the replacement AC-15/AC-16 checks use the authoritative IR-read application identity and expression occurrence together with the CG-generated and retained scalar kind and ordered operand identities and ranges, then require the actual QSL operator route to accept the matching structural claim. The adverse controls change one included member at a time, retain
repeated operand positions and refuse invented occurrence or operand membership. A synthetic
encoding example alone is not evidence of authoritative membership, generated metadata retention
or passing routed replay.

The scalar subject is the claimed operator's FR-322 application node id and its authentic
O-07 occurrence key: that node id with the selected `CheckedOccurrence`'s role `expression`
and ordinal. The retained checked claim contains that subject, occurrence, the scalar obligation-kind value from the allocation above, and positional `arguments`. There is **one argument per operand position**, in operand/harness draw order.
Each argument contains its position ordinal, typed `OperandIdentity` and actual operand range
used by the harness. The identity is a tagged representation: `GraphChild { node_id }` for an
actual graph reference (including a parameter or subterm), or `InlineLiteral { node_id,
occurrence_key, position }` for an inline literal, using the authentic application node id,
selected occurrence key and operand position ordinal. An inline literal's singleton range
carries its value (FR-015 AC-16); QSL emits no extra literal node for it. A reference to an
existing literal node is a graph child. These are canonical representation fields: the measured
Rust enum is `GraphChild(WireNodeId)` or `InlineLiteral`; QSL's encoder expands the inline
arm using the application, occurrence and position retained by the claim. Neither tag
fabricates a checked node id.
Repeated independent draws of the same parameter remain distinct position entries. CG shall
neither sort these scalar entries by parameter identifier nor deduplicate their node ids.
Unrelated enclosing-function parameters are absent. This positional scalar rule does not
change function/frame identity members, encoding or their existing identifier ordering.

CG shall obtain the graph-child node ids, selected occurrence and positional operand correspondence
from the authoritative checked/emitted proving package and retain them through generation and
proving. No generated symbol, operand index used as a node id, chosen occurrence ordinal or
application id mislabeled as `GraphChild` substitutes for that metadata. Position is an argument
member; the typed operand identifier holds the valid tagged representation above. The persisted
draw order and operator operand order must correspond exactly to the retained positional arguments.

Under FR-022's one-item-per-distinct-node generation, an authentic occurrence means the sole
`expression` occurrence of that application in the proving package. Zero or multiple such
occurrences yields typed occurrence-unavailable/ambiguous setup refusal and no identity; CG
shall not pick the first or lowest ordinal. Replay of multi-occurrence nodes is CODE-gated on
an owning generation change that selects and retains the proved item's occurrence. A node's
having several valid QSL occurrences alone supplies no selection provenance.

CG shall take the application identity and authentic occurrence from the admitted IR reader.
CG typed generation shall supply and retain the scalar kind, positional operand identities,
ranges and proving metadata for the complete checked claim body. The closed claim members are the application `node`,
authentic `occurrence_key`, typed `obligation_kind` and ordered `arguments`; each argument
retains its position, tagged operand identity and actual harness range. A graph operand uses
its checked graph-child node; an inline literal uses the application, occurrence and position,
with its value in the singleton range. Position remains numeric and draw ordered; two draws
of one parameter remain distinct. CG compares this complete retained body structurally
against the sent claim and returned report. It does not sort or deduplicate positions, compute
canonical obligation bytes, or treat equality of the node identity alone as claim admission.
Source spans and native outcome are not members of the checked claim; the full report separately
carries operator, decoded values, result range, limits and same-artifact observation. A body-only
change with the same structural node identity must refuse at the body comparison.

Measured IR `CheckedPackageV2::graph()` exposes nodes with authoritative `node_id` and
`occurrences`, and `source_map()` exposes their source map; `CheckedOccurrence` supplies role
and ordinal. The current CG scalar record does not retain its selected expression occurrence
or typed operand identities. IR's public node `body` is still a JSON value; no public typed accessor
for the scalar application's ordered operands, their identities and ranges was measured.
The model-field accessor and wrapped optional operation-leaf walk do not supply that scalar
accessor. IR-648 must expose the same `GraphChild`/`InlineLiteral` identity distinction;
inline-literal identity uses the application/occurrence/position and requires no literal node.
CODE remains gated on [IR-648](https://linear.app/agent-ix/issue/IR-648), the typed scalar
operand accessor on `CheckedPackageV2`, and on retaining the occurrence selected for the proved item. Missing or ambiguous occurrence, operand identity/range
or authoritative scalar kind yields typed setup refusal and no minted identity. The positional rule resolves
literal and repeated-parameter semantics; no ambiguity gate for those semantics remains.
The merged QSL verifier authenticates the occurrence against the application, operand count,
and each graph-child reference at its position. An inline literal must be the actual inline
integer at that position, with its observed value and singleton range; typed mismatches refuse
before structural claim comparison. A graph-child identity check does not itself validate that child's
claimed integer range against its declared proving domain. CG shall retain and validate the
actual harness ranges against the checked/emitted proving schema, retaining their positional
provenance. A parameter or subterm typed by its own `bounded_domain` uses that bound's
checked derived integer interval; its bound must belong to the lowered claim's checked bounds.
An inline literal or referenced literal-value node uses its actual singleton integer value.
For a subterm with no own bound and no literal value, the current generation authority uses
the enclosing scalar claim's first checked result bound, rather than infer a new subterm range.
The replay record must retain that exact result-bound provenance and interval. The same rule
applies to any other operand lowered through the result-bound case. No wider interval, caller
hint or freshly inferred child domain substitutes for the range the harness actually assumed.
If typed access cannot recover that provenance, the route refuses under the IR-648 gate;
this specification neither broadens generation nor changes its result-bound rule. IR-648's authentic metadata
access and CG retention remain CODE gates. Missing or malformed authoritative identity or retained body returns typed setup refusal before replay; no invented identity or sentinel is admitted.

Decoder and original-limits context work can proceed independently.

## Converter Boundary and Original Context

The planned converter accepts the actual immutable public `OperatorParityReport` together
with CG's retained binding and checked driver-completion context; absent report means no
settlement under FR-029 AC-15. It reads `report.claim()`;
it does not construct, deserialize or alter QSL's private report fields. The retained binding
holds the expected sent `OperatorIdentity` separately, so mismatches are observable at this
public boundary without a fabricated report.

`OperatorIdentity` and the structural node identity alone do not bind the replay wire's package, selected function, source
bytes or dependencies. In addition to full-claim equality, the converter shall compare the
actual wire used for that run, recorded by checked driver completion, with the original
proving `call_site` context retained by CG: package identity/reference, selected enclosing
function, dependencies, source byte provision and original limits including `ReplayLimits`.
An equal claim replayed against another internally consistent package or enclosing function
must refuse with no settlement. The driver must record the actual sent context, not echo an
expected claim/context supplied afterward. This checked completion/retention representation
is a CODE gate; no new QSL identity field, tracking hash or public constructor is claimed.

## Setup Refusal Precedence

The builder uses the first applicable refusal in this order, even while the upstream gate holds:

1. Original proof-context, node/operator/schema, package/source and retained staged-byte
   coherence, including retention of the original scalar limits.
2. Selected playback byte ceiling, checked before parsing through the Kani output adapter.
3. Adapter selection and decoding: assertion block, qualified harness name, arity, byte width
   and persisted operand ordering.
4. Decoded operand-domain validation against the original schema.
5. Availability of the typed IR-648 operand/provenance accessor, authoritative scalar-kind
   value/source, retained unique expression occurrence and proving-context/limits record,
   and checked driver observation/context-completion seam.

An otherwise valid request reaches unsupported-capability at step 5 only when at least one
named capability is unavailable. Once those capabilities are available and the prior setup
checks succeed, the builder shall return an admitted replay plan; completion with authenticated
observation and actual original context shall enable replay and conversion. A permanently
refusing builder after these conditions hold cannot satisfy this requirement.
A malformed request reaches its earlier typed refusal; the absent capability never hides it.
After plan completion, the native-observation/staged-byte tie is checked before QSL evaluation.
Cross-binding converter failures return no settlement rather than settling another obligation.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-032-AC-1 | PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). A public consumer passes the routed harness's `ScalarObligationIdentity` directly to the scalar builder; its typed call constructs neither `KaniObligationIdentity` nor V1 `ReplayPackage`. After the ordered setup checks, an otherwise valid request returns unsupported-capability only while a named step-5 capability is unavailable; when all are available it returns an admitted plan, and authenticated completion enables replay/conversion. Conversion of a capability refusal returns no settlement rather than a synthetic function request or invented terminal cause. | Test |
| FR-032-AC-2 | PLANNED (IR-631). The builder decodes actual routed-harness playback in persisted symbolic call order through the Kani output adapter; another harness name, missing/extra operand, wrong byte width, playback above the builder-owned 8 MiB per-harness ceiling or malformed assertion block yields the distinct typed refusal under the stated precedence before replay. | Test |
| FR-032-AC-3 | PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). Each operand at its own lower or upper bound is admitted and the adjacent out-of-domain value is refused before replay. The admitted QSL inputs preserve the catalogued operator, operands, operand ranges, result range and harness limits. Another node/operation, altered operand schema or replay limit differing from the retained original generation context refuses; operand correspondence uses [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-16. | Test |
| FR-032-AC-4 | PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). The driver's actual native observation of the proved generated oracle is required and tied to this operand vector, limits and retained staged artifact bytes; checked completion requires driver authentication. A bare outcome echo, missing observation, another artifact's observation or fresh regeneration that removes the arithmetic mutation refuses before QSL evaluation. Playback bytes and Kani falsification alone cannot supply a generated outcome. | Test |
| FR-032-AC-5 | PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). The public consumer executes the same proved generated artifact and invokes QSL between the builder and converter. QSL compares its actual native observation with authoritative exact operator evaluation using the four-operator projection owned by [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-37, without adding refusal-cause or accounting-counter equality; changing the generated arithmetic while keeping original node/operator identity changes the measured parity result. `Undefined`, `Incomplete` and execution faults cannot be agreeing outcomes of this profile. No CG-local evaluator or replay verdict replaces QSL. | Test |
| FR-032-AC-6 | PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). For every `Diverged`, `Agrees`, `GeneratedFault`, `RefusedInput`, exact `Incomplete` and other `Refused` outcome, the converter compares the report's full claim to the retained sent claim and actual sent wire against the original proving context before conversion; a changed package/function/source/dependency or original limit refuses even with an equal claim. At the immutable-report/retained-binding boundary, an absent report, another result/run, or changing only the obligation, node, occurrence, kind, operator, operand identity/value/range, result range, limits or native outcome yields typed binding refusal and no settlement. Obligation identity equality or agreement alone cannot pass. | Test |
| FR-032-AC-7 | PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). QSL's divergence between generated and authoritative exact outcomes becomes a CG lowering fault and `Failed`; agreement despite Kani falsification becomes `Inconclusive(ScalarAgrees)`; native `Incomplete`/execution fault becomes `GeneratedFault` and `Failed` while exact `Incomplete` becomes `Incomplete(ResourceExhausted)`. `RefusedInput` preserves `invalid_runtime_input`; other typed QSL refusals follow the existing refusal map and non-fault refusals retain their code. This scalar route never returns `Reproduced`, `Refuted` or `Verified`. | Test |
| FR-032-AC-8 | PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). The real QSL-emitted `x + 1` with input `Int[0,9]` and result `Int[0,10]` exercises routed generation, actual Kani arithmetic-mutation playback and same-proved-artifact native observation. A separate bounded-addition fixture with both operands and result in `[-1000,1000]` exercises an admitted operand pair whose exact result is outside the result range. The expected exact result is derived from the actual retained playback, and its correct refusal is never labelled a source violation. The fixture ranges and witness domain/result-range relation are asserted before that observation; no unreachable case or assumed solver choice counts as coverage. | Test |
| FR-032-AC-9 | ~~Retired hash-preimage criterion.~~ Its authoritative member-selection, positional-order, distinct-draw, missing-provenance and changed-member checks are carried by FR-032-AC-15. The retired identity-digest oracle is not reused. | Inspection |
| FR-032-AC-10 | ~~Retired shared-mint criterion.~~ IR-reader structural identity and actual QSL route conformance are carried by FR-032-AC-16. The retired parity encoder is not called. | Inspection |
| FR-032-AC-11 | ~~Retired claimed/recomputed-digest criterion.~~ Exact identity and retained-body mismatch refusal before comparison is carried by FR-032-AC-17. | Inspection |
| FR-032-AC-12 | PLANNED (IR-631) / CODE-GATED (typed scalar-family/metadata retention and actual CG/QSL route). When a structurally self-consistent claim invents an application occurrence or operand child, or misstates an inline literal's value/singleton range, the consumer observes the corresponding merged typed membership refusal before structural claim comparison; a self-consistent string cannot admit invented metadata. | Test |
| FR-032-AC-13 | ~~Retired preimage-encoder failure criterion.~~ Missing or malformed authoritative identity/body refusal with no fabricated identity is carried by FR-032-AC-18. | Inspection |
| FR-032-AC-14 | PLANNED (IR-631) / CODE-GATED (actual CG scalar builder/converter). Adding the scalar builder/converter preserves existing function/clause carry-only behavior and function/frame identity encoding; none is redirected through scalar parity or changed by its positional argument rule. | Test |
| FR-032-AC-15 | PLANNED / CODE-GATED. The admitted IR reader supplies the original application structural identity and authentic expression occurrence. CG typed generation supplies and retains the scalar kind and one argument for each operand position in actual draw order, with typed graph-child or inline-literal provenance and the harness range. Two draws of one parameter remain two entries. Changing a package, path or producer ordinal changes the structural identity; changing an included body member while the identity stays equal is caught by complete retained-body comparison and refuses. A source-span or native-outcome change alone does not alter the claim identity. Missing or ambiguous occurrence, operand provenance or scalar-kind retention refuses before replay; no identifier sort, node deduplication, invented ordinal or unrelated enclosing parameter passes. | Test |
| FR-032-AC-16 | PLANNED / CODE-GATED. An independent fixture obtains the same application `NodeKey`/`CheckedNodeId` from IR admission and CG retention, checks the exact structural identity and complete positional body, and the actual QSL operator route accepts the matching claim. No CG-local identity derivation, copied encoder, stand-in verifier or synthetic acceptance passes. | Test |
| FR-032-AC-17 | PLANNED / CODE-GATED. With a valid original claim retained, altering only the claimed package, path or producer ordinal, or changing one otherwise membership-valid retained body member while reusing the old structural identity, produces a typed stale-dependency/revision-mismatch refusal before scalar comparison. A body-only edit cannot pass because the node identity is equal. No claimed or recomputed digest is required. | Test |
| FR-032-AC-18 | PLANNED / CODE-GATED. If the admitted IR read lacks a valid application identity or authentic occurrence, or CG typed generation lacks a required retained body member, the CG builder returns typed setup refusal without replay or fabricated identity. A caller string or usable sentinel cannot replace the absent authority. | Test |

## Intent and Existing Coverage

This table records the authoring gap analysis, not a computed test matrix.

| Intent | Existing authority and measured tests | Remaining gap |
|---|---|---|
| Real routed scalar generation | [FR-022](../../routed/functional/FR-022-routed-generation.md) AC-2/10/13/15; `tests/it/routed_generation.rs` carries those trace tags | The returned scalar identity has no public replay binding |
| Typed ordered playback and domain validation | [FR-016](./FR-016-witness-native-replay.md) AC-1/2/5/8; surviving `skeleton_spine.rs` tests exercise native replay with a direct identity fixture. The removed `kani_witness_join.rs` and `kani_argument_order.rs` were V1-only historical controls. | Direct scalar schema and operator-level replay inputs |
| QSL refusal and terminal reading | [FR-016](./FR-016-witness-native-replay.md) AC-9/10/11/12 and [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-1/10/13; `skeleton_spine.rs` and `terminal_map.rs` cover existing replay paths; AC-12 remains planned in the replay index | Planned checked scalar conversion to the measured QSL causes; predicate violation/parity readings do not establish scalar semantics |
| Missing settlement produces no terminal value | [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-15; `tests/it/terminal_map.rs` | Reuse in the routed integration without duplicating the terminal requirement |
| Driver calls QSL between public typed builder and converter | No scalar requirement or test on the inspected source | This requirement and planned [TC-047](../matrix/TC-047-routed-scalar-replay-binding.md) |

## Prerequisites and Implementation Status

The generation authorities are [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
AC-37 and AC-16. Existing source-inspection and real-backend tests of those criteria are not
new scalar-replay coverage. The input limits-context record and scalar decode ceiling are new
adapter obligations; no such context is claimed present in `ScalarObligationIdentity` today.

The emitted increment fixture's result range is `Int[0,10]`, as
[FR-022](../../routed/functional/FR-022-routed-generation.md) AC-15 and
`tests/it/routed_generation.rs` record. It supplies no result-range-refusal witness. The separate
addition descriptor already accepted by `tests/it/kani_obligations.rs::scalar_package` has
operand/result bounds `[-1000,1000]`; `(600,600)` giving `1200` illustrates a reachable
out-of-result-range exact result, without prescribing Kani's chosen witness. The check derives
its expected exact result from the actual retained operand pair. Its QSL source counterpart and
original `call_site`
package/source binding must be admitted under the same upstream gates as the positive replay;
no hand-edited emitted package or fabricated source identity substitutes for that admission.

The merged QSL `FR-357` and `qsl-replay/src/scalar.rs` expose
`replay_operator_parity`, `OperatorClaim`, `OperatorParityReport`, `NativeOutcome` and the
scalar result/terminal readings above. The public facade call is
`replay_operator_parity(wire, claim, replay_limits)`, with `ReplayLimits` last as amended
by merged QSL change #645. That input-byte guard is separate from the claim's `ScalarLimits`
for exact evaluation; `ReplayLimits` is not an `OperatorIdentity` member. Retain both original
contexts rather than invent a claim field or use an unretained default. The operator entry recompiles the original source,
checks `package_id`, and validates scalar-node/operator membership in the selected enclosing
function; it does not replay that function as a predicate. `OperatorClaim.identity` carries the obligation identity; the driver separately retains
the staged artifact bytes used to authenticate its native observation.
On measured merged QSL change #650, `OperatorParityReport::claim()` exposes
`OperatorIdentity` on every outcome and `ValueParityReport::claim()` exposes `ValueIdentity`;
FR-357 AC-13 specifies full-claim retention. The
operator report retains the obligation, node, operator, operands/ranges, result range, limits,
occurrence and kind even when request decoding or context validation refuses. The current full claim also retains the native outcome; an obligation-only comparison cannot pass. This is source inspection,
not a passing-test claim; no QSL or CG scalar tests were run for this SPEC change.

The positive operator-level route has these explicit **CODE gates**:

1. CG's consumer must conform to the merged full-claim report API. For this operator slice
   the full claim is the retained obligation identity, scalar node/occurrence, kind,
   operator, ordered operands with identities/ranges, result range, retained limits and native
   outcome. Compare it before settlement on every outcome. The driver separately compares its
   original artifact bytes with the receipt before checked completion; QSL never receives them.
   The merged function-parity report does not authorize CG function-parity implementation here.
2. CG generation must retain the authoritative scalar metadata needed by the positional
   checked claim body and original renderer-limit/proving record. Unique authentic occurrence, typed positional operand identity/range/provenance access
   (IR-648) and the typed scalar-family implementation/retention remain explicit gates.
   CG must take the structural application identity and authentic occurrence from the IR reader,
   retain the typed scalar kind and positional body from CG generation, and compare the complete body even when the node identity is unchanged, and prove matching-claim
   acceptance and mismatch refusal with actual QSL. CG must validate retained graph-child ranges
   against the proving schema.

3. The driver must implement same-proved-artifact execution and observation authentication;
   CG must supply the retained generation/proving record and checked completion/conversion seam,
   including the actual sent original replay wire/context, independently of full-claim equality.
   QSL never sees or authenticates the artifact; carried structural identity equality alone is insufficient.
4. CG's converter/settlement representation must consume the measured legal scalar terminal
   readings only after full-claim and retained-run comparison. Existing predicate disagreement
   or a fabricated scalar cause must not stand in for that conversion.
5. AD-002 R-7 stays in force: retain the **original** proving `call_site` package identity and
   source bytes; QSL recompiles them, checks the same `package_id`, and validates scalar
   node/operator membership in the selected function. Retain the proving source/package tie;
   neither fresh oracle generation nor a synthetic predicate selection substitutes for it.

AD-002's existing CG replay-call list still constrains CG: the planned builder makes no execution
call, and the driver executes the generated artifact and separately calls QSL's public scalar replay
facade. This contract neither adds a fourth CG executor nor claims a new call-site selection. The merged operator facade retains R-6/R-7; actual CG and driver conformance remains gated,
and this document grants no architecture exception.

All acceptance criteria are **Planned**. AC-1's admitted call, AC-3's admitted QSL inputs and
AC-4 through AC-14's completed scalar route are additionally **Gated** on the above contracts;
no test coverage is claimed. Unsupported-capability refusal, ordered decoding and operand
validation can be implemented independently. The merged function-level declared-parameter parity route is outside this operator-level CG contract.

The builder's operand-domain check precedes every replay, as
[FR-024](./FR-024-counterexample-envelope-intake.md) already requires. A decoded value outside
the Kani assumed domain is a CG setup defect and `Failed`. QSL's own range validation is an
envelope backstop whose non-fault refusal keeps its QSL code; it does not authorize bypassing
the CG check. The existing envelope and architecture requirements remain authoritative while the
scalar route is gated.

## Dependencies

- **Upstream**: [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) owns the proved
  projection and literal bindings; [FR-017](../../kani/functional/FR-017-kani-execution-evidence.md)
  owns capture bounds; [FR-022](../../routed/functional/FR-022-routed-generation.md) supplies scalar
  identities; [FR-024](./FR-024-counterexample-envelope-intake.md) owns QSL envelope submission;
  [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) owns terminal readings;
  CG owns the scalar structural claim retention and producer/converter contracts; the driver owns observation
  authentication; QSL-641 owns the amended full-report claim API. Measured QSL FR-357 owns exact
  evaluation, original-package validation and scalar terminal causes. IR-648 in Contract IR
  owns the typed `CheckedPackageV2` scalar operand accessor and blocks IR-631 CODE admission;
  its spec and implementation must be measured before that accessor gate is closed.
- **Related**: [FR-016](./FR-016-witness-native-replay.md) owns existing playback/function replay
  rules; [AD-002](../../assurance/AD-002-cg-qsl-replay-seam.md) retains QSL evaluation ownership.
- **Downstream**: [TC-047](../matrix/TC-047-routed-scalar-replay-binding.md), planned public
  consumer and real-backend evidence for IR-631; driver E9 calls the admitted QSL seam once
  its prerequisites are resolved.
