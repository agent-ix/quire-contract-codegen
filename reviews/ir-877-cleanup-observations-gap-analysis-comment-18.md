<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15291 method=gap-analysis lang=rust pr=quire-contract-codegen#501 model=unavailable run=d9d507d8-42eb-4ddc-a8d0-3eed0b37342f -->

SR-15291 gap-analysis, part 18/18. FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-307 | high | coverage | Inherited NFR-004-M-3 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-308 | high | coverage | Inherited NFR-004-M-4 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-309 | high | coverage | Inherited NFR-006-M-1 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-310 | high | coverage | Inherited NFR-006-M-2 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-311 | high | coverage | Inherited NFR-006-M-3 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-312 | medium | coverage | Inherited Genuine duplicate-source admission-stage evidence remains absent; existing synthetic generator tags do not discharge the owning reader-intake prerequisite. Owner IR-17; existing IR-643 recursion ownership is unchanged. |
| FND-313 | medium | coverage | Inherited Exact public bounded-positive versus private two-form-equivalence and malformed-member stage allocation remains unqualified; synthetic-public tags do not establish original bounds/order/rounding or precise private refusal/no-code/sibling controls. Owner IR-17; existing IR-643 recursion ownership is unchanged. |
| FND-314 | medium | coverage | Inherited Exact external QSpec strict-reader/shape and separately authored genuine-source public-generation stages remain unqualified; appended fixtures and absent-QSPEC early returns cannot discharge them. Owner IR-17; existing IR-643 recursion ownership is unchanged. |
| FND-315 | medium | coverage | Inherited Composite reconstruction mapping remains unimplemented on this branch: emitted header and structural control still enumerate six variants and legacy Declaration(InvalidDeclaration); separate unqualified IR17 draft source is not acceptance evidence. Owner IR-17; existing IR-643 recursion ownership is unchanged. |
| FND-316 | medium | coverage | Inherited test tests::duplicate_expression_uses_each_selected_conversion_context carries untracked tag FR-049-AC-1, which declares no local criterion and binds no obligation. This unchanged tag must not be counted as criterion coverage. Root decision 7ba6731d explicitly assigns this pre-existing mapping to existing IR-878 repository trace/spec-defect triage; it is not claimed previously enumerated or fixed. |
| FND-317 | medium | coverage | Inherited test tests::duplicate_expression_uses_each_selected_conversion_context carries untracked tag FR-049-AC-5, which declares no local criterion and binds no obligation. This unchanged tag must not be counted as criterion coverage. Root decision 7ba6731d explicitly assigns this pre-existing mapping to existing IR-878 repository trace/spec-defect triage; it is not claimed previously enumerated or fixed. |
| FND-318 | medium | coverage | Inherited test tests::scalar_own_result_without_conversion_differs_from_not_applicable carries untracked tag FR-049-AC-1, which declares no local criterion and binds no obligation. This unchanged tag must not be counted as criterion coverage. Root decision 7ba6731d explicitly assigns this pre-existing mapping to existing IR-878 repository trace/spec-defect triage; it is not claimed previously enumerated or fixed. |
| FND-319 | medium | coverage | Inherited test tc_001_boolean_oracle_bundle_is_deterministic_traceable_and_schema_valid carries untracked tag AD-004, which declares no local criterion and binds no obligation. This unchanged tag must not be counted as criterion coverage. Root decision 7ba6731d explicitly assigns this pre-existing mapping to existing IR-878 repository trace/spec-defect triage; it is not claimed previously enumerated or fixed. |

+++ [reviewer data]

```yaml
clean: false
scope:
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
- id: NFR-005-AC-1
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: Every `.rs` file under `src/`, except a file its parent module declares under `#[cfg(test)]`,
    contains zero panic tokens in its non-test code with comments, every `#[cfg(test)]` item and every
    string and character literal removed (`non_test_code_outside_literals`, a variant of `non_test_code`
    that `tests/common/panic_scan.rs` gains), including the body of `fn digest` of `impl CaseIdentity`
    in `src/kani/generate/corpus/bounded_kani_corpus.rs`; no `expect` in that file, `render_artifacts`
    included, is allowed. The tokens are the identifiers `unwrap`, `expect`, `unwrap_unchecked`, `unwrap_err`,
    `
- id: NFR-005-AC-2
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: '`lower_scalar_claim` returns `Err(ScalarLoweringRefusal::NoRenderer)` for a claim whose `checked_bounds`
    is empty and for a claim whose first checked bound has no `IntegerRange` among the derived domains,
    each built directly against the function, and `classify_claim` reports each as `UnsupportedObligation::OperationNotRendered`.'
- id: NFR-005-AC-3
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: '`ItemSettlement::warning` returns `None` for a `Disposition::Unsupported` carrying each of
    `Cause::AbsentKind`, `UnknownKind`, `AbsentExtent`, `UnknownBackend`, `InconsistentCandidates` and
    `AmbiguousBackend`, and still returns its warning for each `unsupported_projection` cause.'
- id: NFR-005-AC-4
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: With a coverage export supplied, `observe_clause` given a map whose oracle-evaluation region
    has no probe returns a row with a `MapMismatch` diagnostic with the message `missing semantic probe`,
    no classification and an `evaluation_count` of `None`, and given one whose consequent region has no
    probe returns the same diagnostic with no classification. With no coverage export it returns `UnavailableObservation`
    for the same maps.
- id: NFR-005-AC-5
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: The step of `generate_kani` that pairs FR-015's records with the Kani group, given hand-built
    inputs in a `#[cfg(test)]` module, returns `RoutedGenerationError::KaniRecordCountMismatch { records,
    items }` carrying both counts when the counts differ, in both directions, and pairs every item with
    its record when they are equal.
- id: NFR-005-AC-6
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: '`rewrite_duplicate_position`, a function of `src/routed/generate.rs` that `route_records`
    (which `generate_kani` calls to pair, rewrite and join FR-015''s records) calls to rewrite a `DuplicateItem`
    record''s `first_index` into the driver''s request index, given hand-built records and a group in
    a `#[cfg(test)]` module, returns `RoutedGenerationError::KaniDuplicatePositionOutOfRange { first_index,
    items }` carrying the position and the group''s length when `first_index` equals or exceeds the group''s
    length, with nothing generated, and returns the request index of the earlier item when `first_index'
- id: NFR-005-AC-7
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: '`observe_clause`, given a map of zero regions and a map of one region, each with and without
    a coverage export, returns a row with a `MapMismatch` diagnostic with the message `typed implication
    census differs from map`, no classification, an `evaluation_count` of `None` and no consequents; given
    a map of two probed regions and a coverage export it returns a row with no consequents and no `MapMismatch`
    diagnostic.'
- id: NFR-005-AC-8
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: The scan locates the body of `generate_kani`, `route_records`, `rewrite_duplicate_position`,
    `observe_clause` and `generate_boolean_oracle_inner` in the literal-free non-test code of its file,
    asserts that it found each of the five and that each holds an identifier of its own, and asserts that
    none contains an index token or a subtraction token, so that a later change cannot bring back one
    of the IR-577 sites. An operand-ending token is `)`, `]`, `}`, `?`, a numeric literal (so the `0`
    of a tuple field `t.0` counts), or an identifier that is not a Rust keyword (`in`, `let`, `mut`, `ref`,
    `retu
- id: NFR-005-M-1
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: Panic tokens in the non-test, literal-free code of `src/`
- id: NFR-005-M-2
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: Measured sites a test can reach that abort instead of returning a typed value
- id: NFR-005-M-3
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: Index, slice and unchecked subtraction expressions in the bodies of the five IR-577 functions
- id: NFR-006-AC-1
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make kani-scope` prints `required` and the path for each of `src/kani/generate/negotiate.rs`,
    `src/oracle/boolean_v1.rs`, `src/routed/generate.rs`, `src/replay/witness.rs`, `src/core/identity.rs`,
    `src/publication/mod.rs`, `tests/it/kani_batching.rs`, `tests/it/skeleton_spine.rs`, `tests/it/bounded_kani_corpus.rs`,
    `tests/it/kani_obligations_state_frame.rs`, `tests/it/scratch_crate.rs`, `tests/exact_scalar_support/package.rs`,
    `tests/checked_package_support/base.rs`, `tests/checked_package_support/rekey.rs`, `tests/state_frame_support/model.rs`,
    `schemas/kani-proof-graph-v2.schema.json`, `sch'
- id: NFR-006-AC-2
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make kani-scope` prints `not required` for each of `spec/kani/functional/FR-017-kani-execution-evidence.md`,
    `reviews/REV-018-bound-coverage-observations.md`, `src/strategy/mod.rs`, `src/evidence/mod.rs`, `Makefile`
    and `.github/workflows/ci.yml`, each fed alone.'
- id: NFR-006-AC-3
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: A rename of `src/kani/old.rs` to `src/strategy/new.rs`, and a rename of `src/strategy/old.rs`
    to `src/kani/new.rs`, each print `required`.
- id: NFR-006-AC-4
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make -n kani` and `make -n kani-gate` expand to the same `cargo test` command line, and that
    line holds each of the six filters `kani_obligations`, `skeleton_spine`, `kani_witness_join`, `bounded_kani_corpus`,
    `kani_generation` and `kani_batching`.'
- id: NFR-006-AC-5
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make kani-gate` starts the lane''s `cargo test` once with `--test-threads=1`, holds the lock
    named by `KANI_LOCK` while it runs, and does not start it while another process holds that lock.'
- id: NFR-006-AC-6
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make kani-gate` exits non-zero and prints `result=failed` when the lane''s filters list zero
    tests, when fewer tests run than are listed, and when any test fails; it exits zero and prints `result=passed`
    only when every listed test ran and passed and the count is above zero.'
- id: NFR-006-AC-7
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: The passing run-result line has the fields `result`, `ran`, `expected`, `elapsed`, `kani`,
    `tree` in that order, with `ran` equal to `expected`, a non-empty `kani` equal to the version the
    launcher reports, and `tree=clean`; it has no commit token. A run whose launcher reports no version
    prints `result=failed`.
- id: NFR-006-AC-8
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: When the tree state or the head commit read after the run differs from the one read before
    the build, `make kani-gate` prints `result=failed` and exits non-zero.
- id: NFR-006-AC-9
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: With a dirty working tree `make kani-gate` prints `tree=dirty`, never `result=passed`, and
    exits non-zero.
- id: NFR-006-AC-10
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: 'With no `cargo-kani` launcher `make kani-gate` starts no test, prints `kani-gate: not run:
    launcher absent` and exits non-zero.'
- id: NFR-006-AC-11
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: 'The tests marked `#[ignore = "kani lane: ..."]` in `tests/it` are exactly the `#[ignore]`d
    tests of `tests/it` whose path holds one of the `make kani` filters.'
- id: NFR-006-AC-12
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make -n ci` expands to no `kani` or `kani-gate` command.'
- id: NFR-006-AC-13
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: For a Kani-touching pull request, the author runs the full gate on its clean, stable candidate
    before opening it, and the merger runs it again on the clean, stable current candidate before merge;
    both actual processes exit successfully with all expected tests run. A copied body line alone cannot
    satisfy either run.
- id: NFR-006-AC-14
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Given Kani-reaching paths changed on `origin/main` since the candidate's merge base and main
    absent from the candidate, `make kani-gate` refuses before running tests and names those paths. Given
    only non-touching main changes, it may run. The merger updates a stale candidate from current main
    and the full gate passes on the updated clean candidate before merge.
- id: NFR-006-AC-15
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: A Kani-touching pull request whose required lane did not run or failed remains unmerged and
    does not claim real-Kani verification; a written explanation does not waive the gate.
- id: NFR-006-AC-16
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: A pull request whose scope is `not required` has no mandatory real-Kani run under this requirement.
- id: NFR-006-AC-17
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Before a version tag is pushed, the full gate actually passes on the clean, stable candidate
    to be tagged; a copied release-ticket line cannot substitute for that run.
- id: NFR-006-AC-18
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Every `include!`, `include_str!`, `include_bytes!` and `#[path]` target named in a file of
    `tests/it` that holds a lane test, and in each file those name in turn, matches a pattern of the Kani-touching
    set.
- id: NFR-006-AC-19
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: On a clean current candidate, `make kani-gate` runs FR-041's module census and real mutation
    controls in its one required lane, and a failure of any claim or shared-helper control makes the gate
    exit non-zero. An optional mutation target or saved earlier result cannot substitute.
- id: NFR-006-M-1
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Merged pull requests touching the Kani-touching set without an actual full gate passing on
    the current candidate immediately before merge
- id: NFR-006-M-2
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Lane runs reported `passed` with `ran` different from `expected` or `ran` of 0
- id: NFR-006-M-3
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`#[ignore = "kani lane: ..."]` tests the lane does not select'
findings:
- fnd: FND-307
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: NFR-004-M-3
  related_ids: []
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  lines: '1'
  excerpt: Campaigns ending `Exhausted` on the same campaigns, and on 256-case default-config campaigns
  finding: 'Inherited NFR-004-M-3 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-308
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: NFR-004-M-4
  related_ids: []
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  lines: '1'
  excerpt: In-domain plus out-of-domain census cases per admitted clause
  finding: 'Inherited NFR-004-M-4 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-309
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: NFR-006-M-1
  related_ids: []
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  lines: '1'
  excerpt: Merged pull requests touching the Kani-touching set without an actual full gate passing on
    the current candidate immediately before merge
  finding: 'Inherited NFR-006-M-1 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-310
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: NFR-006-M-2
  related_ids: []
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  lines: '1'
  excerpt: Lane runs reported `passed` with `ran` different from `expected` or `ran` of 0
  finding: 'Inherited NFR-006-M-2 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-311
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: NFR-006-M-3
  related_ids: []
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  lines: '1'
  excerpt: '`#[ignore = "kani lane: ..."]` tests the lane does not select'
  finding: 'Inherited NFR-006-M-3 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-312
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-8
  related_ids: []
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '676'
  excerpt: Emit only `check_equality`, omitting the `check_type` calls; `check_equality` consults the
    environment solely through `contains_ieee`, which returns `false` for an absent composite key, so
    an empty environment completes with a Boolean instead of refusing. Or emit `.expect(..)`, which panics
    instead; count a private duplicate-member defense as public source-authenticated acceptance, or replace
    its precise private cause with a generic context mismatch; invent a public Assembly cause tag where
    CompileRefusal::cause() returns None.
  finding: Inherited Genuine duplicate-source admission-stage evidence remains absent; existing synthetic
    generator tags do not discharge the owning reader-intake prerequisite. Owner IR-17; existing IR-643
    recursion ownership is unchanged.
- fnd: FND-313
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-16
  related_ids: []
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '684'
  excerpt: Read a `bounded_domain` member as its base scalar's unbounded type or through the union of
    every bound over that base, take the first sibling bound, or skip the form check, so a mismatched
    bound (`text_bounds` over an integer, a bound over a boolean) generates as an unbounded type instead
    of being refused, or refuse a `float_rounding` member as unsupported so the IEEE refusal never fires.
    Or count private two-form/malformed-member controls as genuine public acceptance, drop their exact
    assertions, or claim an unrelated sibling bound proves public member ownership.
  finding: Inherited Exact public bounded-positive versus private two-form-equivalence and malformed-member
    stage allocation remains unqualified; synthetic-public tags do not establish original bounds/order/rounding
    or precise private refusal/no-code/sibling controls. Owner IR-17; existing IR-643 recursion ownership
    is unchanged.
- fnd: FND-314
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-24
  related_ids: []
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '692'
  excerpt: Recurse from `Option` into itself without tracking an unclosed type, or reject every repeated
    node including the List and Tree record back-edges; the former overflows the stack and the latter
    refuses valid recursive records.
  finding: Inherited Exact external QSpec strict-reader/shape and separately authored genuine-source public-generation
    stages remain unqualified; appended fixtures and absent-QSPEC early returns cannot discharge them.
    Owner IR-17; existing IR-643 recursion ownership is unchanged.
- fnd: FND-315
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-17
  related_ids: []
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '685'
  excerpt: Emit `.expect(..)` in the `integer` helper or in a rendered `RationalDomain`, `TextType`, `CardinalityBound`,
    `IntegerInterval` or `DecimalType` reconstruction, so a runtime release that tightens a constructor
    panics a generated oracle; or replace it with `.ok()` or `.unwrap_or(..)` to an unbounded type, so
    a failed reconstruction silently widens the compared type; or report it as an `InvalidDeclaration`
    with a cause the runtime does not state; or have a helper recover a failed constructor (`.or_else(..)`
    to an unbounded bound) or fail with another helper's variant.
  finding: 'Inherited Composite reconstruction mapping remains unimplemented on this branch: emitted header
    and structural control still enumerate six variants and legacy Declaration(InvalidDeclaration); separate
    unqualified IR17 draft source is not acceptance evidence. Owner IR-17; existing IR-643 recursion ownership
    is unchanged.'
- fnd: FND-316
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: src/routed/contract_package_tests.rs::tests::duplicate_expression_uses_each_selected_conversion_context
  related_ids:
  - FR-049-AC-1
  path: src/routed/contract_package_tests.rs
  lines: '1667'
  excerpt: '"FR-049-AC-1",'
  finding: Inherited test tests::duplicate_expression_uses_each_selected_conversion_context carries untracked
    tag FR-049-AC-1, which declares no local criterion and binds no obligation. This unchanged tag must
    not be counted as criterion coverage. Root decision 7ba6731d explicitly assigns this pre-existing
    mapping to existing IR-878 repository trace/spec-defect triage; it is not claimed previously enumerated
    or fixed.
- fnd: FND-317
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: src/routed/contract_package_tests.rs::tests::duplicate_expression_uses_each_selected_conversion_context
  related_ids:
  - FR-049-AC-5
  path: src/routed/contract_package_tests.rs
  lines: '1667'
  excerpt: '"FR-049-AC-5",'
  finding: Inherited test tests::duplicate_expression_uses_each_selected_conversion_context carries untracked
    tag FR-049-AC-5, which declares no local criterion and binds no obligation. This unchanged tag must
    not be counted as criterion coverage. Root decision 7ba6731d explicitly assigns this pre-existing
    mapping to existing IR-878 repository trace/spec-defect triage; it is not claimed previously enumerated
    or fixed.
- fnd: FND-318
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: src/routed/contract_package_tests.rs::tests::scalar_own_result_without_conversion_differs_from_not_applicable
  related_ids:
  - FR-049-AC-1
  path: src/routed/contract_package_tests.rs
  lines: '1582'
  excerpt: '"FR-049-AC-1",'
  finding: Inherited test tests::scalar_own_result_without_conversion_differs_from_not_applicable carries
    untracked tag FR-049-AC-1, which declares no local criterion and binds no obligation. This unchanged
    tag must not be counted as criterion coverage. Root decision 7ba6731d explicitly assigns this pre-existing
    mapping to existing IR-878 repository trace/spec-defect triage; it is not claimed previously enumerated
    or fixed.
- fnd: FND-319
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: tests/it/oracle_generation.rs::tc_001_boolean_oracle_bundle_is_deterministic_traceable_and_schema_valid
  related_ids:
  - AD-004
  path: tests/it/oracle_generation.rs
  lines: '532'
  excerpt: '// Trace: AD-004 L-9, TC-001.'
  finding: Inherited test tc_001_boolean_oracle_bundle_is_deterministic_traceable_and_schema_valid carries
    untracked tag AD-004, which declares no local criterion and binds no obligation. This unchanged tag
    must not be counted as criterion coverage. Root decision 7ba6731d explicitly assigns this pre-existing
    mapping to existing IR-878 repository trace/spec-defect triage; it is not claimed previously enumerated
    or fixed.
bindings: []
review_head: 3d303ea6988496c4e1102faa58a56c1f260193c3
review_verdict: 'FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence
  gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls
  now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.'
part: 18
parts: 18
scope_kind: full-repository criterion presence; no blanket semantic acceptance
```

+++