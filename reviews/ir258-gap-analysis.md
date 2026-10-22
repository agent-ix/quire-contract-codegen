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
