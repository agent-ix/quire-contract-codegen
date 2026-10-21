<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15276 method=gap-analysis lang=spec pr=quire-contract-codegen#499 model=gpt-6.1-sol run=48b15e0e-90a9-4152-af98-92055cee80a1 -->

Exact head ebb26afd36236bda990224c48a2b69fe4ce80a00; base 6b1ba8b4e4906c2f2e5e2b398fd59e7f2fb6c7f8. Part 18/18. Verdict FAIL.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-307 | medium | coverage | Inherited FR-008-AC-7 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. |
| FND-308 | medium | coverage | Inherited FR-008-AC-9 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. |
| FND-309 | medium | coverage | Inherited FR-009-AC-7 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. |
| FND-310 | medium | coverage | Inherited NFR-004-M-1 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. |
| FND-311 | medium | coverage | Inherited NFR-004-M-2 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. |
| FND-312 | medium | coverage | Inherited NFR-004-M-3 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. |
| FND-313 | medium | coverage | Inherited NFR-004-M-4 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. |
| FND-314 | medium | coverage | Inherited authentic duplicate-source admission evidence remains absent on the reviewed branch; existing synthetic generator tags do not discharge merged AC8 stage prerequisites. Owner IR-17, as retained in PR496 and IR877 spec review. |
| FND-315 | medium | coverage | New AC16 public bounded-positive/private two-form-equivalence and precise malformed-member stage allocation is not established by unchanged synthetic-public fixture tags. Full exact-source equivalence, original bounds/order/rounding, precise private refusals/no code/siblings and genuine source/context positive remain unqualified. Owner IR-17, actual 538eac99 claim. |
| FND-316 | medium | coverage | New AC24 exact external-QSpec strict-reader/shape versus independent genuine-source public-generation separation is not implemented by the unchanged fixture-appending/key-decoding helper. Tags and an unset-QSPEC_REPO early return do not discharge either evidence stage. Owner IR-17, actual 538eac99 claim; existing IR643 recursion ownership remains unchanged. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-008-AC-7
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. Beyond the per-operator agreement stated in FR-008-AC-3, which this criterion does not repeat,
    for each of integer.eq, ne, lt, le, gt and ge the selected claim's V2 comparison oracle returns, for every valuation
    of a 1- to 4-member domain (read-versus-read and read-versus-literal, the literal at each member and in either
    authored position), the result of an independently evaluated ordered relation.
- id: FR-008-AC-9
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. A CG integration test calls IR's public clause-context accessor through its public API only,
    and the generator's census, ordered relation and diagnostics equal the accessor's operator, ordinals, provenance,
    observation, bounds and refusal fields for each closed operator, provenance and observation variant.
- id: FR-009-AC-7
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: PLANNED. For each noncommutative operator, a comparison written with the literal in the first position
    produces the same satisfying and violating sets as its mirrored read-first form, enumerated independently.
- id: NFR-004-M-1
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: Discarded / attempted on a fresh 10,000-case campaign per population and fixture
- id: NFR-004-M-2
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: Proptest global rejects on the same campaigns
- id: NFR-004-M-3
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: Campaigns ending `Exhausted` on the same campaigns, and on 256-case default-config campaigns
- id: NFR-004-M-4
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: In-domain plus out-of-domain census cases per admitted clause
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'A declaration closure `TypeEnvironment::new` refuses is refused at generation time carrying that `DeclarationCause`,
    including both recursion passes. The authentic duplicate-field source control instead retains typed AdmittedRefusal::Package
    with owning public CompileRefusal::code() equal to Code::AmbiguousDeclaration and cause() equal to None for
    the Assembly refusal, produces no successful AdmittedContext and is not passed to public CG generation. Private
    CG declaration-admission defenses retain the exact DuplicateMember name/cause and unchanged healthy sibling
    separately, without '
- id: FR-018-AC-16
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Genuine public tuple members naming `integer_range`, `decimal_range`, `rational_range` and `text_bounds`
    nodes reconstruct their exact own bounds, text profile, position order and decimal rounding; in particular the
    numeric controls retain Int[-100,100], Decimal[-100,100;0,0;nearest-even] and Rational[-10,10;1,1], and the
    text control retains Text[0,16;nfc]. With two `text_bounds` nodes over one text scalar, naming either reconstructs
    that node's own `min` and `max`, never the sibling's. The legacy private resolver control retains exact generated-source
    equivalence between numeric bo
- id: FR-018-AC-24
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The exact QSpec `positive-recursive-records.json` retains strict-reader and List/Tree shape assertions;
    independently authored genuine admitted source with those List (`Option<List>`) and Tree (`Sequence<Tree>` bounded
    `[0,3]`) shapes closes each back-edge at its own in-progress record key, generates equality items over both
    record types, and terminates; a synthetic unclosed `Option` self-cycle and a `Sequence` self-cycle each refuse
    as `TypeResolutionCycle` naming the repeated type node, without a panic, stack overflow or generated symbol.
    PLANNED (IR-643).
- id: FR-038-AC-4
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: The four scalar Kani harnesses still compare their embedded oracle result with a separately rendered
    native `i128` arithmetic expression. A mutation of the prod
- id: NFR-002-AC-2
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Every generated Rust file carries the `MIT OR Apache-2.0` SPDX identity.
- id: FR-004-AC-4
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Successful analysis verifies the bound population, source/map, and native execution bindings.
- id: FR-015-AC-7
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Every generated harness contains exactly one non-vacuity cover; a precondition harness covers that the
    precondition holds within the IR bounds, and a contract h
- id: FR-015-AC-33
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The frame-replay payload's anchor, frame and frame occurrence are the values `qsl_replay::call_site`
    names for the operation's name.
- id: FR-015-AC-67
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '`generate_state_frame_obligations` refuses with `MalformedClause` at the node, not `InvalidField` or
    `ConditionNotSupported`, for a clause whose graph field nam'
- id: FR-015-AC-104
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A plain-Integer parameter as any nested leaf keeps `requires_bound` with the Integer `unbounded_type`
    and no harness; a direct reference to a raw application ne
- id: FR-017-AC-13
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A run whose process exited successfully and whose backend reported success with zero successful checks
    is inconclusive with the vacuous-proof reason, never veri
- id: FR-017-AC-35
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'With crate directory `/h/u/ws/crate`, target directory `/h/u/ws/crate/target` and home directory `/h/u`
    supplied in the request, a stand-in whose streams print '
- id: FR-028-AC-1
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Every harness identity records the request's memory ceiling and wall-clock ceiling, and changing either
    ceiling changes the identity.
- id: FR-029-AC-14
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a `DependencyLockError::Input` that carries QSL''s `DuplicateIdentity` refusal (code
    `invalid_package`), as a lock whose only defect is a repeat'
- id: FR-030-AC-7
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: The map is one `match` over the pair (`KaniOutcomeKind`, replay settlement) with no wildcard arm.
- id: FR-034-AC-46
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'PLANNED/UNRUN (IR-682). The analysis covers the actual linked panic strategy: reachable unwind/landing-pad
    and cleanup paths contribute to the stack bound, or t'
- id: FR-041-AC-1
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A nonempty reviewed declaration is checked against the current compiled-module census and the merge-base
    declaration. An absent compiled module, a removed claim
- id: NFR-006-AC-15
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: A Kani-touching pull request whose required lane did not run or failed remains unmerged and does not
    claim real-Kani verification; a written explanation does no
- id: FR-014-AC-14
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A claim whose node this generator never inspected — a duplicate copy, or a record that never lowered
    — or inspected and refused with a typed reason, marks its o
- id: FR-014-AC-32
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node that narrowing conversions narrow to two distinct bounds (`[10, 29]` and `[0, 40]`) is refused
    as `AmbiguousBound`, in generation and in derivation.
- id: FR-014-AC-51
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A selected `quire.op.quantity.convert` claim settles through capability settlement like any other quantity
    operation, as `OperationNotRendered` (Planned, IR-269
- id: FR-018-AC-18
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '`render_value_type` called with `ValueType::Quantity` and with `ValueType::Reference` returns `Err(RenderError::UnsupportedValueType
    { family })` with `family` '
- id: FR-021-AC-7
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A tree-shaped 100,000-node V2 `expression`/`call` graph whose nested children are separate admitted nodes
    reached by `reference` produces a flat SSA instruction
- id: FR-021-AC-25
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Across one body, the counter charges each node entry, child edge, emitted instruction and location record
    once in source traversal order. A setting equal to the
- id: FR-031-AC-12
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). Apart from the generated Rust header lines required by NFR-002, the oracle generated
    for an expression with no arithmetic node is byte-ide
- id: FR-016-AC-12
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A `replay` call that returns `ReplayRefusal::Fault` is returned as `SpineReplayError::Refused` carrying
    that fault.
- id: FR-024-AC-11
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: '`StateClauseReplay::replay` returns exactly the `StateClauseReplayResult` that `qsl_replay::replay_state_clause`
    returns when a test calls it directly with the '
- id: FR-024-AC-30
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: With the installed backend, the real playback of the falsified frame harness of a subject that writes
    a forbidden field is replayed through `FrameReplay::new` a
- id: FR-019-AC-46
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: With several repeated identities, the refusal names the bytewise-first repeated identity independent
    of manifest order; identities differing only in case are di
- id: FR-019-AC-42
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Two otherwise equal CG Process descriptors with absent and present-empty manifest `bounds` remain distinct
    typed values. Changing only between those forms leave
- id: FR-022-AC-2
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: For a set of routed Kani items, each record and harness in the output equals what `negotiate_kani_obligations`
    returns for the same items in ascending request-i
- id: FR-022-AC-20
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). `RoutedGenerationItem.backend` and `.kind` are private while `request_index` and `node_id`
    remain public; external callers cannot create an it
- id: FR-008-AC-4
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: 'Unknown clause, absent claim, unsupported clause kind and malformed comparison report distinct typed
    causes in the stated first-defect order. Diagnostics carry '
- id: FR-010-AC-4
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: '`amount <= 1000` over 0..=1000 refuses `Boundary` with `UnsupportedCampaignConstraint` and emits no
    artifact for that request, while its `Satisfying` bundle sti'
findings:
- fnd: FND-307
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-008-AC-7
  related_ids: []
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  lines: '123'
  excerpt: PLANNED. Beyond the per-operator agreement stated in FR-008-AC-3, which this criterion does not repeat,
    for each of integer.eq, ne, lt, le, gt and ge the selected claim's V2 comparison oracle returns, for every valuation
    of a 1- to 4-member domain (read-versus-read and read-versus-literal, the literal at each member and in either
    authored position), the result of an independently evaluated ordered relation.
  finding: Inherited FR-008-AC-7 computes untagged at exact base and candidate; owner existing IR-878/program ownership,
    actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable.
    No gap closure or execution is claimed.
- fnd: FND-308
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-008-AC-9
  related_ids: []
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  lines: '125'
  excerpt: PLANNED. A CG integration test calls IR's public clause-context accessor through its public API only,
    and the generator's census, ordered relation and diagnostics equal the accessor's operator, ordinals, provenance,
    observation, bounds and refusal fields for each closed operator, provenance and observation variant.
  finding: Inherited FR-008-AC-9 computes untagged at exact base and candidate; owner existing IR-878/program ownership,
    actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable.
    No gap closure or execution is claimed.
- fnd: FND-309
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-009-AC-7
  related_ids: []
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  lines: '77'
  excerpt: PLANNED. For each noncommutative operator, a comparison written with the literal in the first position
    produces the same satisfying and violating sets as its mirrored read-first form, enumerated independently.
  finding: Inherited FR-009-AC-7 computes untagged at exact base and candidate; owner existing IR-878/program ownership,
    actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable.
    No gap closure or execution is claimed.
- fnd: FND-310
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: NFR-004-M-1
  related_ids: []
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  lines: '1'
  excerpt: Discarded / attempted on a fresh 10,000-case campaign per population and fixture
  finding: Inherited NFR-004-M-1 computes untagged at exact base and candidate; owner existing IR-878/program ownership,
    actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable.
    No gap closure or execution is claimed.
- fnd: FND-311
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: NFR-004-M-2
  related_ids: []
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  lines: '1'
  excerpt: Proptest global rejects on the same campaigns
  finding: Inherited NFR-004-M-2 computes untagged at exact base and candidate; owner existing IR-878/program ownership,
    actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable.
    No gap closure or execution is claimed.
- fnd: FND-312
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: NFR-004-M-3
  related_ids: []
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  lines: '1'
  excerpt: Campaigns ending `Exhausted` on the same campaigns, and on 256-case default-config campaigns
  finding: Inherited NFR-004-M-3 computes untagged at exact base and candidate; owner existing IR-878/program ownership,
    actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable.
    No gap closure or execution is claimed.
- fnd: FND-313
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: NFR-004-M-4
  related_ids: []
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  lines: '1'
  excerpt: In-domain plus out-of-domain census cases per admitted clause
  finding: Inherited NFR-004-M-4 computes untagged at exact base and candidate; owner existing IR-878/program ownership,
    actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable.
    No gap closure or execution is claimed.
- fnd: FND-314
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-8
  related_ids:
  - TC-029
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '611'
  excerpt: 'A declaration closure `TypeEnvironment::new` refuses is refused at generation time carrying that `DeclarationCause`,
    including both recursion passes. The authentic duplicate-field source control instead retains typed AdmittedRefusal::Package
    with owning public CompileRefusal::code() equal to Code::AmbiguousDeclaration and cause() equal to None for
    the Assembly refusal, produces no successful AdmittedContext and is not passed to public CG generation. Private
    CG declaration-admission defenses retain the exact DuplicateMember name/cause and unchanged healthy sibling
    separately, without '
  finding: Inherited authentic duplicate-source admission evidence remains absent on the reviewed branch; existing
    synthetic generator tags do not discharge merged AC8 stage prerequisites. Owner IR-17, as retained in PR496
    and IR877 spec review.
- fnd: FND-315
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-16
  related_ids:
  - TC-029
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '153'
  excerpt: Genuine public tuple members naming `integer_range`, `decimal_range`, `rational_range` and `text_bounds`
    nodes reconstruct their exact own bounds, text profile, position order and decimal rounding; in particular the
    numeric controls retain Int[-100,100], Decimal[-100,100;0,0;nearest-even] and Rational[-10,10;1,1], and the
    text control retains Text[0,16;nfc]. With two `text_bounds` nodes over one text scalar, naming either reconstructs
    that node's own `min` and `max`, never the sibling's. The legacy private resolver control retains exact generated-source
    equivalence between numeric bo
  finding: New AC16 public bounded-positive/private two-form-equivalence and precise malformed-member stage allocation
    is not established by unchanged synthetic-public fixture tags. Full exact-source equivalence, original bounds/order/rounding,
    precise private refusals/no code/siblings and genuine source/context positive remain unqualified. Owner IR-17,
    actual 538eac99 claim.
- fnd: FND-316
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-24
  related_ids:
  - TC-029
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '627'
  excerpt: The exact QSpec `positive-recursive-records.json` retains strict-reader and List/Tree shape assertions;
    independently authored genuine admitted source with those List (`Option<List>`) and Tree (`Sequence<Tree>` bounded
    `[0,3]`) shapes closes each back-edge at its own in-progress record key, generates equality items over both
    record types, and terminates; a synthetic unclosed `Option` self-cycle and a `Sequence` self-cycle each refuse
    as `TypeResolutionCycle` naming the repeated type node, without a panic, stack overflow or generated symbol.
    PLANNED (IR-643).
  finding: New AC24 exact external-QSpec strict-reader/shape versus independent genuine-source public-generation
    separation is not implemented by the unchanged fixture-appending/key-decoding helper. Tags and an unset-QSPEC_REPO
    early return do not discharge either evidence stage. Owner IR-17, actual 538eac99 claim; existing IR643 recursion
    ownership remains unchanged.
bindings: []
```

+++