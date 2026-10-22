---
id: SR-15641
title: IR-258 PR535 native prerequisite gap-analysis
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: agent-ix/quire-contract-codegen; IR-258; frozen PR535; src/kani/generate/mod.rs,
  src/kani/generate/outcome.rs, src/kani/generate/rational_predicate.rs, src/lib.rs,
  src/oracle/mod.rs, src/oracle/rational_predicate.rs, tests/it/main.rs, tests/it/rational_predicate_agreement.rs,
  tests/it/rational_predicate_generation.rs, tests/it/rational_predicate_support/fixtures/commutative.native,
  tests/it/rational_predicate_support/fixtures/large_intermediate.native, tests/it/rational_predicate_support/fixtures/wrong_addend.native,
  tests/it/rational_predicate_support/mod.rs
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-046
  type: references
- target: ix://agent-ix/quire-contract-codegen/FR-025
  type: references
---

## Summary

Ticket: IR-258. Independently reviewed exact 24c5e8b46ea5287d93a1479c0276323c0c3345d8 against f93f2715cfebe13a0a21c03f23c27dd5540a0305. Scope is only the 13-path native prerequisite; no spec/plan diff, so spec-review and its subanalyses are not applicable. Code review includes the Rust lane in this artifact; gap analysis is separate.

## Verdict

**FAIL** — Four scoped criteria remain untagged and all eleven full feature obligations remain OPEN. These are explicit feature debt, not a demand to invent proof credit or broaden this native prerequisite. Native acceptance and full-feature acceptance remain separate.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | The shared negotiator and actual whole-function proof harness are absent; source preparation is not predicate proof selection. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md:145 | correct-requirement-no-evidence |
| FND-002 | medium | Native canonical admission and exhaustive tuples exist, but actual symbolic draws/constraints and persisted proof correspondence are absent. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md:146 | correct-requirement-no-evidence |
| FND-003 | medium | Native body/Meter agreement is substantive, but the integrated selected-function path and its invocation accounting are absent. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md:147 | correct-requirement-no-evidence |
| FND-004 | medium | Native incomplete stops are compared, but actual proof assertion/assumption controls and replay settlement are absent. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md:148 | correct-requirement-no-evidence |
| FND-005 | high | No actual healthy proof, mutant solver playback or public false replay exists; this criterion is untagged. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md:149 | correct-requirement-no-evidence |
| FND-006 | high | No emitted predicate harness, single final completion cover or actual run/classification controls exist; this criterion is untagged. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md:150 | correct-requirement-no-evidence |
| FND-007 | medium | The out-of-i64 endpoint is tested, but genuine public unbounded-parameter generation/refusal is absent. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md:151 | correct-requirement-no-evidence |
| FND-008 | medium | Foreign selection and unsupported operations are tested, but total multi-item disposition and supported-sibling negotiation controls are absent. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md:152 | correct-requirement-no-evidence |
| FND-009 | high | The physical backend decoder, grouped schema validation, selected-application binding and public Rational replay are absent; this criterion is untagged. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md:153 | correct-requirement-no-evidence |
| FND-010 | high | No feature runner/report record with actual versions, checks, cover, playback and ordered options exists; this criterion is untagged. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md:154 | correct-requirement-no-evidence |
| FND-011 | medium | Native grouping and canonical pair admission exist, but persisted physical proof/playback and complete public witness roundtrip are absent. | spec/kani/functional/FR-025-generated-subject-abi.md:134 | correct-requirement-no-evidence |

## Coverage

Plan completion: not assessed

Reconciliation: quire matrix --scope . --format json (computed matrix retained in scratch); no run evidence read through Quoin. Scoped FR-046/FR-025-AC-10: 7 tagged, 4 untagged, 0 tagged-by-ignored-test. Tags identify relevant partial tests, not complete discharge. Bindings below are correct associations with partial coverage. All other matrix rows are outside this incremental review verdict. Semantic intent review was explicitly requested by the dispatch and performed within code review; this is not a repository-wide gap assurance claim.

No applicable AssuranceProfile exists in the reviewed spec tree. No CI, manifests, lock, existing ABI/identity/serde, negotiate.rs, runner/report, replay or core accounting changes occur. No test flags, warnings or gate settings were weakened. Existing Boolean/I64 contracts remain untouched.

The owner-admitted reached closure provides the root, true parameter/body/type IDs and source association. FunctionPredicateItem only selects an admitted package/root; it is not a new owner claim API. The renderer performs left-to-right occurrence-preserving postorder, executes repeated additions separately and propagates all owner typed stops. Intermediate domains come from original result types, not input clipping. Domain strings are parsed as exact Integer before rendering. Existing source/work bounds terminate preparation; no unsafe, narrowing cast, async/lock or panic path was added in production.

Canonical admission calls Rational::new and checks both original accessors; zero is only 0/1. Parameters are private in the oracle and populated from owner records, sorted by real node ID; physical positions group two I64 roles under each original parameter. The new native input-refusal API needs the adverse controls in code-review FND-002.

Independent reference trees use owner operations and never call the generated oracle to derive expectations. Public QSL call_site parameter identities join reference a/b by name and node rather than declaration order or generated metadata. Both original fixtures are byte-identical to QSL db729fe811e3927d2ed45fe7a111dfe895cd939f. Native all-domain checks measure 111 values, 12,321 tuples, healthy false=0 and mutant false=12,210; the coherent nested fixture has an actual beyond-i128 intermediate and repeated expressions. Every recorded named charge occurrence is denied independently; full typed outcomes, ordered admitted charges, all ten consumed counters and admission count are compared, with zero/exact/short limits for every counter. Input-domain restrictions do not bound intermediate sums. Reachable body stop fixtures here demonstrate Incomplete; Undefined/Refused forwarding is structurally present, not claimed as independently realized authentic body evidence. Cancellation evidence is limited to actual owner Meter stop controls.

Duplication search: src/oracle and src/kani/generate rational-domain helpers and member vocabularies; existing src/oracle/scalar/mod.rs:2867 RATIONAL_RANGE_MEMBERS is already pub(crate). No independent arithmetic/GCD implementation or fabricated checked node IDs were introduced. Fixture provenance is required by the dispatch.

Verification: read actual leaf-generation-red.log (UnsupportedOperation native leaf failure), component-policy-red.log (0/2 normalization-only failure), leaf-owner-join-final.log (6 focused tests, nested/native child runs pass), leaf-binding-schema-final.log (4 controls pass), leaf-committed-clippy.log (lib/it check completed) and leaf-committed-fmt.log (empty). Freeze receipt reports exit0 for lint/fmt/diff. These are supplied author logs, not independent command reruns. No Cargo, Kani, CBMC, aggregate or namespace execution was performed by this reviewer. The initial mixed-intermediate QSL admission failure is not product RED; neither RED is negotiated-selection RED. No final-gate or full-feature acceptance credit.

## Scope and machine record

```yaml
clean: false
scope:
- id: FR-046-AC-1
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: 'The genuine source function `rational_commutative(a: Rational[-9,9;1,9], b: Rational[-9,9;1,9]): Boolean`
    with body `a + b = b + a` selects its whole declaration and complete body through the public QSL admission,
    strict IR reader/lowering and CG route. Its artifact proves that predicate, not only validity or parity of either
    addition. Replacing the body by `a + b = a + a` selects the same authored mutation with unchanged parameter
    domains.'
- id: FR-046-AC-2
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Every accepted symbolic argument has canonical constructor/accessor roundtrip equality and lies in its
    original inclusive component domains. Its two physical I64 bindings retain numerator/denominator roles, original
    parameter identity and bounds in the FR-025 order. Every semantic value in those domains has exactly one accepted
    representation. Zero is `0/1`; negative, fractional and boundary values remain reachable. For the original domain
    there are 111 canonical values per argument and 12,321 argument tuples; admitting all 171 raw component pairs,
    restricting denominators to one, or clipping an endpoint fails correspondence validation under QSpec FR-196-AC-5.
- id: FR-046-AC-3
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Generated evaluation preserves the authored full tree and declaration identity, exact owner arithmetic,
    intermediate values outside the input domains, arbitrary-precision continuation, full typed outcomes and Meter
    charge/stop order. Native comparison against an independent owner-operation reference checks those observations,
    including each denied charge prefix and exact/short budgets; it does not calculate expectations by running the
    generated oracle twice.
- id: FR-046-AC-4
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Generation introduces no assumption on completion, truth or an absence of undefined/refused/incomplete
    results. Controls producing each reachable noncompletion retain that typed outcome and cannot become a successful
    predicate proof; an outcome-based assume mutation is rejected. A completion assertion failure without a replayed
    false value is not labelled a confirmed semantic false counterexample.
- id: FR-046-AC-5
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: The healthy original function receives `Verified` only from an actual successful proof with all required
    checks and its completion cover satisfied. The wrong-addend mutation produces a substantive failed predicate
    assertion with actual solver playback of canonical in-domain arguments, and replay of that same authored function
    completes with Boolean false. A hypothetical witness or cover witness is not substituted for that playback.
- id: FR-046-AC-6
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Each emitted predicate harness has exactly one final completion cover after all predicate assertions,
    including harnesses with representation constraints. Unsatisfied or unreported completion cover, failed unwind,
    timeout, memory exhaustion or missing verdict cannot yield `Verified`; classifications retain FR-017 and FR-029
    distinctions.
- id: FR-046-AC-7
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: A missing or unbounded Rational parameter domain yields source-associated `requires-bound` naming the
    selected function, argument and missing domain, with no harness. A finite component endpoint outside this encoding's
    signed-i64 capacity yields named `unsupported`, retaining the original endpoint and no harness; generation never
    clips, casts away precision or substitutes a smaller domain.
- id: FR-046-AC-8
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: An unavailable owner-lowered whole-function representation, absent one-to-one correspondence or unsupported
    body operation yields named `unsupported` with its source-associated cause and no harness. Every requested item
    has exactly one disposition under FR-039; a refused item does not remove or alter a supported sibling. Malformed
    source or owner-refused packages remain upstream refusals, not fabricated CG claims.
- id: FR-046-AC-9
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Counterexample decoding validates the two physical I64 entries per original parameter, exact width/comment/order,
    schema roles, required physical-entry and byte limits, canonical components, original domains and selected declaration/application
    binding before public owner replay. Grouping yields one typed owner Rational value per parameter through the
    existing canonical serialization seam; missing, duplicate, foreign, role-swapped, domain-conflicting and malformed
    component schemas or entries retain typed refusals with no replayed failure. Out-of-domain, duplicate-zero,
    noncanonical, wrong-declaration and stale-identity witnesses are rejected by the responsible boundary; CG neither
    mints keys nor substitutes scalar application validity for the authored Boolean result. An unavailable replay
    capability is reported without claiming replay acceptance.
- id: FR-046-AC-10
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Run evidence records the selected function and source association, inclusive component domains and correspondence,
    ordered physical bindings and their logical parameter grouping, actual subject and full ordered backend arguments,
    every unwind/loop bound and option, Kani/Rust/CBMC/solver versions, target, actual dependency cohort and execution
    limits, raw checks/cover/playback, exit and typed terminal outcome. Replay uses the retained generated artifact
    without hand editing. No-run, compile refusal, cover failure, resource exhaustion and semantic falsification
    remain distinguishable; preparation alone earns no proof result.
- id: FR-025-AC-10
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: PLANNED (IR-258). Every supported FR-046 Rational argument has exactly two ordered I64 physical bindings
    grouped to its one original parameter, complete domain and canonical value. Actual playback decodes losslessly
    with existing I64 width/comment checks and physical byte/value-count caps, then reconstructs and submits exactly
    one canonical owner Rational per parameter. Missing, duplicate, foreign, role-swapped or bound-conflicting schema,
    malformed entries, noncanonical pairs and over-limit witnesses refuse at their responsible typed boundary; no
    alternate normalized witness is replayed. Existing Boolean/integer controls remain unchanged.
- id: src/kani/generate/mod.rs
  path: src/kani/generate/mod.rs
  role: examined
  excerpt: '//! The one Kani generator: the request and result vocabulary, the negotiation entry and the'
- id: src/kani/generate/outcome.rs
  path: src/kani/generate/outcome.rs
  role: examined
  excerpt: '//! The request and result vocabulary of Kani obligation negotiation (FR-015): the items a'
- id: src/kani/generate/rational_predicate.rs
  path: src/kani/generate/rational_predicate.rs
  role: examined
  excerpt: //! Physical I64 positions for one prepared Rational predicate's logical arguments.
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: //! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.
- id: src/oracle/mod.rs
  path: src/oracle/mod.rs
  role: examined
  excerpt: '//! The oracle subsystem: generation of the Rust oracles (AD-004, FR-014, FR-018, FR-021).'
- id: src/oracle/rational_predicate.rs
  path: src/oracle/rational_predicate.rs
  role: examined
  excerpt: //! Whole-function Rational predicate native source preparation (FR-046).
- id: tests/it/main.rs
  path: tests/it/main.rs
  role: examined
  excerpt: '//! IR-237: the integration test crate. Cargo builds a `tests/<name>/main.rs` directory as one'
- id: tests/it/rational_predicate_agreement.rs
  path: tests/it/rational_predicate_agreement.rs
  role: examined
  excerpt: //! Native body agreement, distinct from public replay and generated-proof acceptance.
- id: tests/it/rational_predicate_generation.rs
  path: tests/it/rational_predicate_generation.rs
  role: examined
  excerpt: //! Leaf native-source preparation only; negotiated generation/replay are not yet integrated.
- id: tests/it/rational_predicate_support/fixtures/commutative.native
  path: tests/it/rational_predicate_support/fixtures/commutative.native
  role: examined
  excerpt: language "ix:native" edition "1-draft";
- id: tests/it/rational_predicate_support/fixtures/large_intermediate.native
  path: tests/it/rational_predicate_support/fixtures/large_intermediate.native
  role: examined
  excerpt: language "ix:native" edition "1-draft";
- id: tests/it/rational_predicate_support/fixtures/wrong_addend.native
  path: tests/it/rational_predicate_support/fixtures/wrong_addend.native
  role: examined
  excerpt: language "ix:native" edition "1-draft";
- id: tests/it/rational_predicate_support/mod.rs
  path: tests/it/rational_predicate_support/mod.rs
  role: examined
  excerpt: //! Authentic source admission helpers; no prebaked graph or minted node identities.
- id: spec/kani/matrix/TC-060-rational-function-predicate-harnesses.md
  path: spec/kani/matrix/TC-060-rational-function-predicate-harnesses.md
  role: context_only
  excerpt: '# TC-060: Verify finite rational whole-function predicates'
- id: spec/kani/matrix/TC-036-generated-subject-abi.md
  path: spec/kani/matrix/TC-036-generated-subject-abi.md
  role: context_only
  excerpt: '# TC-036: Verify the generated harness subject ABI'
- id: spec/decisions/ADR-004-generated-subject-abi-open-decisions.md
  path: spec/decisions/ADR-004-generated-subject-abi-open-decisions.md
  role: context_only
  excerpt: '# ADR-004: Decisions of the generated harness subject ABI'
findings:
- confidence: high
  related_ids: &id001 []
  fnd: FND-001
  severity: medium
  method: gap-analysis
  check: coverage
  artifact_id: FR-046-AC-1
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  lines: '145'
  excerpt: 'The genuine source function `rational_commutative(a: Rational[-9,9;1,9], b: Rational[-9,9;1,9]): Boolean`
    with body `a + b = b + a` selects its whole declaration and complete body through the public QSL admission,
    strict IR reader/lowering and CG route. Its artifact proves that predicate, not only validity or parity of either
    addition. Replacing the body by `a + b = a + a` selects the same authored mutation with unchanged parameter
    domains.'
  finding: The shared negotiator and actual whole-function proof harness are absent; source preparation is not predicate
    proof selection.
- confidence: high
  related_ids: *id001
  fnd: FND-002
  severity: medium
  method: gap-analysis
  check: coverage
  artifact_id: FR-046-AC-2
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  lines: '146'
  excerpt: Every accepted symbolic argument has canonical constructor/accessor roundtrip equality and lies in its
    original inclusive component domains. Its two physical I64 bindings retain numerator/denominator roles, original
    parameter identity and bounds in the FR-025 order. Every semantic value in those domains has exactly one accepted
    representation. Zero is `0/1`; negative, fractional and boundary values remain reachable. For the original domain
    there are 111 canonical values per argument and 12,321 argument tuples; admitting all 171 raw component pairs,
    restricting denominators to one, or clipping an endpoint fails correspondence validation under QSpec FR-196-AC-5.
  finding: Native canonical admission and exhaustive tuples exist, but actual symbolic draws/constraints and persisted
    proof correspondence are absent.
- confidence: high
  related_ids: *id001
  fnd: FND-003
  severity: medium
  method: gap-analysis
  check: coverage
  artifact_id: FR-046-AC-3
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  lines: '147'
  excerpt: Generated evaluation preserves the authored full tree and declaration identity, exact owner arithmetic,
    intermediate values outside the input domains, arbitrary-precision continuation, full typed outcomes and Meter
    charge/stop order. Native comparison against an independent owner-operation reference checks those observations,
    including each denied charge prefix and exact/short budgets; it does not calculate expectations by running the
    generated oracle twice.
  finding: Native body/Meter agreement is substantive, but the integrated selected-function path and its invocation
    accounting are absent.
- confidence: high
  related_ids: *id001
  fnd: FND-004
  severity: medium
  method: gap-analysis
  check: coverage
  artifact_id: FR-046-AC-4
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  lines: '148'
  excerpt: Generation introduces no assumption on completion, truth or an absence of undefined/refused/incomplete
    results. Controls producing each reachable noncompletion retain that typed outcome and cannot become a successful
    predicate proof; an outcome-based assume mutation is rejected. A completion assertion failure without a replayed
    false value is not labelled a confirmed semantic false counterexample.
  finding: Native incomplete stops are compared, but actual proof assertion/assumption controls and replay settlement
    are absent.
- confidence: high
  related_ids: *id001
  fnd: FND-005
  severity: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-046-AC-5
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  lines: '149'
  excerpt: The healthy original function receives `Verified` only from an actual successful proof with all required
    checks and its completion cover satisfied. The wrong-addend mutation produces a substantive failed predicate
    assertion with actual solver playback of canonical in-domain arguments, and replay of that same authored function
    completes with Boolean false. A hypothetical witness or cover witness is not substituted for that playback.
  finding: No actual healthy proof, mutant solver playback or public false replay exists; this criterion is untagged.
- confidence: high
  related_ids: *id001
  fnd: FND-006
  severity: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-046-AC-6
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  lines: '150'
  excerpt: Each emitted predicate harness has exactly one final completion cover after all predicate assertions,
    including harnesses with representation constraints. Unsatisfied or unreported completion cover, failed unwind,
    timeout, memory exhaustion or missing verdict cannot yield `Verified`; classifications retain FR-017 and FR-029
    distinctions.
  finding: No emitted predicate harness, single final completion cover or actual run/classification controls exist;
    this criterion is untagged.
- confidence: high
  related_ids: *id001
  fnd: FND-007
  severity: medium
  method: gap-analysis
  check: coverage
  artifact_id: FR-046-AC-7
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  lines: '151'
  excerpt: A missing or unbounded Rational parameter domain yields source-associated `requires-bound` naming the
    selected function, argument and missing domain, with no harness. A finite component endpoint outside this encoding's
    signed-i64 capacity yields named `unsupported`, retaining the original endpoint and no harness; generation never
    clips, casts away precision or substitutes a smaller domain.
  finding: The out-of-i64 endpoint is tested, but genuine public unbounded-parameter generation/refusal is absent.
- confidence: high
  related_ids: *id001
  fnd: FND-008
  severity: medium
  method: gap-analysis
  check: coverage
  artifact_id: FR-046-AC-8
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  lines: '152'
  excerpt: An unavailable owner-lowered whole-function representation, absent one-to-one correspondence or unsupported
    body operation yields named `unsupported` with its source-associated cause and no harness. Every requested item
    has exactly one disposition under FR-039; a refused item does not remove or alter a supported sibling. Malformed
    source or owner-refused packages remain upstream refusals, not fabricated CG claims.
  finding: Foreign selection and unsupported operations are tested, but total multi-item disposition and supported-sibling
    negotiation controls are absent.
- confidence: high
  related_ids: *id001
  fnd: FND-009
  severity: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-046-AC-9
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  lines: '153'
  excerpt: Counterexample decoding validates the two physical I64 entries per original parameter, exact width/comment/order,
    schema roles, required physical-entry and byte limits, canonical components, original domains and selected declaration/application
    binding before public owner replay. Grouping yields one typed owner Rational value per parameter through the
    existing canonical serialization seam; missing, duplicate, foreign, role-swapped, domain-conflicting and malformed
    component schemas or entries retain typed refusals with no replayed failure. Out-of-domain, duplicate-zero,
    noncanonical, wrong-declaration and stale-identity witnesses are rejected by the responsible boundary; CG neither
    mints keys nor substitutes scalar application validity for the authored Boolean result. An unavailable replay
    capability is reported without claiming replay acceptance.
  finding: The physical backend decoder, grouped schema validation, selected-application binding and public Rational
    replay are absent; this criterion is untagged.
- confidence: high
  related_ids: *id001
  fnd: FND-010
  severity: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-046-AC-10
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  lines: '154'
  excerpt: Run evidence records the selected function and source association, inclusive component domains and correspondence,
    ordered physical bindings and their logical parameter grouping, actual subject and full ordered backend arguments,
    every unwind/loop bound and option, Kani/Rust/CBMC/solver versions, target, actual dependency cohort and execution
    limits, raw checks/cover/playback, exit and typed terminal outcome. Replay uses the retained generated artifact
    without hand editing. No-run, compile refusal, cover failure, resource exhaustion and semantic falsification
    remain distinguishable; preparation alone earns no proof result.
  finding: No feature runner/report record with actual versions, checks, cover, playback and ordered options exists;
    this criterion is untagged.
- confidence: high
  related_ids: *id001
  fnd: FND-011
  severity: medium
  method: gap-analysis
  check: coverage
  artifact_id: FR-025-AC-10
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  lines: '134'
  excerpt: PLANNED (IR-258). Every supported FR-046 Rational argument has exactly two ordered I64 physical bindings
    grouped to its one original parameter, complete domain and canonical value. Actual playback decodes losslessly
    with existing I64 width/comment checks and physical byte/value-count caps, then reconstructs and submits exactly
    one canonical owner Rational per parameter. Missing, duplicate, foreign, role-swapped or bound-conflicting schema,
    malformed entries, noncanonical pairs and over-limit witnesses refuse at their responsible typed boundary; no
    alternate normalized witness is replayed. Existing Boolean/integer controls remain unchanged.
  finding: Native grouping and canonical pair admission exist, but persisted physical proof/playback and complete
    public witness roundtrip are absent.
bindings:
- test_id: authentic_complete_predicate_prepares_native_source_and_grouped_arguments
  ac_id: FR-025-AC-10
  trace: correct
- test_id: physical_component_admission_is_bijective_and_rejects_normalization_duplicates
  ac_id: FR-025-AC-10
  trace: correct
- test_id: authentic_complete_predicate_prepares_native_source_and_grouped_arguments
  ac_id: FR-046-AC-1
  trace: correct
- test_id: a_foreign_real_function_selection_cannot_replace_the_admitted_closure
  ac_id: FR-046-AC-1
  trace: correct
- test_id: generated_native_bodies_preserve_exact_owner_results_and_all_charge_prefixes
  ac_id: FR-046-AC-2
  trace: correct
- test_id: authentic_complete_predicate_prepares_native_source_and_grouped_arguments
  ac_id: FR-046-AC-2
  trace: correct
- test_id: physical_component_admission_is_bijective_and_rejects_normalization_duplicates
  ac_id: FR-046-AC-2
  trace: correct
- test_id: generated_native_bodies_preserve_exact_owner_results_and_all_charge_prefixes
  ac_id: FR-046-AC-3
  trace: correct
- test_id: admitted_nested_addition_preserves_arbitrary_precision_and_stops
  ac_id: FR-046-AC-3
  trace: correct
- test_id: generated_native_bodies_preserve_exact_owner_results_and_all_charge_prefixes
  ac_id: FR-046-AC-4
  trace: correct
- test_id: admitted_nested_addition_preserves_arbitrary_precision_and_stops
  ac_id: FR-046-AC-4
  trace: correct
- test_id: original_capacity_and_operation_refusals_preserve_the_owner_input
  ac_id: FR-046-AC-7
  trace: correct
- test_id: original_capacity_and_operation_refusals_preserve_the_owner_input
  ac_id: FR-046-AC-8
  trace: correct
- test_id: a_foreign_real_function_selection_cannot_replace_the_admitted_closure
  ac_id: FR-046-AC-8
  trace: correct
```

## Dispositions

Disposition pass 1, exact frozen b41bb3e50a7403dd0f27f2f90d0bb6d865a181d3 on actual main 59d89b64e90d68f4b19d1233ed1a48d91af1c69c. Original findings and earlier review text above remain unchanged.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | Future IR-258 implementation: The shared negotiator and actual whole-function proof harness are absent; source preparation is not predicate proof selection. This repair is limited to native prerequisite quality; the full criterion remains OPEN. No scope waiver, proof credit or full-feature acceptance. |
| FND-002 | deferred | Future IR-258 implementation: Native canonical admission and exhaustive tuples exist, but actual symbolic draws/constraints and persisted proof correspondence are absent. This repair is limited to native prerequisite quality; the full criterion remains OPEN. No scope waiver, proof credit or full-feature acceptance. |
| FND-003 | deferred | Future IR-258 implementation: Native body/Meter agreement is substantive, but the integrated selected-function path and its invocation accounting are absent. This repair is limited to native prerequisite quality; the full criterion remains OPEN. No scope waiver, proof credit or full-feature acceptance. |
| FND-004 | deferred | Future IR-258 implementation: Native incomplete stops are compared, but actual proof assertion/assumption controls and replay settlement are absent. This repair is limited to native prerequisite quality; the full criterion remains OPEN. No scope waiver, proof credit or full-feature acceptance. |
| FND-005 | deferred | Future IR-258 implementation: No actual healthy proof, mutant solver playback or public false replay exists; this criterion is untagged. This repair is limited to native prerequisite quality; the full criterion remains OPEN. No scope waiver, proof credit or full-feature acceptance. |
| FND-006 | deferred | Future IR-258 implementation: No emitted predicate harness, single final completion cover or actual run/classification controls exist; this criterion is untagged. This repair is limited to native prerequisite quality; the full criterion remains OPEN. No scope waiver, proof credit or full-feature acceptance. |
| FND-007 | deferred | Future IR-258 implementation: The out-of-i64 endpoint is tested, but genuine public unbounded-parameter generation/refusal is absent. This repair is limited to native prerequisite quality; the full criterion remains OPEN. No scope waiver, proof credit or full-feature acceptance. |
| FND-008 | deferred | Future IR-258 implementation: Foreign selection and unsupported operations are tested, but total multi-item disposition and supported-sibling negotiation controls are absent. This repair is limited to native prerequisite quality; the full criterion remains OPEN. No scope waiver, proof credit or full-feature acceptance. |
| FND-009 | deferred | Future IR-258 implementation: The physical backend decoder, grouped schema validation, selected-application binding and public Rational replay are absent; this criterion is untagged. This repair is limited to native prerequisite quality; the full criterion remains OPEN. No scope waiver, proof credit or full-feature acceptance. |
| FND-010 | deferred | Future IR-258 implementation: No feature runner/report record with actual versions, checks, cover, playback and ordered options exists; this criterion is untagged. This repair is limited to native prerequisite quality; the full criterion remains OPEN. No scope waiver, proof credit or full-feature acceptance. |
| FND-011 | deferred | Future IR-258 implementation: Native grouping and canonical pair admission exist, but persisted physical proof/playback and complete public witness roundtrip are absent. This repair is limited to native prerequisite quality; the full criterion remains OPEN. No scope waiver, proof credit or full-feature acceptance. |

### Disposition pass 1 evidence and current verdict

**FAIL remains the full scoped feature-gap verdict.** Four criteria remain untagged; all eleven full feature obligations remain OPEN. All eleven findings are deferred accurately to future IR-258 integration, rather than rejected or claimed fixed. This does not veto the separately reviewed native prerequisite quality milestone or waive delivery debt.

Reviewed the complete current 15-path PR diff and the fresh-base interactions. Original two authored commits retain equality in range-diff. Incoming main changes only scalar lookup reuse/classification paths and their tests/reviews; all six incoming paths match actual main, including negotiate.rs and scalar/mod.rs. The native leaf still consumes the owner reached closure, independently of those shared scalar negotiation caches. The pre-existing pub(crate) Rational member constant remains present and unchanged; both new uses reuse it. No FunctionPredicate negotiator arm, manifest/lock, ABI/identity/serde, replay, runner or proof change was introduced. Original native implementation/tests/fixtures outside the constant repair and appended adverse test remain byte-identical. Exact source fixtures, full-domain reference comparisons, repeated-expression/BigInt and all Meter denial/budget controls retain their original semantics.

The new generated-native test runs both authentic predicates and checks 0/1/3 arguments plus canonical -10/1, 10/1 and 1/10 at each of two real parameter positions on fresh and genuinely pre-used Meters. This is 18 cases per predicate, 36 total. The pre-used Meter is charged via actual owner planned_equality, not fabricated counters. Exact typed error/index and the unchanged ordered charges, all ten counters, admissions and truncation state are asserted. It exercises actual compiled generated source; the historical behavior was already correct, so this is coverage repair, not a claimed old product RED.

Read raw pr535-fix1/native.log: seven focused outer tests pass, including two independently compiled adverse child suites reporting 18 cases each; original 111/12321 healthy/mutant and beyond-i128 controls pass. Read clippy.log: lib/it check completes; check-exits.json records exit0 for clippy, native, fmt, diff and Quire. No independent Cargo/proof/aggregate/namespace rerun was needed. These remain author execution logs; this reviewer independently inspected source, diffs, lineage/custody, actual PR head/current remote main, diff-check and computed matrix. The retained native-scope.txt reports an inactive unit with MemoryMax=infinity and MemorySwapMax=infinity, so that file does not independently establish the live execution caps claimed by the handoff. No new RAM policy or execution authorization is inferred.

Fresh computed matrix remains 7 tagged / 4 untagged for the eleven scoped criteria. Full feature gaps are unchanged. Plan completion: not assessed. Spec/plan methods remain N/A because this PR changes neither. Original SR imports matched the prior exact source bytes before this append. Updated scratch copies must replace the committed originals before custody is complete; reviewer does not edit or push the author branch.

```yaml
dispositions:
- fnd: FND-001
  outcome: deferred
  reason: 'Future IR-258 implementation: The shared negotiator and actual whole-function proof harness are absent;
    source preparation is not predicate proof selection. This repair is limited to native prerequisite quality;
    the full criterion remains OPEN. No scope waiver, proof credit or full-feature acceptance.'
- fnd: FND-002
  outcome: deferred
  reason: 'Future IR-258 implementation: Native canonical admission and exhaustive tuples exist, but actual symbolic
    draws/constraints and persisted proof correspondence are absent. This repair is limited to native prerequisite
    quality; the full criterion remains OPEN. No scope waiver, proof credit or full-feature acceptance.'
- fnd: FND-003
  outcome: deferred
  reason: 'Future IR-258 implementation: Native body/Meter agreement is substantive, but the integrated selected-function
    path and its invocation accounting are absent. This repair is limited to native prerequisite quality; the full
    criterion remains OPEN. No scope waiver, proof credit or full-feature acceptance.'
- fnd: FND-004
  outcome: deferred
  reason: 'Future IR-258 implementation: Native incomplete stops are compared, but actual proof assertion/assumption
    controls and replay settlement are absent. This repair is limited to native prerequisite quality; the full criterion
    remains OPEN. No scope waiver, proof credit or full-feature acceptance.'
- fnd: FND-005
  outcome: deferred
  reason: 'Future IR-258 implementation: No actual healthy proof, mutant solver playback or public false replay
    exists; this criterion is untagged. This repair is limited to native prerequisite quality; the full criterion
    remains OPEN. No scope waiver, proof credit or full-feature acceptance.'
- fnd: FND-006
  outcome: deferred
  reason: 'Future IR-258 implementation: No emitted predicate harness, single final completion cover or actual run/classification
    controls exist; this criterion is untagged. This repair is limited to native prerequisite quality; the full
    criterion remains OPEN. No scope waiver, proof credit or full-feature acceptance.'
- fnd: FND-007
  outcome: deferred
  reason: 'Future IR-258 implementation: The out-of-i64 endpoint is tested, but genuine public unbounded-parameter
    generation/refusal is absent. This repair is limited to native prerequisite quality; the full criterion remains
    OPEN. No scope waiver, proof credit or full-feature acceptance.'
- fnd: FND-008
  outcome: deferred
  reason: 'Future IR-258 implementation: Foreign selection and unsupported operations are tested, but total multi-item
    disposition and supported-sibling negotiation controls are absent. This repair is limited to native prerequisite
    quality; the full criterion remains OPEN. No scope waiver, proof credit or full-feature acceptance.'
- fnd: FND-009
  outcome: deferred
  reason: 'Future IR-258 implementation: The physical backend decoder, grouped schema validation, selected-application
    binding and public Rational replay are absent; this criterion is untagged. This repair is limited to native
    prerequisite quality; the full criterion remains OPEN. No scope waiver, proof credit or full-feature acceptance.'
- fnd: FND-010
  outcome: deferred
  reason: 'Future IR-258 implementation: No feature runner/report record with actual versions, checks, cover, playback
    and ordered options exists; this criterion is untagged. This repair is limited to native prerequisite quality;
    the full criterion remains OPEN. No scope waiver, proof credit or full-feature acceptance.'
- fnd: FND-011
  outcome: deferred
  reason: 'Future IR-258 implementation: Native grouping and canonical pair admission exist, but persisted physical
    proof/playback and complete public witness roundtrip are absent. This repair is limited to native prerequisite
    quality; the full criterion remains OPEN. No scope waiver, proof credit or full-feature acceptance.'
```

## Renewed expanded-source review (pass 2)

Reviewed exact published db7806a5f0bab394b993ae47334620c6df1f8149 against actual main 7ea6a9988a4e6ecb5369c77a1941350bde52c4b2. Original findings, prior dispositions and review/custody history above remain unchanged. No new findings; no finding IDs allocated or renumbered. Existing code findings remain fixed and all eleven gap findings remain deferred to future IR-258 implementation, with their full criteria OPEN. No earlier deferred item is claimed fixed by this slice.

### Current verdict and acceptance boundary

**FAIL** — Full scoped feature assurance remains incomplete. The newly computed matrix is 8 tagged / 3 untagged / 0 ignored-only: FR-046-AC-9 now has meaningful partial native request/replay tests, while AC-5, AC-6 and AC-10 remain untagged. All eleven complete obligations remain OPEN, including physical decoding/grouping and actual proof/playback/run evidence. A relevant new tag is not full acceptance.

### Expanded scope and fresh-base interactions

Read both lead boundary decisions, the actual four-file phase diff (function.rs, obligation.rs, new rational_predicate_replay.rs and main registration), the complete retained native scope, and incoming main changes. History merge c381 preserves custody/native bytes; prior source, original SR prefixes and all protected paths remain unchanged by this phase. The fresh main adds Makefile/MSRV and assurance/spec/review changes without a production-code interaction with this seam. PR head and remote main matched the reviewed values. No spec or plan change occurs in the PR diff, so no spec-review/subanalysis applies; code review includes the Rust lane in this same SR.

### Source and identity authority

The new public factories take an admitted FunctionPredicateItem, not a mutable RationalPredicateOracle or caller domain/binding vector. They first compare checked source-package identity and exact source-lock authority/identity/digest facts, then the selected real function, then re-derive the bounded native Rational closure and its original domains. The private join checks complete logical parameter count, unique names and nodes, exact original name/node correspondence and no missing/foreign entry before identity hashing. ReplayPackage inputs/site are private; adverse private-state mutations test guard branches without changing production behavior. Original real function/declaration occurrence and parameter IDs come from owner call_site. No default occurrence or locally minted checked node is added.

RationalRange is additive and contains four canonical decimal-string inclusive endpoints, one ContractArgument per real logical parameter. It does not pretend two physical components are two parameters or substitute fake I64 bounds. Existing function_contract_identity/content_digest remains the sole canonical preimage/digest implementation; logical identifier ordering remains the established O09 order. Independent per-endpoint mutations hold function/site facts constant while changing each serialized domain member, so source-identity changes alone cannot make that test vacuously pass. External actual-source domain/name changes independently change/refuse identities. Existing ArgumentDomain::of, Bool/I64 variants, contract_arguments, preimage construction and frame identity logic are byte-unchanged.

### Adapter, outcomes and test meaning

Integer decimal and Boolean 0/1 spellings remain exact. Rational alone uses the real owner WitnessValue::to_value_text with named typed ValueTextError propagation. All variants share one private encoded-text envelope helper; the integer render_witness API remains intact for function/frame/state callers. Field delimiter validation still precedes request construction. Parameters are joined by real names and transcript entries sorted by real node ID, preserving supplied-order independence. No backend text, copied Rational encoder or CG normalization is introduced. Unknown argument and unsupported legacy variants keep their typed errors; owner request/reader/replay limits and failures remain owner results. The factory re-renders/re-derives bounded preparation once per request, not once per argument, without recompiling source; owner replay recompilation remains the existing public boundary. Linear/quadratic joins are bounded by admitted package/preparation limits. No new unsafe, narrowing cast, async/lock or panic path was added to production.

External tests admit the authentic original healthy and wrong-addend sources and call real public owner replay. They inspect evaluated Boolean and settlement, exercise negative/fraction/zero/endpoints, real reversed declarations and input permutation, each argument out of its original domain, missing/duplicate/foreign names/nodes, wrong selection/package, small input-byte limit, raw noncanonical Rational spellings, unsupported legacy families and delimiters. Raw 0/2, 2/2, zero/negative denominators reach owner spelling refusal; these controls are distinct from future strict physical-backend component validation. The witness inputs are native-injected, never solver-generated playback. Typed owner serialization-failure propagation is structurally reviewed; no realistic Rational encoder failure injection is claimed. Legacy transcript checks and existing independent canonical-preimage/digest tests cover Integer/Boolean/frame/state compatibility. No internal evaluator or private-owner execution seam is called.

### Evidence and remaining gaps

Read actual adapter-red.log: identity test passed and two compiled Rational adapter tests failed UnsupportedWitnessValue(a). Final logs show 13 Rational/native tests, 7 logical-identity tests, 6 function unit tests and 5 exact legacy controls passing (31 total outer tests), plus completed warnings-denied clippy; fmt/diff receipts report zero. Earlier compiler/type-intake errors and corrected owner-refusal expectations are not product RED. Original exhaustive native/Meter controls remain green.

Independently checked recorded active-child cgroup paths and memory.max=17179869184/memory.swap.max=0 for all nine final scopes; terminal memory.events reports oom_kill=0 for each. These measurements concern those new author runs only and do not rewrite the older inactive-infinity limitation or establish proof fit. Source/diff/receipts/log inspection and computed matrix are reviewer checks; no reviewer Cargo, Kani, CBMC, aggregate, namespace or engine execution occurred. No full invocation-accounting, physical-width/comment/grouped-schema proof decoder, negotiated dispatch, one completion cover, actual solver proof/playback, report/version or final acceptance claim follows. Plan completion: not assessed.

### Renewed machine record

```yaml
dispositions: []
scope:
- id: FR-046-AC-1
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: 'The genuine source function `rational_commutative(a: Rational[-9,9;1,9], b: Rational[-9,9;1,9]): Boolean`
    with body `a + b = b + a` selects its whole declaration and complete body through the public QSL admission,
    strict IR reader/lowering and CG route. Its artifact proves that predicate, not only validity or parity of either
    addition. Replacing the body by `a + b = a + a` selects the same authored mutation with unchanged parameter
    domains.'
- id: FR-046-AC-2
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Every accepted symbolic argument has canonical constructor/accessor roundtrip equality and lies in its
    original inclusive component domains. Its two physical I64 bindings retain numerator/denominator roles, original
    parameter identity and bounds in the FR-025 order. Every semantic value in those domains has exactly one accepted
    representation. Zero is `0/1`; negative, fractional and boundary values remain reachable. For the original domain
    there are 111 canonical values per argument and 12,321 argument tuples; admitting all 171 raw component pairs,
    restricting denominators to one, or clipping an endpoint fails correspondence validation under QSpec FR-196-AC-5.
- id: FR-046-AC-3
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Generated evaluation preserves the authored full tree and declaration identity, exact owner arithmetic,
    intermediate values outside the input domains, arbitrary-precision continuation, full typed outcomes and Meter
    charge/stop order. Native comparison against an independent owner-operation reference checks those observations,
    including each denied charge prefix and exact/short budgets; it does not calculate expectations by running the
    generated oracle twice.
- id: FR-046-AC-4
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Generation introduces no assumption on completion, truth or an absence of undefined/refused/incomplete
    results. Controls producing each reachable noncompletion retain that typed outcome and cannot become a successful
    predicate proof; an outcome-based assume mutation is rejected. A completion assertion failure without a replayed
    false value is not labelled a confirmed semantic false counterexample.
- id: FR-046-AC-5
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: The healthy original function receives `Verified` only from an actual successful proof with all required
    checks and its completion cover satisfied. The wrong-addend mutation produces a substantive failed predicate
    assertion with actual solver playback of canonical in-domain arguments, and replay of that same authored function
    completes with Boolean false. A hypothetical witness or cover witness is not substituted for that playback.
- id: FR-046-AC-6
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Each emitted predicate harness has exactly one final completion cover after all predicate assertions,
    including harnesses with representation constraints. Unsatisfied or unreported completion cover, failed unwind,
    timeout, memory exhaustion or missing verdict cannot yield `Verified`; classifications retain FR-017 and FR-029
    distinctions.
- id: FR-046-AC-7
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: A missing or unbounded Rational parameter domain yields source-associated `requires-bound` naming the
    selected function, argument and missing domain, with no harness. A finite component endpoint outside this encoding's
    signed-i64 capacity yields named `unsupported`, retaining the original endpoint and no harness; generation never
    clips, casts away precision or substitutes a smaller domain.
- id: FR-046-AC-8
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: An unavailable owner-lowered whole-function representation, absent one-to-one correspondence or unsupported
    body operation yields named `unsupported` with its source-associated cause and no harness. Every requested item
    has exactly one disposition under FR-039; a refused item does not remove or alter a supported sibling. Malformed
    source or owner-refused packages remain upstream refusals, not fabricated CG claims.
- id: FR-046-AC-9
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Counterexample decoding validates the two physical I64 entries per original parameter, exact width/comment/order,
    schema roles, required physical-entry and byte limits, canonical components, original domains and selected declaration/application
    binding before public owner replay. Grouping yields one typed owner Rational value per parameter through the
    existing canonical serialization seam; missing, duplicate, foreign, role-swapped, domain-conflicting and malformed
    component schemas or entries retain typed refusals with no replayed failure. Out-of-domain, duplicate-zero,
    noncanonical, wrong-declaration and stale-identity witnesses are rejected by the responsible boundary; CG neither
    mints keys nor substitutes scalar application validity for the authored Boolean result. An unavailable replay
    capability is reported without claiming replay acceptance.
- id: FR-046-AC-10
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Run evidence records the selected function and source association, inclusive component domains and correspondence,
    ordered physical bindings and their logical parameter grouping, actual subject and full ordered backend arguments,
    every unwind/loop bound and option, Kani/Rust/CBMC/solver versions, target, actual dependency cohort and execution
    limits, raw checks/cover/playback, exit and typed terminal outcome. Replay uses the retained generated artifact
    without hand editing. No-run, compile refusal, cover failure, resource exhaustion and semantic falsification
    remain distinguishable; preparation alone earns no proof result.
- id: FR-025-AC-10
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: PLANNED (IR-258). Every supported FR-046 Rational argument has exactly two ordered I64 physical bindings
    grouped to its one original parameter, complete domain and canonical value. Actual playback decodes losslessly
    with existing I64 width/comment checks and physical byte/value-count caps, then reconstructs and submits exactly
    one canonical owner Rational per parameter. Missing, duplicate, foreign, role-swapped or bound-conflicting schema,
    malformed entries, noncanonical pairs and over-limit witnesses refuse at their responsible typed boundary; no
    alternate normalized witness is replayed. Existing Boolean/integer controls remain unchanged.
- id: src/kani/generate/mod.rs
  path: src/kani/generate/mod.rs
  role: examined
  excerpt: '//! The one Kani generator: the request and result vocabulary, the negotiation entry and the'
- id: src/kani/generate/outcome.rs
  path: src/kani/generate/outcome.rs
  role: examined
  excerpt: '//! The request and result vocabulary of Kani obligation negotiation (FR-015): the items a'
- id: src/kani/generate/rational_predicate.rs
  path: src/kani/generate/rational_predicate.rs
  role: examined
  excerpt: //! Physical I64 positions for one prepared Rational predicate's logical arguments.
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: //! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.
- id: src/oracle/mod.rs
  path: src/oracle/mod.rs
  role: examined
  excerpt: '//! The oracle subsystem: generation of the Rust oracles (AD-004, FR-014, FR-018, FR-021).'
- id: src/oracle/rational_predicate.rs
  path: src/oracle/rational_predicate.rs
  role: examined
  excerpt: //! Whole-function Rational predicate native source preparation (FR-046).
- id: tests/it/main.rs
  path: tests/it/main.rs
  role: examined
  excerpt: '//! IR-237: the integration test crate. Cargo builds a `tests/<name>/main.rs` directory as one'
- id: tests/it/rational_predicate_agreement.rs
  path: tests/it/rational_predicate_agreement.rs
  role: examined
  excerpt: //! Native body agreement, distinct from public replay and generated-proof acceptance.
- id: tests/it/rational_predicate_generation.rs
  path: tests/it/rational_predicate_generation.rs
  role: examined
  excerpt: //! Leaf native-source preparation only; negotiated generation/replay are not yet integrated.
- id: tests/it/rational_predicate_support/fixtures/commutative.native
  path: tests/it/rational_predicate_support/fixtures/commutative.native
  role: examined
  excerpt: language "ix:native" edition "1-draft";
- id: tests/it/rational_predicate_support/fixtures/large_intermediate.native
  path: tests/it/rational_predicate_support/fixtures/large_intermediate.native
  role: examined
  excerpt: language "ix:native" edition "1-draft";
- id: tests/it/rational_predicate_support/fixtures/wrong_addend.native
  path: tests/it/rational_predicate_support/fixtures/wrong_addend.native
  role: examined
  excerpt: language "ix:native" edition "1-draft";
- id: tests/it/rational_predicate_support/mod.rs
  path: tests/it/rational_predicate_support/mod.rs
  role: examined
  excerpt: //! Authentic source admission helpers; no prebaked graph or minted node identities.
- id: spec/kani/matrix/TC-060-rational-function-predicate-harnesses.md
  path: spec/kani/matrix/TC-060-rational-function-predicate-harnesses.md
  role: context_only
  excerpt: '# TC-060: Verify finite rational whole-function predicates'
- id: spec/kani/matrix/TC-036-generated-subject-abi.md
  path: spec/kani/matrix/TC-036-generated-subject-abi.md
  role: context_only
  excerpt: '# TC-036: Verify the generated harness subject ABI'
- id: spec/decisions/ADR-004-generated-subject-abi-open-decisions.md
  path: spec/decisions/ADR-004-generated-subject-abi-open-decisions.md
  role: context_only
  excerpt: '# ADR-004: Decisions of the generated harness subject ABI'
- id: src/replay/function.rs
  path: src/replay/function.rs
  role: examined
  excerpt: '//! The replay adapter: a decoded Kani falsification goes to QSL''s layer-6 `replay` facade, and'
- id: src/replay/obligation.rs
  path: src/replay/obligation.rs
  role: examined
  excerpt: //! The obligation identity of the function path and of the frame path (ADR-013 O-09,
- id: tests/it/rational_predicate_replay.rs
  path: tests/it/rational_predicate_replay.rs
  role: examined
  excerpt: //! Native-injected witnesses exercise the public adapter; none is solver playback evidence.
- id: src/replay/function.rs::rational_predicate_obligation_identity
  path: src/replay/function.rs
  role: examined
  excerpt: pub fn rational_predicate_obligation_identity(
- id: src/replay/function.rs::rational_predicate_request
  path: src/replay/function.rs
  role: examined
  excerpt: pub fn rational_predicate_request(
- id: src/replay/function.rs::rational_request_checks_source_and_complete_logical_bindings_before_hashing
  path: src/replay/function.rs
  role: examined
  excerpt: fn rational_request_checks_source_and_complete_logical_bindings_before_hashing() {
- id: src/replay/function.rs::rational_identity_has_one_real_parameter_and_all_four_exact_endpoints
  path: src/replay/function.rs
  role: examined
  excerpt: fn rational_identity_has_one_real_parameter_and_all_four_exact_endpoints() {
- id: src/replay/function.rs::legacy_integer_boolean_frame_and_state_transcript_bytes_are_unchanged
  path: src/replay/function.rs
  role: examined
  excerpt: fn legacy_integer_boolean_frame_and_state_transcript_bytes_are_unchanged() {
- id: src/replay/obligation.rs::rational_contract_arguments
  path: src/replay/obligation.rs
  role: examined
  excerpt: pub(crate) fn rational_contract_arguments(
- id: tests/it/rational_predicate_replay.rs::native_injected_rational_witnesses_reach_actual_whole_predicate_replay
  path: tests/it/rational_predicate_replay.rs
  role: examined
  excerpt: fn native_injected_rational_witnesses_reach_actual_whole_predicate_replay() {
- id: tests/it/rational_predicate_replay.rs::logical_domains_and_real_selected_site_bind_the_identity
  path: tests/it/rational_predicate_replay.rs
  role: examined
  excerpt: fn logical_domains_and_real_selected_site_bind_the_identity() {
- id: tests/it/rational_predicate_replay.rs::genuine_reversed_declaration_keeps_name_bound_values
  path: tests/it/rational_predicate_replay.rs
  role: examined
  excerpt: fn genuine_reversed_declaration_keeps_name_bound_values() {
- id: tests/it/rational_predicate_replay.rs::native_rational_adverse_inputs_retain_responsible_owner_refusals
  path: tests/it/rational_predicate_replay.rs
  role: examined
  excerpt: fn native_rational_adverse_inputs_retain_responsible_owner_refusals() {
- id: tests/it/rational_predicate_replay.rs::genuine_wrong_selection_stale_package_and_limits_are_not_repaired
  path: tests/it/rational_predicate_replay.rs
  role: examined
  excerpt: fn genuine_wrong_selection_stale_package_and_limits_are_not_repaired() {
- id: tests/it/rational_predicate_replay.rs::supplied_order_does_not_change_the_canonical_request_or_value_text
  path: tests/it/rational_predicate_replay.rs
  role: examined
  excerpt: fn supplied_order_does_not_change_the_canonical_request_or_value_text() {
new_findings: []
bindings:
- test_id: tests::rational_requests::rational_identity_has_one_real_parameter_and_all_four_exact_endpoints
  ac_id: FR-025-AC-10
  trace: correct
- test_id: authentic_complete_predicate_prepares_native_source_and_grouped_arguments
  ac_id: FR-025-AC-10
  trace: correct
- test_id: physical_component_admission_is_bijective_and_rejects_normalization_duplicates
  ac_id: FR-025-AC-10
  trace: correct
- test_id: logical_domains_and_real_selected_site_bind_the_identity
  ac_id: FR-025-AC-10
  trace: correct
- test_id: authentic_complete_predicate_prepares_native_source_and_grouped_arguments
  ac_id: FR-046-AC-1
  trace: correct
- test_id: a_foreign_real_function_selection_cannot_replace_the_admitted_closure
  ac_id: FR-046-AC-1
  trace: correct
- test_id: generated_native_bodies_preserve_exact_owner_results_and_all_charge_prefixes
  ac_id: FR-046-AC-2
  trace: correct
- test_id: authentic_complete_predicate_prepares_native_source_and_grouped_arguments
  ac_id: FR-046-AC-2
  trace: correct
- test_id: physical_component_admission_is_bijective_and_rejects_normalization_duplicates
  ac_id: FR-046-AC-2
  trace: correct
- test_id: generated_native_bodies_preserve_exact_owner_results_and_all_charge_prefixes
  ac_id: FR-046-AC-3
  trace: correct
- test_id: admitted_nested_addition_preserves_arbitrary_precision_and_stops
  ac_id: FR-046-AC-3
  trace: correct
- test_id: generated_native_input_refusals_preserve_the_entire_meter
  ac_id: FR-046-AC-3
  trace: correct
- test_id: generated_native_bodies_preserve_exact_owner_results_and_all_charge_prefixes
  ac_id: FR-046-AC-4
  trace: correct
- test_id: admitted_nested_addition_preserves_arbitrary_precision_and_stops
  ac_id: FR-046-AC-4
  trace: correct
- test_id: original_capacity_and_operation_refusals_preserve_the_owner_input
  ac_id: FR-046-AC-7
  trace: correct
- test_id: original_capacity_and_operation_refusals_preserve_the_owner_input
  ac_id: FR-046-AC-8
  trace: correct
- test_id: a_foreign_real_function_selection_cannot_replace_the_admitted_closure
  ac_id: FR-046-AC-8
  trace: correct
- test_id: tests::rational_requests::rational_request_checks_source_and_complete_logical_bindings_before_hashing
  ac_id: FR-046-AC-9
  trace: correct
- test_id: tests::rational_requests::rational_identity_has_one_real_parameter_and_all_four_exact_endpoints
  ac_id: FR-046-AC-9
  trace: correct
- test_id: native_injected_rational_witnesses_reach_actual_whole_predicate_replay
  ac_id: FR-046-AC-9
  trace: correct
- test_id: logical_domains_and_real_selected_site_bind_the_identity
  ac_id: FR-046-AC-9
  trace: correct
- test_id: genuine_reversed_declaration_keeps_name_bound_values
  ac_id: FR-046-AC-9
  trace: correct
- test_id: native_rational_adverse_inputs_retain_responsible_owner_refusals
  ac_id: FR-046-AC-9
  trace: correct
- test_id: genuine_wrong_selection_stale_package_and_limits_are_not_repaired
  ac_id: FR-046-AC-9
  trace: correct
- test_id: supplied_order_does_not_change_the_canonical_request_or_value_text
  ac_id: FR-046-AC-9
  trace: correct
reviewed_head: db7806a5f0bab394b993ae47334620c6df1f8149
reviewed_base: 7ea6a9988a4e6ecb5369c77a1941350bde52c4b2
review_verdict: FAIL
prior_code_findings:
  FND-001: fixed
  FND-002: fixed
full_feature_obligations: all eleven OPEN
scoped_matrix:
  tagged: 8
  untagged: 3
  tagged_by_ignored_test: 0
untagged_criteria:
- FR-046-AC-5
- FR-046-AC-6
- FR-046-AC-10
gap_findings:
  FND-001: 'deferred: future IR-258 implementation; full criterion OPEN'
  FND-002: 'deferred: future IR-258 implementation; full criterion OPEN'
  FND-003: 'deferred: future IR-258 implementation; full criterion OPEN'
  FND-004: 'deferred: future IR-258 implementation; full criterion OPEN'
  FND-005: 'deferred: future IR-258 implementation; full criterion OPEN'
  FND-006: 'deferred: future IR-258 implementation; full criterion OPEN'
  FND-007: 'deferred: future IR-258 implementation; full criterion OPEN'
  FND-008: 'deferred: future IR-258 implementation; full criterion OPEN'
  FND-009: 'deferred: future IR-258 implementation; full criterion OPEN'
  FND-010: 'deferred: future IR-258 implementation; full criterion OPEN'
  FND-011: 'deferred: future IR-258 implementation; full criterion OPEN'
live_caps_recorded_for_new_scopes_only: '9/9 child-active snapshots: memory.max=17179869184, memory.swap.max=0,
  oom_kill=0; prior inactive-scope limitation unchanged'
independence: source/diff/receipt/recorded-log and computed-matrix review; no reviewer Cargo/engine/aggregate/namespace
  execution
```

## Renewed sealed-native codec review (pass 3)

Exact frozen head 9873267ac1338419cf8184bf17a9c7ac5dd34e46, source phase from a37de1f2 on retained base 7ea6a9988a4e6ecb5369c77a1941350bde52c4b2. Prior review text/findings/dispositions remain byte-for-byte above. No new findings or finding IDs; original native findings remain fixed. No previously deferred full feature obligation is claimed discharged.

### Current verdict and boundary

**FAIL** — All eleven full delivery obligations remain OPEN/deferred. Matrix remains 8 tagged / 3 untagged (FR-046 AC-5, AC-6, AC-10). AC-9/FR-025-AC-10 now gain substantive native physical decoding/grouping/schema controls; missing actual harness/artifact binding, negotiated dispatch and proof/playback/run qualification still prevent complete discharge.

### Complete-body clarification and base limit

Read merged CG541/11fa61e5 FR-046-AC-3 and TC-060 step 3 as current requirement context. Complete BODY evaluation starts over already-admitted arguments with matching entry Meter state/limits. The single pre-body FunctionCall, conversion and supplied admission state are outside this subject. Historical mentions above (including gap FND-003) of invocation accounting/facade as a body blocker are superseded by this clarification; the remaining FND-003 integration deferral concerns actual selected-body generation/qualification, not an entry charge. No manual charge copy, subtraction/reset trick, internal QSL application call or Runtime bridge is required or added. QSL H/A supplied budgets remain distinct. All eleven full obligations remain OPEN; no full-call criterion is invented. The source branch still has base7ea and no immediate alignment/rerun was authorized: this review is not fresh-base acceptance on current main11fa.

### Scope and implementation

Read all seven phase paths and the unchanged primitive decoder contract, prior native/request/replay code, actual two-commit source delta and all public exports. Only rational_predicate generation metadata, lib exports, factored function context, replay module/new codec, test module registration and new external tests changed. Protected parser/obligation digest authority/harness identity/runner/negotiate/lock/manifest/oracle/old SR bytes are unchanged. No spec/plan artifact is changed by the PR, so spec review/subanalyses remain N/A. Rust and language-independent code review stay in this same SR.

ValidatedRationalPredicateContext has private immutable oracle/source/declaration/obligation fields and no public writable constructor. Existing Rational request and identity methods delegate to its one validation path; the codec factory uses exactly that path. It retains actual admitted package/source lock, owner-selected function/declaration, complete real parameters/domains and existing O09 identity. A caller-modified public Oracle or loaded schema is never source authority. No second parser, unchecked IR constructor, fake logical I64 domain, hash/preimage implementation or owner API was added.

Expected schema is privately derived from the validated context and exposed only by shared reference; supplied loaded schema is compared in full before decoding. New transport structs deny unknown fields and retain typed source metadata, real parameter/occurrence, exact four endpoint strings, unique physical identifiers/positions, roles, I64 types and explicit oracle positions. O09 logical NAME order is intentionally distinct from the unchanged body-oracle NODE order. Physical pairs are sorted by logical name and numerator/denominator; their existing draw IDs retain their original identities while positions are rewritten. Both output views are reconstructed from the same canonical pair. The internal BTreeMap indexing in schema derivation is backed by immutable parameters produced by that same oracle; it is not an untrusted lookup. No unchecked arithmetic/narrowing or async/unsafe/lock path was introduced.

Transcript bytes are checked before parsing; physical entry capacity is checked before allocating primitive metadata, so four positions cannot hide as two grouped values. The unchanged decode_playback enforces block/check/harness selection, caps, arity, exact I64 width/little-endian bytes and decoded comments. Temporary all-Argument I64 bindings are only a reader view and carry no fabricated IR bound/dependency authority. Selected check_text and borrowed raw check_clause are forwarded without reconstruction. A caller harness/module string selects parser input and supplies no executed-artifact authentication.

Owner Rational::new plus exact accessor equality enforces canonicality before domain admission: invalid/nonpositive/normalizing pairs are NonCanonical; canonical values outside an original component domain retain parameter/role OutsideDomain. i64 components widen exactly to owner Integer, including MIN/MAX. No normalization substitutes a valid witness. Valid swapped raw components can denote a different valid input and remain accepted; role-swapped schema is independently refused. Private schema guarantees every original oracle slot is filled once; output remains ordinary decoded native data, with no replay callback or proof classification. Default Debug reports counts rather than raw values/clause; explicit typed fields preserve exact data.

### Tests and evidence

Eight new external tests cover actual source/context acquisition, four-to-two grouping, strict serialized schema roundtrip/unknown fields, role/order/position/name/node/domain/mapping/missing/duplicate/foreign metadata, real asymmetric and reversed declarations, nonidentity name/node mapping, byte/value/arity/width/comment/cover/check/harness refusal, selected raw clause pointer/contents, both parameter domains, canonical zero/negative/fraction/endpoints, required noncanonical pairs, valid alternate raw values, and i64 extrema. Reference parameter order is obtained independently from owner call_site, and the asymmetric test asserts an actual nonidentity mapping. It does not calculate expected oracle order from the generated schema alone. No public test bypass or fake replay/proof callback was added.

Read codec-red.log: two compiled tests fail at missing native decoding/schema refusal, not upstream compile inability. Read mutation-oracle-order.log and patch: replacing true oracle_position mapping yields an actual asymmetric argument-value mismatch. Read mutation-normalization.log and patch: removing accessor equality yields an actual noncanonical acceptance assertion failure. Current frozen source restores both guards. These are native preparation/mutation controls, not generated proof or negotiated-selection RED.

Read final raw logs: 21 Rational/native, 6 function units, 7 logical identities, 15 primitive decoder and 5 exact legacy tests pass, total54 outer tests; clippy completes and fmt/diff receipts report zero. Independently checked all ten recorded active-child cgroup paths, memory.max17179869184, swap.max0 and terminal oom_kill0 snapshots for these new runs. These support only those recorded runs, not capacity fit or older inactive receipts. Reviewer checks were independent source/diff/reader/matrix/log/receipt inspection and diff-check; no reviewer Cargo, Kani, CBMC, aggregate, namespace or engine rerun. Plan completion: not assessed.

### Codec review machine record

```yaml
dispositions: []
scope:
- id: FR-046-AC-1
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: 'The genuine source function `rational_commutative(a: Rational[-9,9;1,9], b: Rational[-9,9;1,9]): Boolean`
    with body `a + b = b + a` selects its whole declaration and complete body through the public QSL admission,
    strict IR reader/lowering and CG route. Its artifact proves that predicate, not only validity or parity of either
    addition. Replacing the body by `a + b = a + a` selects the same authored mutation with unchanged parameter
    domains.'
- id: FR-046-AC-2
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Every accepted symbolic argument has canonical constructor/accessor roundtrip equality and lies in its
    original inclusive component domains. Its two physical I64 bindings retain numerator/denominator roles, original
    parameter identity and bounds in the FR-025 order. Every semantic value in those domains has exactly one accepted
    representation. Zero is `0/1`; negative, fractional and boundary values remain reachable. For the original domain
    there are 111 canonical values per argument and 12,321 argument tuples; admitting all 171 raw component pairs,
    restricting denominators to one, or clipping an endpoint fails correspondence validation under QSpec FR-196-AC-5.
- id: FR-046-AC-3
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Generated complete-body evaluation over already-admitted arguments preserves the authored full tree and
    declaration identity, exact owner arithmetic, intermediate values outside the input domains, arbitrary-precision
    continuation, full typed outcomes, laziness and the body's Meter charge/stop order from entry onward. Native
    comparison against an independent reference using public owner body operations starts from matching entry Meter
    state and limits and checks those observations, including each denied body-charge prefix and zero/exact/short
    budgets. The application's single pre-body FunctionCall charge is excluded at entry, not subtracted after a
    run or manually copied; comparison neither resets mutated Meter state nor assumes it matches. The reference
    does not run the generated oracle twice, call internal QSL application APIs or bridge through Runtime. Full-call
    equivalence remains planned under IR-595 / IR-583 and is not this body comparison.
  source_ref: 11fa61e59650c3cf916b1b4b5f58be9ed3a14844 (context; not merged into source branch)
- id: FR-046-AC-4
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Generation introduces no assumption on completion, truth or an absence of undefined/refused/incomplete
    results. Controls producing each reachable noncompletion retain that typed outcome and cannot become a successful
    predicate proof; an outcome-based assume mutation is rejected. A completion assertion failure without a replayed
    false value is not labelled a confirmed semantic false counterexample.
- id: FR-046-AC-5
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: The healthy original function receives `Verified` only from an actual successful proof with all required
    checks and its completion cover satisfied. The wrong-addend mutation produces a substantive failed predicate
    assertion with actual solver playback of canonical in-domain arguments, and replay of that same authored function
    completes with Boolean false. A hypothetical witness or cover witness is not substituted for that playback.
- id: FR-046-AC-6
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Each emitted predicate harness has exactly one final completion cover after all predicate assertions,
    including harnesses with representation constraints. Unsatisfied or unreported completion cover, failed unwind,
    timeout, memory exhaustion or missing verdict cannot yield `Verified`; classifications retain FR-017 and FR-029
    distinctions.
- id: FR-046-AC-7
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: A missing or unbounded Rational parameter domain yields source-associated `requires-bound` naming the
    selected function, argument and missing domain, with no harness. A finite component endpoint outside this encoding's
    signed-i64 capacity yields named `unsupported`, retaining the original endpoint and no harness; generation never
    clips, casts away precision or substitutes a smaller domain.
- id: FR-046-AC-8
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: An unavailable owner-lowered whole-function representation, absent one-to-one correspondence or unsupported
    body operation yields named `unsupported` with its source-associated cause and no harness. Every requested item
    has exactly one disposition under FR-039; a refused item does not remove or alter a supported sibling. Malformed
    source or owner-refused packages remain upstream refusals, not fabricated CG claims.
- id: FR-046-AC-9
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Counterexample decoding validates the two physical I64 entries per original parameter, exact width/comment/order,
    schema roles, required physical-entry and byte limits, canonical components, original domains and selected declaration/application
    binding before public owner replay. Grouping yields one typed owner Rational value per parameter through the
    existing canonical serialization seam; missing, duplicate, foreign, role-swapped, domain-conflicting and malformed
    component schemas or entries retain typed refusals with no replayed failure. Out-of-domain, duplicate-zero,
    noncanonical, wrong-declaration and stale-identity witnesses are rejected by the responsible boundary; CG neither
    mints keys nor substitutes scalar application validity for the authored Boolean result. An unavailable replay
    capability is reported without claiming replay acceptance.
- id: FR-046-AC-10
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Run evidence records the selected function and source association, inclusive component domains and correspondence,
    ordered physical bindings and their logical parameter grouping, actual subject and full ordered backend arguments,
    every unwind/loop bound and option, Kani/Rust/CBMC/solver versions, target, actual dependency cohort and execution
    limits, raw checks/cover/playback, exit and typed terminal outcome. Replay uses the retained generated artifact
    without hand editing. No-run, compile refusal, cover failure, resource exhaustion and semantic falsification
    remain distinguishable; preparation alone earns no proof result.
- id: FR-025-AC-10
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: PLANNED (IR-258). Every supported FR-046 Rational argument has exactly two ordered I64 physical bindings
    grouped to its one original parameter, complete domain and canonical value. Actual playback decodes losslessly
    with existing I64 width/comment checks and physical byte/value-count caps, then reconstructs and submits exactly
    one canonical owner Rational per parameter. Missing, duplicate, foreign, role-swapped or bound-conflicting schema,
    malformed entries, noncanonical pairs and over-limit witnesses refuse at their responsible typed boundary; no
    alternate normalized witness is replayed. Existing Boolean/integer controls remain unchanged.
- id: src/kani/generate/mod.rs
  path: src/kani/generate/mod.rs
  role: examined
  excerpt: '//! The one Kani generator: the request and result vocabulary, the negotiation entry and the'
- id: src/kani/generate/outcome.rs
  path: src/kani/generate/outcome.rs
  role: examined
  excerpt: '//! The request and result vocabulary of Kani obligation negotiation (FR-015): the items a'
- id: src/kani/generate/rational_predicate.rs
  path: src/kani/generate/rational_predicate.rs
  role: examined
  excerpt: //! Physical I64 positions for one prepared Rational predicate's logical arguments.
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: //! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.
- id: src/oracle/mod.rs
  path: src/oracle/mod.rs
  role: examined
  excerpt: '//! The oracle subsystem: generation of the Rust oracles (AD-004, FR-014, FR-018, FR-021).'
- id: src/oracle/rational_predicate.rs
  path: src/oracle/rational_predicate.rs
  role: examined
  excerpt: //! Whole-function Rational predicate native source preparation (FR-046).
- id: tests/it/main.rs
  path: tests/it/main.rs
  role: examined
  excerpt: '//! IR-237: the integration test crate. Cargo builds a `tests/<name>/main.rs` directory as one'
- id: tests/it/rational_predicate_agreement.rs
  path: tests/it/rational_predicate_agreement.rs
  role: examined
  excerpt: //! Native body agreement, distinct from public replay and generated-proof acceptance.
- id: tests/it/rational_predicate_generation.rs
  path: tests/it/rational_predicate_generation.rs
  role: examined
  excerpt: //! Leaf native-source preparation only; negotiated generation/replay are not yet integrated.
- id: tests/it/rational_predicate_support/fixtures/commutative.native
  path: tests/it/rational_predicate_support/fixtures/commutative.native
  role: examined
  excerpt: language "ix:native" edition "1-draft";
- id: tests/it/rational_predicate_support/fixtures/large_intermediate.native
  path: tests/it/rational_predicate_support/fixtures/large_intermediate.native
  role: examined
  excerpt: language "ix:native" edition "1-draft";
- id: tests/it/rational_predicate_support/fixtures/wrong_addend.native
  path: tests/it/rational_predicate_support/fixtures/wrong_addend.native
  role: examined
  excerpt: language "ix:native" edition "1-draft";
- id: tests/it/rational_predicate_support/mod.rs
  path: tests/it/rational_predicate_support/mod.rs
  role: examined
  excerpt: //! Authentic source admission helpers; no prebaked graph or minted node identities.
- id: spec/kani/matrix/TC-060-rational-function-predicate-harnesses.md
  path: spec/kani/matrix/TC-060-rational-function-predicate-harnesses.md
  role: context_only
  excerpt: '# TC-060: Verify finite rational whole-function predicates'
- id: spec/kani/matrix/TC-036-generated-subject-abi.md
  path: spec/kani/matrix/TC-036-generated-subject-abi.md
  role: context_only
  excerpt: '# TC-036: Verify the generated harness subject ABI'
- id: spec/decisions/ADR-004-generated-subject-abi-open-decisions.md
  path: spec/decisions/ADR-004-generated-subject-abi-open-decisions.md
  role: context_only
  excerpt: '# ADR-004: Decisions of the generated harness subject ABI'
- id: src/replay/function.rs
  path: src/replay/function.rs
  role: examined
  excerpt: '//! The replay adapter: a decoded Kani falsification goes to QSL''s layer-6 `replay` facade, and'
- id: src/replay/obligation.rs
  path: src/replay/obligation.rs
  role: examined
  excerpt: //! The obligation identity of the function path and of the frame path (ADR-013 O-09,
- id: tests/it/rational_predicate_replay.rs
  path: tests/it/rational_predicate_replay.rs
  role: examined
  excerpt: //! Native-injected witnesses exercise the public adapter; none is solver playback evidence.
- id: src/replay/function.rs::rational_predicate_obligation_identity
  path: src/replay/function.rs
  role: examined
  excerpt: pub fn rational_predicate_obligation_identity(
- id: src/replay/function.rs::rational_predicate_request
  path: src/replay/function.rs
  role: examined
  excerpt: pub fn rational_predicate_request(
- id: src/replay/function.rs::rational_request_checks_source_and_complete_logical_bindings_before_hashing
  path: src/replay/function.rs
  role: examined
  excerpt: fn rational_request_checks_source_and_complete_logical_bindings_before_hashing() {
- id: src/replay/function.rs::rational_identity_has_one_real_parameter_and_all_four_exact_endpoints
  path: src/replay/function.rs
  role: examined
  excerpt: fn rational_identity_has_one_real_parameter_and_all_four_exact_endpoints() {
- id: src/replay/function.rs::legacy_integer_boolean_frame_and_state_transcript_bytes_are_unchanged
  path: src/replay/function.rs
  role: examined
  excerpt: fn legacy_integer_boolean_frame_and_state_transcript_bytes_are_unchanged() {
- id: src/replay/obligation.rs::rational_contract_arguments
  path: src/replay/obligation.rs
  role: examined
  excerpt: pub(crate) fn rational_contract_arguments(
- id: tests/it/rational_predicate_replay.rs::native_injected_rational_witnesses_reach_actual_whole_predicate_replay
  path: tests/it/rational_predicate_replay.rs
  role: examined
  excerpt: fn native_injected_rational_witnesses_reach_actual_whole_predicate_replay() {
- id: tests/it/rational_predicate_replay.rs::logical_domains_and_real_selected_site_bind_the_identity
  path: tests/it/rational_predicate_replay.rs
  role: examined
  excerpt: fn logical_domains_and_real_selected_site_bind_the_identity() {
- id: tests/it/rational_predicate_replay.rs::genuine_reversed_declaration_keeps_name_bound_values
  path: tests/it/rational_predicate_replay.rs
  role: examined
  excerpt: fn genuine_reversed_declaration_keeps_name_bound_values() {
- id: tests/it/rational_predicate_replay.rs::native_rational_adverse_inputs_retain_responsible_owner_refusals
  path: tests/it/rational_predicate_replay.rs
  role: examined
  excerpt: fn native_rational_adverse_inputs_retain_responsible_owner_refusals() {
- id: tests/it/rational_predicate_replay.rs::genuine_wrong_selection_stale_package_and_limits_are_not_repaired
  path: tests/it/rational_predicate_replay.rs
  role: examined
  excerpt: fn genuine_wrong_selection_stale_package_and_limits_are_not_repaired() {
- id: tests/it/rational_predicate_replay.rs::supplied_order_does_not_change_the_canonical_request_or_value_text
  path: tests/it/rational_predicate_replay.rs
  role: examined
  excerpt: fn supplied_order_does_not_change_the_canonical_request_or_value_text() {
- id: src/replay/mod.rs
  path: src/replay/mod.rs
  role: examined
  excerpt: '//! The replay subsystem: decoding a Kani witness and replaying it natively (AD-004, FR-016,'
- id: src/replay/rational_predicate.rs
  path: src/replay/rational_predicate.rs
  role: examined
  excerpt: //! Decode native physical inputs against immutable admitted Rational metadata.
- id: tests/it/rational_predicate_playback.rs
  path: tests/it/rational_predicate_playback.rs
  role: examined
  excerpt: //! Synthetic decoder inputs ONLY. These strings are not retained Kani/solver evidence.
- id: src/replay/rational_predicate.rs::pub struct RationalPredicateCodec
  path: src/replay/rational_predicate.rs
  role: examined
  excerpt: pub struct RationalPredicateCodec {
- id: src/replay/rational_predicate.rs::pub enum RationalPredicateDecodeError
  path: src/replay/rational_predicate.rs
  role: examined
  excerpt: pub enum RationalPredicateDecodeError {
- id: src/replay/rational_predicate.rs::pub struct RationalPredicateDecodedInputs<'a>
  path: src/replay/rational_predicate.rs
  role: examined
  excerpt: pub struct RationalPredicateDecodedInputs<'a> {
- id: src/replay/rational_predicate.rs::pub fn decode_inputs<'a>(
  path: src/replay/rational_predicate.rs
  role: examined
  excerpt: pub fn decode_inputs<'a>(
- id: src/kani/generate/rational_predicate.rs::pub struct RationalComponentBinding
  path: src/kani/generate/rational_predicate.rs
  role: examined
  excerpt: pub struct RationalComponentBinding {
- id: src/kani/generate/rational_predicate.rs::pub struct RationalDeclarationBinding
  path: src/kani/generate/rational_predicate.rs
  role: examined
  excerpt: pub struct RationalDeclarationBinding {
- id: src/kani/generate/rational_predicate.rs::pub struct RationalLogicalBinding
  path: src/kani/generate/rational_predicate.rs
  role: examined
  excerpt: pub struct RationalLogicalBinding {
- id: src/kani/generate/rational_predicate.rs::pub struct RationalPredicateSchema
  path: src/kani/generate/rational_predicate.rs
  role: examined
  excerpt: pub struct RationalPredicateSchema {
- id: src/kani/generate/rational_predicate.rs::pub(crate) fn native_predicate_schema(
  path: src/kani/generate/rational_predicate.rs
  role: examined
  excerpt: pub(crate) fn native_predicate_schema(
- id: tests/it/rational_predicate_playback.rs::fn synthetic_four_physical_values_decode_to_two_real_logical_arguments()
  path: tests/it/rational_predicate_playback.rs
  role: examined
  excerpt: fn synthetic_four_physical_values_decode_to_two_real_logical_arguments() {
- id: tests/it/rational_predicate_playback.rs::fn retained_schema_role_position_domain_and_mapping_mutations_are_not_authority()
  path: tests/it/rational_predicate_playback.rs
  role: examined
  excerpt: fn retained_schema_role_position_domain_and_mapping_mutations_are_not_authority() {
- id: tests/it/rational_predicate_playback.rs::fn real_asymmetric_reversed_declarations_map_name_order_back_to_oracle_node_order()
  path: tests/it/rational_predicate_playback.rs
  role: examined
  excerpt: fn real_asymmetric_reversed_declarations_map_name_order_back_to_oracle_node_order() {
- id: tests/it/rational_predicate_playback.rs::fn physical_byte_value_arity_width_comment_and_check_selection_guards_are_reused()
  path: tests/it/rational_predicate_playback.rs
  role: examined
  excerpt: fn physical_byte_value_arity_width_comment_and_check_selection_guards_are_reused() {
- id: tests/it/rational_predicate_playback.rs::fn owner_canonical_roundtrip_and_original_component_domains_are_not_repaired()
  path: tests/it/rational_predicate_playback.rs
  role: examined
  excerpt: fn owner_canonical_roundtrip_and_original_component_domains_are_not_repaired() {
- id: tests/it/rational_predicate_playback.rs::fn foreign_source_function_and_loaded_schema_never_bypass_native_metadata_authority()
  path: tests/it/rational_predicate_playback.rs
  role: examined
  excerpt: fn foreign_source_function_and_loaded_schema_never_bypass_native_metadata_authority() {
- id: tests/it/rational_predicate_playback.rs::fn canonical_i64_extreme_components_widen_losslessly_in_the_actual_admitted_domain()
  path: tests/it/rational_predicate_playback.rs
  role: examined
  excerpt: fn canonical_i64_extreme_components_widen_losslessly_in_the_actual_admitted_domain() {
- id: tests/it/rational_predicate_playback.rs::fn codec_constructor_uses_the_same_validated_source_function_context_as_requests()
  path: tests/it/rational_predicate_playback.rs
  role: examined
  excerpt: fn codec_constructor_uses_the_same_validated_source_function_context_as_requests() {
new_findings: []
bindings:
- test_id: tests::rational_requests::rational_identity_has_one_real_parameter_and_all_four_exact_endpoints
  ac_id: FR-025-AC-10
  trace: correct
- test_id: authentic_complete_predicate_prepares_native_source_and_grouped_arguments
  ac_id: FR-025-AC-10
  trace: correct
- test_id: physical_component_admission_is_bijective_and_rejects_normalization_duplicates
  ac_id: FR-025-AC-10
  trace: correct
- test_id: synthetic_four_physical_values_decode_to_two_real_logical_arguments
  ac_id: FR-025-AC-10
  trace: correct
- test_id: retained_schema_role_position_domain_and_mapping_mutations_are_not_authority
  ac_id: FR-025-AC-10
  trace: correct
- test_id: real_asymmetric_reversed_declarations_map_name_order_back_to_oracle_node_order
  ac_id: FR-025-AC-10
  trace: correct
- test_id: physical_byte_value_arity_width_comment_and_check_selection_guards_are_reused
  ac_id: FR-025-AC-10
  trace: correct
- test_id: owner_canonical_roundtrip_and_original_component_domains_are_not_repaired
  ac_id: FR-025-AC-10
  trace: correct
- test_id: canonical_i64_extreme_components_widen_losslessly_in_the_actual_admitted_domain
  ac_id: FR-025-AC-10
  trace: correct
- test_id: logical_domains_and_real_selected_site_bind_the_identity
  ac_id: FR-025-AC-10
  trace: correct
- test_id: authentic_complete_predicate_prepares_native_source_and_grouped_arguments
  ac_id: FR-046-AC-1
  trace: correct
- test_id: a_foreign_real_function_selection_cannot_replace_the_admitted_closure
  ac_id: FR-046-AC-1
  trace: correct
- test_id: generated_native_bodies_preserve_exact_owner_results_and_all_charge_prefixes
  ac_id: FR-046-AC-2
  trace: correct
- test_id: authentic_complete_predicate_prepares_native_source_and_grouped_arguments
  ac_id: FR-046-AC-2
  trace: correct
- test_id: physical_component_admission_is_bijective_and_rejects_normalization_duplicates
  ac_id: FR-046-AC-2
  trace: correct
- test_id: generated_native_bodies_preserve_exact_owner_results_and_all_charge_prefixes
  ac_id: FR-046-AC-3
  trace: correct
- test_id: admitted_nested_addition_preserves_arbitrary_precision_and_stops
  ac_id: FR-046-AC-3
  trace: correct
- test_id: generated_native_input_refusals_preserve_the_entire_meter
  ac_id: FR-046-AC-3
  trace: correct
- test_id: generated_native_bodies_preserve_exact_owner_results_and_all_charge_prefixes
  ac_id: FR-046-AC-4
  trace: correct
- test_id: admitted_nested_addition_preserves_arbitrary_precision_and_stops
  ac_id: FR-046-AC-4
  trace: correct
- test_id: original_capacity_and_operation_refusals_preserve_the_owner_input
  ac_id: FR-046-AC-7
  trace: correct
- test_id: original_capacity_and_operation_refusals_preserve_the_owner_input
  ac_id: FR-046-AC-8
  trace: correct
- test_id: a_foreign_real_function_selection_cannot_replace_the_admitted_closure
  ac_id: FR-046-AC-8
  trace: correct
- test_id: tests::rational_requests::rational_request_checks_source_and_complete_logical_bindings_before_hashing
  ac_id: FR-046-AC-9
  trace: correct
- test_id: tests::rational_requests::rational_identity_has_one_real_parameter_and_all_four_exact_endpoints
  ac_id: FR-046-AC-9
  trace: correct
- test_id: synthetic_four_physical_values_decode_to_two_real_logical_arguments
  ac_id: FR-046-AC-9
  trace: correct
- test_id: retained_schema_role_position_domain_and_mapping_mutations_are_not_authority
  ac_id: FR-046-AC-9
  trace: correct
- test_id: real_asymmetric_reversed_declarations_map_name_order_back_to_oracle_node_order
  ac_id: FR-046-AC-9
  trace: correct
- test_id: physical_byte_value_arity_width_comment_and_check_selection_guards_are_reused
  ac_id: FR-046-AC-9
  trace: correct
- test_id: owner_canonical_roundtrip_and_original_component_domains_are_not_repaired
  ac_id: FR-046-AC-9
  trace: correct
- test_id: foreign_source_function_and_loaded_schema_never_bypass_native_metadata_authority
  ac_id: FR-046-AC-9
  trace: correct
- test_id: canonical_i64_extreme_components_widen_losslessly_in_the_actual_admitted_domain
  ac_id: FR-046-AC-9
  trace: correct
- test_id: codec_constructor_uses_the_same_validated_source_function_context_as_requests
  ac_id: FR-046-AC-9
  trace: correct
- test_id: native_injected_rational_witnesses_reach_actual_whole_predicate_replay
  ac_id: FR-046-AC-9
  trace: correct
- test_id: logical_domains_and_real_selected_site_bind_the_identity
  ac_id: FR-046-AC-9
  trace: correct
- test_id: genuine_reversed_declaration_keeps_name_bound_values
  ac_id: FR-046-AC-9
  trace: correct
- test_id: native_rational_adverse_inputs_retain_responsible_owner_refusals
  ac_id: FR-046-AC-9
  trace: correct
- test_id: genuine_wrong_selection_stale_package_and_limits_are_not_repaired
  ac_id: FR-046-AC-9
  trace: correct
- test_id: supplied_order_does_not_change_the_canonical_request_or_value_text
  ac_id: FR-046-AC-9
  trace: correct
reviewed_head: 9873267ac1338419cf8184bf17a9c7ac5dd34e46
source_phase_parent: a37de1f2616831db67f148c7e88c5250cfb35aa1
source_base: 7ea6a9988a4e6ecb5369c77a1941350bde52c4b2
latest_main_context: 11fa61e59650c3cf916b1b4b5f58be9ed3a14844
fresh_base_acceptance: not claimed; branch was not rebased or merged
review_verdict: FAIL
prior_code_findings:
  FND-001: fixed
  FND-002: fixed
full_feature_obligations: all eleven OPEN
gap_findings:
  FND-001: 'deferred: future IR-258 integration and qualification; full criterion OPEN'
  FND-002: 'deferred: future IR-258 integration and qualification; full criterion OPEN'
  FND-003: 'deferred: future IR-258 integration and qualification; full criterion OPEN'
  FND-004: 'deferred: future IR-258 integration and qualification; full criterion OPEN'
  FND-005: 'deferred: future IR-258 integration and qualification; full criterion OPEN'
  FND-006: 'deferred: future IR-258 integration and qualification; full criterion OPEN'
  FND-007: 'deferred: future IR-258 integration and qualification; full criterion OPEN'
  FND-008: 'deferred: future IR-258 integration and qualification; full criterion OPEN'
  FND-009: 'deferred: future IR-258 integration and qualification; full criterion OPEN'
  FND-010: 'deferred: future IR-258 integration and qualification; full criterion OPEN'
  FND-011: 'deferred: future IR-258 integration and qualification; full criterion OPEN'
accounting_clarification: complete BODY over already-admitted arguments; entry FunctionCall/conversion/supplied
  admission outside; no invocation-facade blocker or copied entry charge
historical_FND003_interpretation: invocation-accounting wording is superseded by CG541; remaining gap is actual
  integrated selected-body generation/qualification, not entry FunctionCall accounting
scoped_matrix:
  tagged: 8
  untagged: 3
  tagged_by_ignored_test: 0
untagged_criteria:
- FR-046-AC-5
- FR-046-AC-6
- FR-046-AC-10
native_codec_scope: decoded synthetic native inputs and check metadata only; no actual artifact/harness or solver-origin
  authentication
recorded_new_live_caps: 10/10 new active-child snapshots 17179869184 bytes memory.max, swap.max=0, oom_kill=0; historical
  inactive limitation unchanged
reviewer_independence: independent source/diff/owner primitive-reader/matrix/recorded-log/receipt inspection; no
  reviewer Cargo/engine/aggregate/namespace execution
```
