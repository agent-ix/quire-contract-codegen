<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15291 method=gap-analysis lang=rust pr=quire-contract-codegen#501 model=unavailable run=d9d507d8-42eb-4ddc-a8d0-3eed0b37342f -->

SR-15291 gap-analysis, part 17/18. FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-289 | high | coverage | Inherited FR-035-AC-4 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-290 | high | coverage | Inherited FR-035-AC-5 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-291 | high | coverage | Inherited FR-035-AC-6 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-292 | high | coverage | Inherited FR-036-AC-1 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-293 | high | coverage | Inherited FR-036-AC-2 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-294 | high | coverage | Inherited FR-036-AC-3 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-295 | high | coverage | Inherited FR-036-AC-4 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-296 | high | coverage | Inherited FR-036-AC-5 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-297 | high | coverage | Inherited FR-036-AC-6 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-298 | high | coverage | Inherited FR-036-AC-7 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-299 | high | coverage | Inherited FR-039-AC-4 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-300 | high | coverage | Inherited FR-041-AC-4 is tagged-by-ignored-test at the frozen candidate: every bound test is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-301 | high | coverage | Inherited FR-042-AC-1 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-302 | high | coverage | Inherited NFR-001-M-1 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-303 | high | coverage | Inherited NFR-002-M-1 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-304 | high | coverage | Inherited NFR-002-M-3 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-305 | high | coverage | Inherited NFR-004-M-1 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-306 | high | coverage | Inherited NFR-004-M-2 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-036-AC-7
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: Missing/incomplete authoritative inputs, a relation targeting a foreign selection and a dangling
    relation endpoint produce distinct typed census refusals with their offending input/relation and no
    complete census. A valid absent correspondence remains an explicit obligation/criterion/source-symbol
    gap, or a production gap on an existing complete candidate key, instead of that global refusal.
- id: FR-037-AC-1
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: Both `tests/exact_scalar_support/package.rs` and `tests/composite_equality_support/package.rs`
    derive every candidate application node identity through `derive_application_node_id` over the complete
    draft node; neither retains an application preimage encoder, application version constant, or SHA
    call for that identity. An ungrouped fixture's candidate equals the checked reader's admitted node
    ID.
- id: FR-037-AC-2
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: A fixture containing an application in a recursion group supplies the complete ordered draft
    graph to `ApplicationIdentityContext`; its candidate agrees with the reader after group-reference
    rewriting. Missing or mismatched context and a canonical byte-limit refusal retain Contract IR's typed
    error and produce no candidate.
- id: FR-037-AC-3
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: A canonical enum-member fixture identity is derived through `NominalIdentityPreimage::EnumMember(...).digest`,
    agrees with the checked reader, and changes when the member case changes. In-process agreement keys
    that are never compared with a checked-package identity remain classified as fixture-only keys.
- id: FR-037-AC-4
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: Positive exact-scalar and composite-equality packages remain admitted. A stale candidate key
    or a body mutation without rekeying is refused by the checked reader; the fixture builder does not
    repair those negative controls before the assertion.
- id: FR-037-AC-5
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: The consumer change leaves deterministic fixture-only keys distinct from application identities
    and uses `quire-canonical` for ordinary canonical-value hashes where those hashes are needed. Package
    identity refresh uses Contract IR's typed package identity preimage and `quire-canonical`; no independent
    application or package preimage rule is added.
- id: FR-038-AC-1
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: 'A source inventory compares every identity in [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md)''s
    specified derivable set with the implemented scalar, scalar Kani and state/frame selections, recording
    the unimplemented scalar integer `eq/ne` and Boolean variants as open FR-014 work. For the four integer
    arithmetic identities shared by scalar oracle and scalar Kani, and the four integer ordering identities
    shared by scalar oracle and state/frame Kani, the implemented consumers resolve the same typed identity,
    family, arity and ordered operands; no second string-to-operator match '
- id: FR-038-AC-2
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: The typed source distinguishes implemented integer, rational, decimal, IEEE, text, enum and
    quantity operations and their applicable result and mode forms. When FR-014's specified Boolean scalar
    variants are implemented, they use distinct Boolean entries in this same source before the scalar
    emitter selects them. A valid caller-supplied scalar descriptor with a present but disagreeing node
    identity, law or mode still generates its descriptor-named oracle as `caller_declared` with a typed
    blocked item under FR-014-AC-12 and AC-13. A missing node identity retains its typed refusal; an underivabl
- id: FR-038-AC-3
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: Focused V2 oracle, scalar Kani and state/frame tests distinguish the wrong identity, family,
    operand arity or order, comparison symbol and applicable runtime callee from the correct rendering.
    With the correct identity and arity, a wrong fixed result form or wrong member-selected result target,
    including a conversion target, is refused under FR-014's result/parameter checks rather than rendered
    as the right operator with the wrong result. Existing admission and generated-artifact behavior for
    supported identities remains equivalent.
- id: FR-038-AC-4
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: The four scalar Kani harnesses still compare their embedded oracle result with a separately
    rendered native `i128` arithmetic expression. A mutation of the production oracle's selected operation
    or callee makes the real Kani proof fail under an in-domain non-vacuous control; routing the native
    expression through that callee fails review.
- id: FR-038-AC-5
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: The V1 `strategy/bound` relation evaluator remains outside this V2 catalog migration until
    the V2 strategy replacement specified by [AD-004](../../assurance/AD-004-cg-crate-layout.md) and [FR-008](../../strategy/functional/FR-008-bound-domain-strategy-admission.md).
    The hand-built bounded corpus is also excluded, and its duplicate `checked_method` mapping is removed
    only by [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) and AD-004 step 4g retirement
    after that step's proof controls pass. Neither path gains a compatibility forwarding layer.
- id: FR-038-AC-6
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: While the V1 `strategy/bound` relation evaluator remains live, focused tests discriminate all
    six comparison operators and a primary read on the left versus on the right of a noncommutative comparison;
    its host evaluation preserves the relation as authored.
- id: FR-039-AC-1
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: Each of two genuine QSL-admitted source graphs with a different clause or pure-function operation
    lowers through IR to its own admitted FR-330 package. CG's production settlement and generation use
    each package's typed graph and accounting for its exact request index and node ID; supplying the two
    packages with each one's corresponding node ID at the same request position changes the generated
    bytes to reflect the represented operation while backend and generation context stay fixed. If the
    production API still accepts an original checked graph, holding that legacy input fixed across the
    two c
- id: FR-039-AC-2
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: A mixed request with a representable clause or function and an independent `unsupported`, `requires_bound`
    or `invalid_input` sibling retains one typed decision per bound driver item, including duplicate requested
    IDs, and emits only the representable sibling's artifact. An interleaved context-only root retains
    its own exact accounting position and key without creating a driver decision. For two QSL scalar items
    sharing one expression key but selecting different checked application/conversion pairs, CG retains
    each pair's own admitted FR-049 result bound; a conversion outside its driver's cont
- id: FR-039-AC-3
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: A selected dependency function reference resolves only through the reader-validated selected
    package identity and target key. A missing, wrong-identity or stale selected dependency refuses before
    CG; an identically keyed local node or extra unselected dependency cannot redirect generation.
- id: FR-039-AC-4
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: For a genuinely admitted FR-330 package, CG binds a `protocol`/`protocol_reference` carrier's
    exact `body.target` to the local `protocol` node, retaining its typed key, requested identity and
    source occurrences; changing that local target and recomputing the admitted carrier and package identities
    changes the selected target and package identity. A matching key only in a selected dependency does
    not bind locally. An absent local target, a present non-`protocol` target and a target with unequal
    `semantic_type` are refused by the FR-322 reader before CG with the respective `missing_declaration`/
- id: FR-039-AC-5
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: A malformed or noncanonical FR-330 wire, wrong version, invalid accounting/source map, stale
    semantic identity or incomplete reader result reaches no CG consumer and emits no artifact. CG preserves
    the reader's typed refusal or limit result and performs no second parse or checked-graph fallback.
- id: FR-039-AC-6
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: An item whose admitted package accounting is `failed`, `invalid_body` or `body_incomplete`,
    or whose `lowered` form is unrepresentable by the selected backend, emits no artifact and retains
    a distinct typed cause; the package byte-ceiling case emits no artifact for any sibling.
- id: FR-039-AC-7
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: Generated scalar, composite equality and exact function claims each serialize `contract_package_identity`
    equal to the admitted FR-330 `identity` and `node_id` equal to the requested `CheckedNodeId`, with
    no generated-claim `ir_id` key. Each retains exact `semantic_type`, admitted `source_map`, family-specific
    provenance, and `package_id` denoting the originating checked package; each serializes typed `claims`,
    `bounds` and `dependencies`. For the same requested node in two genuinely admitted packages with different
    identities, the claim identity differs; for two different requested nodes in o
- id: FR-039-AC-8
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: 'For each admitted `lowered` bound driver item, Contract IR''s authoritative traversal is total
    over the required root, bounds and selected dependencies, and the generated claim''s reached claim,
    bound and dependency keys and dependent declaration closure equal its output. An unresolved required
    reference cannot reach CG as an admitted `lowered` item: Contract IR records a typed item refusal
    or its strict reader refuses the package; a refused or incomplete reader result yields no item claim
    or artifact. Inspection finds no per-node ID from a second lowering or wire-fragment hash.'
- id: FR-039-AC-9
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: In an admitted package with a bound driver item, an excluded accounting-only position and another
    bound driver item, CG matches all three positions and exact `node_id` values to `feature_accounting`;
    the excluded position keeps its driver index but has no claim or generated artifact, and the following
    bound item keeps its original driver index. An interleaved context-only root has accounting but advances
    neither driver nor envelope item indexes. Removing the excluded position, adding an extra one or changing
    its `node_id` causes a typed whole-request correspondence refusal before any artifact.
- id: FR-039-AC-10
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: Given a bound driver item followed by an excluded accounting-only position, a selected conversion
    context root outside the preceding item's contiguous span yields typed `IncompleteContext` for that
    item when the complete root sequence is otherwise valid. A context-only root directly after the excluded
    position and before the next bound driver item is orphaned and causes a typed whole-request correspondence
    refusal before any artifact, even when its accounting position and `node_id` match. A conversion root
    immediately before the excluded position remains in the preceding item's span.
- id: FR-040-AC-1
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A caller retains a live `Cancel`, sends its clone through a public single-run request, and
    the bounded launcher observes it before spawn and during an actual running backend. Pre-spawn cancellation
    starts no backend; running cancellation confirms owned teardown before evidence returns.
- id: FR-040-AC-2
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: In two live single runs, issue `Requested` then `Deadline`, and reverse the calls. Each run's
    actual launcher stop, `KaniExecutionEvidence.outcome`, constructed IR `Cancelled { source }` and QSL
    `IncompleteCause::Cancelled { source }` retain the first cause; a later event cannot change them.
    A hand-built IR outcome alone does not satisfy this criterion.
- id: FR-040-AC-3
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A run that reaches its execution ceiling with no cancellation event remains `TimedOut`. A `Deadline`
    event recorded only after completion, memory/capture stop or elapsed-ceiling stop does not turn that
    result into `Cancelled`.
- id: FR-040-AC-4
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: Completion before observed cancellation keeps the report-based result; measured memory exhaustion,
    memory observation failure, output over-limit, unread capture and unconfirmed cleanup keep their existing
    result/refusal and confirmed teardown rules when cancellation competes. A cancellation observed first
    and causing stop is not overwritten by later completion or elapsed time.
- id: FR-040-AC-5
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A multi-member group sharing one handle stops wholly on its observed cause, returns `BatchCancelled
    { source, members }`, and classifies no member; another group using a distinct handle retains its
    own result. Reversing the two cause calls changes the cancelled group's source and not the other group's
    evidence. Distinct handles never share one launcher.
- id: FR-040-AC-6
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: The single-run producer constructs an IR `KaniOutcomeKind::Cancelled { source }` from settled
    execution evidence, and its real call path reaches the FR-030 map. Mapping both IR source variants
    exhaustively yields the identical `quire_exact::CancelCause` inside QSL's `IncompleteCause::Cancelled`;
    neither metadata nor a copied enum determines the source.
- id: FR-040-AC-7
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: The direct `quire-exact::CancelCause` constructs `qsl-replay::IncompleteCause::Cancelled` through
    a type-checked exhaustive conversion, while all QSL imports remain through `qsl-replay`; no implicit
    cancellation control, source fallback, or compatibility overload is present.
- id: FR-041-AC-1
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A nonempty reviewed declaration is checked against the current compiled-module census and the
    merge-base declaration. An absent compiled module, a removed claim, and an empty list each name the
    claim or condition and fail; adding a compiled claim does not erase an older one.
- id: FR-041-AC-2
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: With two claimed modules sharing one `src/lib.rs`, a SUCCESS location inside only the first
    and an UNREACHABLE check inside the second reports the second `unreached` and fails. A SUCCESS check
    inside each passes this census; a SATISFIED cover, unknown location or non-SUCCESS status cannot supply
    the missing check.
- id: FR-041-AC-3
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The current build's transitive expectation/subject reachability yields the shared-helper intersection;
    the gate fails when the checked-in generated evidence omits or adds a helper, and records an explicit
    empty set when none is shared. For an exact operation, the expectation uses QSpec vectors or a model
    with no `quire-exact` call.
- id: FR-041-AC-4
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: Each claimed module and computed shared helper has a positive unmodified real-Kani proof and
    a source mutation that compiles, executes, and reports a failed proposition check. Deleting a control
    or making its mutant verify, fail to compile, time out, skip execution, or merely trip the dirty-tree
    guard fails the gate; evidence names the exact diff, command and failed check from this run.
- id: FR-041-AC-5
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A stubbed executor, predetermined outcome or build-only run produces no qualifying module evidence
    and cannot pass the gate.
- id: FR-042-AC-1
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: CG's native Boolean, equality, function and pre-check placeholder constructors use the exact
    nine CG-owned causes in the table, retaining `IllTypedCause` from `check_type` and `check_equality`;
    downstream replay consumers preserve the typed payload or match it explicitly, retain charge/failure
    behavior, and accept neither the former unit constructor nor an invented default.
- id: NFR-001-AC-1
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  excerpt: Identical inputs and configuration produce byte-identical bundles.
- id: NFR-001-AC-2
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  excerpt: An injected generation failure leaves no partially published bundle.
- id: NFR-001-AC-3
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  excerpt: Generation does not modify developer-owned regions or files.
- id: NFR-001-M-1
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  excerpt: Byte differences between repeated generations
- id: NFR-001-M-2
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  excerpt: Partial published bundles after injected failure
- id: NFR-001-M-3
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  excerpt: Developer-owned files modified
- id: NFR-002-AC-1
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Every generated Rust file in a family named in Scope has a header naming `quire-contract-codegen`,
    every input-carried subject identity it renders as defined in Scope, and every requirement, revision
    or clause identifier present in the generation input.
- id: NFR-002-AC-2
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Every generated Rust file carries the `MIT OR Apache-2.0` SPDX identity.
- id: NFR-002-AC-3
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Unsupported and inconclusive states remain explicit rather than silently succeeding.
- id: NFR-002-AC-4
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Generated output makes no project-specific validation, accreditation, certification, or release-approval
    claim.
- id: NFR-002-M-1
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Generated Rust files missing generator or input-carried subject identity
- id: NFR-002-M-2
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Generated files missing dual-license SPDX header
- id: NFR-002-M-3
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Silent unsupported/inconclusive states
- id: NFR-004-AC-1
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: Fresh 10,000-case and 256-case campaigns of every population over the V2 selected claims `VersionUnchanged`,
    `amount < 7`, and a `Precondition` fixture record zero discards, zero proptest global rejects, and
    no `Exhausted` conclusion.
- id: NFR-004-AC-2
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: For every admitted fixture in TC-019, the in-domain census plus out-of-domain array holds at
    most 20 cases.
findings:
- fnd: FND-289
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-035-AC-4
  related_ids: []
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  lines: '110'
  excerpt: For each of the six profiles and both `Text[0,8]` and `Text[1,1]`, a real Kani harness symbolically
    ranges over all 21 sequences of zero to two scalars from the four-scalar alphabet, constructs each
    `TextPayload` by `from_utf8`, compares the generated admission oracle with a separate direct runtime
    call including outcome and metering, and ends with one satisfied cover after its assertions. The proof
    identity names this finite class and its type.
  finding: 'Inherited FR-035-AC-4 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-290
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-035-AC-5
  related_ids: []
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  lines: '111'
  excerpt: In a copy of the generated oracle with its admission call or declared `TextType` altered so
    the oracle accepts a payload the direct runtime call refuses, real Kani falsifies the admission agreement
    assertion with concrete playback; restoring the call verifies over the same symbolic class.
  finding: 'Inherited FR-035-AC-5 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-291
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-035-AC-6
  related_ids: []
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  lines: '112'
  excerpt: A request outside the declared finite proof class has a typed unsupported or requires-bound
    disposition and no harness, and no caller-ingress claim is marked `ir_confirmed` or assigned a checked-package
    node id. The six attempted text-operand `numeric.convert` nodes remain Contract IR refusals and do
    not count as passing caller-admission coverage.
  finding: 'Inherited FR-035-AC-6 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-292
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-036-AC-1
  related_ids: []
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  lines: '140'
  excerpt: 'A genuine typed fixture with criteria C1/C2, obligations O1/O2/O3, source identities S1/S2,
    domain D and two production functions in one module exports three obligation inventory entries and
    exactly four candidate keys for authored tuples (C1,O1,S1,D), (C2,O1,S1,D), (C1,O2,S1,D), (C1,O2,S2,D).
    O3 has no authored criterion correspondence: it remains an inventory/gap record with no candidate.
    Missing production correspondence and authored no-function remain distinct gaps on complete candidate
    keys; a zero-mapping source symbol remains an inventory/gap record. Removing only C2''s relation leaves
    C'
  finding: 'Inherited FR-036-AC-1 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-293
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-036-AC-2
  related_ids: []
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  lines: '141'
  excerpt: For the unchanged AC-1 fixture, omitting a result, reporting unsupported or pre-run refusal,
    retrying and changing method/profile/subject/artifact/run leave the complete obligation inventory,
    all four criterion-obligation-source-domain keys and mapping-gap records unchanged; enumeration precedes
    producer invocation. Removing O1 and its authored relations from the authoritative source removes
    its inventory entry and exactly the two O1 candidate tuples without editing an auxiliary catalog.
  finding: 'Inherited FR-036-AC-2 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-294
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-036-AC-3
  related_ids: []
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  lines: '142'
  excerpt: Capability facts preserve unsupported capability and reason without artifact/run/outcome; pre-run
    refusal facts preserve producer/stage/cause separately from actual execution outcomes. An absent decision
    or observation and an unfamiliar producer state remain distinct, and missing, counterexample, invalid/incomplete,
    unavailable, timeout, exhaustion, cancellation and inconclusive evidence states survive export.
  finding: 'Inherited FR-036-AC-3 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-295
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-036-AC-4
  related_ids: []
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  lines: '143'
  excerpt: A matching executed production fact retains its actual typed artifact and run identities separately
    from attempt identity, exact source/profile/subject/domain/bounds and explicitly bound load-bearing
    check plus production reachability. Wrong, missing and mismatched artifact/run identity controls preserve
    the received typed identity or explicit missing reason for IR joining, without inventing or repairing
    identities. Zero-check, check-location-only, tag-only, LLVM-only and incidental-reachability controls
    never acquire positive observations by export.
  finding: 'Inherited FR-036-AC-4 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-296
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-036-AC-5
  related_ids: []
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  lines: '144'
  excerpt: Shadow-only evidence, a narrower effective bound and separately executed sampled native refinement
    retain distinct subject/scope/assumption observations. Missing refinement stays missing; adding a
    refinement observation does not change the census or create an unqualified-production-proof flag.
  finding: 'Inherited FR-036-AC-5 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-297
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-036-AC-6
  related_ids: []
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  lines: '145'
  excerpt: A downstream consumer above CG and IR borrows the public census/fact types over the reviewed
    neutral correspondence contract without source/transcript/name parsing or a copied schema. Duplicate
    and artifact/run/source/profile/subject/domain-mismatched result controls survive as separate inspectable
    records without changing inventories or candidate tuples; repeated identical inputs produce identical
    ordering.
  finding: 'Inherited FR-036-AC-6 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-298
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-036-AC-7
  related_ids: []
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  lines: '146'
  excerpt: Missing/incomplete authoritative inputs, a relation targeting a foreign selection and a dangling
    relation endpoint produce distinct typed census refusals with their offending input/relation and no
    complete census. A valid absent correspondence remains an explicit obligation/criterion/source-symbol
    gap, or a production gap on an existing complete candidate key, instead of that global refusal.
  finding: 'Inherited FR-036-AC-7 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-299
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-039-AC-4
  related_ids: []
  path: spec/core/functional/FR-039-contract-package-consumption.md
  lines: '213'
  excerpt: For a genuinely admitted FR-330 package, CG binds a `protocol`/`protocol_reference` carrier's
    exact `body.target` to the local `protocol` node, retaining its typed key, requested identity and
    source occurrences; changing that local target and recomputing the admitted carrier and package identities
    changes the selected target and package identity. A matching key only in a selected dependency does
    not bind locally. An absent local target, a present non-`protocol` target and a target with unequal
    `semantic_type` are refused by the FR-322 reader before CG with the respective `missing_declaration`/
  finding: 'Inherited FR-039-AC-4 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-300
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-041-AC-4
  related_ids: []
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  lines: '69'
  excerpt: Each claimed module and computed shared helper has a positive unmodified real-Kani proof and
    a source mutation that compiles, executes, and reports a failed proposition check. Deleting a control
    or making its mutant verify, fail to compile, time out, skip execution, or merely trip the dirty-tree
    guard fails the gate; evidence names the exact diff, command and failed check from this run.
  finding: 'Inherited FR-041-AC-4 is tagged-by-ignored-test at the frozen candidate: every bound test
    is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or
    qualify that obligation.'
- fnd: FND-301
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-042-AC-1
  related_ids: []
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  lines: '31'
  excerpt: CG's native Boolean, equality, function and pre-check placeholder constructors use the exact
    nine CG-owned causes in the table, retaining `IllTypedCause` from `check_type` and `check_equality`;
    downstream replay consumers preserve the typed payload or match it explicitly, retain charge/failure
    behavior, and accept neither the former unit constructor nor an invented default.
  finding: 'Inherited FR-042-AC-1 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-302
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: NFR-001-M-1
  related_ids: []
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  lines: '1'
  excerpt: Byte differences between repeated generations
  finding: 'Inherited NFR-001-M-1 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-303
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: NFR-002-M-1
  related_ids: []
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  lines: '1'
  excerpt: Generated Rust files missing generator or input-carried subject identity
  finding: 'Inherited NFR-002-M-1 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-304
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: NFR-002-M-3
  related_ids: []
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  lines: '1'
  excerpt: Silent unsupported/inconclusive states
  finding: 'Inherited NFR-002-M-3 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-305
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: NFR-004-M-1
  related_ids: []
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  lines: '1'
  excerpt: Discarded / attempted on a fresh 10,000-case campaign per population and fixture
  finding: 'Inherited NFR-004-M-1 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-306
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: NFR-004-M-2
  related_ids: []
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  lines: '1'
  excerpt: Proptest global rejects on the same campaigns
  finding: 'Inherited NFR-004-M-2 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
bindings: []
review_head: 3d303ea6988496c4e1102faa58a56c1f260193c3
review_verdict: 'FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence
  gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls
  now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.'
part: 17
parts: 18
scope_kind: full-repository criterion presence; no blanket semantic acceptance
```

+++