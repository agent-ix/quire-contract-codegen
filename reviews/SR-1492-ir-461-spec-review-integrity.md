---
id: SR-1492
title: "IR-461 integrity analysis: state and frame disposition vocabulary and mapping"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@b094c9cf5685022a41f7723f12d318f4c7643997; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/core/functional/interface-001-codegen-api.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

# SR-1492: IR-461 integrity analysis

## Summary

Ticket: IR-461, PR #275 at b094c9c. This analysis checks the new FR-015 Behavior paragraph,
FR-015-AC-59 to AC-65 and the interface-001 entry against what the rest of FR-015 says and against
the code on origin/main 7345463: `src/kani/generate/frame.rs`, `outcome.rs` and `negotiate.rs`, and
`CompleteLoweringRecordV2` at the locked quire-contract-ir rev dec8ade.

The crate already has one disposition vocabulary. `ObligationDisposition` (outcome.rs:330) has
`supported`, `requires_bound`, `unsupported` and `invalid_request`, with
`UnsupportedObligation::NoFiniteEncoding` as a reason. The routed `Disposition`
(src/routed/capability.rs:333) uses the same four names. The PR adds a third vocabulary instead:
generated, `requires_bound`, `no_finite_encoding` and `refused`. Its mapping also disagrees with how
the crate already uses `NoFiniteEncoding`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new four-way disposition (generated, requires_bound, no_finite_encoding, refused) and `StateFrameRecord` duplicate `ObligationDisposition` and `ObligationRecord` (outcome.rs:330, :355). Both existing types already carry per-item, request-ordered dispositions with typed reasons. `no_finite_encoding` is an existing reason code (`UnsupportedObligation::NoFiniteEncoding`, serde `no_finite_encoding`), not a disposition, so the PR promotes a reason to a disposition. The Behavior paragraph says the lane works "as the negotiation of FR-015-AC-23 does", then defines different names. Fix: reuse `ObligationRecord`/`ObligationDisposition`. A generated clause is `supported`; `NoFiniteEncoding` and the other grounds go under `unsupported` with a reason; malformed requests are `invalid_request`. | spec/kani/functional/FR-015-bounded-kani-obligations.md:278-312, spec/core/functional/interface-001-codegen-api.md:109-112 |
| FND-002 | high | FR-015-AC-62 records a negation, a literal comparison, a two-field comparison and two reads of one side as `no_finite_encoding`, and AC-63 puts a lowering that is not requires-bound under `refused`. That inverts the crate's meaning. A negation or a two-field integer comparison has a finite encoding: planned FR-015-AC-40 encodes `not` and the integer comparisons for the same postcondition `state_clause` node on the V2 clause arm. This generator just does not render them (`ConditionNotSupported`), which the crate elsewhere calls not-rendered (`OperationNotRendered`). Meanwhile negotiate.rs:612-618 maps a lowering `Unsupported` family, which is exactly `NotLowered{Unsupported}` here, to `NoFiniteEncoding`, and AC-63 calls that `refused`. A proof gate (QSL-20) reading `no_finite_encoding` would treat a renderable construct as unprovable in principle. | spec/kani/functional/FR-015-bounded-kani-obligations.md:379, spec/kani/functional/FR-015-bounded-kani-obligations.md:380, spec/kani/functional/FR-015-bounded-kani-obligations.md:298-302 |
| FND-003 | medium | The refusal-to-disposition mapping is stated as prose shapes plus an "any other ground" catch-all, not per `StateFrameRefusal` variant, and it is not unambiguous. (a) `RecordSerialization` is named nowhere. (b) `ConditionNotSupported` covers the unsupported shapes, but it is also returned when an operand node cannot be followed in the graph (frame.rs:663-665, :669-671, :713, :717-720), which is a malformed clause; AC-62 sends one variant to `no_finite_encoding` and the prose would send the malformed case to `refused`. (c) `NotLowered` splits by its `CompleteLoweringRecordV2` arm (RequiresBound vs Unsupported, InvalidInput, InvalidBody, BodyIncomplete, Failed), and the arms are not listed. (d) Nothing forces an update when a variant is added. Fix: add a table keyed on all 17 variants, plus the `NotLowered` record arms, to one disposition each. Have the AC require the mapping to be an exhaustive `match` with no wildcard arm (`StateFrameRefusal` is not `#[non_exhaustive]`, so a new variant then fails to compile), and have AC-65's test iterate every variant. | spec/kani/functional/FR-015-bounded-kani-obligations.md:283-285, spec/kani/functional/FR-015-bounded-kani-obligations.md:303-307 |
| FND-004 | medium | `StateFrameRecord` and `StateFrameBatchError` are named only in interface-001, and FR-015 Outputs is not updated. None of the following is pinned: a record's fields (request index? clause id as `CheckedNodeId`?), the disposition enum's name and serde spelling, where the two harnesses sit, what `requires_bound` carries (`ObligationDisposition::RequiresBound` carries `unbounded_type`; `BoundNotResolved` carries a field name), or the error's variants (compare `KaniObligationError::EmptyRequest`/`TooManyItems{count}`). A coder would have to invent the shape that QSL-20 consumes. Reusing `ObligationRecord`/`KaniObligationError` (FND-001) settles most of this. | spec/core/functional/interface-001-codegen-api.md:109-112, spec/kani/functional/FR-015-bounded-kani-obligations.md:381 |
| FND-005 | medium | The batch does not say what happens to two requests naming the same clause, or to requests from different packages. Harness paths derive only from the clause and anchor digests (`post_<clause12>`, `frame_<anchor12>_<clause12>`, frame.rs:378-392). So the same clause twice, for example with different `state_fields`, yields two `generated` records with different harness bytes at one path. negotiate refuses this with `DuplicateItem` and `MixedBoundPackages`/`PackageMismatch`. State a rule, such as duplicate refused or one package per batch. | spec/kani/functional/FR-015-bounded-kani-obligations.md:287-290, spec/kani/functional/FR-015-bounded-kani-obligations.md:376 |

## Verdict

Request changes. FND-001 and FND-002 are the structural problem. FND-003 to FND-005 follow from it,
and most of them close once the batch reuses `ObligationRecord` and `ObligationDisposition`.

The facts the PR measured about the current code are all correct (see SR-1491).

## New findings (disposition pass 1)

Reviewed at 0eaed13b80be44d62145ef0970b949418a3ccacb.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | The table maps `BoundNotResolved` to `requires_bound` with `unbounded_type` set to the framed object's node, saying the field has no type node of its own. That is wrong in two ways. (a) The member's `value.target` is a type node: in the AC-27 fixture an unbounded `balance` member references the plain integer type `key(T_INTEGER)` (tests/it/kani_obligations_state_frame.rs:179-193). That node is the unbounded type; the object is not a type. (b) `BoundNotResolved` also covers a member bound that is not of `integer_range` form, or whose endpoints do not parse as `i64` (frame.rs:763-805). negotiate classifies those as `unsupported` (`DomainNotRepresentableInI64` or a non-symbolic domain), not `requires_bound`. Fix: name the member's type node, and split out or map the non-i64 and non-range cases. | spec/kani/functional/FR-015-bounded-kani-obligations.md:302, spec/kani/functional/FR-015-bounded-kani-obligations.md:415 |
| FND-007 | medium | The author left `FrameEffectUnsupported` to the owner; the table currently maps it to `NoFiniteEncoding` naming the frame node with family `state`. I recommend `StateFrameRefused` instead. The crate's `NoFiniteEncoding { node_id, node_tag }` means a reachable node *family* has no finite encoding (outcome.rs:255-262, negotiate.rs:612-618). The `state` family of a frame is exactly what this arm encodes, so the reason would state something false. The mapping also drops the effect (Creates, Deletes, Relationship, ForeignField). `ForeignField` is finitely encodable and is only outside this single-struct ABI, which is the same argument the PR now accepts for condition shapes. The enum's own doc says "a frame effect this generator has no finite encoding for", which is the not-rendered sense. If the owner wants `NoFiniteEncoding` for relationships, it should name the `relation` node with family `relation`, not the frame. | spec/kani/functional/FR-015-bounded-kani-obligations.md:304, spec/kani/functional/FR-015-bounded-kani-obligations.md:416 |

## New findings (disposition pass 2)

Reviewed at 9ffcba22c884118b6961c3f5964d607dea50baf1.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | high | AC-60 contradicts the new role independence. AC-60 says every yielding `StateFrame` item's harness is byte-identical to the harness `generate_state_frame_obligations` returns for that clause and role. FR-015 line 291 and interface-001 keep that function returning on the first refusal of the clause. In AC-68's cases, a frame granting every field leaves `contract` supported and a negated condition leaves `frame` supported, but the engine returns `Err(NothingForbidden)` or `Err(ConditionNotSupported)`. So there is no harness for AC-60 to compare with, and AC-68's "the harness the engine would build" is undefined. Fix: name the per-role engine function the role split adds (one role in, `Result<StateFrameHarness, StateFrameRefusal>` out). Make it the oracle of AC-60 and AC-68, and state that `generate_state_frame_obligations` is both roles' `Ok`, else the first refusal. | spec/kani/functional/FR-015-bounded-kani-obligations.md:457, spec/kani/functional/FR-015-bounded-kani-obligations.md:465, spec/kani/functional/FR-015-bounded-kani-obligations.md:291-292, spec/core/functional/interface-001-codegen-api.md:105-108 |

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 0eaed13b80be44d62145ef0970b949418a3ccacb: "no public entry and no disposition type: the records are `ObligationRecord`s, the dispositions are `ObligationDisposition`'s four" |
| FND-002 | fixed | 0eaed13b80be44d62145ef0970b949418a3ccacb: condition shapes map to `StateFrameRefused`, "Not `NoFiniteEncoding`: these shapes have a finite encoding (planned FR-015-AC-40 …)", and `NotLowered` (`Unsupported`) maps to `NoFiniteEncoding` as negotiate does |
| FND-003 | still-open | The variant-keyed table, the no-wildcard rule and AC-66 landed, and `RecordSerialization` and the `ConditionNotSupported` split are settled. But the table covers only six of the seven `CompleteLoweringRecordV2` arms: `NotLowered(Lowered)` has no row. A no-wildcard `match` must handle it, so the coder would invent a mapping or an `unreachable!` panic. Give it a row, for example `unsupported` `RenderFailed` as an internal invariant, or narrow `NotLowered`'s payload type. |
| FND-004 | still-open | Records, dispositions and reasons are now pinned, with serde codes. Where a supported `StateFrame` item's harness goes is not: `KaniObligationOutcome::Emitted` has only `harnesses` (V1) and `scalar_harnesses` (outcome.rs:370-377), and the spec names no field for `StateFrameHarness`. interface-001's output is unchanged. AC-60 cannot be built without inventing this field. |
| FND-005 | fixed | 0eaed13b80be44d62145ef0970b949418a3ccacb: a repeat of the same clause and role is `DuplicateItem`, a different package is `MixedStatePackages`, and the outcome is `Rejected` (AC-64) |
| FND-003 | fixed | 9ffcba22c884118b6961c3f5964d607dea50baf1: `NotLowered` carries only the six refusal arms. A CG-side enum over IR's public field types expresses this with no IR change, and the IR enum is not `#[non_exhaustive]`, so the conversion match stays exhaustive |
| FND-004 | fixed | 9ffcba22c884118b6961c3f5964d607dea50baf1: `state_frame_harnesses: Vec<StateFrameHarness>` on `Emitted`, named in AC-60 and interface-001 |
| FND-006 | fixed | 9ffcba22c884118b6961c3f5964d607dea50baf1: unbounded target goes to `requires_bound` carrying `value.target`; a non-range or non-i64 bound, or a missing member, goes to `StateFrameRefused` |
| FND-007 | fixed | 9ffcba22c884118b6961c3f5964d607dea50baf1: `StateFrameRefused` carrying the effect, not `NoFiniteEncoding` |
| FND-008 | fixed | f287f534a1b80b1bd76aaa09dacbeab0c05f3804: `generate_state_frame_role(request, StateFrameRole)` is the comparison target of AC-60 and AC-68; `generate_state_frame_obligations` returns both roles' harnesses when both succeed and otherwise the first refusal; AC-60 ties both-success clauses to it and AC-68 reads the failing role's refusal from it; no comparison target is left undefined |
