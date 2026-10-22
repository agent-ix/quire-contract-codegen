<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15286 method=gap-analysis lang=spec pr=quire-contract-codegen#500 model=gpt-6.1-sol run=7dd2e3ef-32c3-41f7-a542-27bc1be8d278 -->

Exact head 508cab924b870851e85970b60e851649cb9a00a7; base cead553e98aaec3e0bf2bdab80ab69c3c3ca775f. Part 18/18. Verdict FAIL.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-307 | medium | coverage | Inherited FR-008-AC-7 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No closure or execution is claimed. |
| FND-308 | medium | coverage | Inherited FR-008-AC-9 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No closure or execution is claimed. |
| FND-309 | medium | coverage | Inherited FR-009-AC-7 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No closure or execution is claimed. |
| FND-310 | medium | coverage | Inherited NFR-004-M-1 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No closure or execution is claimed. |
| FND-311 | medium | coverage | Inherited NFR-004-M-2 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No closure or execution is claimed. |
| FND-312 | medium | coverage | Inherited NFR-004-M-3 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No closure or execution is claimed. |
| FND-313 | medium | coverage | Inherited NFR-004-M-4 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No closure or execution is claimed. |
| FND-314 | medium | coverage | Inherited genuine duplicate-source admission stage evidence is absent from the reviewed branch. Existing synthetic generator tags do not discharge the amended reader-intake prerequisite. Owner IR-17, existing scope decisions 68f36578 and 3004a231. |
| FND-315 | medium | coverage | Inherited exact public bounded-positive/private two-form-equivalence and malformed-member stage allocation remains unqualified. Existing synthetic-public tags do not establish unchanged original bounds/order/rounding, precise private refusals/no code/siblings and genuine source/context positive. Owner IR-17, existing scope decisions 68f36578 and 3004a231. |
| FND-316 | medium | coverage | Inherited exact external QSpec strict-reader/shape versus independent genuine-source public-generation separation remains unqualified. Fixture-appending/key-decoding controls and absent-QSPEC early returns do not discharge either stage. Owner IR-17; existing IR643 recursion ownership unchanged. |
| FND-317 | medium | coverage | New Composite reconstruction mapping is not implemented or asserted on the reviewed spec branch: existing generated header and structural test still enumerate six variants and the legacy Declaration(InvalidDeclaration). Separate unqualified IR17 source contains the eighth variant and lookup helper but is not acceptance evidence for this frozen candidate. Owner IR-17; exact genuine owner context and typed Lookup/context/QSV controls must remain, with structural defenses receiving no public admission/execution credit. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-008-AC-9
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. A CG integration test calls IR's public clause-context accessor through its public API only,
    and the generator's census, ordered relation and diagnostics equal the accessor's operator, ordinals, provenance,
    observation, bounds and refusal fields for each closed operator, provenance and observation variant.
- id: FR-008-AC-10
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. Source inspection shows the generator reads no `CheckedSemanticNodeV2.body` field and decodes
    no body JSON (the verification of FR-008-CON-1).
- id: FR-008-CON-1
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: The generator SHALL read V2 operand identity, provenance, observation, operator and bounds only through
    IR's public typed accessor. A private decoder or copied schema is forbidden.
- id: FR-008-CON-2
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: Admission SHALL finish before any strategy, population, census or runner source is published; a refusal
    returns no artifact or partial bundle.
- id: FR-008-CON-3
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: The generator SHALL check each read endpoint for i64 representability before constructing the shared-domain
    relation.
- id: FR-008-CON-4
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: The generator SHALL check each comparison literal for i64 representability before conversion.
- id: FR-009-AC-1
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: "For the QSL-produced, IR-admitted V2 `VersionUnchanged` selected claim (FR-008-AC-1) over 0..=1000,\
    \ every `Satisfying` case has post = pre, every `Violating` case has post \u2260 pre, and every value lies in\
    \ 0..=1000."
- id: FR-009-AC-2
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: 'For every V2 comparison operator, every domain of 1, 2, 3, and 4 members, and both a two-read relation
    and a read-versus-literal relation with the literal at each domain member, the generatable set of each population
    equals the independently enumerated satisfying or violating set: none missing, none extra. An empty side is
    refused with `EmptyPopulation` and never produces an empty strategy.'
- id: FR-009-AC-3
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: Generated population source contains no filter, assume, reject, or discard call, and a seeded 10,000-case
    run of each population records zero discards and zero proptest global rejects.
- id: FR-009-AC-4
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: For `amount < 0` over 0..=1000, `Satisfying` and `Broad` are refused with `EmptyPopulation` naming `Satisfying`,
    and `Violating` generates; for `amount >= 0`, `Violating` and `Broad` are refused naming `Violating`; no refused
    request emits an artifact.
- id: FR-009-AC-5
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: A domain of `i64::MIN..=i64::MAX` with `NotEqual`, `Less`, and `Equal` compiles as generated Rust and
    generates 10,000 seeded cases without overflow or panic, every case in domain and on its side.
- id: FR-009-AC-6
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: Repeated generation for the same `CheckedPackageV2`, selected clause id, authentic claim occurrence,
    and population is byte-identical.
- id: FR-009-AC-7
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: PLANNED. For each noncommutative operator, a comparison written with the literal in the first position
    produces the same satisfying and violating sets as its mirrored read-first form, enumerated independently.
- id: FR-010-AC-1
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: For `amount < 7` over 0..=1000, the in-domain census is exactly [0 Holds, 1 Holds, 6 Holds, 7 Violated,
    8 Violated, 999 Violated, 1000 Violated], and the out-of-domain array is exactly [-1, 1001]; for `amount < 1`,
    where the literal edges coincide with the domain edges, the in-domain census is exactly [0 Holds, 1 Violated,
    2 Violated, 999 Violated, 1000 Violated] with no duplicates, and the out-of-domain array is exactly [-1, 1001].
- id: FR-010-AC-2
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: 'For the QSL-produced, IR-admitted V2 `VersionUnchanged` selected claim (primary post, partner pre; FR-008-AC-1)
    over 0..=1000, the in-domain census is exactly the 10 pairs (post, pre): (0,0 H), (0,1 V), (1,0 V), (1,1 H),
    (1,2 V), (999,998 V), (999,999 H), (999,1000 V), (1000,999 V), (1000,1000 H); the out-of-domain array is exactly
    the 10 pairs (-1,0), (0,-1), (0,1001), (1,-1), (1,1001), (999,-1), (999,1001), (1000,-1), (1000,1001), (1001,1000).'
- id: FR-010-AC-3
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: A selected claim whose accessor domain is exactly `i64::MIN..=i64::MAX` (representable under FR-008-AC-6)
    compiles as generated Rust without overflow or panic, its out-of-domain array holds no case for either outer
    edge, and its unrepresentable-edge array lists both.
- id: FR-010-AC-4
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: '`amount <= 1000` over 0..=1000 refuses `Boundary` with `UnsupportedCampaignConstraint` and emits no
    artifact for that request, while its `Satisfying` bundle still carries the out-of-domain array [-1, 1001].'
- id: FR-010-AC-5
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: Every in-domain census tag equals an independent evaluation of the relation, every out-of-domain case
    has at least one value outside the domain, and repeated generation produces byte-identical arrays.
- id: FR-011-AC-1
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: Over V2 selected claims, a QSL-produced `VersionUnchanged` (`Postcondition`) `Broad` campaign records
    `Holds` cases as `Passed` and `Violated` cases as `FailedPostcondition` and passes; a `Precondition` fixture
    records `Violated` cases as `RejectedPrecondition` in `rejected` and passes with zero proptest global rejects;
    the `amount < 7` `Invariant` records `Violated` cases as `FailedPostcondition`.
- id: FR-011-AC-2
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: A V2 comparison oracle deliberately swapped for its negation fails each campaign with `ConformanceMismatch`
    carrying the case values, expected tag, and observed verdict kind.
- id: FR-011-AC-3
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: The census runner evaluates each of the 10 `VersionUnchanged` in-domain census cases exactly once, in
    census order, and evaluates none of the out-of-domain cases.
- id: FR-011-AC-4
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: A campaign over a report seeded with fixed prior counts returns `discard_rate() == Some((discarded, attempted))`
    and `rejection_rate() == Some((rejected, attempted))` with those exact values, a fresh zero-attempt summary
    returns `None` for both, and a runtime snapshot whose counters are `at_limit` returns `None` for both through
    the generated snapshot-summary operation.
- id: FR-011-AC-5
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: A fresh 10,000-case `Broad` campaign for each fixture returns `discard_rate() == Some((0, attempted))`
    with `attempted > 0`, passes a zero discard ceiling, and does not end `Exhausted`.
- id: FR-012-AC-1
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: For every V2 comparison operator and every domain of 1 to 4 members from FR-009-AC-2, an exhaustive walk
    of every valid `simplify` and `complicate` path of `Satisfying`, `Violating`, and `Broad` value trees visits
    only in-domain candidates that keep their original tag; a valid path calls `complicate` only after at least
    one successful `simplify`.
- id: FR-012-AC-2
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: For 1,000 seeded `VersionUnchanged` value trees of each population over 0..=1000, every candidate on
    the greedy shrink path stays in domain and keeps its tag.
- id: FR-012-AC-3
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: A `Violating` V2 `VersionUnchanged` campaign whose comparison oracle is replaced by `post == pre
- id: FR-012-AC-4
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: After a shrinking campaign, `attempted` equals the number of oracle evaluations the runner performed,
    including shrink replays.
- id: FR-013-AC-1
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: A consumer fixture crate that includes the generated Rust source and whose manifest depends only on `proptest`
    and `quire-contract-runtime` compiles under denied warnings, draws `VersionUnchanged` cases, and reads the selected
    `versionNumber` StateField at `"pre"` and `"post"` through the generated typed-identity, path and observation
    constants.
- id: FR-013-AC-2
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: The generated bundle contains no serialized case, census, or summary file, and the change adds no file
    under `schemas/`.
- id: FR-013-AC-4
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: The generated Rust header carries the selected checked clause id and authentic claim occurrence, and
    generating two distinct selected claims from one package produces different headers and artifact paths.
- id: FR-013-AC-5
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: interface-001 `bound_strategy_slice.consumer` records the case, census, and runner surface a downstream
    consumer compiles against.
- id: NFR-004-AC-1
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: Fresh 10,000-case and 256-case campaigns of every population over the V2 selected claims `VersionUnchanged`,
    `amount < 7`, and a `Precondition` fixture record zero discards, zero proptest global rejects, and no `Exhausted`
    conclusion.
- id: NFR-004-AC-2
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: For every admitted fixture in TC-019, the in-domain census plus out-of-domain array holds at most 20
    cases.
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
- id: FR-008-AC-7
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. Beyond the per-operator agreement stated in FR-008-AC-3, which this criterion does not repeat,
    for each of integer.eq, ne, lt, le, gt and ge the selected claim's V2 comparison oracle returns, for every valuation
    of a 1- to 4-member domain (read-versus-read and read-versus-literal, the literal at each member and in either
    authored position), the result of an independently evaluated ordered relation.
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
- id: FR-018-AC-17
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The `src/lib.rs` that `generate_composite_equality_oracles` returns for the TC-029 step 1 package (every
    generated item) contains zero occurrences of the panic tokens FR-018-AC-19 lists, no `.ok()`, `.unwrap_or(`
    or `.unwrap_or_default(` anywhere, and no `[` index or slice expression directly after an identifier character,
    `)` or `]`; every reconstruction helper in it returns `Result<_, ReconstructionError>`, and each of `rebuild_integer`,
    `rebuild_interval`, `rebuild_rational`, `rebuild_decimal`, `rebuild_text` and `rebuild_cardinality` turns a
    failed constructor or parse into exact
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
    No closure or execution is claimed.
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
    No closure or execution is claimed.
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
    No closure or execution is claimed.
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
    No closure or execution is claimed.
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
    No closure or execution is claimed.
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
    No closure or execution is claimed.
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
    No closure or execution is claimed.
- fnd: FND-314
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-8
  related_ids:
  - TC-029
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '618'
  excerpt: 'A declaration closure `TypeEnvironment::new` refuses is refused at generation time carrying that `DeclarationCause`,
    including both recursion passes. The authentic duplicate-field source control instead retains typed AdmittedRefusal::Package
    with owning public CompileRefusal::code() equal to Code::AmbiguousDeclaration and cause() equal to None for
    the Assembly refusal, produces no successful AdmittedContext and is not passed to public CG generation. Private
    CG declaration-admission defenses retain the exact DuplicateMember name/cause and unchanged healthy sibling
    separately, without '
  finding: Inherited genuine duplicate-source admission stage evidence is absent from the reviewed branch. Existing
    synthetic generator tags do not discharge the amended reader-intake prerequisite. Owner IR-17, existing scope
    decisions 68f36578 and 3004a231.
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
  finding: Inherited exact public bounded-positive/private two-form-equivalence and malformed-member stage allocation
    remains unqualified. Existing synthetic-public tags do not establish unchanged original bounds/order/rounding,
    precise private refusals/no code/siblings and genuine source/context positive. Owner IR-17, existing scope decisions
    68f36578 and 3004a231.
- fnd: FND-316
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-24
  related_ids:
  - TC-029
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '634'
  excerpt: The exact QSpec `positive-recursive-records.json` retains strict-reader and List/Tree shape assertions;
    independently authored genuine admitted source with those List (`Option<List>`) and Tree (`Sequence<Tree>` bounded
    `[0,3]`) shapes closes each back-edge at its own in-progress record key, generates equality items over both
    record types, and terminates; a synthetic unclosed `Option` self-cycle and a `Sequence` self-cycle each refuse
    as `TypeResolutionCycle` naming the repeated type node, without a panic, stack overflow or generated symbol.
    PLANNED (IR-643).
  finding: Inherited exact external QSpec strict-reader/shape versus independent genuine-source public-generation
    separation remains unqualified. Fixture-appending/key-decoding controls and absent-QSPEC early returns do not
    discharge either stage. Owner IR-17; existing IR643 recursion ownership unchanged.
- fnd: FND-317
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-17
  related_ids:
  - TC-029
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '627'
  excerpt: The `src/lib.rs` that `generate_composite_equality_oracles` returns for the TC-029 step 1 package (every
    generated item) contains zero occurrences of the panic tokens FR-018-AC-19 lists, no `.ok()`, `.unwrap_or(`
    or `.unwrap_or_default(` anywhere, and no `[` index or slice expression directly after an identifier character,
    `)` or `]`; every reconstruction helper in it returns `Result<_, ReconstructionError>`, and each of `rebuild_integer`,
    `rebuild_interval`, `rebuild_rational`, `rebuild_decimal`, `rebuild_text` and `rebuild_cardinality` turns a
    failed constructor or parse into exact
  finding: 'New Composite reconstruction mapping is not implemented or asserted on the reviewed spec branch: existing
    generated header and structural test still enumerate six variants and the legacy Declaration(InvalidDeclaration).
    Separate unqualified IR17 source contains the eighth variant and lookup helper but is not acceptance evidence
    for this frozen candidate. Owner IR-17; exact genuine owner context and typed Lookup/context/QSV controls must
    remain, with structural defenses receiving no public admission/execution credit.'
bindings: []
```

+++
