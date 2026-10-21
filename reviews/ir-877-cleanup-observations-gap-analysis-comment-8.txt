<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15291 method=gap-analysis lang=rust pr=quire-contract-codegen#501 model=unavailable run=d9d507d8-42eb-4ddc-a8d0-3eed0b37342f -->

SR-15291 gap-analysis, part 8/18. FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-127 | high | coverage | Inherited FR-024-AC-40 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-128 | high | coverage | Inherited FR-025-AC-1 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-129 | high | coverage | Inherited FR-025-AC-2 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-130 | high | coverage | Inherited FR-025-AC-3 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-131 | high | coverage | Inherited FR-025-AC-4 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-132 | high | coverage | Inherited FR-025-AC-5 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-133 | high | coverage | Inherited FR-025-AC-6 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-134 | high | coverage | Inherited FR-025-AC-7 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-135 | high | coverage | Inherited FR-025-AC-8 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-136 | high | coverage | Inherited FR-025-AC-9 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-137 | high | coverage | Inherited FR-026-AC-1 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-138 | high | coverage | Inherited FR-026-AC-4 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-139 | high | coverage | Inherited FR-028-AC-5 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-140 | high | coverage | Inherited FR-028-AC-6 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-141 | high | coverage | Inherited FR-028-AC-7 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-142 | high | coverage | Inherited FR-028-AC-8 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-143 | high | coverage | Inherited FR-028-AC-9 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-144 | high | coverage | Inherited FR-028-AC-13 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Emit only `check_equality`, omitting the `check_type` calls; `check_equality` consults the
    environment solely through `contains_ieee`, which returns `false` for an absent composite key, so
    an empty environment completes with a Boolean instead of refusing. Or emit `.expect(..)`, which panics
    instead; count a private duplicate-member defense as public source-authenticated acceptance, or replace
    its precise private cause with a generic context mismatch; invent a public Assembly cause tag where
    CompileRefusal::cause() returns None.
- id: FR-018-AC-9
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Emit closures returning plain `bool`; a denial then has no representable result.
- id: FR-018-AC-10
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Order claim-map entries by the expression node id alone, which ties the two items of one node
    and lets a permuted request permute them; or change the operator one oracle emits; or swap which operand's
    runtime value or conversion target is evaluated as left versus right; or swap the emitted `left_source`/`right_source`
    descriptor itself.
- id: FR-018-AC-11
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Mark the operation checked rather than `caller_declared`; refuse the second descriptor as a
    duplicate; or build both symbols from the node's declaration name, so neither names its operator.
- id: FR-018-AC-12
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Emit the plan's pair count or a charge amount as a literal constant in the generated source.
- id: FR-018-AC-13
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Key declarations by request ordinal instead of by the V2 node id.
- id: FR-018-AC-14
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Treat reordered or malformed integer/collection bounds as reader-admitted generator inputs,
    count private parser defense as their public acceptance, or read an admitted bound under a wrong name/value
    so it becomes another range.
- id: FR-018-AC-15
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Read a conversion's own `result_type` or its first nested conversion's result as the source
    type, or read every application operand through to its first argument, so a descriptor naming the
    conversion target (or the inner result) as its source type is accepted over a body that converts from
    another type, or a `rational.div` operand is typed as its integer argument.
- id: FR-018-AC-16
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Read a `bounded_domain` member as its base scalar's unbounded type or through the union of
    every bound over that base, take the first sibling bound, or skip the form check, so a mismatched
    bound (`text_bounds` over an integer, a bound over a boolean) generates as an unbounded type instead
    of being refused, or refuse a `float_rounding` member as unsupported so the IEEE refusal never fires.
    Or count private two-form/malformed-member controls as genuine public acceptance, drop their exact
    assertions, or claim an unrelated sibling bound proves public member ownership.
- id: FR-018-AC-17
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Emit `.expect(..)` in the `integer` helper or in a rendered `RationalDomain`, `TextType`, `CardinalityBound`,
    `IntegerInterval` or `DecimalType` reconstruction, so a runtime release that tightens a constructor
    panics a generated oracle; or replace it with `.ok()` or `.unwrap_or(..)` to an unbounded type, so
    a failed reconstruction silently widens the compared type; or report it as an `InvalidDeclaration`
    with a cause the runtime does not state; or have a helper recover a failed constructor (`.or_else(..)`
    to an unbounded bound) or fail with another helper's variant.
- id: FR-018-AC-18
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Keep a defensive `ValueType::Quantity => unreachable!(..)` or `ValueType::Reference => unreachable!(..)`
    arm after the earlier refusal, so a change to that refusal turns a bad request into a generator panic.
- id: FR-018-AC-19
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Add an `assert!`, `debug_assert!`, `process::abort()` or `.unwrap_unchecked()` to the generator,
    or re-add an `.expect(..)` template, none of which a scan limited to `unwrap`, `expect` and four macros
    catches; or spell a panic so a substring scan misses it (`Option::unwrap`, `.unwrap ()`, a bare `abort()`
    after `use std::process::abort`).
- id: FR-018-AC-20
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Mislabel an admitted `Failed(Bytes)` record as work exhaustion, or synthesize a ContractPackage
    when Contract IR's byte ceiling returns no package so CG fabricates claims or artifacts.
- id: FR-018-AC-21
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'Record the closure from a second reading of the package (a second reader that reads presence
    differently), so the shadow of FR-015 and the production oracle disagree on presence and each is correct
    against its own reading. Member order is not a mutant: a permutation changes no compared observable.'
- id: FR-018-AC-22
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Refuse or stop a legal pair (an `absent` against a `null` slot, two `none`, a nested option)
    as `Refused(CheckedInvariant)`, charge one pair too many for a `some`/`some` option or too few for
    an unequal pair that stops early, or complete with the verdict of the other operator.
- id: FR-018-AC-23
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Check the closure against the generator's own reconstruction (the check of AC-21 alone), so
    a reader that narrows a bound or reads `T?` as `T` shrinks the shadow's domain, the refinement domain
    and the oracle's declaration together and every check agrees.
- id: FR-018-AC-24
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Recurse from `Option` into itself without tracking an unclosed type, or reject every repeated
    node including the List and Tree record back-edges; the former overflows the stack and the latter
    refuses valid recursive records.
- id: FR-018-AC-25
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Reuse the Contract IR lowering counter for reconstruction, charge only record declarations
    while skipping option and sequence edges, or recurse through 65,537 acyclic entries before checking
    the budget; a large item then completes or overflows the stack instead of yielding the named per-item
    refusal.
- id: FR-018-AC-26
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Expect `next`'s value to be a direct reference and reject QSpec's nested optional binding,
    or ignore the wrapper and treat the field as required; the genuine List item refuses or generates
    with the wrong presence. Or delete/soften a precise malformed-wrapper defense or call its private
    result public source/context acceptance.
- id: FR-018-AC-27
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Expect an inline second bounds reference in the Sequence body, or reject every bounded domain
    whose semantic type is not scalar; the QSpec Tree item refuses. Ignoring a missing bound instead generates
    an unbounded collection.
- id: FR-018-AC-28
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Keep a fixed 65,536-entry ceiling even after the caller raises the work limit, or use recursive
    Rust calls for each nested type; the 100,000-entry case refuses or exhausts the 512 KiB stack.
- id: FR-018-AC-29
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Detect the cycle before charging the repeated entry, or follow its child before checking the
    work limit; the boundary case reports the wrong refusal or visits a child that should not run.
- id: FR-018-AC-30
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Retain the fixed source ceiling, emit one nested constructor per type level, or measure bytes
    only after publishing; the 100,000-entry case refuses despite a sufficient caller byte limit, overflows
    a compiler stack, or leaks partial output.
- id: FR-018-AC-31
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Accept `u64::MAX` and increment a `u64` on its first denied charge; the reported count wraps
    to zero or the process panics.
- id: FR-018-AC-32
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Classify a requested formula or fairness root solely by its `temporal` tag and report the clause's
    QSL-121 blocker; classify a lowered `case` root as `FormMismatch`; or key the refused claim by the
    first unsupported child instead of the requested node.
- id: FR-018-AC-33
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Compare the context against lowered ContractPackage identity instead of original source package
    identity, skip full-package/source-lock authentication, or expose the retained context before those
    checks.
- id: FR-018-AC-34
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Build shape/index from only used literals, substitute positions for member keys, or conflate
    identical case names under distinct declarations.
- id: FR-018-AC-35
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Collapse Unknown case and declaration, drop Foreign package details, map Resource/Fault to
    Package, or publish a partial context after stopped admission.
- id: FR-018-AC-36
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Emit a NodeKey literal, accept an arbitrary caller index, skip check_type on the prepared environment,
    or evaluate a wrapper for another descriptor.
- id: FR-018-AC-37
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Add replay compilation to scalar proof execution, let scratch and emitted manifest sources
    drift, or count native enum success as an original Kani receipt.
- id: FR-018-AC-38
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Universally demand context before a key-free refusal, mistake standalone absence for source
    authentication, or return an empty index before complete closure resolution.
- id: FR-018-AC-39
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Check only an operand's outer type, skip a nominal conversion target/nested field, fabricate
    a key, or let capability checks reorder charged refusal/cycle precedence.
- id: FR-018-AC-40
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Treat Some(wrong) as None or skip the package guard because all selected types are primitive.
- id: FR-018-AC-41
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Build only used enum literals or recover from a failed nominal lookup with an empty index.
- id: FR-018-AC-42
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Drop private binding/context checks for None, allow a nominal recipe without context, omit
    check_type, accept a caller index, or make native authentication optional.
- id: FR-018-AC-43
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Narrow a direct canonical Integer operand or target using an unrelated sibling integer_range,
    infer member ownership from the shared base key, or label an erased source conversion as a retained
    conversion-node control.
- id: FR-019-AC-1
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Settlement dispatches one arm per variant of the closed backend kind, and every variant reaches
    an arm that settles rather than falling through.
- id: FR-019-AC-2
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Every requested item receives exactly one of `supported`, `requires-bound`, `unsupported` and
    `invalid-request`, settled from its `candidates` and extent classification, and `candidates` is read
    from the request rather than computed here.
- id: FR-019-AC-3
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: On the Kani arm, an unbounded extent against a `bounded`-only advertisement settles `requires-bound`
    when a finite bound is available, rather than `unsupported`; with no finite bound available it settles
    `unsupported`, warned, with `unsupported_projection`/`unbounded-extent`; and it never settles `supported`.
- id: FR-019-AC-4
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An item with more than one candidate and no named backend settles `invalid-request` with `invalid_capability`/`ambiguous-backend`,
    naming every candidate in candidate order, identically under every registration order.
- id: FR-019-AC-10
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Only an item settled `supported` routes.
- id: FR-019-AC-45
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'A manifest with a second descriptor of one backend identity causes a whole-envelope `EnvelopeRefusal::ManifestRepeatsBackendIdentity
    { identity: String }`, whether the descriptors are identical, differ in `ProviderOrigin` alone, or
    differ in any other CG descriptor field, and whether adjacent or non-adjacent. The call returns no
    item settlements. The variant has no string cause, manifest or digest payload; `negotiate_backend_provider`
    keeps its return signature.'
- id: FR-019-AC-46
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: With several repeated identities, the refusal names the bytewise-first repeated identity independent
    of manifest order; identities differing only in case are distinct. An invalid contract version or
    capability vocabulary is refused as `EnvelopeRefusal::InvalidCapability` before repeat detection or
    any kind read.
- id: FR-019-AC-47
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: QSL's registry snapshot holds each backend identity at most once (FR-290:258-265; FR-331:57),
    so no driver-built FR-331 manifest repeats an identity.
- id: FR-019-AC-48
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'A `Refused(refusal)` outcome from the current Kani oracle-function preflight read produces
    `Err(EnvelopeRefusal::ContractPackageReadRefused { refusal: ContractPackageRefusalProjection })` for
    the whole request, with the reader''s `code`, `path`, `cause`, `locus`, `contract_version`, `document_pointer`,
    `expected` and `actual` copied into the named CG projection; the private `expected_node_id` is absent.
    No item settlement or artifact is returned.'
- id: FR-019-AC-49
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'An `Incomplete(incomplete)` outcome from the current Kani oracle-function preflight read produces
    `Err(EnvelopeRefusal::ContractPackageReadIncomplete { incomplete: ContractPackageIncompleteProjection
    })` for the whole request, with the reader''s `limit_kind`, `limit`, `consumed` and `path` copied
    into the named CG projection. No item settlement or artifact is returned.'
- id: FR-019-AC-50
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An `Admitted` Kani function-application preflight read whose exact generator produces a claim
    for an otherwise supported item retains the existing `supported` settlement.
- id: FR-019-AC-51
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Among several request items with non-`Admitted` preflight outcomes, the first failing item
    in manifest/item request order determines the whole-request `ContractPackageReadRefused` or `ContractPackageReadIncomplete`
    variant and projection, irrespective of outcome kind. Earlier successful items contribute no returned
    settlement or artifact.
- id: FR-019-AC-52
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An `Admitted` Kani function-application read whose exact generator emits no claim, and a structural
    preflight rejection before any read, each retain item-level `KaniFormRefused { OperationNotRendered
    }` without a reader envelope refusal.
- id: FR-019-AC-53
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Given an admitted package with bound driver items at driver indexes 0 and 2 around an excluded
    accounting-only position at index 1 whose accounting is `lowered` with a Kani-supported form, an FR-331
    envelope with exactly two items settles the bound items without an `EnvelopeItemCount` refusal. Both
    `ContractItemSettlement.request_index` and its nested `ItemSettlement.request_index` equal 0 and 2
    respectively, not envelope indexes 0 and 1. The excluded position produces no settlement or claim,
    and its own classifier-call and Kani-preflight-read counts are both zero. An envelope with fewer or
    mo
findings:
- fnd: FND-127
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-024-AC-40
  related_ids: []
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  lines: '384'
  excerpt: A retained reduced `Witness` envelope holds a transcript from its own backend re-run, bound
    to its own reduced assignments; it cannot reuse any ancestor's transcript.
  finding: 'Inherited FR-024-AC-40 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-128
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-025-AC-1
  related_ids: []
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  lines: '94'
  excerpt: For every harness kind, the identifiers of the emitted `kani::any()` bindings, in source order,
    equal the identity's `arguments` identifiers in order, and those ascend by identifier.
  finding: 'Inherited FR-025-AC-1 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-129
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-025-AC-2
  related_ids: []
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  lines: '95'
  excerpt: Every argument binding in a persisted identity names its parameter node id and declared domain.
  finding: 'Inherited FR-025-AC-2 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-130
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-025-AC-3
  related_ids: []
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  lines: '96'
  excerpt: A Boolean argument is a `bool` that reaches the oracle unchanged, and a bounded-integer argument
    is an `i64` that reaches the oracle as `rt::Integer::from` of that value. The values `i64::MIN`, `-1`,
    `0`, `1` and `i64::MAX` each reach the oracle equal to their mathematical value.
  finding: 'Inherited FR-025-AC-3 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-131
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-025-AC-4
  related_ids: []
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  lines: '97'
  excerpt: An argument whose domain has no declared finite bound yields no harness and is never narrowed
    to a machine range.
  finding: 'Inherited FR-025-AC-4 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-132
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-025-AC-5
  related_ids: []
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  lines: '98'
  excerpt: A frame obligation is accounted `unsupported` with a typed reason and no harness.
  finding: 'Inherited FR-025-AC-5 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-133
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-025-AC-6
  related_ids: []
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  lines: '99'
  excerpt: A binding that names no parameter of the selected function, and a parameter of that function
    with no binding, are each refused at generation with a typed reason and no harness.
  finding: 'Inherited FR-025-AC-6 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-134
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-025-AC-7
  related_ids: []
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  lines: '100'
  excerpt: An argument of a family with no row in the Rust-type table (rational, decimal, IEEE, text,
    enum, composite, collection or function) is accounted `unsupported` with a typed reason naming the
    family, and no harness is emitted.
  finding: 'Inherited FR-025-AC-7 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-135
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-025-AC-8
  related_ids: []
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  lines: '101'
  excerpt: A harness over a state-reading obligation passes each state value to the subject as `&mut`
    to a harness-owned value and copies it before the call; the pre-state the harness asserts over is
    the copy, and the post-state is the value after the call.
  finding: 'Inherited FR-025-AC-8 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-136
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-025-AC-9
  related_ids: []
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  lines: '102'
  excerpt: PLANNED (IR-806). Every emitted composite shadow leaf binding retains its original operand/parameter
    node, typed path, declared domain and primitive draw type in symbolic call order; controls distinguish
    absent/null/present slots and option/union members. A missing, duplicate, unknown or domain/type-conflicting
    leaf refuses with a typed reason and no harness; parameter/literal and literal/literal operands retain
    their literal singleton identity/value with no unconstrained draw. Public reconstruction preserves
    these facts through FR-033.
  finding: 'Inherited FR-025-AC-9 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-137
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-026-AC-1
  related_ids: []
  path: spec/routed/functional/FR-026-backend-adapter-contract.md
  lines: '60'
  excerpt: The Kani adapter implements the adapter trait, and a function generic over that trait reaches
    the Kani adapter's generation arm, execution, transcript parser and witness renderer through it.
  finding: 'Inherited FR-026-AC-1 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-138
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-026-AC-4
  related_ids: []
  path: spec/routed/functional/FR-026-backend-adapter-contract.md
  lines: '61'
  excerpt: The execution evidence type is an associated type of the adapter trait, and the Kani adapter's
    is `KaniExecutionEvidence`.
  finding: 'Inherited FR-026-AC-4 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-139
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-028-AC-5
  related_ids: []
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  lines: '206'
  excerpt: Every harness identity, generation refusal and execution evidence names its family.
  finding: 'Inherited FR-028-AC-5 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-140
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-028-AC-6
  related_ids: []
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  lines: '207'
  excerpt: A harness of a family with no supplied shadow records proof subject `production`.
  finding: 'Inherited FR-028-AC-6 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-141
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-028-AC-7
  related_ids: []
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  lines: '208'
  excerpt: A family with a supplied shadow yields a harness with proof subject `bounded_shadow` and its
    refinement obligation in the same result, and never the shadow harness alone.
  finding: 'Inherited FR-028-AC-7 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-142
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-028-AC-8
  related_ids: []
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  lines: '209'
  excerpt: An argument narrowed inside its declared domain records the declared domain, the narrowed bound
    and that the harness covers only the narrowed bound, in the harness identity and in the evidence.
  finding: 'Inherited FR-028-AC-8 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-143
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-028-AC-9
  related_ids: []
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  lines: '210'
  excerpt: A narrowing outside the argument's declared domain is refused with a typed reason and no harness.
  finding: 'Inherited FR-028-AC-9 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-144
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-028-AC-13
  related_ids: []
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  lines: '230'
  excerpt: Emit the shadow as a thin wrapper that calls the production oracle, or replace a hard sub-claim
    with `kani::stub`, or add `kani::assume(left == right)` to make an assertion hold; each makes the
    proof a statement about the code it was meant to stand in for, or about nothing.
  finding: 'Inherited FR-028-AC-13 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
bindings: []
review_head: 3d303ea6988496c4e1102faa58a56c1f260193c3
review_verdict: 'FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence
  gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls
  now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.'
part: 8
parts: 18
scope_kind: full-repository criterion presence; no blanket semantic acceptance
```

+++