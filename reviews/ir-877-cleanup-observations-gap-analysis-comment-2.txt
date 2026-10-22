<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15291 method=gap-analysis lang=rust pr=quire-contract-codegen#501 model=unavailable run=d9d507d8-42eb-4ddc-a8d0-3eed0b37342f -->

SR-15291 gap-analysis, part 2/18. FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-019 | high | coverage | Inherited FR-004-AC-15 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-020 | high | coverage | Inherited FR-004-AC-18 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-021 | high | coverage | Inherited FR-005-AC-3 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-022 | high | coverage | Inherited FR-008-AC-7 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-023 | high | coverage | Inherited FR-008-AC-9 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-024 | high | coverage | Inherited FR-009-AC-7 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-025 | high | coverage | Inherited FR-015-AC-19 is tagged-by-ignored-test at the frozen candidate: every bound test is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-026 | high | coverage | Inherited FR-015-AC-21 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-027 | high | coverage | Inherited FR-015-AC-22 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-028 | high | coverage | Inherited FR-015-AC-23 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-029 | high | coverage | Inherited FR-015-AC-24 is tagged-by-ignored-test at the frozen candidate: every bound test is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-030 | high | coverage | Inherited FR-015-AC-30 is tagged-by-ignored-test at the frozen candidate: every bound test is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-031 | high | coverage | Inherited FR-015-AC-31 is tagged-by-ignored-test at the frozen candidate: every bound test is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-032 | high | coverage | Inherited FR-015-AC-32 is tagged-by-ignored-test at the frozen candidate: every bound test is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-033 | high | coverage | Inherited FR-015-AC-38 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-034 | high | coverage | Inherited FR-015-AC-39 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-035 | high | coverage | Inherited FR-015-AC-40 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-036 | high | coverage | Inherited FR-015-AC-41 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-005-AC-9
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When replacement fails and restoring the prior destination also fails, the publisher reports
    destination state `unknown`, the destination path is absent, and the complete prior bundle and the
    complete staged bundle remain in their sibling directories for recovery.
- id: FR-005-AC-10
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When two artifacts of one bundle share the same path, bundle construction is refused with `DuplicateArtifactPath`
    naming the later `bundle.artifacts[i].path`, before any destination or staging I/O begins.
- id: FR-008-AC-1
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: A fresh QSL-produced, IR-admitted ConfigVersion V2 postcondition with direct `self.versionNumber
    = pre(self.versionNumber)` yields two `State` census entries in authored order, `Post` then `Pre`,
    each 0..=1000, with the same project id, `quire.op.integer.eq`, and the selected clause and authentic
    claim retained. The let-bound alias form yields a typed operand refusal and no bundle.
- id: FR-008-AC-2
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: An IR-admitted V2 `amount < 7` invariant with a typed current read of range 0..=1000 yields
    operator `Less`, primary `amount`, literal 7 and that inclusive census domain. Reversing the two authored
    operands preserves the reversed positions and corresponding operator semantics.
- id: FR-008-AC-3
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: A valid selected comparison generates despite a comparison-oracle-refused sibling claim in
    the same package; requesting the refused claim returns its typed selected-claim cause and locus with
    no bundle. An oracle-construction failure for the selected claim returns no bundle. Each of integer.eq/ne/lt/le/gt/ge
    has an executable selected-claim comparison oracle whose result agrees with the ordered relation and
    generated tags. Arithmetic, alias, Boolean and literal-only shapes refuse at their own structural
    loci.
- id: FR-008-AC-4
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: Unknown clause, absent claim, unsupported clause kind and malformed comparison report distinct
    typed causes in the stated first-defect order. Diagnostics carry selected clause and claim, structural
    path and reached child id when available, without claiming a unique child source region.
- id: FR-008-AC-5
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: A request with several applicable defects returns the first only. Every refusal returns zero
    artifacts; a successful request returns the complete source, census and runner bundle.
- id: FR-008-AC-6
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: Either read endpoint or a literal at i64::MIN - 1, i64::MAX + 1, i128::MIN or i128::MAX refuses
    `UnsupportedRelation` at that operand without narrowing. Two individually i64-representable reads
    with unequal ranges also refuse; equal ranges admit when other rules hold.
- id: FR-008-AC-7
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. Beyond the per-operator agreement stated in FR-008-AC-3, which this criterion does
    not repeat, for each of integer.eq, ne, lt, le, gt and ge the selected claim's V2 comparison oracle
    returns, for every valuation of a 1- to 4-member domain (read-versus-read and read-versus-literal,
    the literal at each member and in either authored position), the result of an independently evaluated
    ordered relation.
- id: FR-008-AC-8
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. The V2 comparison oracle takes only the accessor's typed result for the selected claim,
    and no V1 bound oracle or `BoundPackage` is an input to it.
- id: FR-008-AC-9
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. A CG integration test calls IR's public clause-context accessor through its public
    API only, and the generator's census, ordered relation and diagnostics equal the accessor's operator,
    ordinals, provenance, observation, bounds and refusal fields for each closed operator, provenance
    and observation variant.
- id: FR-008-AC-10
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. Source inspection shows the generator reads no `CheckedSemanticNodeV2.body` field
    and decodes no body JSON (the verification of FR-008-CON-1).
- id: FR-008-CON-1
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: The generator SHALL read V2 operand identity, provenance, observation, operator and bounds
    only through IR's public typed accessor. A private decoder or copied schema is forbidden. | Interface
- id: FR-008-CON-2
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: Admission SHALL finish before any strategy, population, census or runner source is published;
    a refusal returns no artifact or partial bundle. | Integrity
- id: FR-008-CON-3
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: The generator SHALL check each read endpoint for i64 representability before constructing the
    shared-domain relation. | Integrity
- id: FR-008-CON-4
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: The generator SHALL check each comparison literal for i64 representability before conversion.
    | Integrity
- id: FR-009-AC-1
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: For the QSL-produced, IR-admitted V2 `VersionUnchanged` selected claim (FR-008-AC-1) over 0..=1000,
    every `Satisfying` case has post = pre, every `Violating` case has post ≠ pre, and every value lies
    in 0..=1000.
- id: FR-009-AC-2
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: 'For every V2 comparison operator, every domain of 1, 2, 3, and 4 members, and both a two-read
    relation and a read-versus-literal relation with the literal at each domain member, the generatable
    set of each population equals the independently enumerated satisfying or violating set: none missing,
    none extra. An empty side is refused with `EmptyPopulation` and never produces an empty strategy.'
- id: FR-009-AC-3
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: Generated population source contains no filter, assume, reject, or discard call, and a seeded
    10,000-case run of each population records zero discards and zero proptest global rejects.
- id: FR-009-AC-4
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: For `amount < 0` over 0..=1000, `Satisfying` and `Broad` are refused with `EmptyPopulation`
    naming `Satisfying`, and `Violating` generates; for `amount >= 0`, `Violating` and `Broad` are refused
    naming `Violating`; no refused request emits an artifact.
- id: FR-009-AC-5
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: A domain of `i64::MIN..=i64::MAX` with `NotEqual`, `Less`, and `Equal` compiles as generated
    Rust and generates 10,000 seeded cases without overflow or panic, every case in domain and on its
    side.
- id: FR-009-AC-6
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: Repeated generation for the same `CheckedPackageV2`, selected clause id, authentic claim occurrence,
    and population is byte-identical.
- id: FR-009-AC-7
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: PLANNED. For each noncommutative operator, a comparison written with the literal in the first
    position produces the same satisfying and violating sets as its mirrored read-first form, enumerated
    independently.
- id: FR-010-AC-1
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: For `amount < 7` over 0..=1000, the in-domain census is exactly [0 Holds, 1 Holds, 6 Holds,
    7 Violated, 8 Violated, 999 Violated, 1000 Violated], and the out-of-domain array is exactly [-1,
    1001]; for `amount < 1`, where the literal edges coincide with the domain edges, the in-domain census
    is exactly [0 Holds, 1 Violated, 2 Violated, 999 Violated, 1000 Violated] with no duplicates, and
    the out-of-domain array is exactly [-1, 1001].
- id: FR-010-AC-2
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: 'For the QSL-produced, IR-admitted V2 `VersionUnchanged` selected claim (primary post, partner
    pre; FR-008-AC-1) over 0..=1000, the in-domain census is exactly the 10 pairs (post, pre): (0,0 H),
    (0,1 V), (1,0 V), (1,1 H), (1,2 V), (999,998 V), (999,999 H), (999,1000 V), (1000,999 V), (1000,1000
    H); the out-of-domain array is exactly the 10 pairs (-1,0), (0,-1), (0,1001), (1,-1), (1,1001), (999,-1),
    (999,1001), (1000,-1), (1000,1001), (1001,1000).'
- id: FR-010-AC-3
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: A selected claim whose accessor domain is exactly `i64::MIN..=i64::MAX` (representable under
    FR-008-AC-6) compiles as generated Rust without overflow or panic, its out-of-domain array holds no
    case for either outer edge, and its unrepresentable-edge array lists both.
- id: FR-010-AC-4
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: '`amount <= 1000` over 0..=1000 refuses `Boundary` with `UnsupportedCampaignConstraint` and
    emits no artifact for that request, while its `Satisfying` bundle still carries the out-of-domain
    array [-1, 1001].'
- id: FR-010-AC-5
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: Every in-domain census tag equals an independent evaluation of the relation, every out-of-domain
    case has at least one value outside the domain, and repeated generation produces byte-identical arrays.
- id: FR-011-AC-1
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: Over V2 selected claims, a QSL-produced `VersionUnchanged` (`Postcondition`) `Broad` campaign
    records `Holds` cases as `Passed` and `Violated` cases as `FailedPostcondition` and passes; a `Precondition`
    fixture records `Violated` cases as `RejectedPrecondition` in `rejected` and passes with zero proptest
    global rejects; the `amount < 7` `Invariant` records `Violated` cases as `FailedPostcondition`.
- id: FR-011-AC-2
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: A V2 comparison oracle deliberately swapped for its negation fails each campaign with `ConformanceMismatch`
    carrying the case values, expected tag, and observed verdict kind.
- id: FR-011-AC-3
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: The census runner evaluates each of the 10 `VersionUnchanged` in-domain census cases exactly
    once, in census order, and evaluates none of the out-of-domain cases.
- id: FR-011-AC-4
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: A campaign over a report seeded with fixed prior counts returns `discard_rate() == Some((discarded,
    attempted))` and `rejection_rate() == Some((rejected, attempted))` with those exact values, a fresh
    zero-attempt summary returns `None` for both, and a runtime snapshot whose counters are `at_limit`
    returns `None` for both through the generated snapshot-summary operation.
- id: FR-011-AC-5
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: A fresh 10,000-case `Broad` campaign for each fixture returns `discard_rate() == Some((0, attempted))`
    with `attempted > 0`, passes a zero discard ceiling, and does not end `Exhausted`.
- id: FR-012-AC-1
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: For every V2 comparison operator and every domain of 1 to 4 members from FR-009-AC-2, an exhaustive
    walk of every valid `simplify` and `complicate` path of `Satisfying`, `Violating`, and `Broad` value
    trees visits only in-domain candidates that keep their original tag; a valid path calls `complicate`
    only after at least one successful `simplify`.
- id: FR-012-AC-2
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: For 1,000 seeded `VersionUnchanged` value trees of each population over 0..=1000, every candidate
    on the greedy shrink path stays in domain and keeps its tag.
- id: FR-012-AC-3
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: A `Violating` V2 `VersionUnchanged` campaign whose comparison oracle is replaced by `post ==
    pre || post == pre + 1` (so only `Violated` cases with post = pre + 1 mismatch) reports a minimal
    counterexample with post = pre + 1, both in 0..=1000, tagged `Violated`.
- id: FR-012-AC-4
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: After a shrinking campaign, `attempted` equals the number of oracle evaluations the runner
    performed, including shrink replays.
- id: FR-013-AC-1
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: A consumer fixture crate that includes the generated Rust source and whose manifest depends
    only on `proptest` and `quire-contract-runtime` compiles under denied warnings, draws `VersionUnchanged`
    cases, and reads the selected `versionNumber` StateField at `"pre"` and `"post"` through the generated
    typed-identity, path and observation constants.
- id: FR-013-AC-2
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: The generated bundle contains no serialized case, census, or summary file, and the change adds
    no file under `schemas/`.
- id: FR-013-AC-4
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: The generated Rust header carries the selected checked clause id and authentic claim occurrence,
    and generating two distinct selected claims from one package produces different headers and artifact
    paths.
- id: FR-013-AC-5
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: interface-001 `bound_strategy_slice.consumer` records the case, census, and runner surface
    a downstream consumer compiles against.
- id: FR-014-AC-1
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Every requested item receives exactly one generated or typed-refused disposition, and a refused
    item contributes no generated function while its siblings are generated unchanged.
- id: FR-014-AC-2
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Integer (add, subtract, multiply, negate, truncating/floor/Euclidean division, modulo), rational,
    ordering, decimal, IEEE arithmetic/comparison/width conversion, text comparison, enum comparison,
    quantity arithmetic/comparison and exact quantity conversion (FR-014-AC-48) descriptors for admitted
    checked-package expression nodes each generate a function calling the matching runtime `exact` operator.
    Entry-point dispositions for a `quire.op.quantity.convert` node: derivation and generation produce
    the oracle (FR-014-AC-48); the routed path, `negotiate_kani_obligations` and capability settlement '
- id: FR-014-AC-3
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A descriptor whose arity, result or operand scalar type disagrees with the lowered node, a
    node that is not a lowered scalar expression, a node that is unbounded, has an invalid or incomplete
    body, or exhausts lowering work, or a node id requested more than once, is refused with a typed reason.
- id: FR-014-AC-4
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Generated bytes are identical across repeated runs and across permutations of the request order;
    the generated crate is exactly `Cargo.toml`, `src/lib.rs` and `claim-map.json`, its `claim-map.json`
    equals the returned claim map, every generated claim's oracle symbol is defined in `src/lib.rs`, and
    claim-map entries are ordered by node id.
- id: FR-014-AC-5
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Every claim-map entry carries the node id, IR id, package id, source map, claims, bounds and
    operation identity of its item, and, for a `quire.op.quantity.convert` item, the `type_argument` type
    node id (FR-014-AC-48).
- id: FR-014-AC-6
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Executing generated oracles on the QSpec TC-185, TC-186, TC-192 and TC-193 vectors, and on
    these QSpec TC-187 vectors, yields outcomes, charges and consumed counters equal to direct runtime
    execution: U02 (dimension normalization through a product and a quotient, and a zero power), U03 (the
    dimension mismatch), U05 (arithmetic and a power on `degC` and `degF`, refused `ill_typed`), U09 (a
    base dimension of the same declaration and shape as `L` under a different node key, refused `ill_typed`),
    U12 (compound-unit products, quotients and powers, with the `s` family), U13 and U21 (powers), U14
    (th'
- id: FR-014-AC-7
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Composite, collection, function, model, relation, state, temporal and protocol nodes are refused
    with their blocked or unsupported reason and never reported as generated.
- id: FR-014-AC-8
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: The generated crate declares `publish = false`, names the direct exact/QSV and Runtime residue
    dependencies from the one profile authority without a replay-context dependency for Boolean/integer
    scalar proofs, contains no charge amount (every charge comes from runtime metering), and compiles.
- id: FR-014-AC-9
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Generated oracle functions do not panic: an invalid generated constant, including a decimal
    target, stops as `InvalidConstant`, an operand of the wrong width stops before any charge, and generated
    source over its ceiling is a typed error with no output.'
findings:
- fnd: FND-019
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-004-AC-15
  related_ids: []
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  lines: '195'
  excerpt: PLANNED. A controlled attempt to replace the strict-reader typed result, execution record,
    generated source, source map, LLVM export stream or LLVM tool/version with a foreign-run value cannot
    construct a valid same-invocation context, even when paths, QSL `package_id`, source/map and tool/version
    declarations match. In particular, foreign export bytes offered through a detached buffer or a different
    producer stream/child handle refuse as `producer_run_mismatch` before classification because neither
    is the current runner-owned campaign/profile/export chain. A malformed association observable i
  finding: 'Inherited FR-004-AC-15 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-020
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-004-AC-18
  related_ids: []
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  lines: '198'
  excerpt: PLANNED (IR-815). A V2 clause whose generated source map declares an implication count unequal
    to the count derived from its body yields a structured binding non-success naming the clause and no
    classification, and is never treated as implication-free.
  finding: 'Inherited FR-004-AC-18 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-021
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-005-AC-3
  related_ids: []
  path: spec/core/functional/FR-005-cli-conformance.md
  lines: '49'
  excerpt: For every shared-corpus case on which two backends' normalized outcomes differ, the TC-007
    Differential Disposition Register holds exactly one row naming the case, the backends, and either
    a regression fixture that exists in the test tree or a documented profile/backend distinction; a differing
    case with no row, a row naming a fixture that does not exist, or a row with neither disposition fails
    the check.
  finding: 'Inherited FR-005-AC-3 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-022
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-008-AC-7
  related_ids: []
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  lines: '123'
  excerpt: PLANNED. Beyond the per-operator agreement stated in FR-008-AC-3, which this criterion does
    not repeat, for each of integer.eq, ne, lt, le, gt and ge the selected claim's V2 comparison oracle
    returns, for every valuation of a 1- to 4-member domain (read-versus-read and read-versus-literal,
    the literal at each member and in either authored position), the result of an independently evaluated
    ordered relation.
  finding: 'Inherited FR-008-AC-7 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-023
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-008-AC-9
  related_ids: []
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  lines: '125'
  excerpt: PLANNED. A CG integration test calls IR's public clause-context accessor through its public
    API only, and the generator's census, ordered relation and diagnostics equal the accessor's operator,
    ordinals, provenance, observation, bounds and refusal fields for each closed operator, provenance
    and observation variant.
  finding: 'Inherited FR-008-AC-9 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-024
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-009-AC-7
  related_ids: []
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  lines: '77'
  excerpt: PLANNED. For each noncommutative operator, a comparison written with the literal in the first
    position produces the same satisfying and violating sets as its mirrored read-first form, enumerated
    independently.
  finding: 'Inherited FR-009-AC-7 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-025
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-19
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '755'
  excerpt: A zero-input transition over one bounded-integer state value verifies for an identity subject
    and is falsified with a concrete counterexample for a subject that changes the value.
  finding: 'Inherited FR-015-AC-19 is tagged-by-ignored-test at the frozen candidate: every bound test
    is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or
    qualify that obligation.'
- fnd: FND-026
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-21
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '757'
  excerpt: A precondition that reads a post-state value is refused with a typed reason naming the node,
    and no harness is emitted.
  finding: 'Inherited FR-015-AC-21 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-027
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-22
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '758'
  excerpt: A declared proof-dependency census with an empty or duplicate identity, an inconsistent kind,
    state and path combination, or a non-`Required` kind is refused as a typed invalid input with no harness.
  finding: 'Inherited FR-015-AC-22 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-028
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-23
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '759'
  excerpt: Every requested item receives exactly one `ObligationDisposition` (`supported`, `requires-bound`,
    `unsupported` or `invalid-request`), and an item that is not `supported` keeps its source identity
    and typed reason with no harness and no assumption emitted for it.
  finding: 'Inherited FR-015-AC-23 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-029
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-24
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '760'
  excerpt: A falsifying in-domain assignment of a plain bounded-integer comparison claim is reported through
    Kani's concrete playback.
  finding: 'Inherited FR-015-AC-24 is tagged-by-ignored-test at the frozen candidate: every bound test
    is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or
    qualify that obligation.'
- fnd: FND-030
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-30
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '766'
  excerpt: With the installed backend, the operation contract verifies for a healthy subject and is falsified,
    naming the postcondition, for a subject mutated to debit.
  finding: 'Inherited FR-015-AC-30 is tagged-by-ignored-test at the frozen candidate: every bound test
    is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or
    qualify that obligation.'
- fnd: FND-031
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-31
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '767'
  excerpt: With the installed backend, a frame-allowed effect verifies, a frame-forbidden effect is falsified
    naming the forbidden field, and regenerating the frame from a package whose `modifies` is mutated
    to grant nothing falsifies the allowed subject.
  finding: 'Inherited FR-015-AC-31 is tagged-by-ignored-test at the frozen candidate: every bound test
    is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or
    qualify that obligation.'
- fnd: FND-032
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-32
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '768'
  excerpt: The forbidden frame counterexample, executed natively, reproduces as a frame violation of that
    field through QSL's `replay_frame`, and the allowed run replays as a respected frame.
  finding: 'Inherited FR-015-AC-32 is tagged-by-ignored-test at the frozen candidate: every bound test
    is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or
    qualify that obligation.'
- fnd: FND-033
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-38
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '774'
  excerpt: A V2 clause claim naming a `state`/`state_clause` node of an admitted `CheckedPackageV2` whose
    `operation.member.clause` is `precondition`, `postcondition` or `invariant` yields one harness of
    that obligation kind, separate from the harness of every other clause of the package; the kind is
    read from the node and no caller value changes it. PLANNED (IR-489).
  finding: 'Inherited FR-015-AC-38 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-034
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-39
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '775'
  excerpt: 'A V2 clause harness draws one nondeterministic value per drawn input: the pre-state of each
    state field of `self` the body reads, whether bare or under `pre(...)` (the subject''s input state;
    how it reaches the subject is FR-025''s), and each operation parameter (for a precondition or invariant,
    each bare state field read, since the state is the one judged). Each carries an inclusive assumption
    equal to its declared domain: the accessor''s `IntRange` values for a selected model declaration state
    field (FR-015-AC-77), and the IR `bounded_domain` of a parameter''s type. A non-model or admitted
    unsel'
  finding: 'Inherited FR-015-AC-39 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-035
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-40
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '776'
  excerpt: A harness for a V2 clause whose condition, inline or referenced from an admitted Boolean expression
    node, contains Boolean connectives and bounded-integer comparisons (`and`, `or`, `not`, `implies`,
    `eq`, `ne` and the six integer comparisons) embeds an FR-014 oracle of that condition byte-identical
    to the oracle crate's clause-identified function, as FR-015-AC-20 states for a scalar claim; the clause-identified
    oracle needs FR-014-AC-38. PLANNED (IR-489).
  finding: 'Inherited FR-015-AC-40 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-036
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-41
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '777'
  excerpt: 'A V2 clause whose body holds integer arithmetic (add, subtract, multiply, negate) as an operand
    of a comparison lowers, and its harness keeps the independent native assertion of FR-015-AC-37 for
    each arithmetic subterm (the oracle''s outcome against the operation evaluated natively in `i128`
    over the same operands, `Refused` assumed away nowhere): a postcondition `amount < 1000 implies amount
    + 1 <= 1000` over `amount: Int[0, 1000]` yields one contract harness. With the installed backend the
    unmutated clause verifies with its cover satisfied, and the same clause with `+` mutated to return
    `left'
  finding: 'Inherited FR-015-AC-41 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
bindings: []
review_head: 3d303ea6988496c4e1102faa58a56c1f260193c3
review_verdict: 'FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence
  gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls
  now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.'
part: 2
parts: 18
scope_kind: full-repository criterion presence; no blanket semantic acceptance
```

+++