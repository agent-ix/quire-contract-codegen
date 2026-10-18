---
id: SR-12960
title: "IR849 code-review independent review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; tests/it/kani_generation.rs, src/kani/generate/scalar.rs; relevant spec and unchanged trace context"
review_set: subset
---

## Summary

Ticket: IR-849; PR446. Frozen source head a811f47511afed8ef2d85e8e7b201a61cd231771; assigned base and origin/main 85a5a2e3467f18aa275e16ad7c57e3a8cab7cb3f. Reviewer-only: no source/spec/test/branch edits, compiler/backend runs, commits, pushes, GitHub comments or delegation. No applicable AssuranceProfile exists under spec/. Scope excerpts below are verbatim prefixes of at most 600 characters; examined means the stated structural or focused semantic check, not acceptance or proof discharge of an entire requirement. Repository identity is agent-ix/quire-contract-codegen, from GitHub and spec/spec.md. The master spec has no id, so no invented master-spec relationship is emitted.

## Verdict

PASS — changed delta has no new defects. The Findings table describes newly introduced defects only. Its schema-required placeholder is not a whole-repository assurance PASS.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Independent code-review including Rust lane; no separate Rust artifact. All nine mappings and the test rename were examined, including relevant clean units. No normative spec/plan changes: base spec-review and its sub-analyses do not apply. React lane does not apply.

Independent before/after execution used a git-archive of the assigned base in external scratch and the frozen candidate worktree; no checkout/branch mutation was needed. Exact raw commands, stdout, stderr and actual exits are in review/before/ and review/after/. Every matrix/coverage/family and inverse trace JSON was independently recomputed and compared equal to the author's corresponding raw JSON. This checks the author report rather than trusting it. All family/inverse queries returned 0; both strict matrix and strict coverage returned 1 before and after. No exits were masked or projected, and no modules narrowed.

Tools: Quire 0.36.2 (engine 0.50.2), Quoin 0.28.4. Explicit module order: spec-artifacts-iso 0.3.0 then spec-artifacts-process 0.2.0; the process module ref for markers is b82e42e893612b4b4ee94127714b4577ee893502. Duplicate-archetype, inline-data-schema and duplicate-inverse-edge diagnostics remain in raw stderr. The module set is exactly the author's set and neither scope reports zero criteria.

| Axis | Fresh base | Candidate |
| --- | ---: | ---: |
| Matrix criteria | 853 | 853 |
| Tagged | 477 | 479 |
| Untagged | 312 | 308 |
| Ignored-only | 16 | 18 |
| Method without symbol | 48 | 48 |
| Coverage reference rows | 904 | 904 |
| Backed reference rows | 541 | 546 |
| Unbacked reference rows | 226 | 226 |
| Status lies | 0 | 0 |
| Undeclared statuses | 13 | 13 |
| True name-only normalized ID/path/symbol pairs | 14 | 13 |

The 853-criterion matrix and 904-reference-row coverage are different axes; backed is not total minus unbacked. True name-only explicitly subtracts every non-name verifies claim for the same normalized ID/path/symbol; it is not self_named_bound. No new or removed unbacked-row keys exist. The sole last-backing loss is the deliberately removed false NFR-001-M-1 metric claim; no TC or FR/NFR acceptance criterion/constraint loses its last backing. TC-003 loses deterministic bundle (13→12); TC-005 gains actual execution (2→3); TC-007 loses deterministic and actual-proof execution (4→2); TC-014 keeps all five wrappers (5→5); TC-026 loses only scalar name credit (47→46); TC-027 stays 85 and FR-017-CON-2 stays 3. FR-005-AC-4 gains 2; FR-015-AC-11 goes 1→2, AC-19 0→1, AC-20 0→2, AC-24 0→1, AC-25 0→2; NFR-001-AC-1 goes 2→3. These symbol counts and per-form changes are independently retained in independent-comparison.json. Structural claims.verifies are not semantic or evidence-store discharge.

## Focused intent and oracle review

| Wrapper | Independent assertion and mapping assessment |
| --- | --- |
| kani_bundle_is_deterministic_schema_valid_and_stable_rust_compiles | first == second is an observable byte/artifact equality check (NFR-001-AC-1); a Passed dependency yields Ready and execution not_run (partial FR-015-AC-25); schemas, exact harness options and warning-clean external compilation fit TC-005. TC-003, TC-007 and metric NFR-001-M-1 were unrelated and are removed. |
| numeric_state_bindings_are_normalized_bounded_and_schema_valid | Integer graph endpoints are exactly 0 and 1000; two inclusive assume strings and post-state result bounds are asserted. FR-015-AC-11 is partial: no local no-unwind assertion, literal-operand or V2 inferred-interval population. TC-014 also covers schema, mixed ABI and actual Kani execution. |
| every_integer_comparison_supports_a_zero_result_bounded_subject | Six comparison operators, one argument, empty results, -> () and unit post-state closure are asserted: TC-014 only. |
| generated_numeric_oracles_execute_the_shared_inside_and_outside_corpus | The Kani source must contain the independently generated executable oracle bytes. Generated oracle functions execute for -1, 0, 1, 999, 1000 and 1001 with a separate comparison_truth expectation for each of six operators. TC-014 and TC-007 fit; FR-005-AC-4 is executable/Kani subset, and FR-015-AC-20 is integer-comparison subset. Outside-domain oracle execution is not a Kani proof input. |
| declaration_and_dependency_order_do_not_change_the_normalized_bundle | Two environments permute declaration order and must generate equal bundles. TC-014 explicitly owns normalized-order permutations; it is not identical-input NFR-001 evidence. Actual dependency permutation is separately asserted in readiness test. |
| kani_proves_identity_and_prints_numeric_counterexamples | Identity state subject succeeds; changed version fails with concrete playback; incompatible signature fails with mismatched-types output; strict value < 1000 fails and playback names value_current and 1000. FR-015-AC-19, AC-24 and TC-014 fit. This ignored lane is executed by the actual author full gate, not default tests. |
| proof_dependency_graph_derives_readiness_and_preserves_source_sites | Missing/Failed become Incomplete; assumed/stubbed become Conditional; reversed dependency order preserves bundle; source sites occur once, assume/stub markers are counted, laundered graph is rejected by schema. TC-005 and partial FR-015-AC-25 fit. No changed-valid-census identity mutation assertion exists. |
| generated_kani_predicates_are_the_exact_executable_oracles_for_the_boolean_corpus | Both generated pre/post oracle source strings are embedded exactly for short-circuit and total And/Or and implication. FR-015-AC-20 covers this selected Boolean population, with not/eq/ne absent; FR-005-AC-4 and TC-007 are source-embedding evidence, with no Boolean execution or proptest/coverage comparison. |
| kani_executes_the_generated_contract_proof | Actual Kani list and graph.options verification succeed for ready and Conditional assumed/stubbed bundles. TC-005 fits; TC-007 is removed. No typed Verified classification is asserted, so no FR-015-AC-56 claim is added. |

These are relevant partial bindings, not complete AC semantic discharge. No new tests or assertions are required by this annotation-only scope. Removed claims do not need replacing with unrelated tags. The scalar test still checks the typed BoundNotI64 refusal and exact lower/upper values; its TC-026 name-form credit contradicted its deliberate-untraced comment, and the rename removes it without a replacement tag. This is requirement-owned scalar lowering (FR-015), not a new unowned production API.

The already-canonical multiline trace attribute on tc_025_real_kani_runs_verify_separate_obligations_and_falsify_a_seeded_defect continues binding FR-017-CON-2 and TC-027; no source or membership change is needed. The optional tc_035_emitted_model_fields_bind_replay_before_playback retains FR-024-AC-31/33 plus name-only TC-035 unchanged. The unrelated bare TC-003 invalid-request wrapper remains outside the nine wrappers; IR-865 owns broader legacy migration.

## Integrity and checks

Normalized complete-file comparison independently removed the changed single-line trace wrappers, old bare TC/metric comment lines and only the specified scalar-name prefix: both source files are equal. The actual diff independently confirms precisely nine wrapper annotations and one test rename, with all bodies, assertions, ignored markers, function imports, dependencies, schemas, normative documents, scripts and CI unchanged. No copied vocabulary/helper, new source API, gate weakening, coverage threshold change, warning suppression, unsafe code, casts, locks, async behavior or resource mechanism is introduced. Existing Rust test seams invoke actual generation and external tools; no mock substitutes the behavior being claimed. AGENTS.md, CLAUDE.md, Cargo.toml, clippy.toml, rustfmt.toml and deny.toml were read.

Author evidence was independently read, not rerun: generation 8 passed/6 ignored; scalar 1 passed; fmt/fmt-check/lint/compile exit 0. Full author actual make kani-gate settings identify this exact clean head, owned worktree target, QSPEC_DIR, systemd MemoryMax=16G/MemorySwapMax=0 and exit 0, finishing 2026-10-18T23:01:53.845880+00:00. The actual log includes every test and reports 35 passed, 0 failed/ignored, test duration 751.69s, gate ran=35 expected=35 elapsed=801s, Kani 0.68.0 and tree=clean. make invokes the distinct worktree target-codex-backends for the canonical lane; both targets remain. No author PASS is inferred from a copied PR body. NFR-006-AC-13/15 applies by both touched paths; historical ticket compile-only prose does not waive it. Root still owes final make ci and full actual Kani after latest review-artifact custody. Heavy work was prohibited during this review because IR-834 owns the exclusive slot; no duplicate compiler/backend gate was needed for the unchanged bodies.

## Examined units

```yaml
scope:
- id: FR-005-AC-4
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: Executable, proptest, Kani, and coverage semantics agree or retain an explicit
    difference.
- id: FR-015-AC-11
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Every symbolic argument carries an inclusive assumption equal to its IR
    `bounded_domain` or a literal operand's own value (FR-015-AC-16); for a V2 scalar
    claim, an operand with the FR-014 inferred application interval uses that interval
    (FR-015-AC-103). A bounded-integer post-state result is required to lie in the
    same domain; no generated source carries a `#[kani::unwind]`.
- id: FR-015-AC-19
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A zero-input transition over one bounded-integer state value verifies for
    an identity subject and is falsified with a concrete counterexample for a subject
    that changes the value.
- id: FR-015-AC-20
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A harness for a Boolean-connective or bounded-integer comparison claim
    (`and`, `or`, `not`, `implies`, `eq`, `ne` and the six integer comparisons) embeds
    its FR-014 oracle byte-identical to the oracle crate's function.
- id: FR-015-AC-24
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A falsifying in-domain assignment of a plain bounded-integer comparison
    claim is reported through Kani's concrete playback.
- id: FR-015-AC-25
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A valid declared census is folded into the harness identity, readiness
    is `ready` only when every dependency passed and `incomplete` while any is missing
    or failed, and proof execution is recorded `not_run`.
- id: NFR-001-AC-1
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  excerpt: Identical inputs and configuration produce byte-identical bundles.
- id: FR-017-CON-2
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The generator SHALL NOT report a generation-time classification as an execution
    outcome.
- id: NFR-006-AC-13
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: For a Kani-touching pull request, the author runs the full gate on its
    clean, stable candidate before opening it, and the merger runs it again on the
    clean, stable current candidate before merge; both actual processes exit successfully
    with all expected tests run. A copied body line alone cannot satisfy either run.
- id: NFR-006-AC-15
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: A Kani-touching pull request whose required lane did not run or failed
    remains unmerged and does not claim real-Kani verification; a written explanation
    does not waive the gate.
- id: kani_bundle_is_deterministic_schema_valid_and_stable_rust_compiles
  path: tests/it/kani_generation.rs
  role: examined
  excerpt: '#[trace("FR-015-AC-25", "NFR-001-AC-1", "TC-005")]

    #[test]

    fn kani_bundle_is_deterministic_schema_valid_and_stable_rust_compiles('
- id: numeric_state_bindings_are_normalized_bounded_and_schema_valid
  path: tests/it/kani_generation.rs
  role: examined
  excerpt: '#[trace("FR-015-AC-11", "TC-014")]

    #[test]

    #[ignore = "kani lane: run serially through `make kani`"]

    fn numeric_state_bindings_are_normalized_bounded_and_schema_valid('
- id: every_integer_comparison_supports_a_zero_result_bounded_subject
  path: tests/it/kani_generation.rs
  role: examined
  excerpt: '#[trace("TC-014")]

    #[test]

    fn every_integer_comparison_supports_a_zero_result_bounded_subject('
- id: generated_numeric_oracles_execute_the_shared_inside_and_outside_corpus
  path: tests/it/kani_generation.rs
  role: examined
  excerpt: '#[trace("FR-005-AC-4", "FR-015-AC-20", "TC-007", "TC-014")]

    #[test]

    fn generated_numeric_oracles_execute_the_shared_inside_and_outside_corpus('
- id: declaration_and_dependency_order_do_not_change_the_normalized_bundle
  path: tests/it/kani_generation.rs
  role: examined
  excerpt: '#[trace("TC-014")]

    #[test]

    fn declaration_and_dependency_order_do_not_change_the_normalized_bundle('
- id: kani_proves_identity_and_prints_numeric_counterexamples
  path: tests/it/kani_generation.rs
  role: examined
  excerpt: '#[trace("FR-015-AC-19", "FR-015-AC-24", "TC-014")]

    #[test]

    #[ignore = "kani lane: run serially through `make kani`"]

    fn kani_proves_identity_and_prints_numeric_counterexamples('
- id: proof_dependency_graph_derives_readiness_and_preserves_source_sites
  path: tests/it/kani_generation.rs
  role: examined
  excerpt: '#[trace("FR-015-AC-25", "TC-005")]

    #[test]

    fn proof_dependency_graph_derives_readiness_and_preserves_source_sites('
- id: generated_kani_predicates_are_the_exact_executable_oracles_for_the_boolean_corpus
  path: tests/it/kani_generation.rs
  role: examined
  excerpt: '#[trace("FR-005-AC-4", "FR-015-AC-20", "TC-007")]

    #[test]

    fn generated_kani_predicates_are_the_exact_executable_oracles_for_the_boolean_corpus('
- id: kani_executes_the_generated_contract_proof
  path: tests/it/kani_generation.rs
  role: examined
  excerpt: '#[trace("TC-005")]

    #[test]

    #[ignore = "kani lane: run serially through `make kani`"]

    fn kani_executes_the_generated_contract_proof('
- id: TC-003
  path: spec/core/matrix/TC-003-unsupported-diagnostics.md
  role: context_only
  excerpt: 'Verify malformed, orphaned, partial, unsupported and undefined-result
    constructs cannot yield

    complete artifacts or lose their refusal locus.'
- id: TC-005
  path: spec/kani/matrix/TC-005-proof-dependencies.md
  role: examined
  excerpt: 'Verify generated Kani source, framing, typed bindings, and dependency
    graphs preserve assumptions and

    block complete-proof claims when required dependencies are not successful. Keep
    model-domain bounds

    distinct from assumed proof edges.'
- id: TC-007
  path: spec/core/matrix/TC-007-cross-backend-parity.md
  role: examined
  excerpt: 'Verify executable, proptest, Kani, and coverage lowerings agree on the
    shared bounded corpus and every

    permitted differential fixture has an attributed disposition.'
- id: TC-014
  path: spec/kani/matrix/TC-014-numeric-state-kani.md
  role: examined
  excerpt: 'Verify the generalized Kani adapter derives exact Boolean and bounded-`i64`
    subject bindings from

    the same checked clauses as executable oracles, constrains symbolic values from
    IR model domains,

    and emits reproducible proof and counterexample-capable harnesses without widening
    unsupported input.'
- id: TC-026
  path: spec/replay/matrix/TC-026-witness-native-replay.md
  role: context_only
  excerpt: 'Verify that retained counterexamples are decoded, domain-checked and replayed

    natively before any failure is reported.'
- id: TC-027
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: context_only
  excerpt: 'Verify that a bounded Kani obligation runs against the installed backend,
    that

    its outcome is read from the backend rather than defaulted, and that the

    returned evidence describes the invocation that actually happened.'
- id: TC-035
  path: spec/replay/matrix/TC-035-counterexample-envelope-intake.md
  role: context_only
  excerpt: 'Verify that every counterexample reaches QSL as QSL''s counterexample
    envelope keyed by QSL''s

    obligation identity, that out-of-domain and incomplete counterexamples are refused
    before any

    replay, that corpus and frame counterexamples take their own arms, that minimization
    keeps only

    failure-preserving envelopes, and that the generator holds no copy of QSL''s replay
    types.'
- id: a_domain_outside_i64_is_a_typed_refusal_not_a_panic
  path: src/kani/generate/scalar.rs
  role: examined
  excerpt: "    // Deliberately untraced: no FR-015 AC states this. AC-3 (\"unbounded\
    \ or non-finite... refused\")\n    // is the closest in subject but does not describe\
    \ this case -- the domain below is bounded\n    // and finite, just outside `i64`\
    \ -- so citing it would misdescribe what this test proves.\n    //\n    // An\
    \ empty `checked_bounds`, and a first checked bound with no `IntegerRange` domain,\
    \ are the\n    // `NoRenderer` refusal (NFR-005-AC-2, the `tc_042_ac2_*` tests\
    \ below).\n    #[test]\n    fn a_domain_outside_i64_is_a_typed_refusal_not_a_panic()\
    \ {\n"
bindings:
- test_id: kani_bundle_is_deterministic_schema_valid_and_stable_rust_compiles
  ac_id: FR-015-AC-25
  trace: correct
- test_id: kani_bundle_is_deterministic_schema_valid_and_stable_rust_compiles
  ac_id: NFR-001-AC-1
  trace: correct
- test_id: kani_bundle_is_deterministic_schema_valid_and_stable_rust_compiles
  ac_id: TC-005
  trace: correct
- test_id: numeric_state_bindings_are_normalized_bounded_and_schema_valid
  ac_id: FR-015-AC-11
  trace: correct
- test_id: numeric_state_bindings_are_normalized_bounded_and_schema_valid
  ac_id: TC-014
  trace: correct
- test_id: every_integer_comparison_supports_a_zero_result_bounded_subject
  ac_id: TC-014
  trace: correct
- test_id: generated_numeric_oracles_execute_the_shared_inside_and_outside_corpus
  ac_id: FR-005-AC-4
  trace: correct
- test_id: generated_numeric_oracles_execute_the_shared_inside_and_outside_corpus
  ac_id: FR-015-AC-20
  trace: correct
- test_id: generated_numeric_oracles_execute_the_shared_inside_and_outside_corpus
  ac_id: TC-007
  trace: correct
- test_id: generated_numeric_oracles_execute_the_shared_inside_and_outside_corpus
  ac_id: TC-014
  trace: correct
- test_id: declaration_and_dependency_order_do_not_change_the_normalized_bundle
  ac_id: TC-014
  trace: correct
- test_id: kani_proves_identity_and_prints_numeric_counterexamples
  ac_id: FR-015-AC-19
  trace: correct
- test_id: kani_proves_identity_and_prints_numeric_counterexamples
  ac_id: FR-015-AC-24
  trace: correct
- test_id: kani_proves_identity_and_prints_numeric_counterexamples
  ac_id: TC-014
  trace: correct
- test_id: proof_dependency_graph_derives_readiness_and_preserves_source_sites
  ac_id: FR-015-AC-25
  trace: correct
- test_id: proof_dependency_graph_derives_readiness_and_preserves_source_sites
  ac_id: TC-005
  trace: correct
- test_id: generated_kani_predicates_are_the_exact_executable_oracles_for_the_boolean_corpus
  ac_id: FR-005-AC-4
  trace: correct
- test_id: generated_kani_predicates_are_the_exact_executable_oracles_for_the_boolean_corpus
  ac_id: FR-015-AC-20
  trace: correct
- test_id: generated_kani_predicates_are_the_exact_executable_oracles_for_the_boolean_corpus
  ac_id: TC-007
  trace: correct
- test_id: kani_executes_the_generated_contract_proof
  ac_id: TC-005
  trace: correct
```
