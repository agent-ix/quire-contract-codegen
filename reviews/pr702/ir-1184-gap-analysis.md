---
id: SR-202610310164
title: IR-1184 gap-analysis review
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-contract-codegen; origin/main...f7b11de; src/, tests/, spec/;
  IR-1184
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-032
  type: references
- target: ix://agent-ix/quire-contract-codegen/FR-033
  type: references
---

## Summary

Ticket: IR-1184. PASS for the frozen IR-1184 implementation delta. Immutable generation authority rejects edits to public scalar identities/artifacts and public composite claim maps; exact staged path/bytes, positional draws, original limits, source and package are checked independently. Scalar request/report conversion checks every actual wire field and ReplayLimits before full actual-sent claim/report identity and terminal projection. The repeated x+x control executes actual emitted bytes with independent draws [2,4] and calls the public pinned QSL operator route. Composite controls execute genuine generated Eq and a changed native mutant, preserve actual Outcome<bool>, and reject changed staged bytes; they invent no pair count, shadow evidence or backend proof. Verified facade evidence remains explicitly independent. No new copied encoder, manifest/lock mutation, pin purge or CI weakening occurs. Driver honesty remains explicit; these products are checked assertions, not unforgeable execution receipts. IR-631 and IR-806 production/run-settlement pipelines remain gated and are not discharged by this review.

## Verdict

FAIL for whole-repository inherited coverage debt; PASS for scoped IR-1184 delta. None of the inherited findings blocks this scoped PR.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - | correct-requirement-no-evidence |

## Coverage

Reviewed all 16 changed files at f7b11de8da0cc3c7f5e4b86539b7445104d6a756 against origin/main 1b28e456; read AGENTS.md, CLAUDE.md, IR-1184 and its comments, FR-032 and FR-033 boundaries, rust-style API/implementation/testing/safety/wire rules and installed gap-analysis references. No applicable AssuranceProfile was found. No tests, Cargo, Kani or backend proofs were run by this reviewer, as explicitly directed; pre-review fmt/clippy and focused 1 unit plus 37 IT passes were supplied by the lead. Full local integration target is pending the lead’s final single run. The source inventory is 135 Rust files and the test/support inventory is 129 files. Existing subsystem ownership is declared by core, strategy, oracle, publication, evidence, routed, Kani and replay requirements. No newly ownerless public behavior, concrete source/test stub or coverage inflation was confirmed. Arc ownership retains one admitted package per source within scalar generation and one immutable package/artifact set per composite call; scalar harness Eq excludes unrelated whole-call package siblings. Generated scalar Rust output is preserved; the proving record intentionally gains scalarReplayBody. No line coverage was measured. Broad optional whole-repository semantic analysis was skipped; the requested scoped intent/test/code review was performed.
Matrix: 1023 criteria; 585 tagged, 333 untagged, 12 tagged-by-ignored-test, 93 method-without-symbol. Base: 580/338/12/93. Five criteria gain tags: FR-032-AC-6 and FR-032-AC-15..18. FR-033-AC-18 keeps existing tags and adds actual native-artifact control. All 345 remaining untagged/ignored gaps are inherited; each is listed explicitly as inherited context, outside this delta. Quoin matrix has no run evidence, which supplies no test-failure or proof credit.
Plan completion: not assessed

## Examined scope and bindings

```yaml
scope:
- id: FR-032-AC-15
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. The admitted IR reader supplies the original application
    structural identity and authentic expression occurrence. CG typed generation supplies
    and retains the scalar kind and one argument for each operand position in actual
    draw order, with typed graph-child or inline-literal provenance and the harness
    range. Two draws of one parameter remain two entries. Changing a package, path
    or producer ordinal changes the structural identity; changing an included body
    member while the identity stays equal is caught by complete retained-body comparison
    and refuses. A source-span or n
- id: FR-032-AC-16
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. An independent fixture obtains the same application
    `NodeKey`/`CheckedNodeId` from IR admission and CG retention, checks the exact
    structural identity and complete positional body, and the actual QSL operator
    route accepts the matching claim. No CG-local identity derivation, copied encoder,
    stand-in verifier or synthetic acceptance passes.
- id: FR-032-AC-17
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. With a valid original claim retained, altering only
    the claimed package, path or producer ordinal, or changing one otherwise membership-valid
    retained body member while reusing the old structural identity, produces a typed
    stale-dependency/revision-mismatch refusal before scalar comparison. A body-only
    edit cannot pass because the node identity is equal. No claimed or recomputed
    digest is required.
- id: FR-032-AC-18
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. If the admitted IR read lacks a valid application
    identity or authentic occurrence, or CG typed generation lacks a required retained
    body member, the CG builder returns typed setup refusal without replay or fabricated
    identity. A caller string or usable sentinel cannot replace the absent authority.
- id: FR-033-AC-18
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. CG obtains the original equality/inequality application
    structural identity from IR admission and retains the authentic occurrence, kind
    and one argument per operand position in original order. A self-comparison keeps
    two graph-child entries with each drawn Node-keyed bound; distinct parameters
    and composite literals keep their own graph-child entries, with empty Bounds for
    literals. Population-keyed metadata stays in the full proving record; an admitted
    QSL claim containing it refuses as HarnessUnknownKey before identity/body comparison.
    Bounds key ordering and duplicate
- id: FR-032-AC-1
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    A public consumer passes the routed harness's `ScalarObligationIdentity` directly
    to the scalar builder; its typed call constructs neither `KaniObligationIdentity`
    nor V1 `ReplayPackage`. After the ordered setup checks, an otherwise valid request
    returns unsupported-capability only while a named step-5 capability is unavailable;
    when all are available it returns an admitted plan, and authenticated completion
    enables replay/conversion. Conversion of a capability refusal returns no settlement
    rather than a synthe
- id: FR-032-AC-2
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  excerpt: PLANNED (IR-631). The builder decodes actual routed-harness playback in
    persisted symbolic call order through the Kani output adapter; another harness
    name, missing/extra operand, wrong byte width, playback above the builder-owned
    8 MiB per-harness ceiling or malformed assertion block yields the distinct typed
    refusal under the stated precedence before replay.
- id: FR-032-AC-3
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    Each operand at its own lower or upper bound is admitted and the adjacent out-of-domain
    value is refused before replay. The admitted QSL inputs preserve the catalogued
    operator, operands, operand ranges, result range and harness limits. Another node/operation,
    altered operand schema or replay limit differing from the retained original generation
    context refuses; operand correspondence uses [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
    AC-16.
- id: FR-032-AC-4
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    The driver's actual native observation of the proved generated oracle is required
    and tied to this operand vector, limits and retained staged artifact bytes; checked
    completion requires driver authentication. A bare outcome echo, missing observation,
    another artifact's observation or fresh regeneration that removes the arithmetic
    mutation refuses before QSL evaluation. Playback bytes and Kani falsification
    alone cannot supply a generated outcome.
- id: FR-032-AC-5
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    The public consumer executes the same proved generated artifact and invokes QSL
    between the builder and converter. QSL compares its actual native observation
    with authoritative exact operator evaluation using the four-operator projection
    owned by [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-37,
    without adding refusal-cause or accounting-counter equality; changing the generated
    arithmetic while keeping original node/operator identity changes the measured
    parity result. `Undefined`, `Inco
- id: FR-032-AC-6
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    For every `Diverged`, `Agrees`, `GeneratedFault`, `RefusedInput`, exact `Incomplete`
    and other `Refused` outcome, the converter compares the report's full claim to
    the retained sent claim and actual sent wire against the original proving context
    before conversion; a changed package/function/source/dependency or original limit
    refuses even with an equal claim. At the immutable-report/retained-binding boundary,
    an absent report, another result/run, or changing only the obligation, node, occurrence,
    kind, operator
- id: FR-033-AC-19
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: context_only
  excerpt: PLANNED / CODE-GATED (IR-806). Before falsified parity invocation, CG authenticates
    the retained proved generated artifact and original source/package/context, reconstructed
    operands and original execution limits against the driver's actual native observation;
    it preserves the owning same-valid-claim Disagreed precedence before treating
    a missing or failed observation as a refusal. Without a same-valid-claim Disagreed,
    a different artifact, changed retained bytes or context, altered operands or limits,
    missing observation, or fresh regeneration that removes a mutation refuses with
    a typed CG p
- id: FR-033-AC-20
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: context_only
  excerpt: PLANNED / CODE-GATED for the current QSL FR-358 claim shape; earlier IR-666
    converter and IR-635 verified-constructor controls remain partial evidence only
    for unchanged members. Before projecting any QSL outcome, CG compares a genuine
    `report.claim()` against `CompositeIdentity::new` over exactly the actual sent
    obligation, all remaining `CompositeParityClaim` members (node, occurrence, operator,
    obligation kind, harness bounds and limits) and complete `CompositeEvidence` (falsified
    operands, shadow verdict and pair count, native observation and cause, refinement;
    or verified SUCCESS count an
bindings:
- test_id: tests/it/scalar_playback.rs::routed_draw_order_is_preserved_with_distinct_domains
  ac_id: FR-032-AC-2
  trace: correct
- test_id: tests/it/scalar_playback.rs::negate_and_literal_singleton_keep_every_draw
  ac_id: FR-032-AC-2
  trace: correct
- test_id: tests/it/scalar_playback.rs::subtraction_decodes_signed_extreme_operands_without_evaluating_them
  ac_id: FR-032-AC-2
  trace: correct
- test_id: tests/it/scalar_playback.rs::harness_arity_width_and_comment_failures_are_distinct
  ac_id: FR-032-AC-2
  trace: correct
- test_id: tests/it/scalar_playback.rs::assertion_selection_and_malformed_blocks_use_adapter_refusals
  ac_id: FR-032-AC-2
  trace: correct
- test_id: tests/it/scalar_playback.rs::scalar_decoder_rejects_playback_above_its_limit_after_a_loose_parse
  ac_id: FR-032-AC-2
  trace: correct
- test_id: tests/it/scalar_literal_operands.rs::add_keeps_the_plain_literal_inline_through_scalar_generation
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_literal_operands.rs::negate_keeps_the_plain_literal_inline_through_scalar_generation
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_playback.rs::routed_draw_order_is_preserved_with_distinct_domains
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_playback.rs::negate_and_literal_singleton_keep_every_draw
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_playback.rs::subtraction_decodes_signed_extreme_operands_without_evaluating_them
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_playback.rs::inclusive_endpoints_and_adjacent_values_use_each_operands_range
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_playback.rs::typed_schema_byte_ceiling_decode_and_domain_refusals
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_playback.rs::unsupported_operator_and_invalid_persisted_schema_refuse_typed_playback
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_playback.rs::original_renderer_limits_are_retained_with_genuine_routed_source
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_playback.rs::original_limits_refuse_changed_retained_identity_source_or_record
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_replay_binding.rs::scalar_actual_sent_context_and_every_driver_tie_are_independently_checked
  ac_id: FR-032-AC-6
  trace: correct
- test_id: tests/it/scalar_replay_binding.rs::original_scalar_driver_executes_independent_repeated_draws_and_binds_real_qsl
  ac_id: FR-032-AC-15
  trace: correct
- test_id: tests/it/scalar_replay_binding.rs::original_scalar_driver_executes_independent_repeated_draws_and_binds_real_qsl
  ac_id: FR-032-AC-16
  trace: correct
- test_id: tests/it/scalar_replay_binding.rs::original_scalar_driver_executes_independent_repeated_draws_and_binds_real_qsl
  ac_id: FR-032-AC-17
  trace: correct
- test_id: tests/it/scalar_replay_binding.rs::scalar_actual_sent_context_and_every_driver_tie_are_independently_checked
  ac_id: FR-032-AC-17
  trace: correct
- test_id: src/replay/scalar_authority.rs::tests::absent_or_ambiguous_scalar_authority_never_invents_an_occurrence
  ac_id: FR-032-AC-18
  trace: correct
- test_id: tests/it/scalar_replay_binding.rs::original_scalar_driver_executes_independent_repeated_draws_and_binds_real_qsl
  ac_id: FR-032-AC-18
  trace: correct
- test_id: tests/it/scalar_replay_binding.rs::scalar_actual_sent_context_and_every_driver_tie_are_independently_checked
  ac_id: FR-032-AC-18
  trace: correct
- test_id: tests/it/composite_artifact_binding.rs::original_composite_body_and_actual_emitted_bytes_have_independent_authority
  ac_id: FR-033-AC-18
  trace: correct
- test_id: tests/it/composite_original_eq.rs::original_parameter_operands_build_positional_identity_and_public_verified_report
  ac_id: FR-033-AC-18
  trace: correct
- test_id: tests/it/composite_parity_converter.rs::structural_obligation_does_not_admit_a_changed_operator_body
  ac_id: FR-033-AC-18
  trace: correct
- test_id: tests/it/composite_parity_converter.rs::tc_048_falsified_report_requires_every_sent_identity_member
  ac_id: FR-033-AC-18
  trace: correct
```

## Inherited coverage debt (context outside this delta)

These mechanically inherited gaps are coverage context, not new findings or requested changes in IR-1184.


| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| DEBT-001 | high | Inherited untagged criterion FR-005-AC-3; the same coverage status exists at origin/main. This PR does not discharge it. | spec/core/functional/FR-005-cli-conformance.md | correct-requirement-no-evidence |
| DEBT-002 | high | Inherited untagged criterion FR-005-AC-4; the same coverage status exists at origin/main. This PR does not discharge it. | spec/core/functional/FR-005-cli-conformance.md | correct-requirement-no-evidence |
| DEBT-003 | high | Inherited untagged criterion FR-039-AC-4; the same coverage status exists at origin/main. This PR does not discharge it. | spec/core/functional/FR-039-contract-package-consumption.md | correct-requirement-no-evidence |
| DEBT-004 | high | Inherited untagged criterion NFR-001-M-1; the same coverage status exists at origin/main. This PR does not discharge it. | spec/core/non-functional/NFR-001-reproducible-atomic.md | correct-requirement-no-evidence |
| DEBT-005 | high | Inherited untagged criterion NFR-002-M-1; the same coverage status exists at origin/main. This PR does not discharge it. | spec/core/non-functional/NFR-002-provenance-boundary.md | correct-requirement-no-evidence |
| DEBT-006 | high | Inherited untagged criterion NFR-002-M-3; the same coverage status exists at origin/main. This PR does not discharge it. | spec/core/non-functional/NFR-002-provenance-boundary.md | correct-requirement-no-evidence |
| DEBT-007 | high | Inherited untagged criterion FR-004-AC-6; the same coverage status exists at origin/main. This PR does not discharge it. | spec/evidence/functional/FR-004-vacuity-evidence.md | correct-requirement-no-evidence |
| DEBT-008 | high | Inherited untagged criterion FR-004-AC-8; the same coverage status exists at origin/main. This PR does not discharge it. | spec/evidence/functional/FR-004-vacuity-evidence.md | correct-requirement-no-evidence |
| DEBT-009 | high | Inherited untagged criterion FR-004-AC-10; the same coverage status exists at origin/main. This PR does not discharge it. | spec/evidence/functional/FR-004-vacuity-evidence.md | correct-requirement-no-evidence |
| DEBT-010 | high | Inherited untagged criterion FR-004-AC-11; the same coverage status exists at origin/main. This PR does not discharge it. | spec/evidence/functional/FR-004-vacuity-evidence.md | correct-requirement-no-evidence |
| DEBT-011 | high | Inherited untagged criterion FR-004-AC-12; the same coverage status exists at origin/main. This PR does not discharge it. | spec/evidence/functional/FR-004-vacuity-evidence.md | correct-requirement-no-evidence |
| DEBT-012 | high | Inherited untagged criterion FR-004-AC-13; the same coverage status exists at origin/main. This PR does not discharge it. | spec/evidence/functional/FR-004-vacuity-evidence.md | correct-requirement-no-evidence |
| DEBT-013 | high | Inherited untagged criterion FR-004-AC-14; the same coverage status exists at origin/main. This PR does not discharge it. | spec/evidence/functional/FR-004-vacuity-evidence.md | correct-requirement-no-evidence |
| DEBT-014 | high | Inherited untagged criterion FR-004-AC-15; the same coverage status exists at origin/main. This PR does not discharge it. | spec/evidence/functional/FR-004-vacuity-evidence.md | correct-requirement-no-evidence |
| DEBT-015 | high | Inherited untagged criterion FR-004-AC-25; the same coverage status exists at origin/main. This PR does not discharge it. | spec/evidence/functional/FR-004-vacuity-evidence.md | correct-requirement-no-evidence |
| DEBT-016 | high | Inherited untagged criterion FR-036-AC-1; the same coverage status exists at origin/main. This PR does not discharge it. | spec/evidence/functional/FR-036-source-obligation-census.md | correct-requirement-no-evidence |
| DEBT-017 | high | Inherited untagged criterion FR-036-AC-2; the same coverage status exists at origin/main. This PR does not discharge it. | spec/evidence/functional/FR-036-source-obligation-census.md | correct-requirement-no-evidence |
| DEBT-018 | high | Inherited untagged criterion FR-036-AC-3; the same coverage status exists at origin/main. This PR does not discharge it. | spec/evidence/functional/FR-036-source-obligation-census.md | correct-requirement-no-evidence |
| DEBT-019 | high | Inherited untagged criterion FR-036-AC-4; the same coverage status exists at origin/main. This PR does not discharge it. | spec/evidence/functional/FR-036-source-obligation-census.md | correct-requirement-no-evidence |
| DEBT-020 | high | Inherited untagged criterion FR-036-AC-5; the same coverage status exists at origin/main. This PR does not discharge it. | spec/evidence/functional/FR-036-source-obligation-census.md | correct-requirement-no-evidence |
| DEBT-021 | high | Inherited untagged criterion FR-036-AC-6; the same coverage status exists at origin/main. This PR does not discharge it. | spec/evidence/functional/FR-036-source-obligation-census.md | correct-requirement-no-evidence |
| DEBT-022 | high | Inherited untagged criterion FR-036-AC-7; the same coverage status exists at origin/main. This PR does not discharge it. | spec/evidence/functional/FR-036-source-obligation-census.md | correct-requirement-no-evidence |
| DEBT-023 | high | Inherited untagged criterion FR-015-AC-1; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-024 | high | Inherited untagged criterion FR-015-AC-4; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-025 | high | Inherited untagged criterion FR-015-AC-8; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-026 | high | Inherited untagged criterion FR-015-AC-13; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-027 | high | Inherited untagged criterion FR-015-AC-19; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-028 | high | Inherited untagged criterion FR-015-AC-20; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-029 | high | Inherited untagged criterion FR-015-AC-21; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-030 | high | Inherited untagged criterion FR-015-AC-23; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-031 | high | Inherited untagged criterion FR-015-AC-24; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-032 | high | Inherited tagged-by-ignored-test criterion FR-015-AC-30; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-033 | high | Inherited tagged-by-ignored-test criterion FR-015-AC-31; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-034 | high | Inherited tagged-by-ignored-test criterion FR-015-AC-32; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-035 | high | Inherited untagged criterion FR-015-AC-40; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-036 | high | Inherited untagged criterion FR-015-AC-41; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-037 | high | Inherited untagged criterion FR-015-AC-42; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-038 | high | Inherited untagged criterion FR-015-AC-46; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-039 | high | Inherited untagged criterion FR-015-AC-48; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-040 | high | Inherited untagged criterion FR-015-AC-49; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-041 | high | Inherited untagged criterion FR-015-AC-51; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-042 | high | Inherited untagged criterion FR-015-AC-52; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-043 | high | Inherited untagged criterion FR-015-AC-54; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-044 | high | Inherited untagged criterion FR-015-AC-56; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-045 | high | Inherited tagged-by-ignored-test criterion FR-015-AC-57; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-046 | high | Inherited untagged criterion FR-015-AC-69; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-047 | high | Inherited untagged criterion FR-015-AC-70; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-048 | high | Inherited untagged criterion FR-015-AC-71; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-049 | high | Inherited untagged criterion FR-015-AC-72; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-050 | high | Inherited untagged criterion FR-015-AC-73; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-051 | high | Inherited untagged criterion FR-015-AC-74; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-052 | high | Inherited untagged criterion FR-015-AC-75; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-053 | high | Inherited untagged criterion FR-015-AC-76; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-054 | high | Inherited tagged-by-ignored-test criterion FR-015-AC-80; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-055 | high | Inherited untagged criterion FR-015-AC-84; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-056 | high | Inherited untagged criterion FR-015-AC-85; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-057 | high | Inherited untagged criterion FR-015-AC-87; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-058 | high | Inherited untagged criterion FR-015-AC-88; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-059 | high | Inherited untagged criterion FR-015-AC-89; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-060 | high | Inherited untagged criterion FR-015-AC-90; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-061 | high | Inherited untagged criterion FR-015-AC-92; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-062 | high | Inherited untagged criterion FR-015-AC-93; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-063 | high | Inherited untagged criterion FR-015-AC-95; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-064 | high | Inherited untagged criterion FR-015-AC-98; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-065 | high | Inherited untagged criterion FR-015-AC-105; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-066 | high | Inherited untagged criterion FR-015-AC-106; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-067 | high | Inherited untagged criterion FR-015-AC-107; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-068 | high | Inherited untagged criterion FR-015-AC-108; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-069 | high | Inherited untagged criterion FR-015-AC-111; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-070 | high | Inherited tagged-by-ignored-test criterion FR-015-AC-117; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-071 | high | Inherited untagged criterion FR-015-AC-119; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-072 | high | Inherited untagged criterion FR-015-AC-120; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-073 | high | Inherited untagged criterion FR-015-AC-121; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-074 | high | Inherited untagged criterion FR-015-AC-122; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-075 | high | Inherited untagged criterion FR-015-CON-1; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-076 | high | Inherited untagged criterion FR-015-CON-2; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-015-bounded-kani-obligations.md | correct-requirement-no-evidence |
| DEBT-077 | high | Inherited tagged-by-ignored-test criterion FR-017-AC-7; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-017-kani-execution-evidence.md | correct-requirement-no-evidence |
| DEBT-078 | high | Inherited untagged criterion FR-017-AC-19; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-017-kani-execution-evidence.md | correct-requirement-no-evidence |
| DEBT-079 | high | Inherited untagged criterion FR-017-AC-26; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-017-kani-execution-evidence.md | correct-requirement-no-evidence |
| DEBT-080 | high | Inherited tagged-by-ignored-test criterion FR-017-AC-45; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-017-kani-execution-evidence.md | correct-requirement-no-evidence |
| DEBT-081 | high | Inherited tagged-by-ignored-test criterion FR-017-AC-47; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-017-kani-execution-evidence.md | correct-requirement-no-evidence |
| DEBT-082 | high | Inherited untagged criterion FR-017-AC-49; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-017-kani-execution-evidence.md | correct-requirement-no-evidence |
| DEBT-083 | high | Inherited untagged criterion FR-017-AC-54; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-017-kani-execution-evidence.md | correct-requirement-no-evidence |
| DEBT-084 | high | Inherited untagged criterion FR-017-AC-57; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-017-kani-execution-evidence.md | correct-requirement-no-evidence |
| DEBT-085 | high | Inherited untagged criterion FR-017-AC-58; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-017-kani-execution-evidence.md | correct-requirement-no-evidence |
| DEBT-086 | high | Inherited untagged criterion FR-017-AC-59; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-017-kani-execution-evidence.md | correct-requirement-no-evidence |
| DEBT-087 | high | Inherited untagged criterion FR-017-AC-60; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-017-kani-execution-evidence.md | correct-requirement-no-evidence |
| DEBT-088 | high | Inherited untagged criterion FR-025-AC-1; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-025-generated-subject-abi.md | correct-requirement-no-evidence |
| DEBT-089 | high | Inherited untagged criterion FR-025-AC-2; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-025-generated-subject-abi.md | correct-requirement-no-evidence |
| DEBT-090 | high | Inherited untagged criterion FR-025-AC-3; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-025-generated-subject-abi.md | correct-requirement-no-evidence |
| DEBT-091 | high | Inherited untagged criterion FR-025-AC-4; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-025-generated-subject-abi.md | correct-requirement-no-evidence |
| DEBT-092 | high | Inherited untagged criterion FR-025-AC-5; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-025-generated-subject-abi.md | correct-requirement-no-evidence |
| DEBT-093 | high | Inherited untagged criterion FR-025-AC-6; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-025-generated-subject-abi.md | correct-requirement-no-evidence |
| DEBT-094 | high | Inherited untagged criterion FR-025-AC-7; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-025-generated-subject-abi.md | correct-requirement-no-evidence |
| DEBT-095 | high | Inherited untagged criterion FR-025-AC-8; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-025-generated-subject-abi.md | correct-requirement-no-evidence |
| DEBT-096 | high | Inherited untagged criterion FR-025-AC-9; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-025-generated-subject-abi.md | correct-requirement-no-evidence |
| DEBT-097 | high | Inherited untagged criterion FR-025-AC-10; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-025-generated-subject-abi.md | correct-requirement-no-evidence |
| DEBT-098 | high | Inherited untagged criterion FR-028-AC-5; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-028-bounded-proof-ceilings.md | correct-requirement-no-evidence |
| DEBT-099 | high | Inherited untagged criterion FR-028-AC-6; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-028-bounded-proof-ceilings.md | correct-requirement-no-evidence |
| DEBT-100 | high | Inherited untagged criterion FR-028-AC-7; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-028-bounded-proof-ceilings.md | correct-requirement-no-evidence |
| DEBT-101 | high | Inherited untagged criterion FR-028-AC-8; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-028-bounded-proof-ceilings.md | correct-requirement-no-evidence |
| DEBT-102 | high | Inherited untagged criterion FR-028-AC-9; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-028-bounded-proof-ceilings.md | correct-requirement-no-evidence |
| DEBT-103 | high | Inherited untagged criterion FR-028-AC-13; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-028-bounded-proof-ceilings.md | correct-requirement-no-evidence |
| DEBT-104 | high | Inherited untagged criterion FR-028-AC-14; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-028-bounded-proof-ceilings.md | correct-requirement-no-evidence |
| DEBT-105 | high | Inherited untagged criterion FR-028-AC-15; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-028-bounded-proof-ceilings.md | correct-requirement-no-evidence |
| DEBT-106 | high | Inherited untagged criterion FR-028-AC-16; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-028-bounded-proof-ceilings.md | correct-requirement-no-evidence |
| DEBT-107 | high | Inherited untagged criterion FR-028-AC-17; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-028-bounded-proof-ceilings.md | correct-requirement-no-evidence |
| DEBT-108 | high | Inherited untagged criterion FR-028-AC-18; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-028-bounded-proof-ceilings.md | correct-requirement-no-evidence |
| DEBT-109 | high | Inherited untagged criterion FR-028-AC-19; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-028-bounded-proof-ceilings.md | correct-requirement-no-evidence |
| DEBT-110 | high | Inherited untagged criterion FR-028-AC-20; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-028-bounded-proof-ceilings.md | correct-requirement-no-evidence |
| DEBT-111 | high | Inherited untagged criterion FR-028-AC-22; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-028-bounded-proof-ceilings.md | correct-requirement-no-evidence |
| DEBT-112 | high | Inherited untagged criterion FR-028-AC-23; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-028-bounded-proof-ceilings.md | correct-requirement-no-evidence |
| DEBT-113 | high | Inherited untagged criterion FR-028-AC-24; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-028-bounded-proof-ceilings.md | correct-requirement-no-evidence |
| DEBT-114 | high | Inherited untagged criterion FR-029-AC-17; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-029-run-outcome-terminal-record.md | correct-requirement-no-evidence |
| DEBT-115 | high | Inherited untagged criterion FR-029-AC-19; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-029-run-outcome-terminal-record.md | correct-requirement-no-evidence |
| DEBT-116 | high | Inherited untagged criterion FR-029-AC-21; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-029-run-outcome-terminal-record.md | correct-requirement-no-evidence |
| DEBT-117 | high | Inherited untagged criterion FR-029-AC-22; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-029-run-outcome-terminal-record.md | correct-requirement-no-evidence |
| DEBT-118 | high | Inherited untagged criterion FR-029-AC-23; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-029-run-outcome-terminal-record.md | correct-requirement-no-evidence |
| DEBT-119 | high | Inherited untagged criterion FR-029-AC-24; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-029-run-outcome-terminal-record.md | correct-requirement-no-evidence |
| DEBT-120 | high | Inherited untagged criterion FR-029-AC-25; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-029-run-outcome-terminal-record.md | correct-requirement-no-evidence |
| DEBT-121 | high | Inherited untagged criterion FR-029-AC-26; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-029-run-outcome-terminal-record.md | correct-requirement-no-evidence |
| DEBT-122 | high | Inherited untagged criterion FR-029-AC-27; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-029-run-outcome-terminal-record.md | correct-requirement-no-evidence |
| DEBT-123 | high | Inherited untagged criterion FR-029-AC-30; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-029-run-outcome-terminal-record.md | correct-requirement-no-evidence |
| DEBT-124 | high | Inherited untagged criterion FR-029-AC-31; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-029-run-outcome-terminal-record.md | correct-requirement-no-evidence |
| DEBT-125 | high | Inherited untagged criterion FR-030-AC-19; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md | correct-requirement-no-evidence |
| DEBT-126 | high | Inherited untagged criterion FR-030-AC-20; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md | correct-requirement-no-evidence |
| DEBT-127 | high | Inherited untagged criterion FR-030-AC-21; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md | correct-requirement-no-evidence |
| DEBT-128 | high | Inherited untagged criterion FR-030-AC-22; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md | correct-requirement-no-evidence |
| DEBT-129 | high | Inherited untagged criterion FR-034-AC-1; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-130 | high | Inherited untagged criterion FR-034-AC-2; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-131 | high | Inherited untagged criterion FR-034-AC-3; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-132 | high | Inherited untagged criterion FR-034-AC-4; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-133 | high | Inherited untagged criterion FR-034-AC-5; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-134 | high | Inherited untagged criterion FR-034-AC-6; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-135 | high | Inherited untagged criterion FR-034-AC-7; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-136 | high | Inherited untagged criterion FR-034-AC-8; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-137 | high | Inherited untagged criterion FR-034-AC-9; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-138 | high | Inherited untagged criterion FR-034-AC-10; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-139 | high | Inherited untagged criterion FR-034-AC-11; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-140 | high | Inherited untagged criterion FR-034-AC-12; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-141 | high | Inherited untagged criterion FR-034-AC-13; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-142 | high | Inherited untagged criterion FR-034-AC-14; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-143 | high | Inherited untagged criterion FR-034-AC-15; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-144 | high | Inherited untagged criterion FR-034-AC-16; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-145 | high | Inherited untagged criterion FR-034-AC-17; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-146 | high | Inherited untagged criterion FR-034-AC-18; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-147 | high | Inherited untagged criterion FR-034-AC-19; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-148 | high | Inherited untagged criterion FR-034-AC-20; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-149 | high | Inherited untagged criterion FR-034-AC-21; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-150 | high | Inherited untagged criterion FR-034-AC-22; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-151 | high | Inherited untagged criterion FR-034-AC-23; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-152 | high | Inherited untagged criterion FR-034-AC-24; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-153 | high | Inherited untagged criterion FR-034-AC-25; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-154 | high | Inherited untagged criterion FR-034-AC-26; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-155 | high | Inherited untagged criterion FR-034-AC-27; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-156 | high | Inherited untagged criterion FR-034-AC-28; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-157 | high | Inherited untagged criterion FR-034-AC-29; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-158 | high | Inherited untagged criterion FR-034-AC-31; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-159 | high | Inherited untagged criterion FR-034-AC-32; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-160 | high | Inherited untagged criterion FR-034-AC-33; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-161 | high | Inherited untagged criterion FR-034-AC-34; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-162 | high | Inherited untagged criterion FR-034-AC-35; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-163 | high | Inherited untagged criterion FR-034-AC-36; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-164 | high | Inherited untagged criterion FR-034-AC-37; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-165 | high | Inherited untagged criterion FR-034-AC-38; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-166 | high | Inherited untagged criterion FR-034-AC-39; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-167 | high | Inherited untagged criterion FR-034-AC-40; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-168 | high | Inherited untagged criterion FR-034-AC-51; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-169 | high | Inherited untagged criterion FR-034-AC-52; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-170 | high | Inherited untagged criterion FR-034-AC-53; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-171 | high | Inherited untagged criterion FR-034-AC-54; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-172 | high | Inherited untagged criterion FR-034-AC-57; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-173 | high | Inherited untagged criterion FR-034-AC-58; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-174 | high | Inherited untagged criterion FR-034-AC-59; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-175 | high | Inherited untagged criterion FR-034-AC-60; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-176 | high | Inherited untagged criterion FR-034-AC-61; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-177 | high | Inherited untagged criterion FR-034-AC-62; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-178 | high | Inherited untagged criterion FR-034-AC-63; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-179 | high | Inherited untagged criterion FR-034-AC-65; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-180 | high | Inherited untagged criterion FR-034-AC-66; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-181 | high | Inherited untagged criterion FR-034-AC-67; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-182 | high | Inherited untagged criterion FR-034-AC-69; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-183 | high | Inherited untagged criterion FR-034-AC-70; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-184 | high | Inherited untagged criterion FR-034-AC-71; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-185 | high | Inherited untagged criterion FR-034-AC-73; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-186 | high | Inherited untagged criterion FR-034-AC-74; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-187 | high | Inherited untagged criterion FR-034-AC-75; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-188 | high | Inherited untagged criterion FR-034-AC-76; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-189 | high | Inherited untagged criterion FR-034-AC-77; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-190 | high | Inherited untagged criterion FR-034-AC-78; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-191 | high | Inherited untagged criterion FR-034-AC-79; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-192 | high | Inherited untagged criterion FR-034-AC-80; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-193 | high | Inherited untagged criterion FR-034-AC-81; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-194 | high | Inherited untagged criterion FR-034-AC-82; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-195 | high | Inherited untagged criterion FR-034-AC-83; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-196 | high | Inherited untagged criterion FR-034-AC-84; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-197 | high | Inherited untagged criterion FR-034-AC-85; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-198 | high | Inherited untagged criterion FR-034-AC-86; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-199 | high | Inherited untagged criterion FR-034-AC-87; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-200 | high | Inherited untagged criterion FR-034-AC-88; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-201 | high | Inherited untagged criterion FR-034-AC-89; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-202 | high | Inherited untagged criterion FR-034-AC-90; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-203 | high | Inherited untagged criterion FR-034-AC-91; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-204 | high | Inherited untagged criterion FR-034-AC-92; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-205 | high | Inherited untagged criterion FR-034-AC-94; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-206 | high | Inherited untagged criterion FR-034-AC-95; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-207 | high | Inherited untagged criterion FR-034-AC-96; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-208 | high | Inherited untagged criterion FR-034-AC-97; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-209 | high | Inherited untagged criterion FR-034-AC-98; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-210 | high | Inherited untagged criterion FR-034-AC-99; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-211 | high | Inherited untagged criterion FR-034-AC-100; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-034-caller-death-ownership.md | correct-requirement-no-evidence |
| DEBT-212 | high | Inherited untagged criterion FR-041-AC-1; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-041-claimed-module-proof-gate.md | correct-requirement-no-evidence |
| DEBT-213 | high | Inherited untagged criterion FR-041-AC-2; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-041-claimed-module-proof-gate.md | correct-requirement-no-evidence |
| DEBT-214 | high | Inherited untagged criterion FR-041-AC-3; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-041-claimed-module-proof-gate.md | correct-requirement-no-evidence |
| DEBT-215 | high | Inherited untagged criterion FR-041-AC-4; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-041-claimed-module-proof-gate.md | correct-requirement-no-evidence |
| DEBT-216 | high | Inherited untagged criterion FR-041-AC-5; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-041-claimed-module-proof-gate.md | correct-requirement-no-evidence |
| DEBT-217 | high | Inherited untagged criterion FR-041-AC-6; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-041-claimed-module-proof-gate.md | correct-requirement-no-evidence |
| DEBT-218 | high | Inherited untagged criterion FR-041-AC-7; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-041-claimed-module-proof-gate.md | correct-requirement-no-evidence |
| DEBT-219 | high | Inherited untagged criterion FR-041-AC-8; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-041-claimed-module-proof-gate.md | correct-requirement-no-evidence |
| DEBT-220 | high | Inherited untagged criterion FR-041-AC-9; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-041-claimed-module-proof-gate.md | correct-requirement-no-evidence |
| DEBT-221 | high | Inherited untagged criterion FR-041-AC-10; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-041-claimed-module-proof-gate.md | correct-requirement-no-evidence |
| DEBT-222 | high | Inherited untagged criterion FR-046-AC-1; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md | correct-requirement-no-evidence |
| DEBT-223 | high | Inherited untagged criterion FR-046-AC-2; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md | correct-requirement-no-evidence |
| DEBT-224 | high | Inherited untagged criterion FR-046-AC-3; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md | correct-requirement-no-evidence |
| DEBT-225 | high | Inherited untagged criterion FR-046-AC-4; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md | correct-requirement-no-evidence |
| DEBT-226 | high | Inherited untagged criterion FR-046-AC-5; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md | correct-requirement-no-evidence |
| DEBT-227 | high | Inherited untagged criterion FR-046-AC-6; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md | correct-requirement-no-evidence |
| DEBT-228 | high | Inherited untagged criterion FR-046-AC-7; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md | correct-requirement-no-evidence |
| DEBT-229 | high | Inherited untagged criterion FR-046-AC-8; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md | correct-requirement-no-evidence |
| DEBT-230 | high | Inherited untagged criterion FR-046-AC-9; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md | correct-requirement-no-evidence |
| DEBT-231 | high | Inherited untagged criterion FR-046-AC-10; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-046-rational-function-predicate-harnesses.md | correct-requirement-no-evidence |
| DEBT-232 | high | Inherited untagged criterion FR-047-AC-1; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-047-v2-function-proof-producer.md | correct-requirement-no-evidence |
| DEBT-233 | high | Inherited untagged criterion FR-047-AC-2; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-047-v2-function-proof-producer.md | correct-requirement-no-evidence |
| DEBT-234 | high | Inherited untagged criterion FR-047-AC-3; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-047-v2-function-proof-producer.md | correct-requirement-no-evidence |
| DEBT-235 | high | Inherited untagged criterion FR-047-AC-4; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-047-v2-function-proof-producer.md | correct-requirement-no-evidence |
| DEBT-236 | high | Inherited untagged criterion FR-047-AC-5; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-047-v2-function-proof-producer.md | correct-requirement-no-evidence |
| DEBT-237 | high | Inherited untagged criterion FR-047-AC-6; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-047-v2-function-proof-producer.md | correct-requirement-no-evidence |
| DEBT-238 | high | Inherited untagged criterion FR-047-AC-7; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-047-v2-function-proof-producer.md | correct-requirement-no-evidence |
| DEBT-239 | high | Inherited tagged-by-ignored-test criterion FR-048-AC-3; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/functional/FR-048-constrained-state-clause-method.md | correct-requirement-no-evidence |
| DEBT-240 | high | Inherited untagged criterion NFR-006-AC-19; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md | correct-requirement-no-evidence |
| DEBT-241 | high | Inherited untagged criterion NFR-006-M-1; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md | correct-requirement-no-evidence |
| DEBT-242 | high | Inherited untagged criterion NFR-006-M-2; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md | correct-requirement-no-evidence |
| DEBT-243 | high | Inherited untagged criterion NFR-006-M-3; the same coverage status exists at origin/main. This PR does not discharge it. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md | correct-requirement-no-evidence |
| DEBT-244 | high | Inherited untagged criterion FR-014-AC-54; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-014-exact-scalar-oracles.md | correct-requirement-no-evidence |
| DEBT-245 | high | Inherited untagged criterion FR-018-AC-21; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-018-composite-equality-oracles.md | correct-requirement-no-evidence |
| DEBT-246 | high | Inherited untagged criterion FR-018-AC-22; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-018-composite-equality-oracles.md | correct-requirement-no-evidence |
| DEBT-247 | high | Inherited untagged criterion FR-018-AC-23; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-018-composite-equality-oracles.md | correct-requirement-no-evidence |
| DEBT-248 | high | Inherited untagged criterion FR-021-AC-30; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-021-function-application-oracles.md | correct-requirement-no-evidence |
| DEBT-249 | high | Inherited untagged criterion FR-031-AC-11; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md | correct-requirement-no-evidence |
| DEBT-250 | high | Inherited untagged criterion FR-031-AC-27; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md | correct-requirement-no-evidence |
| DEBT-251 | high | Inherited untagged criterion FR-031-AC-19; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md | correct-requirement-no-evidence |
| DEBT-252 | high | Inherited untagged criterion FR-031-AC-20; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md | correct-requirement-no-evidence |
| DEBT-253 | high | Inherited untagged criterion FR-031-AC-21; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md | correct-requirement-no-evidence |
| DEBT-254 | high | Inherited untagged criterion FR-031-AC-23; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md | correct-requirement-no-evidence |
| DEBT-255 | high | Inherited untagged criterion FR-031-AC-24; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md | correct-requirement-no-evidence |
| DEBT-256 | high | Inherited untagged criterion FR-031-AC-25; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md | correct-requirement-no-evidence |
| DEBT-257 | high | Inherited untagged criterion FR-035-AC-1; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-035-caller-text-admission.md | correct-requirement-no-evidence |
| DEBT-258 | high | Inherited untagged criterion FR-035-AC-2; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-035-caller-text-admission.md | correct-requirement-no-evidence |
| DEBT-259 | high | Inherited untagged criterion FR-035-AC-3; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-035-caller-text-admission.md | correct-requirement-no-evidence |
| DEBT-260 | high | Inherited untagged criterion FR-035-AC-4; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-035-caller-text-admission.md | correct-requirement-no-evidence |
| DEBT-261 | high | Inherited untagged criterion FR-035-AC-5; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-035-caller-text-admission.md | correct-requirement-no-evidence |
| DEBT-262 | high | Inherited untagged criterion FR-035-AC-6; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-035-caller-text-admission.md | correct-requirement-no-evidence |
| DEBT-263 | high | Inherited untagged criterion FR-044-AC-1; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-044-export-generated-closure-evaluation-corpus.md | correct-requirement-no-evidence |
| DEBT-264 | high | Inherited untagged criterion FR-044-AC-2; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-044-export-generated-closure-evaluation-corpus.md | correct-requirement-no-evidence |
| DEBT-265 | high | Inherited untagged criterion FR-044-AC-4; the same coverage status exists at origin/main. This PR does not discharge it. | spec/oracle/functional/FR-044-export-generated-closure-evaluation-corpus.md | correct-requirement-no-evidence |
| DEBT-266 | high | Inherited untagged criterion FR-024-AC-7; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-024-counterexample-envelope-intake.md | correct-requirement-no-evidence |
| DEBT-267 | high | Inherited untagged criterion FR-024-AC-8; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-024-counterexample-envelope-intake.md | correct-requirement-no-evidence |
| DEBT-268 | high | Inherited tagged-by-ignored-test criterion FR-024-AC-18; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-024-counterexample-envelope-intake.md | correct-requirement-no-evidence |
| DEBT-269 | high | Inherited tagged-by-ignored-test criterion FR-024-AC-34; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-024-counterexample-envelope-intake.md | correct-requirement-no-evidence |
| DEBT-270 | high | Inherited untagged criterion FR-024-AC-35; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-024-counterexample-envelope-intake.md | correct-requirement-no-evidence |
| DEBT-271 | high | Inherited untagged criterion FR-024-AC-36; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-024-counterexample-envelope-intake.md | correct-requirement-no-evidence |
| DEBT-272 | high | Inherited untagged criterion FR-024-AC-37; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-024-counterexample-envelope-intake.md | correct-requirement-no-evidence |
| DEBT-273 | high | Inherited untagged criterion FR-024-AC-38; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-024-counterexample-envelope-intake.md | correct-requirement-no-evidence |
| DEBT-274 | high | Inherited untagged criterion FR-024-AC-39; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-024-counterexample-envelope-intake.md | correct-requirement-no-evidence |
| DEBT-275 | high | Inherited untagged criterion FR-024-AC-40; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-024-counterexample-envelope-intake.md | correct-requirement-no-evidence |
| DEBT-276 | high | Inherited untagged criterion FR-024-AC-49; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-024-counterexample-envelope-intake.md | correct-requirement-no-evidence |
| DEBT-277 | high | Inherited untagged criterion FR-024-AC-51; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-024-counterexample-envelope-intake.md | correct-requirement-no-evidence |
| DEBT-278 | high | Inherited untagged criterion FR-032-AC-1; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md | correct-requirement-no-evidence |
| DEBT-279 | high | Inherited untagged criterion FR-032-AC-4; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md | correct-requirement-no-evidence |
| DEBT-280 | high | Inherited untagged criterion FR-032-AC-5; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md | correct-requirement-no-evidence |
| DEBT-281 | high | Inherited untagged criterion FR-032-AC-7; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md | correct-requirement-no-evidence |
| DEBT-282 | high | Inherited untagged criterion FR-032-AC-8; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md | correct-requirement-no-evidence |
| DEBT-283 | high | Inherited untagged criterion FR-032-AC-12; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md | correct-requirement-no-evidence |
| DEBT-284 | high | Inherited untagged criterion FR-032-AC-14; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md | correct-requirement-no-evidence |
| DEBT-285 | high | Inherited untagged criterion FR-033-AC-2; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-033-composite-parity-replay-binding.md | correct-requirement-no-evidence |
| DEBT-286 | high | Inherited untagged criterion FR-033-AC-3; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-033-composite-parity-replay-binding.md | correct-requirement-no-evidence |
| DEBT-287 | high | Inherited untagged criterion FR-033-AC-4; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-033-composite-parity-replay-binding.md | correct-requirement-no-evidence |
| DEBT-288 | high | Inherited untagged criterion FR-033-AC-5; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-033-composite-parity-replay-binding.md | correct-requirement-no-evidence |
| DEBT-289 | high | Inherited untagged criterion FR-033-AC-8; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-033-composite-parity-replay-binding.md | correct-requirement-no-evidence |
| DEBT-290 | high | Inherited untagged criterion FR-033-AC-10; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-033-composite-parity-replay-binding.md | correct-requirement-no-evidence |
| DEBT-291 | high | Inherited untagged criterion FR-033-AC-14; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-033-composite-parity-replay-binding.md | correct-requirement-no-evidence |
| DEBT-292 | high | Inherited untagged criterion FR-033-AC-15; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-033-composite-parity-replay-binding.md | correct-requirement-no-evidence |
| DEBT-293 | high | Inherited untagged criterion FR-033-AC-17; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-033-composite-parity-replay-binding.md | correct-requirement-no-evidence |
| DEBT-294 | high | Inherited untagged criterion FR-033-AC-19; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-033-composite-parity-replay-binding.md | correct-requirement-no-evidence |
| DEBT-295 | high | Inherited untagged criterion FR-033-AC-20; the same coverage status exists at origin/main. This PR does not discharge it. | spec/replay/functional/FR-033-composite-parity-replay-binding.md | correct-requirement-no-evidence |
| DEBT-296 | high | Inherited untagged criterion FR-019-AC-3; the same coverage status exists at origin/main. This PR does not discharge it. | spec/routed/functional/FR-019-capability-settlement.md | correct-requirement-no-evidence |
| DEBT-297 | high | Inherited untagged criterion FR-019-AC-15; the same coverage status exists at origin/main. This PR does not discharge it. | spec/routed/functional/FR-019-capability-settlement.md | correct-requirement-no-evidence |
| DEBT-298 | high | Inherited untagged criterion FR-019-AC-24; the same coverage status exists at origin/main. This PR does not discharge it. | spec/routed/functional/FR-019-capability-settlement.md | correct-requirement-no-evidence |
| DEBT-299 | high | Inherited untagged criterion FR-019-AC-34; the same coverage status exists at origin/main. This PR does not discharge it. | spec/routed/functional/FR-019-capability-settlement.md | correct-requirement-no-evidence |
| DEBT-300 | high | Inherited untagged criterion FR-019-AC-35; the same coverage status exists at origin/main. This PR does not discharge it. | spec/routed/functional/FR-019-capability-settlement.md | correct-requirement-no-evidence |
| DEBT-301 | high | Inherited untagged criterion FR-019-AC-36; the same coverage status exists at origin/main. This PR does not discharge it. | spec/routed/functional/FR-019-capability-settlement.md | correct-requirement-no-evidence |
| DEBT-302 | high | Inherited untagged criterion FR-019-AC-37; the same coverage status exists at origin/main. This PR does not discharge it. | spec/routed/functional/FR-019-capability-settlement.md | correct-requirement-no-evidence |
| DEBT-303 | high | Inherited untagged criterion FR-019-AC-38; the same coverage status exists at origin/main. This PR does not discharge it. | spec/routed/functional/FR-019-capability-settlement.md | correct-requirement-no-evidence |
| DEBT-304 | high | Inherited untagged criterion FR-019-AC-39; the same coverage status exists at origin/main. This PR does not discharge it. | spec/routed/functional/FR-019-capability-settlement.md | correct-requirement-no-evidence |
| DEBT-305 | high | Inherited untagged criterion FR-019-AC-40; the same coverage status exists at origin/main. This PR does not discharge it. | spec/routed/functional/FR-019-capability-settlement.md | correct-requirement-no-evidence |
| DEBT-306 | high | Inherited untagged criterion FR-019-AC-41; the same coverage status exists at origin/main. This PR does not discharge it. | spec/routed/functional/FR-019-capability-settlement.md | correct-requirement-no-evidence |
| DEBT-307 | high | Inherited untagged criterion FR-022-AC-26; the same coverage status exists at origin/main. This PR does not discharge it. | spec/routed/functional/FR-022-routed-generation.md | correct-requirement-no-evidence |
| DEBT-308 | high | Inherited untagged criterion FR-022-AC-27; the same coverage status exists at origin/main. This PR does not discharge it. | spec/routed/functional/FR-022-routed-generation.md | correct-requirement-no-evidence |
| DEBT-309 | high | Inherited untagged criterion FR-022-AC-28; the same coverage status exists at origin/main. This PR does not discharge it. | spec/routed/functional/FR-022-routed-generation.md | correct-requirement-no-evidence |
| DEBT-310 | high | Inherited untagged criterion FR-026-AC-1; the same coverage status exists at origin/main. This PR does not discharge it. | spec/routed/functional/FR-026-backend-adapter-contract.md | correct-requirement-no-evidence |
| DEBT-311 | high | Inherited untagged criterion FR-026-AC-4; the same coverage status exists at origin/main. This PR does not discharge it. | spec/routed/functional/FR-026-backend-adapter-contract.md | correct-requirement-no-evidence |
| DEBT-312 | high | Inherited untagged criterion FR-045-AC-1; the same coverage status exists at origin/main. This PR does not discharge it. | spec/routed/functional/FR-045-hyper-smt-induction-map.md | correct-requirement-no-evidence |
| DEBT-313 | high | Inherited untagged criterion FR-045-AC-5; the same coverage status exists at origin/main. This PR does not discharge it. | spec/routed/functional/FR-045-hyper-smt-induction-map.md | correct-requirement-no-evidence |
| DEBT-314 | high | Inherited untagged criterion FR-002-AC-7; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-002-tristate-proptest.md | correct-requirement-no-evidence |
| DEBT-315 | high | Inherited untagged criterion FR-002-AC-8; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-002-tristate-proptest.md | correct-requirement-no-evidence |
| DEBT-316 | high | Inherited untagged criterion FR-002-AC-10; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-002-tristate-proptest.md | correct-requirement-no-evidence |
| DEBT-317 | high | Inherited untagged criterion FR-002-AC-11; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-002-tristate-proptest.md | correct-requirement-no-evidence |
| DEBT-318 | high | Inherited untagged criterion FR-002-AC-12; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-002-tristate-proptest.md | correct-requirement-no-evidence |
| DEBT-319 | high | Inherited untagged criterion FR-002-AC-13; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-002-tristate-proptest.md | correct-requirement-no-evidence |
| DEBT-320 | high | Inherited untagged criterion FR-002-AC-14; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-002-tristate-proptest.md | correct-requirement-no-evidence |
| DEBT-321 | high | Inherited untagged criterion FR-002-AC-15; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-002-tristate-proptest.md | correct-requirement-no-evidence |
| DEBT-322 | high | Inherited untagged criterion FR-002-AC-16; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-002-tristate-proptest.md | correct-requirement-no-evidence |
| DEBT-323 | high | Inherited untagged criterion FR-002-AC-17; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-002-tristate-proptest.md | correct-requirement-no-evidence |
| DEBT-324 | high | Inherited untagged criterion FR-002-AC-18; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-002-tristate-proptest.md | correct-requirement-no-evidence |
| DEBT-325 | high | Inherited untagged criterion FR-002-AC-19; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-002-tristate-proptest.md | correct-requirement-no-evidence |
| DEBT-326 | high | Inherited untagged criterion FR-002-AC-20; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-002-tristate-proptest.md | correct-requirement-no-evidence |
| DEBT-327 | high | Inherited untagged criterion FR-002-AC-22; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-002-tristate-proptest.md | correct-requirement-no-evidence |
| DEBT-328 | high | Inherited untagged criterion FR-002-AC-24; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-002-tristate-proptest.md | correct-requirement-no-evidence |
| DEBT-329 | high | Inherited untagged criterion FR-002-AC-25; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-002-tristate-proptest.md | correct-requirement-no-evidence |
| DEBT-330 | high | Inherited untagged criterion FR-002-AC-26; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-002-tristate-proptest.md | correct-requirement-no-evidence |
| DEBT-331 | high | Inherited untagged criterion FR-008-AC-7; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md | correct-requirement-no-evidence |
| DEBT-332 | high | Inherited untagged criterion FR-008-AC-9; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md | correct-requirement-no-evidence |
| DEBT-333 | high | Inherited untagged criterion FR-009-AC-7; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-009-constructive-correlated-populations.md | correct-requirement-no-evidence |
| DEBT-334 | high | Inherited untagged criterion FR-011-AC-6; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-011-numeric-harness-campaigns.md | correct-requirement-no-evidence |
| DEBT-335 | high | Inherited untagged criterion FR-011-AC-7; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-011-numeric-harness-campaigns.md | correct-requirement-no-evidence |
| DEBT-336 | high | Inherited untagged criterion FR-011-AC-10; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-011-numeric-harness-campaigns.md | correct-requirement-no-evidence |
| DEBT-337 | high | Inherited untagged criterion FR-011-AC-12; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-011-numeric-harness-campaigns.md | correct-requirement-no-evidence |
| DEBT-338 | high | Inherited untagged criterion FR-011-AC-18; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-011-numeric-harness-campaigns.md | correct-requirement-no-evidence |
| DEBT-339 | high | Inherited untagged criterion FR-011-AC-19; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-011-numeric-harness-campaigns.md | correct-requirement-no-evidence |
| DEBT-340 | high | Inherited untagged criterion FR-011-AC-20; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-011-numeric-harness-campaigns.md | correct-requirement-no-evidence |
| DEBT-341 | high | Inherited untagged criterion FR-011-AC-21; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/functional/FR-011-numeric-harness-campaigns.md | correct-requirement-no-evidence |
| DEBT-342 | high | Inherited untagged criterion NFR-004-M-1; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md | correct-requirement-no-evidence |
| DEBT-343 | high | Inherited untagged criterion NFR-004-M-2; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md | correct-requirement-no-evidence |
| DEBT-344 | high | Inherited untagged criterion NFR-004-M-3; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md | correct-requirement-no-evidence |
| DEBT-345 | high | Inherited untagged criterion NFR-004-M-4; the same coverage status exists at origin/main. This PR does not discharge it. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md | correct-requirement-no-evidence |


## Whole-repository computed matrix context

Mechanical trace inventory; this does not claim semantic inspection of each inherited criterion.

```yaml
scope:
- id: FR-005-AC-1
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: Failed generation leaves no partial published bundle or edited developer
    region.
- id: FR-005-AC-2
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: Supported platforms reproduce byte-identical files.
- id: FR-005-AC-3
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: For every shared-corpus case on which two backends' normalized outcomes
    differ, the TC-007 Differential Disposition Register holds exactly one row naming
    the case, the backends, and either a regression fixture that exists in the test
    tree or a documented profile/backend distinction; a differing case with no row,
    a row naming a fixture that does not exist, or a row with neither disposition
    fails the check.
- id: FR-005-AC-4
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: 'OPEN: after IR-348 V1 retirement, agreement must be remeasured across
    surviving executable, proptest, Kani and coverage paths; removal alone is not
    agreement. Original criterion retained: Executable, proptest, Kani, and coverage
    semantics agree or retain an explicit difference.'
- id: FR-005-AC-5
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: A bundle naming more than 4096 artifacts, an artifact whose contents exceed
    16,777,216 bytes, or a complete artifact byte total exceeding 134,217,728 bytes
    is refused at construction as a bounded-resource `InvalidBundle` diagnostic before
    any staging or destination I/O begins, so no partial bundle is ever published.
- id: FR-005-AC-6
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When an artifact path is absolute, or contains a `..` segment, a `.` segment,
    an empty segment, a trailing separator, a backslash or a control character, or
    is empty, bundle construction is refused with `UnsafeArtifactPath` naming the
    offending `bundle.artifacts[i].path`, before any destination or staging I/O begins.
- id: FR-005-AC-7
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When publication fails before the destination is replaced, or fails after
    replacement began and the prior destination is restored, the publisher reports
    destination state `unchanged`, the destination holds its prior bytes or remains
    absent, an adjacent developer-owned file is byte-identical, and no staging or
    backup sibling remains.
- id: FR-005-AC-8
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When removal of the replaced bundle's backup fails after the staged bundle
    has replaced the destination, the publisher reports destination state `published`,
    the destination holds every artifact of the new bundle, and the backup sibling
    remains as visible residue.
- id: FR-005-AC-9
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When replacement fails and restoring the prior destination also fails,
    the publisher reports destination state `unknown`, the destination path is absent,
    and the complete prior bundle and the complete staged bundle remain in their sibling
    directories for recovery.
- id: FR-005-AC-10
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When two artifacts of one bundle share the same path, bundle construction
    is refused with `DuplicateArtifactPath` naming the later `bundle.artifacts[i].path`,
    before any destination or staging I/O begins.
- id: FR-037-AC-1
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: RETIRED, replaced by FR-037-AC-6; `derive_application_node_id`, `ApplicationNodeDraft`
    and application preimage identities no longer exist.
- id: FR-037-AC-2
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: RETIRED, replaced by FR-037-AC-7; `ApplicationIdentityContext` and canonical
    preimage byte-limit derivation no longer exist.
- id: FR-037-AC-3
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: RETIRED, replaced by FR-037-AC-8; enum-member digest identity no longer
    defines its structural node ID.
- id: FR-037-AC-4
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: RETIRED, replaced by FR-037-AC-9; body-only changes need an admitted-body
    comparison, not automatic rekeying.
- id: FR-037-AC-5
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: RETIRED, replaced by FR-037-AC-10; package identity refresh through a typed
    digest preimage is obsolete.
- id: FR-037-AC-6
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: Both fixture builders preserve every producer-assigned structural application
    `CheckedNodeId` in the assembled node and its dependent references; an ungrouped
    fixture's requested ID equals the reader-admitted node ID. Inspection finds no
    local application ID derivation, application preimage encoder, version constant
    or digest call for that identity.
- id: FR-037-AC-7
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: A recursion-group fixture retains the complete group, each member's exact
    producer-assigned structural ID, including its own ordinal, and references to
    those exact IDs; the strict reader admits the valid group without imposing contiguous
    group positions. A missing referenced member or a declaration path inconsistent
    with `node_id.path` produces the reader's typed refusal and no admitted fixture.
    Exceeding the complete-package byte ceiling retains the reader's typed limit result
    and admits no fixture.
- id: FR-037-AC-8
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: A canonical enum-member fixture carries its producer-assigned structural
    `CheckedNodeId`; the reader admits its member declaration and exact reference.
    Changing the case through the producer yields the corresponding structural member
    ID without a CG nominal preimage or digest. In-process agreement keys never compared
    with a checked-package identity remain classified as fixture-only.
- id: FR-037-AC-9
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: Positive exact-scalar and composite-equality fixture packages remain reader-admitted.
    A declaration whose path disagrees with `node_id.path` is refused by the reader.
    A changed body under the same valid structural key is detected separately by comparison
    with the expected admitted source body and is not silently credited as the original
    fixture; neither negative control is repaired before its assertion. A different
    valid unused ordinal with consistent references is not itself a reader refusal.
- id: FR-037-AC-10
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: Deterministic fixture-only keys remain distinct from canonical checked-package
    identities. Ordinary canonical-value identities use their owning API where needed;
    originating package identity comes from the producer and reader admission, with
    no CG package preimage, package digest refresh or independent application identity
    rule.
- id: FR-038-AC-1
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: 'A source inventory compares every identity in [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md)''s
    specified derivable set with the implemented scalar, scalar Kani and state/frame
    selections, recording the unimplemented scalar integer `eq/ne` and Boolean variants
    as open FR-014 work. For the four integer arithmetic identities shared by scalar
    oracle and scalar Kani, and the four integer ordering identities shared by scalar
    oracle and state/frame Kani, the implemented consumers resolve the same typed
    identity, family, arity and ordered operands; no second string-to-operator match '
- id: FR-038-AC-2
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: The typed source distinguishes implemented integer, rational, decimal,
    IEEE, text, enum and quantity operations and their applicable result and mode
    forms. When FR-014's specified Boolean scalar variants are implemented, they use
    distinct Boolean entries in this same source before the scalar emitter selects
    them. A valid caller-supplied scalar descriptor with a present but disagreeing
    node identity, law or mode still generates its descriptor-named oracle as `caller_declared`
    with a typed blocked item under FR-014-AC-12 and AC-13. A missing node identity
    retains its typed refusal; an underivabl
- id: FR-038-AC-3
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: Focused V2 oracle, scalar Kani and state/frame tests distinguish the wrong
    identity, family, operand arity or order, comparison symbol and applicable runtime
    callee from the correct rendering. With the correct identity and arity, a wrong
    fixed result form or wrong member-selected result target, including a conversion
    target, is refused under FR-014's result/parameter checks rather than rendered
    as the right operator with the wrong result. Existing admission and generated-artifact
    behavior for supported identities remains equivalent.
- id: FR-038-AC-4
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: The four scalar Kani harnesses still compare their embedded oracle result
    with a separately rendered native `i128` arithmetic expression. A mutation of
    the production oracle's selected operation or callee makes the real Kani proof
    fail under an in-domain non-vacuous control; routing the native expression through
    that callee fails review.
- id: FR-038-AC-5
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: The V1 `strategy/bound` relation evaluator remains outside this V2 catalog
    migration until the V2 strategy replacement specified by [AD-004](../../assurance/AD-004-cg-crate-layout.md)
    and [FR-008](../../strategy/functional/FR-008-bound-domain-strategy-admission.md).
    The hand-built bounded corpus is also excluded, and its duplicate `checked_method`
    mapping is removed only by [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
    and AD-004 step 4g retirement after that step's proof controls pass. Neither path
    gains a compatibility forwarding layer.
- id: FR-038-AC-6
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: While the V1 `strategy/bound` relation evaluator remains live, focused
    tests discriminate all six comparison operators and a primary read on the left
    versus on the right of a noncommutative comparison; its host evaluation preserves
    the relation as authored.
- id: FR-039-AC-1
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: Each of two genuine QSL-admitted source graphs with a different clause
    or pure-function operation lowers through IR to its own admitted FR-330 package.
    CG's production settlement and generation use each package's typed graph and accounting
    for its exact request index and node ID; supplying the two packages with each
    one's corresponding node ID at the same request position changes the generated
    bytes to reflect the represented operation while backend and generation context
    stay fixed. If the production API still accepts an original checked graph, holding
    that legacy input fixed across the two c
- id: FR-039-AC-2
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: A mixed request with a representable clause or function and an independent
    `unsupported`, `requires_bound` or `invalid_input` sibling retains one typed decision
    per bound driver item, including duplicate requested IDs, and emits only the representable
    sibling's artifact. An interleaved context-only root retains its own exact accounting
    position and key without creating a driver decision. For two QSL scalar items
    sharing one expression key but selecting different checked application/conversion
    pairs, CG retains each pair's own admitted FR-049 result bound; a conversion outside
    its driver's cont
- id: FR-039-AC-3
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: A selected dependency function reference resolves only through the reader-validated
    selected package identity and target key. A missing, wrong-identity or stale selected
    dependency refuses before CG; an identically keyed local node or extra unselected
    dependency cannot redirect generation.
- id: FR-039-AC-4
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: For a genuinely admitted FR-330 package, CG binds a `protocol`/`protocol_reference`
    carrier's exact `body.target` to the local `protocol` node, retaining its typed
    key, requested identity and source occurrences; changing that local target and
    recomputing the admitted carrier and package identities changes the selected target
    and package identity. A matching key only in a selected dependency does not bind
    locally. An absent local target, a present non-`protocol` target and a target
    with unequal `semantic_type` are refused by the FR-322 reader before CG with the
    respective `missing_declaration`/
- id: FR-039-AC-5
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: A malformed or noncanonical FR-330 wire, wrong version, invalid accounting/source
    map, stale semantic identity or incomplete reader result reaches no CG consumer
    and emits no artifact. CG preserves the reader's typed refusal or limit result
    and performs no second parse or checked-graph fallback.
- id: FR-039-AC-6
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: An item whose admitted package accounting is `failed`, `invalid_body` or
    `body_incomplete`, or whose `lowered` form is unrepresentable by the selected
    backend, emits no artifact and retains a distinct typed cause; the package byte-ceiling
    case emits no artifact for any sibling.
- id: FR-039-AC-7
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: RETIRED, replaced by FR-039-AC-11; the former `ContractPackageV1.identity()`
    and digest-shaped source assumption is obsolete and earns no implementation credit.
- id: FR-039-AC-8
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: 'For each admitted `lowered` bound driver item, Contract IR''s authoritative
    traversal is total over the required root, bounds and selected dependencies, and
    the generated claim''s reached claim, bound and dependency keys and dependent
    declaration closure equal its output. An unresolved required reference cannot
    reach CG as an admitted `lowered` item: Contract IR records a typed item refusal
    or its strict reader refuses the package; a refused or incomplete reader result
    yields no item claim or artifact. Inspection finds no per-node ID from a second
    lowering or wire-fragment hash.'
- id: FR-039-AC-9
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: Before matching positions, CG verifies that the owning reader-admitted
    V2 source's exact top-level name and complete lock equal the admitted V1 `package.source`
    name and complete lock, either directly or through Contract IR's explicit authenticated
    source handoff. A wrong-name or wrong-lock source refuses the whole request before
    any artifact even when its requested key resolves to the same structural key as
    V1 accounting. For a matching source, CG matches a bound driver item, an excluded
    accounting-only position and another bound driver item to `feature_accounting`
    by position and equal sourc
- id: FR-039-AC-10
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: Given a bound driver item followed by an excluded accounting-only position,
    a selected conversion context root outside the preceding item's contiguous span
    yields typed `IncompleteContext` for that item when the complete root sequence
    is otherwise valid. A context-only root directly after the excluded position and
    before the next bound driver item is orphaned and causes a typed whole-request
    correspondence refusal before any artifact, even when its accounting position
    and source-bound structural key match. A conversion root immediately before the
    excluded position remains in the preceding item
- id: FR-039-AC-11
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: Generated scalar, composite equality and exact function claims each serialize
    `contract_package_identity` equal to the admitted FR-330 `package.source` structural
    package name plus complete lock, replacing the removed `ContractPackageV1.identity()`
    accessor, and `node_id` equal to the original requested V2 `CheckedNodeId`, including
    graph-local indexes; their containing claim-map entries use that same `node_id`,
    with no generated-claim `ir_id` key or second public V1 node identity. CG matches
    the request to V1 accounting through equal source-bound structural keys resolved
    by `ContractSourceKey
- id: FR-043-AC-1
  path: spec/core/functional/FR-043-owned-lock-coherence-gate.md
  role: examined
  excerpt: 'RETIRED under IR-1105: no owned `quire check-lock Cargo.lock` invocation
    is required in `make deny`.'
- id: FR-043-AC-2
  path: spec/core/functional/FR-043-owned-lock-coherence-gate.md
  role: examined
  excerpt: 'RETIRED under IR-1105: no duplicate-package or split-revision checker
    controls are required.'
- id: FR-043-AC-3
  path: spec/core/functional/FR-043-owned-lock-coherence-gate.md
  role: examined
  excerpt: 'RETIRED under IR-1105: no missing-checker or checker-refusal Make gate
    is required.'
- id: FR-043-AC-4
  path: spec/core/functional/FR-043-owned-lock-coherence-gate.md
  role: examined
  excerpt: 'RETIRED under IR-1105: the planned replacement-checker assertion does
    not apply. The local `scripts/check_one_copy.awk` implementation is removed as
    part of the same retirement.'
- id: FR-043-AC-5
  path: spec/core/functional/FR-043-owned-lock-coherence-gate.md
  role: examined
  excerpt: 'RETIRED under IR-1105: no lock-coherence checker qualification or receipt
    is required.'
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
  excerpt: Every generated Rust file in a family named in Scope has a header naming
    `quire-contract-codegen`, every input-carried subject identity it renders as defined
    in Scope, and every requirement, revision or clause identifier present in the
    generation input.
- id: NFR-002-AC-2
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Every generated Rust file carries the `MIT OR Apache-2.0` SPDX identity.
- id: NFR-002-AC-3
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Unsupported and inconclusive states remain explicit rather than silently
    succeeding.
- id: NFR-002-AC-4
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Generated output makes no project-specific validation, accreditation, certification,
    or release-approval claim.
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
- id: NFR-005-AC-1
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: Every `.rs` file under `src/`, except a file its parent module declares
    under `#[cfg(test)]`, contains zero panic tokens in its non-test code with comments,
    every `#[cfg(test)]` item and every string and character literal removed (`non_test_code_outside_literals`,
    a variant of `non_test_code` that `tests/common/panic_scan.rs` gains), including
    the body of `fn digest` of `impl CaseIdentity` in `src/kani/generate/corpus/bounded_kani_corpus.rs`;
    no `expect` in that file, `render_artifacts` included, is allowed. The tokens
    are the identifiers `unwrap`, `expect`, `unwrap_unchecked`, `unwrap_err`, `
- id: NFR-005-AC-2
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: '`lower_scalar_claim` returns `Err(ScalarLoweringRefusal::NoRenderer)`
    for a claim whose `checked_bounds` is empty and for a claim whose first checked
    bound has no `IntegerRange` among the derived domains, each built directly against
    the function, and `classify_claim` reports each as `UnsupportedObligation::OperationNotRendered`.'
- id: NFR-005-AC-3
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: '`ItemSettlement::warning` returns `None` for a `Disposition::Unsupported`
    carrying each of `Cause::AbsentKind`, `UnknownKind`, `AbsentExtent`, `UnknownBackend`,
    `InconsistentCandidates` and `AmbiguousBackend`, and still returns its warning
    for each `unsupported_projection` cause.'
- id: NFR-005-AC-4
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: With a coverage export supplied, `observe_clause` given a map whose oracle-evaluation
    region has no probe returns a row with a `MapMismatch` diagnostic with the message
    `missing semantic probe`, no classification and an `evaluation_count` of `None`,
    and given one whose consequent region has no probe returns the same diagnostic
    with no classification. With no coverage export it returns `UnavailableObservation`
    for the same maps.
- id: NFR-005-AC-5
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: The step of `generate_kani` that pairs FR-015's records with the Kani group,
    given hand-built inputs in a `#[cfg(test)]` module, returns `RoutedGenerationError::KaniRecordCountMismatch
    { records, items }` carrying both counts when the counts differ, in both directions,
    and pairs every item with its record when they are equal.
- id: NFR-005-AC-6
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: '`rewrite_duplicate_position`, a function of `src/routed/generate.rs` that
    `route_records` (which `generate_kani` calls to pair, rewrite and join FR-015''s
    records) calls to rewrite a `DuplicateItem` record''s `first_index` into the driver''s
    request index, given hand-built records and a group in a `#[cfg(test)]` module,
    returns `RoutedGenerationError::KaniDuplicatePositionOutOfRange { first_index,
    items }` carrying the position and the group''s length when `first_index` equals
    or exceeds the group''s length, with nothing generated, and returns the request
    index of the earlier item when `first_index'
- id: NFR-005-AC-7
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: '`observe_clause`, given a map of zero regions and a map of one region,
    each with and without a coverage export, returns a row with a `MapMismatch` diagnostic
    with the message `typed implication census differs from map`, no classification,
    an `evaluation_count` of `None` and no consequents; given a map of two probed
    regions and a coverage export it returns a row with no consequents and no `MapMismatch`
    diagnostic.'
- id: NFR-005-AC-8
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: The scan locates the body of `generate_kani`, `route_records`, `rewrite_duplicate_position`,
    `observe_clause` and `generate_boolean_oracle_inner` in the literal-free non-test
    code of its file, asserts that it found each of the five and that each holds an
    identifier of its own, and asserts that none contains an index token or a subtraction
    token, so that a later change cannot bring back one of the IR-577 sites. An operand-ending
    token is `)`, `]`, `}`, `?`, a numeric literal (so the `0` of a tuple field `t.0`
    counts), or an identifier that is not a Rust keyword (`in`, `let`, `mut`, `ref`,
    `retu
- id: NFR-005-M-1
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: Panic tokens in the non-test, literal-free code of `src/`
- id: NFR-005-M-2
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: Measured sites a test can reach that abort instead of returning a typed
    value
- id: NFR-005-M-3
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: Index, slice and unchecked subtraction expressions in the bodies of the
    five IR-577 functions
- id: FR-004-AC-1
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: An evaluated implication whose consequent is unobserved yields a vacuity
    finding even when every oracle return was true; the consuming coverage obligation
    denies success for that clause.
- id: FR-004-AC-2
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: A clause whose oracle-evaluation region was not observed is unexecuted,
    not vacuous.
- id: FR-004-AC-3
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: The four measured clause classifications form a total partition; an implication-bearing
    clause is exercised only when every expected consequent entry probe is observed,
    and implication-free clauses require observed evaluation.
- id: FR-004-AC-4
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Successful analysis verifies the bound population, source/map, and native
    execution bindings.
- id: FR-004-AC-5
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Malformed, summary-only, identity-mismatched, path-ambiguous, unsupported,
    or missing observation inputs retain structured non-success outcomes without invented
    classifications.
- id: FR-004-AC-6
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Campaign counts and test outcome remain complete facts independent of coverage
    classification.
- id: FR-004-AC-7
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Removing an expected clause, consequent, evaluation probe, or all clauses
    prevents successful analysis; the expected census comes from bound typed IR.
- id: FR-004-AC-8
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Adverse coverage and non-success native execution cannot discharge the
    coverage obligation merely because a report serialized successfully.
- id: FR-004-AC-9
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Complete bound observations without a native campaign run are reported
    as observations only and never as a passed coverage result. Valid informational-only
    populations retain their references as no executable work.
- id: FR-004-AC-10
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED. CG binds coverage inside the same live campaign-runner invocation,
    using the original `/2` wire bytes and their QSL strict-reader typed result directly,
    the producer execution record, and the same generated source/map buffers used
    by that campaign. The runner starts the campaign with fresh private profile output,
    performs any required LLVM merge over only that output, starts LLVM export over
    only that profile, and captures exact export bytes and tool/version from that
    LLVM child's direct output stream while retaining the linked child handles. Binding
    verifies this live campaign/profil
- id: FR-004-AC-11
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED. The consuming report retains vacuity and native outcome as evidence
    for IR FR-045; an oracle-success run with an unobserved consequent denies the
    coverage obligation, and LLVM probe observations alone produce no completed-proof
    credit or separate eligible denominator.
- id: FR-004-AC-12
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: 'PLANNED. CG consumes QSL''s typed strict-reader clause-run or command-error
    variant for exact `native-run-result/2`: a decisive basis retains its witness
    with assigned members, while `closed-scope` or `unavailable` retains none. Missing
    or other versions, missing/unknown basis, malformed witness, contradictory basis/witness
    and mixed/invalid variant shape are located wire refusals before binding; no `/1`
    fallback or CG copy of the wire reader exists.'
- id: FR-004-AC-13
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED. QSL `run_clause` success and a decisive witness remain source-clause
    facts and do not count as generated-campaign execution or LLVM coverage; QSL violation,
    refusal, undefined, incomplete, cancelled, unsupported and internal failure remain
    distinct typed adverse/non-success facts. A bound live run does not upgrade any
    QSL disposition or itself grant campaign success, measured classification, proof
    credit or coverage discharge. The same reader's command-error variant retains
    stage/code/optional cause/message/details and unavailable basis as non-success,
    with no campaign qualification o
- id: FR-004-AC-14
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: 'PLANNED. Only CG''s campaign runner constructs the private, non-cloneable,
    non-serializable live run context; callers cannot construct or restore it from
    a `/2` document, paths, QSL `package_id`, metadata, or a forwarded report. Absent,
    foreign or expired context, or an omitted association yields structured binding
    non-success retaining the typed QSL outcome and available identities, with no
    qualification, classification or discharge. A complete same-invocation context
    admits binding without changing the QSL outcome. The caller supplies maximum encoded-input
    and decoded-working-byte limits; CG '
- id: FR-004-AC-15
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED. A controlled attempt to replace the strict-reader typed result,
    execution record, generated source, source map, LLVM export stream or LLVM tool/version
    with a foreign-run value cannot construct a valid same-invocation context, even
    when paths, QSL `package_id`, source/map and tool/version declarations match.
    In particular, foreign export bytes offered through a detached buffer or a different
    producer stream/child handle refuse as `producer_run_mismatch` before classification
    because neither is the current runner-owned campaign/profile/export chain. A malformed
    association observable i
- id: FR-004-AC-16
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). The expected clause population and implication counts
    of the V2 analyzer equal those independently derived from the admitted `CheckedPackageV2`
    clause bodies.
- id: FR-004-AC-17
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Source inspection shows the V2 analyzer opens a `CheckedPackageV2`
    body only through the `core/ir/` layer of AD-004, and names no `BoundPackage`
    or `ClauseRef`, both of which today's `analyze_bound_coverage` names.
- id: FR-004-AC-18
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). A V2 clause whose generated source map declares an implication
    count unequal to the count derived from its body yields a structured binding non-success
    naming the clause and no classification, and is never treated as implication-free.
- id: FR-004-AC-19
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, a clause whose oracle evaluation
    was observed, that has at least one implication and for which no expected consequent
    probe was observed is `vacuous`.
- id: FR-004-AC-20
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, a mapped clause whose oracle-evaluation
    region has no positive execution count is `unexecuted`.
- id: FR-004-AC-21
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, an implication-bearing clause
    for which some but not all expected consequent probes were observed is `partially_exercised`.
- id: FR-004-AC-22
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, an implication-bearing clause
    with every expected consequent probe observed, and an implication-free clause
    with its evaluation region observed, is `exercised`.
- id: FR-004-AC-23
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, removing one clause, one
    consequent probe, one evaluation probe, or every clause from the generated artifacts,
    each alone, prevents successful analysis.
- id: FR-004-AC-24
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, each malformed input class
    of FR-004-AC-5 yields a non-success outcome with no invented classification.
- id: FR-004-AC-25
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: 'SUPERSEDED by FR-004-AC-26 under QSpec #449 / IR-1070; no implementation
    credit. Original planned text: For any executable clause in the V2 analyzer''s
    expected population selected for strategy generation, the generator uses the accepted
    QSL point returned by Contract IR FR-058''s typed accessor for the admitted package,
    checked clause id and authentic `claim` `CheckedOccurrence`. Two authentic occurrences
    on one node with different accepted points retain their distinct rendered points
    when selected separately. A wrong-role, wrong-ordinal or other-node occurrence,
    an authentic tuple with no acce'
- id: FR-004-AC-26
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: 'PLANNED (IR-1070). For each selected executable V2 clause, the generator
    uses the exact accepted point of that point-qualified clause node and authentic
    claim through IR FR-058; separate nodes with equal condition and anchor but different
    points produce distinct rendered points when selected separately, while two authentic
    claims of one pointed node produce the same point. The analyzer retains one expected
    population entry per executable clause node and grants no campaign evidence for
    a refused selection. Wrong-role, wrong-ordinal, other-node, absent-point-node,
    and renderer-mutant selections '
- id: FR-036-AC-1
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: 'A genuine typed fixture with criteria C1/C2, obligations O1/O2/O3, source
    identities S1/S2, domain D and two production functions in one module exports
    three obligation inventory entries and exactly four candidate keys for authored
    tuples (C1,O1,S1,D), (C2,O1,S1,D), (C1,O2,S1,D), (C1,O2,S2,D). O3 has no authored
    criterion correspondence: it remains an inventory/gap record with no candidate.
    Missing production correspondence and authored no-function remain distinct gaps
    on complete candidate keys; a zero-mapping source symbol remains an inventory/gap
    record. Removing only C2''s relation leaves C'
- id: FR-036-AC-2
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: For the unchanged AC-1 fixture, omitting a result, reporting unsupported
    or pre-run refusal, retrying and changing method/profile/subject/artifact/run
    leave the complete obligation inventory, all four criterion-obligation-source-domain
    keys and mapping-gap records unchanged; enumeration precedes producer invocation.
    Removing O1 and its authored relations from the authoritative source removes its
    inventory entry and exactly the two O1 candidate tuples without editing an auxiliary
    catalog.
- id: FR-036-AC-3
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: Capability facts preserve unsupported capability and reason without artifact/run/outcome;
    pre-run refusal facts preserve producer/stage/cause separately from actual execution
    outcomes. An absent decision or observation and an unfamiliar producer state remain
    distinct, and missing, counterexample, invalid/incomplete, unavailable, timeout,
    exhaustion, cancellation and inconclusive evidence states survive export.
- id: FR-036-AC-4
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: A matching executed production fact retains its actual typed artifact and
    run identities separately from attempt identity, exact source/profile/subject/domain/bounds
    and explicitly bound load-bearing check plus production reachability. Wrong, missing
    and mismatched artifact/run identity controls preserve the received typed identity
    or explicit missing reason for IR joining, without inventing or repairing identities.
    Zero-check, check-location-only, tag-only, LLVM-only and incidental-reachability
    controls never acquire positive observations by export.
- id: FR-036-AC-5
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: Shadow-only evidence, a narrower effective bound and separately executed
    sampled native refinement retain distinct subject/scope/assumption observations.
    Missing refinement stays missing; adding a refinement observation does not change
    the census or create an unqualified-production-proof flag.
- id: FR-036-AC-6
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: A downstream consumer above CG and IR borrows the public census/fact types
    over the reviewed neutral correspondence contract without source/transcript/name
    parsing or a copied schema. Duplicate and artifact/run/source/profile/subject/domain-mismatched
    result controls survive as separate inspectable records without changing inventories
    or candidate tuples; repeated identical inputs produce identical ordering.
- id: FR-036-AC-7
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: Missing/incomplete authoritative inputs, a relation targeting a foreign
    selection and a dangling relation endpoint produce distinct typed census refusals
    with their offending input/relation and no complete census. A valid absent correspondence
    remains an explicit obligation/criterion/source-symbol gap, or a production gap
    on an existing complete candidate key, instead of that global refusal.
- id: FR-015-AC-1
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Pre, post, invariant and frame obligations of one contract produce separate
    harnesses with distinct identities.
- id: FR-015-AC-2
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'OPEN after the 2026-10-27 IR-348 V1 retirement: the only tagged `BoundClause`
    test retires; surviving V2 behavior and replacement evidence remain required.
    Original criterion retained: Every harness identity records its model-domain bounds,
    taken only from IR `bounded_domain` nodes, and its solver and options and unwind
    bound.'
- id: FR-015-AC-3
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An unbounded or non-finite obligation is refused with a typed reason and
    no harness.
- id: FR-015-AC-4
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: No harness assumption excludes an undefined, refused or incomplete runtime
    outcome.
- id: FR-015-AC-5
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An obligation whose bounds are unsatisfiable is refused with a typed reason
    and no harness.
- id: FR-015-AC-6
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An obligation over an oracle whose operation identity is `caller_declared`
    is refused with a typed reason and no harness.
- id: FR-015-AC-7
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'OPEN for surviving harness families; the V1 bundle portion is retired
    by the 2026-10-27 owner ruling on IR-348. Original criterion retained: Every generated
    harness contains exactly one non-vacuity cover; a precondition harness covers
    that the precondition holds within the IR bounds, and a contract harness''s cover
    stands after the contract call, so a run that satisfies every check without satisfying
    the cover is not a proof. The cover is the last statement of the harness, after
    every assertion the harness carries: the state-clause and frame-effect harnesses
    and the scalar harness place it afte'
- id: FR-015-AC-8
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'OPEN after the 2026-10-27 IR-348 V1 retirement: the only tagged `BoundClause`
    test retires; surviving V2 behavior and replacement evidence remain required.
    Original criterion retained: A postcondition or invariant harness emits every
    package precondition sharing its anchor operation as a `requires` on its generated
    contract and records each in its identity''s embedded oracles; it embeds no other
    obligation''s oracle; and an obligation whose sibling precondition is not a supported
    item of the same request is refused with a typed reason naming that precondition
    and no harness.'
- id: FR-015-AC-9
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: "OPEN after the 2026-10-27 IR-348 V1 retirement: the only tagged `BoundClause`\
    \ test retires; surviving V2 behavior and replacement evidence remain required.\
    \ Original criterion retained: Every harness identity records solver `cadical`\
    \ and the complete ordered option vector \u2014 function contracts, concrete playback,\
    \ the exact fully qualified harness, `--exact`, the explicit unwind, the explicit\
    \ solver, `--output-format regular`, and `--concrete-playback print` \u2014 and\
    \ no option enabling stubbing is emitted."
- id: FR-015-AC-10
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'OPEN after the 2026-10-27 IR-348 V1 retirement: the only tagged `BoundClause`
    test retires; surviving V2 behavior and replacement evidence remain required.
    Original criterion retained: Regeneration from equal inputs is byte-identical,
    and changing the unwind bound or the customer subject changes the harness identity.'
- id: FR-015-AC-11
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'OPEN after the 2026-10-27 IR-348 V1 retirement: the only tagged `BoundClause`
    test retires; surviving V2 behavior and replacement evidence remain required.
    Original criterion retained: Every symbolic argument carries an inclusive assumption
    equal to its IR `bounded_domain` or a literal operand''s own value (FR-015-AC-16);
    for a V2 scalar claim, an operand with the FR-014 inferred application interval
    uses that interval (FR-015-AC-103). A bounded-integer post-state result is required
    to lie in the same domain; no generated source carries a `#[kani::unwind]`.'
- id: FR-015-AC-12
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'OPEN after the 2026-10-27 IR-348 V1 retirement: the only tagged `BoundClause`
    test retires; surviving V2 behavior and replacement evidence remain required.
    Original criterion retained: A request naming no items, more than 256 items, an
    unparsable subject path, or an unwind bound outside `1..=1024` is refused whole,
    with no item accounted and no harness exposed.'
- id: FR-015-AC-13
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'OPEN after the 2026-10-27 IR-348 V1 retirement: the only tagged `BoundClause`
    test retires; surviving V2 behavior and replacement evidence remain required.
    Original criterion retained: An otherwise-supported obligation whose generated
    harness source exceeds the bounded-resource ceiling is refused with a distinct
    resource-limit reason naming the generated size, and one that fits the ceiling
    but fails to parse as Rust is refused with a distinct syntax reason naming the
    parse error; neither is reported as the internal-invariant render-assembly fallback.'
- id: FR-015-AC-14
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A scalar-claim item whose graph node carries a tag/form pair this generator
    does not model as a contract role is refused with a typed reason naming that tag
    and form, and no harness is emitted; a claim naming a node id absent from the
    graph entirely is refused rather than accounted as supported with no contract
    role recorded.
- id: FR-015-AC-15
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A scalar-claim item whose claim is a `NoDerivableClaim` refusal yields
    `unsupported` with the `no_derivable_claim` reason naming the node and the derivation
    refusal, emits no harness, and does not reject the group.
- id: FR-015-AC-16
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A scalar harness constrains a literal operand, inline or a reference to
    a `value` node whose body is a literal, to exactly its own value: `x + 1` over
    `x: Int[0, 9]` into `Int[0, 10]` has arguments `[0, 9]` and `[1, 1]`.'
- id: FR-015-AC-17
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A scalar claim whose literal operand does not fit `i64` (`x + 10^23`) is
    `unsupported` as `domain_not_representable_in_i64` naming the literal as both
    endpoints, with no harness.
- id: FR-015-AC-18
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A scalar claim whose operation over its operand ranges yields no result
    inside the result range (`x + 100` over `x: Int[0, 9]` into `Int[0, 10]`, reachable
    `[100, 109]`) is `unsupported` as `result_bound_unreachable` naming both ranges,
    with no harness.'
- id: FR-015-AC-19
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'OPEN: V1 contract-arm evidence retires under the 2026-10-27 owner ruling
    on IR-348; a supported V2 transition control remains to be measured. Original
    criterion retained: A zero-input transition over one bounded-integer state value
    verifies for an identity subject and is falsified with a concrete counterexample
    for a subject that changes the value.'
- id: FR-015-AC-20
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'OPEN: V1 clause evidence retires under the 2026-10-27 owner ruling on
    IR-348; V2 clause behavior remains planned under AC-40. Original criterion retained:
    A harness for a Boolean-connective or bounded-integer comparison claim (`and`,
    `or`, `not`, `implies`, `eq`, `ne` and the six integer comparisons) embeds its
    FR-014 oracle byte-identical to the oracle crate''s function.'
- id: FR-015-AC-21
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A precondition that reads a post-state value is refused with a typed reason
    naming the node, and no harness is emitted.
- id: FR-015-AC-22
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A declared proof-dependency census with an empty or duplicate identity,
    an inconsistent kind, state and path combination, or a non-`Required` kind is
    refused as a typed invalid input with no harness.
- id: FR-015-AC-23
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Every requested item receives exactly one `ObligationDisposition` (`supported`,
    `requires-bound`, `unsupported` or `invalid-request`), and an item that is not
    `supported` keeps its source identity and typed reason with no harness and no
    assumption emitted for it.
- id: FR-015-AC-24
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'OPEN: V1 comparison-claim evidence retires under the 2026-10-27 owner
    ruling on IR-348; supported V2 playback remains to be measured. Original criterion
    retained: A falsifying in-domain assignment of a plain bounded-integer comparison
    claim is reported through Kani''s concrete playback.'
- id: FR-015-AC-25
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A valid declared census is folded into the harness identity, readiness
    is `ready` only when every dependency passed and `incomplete` while any is missing
    or failed, and proof execution is recorded `not_run`.
- id: FR-015-AC-26
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: One `postcondition` `state_clause` of an admitted package, whose condition
    is one integer comparison of the pre and post reads of one field through its first
    parameter `self`, yields a separate operation-contract harness and frame-effect
    harness for the operation its anchor names. Each has exactly one non-vacuity cover,
    and its identity is scoped to the operation, anchor, frame and framed object;
    the two harnesses of two clauses of one operation are written to different paths.
- id: FR-015-AC-27
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The operation-contract harness assumes each ranged state field within the
    inclusive `i64` endpoints returned by `model_object_fields` for the QSL-emitted
    selected model declaration (FR-015-AC-77). It draws a present non-clause field
    with no usable range without an assumption (FR-015-AC-78), asserts the clause's
    comparison between pre-state and post-state, and refuses a clause whose own field
    has no usable range. A non-model object or admitted unselected model/object_type
    body is refused rather than used as a range source (FR-015-AC-78). PLANNED (IR-624).
- id: FR-015-AC-28
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The frame-effect harness takes the granted fields from the frame node's
    `modifies` entries and asserts every other state field the caller names unchanged.
- id: FR-015-AC-29
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A state clause that is not a postcondition, whose condition is not one
    comparison of the pre and post reads of one field through `self` (a literal operand,
    a negation, two fields, two reads of one side, or a read through another parameter),
    whose frame creates, deletes or grants a relationship or a foreign field, whose
    frame grants every state field, whose field the caller's state lacks, or whose
    clause field declares no integer range, is refused with a typed reason and no
    harness; a malformed request is refused first.
- id: FR-015-AC-30
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: With the installed backend, the operation contract verifies for a healthy
    subject and is falsified, naming the postcondition, for a subject mutated to debit.
- id: FR-015-AC-31
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: With the installed backend, a frame-allowed effect verifies, a frame-forbidden
    effect is falsified naming the forbidden field, and regenerating the frame from
    a package whose `modifies` is mutated to grant nothing falsifies the allowed subject.
- id: FR-015-AC-32
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The forbidden frame counterexample, executed natively, reproduces as a
    frame violation of that field through QSL's `replay_frame`, and the allowed run
    replays as a respected frame.
- id: FR-015-AC-33
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The frame-replay payload's anchor, frame and frame occurrence are the values
    `qsl_replay::call_site` names for the operation's name.
- id: FR-015-AC-34
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The frame-replay envelope's `clause_node` is the payload's frame node and
    its `occurrence_key` is the payload's frame occurrence.
- id: FR-015-AC-35
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An operation the unit names no frame for, whether the domain package declares
    it or not, is refused by the call site when the frame-replay request is built.
- id: FR-015-AC-36
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '`FrameReplay::replay` returns QSL''s `replay_frame` result: a forbidden
    write settles a reproduced violation naming the written field, and a write the
    frame grants settles `inconclusive` with no frame witness.'
- id: FR-015-AC-37
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A scalar function-application harness asserts the oracle''s outcome against
    the clause''s operation evaluated natively in `i128` over the same symbolic operands,
    independently of the oracle: `Completed` with exactly that value when it lies
    in the result bound, `Refused` when it does not, and any other outcome fails.
    With the installed backend, the healthy oracle verifies and an oracle whose arithmetic
    is mutated is falsified on that assertion.'
- id: FR-015-AC-38
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 clause claim naming a `state`/`state_clause` node of an admitted `CheckedPackageV2`
    whose `operation.member.clause` is `precondition`, `postcondition` or `invariant`
    yields one harness of that obligation kind, separate from the harness of every
    other clause of the package; the kind is read from the node and no caller value
    changes it. For a collection-valued operation clause, the one operation-clause
    harness carries a generated `CgObjectState` with every admitted model field as
    an exact `Value`, the selected one being `Value::Collection`; the customer subject
    receives one mutable exact-va
- id: FR-015-AC-39
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A V2 clause harness draws one nondeterministic value per drawn input:
    the pre-state of each state field of `self` the body reads, whether bare or under
    `pre(...)` (the subject''s input state; how it reaches the subject is FR-025''s),
    and each operation parameter (for a precondition or invariant, each bare state
    field read, since the state is the one judged). Each carries an inclusive assumption
    equal to its declared domain: the accessor''s `IntRange` values for a selected
    model declaration state field (FR-015-AC-77), and the IR `bounded_domain` of a
    parameter''s type. A non-model or admitted unsel'
- id: FR-015-AC-40
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A harness for a V2 clause whose condition, inline or referenced from an
    admitted Boolean expression node, contains Boolean connectives and bounded-integer
    comparisons (`and`, `or`, `not`, `implies`, `eq`, `ne` and the six integer comparisons)
    embeds an FR-014 oracle of that condition byte-identical to the oracle crate's
    clause-identified function, as FR-015-AC-20 states for a scalar claim; the clause-identified
    oracle needs FR-014-AC-38. PLANNED (IR-489).
- id: FR-015-AC-41
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A V2 clause whose body holds integer arithmetic (add, subtract, multiply,
    negate) as an operand of a comparison lowers, and its harness keeps the independent
    native assertion of FR-015-AC-37 for each arithmetic subterm (the oracle''s outcome
    against the operation evaluated natively in `i128` over the same operands, `Refused`
    assumed away nowhere): a postcondition `amount < 1000 implies amount + 1 <= 1000`
    over `amount: Int[0, 1000]` yields one contract harness. With the installed backend
    the unmutated clause verifies with its cover satisfied. In separate unconstrained
    installed-Kani runs of tha'
- id: FR-015-AC-42
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 clause body holding an operator the V2 arm does not support (division,
    modulo, absolute value, and every operator outside connectives, the six integer
    comparisons and add, subtract, multiply, negate; those operators are planned scope
    for a later criterion) is `unsupported`, with a typed reason naming the node and
    the operator and no harness; a node that is not a modelled clause role, or a node
    id absent from the package, is refused as FR-015-AC-14 states; no item is skipped
    silently, and a supported item of the same request is unaffected. PLANNED (IR-489).
- id: FR-015-AC-43
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 postcondition harness emits every `precondition` clause node sharing
    its anchor node as a `requires` and records each in its identity's embedded oracles,
    embeds no other obligation's oracle, and a postcondition whose sibling precondition
    is not a supported item of the same request is refused naming that precondition
    with no harness; a V2 invariant is anchored at an object type, binds `self` alone,
    and so has no operation precondition to assume. For a collection postcondition,
    each supported sibling precondition evaluates on the same complete typed prestate
    and operation parameters that re
- id: FR-015-AC-44
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A census declared on a V2 clause claim with an empty or duplicate dependency
    identity, an inconsistent kind, state and path combination, or a non-`Required`
    kind is refused as a typed invalid input with no harness, by the same rules as
    FR-015-AC-22; the census request and readiness types it uses are not `ProofDependencyGraph`.
    Implemented for the supported V2 clause path (IR-1044); the tagged TC-025 test
    exercises the public request with an admitted QSL precondition and each listed
    refusal.
- id: FR-015-AC-45
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A valid census declared on a V2 clause claim is folded into that harness's
    identity record, readiness is `ready` only when every dependency passed and `incomplete`
    while any is missing or failed, and proof execution is recorded `not_run`, by
    the same rules as FR-015-AC-25; in addition, the folded census is independent
    of the order the caller lists its dependencies. Implemented for the supported
    V2 clause path (IR-1044); the tagged TC-025 test checks the in-memory and persisted
    identity for passed, missing, failed and reordered declarations. This is generation-time
    evidence, not a backend verdi
- id: FR-015-AC-46
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Every V2 clause harness contains exactly one non-vacuity cover, placed
    after every assumption, after the subject call and after every assertion the harness
    carries (the last statement, as FR-015-AC-7 states), that states the family''s
    reachability, as FR-015-AC-7 does: a precondition harness covers that the precondition
    holds within the bounds; a postcondition harness covers, after the subject call
    returns, that the requires and the bounds are jointly satisfiable; an invariant
    harness covers that the drawn state satisfies its domain assumptions. The subject
    call is the call of the customer subj'
- id: FR-015-AC-47
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Regeneration of a V2 clause harness from equal inputs is byte-identical,
    and changing the unwind bound or the customer subject changes the harness identity
    record (as FR-015-AC-10 states). PLANNED (IR-489).
- id: FR-015-AC-48
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The obligation identity of a V2 clause claim is formed by AD-003 E-1 from
    the clause node id, its occurrence key, the obligation kind, and the arguments
    each as parameter node id and declared domain for an operation parameter, as E-1
    states (the domain FR-015-AC-39 defines). A state field read is named by its declaring
    node id and field name, never by a node of its own; that E-1 lists parameters
    only is an open question for the AD-003 owner and E-1 is not widened here. The
    source span is excluded; changing any included member changes it, and changing
    the span, the unwind bound or the subject d
- id: FR-015-AC-49
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 invariant claim yields a harness that draws its state and parameters
    within their declared domains (FR-015-AC-39) and asserts the invariant clause;
    its subject is the clause itself, with no subject call. Preservation of the invariant
    under an operation is not specified by this criterion. PLANNED (IR-489).
- id: FR-015-AC-50
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '`negotiate` reports an obligation whose scalar refusal is `ExactScalarRefusal::LoweringByteLimitExceeded`
    or `LoweringLimitUnrecognised` as `Outcome::Unsupported(UnsupportedObligation::OracleRefused
    { refusal })` with `refusal` equal to the scalar refusal, field for field, and
    neither is mapped to another `UnsupportedObligation` variant nor to `LoweringWorkExhausted`.'
- id: FR-015-AC-51
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corpus path, a finite input validated under a profile selection
    other than the offered profile's is refused as a typed `InvalidInput` `kani_profile_input_mismatch`
    result naming the request's source id, with no artifact and no case identity recorded.
- id: FR-015-AC-52
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corpus path, the context of every successful `BoundedCorpusExpectation`
    and typed refusal that path returns (`kani_corpus_dependency_invalid`, `kani_corpus_identity_collision`,
    a lowering refusal and `kani_profile_input_mismatch`) is the profile selection's
    revision. The expectation is not a Kani run verdict.
- id: FR-015-AC-53
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'OPEN for surviving V2 precondition, scalar, state-clause, frame-effect
    and corpus harnesses; the V1 contract and bundle portions retire under the 2026-10-27
    owner ruling on IR-348. Original criterion retained: Each of the seven harness
    kinds the generator emits (precondition, V1 contract, scalar, state-clause, frame-effect,
    V1 bundle and corpus) is subject to the cover-last rule of FR-015-AC-7: exactly
    one `kani::cover!`, the last statement of the body. What the cover witnesses,
    by kind: a precondition harness, that the precondition holds within the IR bounds
    (it asserts no universal truth of '
- id: FR-015-AC-54
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'RETIRED (2026-10-27 owner ruling, IR-348; V1 bundle-only, original criterion
    retained): A V1 bundle harness (`generate_kani_bundle`, the `proof_for_contract`
    harness it emits for a requires clause, an ensures clause and the bundle''s bounded
    arguments) ends with exactly one `kani::cover!` after the call of its contract,
    which witnesses that the requires clause and the argument and result bounds are
    jointly satisfiable, so a run whose requires clause no bounded argument satisfies
    never reaches it; the bundle''s `ensures` is checked at the contract call, so
    the cover follows every check the harnes'
- id: FR-015-AC-55
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: While a family's corpus arm remains (deleted by FR-015-AC-109 for arithmetic,
    FR-015-AC-110 for collection, FR-015-AC-111 for graph), a bounded-corpus harness
    of that family ends with exactly one `kani::cover!` after its `assert!` of the
    case's oracle, which witnesses that the harness runs to its end past that assertion.
- id: FR-015-AC-56
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'RETIRED (2026-10-27 owner ruling, IR-348; V1 bundle-only, original criterion
    retained): With the installed backend, a V1 bundle harness whose requires clause
    some bounded argument satisfies and whose `ensures` holds for every such argument
    classifies `Verified` (FR-017-AC-4); the same bundle with a requires clause no
    bounded argument satisfies classifies `CoverUnsatisfied` with its satisfied and
    total cover counts, never `Verified` and never `Falsified`.'
- id: FR-015-AC-57
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: While its arm remains (FR-015-AC-109 to FR-015-AC-111), with the installed
    backend, a bounded-corpus harness of each of the arithmetic, graph and collection
    families whose oracle is true classifies `Verified`, and a graph or collection
    harness whose oracle is false (an arithmetic case's oracle is always true) classifies
    `Falsified` carrying the assertion's playback, which until FR-015-AC-111 deletes
    the corpus replay is replayed through the corpus case's QSL source (`tc_023_real_corpus_playback_replays_through_its_qsl_source`),
    never `Inconclusive` for lack of a counterexample.
- id: FR-015-AC-58
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'OPEN for surviving emitting entries; V1 contract and bundle entries retire
    under the 2026-10-27 owner ruling on IR-348. Original criterion retained: A test
    generates a harness through every emitting entry point (the precondition, V1 contract,
    scalar, state-clause and frame-effect families, `generate_kani_bundle`, and the
    corpus generator for each of its families whose arm remains, per FR-015-AC-109
    to FR-015-AC-111), parses each emitted source, and fails for any function attributed
    `#[kani::proof]` or `#[kani::proof_for_contract]` whose body does not contain
    exactly one `kani::cover!`, which m'
- id: FR-015-AC-59
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A request of `StateFrame` items in which the first item is refused and
    later items are supported, requires-bound, unsupported or invalid yields exactly
    one `ObligationRecord` per item in request order, `kind` `postcondition` for a
    `contract` item and `frame` for a `frame` item; and each later item's record (disposition
    and reason) equals the record the same item has in a request holding that item
    only; a request of three refused items yields three records. The requires-bound
    item comes from a lowering `RequiresBound` record in that same admitted package,
    not an object-body member target; remov
- id: FR-015-AC-60
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A `StateFrame` item that yields is recorded `supported` carrying its harness
    symbol, and its harness, returned in the `state_frame_harnesses` field of `KaniObligationOutcome::Emitted`
    in request order, is byte-identical to the harness `generate_state_frame_role`
    returns for the same clause and role, and for a clause both of whose roles succeed
    to the corresponding harness `generate_state_frame_obligations` returns. COVERED
    (IR-461): a crate-internal test uses an admitted V2 package to compare direct
    role generation with each negotiated supported item, its symbol and exact harness
    bytes, and th'
- id: FR-015-AC-61
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A `StateFrame` item whose lowering is a requires-bound record is recorded
    `requires_bound` with that record's `unbounded_type` and no harness. A selected
    model declaration's clause read of a native `Integer` that makes lowering return
    `RequiresBound` keeps that lowering refusal; it is not remapped to value-based
    `ModelMemberNotI64Range`. The latter applies to a present listed `Integer` field
    only if range resolution is reached, such as an unread field. A non-model or admitted
    unselected model/object_type body is refused as `ModelFieldsUnavailable`, without
    inventing an `unbounded_type` node or
- id: FR-015-AC-62
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A `StateFrame` item whose lowering is an unsupported-family record is recorded
    `unsupported` with reason `NoFiniteEncoding` naming the record's node and family,
    and an item whose node is not a `state_clause` is recorded `unsupported` with
    `UnknownNodeKind`, each with no harness.
- id: FR-015-AC-63
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A `StateFrame` item whose condition is a negation, compares to a literal,
    uses an operator other than the six integer comparisons, reads through a parameter
    other than `self`, compares two fields or two reads of one side, or whose frame
    creates or deletes an object or grants a relationship or foreign field (naming
    the effect), is recorded `unsupported` with `StateFrameRefused` and no harness.
    The same result holds for a clause that is not a postcondition, is malformed,
    whose frame grants every field, whose lowering is an invalid-body, incomplete
    or over-budget record, or whose object is non-mo
- id: FR-015-AC-64
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A `StateFrame` item with an unparsable state path or an unparsable item
    subject path, an invalid or lacking state field, an absent node, a repeat of an
    earlier item or a package other than the first item's is recorded `invalid_request`
    (`InvalidStatePath`, `InvalidStateField`, `UnknownNode`, `DuplicateItem`, `MixedStatePackages`),
    and the outcome is `Rejected` with every item's record and no harness bytes.
- id: FR-015-AC-65
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A request holding `StateFrame` items and no items, more than `MAX_OBLIGATION_ITEMS`,
    an unparsable request subject path or an unwind bound outside `1..=MAX_OBLIGATION_UNWIND`
    is refused whole with the existing `KaniObligationError` variant and no record.
- id: FR-015-AC-66
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A test builds every retained `StateFrameRefusal` variant (including `ModelFieldsUnavailable`,
    `MemberAbsent`, `ModelMemberNotI64Range` with each cause, and each of the six
    refusal arms of `NotLowered`) and calls the mapping on it; each yields the disposition
    and reason of the table. It does not construct retired body-member `BoundNotResolvedCause`
    values. The variants a request reaches only with a defective generator or a source
    over 1 MiB (`UnwindOutOfRange`, `InvalidGeneratedSyntax`, `RecordSerialization`)
    are built directly, and `ResourceLimitExceeded` is built directly or reached with
    a su
- id: FR-015-AC-67
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '`generate_state_frame_obligations` refuses with `MalformedClause` at the
    node, not `InvalidField` or `ConditionNotSupported`, for a clause whose graph
    field name is not a Rust identifier; a field name the caller supplies that is
    not an identifier is still `InvalidField`.'
- id: FR-015-AC-68
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A `StateFrame` request whose frame grants every state field records its
    `frame` item `unsupported` (`StateFrameRefused`) and its `contract` item `supported`
    with the harness `generate_state_frame_role` returns for the `contract` role;
    a request whose condition is a negation records its `contract` item `unsupported`
    and its `frame` item `supported` with the harness `generate_state_frame_role`
    returns for the `frame` role; in both, `generate_state_frame_obligations` returns
    the refusal of the failing role; a request whose lowering is refused records both
    items alike. COVERED (IR-461): a crate-in'
- id: FR-015-AC-69
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A composite-equality claim over operands that are parameters or literals
    of one record, tuple or option type whose leaves are Boolean or bounded integers,
    with the descriptor FR-018 generated, yields one harness whose identity records
    family `composite_equality`, proof subject `bounded_shadow` and a refinement obligation
    in the same result (FR-028-AC-7), separate from the harness of every other item;
    both operators (`Equal`, `NotEqual`) yield one; and the operator and operand types
    are read from the descriptor while no bound is. PLANNED (IR-264).
- id: FR-015-AC-70
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'The harness draws one nondeterministic value per leaf of each parameter
    operand and no other value: a `bool` per Boolean leaf, an `i64` per bounded-integer
    leaf assumed inside its `integer_range` (FR-015-AC-11), a presence `bool` per
    option and a payload only for a present option, and for a record field of optional
    presence one of `present`, `absent` or `null`; a literal operand''s leaves are
    constrained to exactly its value (FR-015-AC-16); the drawn arguments ascend by
    parameter node id then by leaf path, each bound to its node id and path (FR-025-AC-1,
    FR-025-AC-2), and no composite is itself'
- id: FR-015-AC-71
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Over every drawn pair the harness asserts that the shadow's verdict equals
    the expectation's and that the shadow's pair count equals the expectation's, where
    the expectation is computed from the drawn values by QSpec FR-149's rules (same
    declaration and pairwise-equal fields or positions in order; two `none` equal;
    `none` and present unequal; `absent` equals `absent`, `null` equals `null`, `absent`
    and `null` unequal; the pair count is the node count of the occurrence-pair tree
    with no early exit after an unequal pair), the verdict of `NotEqual` is the negation
    of `Equal`'s with the same count
- id: FR-015-AC-72
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The emitted shadow and harness, parsed, contain no loop (`while`, `loop`,
    `for`), no recursive call, no `Box`, `Vec`, `Rc`, `String` or map, and no integer
    type wider than `i64`; and a closure whose pair-node count (the node count of
    the largest occurrence-pair tree its type admits) exceeds `SHADOW_PAIR_NODE_BUDGET`
    is refused as `ShadowShapeOverBudget` naming the count and the budget, one at
    the budget generates, and the budget and the count are recorded in the identity
    and the evidence. The value of the budget is set by the measurement of the code
    change and recorded in TC-025's status. PLAN
- id: FR-015-AC-73
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Each row of the unsupported-shape table above is produced by an item reaching
    that shape and yields its disposition and reason with no harness: an unbounded
    integer leaf `requires_bound`; a bound outside `i64` `domain_not_representable_in_i64`;
    a recursive declaration `requires_bound` naming the declaration; one leaf of each
    family the table lists `ShadowFamilyNotBuilt` naming that family; a `reference`
    composite, as FR-018-AC-7; an operand that is neither a parameter nor a literal,
    and a `convert<T>` operand, `OperandNotSupported`; an over-budget closure `ShadowShapeOverBudget`;
    an item FR-01'
- id: FR-015-AC-74
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: With the installed backend, the harness of the corpus case (two record-typed
    and two tuple-typed values compared with `=` and `!=`, a record with an optional
    field and a nested option, Boolean and bounded-integer leaves) is classified `Verified`
    with its cover satisfied within the memory and wall-clock ceilings its identity
    records, and its execution evidence records the versions and options of FR-028-AC-23;
    the refinement run of the same case agrees on every case it runs, and the case's
    refinement domain is at most the case cap, so the run is classified `exhaustive`
    and records its case count
- id: FR-015-AC-75
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'With the installed backend each seeded mutant of the shadow is `Falsified`
    with a concrete playback: an `absent` slot read equal to a `null` slot; one field
    of the left operand compared with the next field of the right; the pair count
    omitting the payload pair of two present options; the count stopping after the
    first unequal field; the operator `NotEqual` rendered as `Equal`; and a field
    left out of the comparison. Run natively, each seeded mutant of the production
    oracle settles `refinement_failed` naming the first disagreeing case: the opposite
    operator emitted; an `absent` slot compared eq'
- id: FR-015-AC-76
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The identity of a composite-equality harness records family `composite_equality`,
    proof subject `bounded_shadow`, the closure's declaration keys each with its leaves'
    declared domains, the operator, the size budget and the closure's pair-node count,
    the abstractions `representation`, `integer_widening`, `absent_payload` and `unrolled_walk`
    each with its discharging evidence, and the behaviours the proof does not exercise
    (`meter_limits`, `ill_typed_operands`, `charge_amounts_other_than_pairs`, `foreign_reference`,
    `declaration_reconstruction`) each with the native criterion that covers it (FR-
- id: FR-015-AC-77
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. For the QSL-emitted twin package, `field_range` calls `model_object_fields`
    on the framed object''s selected model declaration and obtains both `balance`
    and unread `audit` with `IntRange { lower: 0, upper: 1000 }`; it converts their
    `i128` endpoints to `i64` and returns 0 to 1000 for each. A second admitted unit
    with the same declaration but no read of `audit` returns the same range for `audit`;
    IR FR-038-AC-140 independently shows a read''s range-node body does not change
    the accessor''s bound. The genera'
- id: FR-015-AC-78
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG implementation pending. `field_range`
    returns `MemberAbsent` for a requested name absent from the selected model declaration's
    accessor, including a clause's own field; an accessor `UnknownNode`, `NotModelObjectType`
    or `AmbiguousField(name)` returns `ModelFieldsUnavailable` with that error and
    object id, with no body or read fallback. A non-model object and an admitted unselected
    model/object_type with a nonempty body both yield `NotModelObjectType` and refuse
    even if body members claim valid ranges. A selected model/object_type whose nonempty
    body
- id: FR-015-AC-79
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. From the package QSL emits for the twin's unit, both roles of each clause
    (`BalanceNeverDrops`, `AuditNeverDrops`) are generated; each identity's `state_fields`
    equals the request's list in its order, its `domains` equal the accessor ranges
    of FR-015-AC-77, and its `scope.anchor` and `scope.frame` equal the ids `qsl_replay::call_site`
    names, asserted before replay. The accessor's ascending-name order does not reorder
    `state_fields`. A request naming a field absent from the accessor is refused;
    one omitti
- id: FR-015-AC-80
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. With the installed backend, the cases of FR-015-AC-30 and FR-015-AC-31
    run over harnesses generated from the package QSL emits: the healthy subject verifies,
    the subject mutated to debit is falsified naming the postcondition, a granted
    write verifies and a write to an ungranted field is falsified naming that field.'
- id: FR-015-AC-81
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. A present listed model field with `member_type()` of `None`, a non-`IntRange`
    variant, or `IntRange` outside `i64` is recorded as unranged with `TypeNotRange`
    in `StateFrameIdentity` and its persisted record, even when no read names it;
    a present unread `IntRange` within `i64` is recorded in `domains` and is absent
    from the unranged list. Equal inputs produce byte-identical records; a record
    naming a field twice is invalid. The model declaration path records no `NoRead`
    reason. QSL-emission gaps for `Non
- id: FR-015-AC-82
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-profile classifier, a request for
    distinct named constructs returns one `CapabilityEntry` per requested construct
    in request order, preserving each admitted `Supported` module or `Refused` cause
    code exactly. The Contract IR profile reader rejects the removed `inconclusive`
    disposition rather than admitting it; no profile disposition or profile-supplied
    code becomes an `Inconclusive` run outcome or asserts proof vacuity.
- id: FR-015-AC-83
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-profile classifier, an empty, duplicate
    or absent construct is a typed `KaniOutcome` refusal rather than a partial disposition
    census.
- id: FR-015-AC-84
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the arithmetic slice (FR-015-AC-109) deletes the finite-input arithmetic
    lowerer, a profile/dispatch-admitted add, subtract, multiply, divide or remainder
    returns the exact checked `i128` result only inside the request's inclusive result
    range.
- id: FR-015-AC-85
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the arithmetic slice (FR-015-AC-109) deletes the finite-input arithmetic
    lowerer, an inverted result range is `InvalidInput`; zero division, checked overflow
    and a result outside the range are typed refusals, with no arithmetic result or
    generated artifact.
- id: FR-015-AC-86
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the graph slice (FR-015-AC-111) deletes the finite-graph lowerer,
    a profile/dispatch-admitted request follows only the selected reference field,
    finds a positive-length path in sorted depth-first order, and checks the expansion
    bound before entering each new object; a target edge is found without expanding
    its target.
- id: FR-015-AC-87
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the graph slice (FR-015-AC-111) deletes the finite-graph lowerer,
    a zero expansion bound or unknown endpoint is `InvalidInput`, and exhausting the
    bound is `ResourceExhausted`, with no graph result or generated artifact.
- id: FR-015-AC-88
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the collection slice (FR-015-AC-110) deletes the finite-collection
    lowerer, a profile/dispatch-admitted `ForAllNonNegative` or `ExistsEqual` query
    retains input order and duplicates and returns its truth value and examined-item
    count; a decisive item ends that count, otherwise every item counts.
- id: FR-015-AC-89
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the collection slice (FR-015-AC-110) deletes the finite-collection
    lowerer, an input longer than `max_items` is `ResourceExhausted`, with no query
    result or generated artifact.
- id: FR-015-AC-90
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm
    ends at its own slice, FR-015-AC-109 to FR-015-AC-111), one admitted arithmetic,
    graph or collection case emits all four artifacts (oracle, finite strategy, Kani
    harness and proof-dependency graph) from the same selected profile, validated
    finite input and family lowering.
- id: FR-015-AC-91
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corpus path, that path returns a `BoundedCorpusExpectation`:
    the family lowering''s true Boolean oracle predicts that the one generated assertion
    holds with one expected SUCCESS check, and false predicts that it fails with zero
    expected SUCCESS checks. This value is neither an observed `KaniOutcome` nor a
    native replay verdict and cannot enter the FR-030 terminal mapper. This interim
    expectation ends when FR-015-AC-111 deletes the path.'
- id: FR-015-AC-92
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm
    ends at its own slice, FR-015-AC-109 to FR-015-AC-111), a declared dependency
    census with an empty or repeated identity, inconsistent kind/state/path or a kind
    other than `Required` refuses as typed `InvalidInput` with no artifact or case-identity
    claim; a valid census is normalized into the emitted proof graph.
- id: FR-015-AC-93
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corpus path, a profile/input mismatch, family-lowering
    refusal, proof-graph serialization refusal or duplicate case identity returns
    its typed refusal with the selected profile revision, emits no artifact and claims
    no new case identity.
- id: FR-015-AC-94
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm
    ends at its own slice, FR-015-AC-109 to FR-015-AC-111), equal case inputs produce
    equal artifact paths and contents regardless of emission order, and reordering
    the same finite graph's objects or references does not change them; changing a
    request field, selected profile, finite input, ceilings or normalized dependency
    census changes the case identity and artifact paths.
- id: FR-015-AC-95
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'AD-004 step 4g retires the hand-built bounded-corpus path in the three
    per-family slices FR-015-AC-109 (arithmetic), FR-015-AC-110 (collection) and FR-015-AC-111
    (graph, which deletes the whole public corpus API, its generation-time expectation
    and typed refusal outcome); this criterion is the end state they reach together
    and adds no deletion of its own. The end state is: QSL-emitted packages admitted
    through Contract IR feed the one V2 generator for arithmetic, collection and graph
    cases; the collection and graph cases satisfy FR-015-AC-96 to FR-015-AC-102, including
    typed model-derived symb'
- id: FR-015-AC-96
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An admitted QSL V2 invariant, precondition or postcondition clause, or
    function body, that contains a `quire.op.collection.forall` or `quire.op.collection.exists`
    operation over a selected model collection field is a collection claim. A constraint
    clause follows FR-015-AC-128; it is not a standalone proof obligation. An invariant
    or function-body collection claim uses the direct collection harness. A precondition
    or postcondition collection claim uses its FR-015-AC-38/39/43 operation-clause
    harness, with the same selected symbolic field domain and no second standalone
    collection proof. For a f
- id: FR-015-AC-97
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: For an admitted collection claim whose selected field has multiplicity
    lower bound `0` and no finite upper bound, QSL admits an unbounded collection
    type; when its element type is supported, this generator settles `requires_bound`
    naming the selected field or unbounded collection type and emits no harness. An
    admitted field with no finite element domain likewise settles `requires_bound`
    naming that element type and emits no harness. A field with multiplicity lower
    bound greater than `0` and no finite upper bound has no QSL kernel type ([QSL
    FR-056-AC-10](ix://agent-ix/quire-spec-language/FR-05
- id: FR-015-AC-98
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A graph claim (one reachability application of either form above, at its
    `state_clause` node id and argument path) generates a harness when the admitted
    package gives its covering object type `E` a finite population `[0, N]` through
    `select_population` and the selected edge has finite multiplicity (for `quire.op.model.reaches_field`
    the field's `Reference`, `Option` or bounded `Collection` member; for `quire.op.model.reaches`
    the destination end's finite multiplicity). Every symbolic object and edge is
    bounded inclusively by those declarations, including zero and maximum where admitted.
    Reacha
- id: FR-015-AC-99
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'For a function-body or invariant collection claim, the direct harness
    asserts the admitted body over symbolic inputs drawn from the selected model field''s
    typed domain and ends with the FR-015-AC-7 cover after its assertions. A precondition
    collection claim is marked as a collection quantifier in its typed V2 clause obligation
    identity and uses the same symbolic field draw in its FR-015-AC-38/39 clause harness:
    an assertion checks that evaluation completes a Boolean on every draw (both true
    and false are allowed), and a separate cover checks that some draw completes true.
    It neither assumes th'
- id: FR-015-AC-100
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: For the collection family, changing the selected field's authored cardinality
    or element bound, admitted body claim, customer subject or unwind changes the
    harness identity; equal admitted inputs regenerate byte-identically. The identity
    records every symbolic collection domain and the solver and complete ordered option
    vector, and FR-017 run evidence records the solver kind, unwind and every option
    used by the installed backend in invocation order. For the graph family (AC-98),
    changing the claim's clause id or argument path, the reachability form, relationship
    or field, its role or name, the
- id: FR-015-AC-101
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A request whose symbolic collection domain exceeds the generator's representational
    or resource ceiling settles `unsupported` with a typed reason naming the failing
    domain or ceiling and no harness; it never tightens the model domain to make a
    proof tractable. A missing selected model field is refused at QSL binding with
    its typed source-located reason, and a malformed selected projection is refused
    by Contract IR strict admission with its typed located cause; neither produces
    an admitted collection claim, CG item or harness. For an admitted claim whose
    checked selected declaration or field ca
- id: FR-015-AC-102
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'An admitted collection body claim is settled through the FR-015 V2 obligation
    path, one disposition per requested item; supported items alone emit harnesses.
    The finite-input lowerer and hand-built corpus of AC-86 to AC-95 neither supply
    the symbolic bounds nor classify the proof verdict. Graph items take the same
    V2 path (IR-805), one disposition per requested item, and are not implementation
    or proof evidence for IR-268. IMPLEMENTED for the collection family (IR-268, CG
    #396 fe20ecc): `tests/it/kani_obligations_symbolic_domains.rs` `collection_negotiation_accounts_for_each_supported_or_refus'
- id: FR-015-AC-103
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Through `generate_routed`, `(x + 1) + 1` over `x: Int[0, 9]` into `Int[0,
    20]`, `x * -1` over `x: Int[1, 9]` into `Int[-9, -1]`, and `-(x + 1)` over `x:
    Int[0, 9]` into `Int[-10, -1]` each settle `Supported` with an IR-confirmed outer
    oracle and a harness. Their respective argument ranges are `[1, 10]` and `[1,
    1]`, `[1, 9]` and `[-1, -1]`, and `[1, 10]`; the result assertions use only the
    outer declared result bounds. Each inferred range carries its source application
    id in the harness identity.'
- id: FR-015-AC-104
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A plain-Integer parameter as any nested leaf keeps `requires_bound` with
    the Integer `unbounded_type` and no harness; a direct reference to a raw application
    never borrows a separate narrowing conversion's domain, and an inferred endpoint
    outside `i64` settles `unsupported` as `domain_not_representable_in_i64` naming
    both exact endpoints. `OperandRangeLimitExceeded`, `OperandUnsupported`, `OperandTypeMismatch`
    and `InvalidBody` settle `unsupported` with `oracle_refused` carrying the exact
    scalar refusal, including its node, position or limit fields, and no harness.
    Refused items leave a suppor
- id: FR-015-AC-105
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'For a graph claim the harness draws an object count in `[0, N]` of the
    one covering population and symbolic objects within it; for every object, the
    selected edge as its shape states: a `Reference` field has exactly one target,
    an `Option` field an absent or one target, a `Collection` field or finite multi-valued
    relationship end a count in `[lower, upper]` (a unique end admits no repeated
    target, a non-unique end repeats, and `lower > 0`, such as `{2,3}`, forces at
    least `lower` targets) and every target is an object of the population. Start
    and target are drawn from that population and may b'
- id: FR-015-AC-106
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'The independent native oracle replays the same authored clause over each
    original-domain concrete graph through QSL''s replay (`qsl_replay`), and does
    not substitute a separately authored reachability. With the installed backend,
    a QSL-emitted, IR-admitted healthy case yields `Verified` with its cover satisfied
    for one field-edge claim and one relationship-end claim over a `{0,3}` unique
    and a `{0,3}` non-unique end. Each semantic mutant yields `Falsified` at the clause
    assertion with an original-domain playback and native replay of that same authored
    clause: reachability that allows a zero-len'
- id: FR-015-AC-107
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Graph claims are located by `state_clause` node id and argument path:
    one reachability written twice in one condition yields two items at positions
    0 and 1; one node under two clauses yields two items with different identities;
    a field-edge and a relationship-end reachability yield separate items; a collection
    claim, a clause with no reachability and a path that does not name a reachability
    application each settle `unsupported` naming the missing claim with no harness.
    A mixed request settles one disposition per item and a supported item''s harness
    is unchanged by a refused sibling. PLANNED (IR'
- id: FR-015-AC-108
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A graph claim settles by the disposition sequence in the Description (the
    paragraph "Disposition sequence for a graph claim"), with no harness, domain reduction
    or caller `max_expansions` substitute. While the population selection for the
    claim is `Unavailable` (no available population selection for this admitted claim),
    the claim settles `unsupported` naming it. Given a selection, `Unbounded` or `UnknownClosureOpenExtent`,
    that is a claim whose population has no finite authored bound (once a selection
    is available), settles `requires_bound` naming the population; an unbounded `Collection`
    edg
- id: FR-015-AC-109
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-598 arithmetic slice only; merged IR-858 satisfies AD-004
    step 4b). After this slice no source under `src/` contains the corpus `Arithmetic`
    request variant and arm, `BoundedCorpusFamily::DefinednessArithmetic`, `prepare_checked_arithmetic`,
    `checked_method`, `render_arithmetic_*` or `lower/definedness_arithmetic.rs` and
    their re-exports, and no test in `tests/` drives them. The slice deletes only
    what the arithmetic family owns: the corpus replay (`BoundedCorpusCase::replay_counterexample`,
    `compile_corpus_qsl_package`, `BoundedCorpusReplayError` and the replay-harness
    identit'
- id: FR-015-AC-110
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-598 collection slice; gated on IR-858, AD-004 step 4b, and
    on AD-004 step 4h, because the collection family renders through `HarnessSpec`
    and `render.rs` only after 4h). The gate opens when the AC-99 installed-backend
    controls are green, with their tagged tests present (`tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies`
    and the symbolic-domain tests tagged FR-015-AC-96 and FR-015-AC-99 in `tests/it/kani_obligations_symbolic_domains.rs`).
    Evidence for a real-Kani precondition is a recorded `make kani` or `make kani-gate`
    run on the slice PR's head that is green
- id: FR-015-AC-111
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-598 graph and final slice; gated on IR-858, AD-004 step 4b,
    on FR-015-AC-109 and FR-015-AC-110 having landed, and on IR-805, whose graph claims
    need CG integration of the merged IR-851/IR-852 accessors and QSL-820 real-package
    population emission (after QSL-819)). Graph claims are served by FR-015-AC-98
    and FR-015-AC-105 to FR-015-AC-108, and FR-015-AC-106's real-Kani graph replay
    is the replacement for the corpus graph replay; Evidence for a real-Kani precondition
    is a recorded `make kani` or `make kani-gate` run on the slice PR's head that
    is green, cited in the PR (the real-Kani
- id: FR-015-AC-112
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-858, AD-004 step 4b). `HarnessSpec::new` is the only constructor
    and refuses with a typed `HarnessSpecError`, never a panic and never a harness:
    a spec with no cover or more than one cover; a cover whose condition is the literal
    `true`; no `Call` or more than one; an `Input` or `Assume` after the `Call`; an
    `Assert` before it; a binding name bound twice (other than an input''s own rebinding);
    a name, `result` or `callee` that is not a valid Rust identifier; an input domain
    with `minimum` above `maximum` or outside its type; an `unwind` outside `1..=MAX_OBLIGATION_UNWIND`;
    a mess'
- id: FR-015-AC-113
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-858). For every spec that constructs, `render` emits the
    header, the oracle source, the module wrapper and the proof attribute, then the
    spec''s statements in spec order, then its one cover as the last statement of
    the function: after every assumption, the subject call and every assertion, whatever
    order a family listed them in. A hand-built spec in each of several statement
    orders renders its cover last; a cover is not a field of `body` and no statement
    order moves it (AD-004 "The HarnessSpec and render contract (step 4b)", R-1; FR-015-AC-7).'
- id: FR-015-AC-114
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Implemented (IR-858). `render` of one spec returns equal bytes on repeated
    calls and in a second process, and a spec whose `Input` list is reordered renders
    the reordered source and not the same bytes (AD-004 "The HarnessSpec and render
    contract (step 4b)", R-1).
- id: FR-015-AC-115
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-858). After the scalar port, for each claim of a fixed
    set (each of `quire.op.integer.add`, `sub`, `mul` and `negate`; a claim with a
    literal operand; a claim with an inferred operand; a claim with each operand at
    an `i64` extreme; claims at two unwind values including `MAX_OBLIGATION_UNWIND`;
    claims at two ceilings values) the ported path renders the same artifact (path
    and source bytes), `ScalarObligationIdentity`, `options` and `ObligationRecord`,
    and the same digests derived from them, as a test-only reference copy of the pre-port
    `render_scalar` computes in the same test. '
- id: FR-015-AC-116
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-858). `render` holds no family-specific template text:
    for hand-built specs whose label, expressions and messages are not the scalar
    family''s, the rendered source consists of the fixed header, the spec''s `oracle_source`,
    the module wrapper, the proof attribute, the spec''s statements rendered from
    their typed fields, and the one cover, and contains no other text; and the ported
    scalar harness is emitted by `render`, so that the FR-015-AC-58 inspection drives
    `render.rs` for it (AD-004 "The HarnessSpec and render contract (step 4b)", R-1
    item 3).'
- id: FR-015-AC-117
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Implemented (IR-858). With the installed backend, after the scalar port,
    the unmutated scalar harness classifies `Verified` with its cover satisfied and
    the existing scalar mutation control (the oracle's `Add` swapped for `Subtract`,
    FR-015-AC-37) classifies `Falsified`; and a hand-built `HarnessSpec` with an unsatisfiable
    `Assume`, rendered by `render`, does not classify `Verified` (L-4). This run is
    a smoke over the ported source and not the evidence of equivalence, which FR-015-AC-115
    is (AD-004 "The HarnessSpec and render contract (step 4b)", P-3).
- id: FR-015-AC-118
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: For an admitted V2 scalar `implies` claim over two Boolean operands, the
    generated harness embeds its FR-014 oracle as FR-015-AC-20 requires and draws
    both operands symbolically. It calls `quire_contract_runtime::operators::implies_short_circuit`
    with `left` and a closure returning `right` as the subject expression, asserts
    that subject result equals an independent native `!left || right` expectation
    over the same operands, and separately asserts that the oracle's `Outcome<bool>`
    is `Completed` with that expectation; neither subject nor oracle computes the
    native expectation. With installed Ka
- id: FR-015-AC-119
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'PLANNED (IR-917; depends on FR-015-AC-40 and FR-015-AC-41). QSL source
    authors an operation with one parameter `amount: Int[0, 1000]` and its V2 postcondition
    `amount < 1000 implies amount + 1 <= 1000`; both occurrences of `amount` read
    that operation parameter, not a state field or subject-produced post-state value.
    The admitted `CheckedPackageV2` yields a clause-identified FR-014-AC-38 oracle
    and an FR-015 postcondition harness for that same clause, not a scalar implication
    harness. Under FR-015-AC-39 the harness draws the parameter once within its declared
    finite domain and passes that same'
- id: FR-015-AC-120
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'PLANNED (IR-942; depends on QSL ADR-013 O-07/O-09 and QSV FR-123). After
    the selected owner and occurrence validate under AC-122, CG forms the per-step
    record content identity from an RFC 8785 object whose only members are `subject_kind:
    "protocol-refinement-step"` and `record: StepSimulationRecord` encoded by QSV.
    Two separately valid selected refinements with the same obligation and hypotheses
    but different record `refinement` values, each with its own matching occurrence,
    yield different content identities; a record with a mismatched owner has no identity
    to compare. Within one admitted ref'
- id: FR-015-AC-121
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-942). A per-step `KaniObligationIdentity` uses the QSV-owned
    `StepSimulationRecord` as a typed subject, carries its content identity and the
    owning refinement's O-07 `generated` occurrence key separately, and keeps any
    CG proof or obligation kind in the harness record rather than the record content
    preimage. CG publishes a persisted harness-record digest over every `KaniObligationIdentity`
    member except `source_span`; changing any included member changes this digest,
    while changing only `source_span` does not. This digest is distinct from the per-step
    record content identity. The r
- id: FR-015-AC-122
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'PLANNED (IR-942). A selected proof-eligible refinement under QSpec FR-322-AC-88
    through AC-90 with an `Initial` record and `Step { row: i }` records for its admitted
    FR-375 `steps` order yields generated occurrences on the owner node at ordinals
    0 and `i + 1`, respectively. An ineligible refinement under FR-322-AC-91 (hidden
    field, history field, or `any` step target) carries only its `declaration` occurrence
    and yields no per-step record, harness, or proof credit; the IR reader refuses
    a generated occurrence on it as `invalid_package`/`invalid-value` before CG setup.
    Two step rows tied on nod'
- id: FR-015-AC-123
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-110 retires `prepare_bounded_collection_query`, external
    public-consumer calls to that collection operation have the direct result signature
    specified by `interface-001`, with `KaniOutcome` as its error type. Until FR-015-AC-111
    retires `prepare_finite_graph_reaches` and `generate_bounded_kani_corpus_case`,
    external public-consumer calls to those graph and corpus operations have their
    direct result signatures specified by `interface-001`, with `KaniOutcome` as their
    error type. Each retained operation has no public `FamilyLoweringError`, `BoundedCorpusError`,
    `OutcomeConstructi
- id: FR-015-AC-124
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: After infallible-constructor adoption, each retained family/corpus adverse
    control returns the original exact non-success kind, typed cause, source identity
    and selected profile/bound context, with no Boolean claim, partial artifact or
    premature identity claim; successful outputs and failure ordering remain unchanged.
- id: FR-015-AC-125
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Authentic QSL-emitted, IR-admitted bounded integer scalar and clause inputs
    generate kernel-owned native carriers and checked operation plans for all intermediates,
    with original graph identities/domains retained. Add/Subtract/Negate/ordering
    retain their admitted signed-i128 capacities; Multiply is eligible exactly when
    both whole operand intervals fit signed32 and uses the exact four-corner signed64
    image retained in i128. A Multiply operand interval outside signed32 makes the
    whole requested arithmetic component native-ineligible, even with a zero opposite
    operand, fitting actual payload or
- id: FR-015-AC-126
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Genuine TC-027 healthy and bound mutant, TC-025 arithmetic mutant, IR-917
    clause implication and IR-948 V2 pre/post/invariant profiles regenerate through
    public CG with the reviewed admitted kernel API. Original semantic domains, selected
    propositions/clauses, independent expected values, subject controls, assertions,
    final completion covers and existing solver/unwind/time settings remain. Static
    plan-admission checks establish that the original TC-027, IR-917 and IR-948 fixture
    operand/intermediate intervals remain admitted under this policy, including their
    original 0..1000 domains; those ch
- id: FR-015-AC-127
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: End-to-end denied/cancelled admitted-native controls retain all real Meter
    counters, charge order, complete incomplete/refusal fields and admitted prefixes
    at every reached charge point, compared with independent normative expectations
    and the general exact route. A semantic result-domain failure remains after arithmetic
    charge and before result-retain, separately from generation-time native-capacity
    failure.
- id: FR-015-AC-128
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'When a request submits a QSL V2 constraint clause containing a collection
    quantifier as a standalone collection claim, the generator SHALL settle that item
    `unsupported` with `CollectionClaimNotRendered` / `UnsupportedClaimShape`, no
    harness and no proof credit, without using the constraint as an assumption, bound
    or capacity rescue. PARTIAL (IR-1091, CG #647): a source-backed tagged refusal
    control exists; its final reviewed-head gate is pending. Constrained-method semantics
    remain planned under IR-1157.'
- id: FR-015-CON-1
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Clause ABI construction SHALL check both endpoints of a bounded integer
    dependency before conversion to i64 and refuse one outside i64 as `UnsupportedDependency`
    in the clause-lowering detail, without producing a binding.
- id: FR-015-CON-2
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Subject binding SHALL check both endpoints of a bounded integer dependency
    before conversion to i64 and refuse one outside i64 as `UnsupportedBinding`, without
    producing a binding or harness.
- id: FR-017-AC-2
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: An absent launcher is refused with a typed reason naming its path before
    anything runs.
- id: FR-017-AC-4
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A run is verified only when the process exited successfully, the report
    states success, and every cover property is reported satisfied; a successful run
    with an unsatisfied cover is cover-unsatisfied with its satisfied and total counts;
    a report of success from an unsuccessfully exited process, a report with no cover
    property and a report with no successful check are each inconclusive with their
    own reason; console banners never decide the verdict.
- id: FR-017-AC-5
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A failed non-unwinding check with a concrete playback is falsified carrying
    that playback verbatim and never the playback of a satisfied cover; a failure
    with no playback is inconclusive for that reason; a failed unwinding assertion
    is inconclusive as an exhausted bound rather than falsified, and a succeeded unwinding
    check in a results listing is not a failure.
- id: FR-017-AC-6
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The execution evidence of a single run carries the obligation kind, the
    harness path, the launcher path, the complete argument vector, the unwind bound,
    the solver, the exit code and the outcome.
- id: FR-017-AC-7
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A crate whose library source does not contain the harness source byte for
    byte is refused, and no backend runs.
- id: FR-017-AC-11
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A routed FR-022/FR-014 exact-scalar harness (`KaniScalarObligationHarness`)
    runs through `execute_kani_obligation` and `kani_launch_command` the same way
    an FR-015 contract harness does: a crate whose library source lacks its generated
    source byte for byte is `HarnessNotInCrate`, its covers classify a run identically
    (all satisfied is verified, an unsatisfied one is cover-unsatisfied, none printed
    is inconclusive), and its evidence carries `None` for obligation kind, since an
    exact-scalar claim carries no contract role.'
- id: FR-017-AC-12
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Real Kani captures of a verified run, a falsified run with a playback,
    an exhausted unwind bound, a run whose only check is unreachable, a partly satisfied
    cover and a run with no cover each parse into the expected typed report of harness
    status, successful checks, cover counts and failed checks, and classify to the
    expected outcome; the falsifying playback block passes through verbatim.
- id: FR-017-AC-13
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: RETIRED, replaced by FR-017-AC-61; its generated-source digest member and
    comparison are obsolete. The replacement retains the zero-checks classification,
    typed collection admission, source-byte custody, bound-assertion location, all
    three entry refusals, batch atomicity and report-binding controls; this row earns
    no implementation credit.
- id: FR-017-AC-14
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A launcher whose stdout or stderr carries more than 8 MiB times its harness
    count is stopped, its process group is killed, and the run is refused with `kani_output_over_limit`
    naming the stream, the limit and the count, with no outcome and no retained text;
    a launcher whose stream is exactly the limit completes with its real exit status
    and the whole stream retained. No stream is ever truncated and then classified.
- id: FR-017-AC-15
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A timeout too large to add to the current instant never elapses and does
    not panic.
- id: FR-017-AC-16
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The launcher''s capture threads are stopped and joined on every outcome:
    they return what the launcher wrote before it ended, stop while a write end is
    still held open, and stop within their drain limit while a straggler keeps writing,
    so a process holding a pipe open does not delay the return.'
- id: FR-017-AC-17
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A run that times out has the whole group the launcher leads killed, a real
    grandchild included.
- id: FR-017-AC-18
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A report that is malformed, of an unknown check or harness status, of another
    schema version, that holds other than one harness result in a single run, or whose
    harness states success while it lists a failed, errored, undetermined or unknown
    check (of any class, including an unwinding assertion) is refused with its own
    typed cause and never classified, for every obligation kind; a run that exited
    successfully and exported no report is refused, and one that exited unsuccessfully
    and exported none is `NoVerdict`.
- id: FR-017-AC-19
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The generator appends `-Z unstable-options --export-json /proc/self/fd/N`
    after unchanged harness options, with actual child-only anonymous pipe writer
    N >= 5, and records the actual argv bytes. It uses per-run unnamed report authority,
    never a named target-directory report or fallback. FR-034 AC-32/33 own collector
    bounds, resource-stop classification, kernel lifetime and immutable final handoff.
    Zero-byte EOF means no exported report; nonempty partial/malformed content is
    refused under AC-18 unless an independently established resource/deadline stop
    takes precedence with the existing single-
- id: FR-017-AC-20
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The evidence and the classified run list every check of a real run with
    its id, class, source file and line and status, a line stated as unknown is absent,
    and a non-numeric line or a check with no location is a refused report; the view
    serializes as `id`, `class`, `location { file, line }` and `status` and is not
    deserializable.
- id: FR-017-AC-21
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: N harnesses with equal option vectors (the harness selection removed) and
    equal identity `ProofCeilings` (`wall_clock` T and `memory_bytes` M) start exactly
    one launcher process whose argument vector holds one `--harness <module::harness>
    --exact` pair per member in request order, `--harness-timeout` T, then the shared
    options; N harnesses in G such groups start G processes; so N > 1 compatible harnesses
    start fewer than N processes; one harness starts one process with the single-run
    argument vector. Requests whose wall-clock or memory ceilings differ start separate
    processes. The group's proc
- id: FR-017-AC-22
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Over a real or captured batch report, members are matched by `module::harness`
    path (two members with the bare symbol `check` stay distinct), and each member''s
    outcome, checks and SUCCESS count come from its own entry: a falsified member
    beside a verified member leaves the verified member verified; a Success member
    in a non-zero exit with no Failure entry is inconclusive, and with one Failure
    entry is verified; a Failure entry with no checks and exit status `timeout` is
    inconclusive timed-out naming T while the others keep their results; a Failure
    entry with no checks and no timeout is inconcl'
- id: FR-017-AC-23
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A group's report that lacks a requested harness, holds one twice or holds
    an unrequested one refuses the group with a typed cause and classifies no member;
    a member whose source is not in the crate refuses the whole batch with `HarnessNotInCrate`
    and launches nothing; a falsified member's playback is the first counterexample
    block headed for its own path and never another member's block, a member that
    fails two property checks (two counterexample blocks under its path, as real Kani
    prints) takes the first, as the same harness alone does, and neither it nor its
    neighbours lose their evidence, a
- id: FR-017-AC-24
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A launcher that exits on its own after leaving a real grandchild in its
    process group has that grandchild killed by the time the run returns, the same
    as on a timeout.
- id: FR-017-AC-25
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A capture thread that panics, or whose poll or read of the stream errs,
    refuses the run with `kani_output_unread`; the run is never classified from empty
    text.
- id: FR-017-AC-26
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A generated FR-035 caller Text-admission harness enters the same production
    execution path in single and compatible batch runs; its evidence carries the exact
    typed caller identity (profile, bounds and finite payload class) supplied with
    that harness and `None` for contract obligation kind. A matching successful report
    with a satisfied cover and successful agreement assertion classifies `Verified`;
    an unsatisfied cover classifies `CoverUnsatisfied` with counts; a failed agreement
    assertion with its own concrete playback classifies `Falsified` with that playback.
    A source-mismatched harness, mi
- id: FR-017-AC-27
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A genuine routed `x + 1` scalar harness from `generate_routed`, its complete
    emitted oracle package and one in-range concrete operand are passed to the paired
    runner. One real Kani run reports `Verified`, and the runner invokes the public
    oracle from the same staged library source at that operand with the request's
    exact `ScalarLimits`; the receipt contains the Kani evidence and the actual typed
    runtime result of that invocation. A Kani verdict alone or a result reconstructed
    by the caller is insufficient.
- id: FR-017-AC-28
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Before either child starts, a package or harness whose path or bytes differ
    from the retained routed output, a selected oracle symbol or argument order that
    differs from the retained scalar identity, or an operand count/value outside the
    retained ordered ranges is refused with a typed cause and no receipt. For an accepted
    request, the exact supplied `ScalarLimits` are passed unchanged to `KaniScalarNativeRequest`,
    used to initialize the native meter, and returned in the receipt; no hidden or
    substituted limit value is permitted. CG attests these supplied, used and returned
    values are equal, bu
- id: FR-017-AC-29
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The runner captures and compares the exact path and byte set of every
    staged execution input: the complete generated package, retained harness, canonical
    native adapter, observer manifest and both independently generated dependency
    locks. Neither Kani nor the native invocation can write the staged source tree.
    A changed, replaced or additional staged file before or after either invocation
    yields a typed failure and no receipt. The receipt separately binds the scalar
    identity, oracle symbol, ordered concrete operands and exact supplied `ScalarLimits`;
    none may be substituted after measurement.'
- id: FR-017-AC-30
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The public receipt exposes read-only captured staged entries, scalar identity,
    ordered operands, exact supplied and meter-used `ScalarLimits`, verified Kani
    evidence and actual typed native outcome. Callers cannot construct, deserialize
    or mutate a receipt, set its outcome, or turn any non-verified, refused, inconclusive,
    faulted or malformed run into a receipt.
- id: FR-017-AC-31
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The paired runner executes an order-sensitive routed scalar operation with
    two symbolic operands in two concrete operand orders. Each real Kani run is verified;
    the native invocation returns the result for that order; and each receipt binds
    its exact ordered operand vector and typed native result. Swapping the operands
    changes the observed result and receipt context while leaving the staged generated
    package unchanged.
- id: FR-017-AC-32
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'In the `make kani` lane, a crate whose library source holds the generated
    harness byte for byte and, beside it, an item that does not compile (a reference
    to an undefined name) is run through `execute_kani_obligation`: the run exits
    unsuccessfully, exports no report and is `Inconclusive` `NoVerdict` with its exit
    code, and its evidence carries a diagnostic record whose phase is `compile`, whose
    stdout excerpt holds the rustc error code and Kani''s `error: Failed to execute
    cargo` line and whose stderr excerpt holds Cargo''s `error: could not compile`
    line with the crate path as `<CRATE_DIR>`. A '
- id: FR-017-AC-33
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Over single-run launcher stand-ins that exit 1: a report of success beside
    the unsuccessful exit is `unknown` even when its stdout holds `Solving with CaDiCaL
    3.0.0`; with no report, stdout holding the Kani banner, `Checking harness proofs::check...`
    and `Solving with CaDiCaL 3.0.0` is `solver`; the banner and the `Checking harness`
    line alone is `backend`; the banner and `error: Failed to execute cargo (exit
    status: 101). Found 1 compilation errors.` is `compile`, as is stderr `error:
    could not compile` alone with the banner; a usage error on stderr and nothing
    on stdout is `launcher`; the ba'
- id: FR-017-AC-34
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A redacted stream of exactly 8192 bytes is kept whole with `omittedBytes`
    0; one of 8193 bytes whose final line starts within the window keeps the longest
    line-aligned suffix of at most 8192 bytes with `omittedBytes` the byte count before
    it; a stream whose final line alone exceeds 8192 bytes keeps at most 8192 bytes
    ending at its last byte, starting on a character boundary; a stream that is one
    10000-byte line followed by `\n` keeps the last 8192 bytes of it, ending in `\n`,
    never an empty excerpt; a stream of a 10000-byte line, `\n`, `error: last\n` keeps
    exactly `error: last\n`; a 4-byte ch'
- id: FR-017-AC-35
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'With crate directory `/h/u/ws/crate`, target directory `/h/u/ws/crate/target`
    and home directory `/h/u` supplied in the request, a stand-in whose streams print
    `(/h/u/ws/crate)`, `/h/u/ws/crate/src/lib.rs:6:21`, `/h/u/ws/crate/target/debug/x.out`,
    `/h/u/.kani/bin`, `/h/u/ws/crate2/src/lib.rs`, `/opt/tool/bin`, `https://host/x/y`,
    `and/or`, `/`, `2026-10-16T22:27:04.275975036Z`, `Runtime Symex: 0.00331318s`,
    `Runtime Solver: 1.385e-05s`, `in 0.02s`, `check_ttl_30s`, `u32s`, `pid 4242`
    and `thread ''main'' (4242) panicked` yields, in order, `(<CRATE_DIR>)`, `<CRATE_DIR>/src/lib.rs:6:21`,
    `<TARGET_'
- id: FR-017-AC-36
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The diagnostic record never feeds classification: two runs with the same
    report and exit and different stream text classify to the same outcome, checks,
    SUCCESS count and playback, and two `NoVerdict` runs with equal exit and no report
    whose streams differ have equal evidence in every field but `diagnostics`, `memory`
    (the observed peak resident bytes vary run to run) and the run''s report-path
    argument (the last argument, unique to each run); the FR-029 terminal value of
    a `NoVerdict` evidence carrying a record is `Failed`, the value FR-029-AC-4 gives
    the no-verdict reason, equal to the termin'
- id: FR-017-AC-37
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Over single-run launcher stand-ins that exit 1, print `Checking harness
    proofs::check...` and `CBMC failed with status 64` on stdout and `Unknown option:
    --no-such-option` on stderr, and export a report whose one result `proofs::check`
    is `Failure` with no checks and whose one `error_details` entry for `proofs::check`
    has the exit status shown: `exit_code_64` is `Inconclusive` `FailedWithoutCounterexample`
    with `failureCause` `{ "kind": "exit_code", "code": 64 }` and a `diagnostics`
    record of phase `unknown` whose stderr excerpt holds `Unknown option: --no-such-option`;
    `exit_code_15` has `cod'
- id: FR-017-AC-38
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A single-run stand-in that exits 1 and exports a `Failure` report listing
    a failed assertion check with no playback on the console, whose entry for the
    harness has exit status `properties_failed`, is `FailedWithoutCounterexample`
    with `failureCause` `{ "kind": "properties_failed" }` and no `diagnostics` member.
    The same report with no `error_details` member, with an entry that names another
    harness only, with two entries naming the harness, with an entry whose `exit_status`
    is null, or whose one result names no `harness_id`, is `FailedWithoutCounterexample`
    with neither a `failureCause` nor a '
- id: FR-017-AC-39
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A batch stand-in of members `a::check`, `b::check`, `c::check`, `d::check`
    and `e::check` whose process exits 1 and whose report has `a::check` `Success`
    with a passing assertion, every cover satisfied and no exit status, `b::check`
    `Failure` with no checks and exit status `exit_code_64`, `c::check` `Failure`
    with no checks and exit status `timeout`, `d::check` `Failure` listing a failed
    assertion with no playback block headed for its path and exit status `timeout`,
    and `e::check` `Failure` listing a failed assertion with no playback block headed
    for its path and exit status `properties_failed
- id: FR-017-AC-40
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The cause never feeds classification: three single-run stand-ins with
    the same `Failure` report, no checks and the same exit, whose entries give `exit_code_64`,
    `out_of_memory` and `cbmc_exploded` and whose streams differ, classify to the
    same outcome, checks, SUCCESS count and playback, and their evidence is equal
    in every field but `failureCause`, `diagnostics`, `memory` and the report-path
    argument; the cause is the only input of the FR-029 terminal value that differs:
    `exit_code_64` and `cbmc_exploded` give `Failed`, equal to the terminal value
    of the same evidence with `failureCause` and '
- id: FR-017-AC-41
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Every `KaniScalarNativeError` variant has a `code()` that is a total `&''static
    str` from exactly this set: `invalid_request`, `artifact_package`, `snapshot_io`,
    `kani_execution`, `proof_not_verified`, `oracle_stop`, `native_launch`, `dependency_lock`,
    `native_result`, `snapshot_changed`. The set is closed and the codes are pairwise
    distinct. The type is `#[non_exhaustive]` (a rustdoc `compile_fail` exhaustive
    match without a wildcard arm, beside a compiling control with one), and `class()`
    returns a closed `KaniScalarNativeErrorClass` (a compiling exhaustive two-arm
    control with no wildcard, a'
- id: FR-017-AC-42
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '`class()` is `CompletionRefusal` for `invalid_request`, `artifact_package`
    and `oracle_stop`, and `EnvironmentFault` for `snapshot_io`, `native_launch`,
    `dependency_lock`, `native_result` and `snapshot_changed`. For `proof_not_verified`
    it is `CompletionRefusal` for evidence outcome `Falsified`, `CoverUnsatisfied`,
    and `Inconclusive` `MissingCoverSummary`, `VacuousProof`, `UnwindBoundExhausted`
    or `FailedWithoutCounterexample` with no `failureCause` or `properties_failed`,
    and `EnvironmentFault` for `Cancelled`, `Inconclusive` `TimedOut`, `MemoryExhausted`
    or `NoVerdict`, and `FailedWithoutCou'
- id: FR-017-AC-43
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '`NativeResult` is produced only for the helper''s malformed, missing or
    unsupported structured result; a malformed helper result is an `EnvironmentFault`,
    and no identity encoder runs in the paired runner.'
- id: FR-017-AC-44
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The code table and the mapping to class are frozen: a test lists every
    variant with its expected code and its mapping to class (one class for a fixed
    variant; for `proof_not_verified` the class of each `KaniRunOutcome`, `KaniInconclusiveReason`
    and `KaniFailureCause` value; for `kani_execution` the class of each nested refusal),
    fails if a variant, outcome, reason, cause or nested refusal exists that the list
    lacks or if a code value or a mapped class differs, and fails if two variants
    share a code. Adding a variant, code, class or mapping entry is a spec change
    to the "Scalar native error dis'
- id: FR-017-AC-45
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: For a genuine routed scalar package, the receipt carries no `DigestRecord`,
    `ScalarProofContentIdentity` or generated-content digest. The staged path and
    byte set, exact supplied limits and native observation remain available through
    their owning read-only receipt fields.
- id: FR-017-AC-46
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'For a genuine routed `x + 1` package the captured staged set contains
    exactly these ten paths in UTF-8 byte order: `Cargo.lock`, `Cargo.toml`, `claim-map.json`,
    the admitted `kani-obligations/<module>.json` record, `observer/Cargo.lock`, `observer/Cargo.toml`,
    `source-maps/lib.json`, `src/bin/ir791_native.rs`, `src/lib.rs` (harness), and
    `src/oracle-artifacts-lib.rs` (emitted library). All four original emitted artifacts,
    harness and record compare byte-identically under the rename map. Removing, renaming,
    duplicating or changing one byte of any entry yields a typed failure and no receipt.
    Non'
- id: FR-017-AC-47
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Two receipts for the same package and harness with different operands,
    different `ScalarLimits` and different native outcomes retain the same staged
    path and byte set; each receipt retains its own operands, limits and outcome.
    This holds for the swapped-operand runs of FR-017-AC-31.
- id: FR-017-AC-48
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The receipt exposes read-only the ordered staged `(path, bytes)` entries
    captured before execution, including both manifests, both independently generated
    locks, the source map and canonical adapter. All ten entries remain sealed and
    compared across both execution boundaries. They remain readable after the runner
    removes the private snapshot; no accessor reads the filesystem. `Drop` is a best-effort
    backstop only. A failure to remove the snapshot directory changes neither verdict
    nor entries and appears only in read-only `cleanup_diagnostic()`; successful cleanup
    leaves that diagnostic empty. '
- id: FR-017-AC-49
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: For consumer-owned byte comparison, the CG receipt exposes the original
    emitted `Cargo.toml`, `claim-map.json` and `source-maps/lib.json` at their own
    paths, emitted `src/lib.rs` as `src/oracle-artifacts-lib.rs`, retained harness
    as `src/lib.rs`, and retained record as `kani-obligations/<module>.json`. A CG
    test compares these entries byte for byte with the original routed artifacts,
    harness and record; a changed byte or path is detectable without a digest. The
    driver-owned refusal and no-claim behavior is verified under FR-032-AC-4, not
    credited by this CG test.
- id: FR-017-AC-50
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The `NativeMeterObservation` of a receipt exposes read-only the `ScalarLimits`
    the native meter was constructed with. They are read back from the native invocation''s
    own output, not copied from the request: a test seam that makes the adapter report
    a well-formed but different limit value makes the helper''s structured result
    unsupported in FR-017-AC-43''s sense and yields `NativeResult` / `native_result`
    / `EnvironmentFault` and no receipt. For an accepted request the request limits,
    the meter-observed limits and `scalar_limits()` are equal, and the meter-observed
    limits are unchanged after the '
- id: FR-017-AC-51
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A consumer compile test names and constructs `quire_contract_codegen::ScalarLimits`
    using only codegen crate paths, a typed accessor to `KaniScalarNativeRequest::scalar_limits`
    compiles with that same return type, and assignment to the owning `quire_exact::ScalarLimits`
    compiles without conversion, wrapper or compatibility shim. The CG public name
    and field shape remain unchanged. Planned with IR-17 kernel adoption.
- id: FR-017-AC-52
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Production emitted bytes and feature-free exact dependency remain unchanged.
    The isolated private observer uses resolver 2 and its sole example compiles the
    single canonical adapter in place through `../src/bin/ir791_native.rs`, with runtime
    `exact` and kernel `test-support` exposed only by dev-dependencies; no wrapper,
    copied source, symlink, normal/build logging edge or CLI feature override is allowed.
    Native invocation explicitly names `observer/Cargo.toml` and the example; the
    real production proof retains its harness, ordered options, claims, covers, bounds
    and backend gates. A control re
- id: FR-017-AC-53
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: RETIRED WITH REPLACEMENT FR-017-AC-62. The source-identity comparison is
    withdrawn; the independently resolved package, repository and version checks,
    typed refusal and complete staged-byte custody are specified by AC-62. This identifier
    remains in trace history without active coverage credit.
- id: FR-017-AC-54
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A successful claimed-init confirmation poll with no `IN` yields public
    `MemoryObservationFailed` with the existing outer and nested prefixes and labelled
    observations of that attempt: returned poll count, raw revents bits, monotonic
    poll elapsed duration with its unit, captured init PID/start ticks, captured wrapper
    PID or explicit absence, init-handle presence, gate presence and cleaned state.
    Test controls through the existing `PollOperation` seam delegate first to real
    poll on the real owned pidfd, record its positive count and `IN`, then clear revents
    and return either zero or that genuine'
- id: FR-017-AC-55
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Through the real public execution entry and real owned namespace, an unchanged
    real-poll control retains its original public result and confirmed cleanup; each
    AC-54 omission refuses even when cancellation is submitted before or after the
    cleanup barrier. The confirmation seam sees the same claimed pidfd and exactly
    one poll with timeout `(5, 0)` per cleanup invocation. Failure retains gate/cleaned
    state and the existing `Drop` retry, and actual backend processes/namespace are
    gone after return. The existing real-poll-delegating errno control retains its
    public refusal rather than cancellation
- id: FR-017-AC-56
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Inspection of the cleanup change shows that init-pidfd KILL still precedes
    wrapper-group KILL, the single confirmation poll retains its `(5, 0)` timeout,
    signal-error/poll-error/missing-readiness precedence is unchanged, syscall errors
    retain their original kind and errno, and only successful cleanup closes the gate
    and marks cleaned. Existing `Drop` behavior is unchanged. Diagnostic observations
    come only from that poll and existing owned fields; no added wait, retry, skip,
    PID reopen, procfs/cgroup rescan or invented host-state field is introduced.
- id: FR-017-AC-57
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: PLANNED (IR-917). The public `execute_kani_concrete_playback(&KaniExecutionRequest,
    &KaniConcretePlayback) -> Result<KaniConcretePlaybackOutcome, KaniExecutionRefusal>`
    accepts only a seal-validated V2 clause harness; a Contract, Scalar, StateFrame
    or Collection harness refuses before launch as `ConcretePlaybackUnsupportedHarness
    { harness_path }`. For an accepted harness, `test_name` is the exact libtest `module::function`
    path and `appended_test` is one complete parseable `#[test]` function. Invalid
    retained V2 harness provenance refuses as `HarnessProvenanceMismatch`; a missing,
    duplicated,
- id: FR-017-AC-58
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: PLANNED (IR-917). For admitted concrete playback CG runs `kani playback
    --lib -Z function-contracts -Z concrete-playback -- <test_name> --exact --nocapture`
    through the existing owned bounded launcher with the request's installation, target
    directory, cancellation handle and harness-identity wall-clock/memory ceilings;
    `--lib` selects one library test target, and `-Z function-contracts` repeats the
    proof compilation option as Kani requires for playback. Stdout and stderr retain
    the existing one-harness limits and over-limit/unread typed refusals. A concluded
    process that passes AC-59's success
- id: FR-017-AC-59
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'PLANNED (IR-917). A successfully exited concrete-playback process yields
    `Completed { succeeded: true, .. }` only if its bounded stdout has one intact
    exact libtest pass line for the selected test (plain `test <name> ... ok` or expected-panic
    `test <name> - should panic ... ok`) and exactly one compatible summary `test
    result: ok. 1 passed; 0 failed;` from the selected library target. A zero-match
    success, another or duplicate summary, another test name, or `--nocapture` text
    interleaved through the selected pass line refuses as `ConcretePlaybackTestNotObserved
    { test_name }`; the parser fails'
- id: FR-017-AC-60
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: PLANNED (IR-917). Through the public `execute_kani_concrete_playback` entry
    and an installed Kani backend, a healthy V2 clause playback test reports its selected
    test as passed, and the plus-one owner mutant's `amount_current = 999` playback
    executes the same sealed generated arithmetic assertion, observes its unique assertion
    message or source location and reports the selected expected-panic test as passed.
    Both installed runs use the same `-Z function-contracts` compilation option as
    the sealed proof, retain their bounded stdout/stderr and memory observations,
    and confirm owned teardown. The
- id: FR-017-AC-61
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A run whose process exited successfully and whose backend reported success
    with zero successful checks is inconclusive with the vacuous-proof reason, never
    verified, except that a non-collection precondition harness with no assertion
    is decided by its cover summary. A collection precondition is identified by the
    typed V2 clause identity's precondition kind and admitted collection-quantifier
    marker even if its generated assertion is missing. Its sealed identity retains
    the unique artifact-relative line of the FR-015-AC-99 Boolean-completion assertion
    and carries no generated-source digest or eq
- id: FR-017-AC-62
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Both package locks are independently freshly resolved by the bounded offline
    workflow, without copying a parent/other lock or substituting a cached resolution.
    In each staged lock, exactly one entry for each owning `quire-contract-runtime`
    and `quire-exact` package names its authoritative `agent-ix/quire-contract-runtime`
    or `agent-ix/quire-exact` repository and supplies a valid Cargo package version.
    For each package, the production and observer locks agree on the literal package
    name, repository name and version; source URL query and fragment values are not
    comparison keys. The independent o
- id: FR-017-CON-1
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The generator SHALL NOT convert a non-verified outcome into a proof claim.
- id: FR-017-CON-2
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The generator SHALL NOT report a generation-time classification as an execution
    outcome.
- id: FR-025-AC-1
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: For every harness kind, emitted `kani::any()` bindings in source order
    equal the persisted physical schema obtained by row-specific expansion of the
    identity's logical `arguments`, which ascend by parameter identifier. Existing
    Boolean/integer one-binding expansion is unchanged; the planned Rational row expands
    each original parameter as numerator then denominator.
- id: FR-025-AC-2
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: Every argument binding in a persisted identity names its original parameter
    node id and declared domain. Each planned Rational component also retains its
    role, position and own original bounds within that one parameter's complete Rational
    domain.
- id: FR-025-AC-3
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: A Boolean argument is a `bool` that reaches the oracle unchanged, and a
    bounded-integer argument is an `i64` that reaches the oracle as `rt::Integer::from`
    of that value. The values `i64::MIN`, `-1`, `0`, `1` and `i64::MAX` each reach
    the oracle equal to their mathematical value.
- id: FR-025-AC-4
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: An argument whose domain has no declared finite bound yields no harness
    and is never narrowed to a machine range.
- id: FR-025-AC-5
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: A frame obligation is accounted `unsupported` with a typed reason and no
    harness.
- id: FR-025-AC-6
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: A binding that names no parameter of the selected function, and a parameter
    of that function with no binding, are each refused at generation with a typed
    reason and no harness.
- id: FR-025-AC-7
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: An argument with no applicable row (decimal, IEEE, text, enum, composite,
    collection, function, or Rational outside the planned FR-046 row) is accounted
    `unsupported` with a typed reason naming its family/capability, and no harness
    is emitted. The planned Rational row does not authorize a harness without its
    lossless decoding and public replay prerequisites.
- id: FR-025-AC-8
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: A harness over a state-reading obligation passes each state value to the
    subject as `&mut` to a harness-owned value and copies it before the call; the
    pre-state the harness asserts over is the copy, and the post-state is the value
    after the call.
- id: FR-025-AC-9
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: PLANNED (IR-806). Every emitted composite shadow leaf binding retains its
    original operand/parameter node, typed path, declared domain and primitive draw
    type in symbolic call order; controls distinguish absent/null/present slots and
    option/union members. A missing, duplicate, unknown or domain/type-conflicting
    leaf refuses with a typed reason and no harness; parameter/literal and literal/literal
    operands retain their literal singleton identity/value with no unconstrained draw.
    Public reconstruction preserves these facts through FR-033.
- id: FR-025-AC-10
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: PLANNED (IR-258). Every supported FR-046 Rational argument has exactly
    two ordered I64 physical bindings grouped to its one original parameter, complete
    domain and canonical value. Actual playback decodes losslessly with existing I64
    width/comment checks and physical byte/value-count caps, then reconstructs and
    submits exactly one canonical owner Rational per parameter. Missing, duplicate,
    foreign, role-swapped or bound-conflicting schema, malformed entries, noncanonical
    pairs and over-limit witnesses refuse at their responsible typed boundary; no
    alternate normalized witness is replayed. Exis
- id: FR-028-AC-1
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Every harness identity records the request's memory ceiling and wall-clock
    ceiling, and changing either ceiling changes the identity.
- id: FR-028-AC-2
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A backend Kani run that exceeds its wall-clock ceiling is stopped and classified
    KaniRunOutcome::Inconclusive with the timed-out reason naming the ceiling, never
    verified or falsified. FR-029 maps this source outcome to final QSL Incomplete(TimedOut);
    the backend classification is not the terminal category.
- id: FR-028-AC-3
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A backend Kani run that exceeds its memory ceiling is stopped and classified
    KaniRunOutcome::Inconclusive with the memory-exhausted reason naming the ceiling,
    distinct from timed-out and never verified or falsified. FR-029 maps it to final
    QSL Incomplete(ResourceExhausted); FR-028-AC-24 governs the separate refinement
    run.
- id: FR-028-AC-4
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Every execution evidence records the bounds of each symbolic argument and
    the two ceilings the run was held to.
- id: FR-028-AC-5
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Every harness identity, generation refusal and execution evidence names
    its family.
- id: FR-028-AC-6
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A harness of a family with no supplied shadow records proof subject `production`.
- id: FR-028-AC-7
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A family with a supplied shadow yields a harness with proof subject `bounded_shadow`
    and its refinement obligation in the same result, and never the shadow harness
    alone.
- id: FR-028-AC-8
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: An argument narrowed inside its declared domain records the declared domain,
    the narrowed bound and that the harness covers only the narrowed bound, in the
    harness identity and in the evidence.
- id: FR-028-AC-9
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A narrowing outside the argument's declared domain is refused with a typed
    reason and no harness.
- id: FR-028-AC-12
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: In a group of N harnesses (one launcher process of a FR-017 batch) sharing
    a wall-clock budget T, a member the backend cuts off (entry status Failure, no
    checks, exit status `timeout`) is `inconclusive` timed-out naming T and never
    falsified, while members that finished keep their verified or falsified results;
    a group still running at N times T is killed, refused as timed out, and no member
    is classified; an N times T too large to represent never elapses and does not
    panic; a T above 4294967295 whole seconds launches the group without `--harness-timeout`
    and the group still runs, its outer bo
- id: FR-028-AC-13
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: The source of a `bounded_shadow` harness, parsed, names no crate but `core`,
    `alloc`, `std` and `kani` (so no path into `quire_contract_runtime`, `quire_exact`
    or any other crate the production oracle's crate depends on), no `kani::stub`
    or `kani::stub_verified`, and no `kani::assume` outside the argument-domain assumptions
    of FR-015-AC-11; a mutated generator that imports a runtime type, emits a stub
    or assumes a relation between two drawn values fails each check. PLANNED (IR-241).
- id: FR-028-AC-14
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: The result of a `bounded_shadow` family carries a refinement obligation,
    as generated Rust source in the same result, that names the production subject
    by its symbol, the shadow, the bounded domain as one inclusive bound per drawn
    argument, the abstraction function from a shadow value to a production value,
    and the observables the family's comparison reads (FR-028-AC-15); the abstraction
    function lives in one module that the shadow, the expectation and the Kani harness
    do not import, and it builds each production value through the production oracle's
    own environment, the one its generated envi
- id: FR-028-AC-15
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: For every case the refinement run executes, the production subject's complete
    outcome under an unlimited meter agrees with the shadow's result on the observables
    its family names; for composite equality (FR-015-AC-71) that is `Completed` with
    the shadow's verdict and an admitted `equality.pair` count equal to the shadow's
    count. A case whose production outcome is `Refused` or `Incomplete`, a case whose
    production value the oracle's own environment refuses to construct (a construction
    refusal), or a case that differs on a named observable, is recorded as a disagreement
    carrying the case's drawn
- id: FR-028-AC-16
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A refinement run over a domain of at most the request's case cap cases
    executes every case and is classified `exhaustive`; a run over a larger domain,
    or one whose case count overflows `u128`, is classified `sampled`, runs the boundary
    cases and then exactly the request's sample size of seeded cases drawn uniformly
    over the whole domain from the request's seed, and records the class, the case
    count or that it overflowed, the cases run and the seed; two runs with equal inputs
    run the same cases. The boundary cases are each leaf's boundary values (each integer
    bound, its neighbour inside the bou
- id: FR-028-AC-17
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A verified `production` harness reports `production_proved`; a verified
    `bounded_shadow` harness reports `shadow_proved_refinement_exhaustive`, `shadow_proved_refinement_sampled`,
    `shadow_proved_refinement_not_run` or `shadow_proved_refinement_inconclusive`
    by its refinement run's class (`not_run` when none ran, `inconclusive` when the
    run reached a ceiling, FR-028-AC-24); a refinement disagreement reports `refinement_failed`
    whatever the shadow harness settled, including a verified one; and no result of
    a `bounded_shadow` harness reports `production_proved`. PLANNED (IR-241).
- id: FR-028-AC-18
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A harness identity of a shadow family lists each abstraction the shadow
    makes by name with the evidence that discharges it (a Kani check of the harness
    or a refinement comparison), each behaviour of the production subject the proof
    does not exercise, and each helper the shadow shares with its expectation, marked
    `value_bearing` (it returns a bound, a presence, a member set or a count the comparison
    reads) or `order_only` (it returns only an order the compared observables do not
    read, as FR-149's conjunction and node count do not read field order), with the
    reason for each `order_only`; a shado
- id: FR-028-AC-19
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A refinement domain that does not contain every case the shadow harness's
    assumptions admit (a bound narrower on any argument) is refused with a typed reason
    naming the argument, with no harness; and the domain a shadow harness draws from
    equals the declared domain, or the narrowed bound FR-028-AC-8 records. PLANNED
    (IR-241).
- id: FR-028-AC-20
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: 'An item whose shape lies outside its family''s shadow support is refused:
    a missing bound as `requires_bound` naming the unbounded node, a shape no shadow
    covers as `unsupported` naming the family and the capability, a shape over the
    family''s size budget as `unsupported` naming the size and the budget; none emits
    a harness, none invents a bound, and none falls back to a `production` harness;
    siblings of a refused item settle as they do alone. PLANNED (IR-241).'
- id: FR-028-AC-21
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: The backend process tree of every run is held to the harness identity's
    memory ceiling by a mechanism that observes or limits the tree's memory (the choice
    is the code change's, Open Questions); every execution evidence records the mechanism
    and, where it observes memory, the tree's peak resident memory; a tree that exceeds
    the ceiling is killed whole and settles as FR-028-AC-3 states, with `KaniInconclusiveReason::MemoryExhausted`,
    serialized `memory_exhausted`, a variant distinct from every existing reason;
    a FR-017 batch's launcher process tree is held to the ceiling as one group, killed
    wh
- id: FR-028-AC-22
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: 'A shadow family''s machinery catches each seeded mutant: a shadow harness
    over a mutated shadow is `Falsified` with a concrete playback, a refinement run
    over a mutated production subject settles `refinement_failed` naming the first
    disagreeing case, a mutant of a `value_bearing` shared helper that the production
    subject reads independently settles `refinement_failed`, one that it reads through
    the same helper is caught by the family''s independent check of that helper (FR-018-AC-23
    for the composite closure), and the unmutated family verifies with its cover satisfied
    and a refinement run that a'
- id: FR-028-AC-23
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Every execution evidence records the Kani version, the Rust compiler version,
    the solver name and version, the unwind bound and the complete option vector the
    run used, for a `production` harness and a `bounded_shadow` harness alike, and
    a refinement evidence records the Rust compiler version and the production subject's
    crate version. PLANNED (IR-241).
- id: FR-028-AC-24
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: 'The refinement run is a native process launched through one typed refinement
    execution entry that reuses FR-017''s launcher and FR-028''s ceilings: it is held
    to the memory and wall-clock ceilings its harness identity records, its evidence
    (class, case count, cases run, seed, both ceilings, mechanism, peak where observed,
    outcome, versions) is recorded beside the shadow run''s evidence, and a run that
    reaches either ceiling is stopped, records the ceiling, and yields strength `shadow_proved_refinement_inconclusive`,
    never `exhaustive`, `sampled`, `not_run` or `refinement_failed`. PLANNED (IR-241)'
- id: FR-029-AC-1
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`verified` with three SUCCESS checks maps to `Proved { success_checks:
    3 }`, and `falsified` with a reproduced replay maps to `Refuted`.'
- id: FR-029-AC-2
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`inconclusive` with the vacuous-proof reason and `cover-unsatisfied` each
    map to `Proved { success_checks: 0 }`, whose QSL category is `inconclusive` with
    cause `KaniVacuousProof`.'
- id: FR-029-AC-3
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The timed-out reason maps to `Incomplete(TimedOut)`, and the memory-exhausted
    and exhausted-unwind-bound reasons each map to `Incomplete(ResourceExhausted)`.
- id: FR-029-AC-4
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The no-verdict reason maps to `Failed`.
- id: FR-029-AC-5
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The missing-cover-summary reason maps to `Failed`. The failure-without-counterexample
    reason maps by its cause (FR-029-AC-32 to FR-029-AC-35).
- id: FR-029-AC-6
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The ordinary production/source-predicate map returns no `Tested`.
- id: FR-029-AC-8
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a replay disagreement of each `DisagreementCause` (`Verdicts`,
    `Witness` and `NoValue`) maps to `Inconclusive(ReplayParity)` carrying that `DisagreementCause`.'
- id: FR-029-AC-9
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a non-fault `ReplayRefusal` maps to `Inconclusive(ReplayRefused)`
    carrying `ReplayRefusal::code()` of that refusal.'
- id: FR-029-AC-10
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a fault maps to `Failed` for each of: the call-site fault''s
    own reading (`ReplaySettlement::Fault`, which QSL''s replay result does not carry:
    QSL reports a fault as an error); `ReplayRefusal::Fault` and `ReplayRefusal::Admission(AdmissionFailure::Fault)`,
    each bare and as the cause of `SpineReplayError::Refused`, `FrameReplayError::Refused`
    and `StateClauseReplayError::Refused`; and `CallSiteRefusal::Fault` bare and wrapped
    in `ReplayPackageError::CallSite`, `FrameReplayError::CallSite` and `StateClauseReplayError::CallSite`.'
- id: FR-029-AC-11
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with each CG-raised failure that carries no QSL code maps
    to `Failed`: `SpineReplayError::UnboundArgument`, `FieldDelimiter`, `Transcript`,
    `WrongArm` and `Identity`; `FrameReplayError::Transcript`, `Envelope`, `Name`,
    `NotAFrame`, `FieldSetMismatch`, `Decode`, `OutOfDomain`, `PreState` (a pre state
    the generator cannot read from the driver''s documents, or one that differs from
    the playback; a document whose bytes do not match its digest is QSL''s `Refused`,
    read by its code), `ScopeMismatch` and `Identity`; `ReplayPackageError::InvalidFunction`;
    a playback outside the harness proof'
- id: FR-029-AC-12
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: Across every replay settlement other than reproduced, `falsified` maps
    to a value other than `Refuted`.
- id: FR-029-AC-13
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a non-fault `CallSiteRefusal` or a `DependencyLockError::Input`,
    each bare and wrapped in `ReplayPackageError` and `FrameReplayError`, maps to
    `Inconclusive(ReplayRefused)` carrying `CallSiteRefusal::code()` or `DependencyInputRefusal::code()`
    of that refusal, and never to `Declined`.'
- id: FR-029-AC-14
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a `DependencyLockError::Input` that carries QSL''s `DuplicateIdentity`
    refusal (code `invalid_package`), as a lock whose only defect is a repeated library
    identity produces it (FR-016-AC-24), maps to `Inconclusive(ReplayRefused)` carrying
    `invalid_package`.'
- id: FR-029-AC-15
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: For the ordinary source-predicate `ReplaySettlement` input, a falsified
    outcome given no replay settlement is refused with `TerminalPairError::MissingSettlement`,
    and each other outcome (`verified`, `cover-unsatisfied`, `cancelled` and every
    inconclusive reason) given a settlement is refused with `TerminalPairError::UnexpectedSettlement`;
    neither returns a terminal value. The distinct typed composite-parity settlement
    input is governed by AC-17 and AC-19 to AC-28, not this criterion.
- id: FR-029-AC-16
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'The state-clause replay path (FR-024) settles as the other replay paths
    do: a `StateClauseReplayResult` that settles `ReproducedWithEvaluatedWitness`
    in category `violation` is a reproduction and an `inconclusive` one is a disagreement
    carrying its `DisagreementCause`, so `falsified` maps to `Refuted` and `Inconclusive(ReplayParity)`;
    `StateClauseReplayError::Refused` and `CallSite` read as `ReplayRefusal` and `CallSiteRefusal`
    do for a non-fault refusal (`Inconclusive(ReplayRefused)` with its catalog code;
    their fault reading is FR-029-AC-10''s), and `Dependencies` as `DependencyLockError`
    doe'
- id: FR-029-AC-17
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-806). IR-635''s Eq-only verified constructor invokes QSL with
    supplied evidence; the closed FR-028 strength producer/projection remains gated.
    The strength projection enumerates every FR-028-AC-17 strength: production retains
    its ordinary map; exhaustive projects to Exhausted, sampled/not_run to NotExhausted,
    ceiling-inconclusive to CeilingReached, and refinement_failed to Disagreed. Backend-inconclusive
    outcomes acquire no invented verified proof strength.'
- id: FR-029-AC-18
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: For the `falsified` state-clause run of the wrapping debit subject `deposit_debiting`,
    whose post-state value (-1) lies outside `balance`'s declared range, this repository
    builds the post snapshot with that exact unclamped value and does not refuse it
    itself (not as `StateClauseReplayError::OutOfDomain` or any other CG defect),
    and the run maps to `Refuted`, the ordinary violated terminal, and not to `Inconclusive(ReplayRefused)`
    or `Failed`. QSL settles the exact post-state observation as a reproduced violation
    (QSL-634).
- id: FR-029-AC-19
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED/GATED (IR-806/QSL-640). After the cross-outcome disagreement rule,
    verified shadow CeilingReached yields Incomplete(ResourceExhausted); nonzero Exhausted
    with QSL-derived complete coverage yields Proved n; remaining completed nonzero
    verification yields Tested. Tested is excluded for disagreement, ceilings and
    zero checks; tightening or missing coverage cannot promote it to Proved.
- id: FR-029-AC-20
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PARTIAL (IR-666 direct F-1 to F-7 report consumption covered; IR-806 must
    reach these rows from a CG-built request over the original proving context). An
    identity-valid falsified composite shadow consumes the shared Refinement and ordered
    F-1 to F-7 result: Disagreed/Failed/CgDefect; native Incomplete or ExecutionFault
    as GeneratedFault/Failed with NativeCause, winning over an invalid operand and
    every later limit; RefusedInput/ReplayRefused with QSL code and operand index,
    winning over CeilingReached; admission accounting exhaustion as Incomplete(ResourceExhausted)/Admission
    with counter; exa'
- id: FR-029-AC-21
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-806): `run_terminal_value` has no proof-subject or proof-strength
    input and `TerminalPairError` has no variant for a bounded-shadow outcome; adding
    either to the map is a deliberate, tested change rather than a wildcard arm.'
- id: FR-029-AC-22
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-806). The strength set enumerated by the map equals FR-028-AC-17's
    closed set; adding a strength without its projection and outcome row fails the
    build or test rather than entering a wildcard mapping.
- id: FR-029-AC-23
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED/GATED (IR-806/QSL-640). A stopped backend run retains its ordinary
    timeout/memory inconclusive classification; a stopped refinement retains FR-028-AC-24's
    CeilingReached projection and recorded ceiling. Neither stop, nor QSL settlement
    resource refusal, becomes completed NotExhausted evidence or Tested.
- id: FR-029-AC-24
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED/GATED (IR-806/QSL-640). After common claim/source/node/bounds/O-09
    validation, retained refinement_failed/Disagreed evidence for the same valid claim
    yields Failed/CgDefect for verified, falsified, inconclusive and cover-unsatisfied
    backend outcomes, including zero checks, an operand field x declared Int[0, 9]
    but supplied as x: 12, missing/fault native observation and a falsified replay
    agreeing on its retained case; no early operand/native setup refusal loses this
    disagreement, which precedes resource, vacuity and agreement rows. Missing observations
    are never fabricated.'
- id: FR-029-AC-25
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-806). With no retained refinement disagreement, every bounded_shadow
    inconclusive reason maps through the ordinary rows: vacuous proof and cover-unsatisfied
    are Proved0; timeout is Incomplete(TimedOut); memory and unwind exhaustion are
    Incomplete(ResourceExhausted); no-verdict and missing-summary are Failed; failure-without-counterexample
    takes the cause rows (FR-029-AC-32 to FR-029-AC-35). No vacuous outcome is coerced
    into verified evidence.'
- id: FR-029-AC-26
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED/GATED (IR-806/QSL-640). The converter preserves the actual QSL
    record category/cause: Proved0 remains inconclusive/KaniVacuousProof and is never
    promoted by its payload. Zero-count backend cases use AC-25, not an unreachable
    verified-count-zero row.'
- id: FR-029-AC-27
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED/GATED (IR-806/QSL-640). The distinct typed composite settlement
    input refuses a missing settlement or another claim's node/run/operation/operand/domain/limits/content
    binding with no value; ordinary source-predicate AC-15 does not consume or reject
    a valid verified-parity settlement as UnexpectedSettlement.
- id: FR-029-AC-28
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'IR-666 public converter delivered. For any full-identity-bound QSL `CompositeParityResult::Refused`
    report, CG preserves `report.terminal_value()`: non-fault refusal is Inconclusive(ReplayRefused)
    with QSL''s code, while `Fault` and `Admission(Fault)` are Failed. QSL `prepare`
    refusal precedes F-1; operand-admission and exact-comparison refusals follow F-1,
    so Disagreed wins over those later faults. CG verifies a reachable non-fault report
    by invoking QSL''s public facade with a valid wire request and a replay input-byte
    limit below that request''s size, then passing the actual report through its'
- id: FR-029-AC-29
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: A direct `KaniRunOutcome::Cancelled { source }` maps both IR source variants
    to QSL `IncompleteCause::Cancelled` with the matching exact cause, using the same
    exhaustive conversion as FR-030; supplying a replay settlement returns `UnexpectedSettlement`.
    The production cancellation path publishes only FR-030's result.
- id: FR-029-AC-30
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-806). A `bounded_shadow` outcome (verified, falsified, inconclusive
    or cover-unsatisfied) reaches only the composite parity route on the production
    path and never `run_terminal_value`, because `KaniRunOutcome` cannot tell a shadow
    from a production proof and a verified shadow outcome there would read as `Proved`,
    which FR-028-AC-17 forbids.
- id: FR-029-AC-31
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-806). The parity route fabricates no predicate replay and
    no terminal cause for a bounded-shadow outcome: its terminal value is the one
    QSL''s report carries under AC-17 and AC-19 to AC-27.'
- id: FR-029-AC-32
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-848). A `FailedWithoutCounterexample` run with cause `out_of_memory`
    maps to `Incomplete(ResourceExhausted)`, and the same reason with cause `properties_failed`,
    `exit_code` (any code), `unknown` or no cause maps to `Failed`. Over a single-run
    stand-in that exits 1 and exports a `Failure` report for `proofs::check` with
    no checks and one `error_details` entry with exit status `out_of_memory`, the
    evidence passed to `run_terminal_value` gives `Incomplete(ResourceExhausted)`;
    the same report with exit status `exit_code_64`, `cbmc_exploded`, `properties_failed`,
    a null status or no `e
- id: FR-029-AC-33
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-848). A `FailedWithoutCounterexample` run with cause `timeout`
    maps to `Incomplete(TimedOut)`: the single-run stand-in of FR-029-AC-32 with exit
    status `timeout` gives `Incomplete(TimedOut)`. The batch member `c::check` of
    FR-017-AC-39 (a `Failure` with no checks and exit status `timeout`, classified
    `TimedOut`) keeps `Incomplete(TimedOut)` through the timed-out row, and the member
    `d::check` (a `Failure` listing a failed check with no playback, exit status `timeout`,
    classified `FailedWithoutCounterexample` with cause `timeout`) gives `Incomplete(TimedOut)`
    through the cause.'
- id: FR-029-AC-34
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-848). CG''s own memory stop (the memory-exhausted reason)
    and a `FailedWithoutCounterexample` run with cause `out_of_memory` give the same
    terminal value, `Incomplete(ResourceExhausted)`, so one exhaustion has one value
    whoever observed it. The cause is read for the failure-without-counterexample
    reason only: every other outcome and reason maps identically whatever cause is
    passed, including a cause passed with `NoVerdict` or `MissingCoverSummary`.'
- id: FR-029-AC-35
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-848). The cause never creates a verdict. Over every combination
    of outcome, reason and cause (absent, `properties_failed`, `timeout`, `out_of_memory`,
    `exit_code`, `unknown`), and with the replay settlement each outcome takes: a
    `falsified` outcome with a reproduced replay is `Refuted` whatever the cause;
    the cause never turns a non-`Refuted` result into `Refuted`, and never turns any
    result into `Proved` (`Proved` appears only for the rows FR-029-AC-1 and FR-029-AC-2
    give). The `NoVerdict` and `MissingCoverSummary` reasons map to `Failed` whatever
    the cause. Classification is unch'
- id: FR-029-AC-36
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-848). The rationale of this requirement states that Kani reports
    any `SIGKILL` of CBMC as `out_of_memory`, that an `out_of_memory` cause is accepted
    as `Incomplete(ResourceExhausted)` without corroboration, and that the raw cause
    stays in the evidence.
- id: FR-030-AC-1
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: Every pair (`KaniOutcomeKind`, replay settlement) that the map's input
    can express and whose retained proof count fits QSL's `u32` range maps to exactly
    one `TerminalValue`; an out-of-range proof count returns only typed `ProvedCountOutOfRange`.
- id: FR-030-AC-2
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Refused`, `InvalidInput` and `IncompleteInput` map to `Declined` with
    `ProofRefusalCause::Refused`, `InvalidInput` and `IncompleteInput` respectively,
    so no refusal cause is lost, and each carries the outcome''s `Std001Code` unchanged
    as `DeclineCode::Std001`, including a code STD-001 does not register.'
- id: FR-030-AC-3
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`TimedOut` and `ResourceExhausted` map to their distinct `Incomplete`
    causes. `Cancelled { source: Requested }` and `Cancelled { source: Deadline }`
    map to `IncompleteCause::Cancelled` carrying the corresponding `quire_exact::CancelCause`,
    with no source inferred from `code`, `source_id`, `context` or elapsed time.'
- id: FR-030-AC-4
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Proved` with three retained SUCCESS checks maps to `Proved { basis: Checks
    { success_checks: 3 }, certification: Certified }` without a separate caller count;
    a retained count exceeding QSL''s `u32` range returns typed `ProvedCountOutOfRange`
    with no terminal value; `Counterexample` with reproduced replay maps to `Refuted`.
    Contract IR refuses a zero-check `Proved`.'
- id: FR-030-AC-5
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Inconclusive` with cause `kani_vacuous_proof` maps to `Proved { basis:
    Checks { success_checks: 0 }, certification: Certified }` (category inconclusive/vacuous);
    Contract IR refuses every other `Inconclusive` cause before mapping.'
- id: FR-030-AC-6
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: No outcome maps to `Tested`.
- id: FR-030-AC-7
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: The map is one `match` over the pair (`KaniOutcomeKind`, replay settlement)
    with no wildcard arm.
- id: FR-030-AC-8
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Unavailable` with cause `kani_checker_absent` maps to `Unsupported(SolverAbsent)`
    and with `kani_backend_absent` maps to `Unsupported(BackendAbsent)`; Contract
    IR refuses every other `Unavailable` cause before mapping.'
- id: FR-030-AC-9
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a replay disagreement maps to `Inconclusive(ReplayParity)`,
    and with a non-fault `ReplayRefusal` maps to `Inconclusive(ReplayRefused)` carrying
    `ReplayRefusal::code()` of that refusal.'
- id: FR-030-AC-10
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a fault, walked through every wrapper FR-029-AC-10
    lists, and with each CG-raised failure FR-029-AC-11 lists, maps to `Failed`.'
- id: FR-030-AC-11
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: Across every replay settlement other than reproduced, `Counterexample`
    maps to a value other than `Refuted`.
- id: FR-030-AC-12
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a non-fault `CallSiteRefusal` or a `DependencyLockError::Input`,
    each bare and wrapped, maps to `Inconclusive(ReplayRefused)` carrying that refusal''s
    QSL catalog code, never to `Declined`.'
- id: FR-030-AC-13
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a `DependencyLockError::Input` that carries QSL''s
    `DuplicateIdentity` refusal (code `invalid_package`), as a lock whose only defect
    is a repeated library identity produces it (FR-016-AC-24), maps to `Inconclusive(ReplayRefused)`
    carrying `invalid_package`.'
- id: FR-030-AC-14
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: A `Counterexample` given no replay settlement is refused with `TerminalPairError::MissingSettlement`,
    and each other kind given a settlement is refused with `TerminalPairError::UnexpectedSettlement`;
    neither returns a terminal value.
- id: FR-030-AC-15
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: PARTIAL (IR-666 direct public report controls; IR-806 must reach the same
    rows from a CG-built request over the original proving context). For an identity-valid
    IR `Counterexample` composite claim, F-1 Disagreed yields Failed/CgDefect even
    with a native fault or invalid operand; without Disagreed, native Incomplete and
    ExecutionFault each yield a binding-checked GeneratedFault/Failed report with
    the actual NativeCause even when an operand would refuse, admission or exact limits
    would stop, or refinement is CeilingReached. CG asserts the typed result and terminal
    value; QSL FR-358 owns the inte
- id: FR-030-AC-16
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: PARTIAL (IR-666 direct public report controls; IR-806 must reach the same
    rows from a CG-built request over the original proving context). With valid identity,
    no Disagreed and Completed native, an out-of-domain operand yields F-3 RefusedInput/ReplayRefused
    with its index and QSL code even with CeilingReached; a request accounting limit
    reached during admission yields F-4 Incomplete(ResourceExhausted)/Admission with
    the counter; an exact limit reached after admission yields F-5 Incomplete(ResourceExhausted)/ExactEvaluation
    even with CeilingReached; with sufficient limits, CeilingReached yields
- id: FR-030-AC-17
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: PARTIAL (IR-666 F-7 and binding controls; IR-806 original-artifact path
    planned). With valid identity and all earlier rows absent, changing only the retained
    shadow verdict or pair count yields F-7 Diverged/Failed/CgDefect; agreement yields
    Inconclusive(ScalarAgrees) with the CompositeEquality claim and Equality outcome.
    A binding-valid QSL `prepare` non-fault `Refused` report precedes Disagreed and
    yields Inconclusive(ReplayRefused) with QSL's code. `Refused` from later admission
    or exact comparison follows Disagreed; if QSL returns `Fault` or `Admission(Fault)`,
    its bound report yields Faile
- id: FR-030-AC-18
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: Every mapped Kani `Proved` outcome carries `ProofBasis::Checks` with its
    retained positive count and `Certification::Certified`; the `Inconclusive(kani_vacuous_proof)`
    outcome carries zero checks with the inconclusive/vacuous category. No other Kani
    outcome carries basis or certification.
- id: FR-030-AC-19
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: For both `BoundedComplete { depth }` and `Inductive { depth }`, an SMT
    proof whose certificate FR-314 verifies maps to `Proved` with the same basis/depth
    and `Certified`; no certificate or an `Unverifiable` unchecked rule maps to `Proved`
    with the same basis/depth and `Uncertified`, retaining the unverifiable locus
    when present. No SMT proof maps to `Trusted`.
- id: FR-030-AC-20
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: For either SMT basis, a foreign query, wrong certificate shape, invalid
    checked proof step or missing final empty clause rejected by FR-314 maps to `Inconclusive(CertificateRejected
    { rule, at })` retaining the exact rejection; none produces `Proved`, basis or
    certification. When cvc5 and Z3 also disagree, `CertificateRejected` remains the
    one terminal cause while the parity failure and both verdicts are retained alongside
    it.
- id: FR-030-AC-21
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: SMT/cross-check disagreement without certificate rejection keeps both verdicts
    and settles inconclusive; agreement does not change certification; missing Z3
    retains its warning and cvc5 settlement. An SMT counterexample becomes `Refuted`
    only after FR-197 replay reproduces it; replay disagreement, non-fault refusal
    and fault map to their distinct non-proof results.
- id: FR-030-AC-22
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: Every expressible accepted SMT backend/checker/replay settlement produces
    one terminal result, never `Tested`; no non-`Proved` result carries basis or certification.
    Negotiation refusals produce no terminal from this map, and neither SMT nor Kani
    map consumes the other backend's outcome.
- id: FR-034-AC-1
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Before the exclusive caller lease, matching helper identity, verified monitor/INIT/peer,
    observer readiness and positive typed Dispatch are established, no production
    backend instruction is created. Caller SIGKILL, abort or OOM-equivalent forced
    death during initialization may start only bounded guardian bootstrap, which exits
    without backend on absent lease or authorization.
- id: FR-034-AC-2
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Before any M spawn, O is positively verified outer PID 1 with armed parent-death
    protection and original-exclusive bootstrap authority. Unclaimed inner I is contained
    by O even if M dies before internal child_wait handoff or startup-info reader
    loss aborts M. Owned outer pidfd termination confirms teardown; missing authority
    refuses before spawn. No host scan, monitor/group exit or successful signal proves
    cleanup.
- id: FR-034-AC-3
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: In ClaimedBootstrap, InitReady or Dispatched, original-caller lease EOF
    causes guardian cancellation/exit and confirmed INIT termination; Dispatched descendants
    are torn down by the kernel. The test controller observes dead-caller cleanup
    without manufacturing that caller's outcome or evidence.
- id: FR-034-AC-4
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Mutual authority is the specific anonymous connected pair: guardian checks
    the actual mapped creator UID and only the original caller holds its executor
    endpoint. The bounded executor binds kernel credentials of Ready''s actual sender
    to claimed host INIT/start/pidfd/parent/namespace, not socketpair creator credentials
    alone. Guardian''s creator PID0 is not a host identity. A foreign actor cannot
    rebind a public name or supply replacement lease/Dispatch; another pair, mismatched
    UID or replayed authority refuses.'
- id: FR-034-AC-5
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: C retains actual unreaped Child L; L/O/M/I ownership is verified through
    host pidfds, start/parent/namespace identities and actual outer/inner PID 1. O
    owns actual M Child and I is M's inner child. Reused, stale, incomplete, translated
    incorrectly or dead identity refuses before Dispatch.
- id: FR-034-AC-6
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: The owned tree memory observer is ready before typed Dispatch; failed observer
    preparation releases no backend instruction.
- id: FR-034-AC-7
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Bounded inner startup/identity recovery is required before Dispatch. M
    death/reparenting or malformed/missing info cancels through retained outer INIT
    authority and yields typed setup refusal; complete outer termination is independently
    confirmed. No host scan or gate EOF authorizes a backend.
- id: FR-034-AC-8
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: After Dispatch, original-caller death cancels double-fork/reparented, setsid,
    late-born and nested-PID-namespace descendants through kernel-owned INIT teardown,
    including descendants absent from previous samples. Observed-PID lists or normal-operation
    host scans are not cancellation authority.
- id: FR-034-AC-9
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Cancelling one owned namespace leaves concurrent independent runs and unrelated
    host children unaffected.
- id: FR-034-AC-10
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Normal backend completion, backend exec failure, explicit cancellation,
    timeout, memory excess, capture failure and observation failure each require confirmed
    owned teardown before any proof conclusion. Backend exit or valid success output
    alone never authorizes acceptance.
- id: FR-034-AC-11
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Guardian SIGKILL, abort or OOM-equivalent death during bootstrap, InitReady
    or immediately after Dispatch cannot release an unowned production backend: the
    guardian is INIT, and its death tears down that namespace. With a live original
    caller, guardian failure always yields a typed refusal and never verified/falsified
    evidence, even after confirmed teardown and beside a valid success report.'
- id: FR-034-AC-12
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Every startup refusal, completion and cancellation settles actual owned
    roles before conclusions. Anonymous controls and report pipe/memfd have no persistent
    names and end at final close. All report-owner death reclaims kernel report backing
    without Drop/unlink; other assigned artifacts retain surviving-owner cleanup and
    truthful all-owner-death limits. No dead actor is claimed to reap or unlink.
- id: FR-034-AC-13
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: The bounded executor requires an explicit path to this package's actual
    Cargo executable quire-kani-guardian, built and supplied with the library from
    the same package source/build inputs. A missing, non-executable or unusable helper
    gives a typed setup refusal before backend Dispatch, with no PATH/global discovery,
    copied executable, shell substitute or alternate launcher. Cargo library dependency
    resolution alone is not helper delivery.
- id: FR-034-AC-14
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Setup docs require the matched package helper roles, safe caller-context
    user/PID/private-mount permissions, verified mapping/setgroups/private proc, nested
    installed bwrap and pidfd/observer facilities. The compiled caller security profile
    is checked, not inferred from binary mode. Caller-profile restriction can refuse
    runs formerly available through bwrap-only permission; ordinary-caller availability
    requires actual measurement. Missing capability gives typed pre-Dispatch refusal
    without host-policy mutation or weaker mode. Added L/O roles and bounded controls/storage
    are charged to existing
- id: FR-034-AC-15
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Typed private controls reject malformed and unknown fields and enforce
    finite encoded-byte, pending-message and startup-work bounds. Ancillary descriptor
    count/type is exact, received descriptors are CLOEXEC, and unknown/extra/truncated
    ancillary data refuses while closing all received descriptors. Invalid/overlimit
    control or EOF cancels or refuses rather than authorizing Dispatch.
- id: FR-034-AC-16
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Only the original caller holds the pair's executor endpoint. Guardian receives
    only its control end as fd0 through safe CLOEXEC child-only mapping, borrows it
    with a safe descriptor API and receives actual backend stdin separately as OwnedFd
    through safe ancillary rights. Backend fd0 restores that original stdin or closed
    state, while the control/lease descriptor is excluded from backend and unrelated
    exec inheritance. No arbitrary raw-fd adoption or unsafe exception occurs.
- id: FR-034-AC-17
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Backend receives unchanged raw argument bytes except the explicitly allocated
    internal report locator: harness options then -Z unstable-options --export-json
    /proc/self/fd/N with actual mapped N >=5. Evidence records that exact argv. Inherited
    stdin, inherited/overridden environment and cwd remain unchanged.'
- id: FR-034-AC-18
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Guardian diagnostics and control use separate channels from backend stdout/stderr
    captures and cannot contaminate reports; existing bounded capture and capture-failure
    behavior remain authoritative.
- id: FR-034-AC-19
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Original identity ceilings and existing outcome classifications remain
    authoritative after confirmed cleanup: memory excess and wall expiry stay distinct,
    ambiguous live-worker RSS refuses, and ordinary completed/refused/falsified reports
    retain their meanings. Native refinement uses this ownership path when its FR-028
    AC-24 typed entry is implemented, with unchanged refinement class/evidence rules.'
- id: FR-034-AC-20
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Guardian connection, startup, identity verification and Dispatch use the
    original monotonic identity deadline without resetting it. An already expired
    deadline causes no Dispatch and settles inconclusive with the timed-out reason
    as FR-028 AC-2 states; actual identity-deadline expiry during setup or execution
    uses the same classification.
- id: FR-034-AC-21
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Guardian connection and handshake have a finite setup cap within the remaining
    original deadline. Cap expiry while that deadline remains live is a typed setup
    refusal, distinct from identity-deadline timeout.
- id: FR-034-AC-22
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Control/capture shutdown and cleanup observation waits have finite bounds.
    Unconfirmed termination refuses with a live caller rather than accepting a proof
    or claiming physical disappearance of an uninterruptible task.
- id: FR-034-AC-23
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Caller-death fixtures use the actual matching normal helper and shared
    production functions/order with bounded prefix selection as input only. Actual
    caller/group death is followed by independent original owned M/I pidfd termination,
    no backend Dispatch/marker and no leaked owned inner processes before emergency
    cleanup. Unclaimed Bootstrap supplies no invented INIT pin or teardown credit;
    AC-31 remains independently owed. Cached prefix or gate-retained metadata proves
    no live gate. Feature-off positive post-Dispatch controls remain required.
- id: FR-034-AC-24
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Actual lease-EOF cancellation passes independent original I termination
    and no backend Dispatch/marker before escalation, plus death of a positively acknowledged
    pinned worker after Dispatch. Ignored EOF, skipped actual lease close, removed
    positive Dispatch, non-INIT watcher, broken session isolation and startup-order
    mutants independently fail their genuine predicates; later cleanup cannot rescue
    them. The immediate private LeaseCloseObservation remains control flow, not a
    stored certificate. Stopped-I pending-authorization coordination remains owed
    without close/publication snapshots or ord
- id: FR-034-AC-25
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Before backend Dispatch, the helper handshake matches the actual running
    CG library's build, protocol and lifecycle-capability identity against the actual
    invoked first-party executable. A stale helper or changed lifecycle implementation
    refuses, even if a caller supplies a matching version label or digest. Expected
    identity derives from actual library/helper build artifacts, not caller assertions
    or manually maintained tracking pins.
- id: FR-034-AC-26
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: The guardian session/group is distinct from the original caller before
    Ready/Dispatch. Real feature-on caller-group deaths and feature-off positive post-Dispatch
    death controls require independent owned termination/no backend/no leaked processes.
    Unclaimed Bootstrap cannot infer I death or held gate from monitor death or cached
    phase; AC-31 remains separately mandatory. Claimed I termination, operational
    lease cleanup and directly killed guardian kernel teardown remain required.
- id: FR-034-AC-27
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: guardian-test-support is off by default, absent from default features,
    and exposes exactly one documented fixture operation only when explicitly enabled.
    A feature-off consumer cannot use that operation. No public lease, process-ownership
    handle, cancellation entry or cleanup-deferring callback is exported. The private
    initialized death witness transfers exactly one actual owned monitor or validated
    INIT pin, with its typed authority; BeforeMonitor transfers none. Reporter startup
    stdout is safely duplicated into a non-stdio CLOEXEC OwnedFd and itself marked
    CLOEXEC before any spawn; every chi
- id: FR-034-AC-28
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Fixture prefix selects unchanged production transitions, never certifies
    live state. Actual original identity/right custody, independent pre-escalation
    I/worker/marker checks and immediate unchanged cleanup on success/refusal/error
    replace stored stage/close/publication certificates. Intentional C/group self-death
    follows complete bounded operational pin delivery without ACK/pause or fabricated
    result. Missing identity, observation or genuine coordination fails; unavailable
    exact early-release observation remains UNBACKED. No stored record or later cleanup
    repairs an oracle.
- id: FR-034-AC-29
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: The packaged caller fixture and real helper link the same normal library
    artifact through consumer-manifest package selection with matching target/profile/features/compiler
    flags. Separate named feature-off/on invocations avoid self dev-dependency feature
    unification. No cfg-test library or identity override is accepted. The bounded
    executor refuses feature mismatch in both directions before Dispatch; feature-off
    consumer compilation verifies absence of the fixture operation.
- id: FR-034-AC-30
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: CG publishes the test-only feature contract and allocates production-driver
    dependency-edge exclusion to IR-649's QSL driver work. The contract requires all
    downstream production-build profiles to reject direct or transitively unified
    guardian-test-support. CG inspection verifies the published allocation and checks;
    downstream assertion evidence is owned by IR-649.
- id: FR-034-AC-31
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Real production fixtures fail the merged PR #295 recipe at exact post-clone/pre-internal-child_wait
    caller death and natural M/info-reader failure, and pass only with confirmed actual
    outer INIT and unclaimed inner/escaped descendant termination. Exercise C/group,
    L, O and I death before/after parent-death arm with positive barriers; pre-arm
    has no inner child and exclusive EOF gives bounded refusal. Original-exclusive-lease
    race, actual host/outer identity translation, mapping/private-proc capability
    refusal, real observer, ordinary-caller capability measurement, spawning-thread
    lifetime and '
- id: FR-034-AC-32
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: The bounded O event loop collects anonymous report pipe into memfd without
    a backend/collector completion wait cycle. A hard 16 MiB plus one detection-byte
    retention bound and the defined owned_RSS + caller_run_buffers + page-rounded
    F_GETPIPE_SZ + page-rounded pre-reserved memfd maximum comparison applies while
    writing against the original ceiling; unmapped shmem is not zero. Beyond cap yields
    owned cancellation and single-run KaniRunOutcome::Inconclusive with MemoryExhausted;
    batches keep whole-batch memory-exhausted refusal, with no member classified.
    Evidence names the report cap separatel
- id: FR-034-AC-33
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Authenticated Completed precedes original-lease close, confirmed inner
    teardown and M termination/reap closing ALL pipe writers, including reopened procfd
    and descendant copies. Bounded actual-EOF drain precedes immutable WRITE/GROW/SHRINK/SEAL
    seals, consumer F_GET_SEALS verification and stable actual OwnedFd reads, all
    under the original deadline. Separate final control/report delivery remains live
    after lease close. Pre-Completed reader failure cancels O/I. All storage-owner
    death reclaims backing at final close without persistent report residue or surviving-owner
    dependence. Inheritance, s
- id: FR-034-AC-34
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Live-C production close_lease_and_observe retains L/O/M/I ownership and
    final report control independently of consumed original lease. No outer kill,
    bootstrap EOF or parent-death cascade masks AC-24's independent pre-escalation
    EOF and termination checks; ignored-EOF still fails its named actual-behavior
    predicate before cleanup. Missing conservative backing accounting or immutable
    kernel report sealing gives typed refusal and owned cancellation. All roles, private
    namespace/proc setup, controls, report backing and observation use existing whole-run
    ceilings and original deadline, with no res
- id: FR-034-AC-35
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. L creates private network/root before O, O validates before
    M, and nested bind / / refers to confined O root. Safe backend-only seccomp/privilege
    installation before Dispatch preserves the same positively owned PID and original
    recipe, with no unfiltered Dispatch or extra surviving process/ownership/deadline;
    policy installation failure is pre-Dispatch, actual recipe exec/failure is post-Dispatch
    with existing bounded handling/no fabricated evidence; trusted installer starts
    in sanitized loader environment and restores original backend environment only
    at filtered recipe exec; I
- id: FR-034-AC-36
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. C inventories actual backend fd0/fd1/fd2. Real OriginalStdin::Open
    socket input and failed fstat inspection refuse before Dispatch; actual production
    fd1/fd2 are capture pipes verified by mapping/inventory Analysis, not caller socket
    cases. Caller fd0..2 stability throughout setup is a trusted precondition; observed
    capture inconsistency refuses, with no claim to detect every ambient mutation.
    C internally captures/pins OriginalStdin before child/control fd reuse, without
    a public request field; authoritative initial absence/CLOEXEC yields Closed, while
    later Open inspection EBA
- id: FR-034-AC-37
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Trusted I retains its exclusive lease through Dispatch;
    O retains separate final delivery after original lease close. All bootstrap/ownership/report/reporter
    controls remain owned/CLOEXEC outside intended mappings and unavailable to arbitrary
    backend, sibling exec and descendants. I confirms non-dumpability after final
    credentials and backend cannot hold or regain CAP_SYS_PTRACE in I owning user
    namespace; real backend /proc/1/fd, pidfd_getfd and ptrace gates prove protection
    independently of host Yama. Actual leaked-control/protection mutants fail before
    emergency cleanup; rest
- id: FR-034-AC-38
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Named fixed SETTLE_RESERVE R is measured/rounded; its finite
    effective reserve stays inside original whole T; short finite ceilings stay admitted
    with R_eff=min(R,T/2), no minimum-budget cause. At finite workdeadline=T-R_eff
    stop/cancel; normal returns positively settle/reap all owned roles, captures and
    existing creator thread by original T before conclusions; original None/overflow
    never-elapsing work remains admitted and FIRST actual stop starts one R settlement
    deadline. Confirmed workdeadline expiry preserves existing TimedOut classification
    naming T; unconfirmed settlement
- id: FR-034-AC-39
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Native guardian/backend-installer policy support is source/cfg
    Analysis; unreachable unsupported-native support is not a fabricated runtime Test.
    Actual installation failures are separate production Tests when available, otherwise
    unavailable/UNRUN with no Test completion credit. Planned mandatory admission
    context distinguishes BackendIpcExclusion capability support/filter installation
    from TrustedOwnerProtection privilege installation, retaining typed cause provenance
    without message parsing or fabricated errno. The policy terminates the calling
    process on an unsupported sysca
- id: FR-034-AC-40
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Only declared trusted role/operation/phase causes use the
    cross-role representation. Actual kind/errno, independently authenticated optional
    finite provenance and loss survive privately; public projection applies only to
    existing io::Error-cause-bearing refusals. Post-admission observation/settlement
    detail-only results keep their existing shape/mapping, private finite facts/loss
    and diagnostic-only text, with no public marker/carrier/downcast promise or Tool
    reclassification; generic original custom sources/chains are lost explicitly,
    including valid original boxed declared ope
- id: FR-034-AC-41
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). For a named native-accounting conformance claim,
    the analysis identifies the exact consumer build configuration and compiler actually
    used, including resolved profile/features/target/flags/runtime/allocator. A proof
    for a different configuration cannot establish that claim.
- id: FR-034-AC-42
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). The proof is derived from GNU objdump or llvm-objdump
    disassembly using stable compiler inputs of the actual final linked executable
    after LTO/linking. A crate-level or pre-link emit-asm/emit-obj result cannot pass
    as that final-artifact proof.
- id: FR-034-AC-43
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). Every named operation and thread-entry root maps
    to final address ranges or conservatively covered host frames, and every reachable
    direct or indirect call target is accounted for. An unresolved reachable indirect
    target leaves the native-workspace bound UNPROVEN.
- id: FR-034-AC-44
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). A reachable recursive call cycle, including one
    through an external callee, leaves the native-workspace bound UNPROVEN under this
    nonrecursive method.
- id: FR-034-AC-45
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). The native-stack bound sums per-thread maxima for
    every concurrently live thread instance carrying named state, accounting for thread-entry
    roots, actual tail-call replacement and dynamic adjustments. Unknown instance
    counts or lifetimes fail the claim. An unresolved frame or adjustment cannot be
    filled by a guessed reserve.
- id: FR-034-AC-46
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'PLANNED/UNRUN (IR-682). The analysis covers the actual linked panic strategy:
    reachable unwind/landing-pad and cleanup paths contribute to the stack bound,
    or the final artifact establishes their absence under abort. Assuming the strategy
    from a source profile default fails.'
- id: FR-034-AC-47
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). Each relevant external library/runtime/libc/allocator
    frame has an explicit finite bound source matching the actual loaded runtime ABI/configuration.
    A missing source leaves the whole named-workspace claim UNPROVEN without changing
    the existing incidental-allocation exclusions.
- id: FR-034-AC-48
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). The named heap inventory covers actual returned
    Rust capacities times element size, metadata and simultaneous lifetimes, excluding
    incidental allocator chunk overhead. A larger actual capacity or overlapping temporary
    omitted from the checked caller-buffer sum fails the conformance claim.
- id: FR-034-AC-49
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). A change to the source head, resolved consumer
    configuration/compiler, resulting executable or loaded runtime/bound premises
    invalidates the former proof and requires analysis again. A stale proof or merely
    matching caller/helper artifact does not satisfy native-accounting conformance.
- id: FR-034-AC-50
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). Missing or incomplete proof, or a proven bound
    exceeding the independently declared runtime charge, leaves the configuration
    UNPROVEN and fails the conformance gate; a missing actual named charge retains
    the existing runtime refusal. This build gate introduces no runtime refusal/public
    capability or evidence of completed Analysis; the initial release configuration
    and other eligible configurations remain UNPROVEN until their own proofs exist.
- id: FR-034-AC-51
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Given genuine live-I construction preconditions, the child's
    own acknowledgement establishes release-dependent birth after the actual completed
    ordinary O sample.
- id: FR-034-AC-52
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual retained-I pidfd termination confirmation strictly
    precedes matching kernel report seal; independently observed fabricated-live-I
    and late-poll controls fail without stored-record or ordinal proof.
- id: FR-034-AC-53
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual retained M consuming Child wait/reap strictly precedes
    matching kernel report seal; independent fabricated-reap and late-reap controls
    fail without cached Some, copied history or ordinal proof.
- id: FR-034-AC-54
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. On the independently grounded supported cooperative cancellation
    path, the actual original O gate owner retains the gate through actual I confirmation
    before actual close; early-close and omitted-confirmation controls fail without
    marker, stage or refusal-label proof. Genuine unavailable confirmation at the
    original cutoff follows AC-38 CleanupUnconfirmed with explicit actual-owner abandonment/right
    disposal and no ordering or cleanup credit, never a renamed fatal close or diagnosis
    of kernel fault. Healthy restored order and unavailable-confirmation refusal are
    separate control
- id: FR-034-AC-55
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Independently checked final-source Analysis establishes
    contained teardown authority through actual namespace INIT death and retained
    owner settlement, with no sampled-PID membership kill path; the separate AC-24
    non-INIT-watcher runtime adverse/restored control remains owed.
- id: FR-034-AC-56
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Published-source Analysis establishes O's final complete
    accounting after actual I confirmation, retained M reap, writer EOF and immutable
    seal for successful report Commit, with all later due ticks preserved; earlier
    final accounting invalidates literal-FINAL-oracle retirement.
- id: FR-034-AC-57
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: "PLANNED/UNRUN (IR-687). When L awaits its first O arm result, the newly\
    \ allocated authenticated O\u2192L OperationalFailure carries zero ancillary rights."
- id: FR-034-AC-58
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). L accepts that negative only from its actual retained
    O Child PID with kernel sender UID/GID0 and the original build/run/source/state/stamp
    authority.
- id: FR-034-AC-59
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: "PLANNED/UNRUN (IR-687). For L\u2192C AwaitArm negatives, C rejects missing,\
    \ extra, wrong-type or substituted rights through owned cleanup, without a zero-right\
    \ fallback."
- id: FR-034-AC-60
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). The negative O pidfd grants C only exact O termination
    observation and existing owned-cancellation signalling, while L retains its direct
    Child/wait obligation.
- id: FR-034-AC-61
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). Negative capability custody grants no Armed, namespace/network,
    phase, report or Dispatch admission.
- id: FR-034-AC-62
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). C adopts the authentic O producer stop before L
    wait and preserves the original earliest cutoff, including absent work deadline,
    without receipt-time reset.
- id: FR-034-AC-63
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). C exposes a negative original-cause result only
    after actual O Child wait at L and required L/capture/creator/control-EOF settlement
    under that cutoff.
- id: FR-034-AC-64
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). The new negative routes reuse existing charged
    controls and ceilings without a new envelope type, ACK, window, cap or public
    category.
- id: FR-034-AC-65
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). Before first-byte COMMIT, a fresh complete exhausting
    pre-send tick retires an unsent genuine OperationalFailure into existing OwnerStop
    under existing precedence, even when the failure occurred earlier.
- id: FR-034-AC-66
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: "PLANNED/UNRUN (IR-687). Only the allocated L-local pre-arm negative route\
    \ forwards the same bytes to C with exactly one clone of L\u2019s retained O pidfd."
- id: FR-034-AC-67
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). C authenticates the L-forwarded negative only in
    AwaitArm through the actual L chain and original O/run/build bindings.
- id: FR-034-AC-68
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). C does not use the negative pidfd for pidfd_getfd,
    pidfd setns, process_madvise, other-actor authority or numeric-PID reopening.
- id: FR-034-AC-69
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). Missing or abnormal whole-chain settlement overrides
    the provisional negative with existing CleanupUnconfirmed and no execution evidence.
- id: FR-034-AC-70
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). The first emitted terminal byte irreversibly fixes
    the candidate; partial/full publication cannot be rewritten, spliced or followed
    by a second terminal frame.
- id: FR-034-AC-71
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). O includes every due observation and a genuine
    fresh complete named-input tick before initial terminal emission; failed current
    input never borrows historical/default peaks.
- id: FR-034-AC-72
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). After COMMIT, later successful complete exhaustion
    stays in actual private O accounting until genuine settlement; retention is an
    Analysis obligation with no current runtime seam or Test credit.
- id: FR-034-AC-73
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). A claimed-startup negative publication remains
    provisional until positive actual I termination and separate real M Child reap,
    with required writer EOF/final due accounting before normal O return.
- id: FR-034-AC-74
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: "PLANNED/UNRUN (IR-687). The negative\u2019s absent peak field supplies\
    \ no zero measurement, no-history claim or current-validity substitute."
- id: FR-034-AC-75
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). A later genuine observation/collector/transport
    error after COMMIT remains owned through bounded cleanup and prevents normal O
    Code0 acceptance through existing CleanupUnconfirmed.
- id: FR-034-AC-76
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). A genuine admitted open-file-description Test oracle
    distinguishes L's retained-pidfd clone from independently opened same-O and foreign
    pidfds; inode equality supplies no clone evidence.
- id: FR-034-AC-77
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). On a fresh complete Exhausted tick after first-byte
    COMMIT, O immediately initiates the same actual owned cancellation of live claimed
    I and retained M without waiting for C lease closure, preserving the original
    cutoff and committed bytes.
- id: FR-034-AC-78
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. C admits actual original O Armed only through genuine retained
    O/build/run authentication and original owned rights; wrong sender/run or unvalidated
    capability cannot grant positive admission.
- id: FR-034-AC-79
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual production control reception rejects malformed, partial
    or forbidden-right traffic without granting positive phase/Dispatch/report authority;
    a stored event or EOF never supplies a missing authenticated control.
- id: FR-034-AC-80
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Genuine original authenticated I Completed alone establishes
    completion reception; M exit, EOF, phase or copied metadata cannot substitute.
    The separate stored completion certificate is retired.
- id: FR-034-AC-81
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Every surviving actual simultaneous named fixture/control
    storage and pipe/backing reservation is charged before exposure; retired certificate
    storage is removed from charges only after its actual allocation disappears.
- id: FR-034-AC-82
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual supported-source/artifact Analysis and mechanical
    paired consumer/helper checks establish feature-off absence and unchanged production
    frames/rights after certificate retirement.
- id: FR-034-AC-83
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. The original exclusive caller lease closes independently
    of retained monitor/INIT ownership and does not leak into children; a leaked alias
    cannot preserve authorization after actual caller death.
- id: FR-034-AC-84
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Every remaining fixture reporter/control/report endpoint
    is excluded from untrusted child/exec mappings and public control handles through
    actual descriptor checks.
- id: FR-034-AC-85
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual retained M consuming reap is not established by repeated
    or cached Child Some(status); an independent genuine-operation predicate rejects
    cache-only or fabricated success.
- id: FR-034-AC-86
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. O establishes same-M parent-only prior-WNOWAIT/FIRST-uncached-Some/post-ECHILD
    provenance under the original sole-waiter/disposition/history; independent checks
    reject fabricated, cached or missing consuming-operation facts without stored
    history as proof.
- id: FR-034-AC-87
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual I confirmation/M consuming reap precede report seal.
    On AC-54's independently grounded supported cooperative path, actual I confirmation
    precedes retained-gate close; late-operation/early-close mutants fail. AC-38 unavailable-confirmation
    abandonment at the original cutoff grants no seal, order or cleanup credit; a
    skipped poll, lost pin or owner defect is not a kernel-fault diagnosis. Producer
    ordinal certificates are retired.
- id: FR-034-AC-88
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. O preserves every due ordinary sample and existing control/cutoff
    transition without observation ACK, pause, stored publication permission or suppressed
    tick.
- id: FR-034-AC-89
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Genuine original I Completed reception and original stop
    adoption precede actual original lease close, without a separate stored completion
    certificate.
- id: FR-034-AC-90
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Any remaining optional fixture write failure does not terminate
    O or alter ordinary sampling/control/settlement/cancellation; it cannot rescue
    ignored EOF. The certificate pipe writer is retired, not activated by this condition.
- id: FR-034-AC-91
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Ordinary feature-on execution remains available without
    retired observation binding, endpoint, event emission or default identity.
- id: FR-034-AC-92
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Genuine adopted-orphan acknowledgement and release-dependent
    child birth after an actual complete ordinary O sample remain required without
    a stored sample-announcement certificate; missing real coordination is UNBACKED.
- id: FR-034-AC-93
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Every surviving feature-on named native workspace has separate
    supported paired-artifact finite-bound Analysis and an independent charge; sole-purpose
    certificate workspace retires only with its actual allocation.
- id: FR-034-AC-94
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. C disposes the defined pending claim through actual close
    of its raw I right without positive admission or capability; invalid prior traffic
    and live identity mismatches refuse, while the defined same-cursor negative receipt
    preserves the original cause only after authenticated delivery and whole-chain
    settlement.
- id: FR-034-AC-95
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual O retains the original report backing identity through
    collection, kernel sealing and final descriptor delivery; a substituted backing
    cannot satisfy the genuine report predicate.
- id: FR-034-AC-96
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Missing, malformed, wrong or unvalidated original O/build/run/control
    authority refuses without positive admission, retaining actual received-right
    custody and original cutoff; no default identity supplies authority.
- id: FR-034-AC-97
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Retired observation-binding echo creates no replacement
    comparison; actual original report backing equality and delivery/settlement authority
    remain required.
- id: FR-034-AC-98
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Feature-off paired consumers/helpers retain unchanged phase
    bytes/rights with no retired early field or branches; ordinary feature-on execution
    has no retired endpoint/binding requirement.
- id: FR-034-AC-99
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Any remaining optional O fixture write requires actual ignored/unblocked/untraced
    stable single-thread proof, including independent across-PID-namespace initial
    untraced evidence, pending/permission/configuration/accounting and real normal-helper
    controls. Certificate-writer retirement supplies no signal proof or activation.
- id: FR-034-AC-100
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Unsupported, malformed, unavailable or unproved profiles
    permit no optional fixture write, preserving pending signals, ordinary flow and
    original cutoff; no later cleanup or private-proc zero alone repairs eligibility.
- id: FR-040-AC-1
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A caller retains a live `Cancel`, sends its clone through a public single-run
    request, and the bounded launcher observes it before spawn and during an actual
    running backend. Pre-spawn cancellation starts no backend; running cancellation
    confirms owned teardown before evidence returns.
- id: FR-040-AC-2
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: In two live single runs, issue `Requested` then `Deadline`, and reverse
    the calls. Each run's actual launcher stop, `KaniExecutionEvidence.outcome`, constructed
    IR `Cancelled { source }` and QSL `IncompleteCause::Cancelled { source }` retain
    the first cause; a later event cannot change them. A hand-built IR outcome alone
    does not satisfy this criterion.
- id: FR-040-AC-3
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A run that reaches its execution ceiling with no cancellation event remains
    `TimedOut`. A `Deadline` event recorded only after completion, memory/capture
    stop or elapsed-ceiling stop does not turn that result into `Cancelled`.
- id: FR-040-AC-4
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: Completion before observed cancellation keeps the report-based result;
    measured memory exhaustion, memory observation failure, output over-limit, unread
    capture and unconfirmed cleanup keep their existing result/refusal and confirmed
    teardown rules when cancellation competes. A cancellation observed first and causing
    stop is not overwritten by later completion or elapsed time.
- id: FR-040-AC-5
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A multi-member group sharing one handle stops wholly on its observed cause,
    returns `BatchCancelled { source, members }`, and classifies no member; another
    group using a distinct handle retains its own result. Reversing the two cause
    calls changes the cancelled group's source and not the other group's evidence.
    Distinct handles never share one launcher.
- id: FR-040-AC-6
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: The single-run producer constructs an IR `KaniOutcomeKind::Cancelled {
    source }` from settled execution evidence, and its real call path reaches the
    FR-030 map. Mapping both IR source variants exhaustively yields the identical
    `quire_exact::CancelCause` inside QSL's `IncompleteCause::Cancelled`; neither
    metadata nor a copied enum determines the source.
- id: FR-040-AC-7
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: The direct `quire-exact::CancelCause` constructs `qsl-replay::IncompleteCause::Cancelled`
    through a type-checked exhaustive conversion, while all QSL imports remain through
    `qsl-replay`; no implicit cancellation control, source fallback, or compatibility
    overload is present.
- id: FR-041-AC-1
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A nonempty set of current-run generated module claims is checked against
    the current compiled-module census. An absent compiled module or empty set names
    the claim or condition and fails. Open pending a genuine current-build claim source.
    The former merge-base declaration comparison is retired by IR-1105.
- id: FR-041-AC-2
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: With two claimed modules sharing one `src/lib.rs`, a SUCCESS location inside
    only the first and an UNREACHABLE check inside the second reports the second `unreached`
    and fails. A SUCCESS check inside each passes this census; a SATISFIED cover,
    unknown location or non-SUCCESS status cannot supply the missing check.
- id: FR-041-AC-3
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The current build's transitive expectation/subject reachability yields
    the shared-helper intersection; the gate fails when the checked-in generated evidence
    omits or adds a helper, and records an explicit empty set when none is shared.
    For an exact operation, the expectation uses QSpec vectors or a model with no
    `quire-exact` call.
- id: FR-041-AC-4
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: Each claimed module and computed shared helper has a positive unmodified
    real-Kani proof and a source mutation that compiles, executes, and reports a failed
    proposition check. Deleting a control or making its mutant verify, fail to compile,
    time out, skip execution, or merely trip the dirty-tree guard fails the gate;
    evidence names the exact diff, command and failed check from this run.
- id: FR-041-AC-5
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A stubbed executor, predetermined outcome or build-only run produces no
    qualifying module evidence and cannot pass the gate.
- id: FR-041-AC-6
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The V2 claim is produced by the admitted V2 Boolean implication request
    through the production scalar renderer; its generated module and harness identities
    match the current-build claim source and compiled census. A V1 source, hand-authored
    module, missing module, changed identity or uncompiled claim is rejected by name.
    Open pending a genuine current-build claim source.
- id: FR-041-AC-7
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The V2 module exposes separate named subject and expectation roots in the
    current compiled call graph. The subject reaches the runtime implication operation;
    the expectation evaluates `!left
- id: FR-041-AC-8
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The unmodified generated V2 module has a location-attributed SUCCESS proposition
    and satisfied final cover with installed Kani. Replacing only its parsed subject-call
    expression with `false` compiles and executes the same harness, then yields an
    in-module FAILURE at the subject-versus-expectation assertion with in-domain playback
    where `left` is false; each computed shared helper has its own positive and falsifying
    control.
- id: FR-041-AC-9
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: After AC-6 through AC-8 pass in the current lane, the V2 implication module
    is included in the current-build claim census. The required `make kani-gate` run
    executes its real-Kani control and fails on any missing, skipped or failing claim
    or helper control. This completes the IR-1047 machinery delivery independently
    of the new Boolean state-transition claim in AC-10; implication proof credit does
    not discharge that transition obligation. Open pending a genuine current-build
    claim source.
- id: FR-041-AC-10
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A production V2 module implements the Boolean state-transition obligation
    originally described by FR-003@4, with precondition `input_current || state_pre`,
    postcondition `state_post`, the actual transition subject, authentic checked-package/source
    provenance and a separately named independent expectation. The new claim has its
    own stable V2 module and obligation identities and is registered in the current
    compiled census without restoring a V1 row or asserting V1 identity continuity.
    Installed Kani gives that module an in-module SUCCESS proposition and satisfied
    cover; its in-module source mut
- id: FR-046-AC-1
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: 'The genuine source function `rational_commutative(a: Rational[-9,9;1,9],
    b: Rational[-9,9;1,9]): Boolean` with body `a + b = b + a` selects its whole declaration
    and complete body through the public QSL admission, strict IR reader/lowering
    and CG route. Its artifact proves that predicate, not only validity or parity
    of either addition. Replacing the body by `a + b = a + a` selects the same authored
    mutation with unchanged parameter domains.'
- id: FR-046-AC-2
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: 'Every accepted symbolic argument has canonical constructor/accessor roundtrip
    equality and lies in its original inclusive component domains. Its two physical
    I64 bindings retain numerator/denominator roles, original parameter identity and
    bounds in the FR-025 order. Every semantic value in those domains has exactly
    one accepted representation. Zero is `0/1`; negative, fractional and boundary
    values remain reachable. For the original domain there are 111 canonical values
    per argument and 12,321 argument tuples; admitting all 171 raw component pairs,
    restricting denominators to one, or clipping '
- id: FR-046-AC-3
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Generated complete-body evaluation over already-admitted arguments preserves
    the authored full tree and declaration identity, exact owner arithmetic, intermediate
    values outside the input domains, arbitrary-precision continuation, full typed
    outcomes, laziness and the body's Meter charge/stop order from entry onward. Native
    comparison against an independent reference using public owner body operations
    starts from matching entry Meter state and limits and checks those observations,
    including each denied body-charge prefix and zero/exact/short budgets. The application's
    single pre-body FunctionC
- id: FR-046-AC-4
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Generation introduces no assumption on completion, truth or an absence
    of undefined/refused/incomplete results. Controls producing each reachable noncompletion
    retain that typed outcome and cannot become a successful predicate proof; an outcome-based
    assume mutation is rejected. A completion assertion failure without a replayed
    false value is not labelled a confirmed semantic false counterexample.
- id: FR-046-AC-5
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: The healthy original function receives `Verified` only from an actual successful
    proof with all required checks and its completion cover satisfied. The wrong-addend
    mutation produces a substantive failed predicate assertion with actual solver
    playback of canonical in-domain arguments, and replay of that same authored function
    completes with Boolean false. A hypothetical witness or cover witness is not substituted
    for that playback.
- id: FR-046-AC-6
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Each emitted predicate harness has exactly one final completion cover after
    all predicate assertions, including harnesses with representation constraints.
    Unsatisfied or unreported completion cover, failed unwind, timeout, memory exhaustion
    or missing verdict cannot yield `Verified`; classifications retain FR-017 and
    FR-029 distinctions.
- id: FR-046-AC-7
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: A missing or unbounded Rational parameter domain yields source-associated
    `requires-bound` naming the selected function, argument and missing domain, with
    no harness. A finite component endpoint outside this encoding's signed-i64 capacity
    yields named `unsupported`, retaining the original endpoint and no harness; generation
    never clips, casts away precision or substitutes a smaller domain.
- id: FR-046-AC-8
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: An unavailable owner-lowered whole-function representation, absent one-to-one
    correspondence or unsupported body operation yields named `unsupported` with its
    source-associated cause and no harness. Every requested item has exactly one disposition
    under FR-039; a refused item does not remove or alter a supported sibling. Malformed
    source or owner-refused packages remain upstream refusals, not fabricated CG claims.
- id: FR-046-AC-9
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Counterexample decoding validates the two physical I64 entries per original
    parameter, exact width/comment/order, schema roles, required physical-entry and
    byte limits, canonical components, original domains and selected declaration/application
    binding before public owner replay. Grouping yields one typed owner Rational value
    per parameter through the existing canonical serialization seam; missing, duplicate,
    foreign, role-swapped, domain-conflicting and malformed component schemas or entries
    retain typed refusals with no replayed failure. Out-of-domain, duplicate-zero,
    noncanonical, wrong-dec
- id: FR-046-AC-10
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Run evidence records the selected function and source association, inclusive
    component domains and correspondence, ordered physical bindings and their logical
    parameter grouping, actual subject and full ordered backend arguments, every unwind/loop
    bound and option, Kani/Rust/CBMC/solver versions, target, actual dependency cohort
    and execution limits, raw checks/cover/playback, exit and typed terminal outcome.
    Replay uses the retained generated artifact without hand editing. No-run, compile
    refusal, cover failure, resource exhaustion and semantic falsification remain
    distinguishable; preparatio
- id: FR-047-AC-1
  path: spec/kani/functional/FR-047-v2-function-proof-producer.md
  role: examined
  excerpt: An admitted current-V2 function claim selected at one exact occurrence
    produces one executable Kani harness and persisted V2 identity. The harness drives
    the generated FR-021 body module directly (`Factory::begin` then `resume(Resume::Start(..))`)
    beside the genuinely read retained `CheckedPackageV2`, per FR-021-AC-42; the host
    owner-path entry `oracle_{symbol}` is `cfg(not(kani))` and its acquisition/selection/registration
    glue earns no proof credit here. The installed backend verifies a healthy subject
    with a satisfied cover; a separately admitted body-only mutant at the corresponding
    author
- id: FR-047-AC-2
  path: spec/kani/functional/FR-047-v2-function-proof-producer.md
  role: examined
  excerpt: For two distinct claim occurrences on one node, both outputs retain their
    own occurrence, original request index and selected declaration; their persisted
    identities and replay contexts do not cross-bind. A function name or equal body
    cannot merge them. PLANNED (IR-1121).
- id: FR-047-AC-3
  path: spec/kani/functional/FR-047-v2-function-proof-producer.md
  role: examined
  excerpt: Unknown claim or declaration, ambiguous occurrence or declaration, and
    a claim whose selected form is not a function each yield distinguishable typed
    selection refusals before oracle emission, Kani execution or playback decoding;
    an unrelated admitted sibling is unchanged. PLANNED (IR-1121).
- id: FR-047-AC-4
  path: spec/kani/functional/FR-047-v2-function-proof-producer.md
  role: examined
  excerpt: Changing only the admitted package, source owner or source bytes, dependency
    selection, request index, function declaration, claim occurrence, obligation kind,
    parameter identity/domain, generated artifact or proof ceiling produces a named
    typed mismatch or a distinct admitted proof identity; the harness and replay package
    never bind across those changes. The canonical proof-content identity may bind
    exact proved content; no tracking hash or source-version pin is introduced. PLANNED
    (IR-1121).
- id: FR-047-AC-5
  path: spec/kani/functional/FR-047-v2-function-proof-producer.md
  role: examined
  excerpt: The retained original `ReplayInputs`, selected QSL site and emitted `V2FunctionProofIdentity`
    bind through CG's planned `bind_v2_function_replay` before decoding. The same
    bound context and identity admit the planned typed `replay_v2_function_counterexample`
    for a falsifying Kani transcript; wrong package, source, dependency, request or
    identity is refused with its named typed cause before native replay. A dependency-owned
    function selects its actual declaring package and occurrence through an upstream
    QSL imported-function site API; that branch remains Planned under IR-1122 until
    the API exis
- id: FR-047-AC-6
  path: spec/kani/functional/FR-047-v2-function-proof-producer.md
  role: examined
  excerpt: A transcript one byte above the smaller caller/capture byte ceiling, or
    one decoded value above the smaller value-count ceiling, is refused before native
    replay. An in-bound transcript decodes values only against the emitted V2 identity's
    complete ordered domains; missing, extra and out-of-domain values do not become
    a reproduced failure. FR-016-AC-6's required public decode bounds are a prerequisite
    and must be used by the V2 entry; the existing bounded V1 decode does not prove
    V2 wiring. PLANNED (IR-1121).
- id: FR-047-AC-7
  path: spec/kani/functional/FR-047-v2-function-proof-producer.md
  role: examined
  excerpt: The producer uses one admitted checked package and owner-selected callable
    for generation and proving, and preserves FR-021's `Undefined`, `Refused`, `Incomplete`
    and checked-invariant distinctions. The FR-042 wrong-completed-result and genuine
    environment-construction guards remain their own separately planned producer tests;
    this criterion does not count an invented fault as coverage of either guard. PLANNED
    (IR-1121; FR-021/FR-042 prerequisites).
- id: FR-048-AC-1
  path: spec/kani/functional/FR-048-constrained-state-clause-method.md
  role: examined
  excerpt: A selected constraint resolves in the obligation's admitted package and
    state context; absent, ambiguous, wrong-kind, wrong-package and wrong-context
    selections each yield a distinct typed refusal before harness generation or backend
    execution.
- id: FR-048-AC-2
  path: spec/kani/functional/FR-048-constrained-state-clause-method.md
  role: examined
  excerpt: The same safety obligation run without a constraint, with constraint A,
    and with constraint B has three distinct method execution identities and retains
    the same semantic obligation identity. Cross-method evidence or replay substitution
    refuses before conversion; equal requests regenerate identically.
- id: FR-048-AC-3
  path: spec/kani/functional/FR-048-constrained-state-clause-method.md
  role: examined
  excerpt: An initial boundary state and a later boundary state each undergo safety
    evaluation. A seeded safety violation at either boundary is falsified with in-domain
    playback; replacing the boundary branch with an early assumption would fail these
    controls. Neither boundary expands, stutters or counts as deadlock.
- id: FR-048-AC-4
  path: spec/kani/functional/FR-048-constrained-state-clause-method.md
  role: examined
  excerpt: A healthy bounded method reaching a boundary satisfies its separate boundary
    cover and ordinary cover, yields ConstraintReached with Reached observation, and
    produces no ordinary Verified or proved terminal. A boundary cover records reachability
    rather than an invented distinct-state count.
- id: FR-048-AC-5
  path: spec/kani/functional/FR-048-constrained-state-clause-method.md
  role: examined
  excerpt: A healthy method reaching no boundary retains a completed NotReached observation
    and, only with complete exhaustive evidence, a method-qualified Proved/Exhaustive/Uncertified
    terminal retaining the constraint under QSL FR-160; it receives no unconditional
    proof credit. An interrupted or missing boundary check retains Unknown and an
    inconclusive result. An unmet boundary cover alone cannot classify the ordinary
    cover as unsatisfied.
- id: FR-048-AC-6
  path: spec/kani/functional/FR-048-constrained-state-clause-method.md
  role: examined
  excerpt: A constraint that is Undefined at a reached state retains the evaluation
    violation rather than classifying that state as a boundary. Typed Refused and
    Incomplete evaluations retain their original causes and cannot become a Boolean
    assumption or proof. CallFailure instead stops with runtime_invariant / established-invariant-broken,
    without being recast as Undefined, Refused or a boundary.
- id: FR-048-AC-7
  path: spec/kani/functional/FR-048-constrained-state-clause-method.md
  role: examined
  excerpt: Removing ordinary non-vacuity evidence, exhausting unwind or resources,
    cancelling, or supplying malformed backend checks preserves the corresponding
    non-proof outcome even if the boundary cover was satisfied. Reordering check output
    does not change classification.
- id: FR-048-AC-8
  path: spec/kani/functional/FR-048-constrained-state-clause-method.md
  role: examined
  excerpt: The public CG conversion implements every mapping row above using typed
    evidence and exact method binding. A boundary safety violation follows same-method
    replay and FR-029 precedence; replay disagreement, refusal, absent playback and
    identity mismatch never produce refuted or proved by default.
- id: FR-048-AC-9
  path: spec/kani/functional/FR-048-constrained-state-clause-method.md
  role: examined
  excerpt: A public QSL terminal representation preserves the constraint method, the
    ConstraintReached non-proof status and the no-boundary method-qualified proof
    status; where that representation is unavailable, conversion returns a typed unsupported
    refusal. No private wire variant, missing method or unconditional proved fallback
    is emitted.
- id: FR-048-AC-10
  path: spec/kani/functional/FR-048-constrained-state-clause-method.md
  role: examined
  excerpt: Installed-backend healthy-boundary, no-boundary and seeded-boundary-violation
    controls exercise the generated method and retain their exact typed check evidence.
    Independent QSL evaluation agrees on the bounded reached states and boundary safety
    observations. Neither generation-only inspection, saved verdict nor timeout substitutes
    for this evidence.
- id: FR-048-AC-11
  path: spec/kani/functional/FR-048-constrained-state-clause-method.md
  role: examined
  excerpt: A standalone collection constraint remains unsupported under FR-015-AC-128
    with no harness or proof credit. A method constraint never rescues an otherwise
    unbounded or unsupported state domain.
- id: NFR-006-AC-1
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make kani-scope` prints `required` and the path for each of `src/kani/generate/negotiate.rs`,
    `src/oracle/boolean_v1.rs`, `src/routed/generate.rs`, `src/replay/witness.rs`,
    `src/core/identity.rs`, `src/publication/mod.rs`, `tests/it/kani_batching.rs`,
    `tests/it/skeleton_spine.rs`, `tests/it/bounded_kani_corpus.rs`, `tests/it/kani_obligations_state_frame.rs`,
    `tests/it/scratch_crate.rs`, `tests/exact_scalar_support/package.rs`, `tests/checked_package_support/base.rs`,
    `tests/checked_package_support/rekey.rs`, `tests/state_frame_support/model.rs`,
    `schemas/kani-proof-graph-v2.schema.json`, `sch'
- id: NFR-006-AC-2
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make kani-scope` prints `not required` for each of `spec/kani/functional/FR-017-kani-execution-evidence.md`,
    `reviews/REV-018-bound-coverage-observations.md`, `src/strategy/mod.rs`, `src/evidence/mod.rs`,
    `Makefile` and `.github/workflows/ci.yml`, each fed alone.'
- id: NFR-006-AC-3
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Given both old and new paths for a rename from `src/kani/old.rs` to `src/strategy/new.rs`,
    or from `src/strategy/old.rs` to `src/kani/new.rs`, `make kani-scope` prints `required`
    for each.
- id: NFR-006-AC-4
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: 'OPEN during IR-348 migration: the `kani_generation` filter remains and
    must select V2 tests after V1-only tests retire. Original criterion retained:
    `make -n kani` and `make -n kani-gate` expand to the same `cargo test` command
    line, and that line holds each of the six filters `kani_obligations`, `skeleton_spine`,
    `kani_witness_join`, `bounded_kani_corpus`, `kani_generation` and `kani_batching`.'
- id: NFR-006-AC-5
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make kani-gate` starts the lane''s `cargo test` once with `--test-threads=1`,
    holds the lock named by `KANI_LOCK` while it runs, and does not start it while
    another process holds that lock.'
- id: NFR-006-AC-6
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make kani-gate` exits non-zero and prints `result=failed` when the lane''s
    filters list zero tests, when fewer tests run than are listed, and when any test
    fails; it exits zero and prints `result=passed` only when every listed test ran
    and passed and the count is above zero.'
- id: NFR-006-AC-7
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: The passing run-result line has the fields `result`, `ran`, `expected`,
    `elapsed`, `kani`, `tree` in that order, with `ran` equal to `expected`, a non-empty
    `kani` equal to the version the launcher reports, and `tree=clean`. A run whose
    launcher reports no version prints `result=failed`.
- id: NFR-006-AC-8
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: When the tree is clean before the build but dirty after the run, `make
    kani-gate` prints `tree=dirty`, never `result=passed`, and exits non-zero without
    reading or comparing a Git revision.
- id: NFR-006-AC-9
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: With a dirty working tree before the build, `make kani-gate` prints `tree=dirty`,
    never `result=passed`, and exits non-zero.
- id: NFR-006-AC-10
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: 'With no `cargo-kani` launcher `make kani-gate` starts no test, prints
    `kani-gate: not run: launcher absent` and exits non-zero.'
- id: NFR-006-AC-11
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: 'The tests marked `#[ignore = "kani lane: ..."]` in `tests/it` are exactly
    the `#[ignore]`d tests of `tests/it` whose path holds one of the `make kani` filters.'
- id: NFR-006-AC-12
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make -n ci` expands to no `kani` or `kani-gate` command.'
- id: NFR-006-AC-13
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: For a Kani-touching pull request, the author runs the full gate on its
    clean candidate before opening it, and the merger runs it again on the clean current
    candidate before merge; both actual processes exit successfully with all expected
    tests run. A copied body line alone cannot satisfy either run.
- id: NFR-006-AC-15
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: A Kani-touching pull request whose required lane did not run or failed
    remains unmerged and does not claim real-Kani verification; a written explanation
    does not waive the gate.
- id: NFR-006-AC-16
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: A pull request whose scope is `not required` has no mandatory real-Kani
    run under this requirement.
- id: NFR-006-AC-17
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Before a version tag is pushed, the full gate actually passes on the clean
    candidate to be tagged; a copied release-ticket line cannot substitute for that
    run.
- id: NFR-006-AC-18
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Every `include!`, `include_str!`, `include_bytes!` and `#[path]` target
    named in a file of `tests/it` that holds a lane test, and in each file those name
    in turn, matches a pattern of the Kani-touching set.
- id: NFR-006-AC-19
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: On a clean current candidate, `make kani-gate` runs FR-041's current-build
    module census and real mutation controls in its one required lane, and a failure
    of any claim or shared-helper control makes the gate exit non-zero. An optional
    mutation target, retired fixed inventory or saved earlier result cannot substitute.
    Open until a genuine current-build claim source exists.
- id: NFR-006-M-1
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Merged pull requests touching the Kani-touching set without an actual full
    gate passing on the current candidate immediately before merge
- id: NFR-006-M-2
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Lane runs reported `passed` with `ran` different from `expected` or `ran`
    of 0
- id: NFR-006-M-3
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`#[ignore = "kani lane: ..."]` tests the lane does not select'
- id: FR-014-AC-1
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Every requested item receives exactly one generated or typed-refused disposition,
    and a refused item contributes no generated function while its siblings are generated
    unchanged.
- id: FR-014-AC-2
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Integer (add, subtract, multiply, negate, truncating/floor/Euclidean division,
    modulo), rational, ordering, decimal, IEEE arithmetic/comparison/width conversion,
    text comparison, enum comparison, quantity arithmetic/comparison and exact quantity
    conversion (FR-014-AC-48) descriptors for admitted checked-package expression
    nodes each generate a function calling the matching direct owner operator described
    above. Entry-point dispositions for a `quire.op.quantity.convert` node: derivation
    and generation produce the oracle (FR-014-AC-48); the routed path, `negotiate_kani_obligations`
    and capabilit'
- id: FR-014-AC-3
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A descriptor whose arity, result or operand scalar type disagrees with
    the lowered node, a node that is not a lowered scalar expression, a node that
    is unbounded, has an invalid or incomplete body, or exhausts lowering work, or
    a node id requested more than once, is refused with a typed reason.
- id: FR-014-AC-4
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Generated bytes are identical across repeated runs and across permutations
    of the request order; the generated crate is exactly `Cargo.toml`, `src/lib.rs`
    and `claim-map.json`, its `claim-map.json` equals the returned claim map, every
    generated claim's oracle symbol is defined in `src/lib.rs`, and claim-map entries
    are ordered by node id.
- id: FR-014-AC-5
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: RETIRED, replaced by FR-014-AC-56; the per-node IR id and digest-shaped
    package identity are obsolete.
- id: FR-014-AC-6
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Executing generated oracles on the QSpec TC-185, TC-186, TC-192 and TC-193
    vectors, and on these QSpec TC-187 vectors, yields outcomes, charges and consumed
    counters equal to direct runtime execution: U02 (dimension normalization through
    a product and a quotient, and a zero power), U03 (the dimension mismatch), U05
    (arithmetic and a power on `degC` and `degF`, refused `ill_typed`), U09 (a base
    dimension of the same declaration and shape as `L` under a different node key,
    refused `ill_typed`), U12 (compound-unit products, quotients and powers, with
    the `s` family), U13 and U21 (powers), U14 (th'
- id: FR-014-AC-7
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Composite, collection, function, model, relation, state, temporal and protocol
    nodes are refused with their blocked or unsupported reason and never reported
    as generated.
- id: FR-014-AC-8
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: The generated crate declares `publish = false`, names the direct exact/QSV
    and Runtime residue dependencies from the one profile authority without a replay-context
    dependency for Boolean/integer scalar proofs, contains no charge amount (every
    charge comes from runtime metering), and compiles.
- id: FR-014-AC-9
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Generated oracle functions do not panic: an invalid generated constant,
    including a decimal target, stops as `InvalidConstant`, an operand of the wrong
    width stops before any charge, and generated source over its ceiling is a typed
    error with no output.'
- id: FR-014-AC-10
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A descriptor parameter whose admitted bound is missing, repeated or unequal
    to the reachable `bounded_domain` node, an operand that is neither a literal nor
    a reference, and a literal quantity operand, are each refused by CG with a typed
    reason. A malformed bound with a noncanonical integer endpoint such as `05` is
    refused by Contract IR's strict checked-package reader before CG receives an admitted
    item; CG retains its defensive typed `UnreadableBound` refusal for an admitted
    bound it cannot decode. Contract IR's FR-038 "Mode" reader rule also refuses a
    bare Float32 addition authored with the
- id: FR-014-AC-11
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A claim whose descriptor passes every check this requirement states, and
    whose descriptor agrees with the node's catalogued `operation.identity`, law definition
    and mode value, marks its operation `ir_confirmed`. An item refused by any of
    those checks is `caller_declared` even where its operation agrees.
- id: FR-014-AC-12
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A descriptor naming a different catalogued operation than the node's own
    is not confirmed, including where the two share one operand shape and one checked
    bound; nor is a descriptor naming an operation the catalogue has no entry for.
- id: FR-014-AC-13
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A descriptor whose implied identity matches the node's but whose law definition
    is absent from that node's `operation.laws`, or whose mode value disagrees with
    that node's `operation.mode`, is not confirmed; the item still lowers and still
    generates the oracle its descriptor names, marked `caller_declared` with a typed
    blocked item.
- id: FR-014-AC-14
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: "A claim whose node this generator never inspected \u2014 a duplicate copy,\
    \ or a record that never lowered \u2014 or inspected and refused with a typed\
    \ reason, marks its operation `caller_declared` with a typed blocked item and\
    \ reports the request item's own descriptor-derived identity."
- id: FR-014-AC-15
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A requested item whose node closure exceeds the 65,536-unit lowering work
    ceiling is refused as `LoweringWorkExhausted`, naming the ceiling and the consumed
    counter, contributes no generated function, and leaves every other item's disposition
    unaffected.
- id: FR-014-AC-16
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A bound is read from `binding` members looked up by name in any order (`min`/`max`,
    `numerator_min` through `denominator_max`, `coefficient_min` through `rounding`,
    `rounding`, `min`/`max`/`text_profile`); a bare literal member, a missing, duplicate
    or unlisted name is refused as an unreadable bound.
- id: FR-014-AC-17
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A `reference` operand whose target is typed by a `bounded_domain` node
    is classified by that domain's base scalar type, so `x + 1` over a parameter `x`
    of type `Int[0, 9]` generates, and a bounded text parameter is still a text operand.
- id: FR-014-AC-18
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: '`derive_exact_scalar_items` over an admitted V2 package returns, per node
    id in input order, the descriptor determined by the node''s `operation.identity`,
    `operation.mode`, `operation.laws`, operand forms and the one bound of the needed
    form on its result type; a node that is not an application, has no identity, names
    an identity outside the derivable set, has operand forms, a law or a mode that
    select no parameter, is refused with the matching `ClaimDerivationRefusal` and
    no descriptor; a node that is absent, does not lower or has a refused bound (missing,
    repeated or unreadable) is refused w'
- id: FR-014-AC-19
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A derived item generates an oracle whose operation is `ir_confirmed`, and
    for every item of the corpus the derived descriptor equals the descriptor the
    fixture declares.
- id: FR-014-AC-20
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: An integer node over two distinct bounded parameters, `Int[0, 9]` and `Int[10,
    20]`, each typed by its own `integer_range` `bounded_domain`, with its result
    typed by a third `integer_range` `[0, 29]`, generates an `ir_confirmed` oracle
    whose descriptor is compared with the result bound only; the claim's checked bounds
    are the result bound, then each operand's own bound; derivation returns the descriptor
    over `[0, 29]`.
- id: FR-014-AC-21
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node whose result is typed by a scalar type and whose reachable bounds
    are its operands' own and exactly one other takes that other as its result bound.
- id: FR-014-AC-22
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node whose result is typed by a scalar type and whose two or more reachable
    bounds of the form are not exactly one outside the operands' own bounds is refused
    as `AmbiguousBound`, in generation and in derivation.
- id: FR-014-AC-23
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: An operand, or the result, typed by a `bounded_domain` of a form other
    than the descriptor's is refused as `MissingBound`.
- id: FR-014-AC-24
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'An operand''s own bound need not lie inside the result bound: `a + wide`
    over `[0, 9]` and `[0, 50]` with a `[0, 29]` result, `-e` over `[1, 9]` with a
    `[-9, -1]` result and `e * f` over `[1, 9]` and `[100, 200]` with a `[100, 1800]`
    result each generate, derive the same descriptor, and record each operand''s own
    bound.'
- id: FR-014-AC-25
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A `reference` operand to a plain scalar parameter or a non-literal `value`
    node is refused as `RequiresBound` naming its type, in generation and in derivation,
    where another operand owns a bound or the result is typed by one (`x + y`, `y
    + y`, and `x + y` over a scalar-typed result); a literal operand is never refused
    as `RequiresBound`. An eligible application reference follows FR-014-AC-45 instead.
- id: FR-014-AC-26
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A `reference` operand whose target is a `value` node with a literal body
    is a literal operand: `x + 1` over `x: Int[0, 9]` narrowed to `Int[0, 10]`, with
    the literal as a reference to its own plain-Integer-typed `value` node, generates
    with checked bounds `[0, 10]` then `[0, 9]` and derives the descriptor over `[0,
    10]`.'
- id: FR-014-AC-27
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A node whose result is typed by a scalar type and that only narrowing
    conversions (`quire.op.numeric.narrow`, exactly one argument referencing the node)
    consume takes the bound they narrow it to as its result bound: `x + y` over `Int[0,
    9]` and `Int[10, 20]` narrowed to `Int[10, 29]` generates with checked bounds
    `[10, 29]`, `[0, 9]`, `[10, 20]` and derives the descriptor over `[10, 29]`, a
    descriptor over `[0, 9]` is `BoundMismatch` against `[10, 29]`, and `-z` over
    `Int[0, 9]` narrowed to `Int[-9, 0]` checks `[-9, 0]` then `[0, 9]`.'
- id: FR-014-AC-28
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'The referenced node''s body, not its form or type, makes an operand a
    literal: `x + n`, where `n` is a plain-Integer-typed `value` node of form `literal`
    whose body is not a literal, is refused as `RequiresBound` naming Integer, in
    generation and in derivation.'
- id: FR-014-AC-29
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: '`x + p` narrowed to `Int[0, 29]`, where `p` is a parameter typed by the
    plain Integer type, is refused as `RequiresBound` naming Integer, in generation
    and in derivation.'
- id: FR-014-AC-30
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: '`q + q` narrowed to `Int[0, 29]`, where no operand owns a bound and `q`''s
    node reaches `[0, 29]`, is refused as `RequiresBound` naming Integer, in generation
    and in derivation: the narrowing bounds the result as a `bounded_domain` result
    type does.'
- id: FR-014-AC-31
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A literal operand of an integer operation that is not a canonical `integer`
    literal is refused as `OperandTypeMismatch` naming its position and literal kind,
    whatever the referenced node''s type: `x + "a"` with the text literal in a plain-Integer-typed
    `value` node is refused at position 1 with kind `text`, in generation and in derivation.'
- id: FR-014-AC-32
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node that narrowing conversions narrow to two distinct bounds (`[10,
    29]` and `[0, 40]`) is refused as `AmbiguousBound`, in generation and in derivation.
- id: FR-014-AC-33
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node narrowed to a `text_bounds` bound over Integer, or to an `integer_range`
    over Rational, is refused as `MissingBound` naming Integer and `integer_range`,
    in generation and in derivation.
- id: FR-014-AC-34
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node that a narrowing conversion and a node that is not a narrowing conversion
    both consume (`x + 2` narrowed to `Int[0, 11]` and an operand of `(x + 2) + x`)
    is refused as `AmbiguousBound`, in generation and in derivation, while `x + 1`,
    consumed only by its narrowing, generates.
- id: FR-014-AC-35
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Boolean `and`, `or`, `not`, `implies`, `eq` and `ne` nodes and integer
    `eq`, `ne`, `lt`, `le`, `gt` and `ge` nodes each derive a descriptor and generate
    an `ir_confirmed` oracle; each connective's oracle calls `evaluate_boolean` or
    `evaluate_boolean_short_circuit`, and each `eq`/`ne` oracle calls `check_equality`
    and `CheckedEquality::evaluate`, each returning `Outcome<bool>`.
- id: FR-014-AC-36
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Every oracle's source map declares the node's implication-consequent count,
    one evaluation-entry probe on the function-entry line disjoint from every consequent
    region, and one entry-token probe inside the exact region of each consequent;
    a dropped or duplicated region does not change the declared count.
- id: FR-014-AC-37
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A differential corpus covering every node FR-014-AC-35 names compiles
    against the runtime alone, and each oracle''s outcome, admitted charges and consumed
    counters equal direct runtime execution. This criterion claims no agreement with
    the QSL value authority: that boundary is owned by quire-integration IR-842 (generated-oracle
    against QSL agreement), and FR-014-AC-47 keeps CG''s artifacts from claiming it.'
- id: FR-014-AC-38
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: The Boolean condition in the third argument of a `state`/`state_clause`
    node's `quire.op.state.clause` body, addressed by the clause node id, derives
    a descriptor and generates an `ir_confirmed` oracle over its connectives, integer
    comparisons and integer add, subtract, multiply and negate, returning `Outcome<bool>`.
    When the condition is a reference to an admitted Boolean expression node, the
    oracle uses that node's expression, not the reference as a Boolean variable; an
    inline condition uses its own expression. In both cases the claim and generated
    oracle retain the clause node identity, not
- id: FR-014-AC-39
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'The generated `src/lib.rs` of every crate the TC-024 corpus generates,
    and the non-test code of `src/oracle/scalar/mod.rs`, `src/oracle/scalar/derive.rs`,
    `src/oracle/scalar/lower.rs` and `src/oracle/scalar/render.rs` (comments and every
    `#[cfg(test)]` item removed wherever the item sits, string literals the generator
    emits counted), contain zero panic tokens: the identifiers `unwrap`, `expect`,
    `unwrap_unchecked`, `unwrap_err`, `expect_err`, `unwrap_err_unchecked`, `panic_any`
    and `resume_unwind` however written (called, with whitespace before the paren,
    named on a path such as `Option::unwra'
- id: FR-014-AC-40
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A package whose lowering returns `failed` for the `bytes` limit (Contract
    IR FR-038-AC-95) is refused per item as `ExactScalarRefusal::LoweringByteLimitExceeded
    { limit, consumed }` with `limit` and `consumed` equal to the record''s, never
    as `LoweringWorkExhausted`, and contributes no generated function. Through a whole
    call of `generate_exact_scalar_oracles` whose input is read under a byte ceiling
    that admits the checked package and is one byte below the canonical length of
    the lowered contract package of that call (the package `lower` returns for the
    call''s requested nodes, not the checked '
- id: FR-014-AC-41
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A `failed` record whose `limit_kind` is `work` is still refused as `LoweringWorkExhausted`
    with the record''s `limit` and `consumed` (FR-014-AC-15 unchanged), and one whose
    `limit_kind` is neither `work` nor `bytes` (each of `nodes`, `edges`, `occurrences`
    and `diagnostics`) is refused as `ExactScalarRefusal::LoweringLimitUnrecognised`
    with `limit_kind` equal to that snake_case name and the record''s `limit` and
    `consumed`, without a panic and without a `LoweringWorkExhausted` or `LoweringByteLimitExceeded`.
    Each of the two is asserted on hand-built `Failed` records given to the scalar
    `Failed` '
- id: FR-014-AC-42
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'In each of `src/oracle/scalar/mod.rs`, `src/oracle/scalar/derive.rs`,
    `src/oracle/scalar/lower.rs`, `src/oracle/scalar/render.rs`, `src/oracle/equality/mod.rs`
    and `src/oracle/function/mod.rs`, the non-test code, as FR-014-AC-39 defines it
    (comments and every `#[cfg(test)]` item removed wherever the item sits, taken
    with `non_test_code` and `comments_stripped` of `tests/common/panic_scan.rs`),
    names no `CheckedPackageLimit` and accesses no `.limit_kind`: only `classify_lowering_failure`
    in the shared `oracle` module reads a `failed` record''s `limit_kind`. Across
    the four scalar files, exactly '
- id: FR-014-AC-44
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: For each IR-admitted direct request naming a `temporal`/`formula`, `temporal`/`fairness`
    or `expression`/`case` node, the scalar claim map has exactly one entry with that
    requested node id and `ExactScalarRefusal::Unsupported`. For an IR `Unsupported`
    lowering record, the refusal's `unsupported_node_id` and `node_tag` equal that
    record's first unsupported reachable node and tag, even if that node is not the
    request root; for a lowered `case` root they name the root and `expression`. A
    lowered `case` is refused as `Unsupported`, not `FormMismatch`. Each refused item
    has no generated function or
- id: FR-014-AC-45
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'For a QSL-shaped outer integer operation narrowed to an `integer_range`,
    a direct reference to a plain-Integer inner application over independently bounded
    operands supplies a conservative inclusive operand interval: `(x + 1) + 1` for
    `x: Int[0, 9]` supplies `[1, 10]` for its inner operand, and `-(x + 1)` supplies
    `[1, 10]` to the outer negate; `x * -1` for `x: Int[1, 9]` supplies `[-1, -1]`
    from a negate application over the canonical literal `1`. Each outer node derives
    and generates an `ir_confirmed` oracle, records the inferred range by argument
    position without inventing a checked bound i'
- id: FR-014-AC-46
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: An inferred interval uses only the four named integer operations and canonical
    literal or `integer_range`-typed leaves. A direct reference to a raw application
    consumed by a separate narrowing conversion never takes that conversion's domain
    as its operand interval; where the raw application has another consumer, requesting
    that application itself still yields FR-014-AC-34's `AmbiguousBound`. A plain-Integer
    parameter at any depth remains `RequiresBound` naming Integer. A missing, cyclic
    or unsupported application, wrong-kind or noncanonical literal, and an inference
    depth, work or endpoint-bit
- id: FR-014-AC-47
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: No test name, trace tag, test output line or doc comment of every oracle-agreement
    corpus of FR-014, FR-018 and FR-021 (the TC-185, TC-186, TC-187, TC-192 and TC-193
    vectors, the FR-014-AC-37 corpus, the composite-equality corpus and the function-application
    corpus), no TC-024, TC-029 or TC-031 step and no oracle-matrix row for those cases
    or corpora claims, or presents as evidence, agreement with the QSL value authority;
    text that states this boundary is permitted.
- id: FR-014-AC-48
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'For an admitted V2 package or a V1 package with source-authenticated operation
    views, a `quire.op.quantity.convert` node whose `type_argument` names a quantity
    unit and whose `operation.mode` is `exact` derives and generates an `ir_confirmed`
    oracle, described by the public operation `quantity_conversion` (`ExactScalarOperation::QuantityConversion`,
    a unit-free variant: the target unit is a call-time argument `unit: &IdentifiedUnit`,
    and the generator does not check it against the `type_argument`, which the node
    carries only as a type node, Contract IR FR-038-AC-84 and FR-038-AC-213), with
    no '
- id: FR-014-AC-49
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'When QSV refuses a `quire.op.quantity.convert` with `IllTyped` before
    any charge, the oracle stops as `ill_typed` with zero admitted charges and no
    change to any consumed counter, equal to direct `convert_quantity` execution (Contract
    Runtime FR-007-AC-21): `1 m` to `s` (U03-style `m` to `s`), U09 (a base dimension
    of the same declaration and shape as `L` under a different node key), `1 N_m`
    to `J` (U22) and compound `4 m^2` to `m`. An affine source or target converts
    and is not refused as `AffineUnitArithmetic`; only arithmetic on an affine unit
    is refused (FR-014-AC-6, U05).'
- id: FR-014-AC-50
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'For an admitted V2 package or a V1 package with source-authenticated operation
    views, no oracle generated for `quire.op.quantity.convert` returns a scalar: its
    result is always a `Quantity` in the supplied target unit, and a conversion node
    whose `operation.mode` is not `exact` is refused with `ClaimDerivationRefusal::ModeNotDerivable`,
    checked first, so a scalar `type_argument` with a non-`exact` mode is `ModeNotDerivable`;
    one whose mode is `exact` and whose `type_argument` is a scalar type (`Decimal`
    or `Integer`, QSpec TC-187 U07, U08, U16, U17, U18 and U29) or a compound unit
    (U25''s first'
- id: FR-014-AC-51
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: After admitted V2 or source-authenticated V1 operation views are available,
    a selected `quire.op.quantity.convert` claim settles through capability settlement
    like any other quantity operation, as `OperationNotRendered` (Planned, IR-269),
    because the Kani scalar renderer builds no quantity harness; the conversion is
    credited by its oracle only and is uncredited for Kani. The settlement is not
    a refusal by name, and a quantity harness is IR-269's work, not this requirement's.
    The owning settlement row is TC-030's Quantity scalar row, which names `quire.op.quantity.convert`
    and cites this criter
- id: FR-014-AC-52
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Every generated quantity operand is a QSV `UnitQuantity<''_>` resolved
    through the kernel quantity''s own id by `UnitTable::resolve` or `UnitScope::resolve`,
    with no CG-created unit ids or unrelated metadata pairing. Comparison calls QSV
    `compare_quantity` and preserves its typed `IllTyped`. Arithmetic calls QSV `evaluate_quantity_unit`
    and returns `Result<(Outcome<Quantity>, IdentifiedUnit), OracleStop>` with the
    exact owner-provided identified result unit on every `Ok`, including stopped outcomes,
    and preserves both `QuantityRefusal::IllTyped` and `QuantityRefusal::Identity`
    as distinct typed '
- id: FR-014-AC-53
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: RETIRED, replaced by FR-014-AC-57; body-only mutation under a producer-assigned
    structural key does not cause a stale-key reader refusal.
- id: FR-014-AC-54
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: PLANNED (IR-917). A QSL-authored V2 clause comparison containing an integer
    literal wrapped by the checked FR-093 `quire.op.numeric.narrow` shape derives
    and generates its clause-identified oracle when the complete literal parses and
    lies in the target `Int[..]` interval. The oracle uses the unchanged integer value
    and yields the same ordered admitted charges, consumed counters and outcome as
    independent direct-owner evaluation of that clause arithmetic with that integer;
    the wrapper itself admits no charge. Missing target or operand, out-of-range literal,
    nonliteral operand and unparseable li
- id: FR-014-AC-55
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A plain reader-admitted V1 package with a represented application that
    reaches a mode, law-definition or member-target view required by a routed request
    is refused by `generate_routed` as the whole-call `RoutedGenerationError::UnauthenticatedOperationView
    { request_index, node_id }` naming the lowest `request_index` among such routed
    items, independent of the caller's routed-item slice order, before oracle, capability
    settlement or harness output. This includes quantity conversion and division-profile
    selection; a supported view-independent integer route over a plain-read package
    retains its e
- id: FR-018-AC-1
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Every requested item receives exactly one generated or typed-refused disposition;
    a refused item contributes no function, no environment constructor and no symbol
    to the generated crate, while its siblings are generated unchanged.
- id: FR-018-AC-2
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'On every vector of the composite-equality corpus, the generated oracle''s
    `Outcome<bool>`, its admitted charge sequence and its consumed counters are equal
    to those of `quire_contract_runtime::exact::TypeEnvironment::check_equality` plus
    `CheckedEquality::evaluate` invoked directly on an independently constructed environment,
    operands and fresh `Meter`. This criterion claims no agreement with the QSL value
    authority: that boundary is owned by quire-integration IR-842 (FR-014-AC-47).'
- id: FR-018-AC-3
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "Each generated item's claim-map entry records the descriptor the request\
    \ supplied \u2014 operator, both operand source types and both conversion targets\
    \ \u2014 and it is equal to the request's, so the independent native run of AC-2\
    \ is driven from the request and never read back out of the generated crate. An\
    \ authentic asymmetric source control preserves the actual bounded-left/unbounded-Integer-right\
    \ source IDs and independent caller target; a source conversion erased from the\
    \ emitted graph is not evidence of a retained conversion-reference branch."
- id: FR-018-AC-4
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Each generated item's claim-map entry records an `EqualitySchedule` equal
    to `CheckedEquality::schedule()` of the `CheckedEquality` its descriptor admits,
    for a top-level text, enum and quantity pair and for each `Plan`-schedule composite
    and collection shape.
- id: FR-018-AC-5
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A descriptor with a `convert<T>` operand outside `admits_equality_conversion`,
    distinct text profiles, distinct enum declarations, incompatible dimensions, distinct
    units, or no common type is refused at generation time with its `IllTypedCause`,
    emits no code, and admits no charge on any `Meter`.
- id: FR-018-AC-6
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "An operand type bearing an IEEE value at any depth \u2014 including a\
    \ `float64` field of a record nested inside a `sequence` \u2014 is refused as\
    \ `OperatorIneligible`, and the refusal agrees with `TypeEnvironment::contains_ieee`\
    \ on the reconstructed type at every depth in the corpus."
- id: FR-018-AC-7
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: On the FR-330 package path, model, function, state and temporal roots outside
    the equality lowering profile retain their exact typed `Unsupported` feature-accounting
    record and source occurrences in `PackageAccounting`; each has its own requested
    key and node tag, no symbol or oracle source, and does not change an independent
    admitted sibling. A `reference` composite targeting a record value type instead
    of a permitted target (model object type, process, state machine, systems interface,
    or relationship) is refused by Contract IR's strict reader as `InvalidPackage/InvalidDerivedShape`
    at the r
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A declaration closure `TypeEnvironment::new` refuses is refused at generation
    time carrying that `DeclarationCause`, including both recursion passes. The authentic
    duplicate-field source control instead retains typed AdmittedRefusal::Package
    with owning public CompileRefusal::code() equal to Code::AmbiguousDeclaration
    and cause() equal to None for the Assembly refusal, produces no successful AdmittedContext
    and is not passed to public CG generation. Private CG declaration-admission defenses
    retain the exact DuplicateMember name/cause and unchanged healthy sibling separately,
    without public-acc
- id: FR-018-AC-9
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "Every generated oracle function returns `Outcome<bool>`, never `bool`:\
    \ with a denial injected at each of `equality.plan-form`, `equality.plan`, `equality.pair`,\
    \ `equality.result-retain` and each conversion charge point in turn, the oracle\
    \ yields `Outcome::Incomplete` naming that point, the denied charge is not applied\
    \ \u2014 every counter equals those of the same run stopped immediately before\
    \ that point \u2014 and never a completed Boolean."
- id: FR-018-AC-10
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "Generated bytes are identical across repeated runs and across permutations\
    \ of the request order, including a request carrying two descriptors over one\
    \ node id, and claim-map entries are ordered by the descriptor key \u2014 expression\
    \ node id, operator rank, then each operand's source-type and conversion-target\
    \ node ids, every node id compared in node-id order; the generated crate is compiled\
    \ and executed under AC-2 against a native run driven from the request's own descriptor\
    \ (AC-3), so a generated crate with a changed emitted operator, operand order\
    \ or descriptor fails AC-2."
- id: FR-018-AC-11
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Every claim marks its operation `caller_declared`, the claim map carries
    the blocked item "operation identity not consumed by codegen's generators", and
    two items over one node differing only in `EqualityOperator` both generate with
    that mark, under distinct symbols, each built from its own operator's stem, and
    produce complementary outcomes on a vector whose operands differ.
- id: FR-018-AC-12
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The generated crate declares `publish = false`, names direct exact/QSV
    and root replay dependencies through the one manifest authority, contains no charge
    amount and no planned pair count (every charge and every pair comes from runtime
    metering), and compiles.
- id: FR-018-AC-13
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: ~~Retired digest-era declaration-key comparison.~~ FR-018-AC-46 carries
    forward every claim-entry, ownership, identity, lookup, order and refusal check
    using structural `NodeKey`; the digest oracle is not reused.
- id: FR-018-AC-14
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Admitted `bounded_domain` nodes an operand type reaches retain the exact
    named bound values FR-014 specifies. For `integer_range` and `collection_bounds`,
    Contract IR FR-038 admission requires exactly `min` then `max` bindings with their
    canonical typed integer literals; a reordered, bare, missing, duplicate or unlisted
    binding control is an IR-intake refusal before CG, with no admitted package or
    public CG `UnreadableBound` result. CG private parser tests retain their precise
    malformed-bound defenses separately and do not count as public-generator acceptance
    for these reader-ineligible inputs
- id: FR-018-AC-15
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'A `binary` node''s operand is read through its `reference`: a reference
    to a `convert` expression node is read as the type of what it converts, following
    nested conversions to the first node that is not a conversion, and a reference
    to any other node, including a non-`convert` application, as that node''s own
    `semantic_type`. A descriptor whose `source_type` is the read type generates,
    and any other source is refused as an operand-type disagreement at that position,
    reporting the read type as found. A genuine source fixture with no emitted convert
    node exercises its actual parameter source types'
- id: FR-018-AC-16
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Genuine public tuple members naming `integer_range`, `decimal_range`, `rational_range`
    and `text_bounds` nodes reconstruct their exact own bounds, text profile, position
    order and decimal rounding; in particular the numeric controls retain Int[-100,100],
    Decimal[-100,100;0,0;nearest-even] and Rational[-10,10;1,1], and the text control
    retains Text[0,16;nfc]. With two `text_bounds` nodes over one text scalar, naming
    either reconstructs that node's own `min` and `max`, never the sibling's. The
    legacy private resolver control retains exact generated-source equivalence between
    numeric bounded-doma
- id: FR-018-AC-17
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The `src/lib.rs` that `generate_composite_equality_oracles` returns for
    the TC-029 step 1 package (every generated item) contains zero occurrences of
    the panic tokens FR-018-AC-19 lists, no `.ok()`, `.unwrap_or(` or `.unwrap_or_default(`
    anywhere, and no `[` index or slice expression directly after an identifier character,
    `)` or `]`; every reconstruction helper in it returns `Result<_, ReconstructionError>`,
    and each of `rebuild_integer`, `rebuild_interval`, `rebuild_rational`, `rebuild_decimal`,
    `rebuild_text` and `rebuild_cardinality` turns a failed constructor or parse into
    exactly the mat
- id: FR-018-AC-18
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '`render_value_type` called with `ValueType::Quantity` and with `ValueType::Reference`
    returns `Err(RenderError::UnsupportedValueType { family })` with `family` equal
    to `"quantity"` and `"reference"` respectively and does not panic, and the item-boundary
    mapping turns `RenderError::UnsupportedValueType` into `CompositeEqualityRefusal::Unsupported`
    with `node_tag` equal to `family` and `unsupported_node_id` equal to the item''s
    expression node id, emitting no code for the item and leaving its siblings unchanged,
    while `RenderError::Generation(OracleGenerationError::UnknownRuntimeVariant {
    .. })`'
- id: FR-018-AC-19
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'The non-test code of `src/oracle/equality/mod.rs` (comments and every
    `#[cfg(test)]` item removed wherever the item sits, string literals the generator
    emits counted) contains zero panic tokens: the identifiers `unwrap`, `expect`,
    `unwrap_unchecked`, `unwrap_err`, `expect_err`, `unwrap_err_unchecked`, `panic_any`
    and `resume_unwind` however written (called, with whitespace before the paren,
    named on a path such as `Option::unwrap`, or imported); the macros `panic`, `unreachable`,
    `todo`, `unimplemented`, `assert`, `assert_eq`, `assert_ne`, `debug_assert`, `debug_assert_eq`
    and `debug_assert_ne'
- id: FR-018-AC-20
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: When an admitted FR-330 package contains a `Failed` feature-accounting
    record, its equality claim is refused as `CompositeEqualityRefusal::PackageAccounting`
    with the exact `limit_kind`, `limit`, `consumed` and source occurrences from that
    record; it is never relabelled as work exhaustion. When a checked package is admitted
    under a byte ceiling one byte below its otherwise canonical lowered contract package
    length, Contract IR FR-038-AC-95 returns `Failed(Bytes)` for every requested root
    with that ceiling as `limit` and one shared `consumed` above it, and exposes neither
    canonical package byte
- id: FR-018-AC-21
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Each generated item's claim-map entry records its declaration closure as
    every declaration's key, form (record, tuple or option), members in declaration
    order with each member's name, presence and type, and each leaf's family and declared
    bound, and the closure equals member by member (name, presence, value type, order)
    the declarations of the `TypeEnvironment` the item's generated environment constructor
    returns, for every item of the TC-029 corpus whose leaves are Boolean or bounded
    integers. PLANNED (IR-264).
- id: FR-018-AC-22
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'For every pair of operand values in the item''s refinement domain (FR-028-AC-16:
    every case where the domain fits the case cap, the boundary and seeded cases otherwise;
    every presence state of an option and of an optional field included) of an item
    whose leaves are Boolean or bounded integers, the generated oracle called with
    its own environment and a `Meter` whose every limit is `u64::MAX` returns `Outcome::Completed`
    with the verdict of QSpec FR-149 for the item''s operator and admits exactly `equality.plan-form`,
    `equality.plan`, one `equality.pair` per node of the occurrence-pair tree, then '
- id: FR-018-AC-23
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: For every item of the TC-029 corpus whose leaves are Boolean or bounded
    integers, the claim-map closure's member names, presence and each integer leaf's
    inclusive bounds equal those of an independent read of the checked package (the
    V2 composite and `bounded_domain` nodes read through Contract IR's reader, never
    through the generator's reconstruction); a closure that dropped a member, narrowed
    a bound by one or read an optional member as required fails the check. PLANNED
    (IR-264).
- id: FR-018-AC-24
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The exact QSpec `positive-recursive-records.json` retains strict-reader
    and List/Tree shape assertions; independently authored genuine admitted source
    with those List (`Option<List>`) and Tree (`Sequence<Tree>` bounded `[0,3]`) shapes
    closes each back-edge at its own in-progress record key, generates equality items
    over both record types, and terminates; a synthetic unclosed `Option` self-cycle
    and a `Sequence` self-cycle each refuse as `TypeResolutionCycle` naming the repeated
    type node, without a panic, stack overflow or generated symbol. PLANNED (IR-643).
- id: FR-018-AC-25
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'The per-item type-resolution counter charges one unit for each root type-node
    entry or followed type-node reference or bounded-domain `semantic_type` edge,
    including a repeated or memoized reference, across both operands and conversion
    targets, independently of Contract IR lowering; with the default limit, an acyclic
    chain reaches its 65,537th entry and refuses only its item as `TypeResolutionWorkExhausted
    { limit: 65_536, consumed: 65_537 }` before further descent and without call-stack
    exhaustion or generated symbols, while a sibling needing at most 65,536 entries
    still generates. PLANNED (I'
- id: FR-018-AC-26
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Reading the QSpec List `next` field as `binding(next, aggregate([binding(optional,
    reference Option<List>)]))` yields exactly one `FieldDeclaration` named `next`,
    with `Presence::Optional` and payload type `ValueType::Composite(List NodeKey)`
    on the genuine public admitted List path. At the separately labelled private parser/type-resolution
    seam, removing or renaming `optional`, adding a second nested member, or targeting
    a non-option node retains exact MalformedComposite, no symbol, no required-field
    reinterpretation and an unchanged healthy Tree sibling. These inconsistent internal
    mutations
- id: FR-018-AC-27
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Reading the QSpec Tree `kids` field's direct reference to `collection_bounds`
    follows that node's `semantic_type` to `Sequence<Tree>`, reads its sole element
    reference as the Tree `NodeKey`, and reconstructs cardinality `[0,3]` from that
    bound node's named `min` and `max`; changing the semantic type to a non-sequence,
    dropping or duplicating either bound, or making either value non-integer refuses
    only that item with a typed cause and emits no code, never silently yielding an
    unbounded collection. PLANNED (IR-643).
- id: FR-018-AC-28
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: With a caller-selected type-resolution work limit of at least 100,000 and
    `cg.equality.source_bytes` raised to the measured flat source size, an admitted
    acyclic type chain of 100,000 entries reconstructs and generates an oracle with
    the same result as an equivalent shallow type on a 512 KiB stack; with the same
    raised source-byte limit, the default type-resolution work limit refuses it at
    entry 65,537, and a selected work limit one below its entry count refuses it at
    the first denied entry. No depth setting or fixed depth refusal participates in
    these runs. PLANNED (IR-511).
- id: FR-018-AC-29
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A cycle detected after a charged repeated entry reports `TypeResolutionCycle`
    without following a child; if that repeated entry is the first denied work entry,
    `TypeResolutionWorkExhausted` wins and reports the effective limit and first denied
    count. An unaffected sibling keeps its original disposition in either case. PLANNED
    (IR-511).
- id: FR-018-AC-30
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The 100,000-entry acyclic closure renders as flat source without a type-depth-sized
    constructor expression; under the 1,048,576-byte default it returns `SourceTooLarge`
    naming that limit when the source exceeds it, and under a caller limit equal to
    its measured source bytes it emits a complete, compilable oracle within the publication
    artifact and bundle byte limits. One byte less refuses before that byte is appended
    and publishes no partial artifact. PLANNED (IR-511).
- id: FR-018-AC-31
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A caller-selected type-resolution work limit of `u64::MAX` refuses as `InvalidTypeResolutionWorkLimit`
    before lowering or charging; with `u64::MAX - 1`, the first denied count fits
    in the `u64` `consumed` field and is reported without wraparound. PLANNED (IR-511).
- id: FR-018-AC-32
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Each IR-admitted direct `temporal`/`formula`, `temporal`/`fairness` and
    `expression`/`case` request has one claim-map entry keyed by its requested node
    id and refused as `CompositeEqualityRefusal::Unsupported`. For an IR `Unsupported`
    lowering record, the refusal's `unsupported_node_id` and `node_tag` equal that
    record's first unsupported reachable node and tag, which need not be the requested
    root; for a lowered `case` root they name the root and `"expression"`. The request
    contributes no generated symbol or source, never panics or disappears, and leaves
    a supported sibling's claim and source
- id: FR-018-AC-33
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A genuine root-facade compilation of authentic retained source, original
    dependencies and admitted request limits has equal compiled/context/original package
    identities and exact full recompiled/original CheckedPackageV2 equality before
    its context is exposed. An explicitly supplied different context package refuses
    generation and emitted environment construction before classification or lookup,
    including primitive-only requests, publishes no prepared value, and cannot evaluate
    with a completed Boolean or meter charge.
- id: FR-018-AC-34
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The actual generated ABI executes ordered and unordered enum comparisons
    over all declared cases, including a case absent from every selected literal and
    an enum parameter with no member literals. Exact shape variants and canonical
    positions come from genuine facade/QSV values. Same-spelled cases from distinct
    declarations retain distinct identities; matching and mismatching declaration
    comparisons preserve the owning equality disposition.
- id: FR-018-AC-35
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Unknown declaration, unknown case and a directly or transitively imported
    declaration produce the original typed Unknown/Foreign details at consumed and
    generated lookup boundaries. Malformed source/dependency input, QSL stage or above-reader
    resource limits, and internal Fault remain Package, Resource and Fault at the
    source-retaining boundary; a subsequent IR read stop remains the separate PackageReadLimit,
    with no partial context or successful prepared wrapper. The current ImportedContextUnsupported
    guard remains until original admitted dependency packages are retained.
- id: FR-018-AC-36
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'Emitted source uses only structural checked-package `CheckedNodeId` and
    case identifiers for lookup and contains no NodeKey constructor/decoder or generated
    nominal preimage/hash. Only the prepared wrapper''s private genuine index reaches
    check_equality. A private environment-only fault fixture still satisfies AC-8,
    every original charge-stop case still satisfies AC-9, and a wrong item/descriptor
    prepared wrapper refuses before charging. Record/tuple/recursive/collection cases
    remain required by AC-2 and AC-21 through AC-30; an absent owner lookup capability
    is recorded as unresolved migration '
- id: FR-018-AC-37
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Equality's generated manifest and native scratch consumers use the one
    profile authority's direct exact/QSV/replay dependencies, while ordinary Boolean/integer
    FR-014 scalar proof manifests and oracle bodies acquire no replay source-compilation/context
    leg. Native generated enum acceptance is reported separately from original TC025/TC027
    proof qualification; neither result substitutes for the other.
- id: FR-018-AC-38
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: With `None`, standalone generation and emitted preparation accept successfully
    reconstructed non-nominal primitive and nested Option/Collection closures; their
    index is empty only after complete zero-enum resolution. Existing admitted-IR
    conversion-form and quantity-family negatives retain respectively exact FormMismatch("conversion")
    and Unsupported("quantity"), with no replacement ContextPackageMismatch or MissingAdmittedContext,
    no symbols for the refused item and unchanged independent siblings. This does
    not claim matching QSL source emission or source authentication for those IR fixtures.
- id: FR-018-AC-39
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '`None` at the first reached nominal identity returns typed MissingAdmittedContext
    naming that declaration. Cover left and right source operands, left and right
    conversion targets from primitive sources, nested Option and Collection enum types,
    and eventual record/tuple field and recursive closures through their genuine owner
    capability. No key, default/partial index, omitted nominal declaration or Unsupported
    substitute is produced. Retain the existing charged-entry/cycle/error precedence
    and unaffected sibling results; the separate missing record/tuple owner lookup
    remains outstanding.'
- id: FR-018-AC-40
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Some(wrong-package context) fails the whole standalone call after existing
    setting/accounting checks and before item classification, for primitive-only,
    nominal and empty requests. Emitted context-free and nominal constructors both
    reject the same supplied mismatch before reconstruction. None and Some(wrong)
    have distinct outcomes; no failure publishes partial artifacts or a prepared wrapper.
- id: FR-018-AC-41
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A reached enum still reconstructs every owner-admitted case, including
    unused cases and an enum parameter with no member literal, using the supplied
    genuine keys and canonical positions. Failed nominal admission/lookup/traversal
    cannot yield an empty-index success. A successfully resolved zero-enum closure
    may use an empty index; merely inspecting an outer primitive/Option/Collection
    tag does not establish zero enums.
- id: FR-018-AC-42
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Generated oracle invocations under Some or valid context-free None preserve
    private package/item/descriptor/index binding, both check_type guards, same-index
    check_equality and evaluate. Private wrong-binding, present-context mismatch,
    missing nominal context and environment-only faults refuse without charge. The
    native OriginalCompositeEqContext route retains every mandatory authentic source/lock/package/whole-IR
    comparison and cannot use None to bypass them. No scalar proof manifest/body acquires
    replay compilation.
- id: FR-018-AC-43
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'For genuine source with a: Int[0,9] and b: Integer, the admitted left
    parameter names its integer_range and the right parameter names canonical base
    Integer. Direct source/target reconstruction preserves Int[0,9] versus unbounded
    Integer despite the sibling range: a caller descriptor converting the left to
    Integer compares with the unbounded right under the original equality rule, including
    right values outside [0,9]; an unbounded Integer source cannot acquire a narrowing
    conversion by borrowing that sibling interval. Claim/native descriptor checks
    remain independent of generated output. If so'
- id: FR-018-AC-44
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'Genuine Union equality generation preserves the facade declaration key,
    settled node, authoritative member bindings and declared member/payload order
    across a nested Union closure, using the specified lookup and verified sealing
    in both generator and emitted preparation. In-process owner-API controls admit
    values of every member, including record and nested-Union payloads; distinct same-shaped
    unions retain incompatible identities. Missing/wrong capability, Unknown/Foreign/WrongKind
    lookup, name/form disagreement and seal faults retain their typed causes, no successful
    prepared wrapper and no '
- id: FR-018-AC-45
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: After the existing setting/accounting and original-package context guards
    succeed, generation-time construction and emitted preparation supplied Some(context)
    attach a clone of that context's complete authoritative unit_table() to the constructed
    TypeEnvironment, retaining declared but unused units without rebuilding a table,
    deriving UnitId/NodeKey or substituting another package's table. Valid context-free
    None behavior remains unchanged. Wrong-package refusal retains its whole-request
    precedence; direct and nested Quantity items retain their existing typed Unsupported
    behavior and refusal p
- id: FR-018-AC-46
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'Every claim-map entry carries the node id, IR id, package id, source map,
    claims, reconstructed declaration keys and selected schedule of its item. For
    every V2 node id its operand types reach, the declaration key is the exact owner-issued
    `NodeKey` selected through admitted QSL lookup: its package, ordered path segments
    and ordinal equal the admitted node''s structural identity. A malformed, unknown
    or foreign identity retains its typed owning refusal and cannot become a valid
    key. Claim selection and order remain deterministic; CG and emitted source do
    not construct or decode keys.'
- id: FR-021-AC-1
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: For a request whose required source views are admitted, every requested
    `call` item receives exactly one scheduled or typed-refused disposition; a refused
    item contributes no instruction root, concrete symbol or location entry, while
    an unrelated admitted item is unchanged. A plain-read V1 request with an eligible
    temporal-tag `Unsupported` record and `NotAdmitted` requested-root kind returns
    the whole-call AC-41 refusal before dispositions.
- id: FR-021-AC-2
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'For each function-application corpus vector, the oracle''s outcome, admitted
    charge sequence and consumed counters equal a direct application independently
    assembled from the request with a fresh meter through QSL FR-485 acquisition,
    callable selection and session application of the QSL interpreted callable, with
    no generated body registered in that direct registry. The historical Contract
    Runtime `exact` port cannot serve as the target or independent comparison. This
    criterion claims no agreement with the QSL value authority: that boundary is owned
    by quire-integration IR-842 (FR-014-AC-47, FR'
- id: FR-021-AC-3
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Each scheduled item records the requested function name, actual declaring
    V2 `function` node id, and zero-based index among surviving schedule declarations
    sorted by `FunctionScheduleDeclaration.function_node_id` (complete structural
    identity). Each concrete exact-oracle claim also records the actual V2 function
    node id separately from its `ExactFunctionDeclaration.node_id` body-root expression
    key, and its zero-based original source function index, matching the FR-485 checked
    callable origin even when earlier functions are omitted or refused. Permuting
    either API's caller declaration order or
- id: FR-021-AC-4
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: For an admitted callable, the oracle forwards arity, wrong-value-kind and
    dangling-reference Begin input refusals unchanged from `ApplicationBeginRefusal::Call(CallFailure::Input)`
    before any execution charge. An absent requested name retains CG `ExactFunctionRefusal::UnknownFunction`;
    an invalid owner or callable id is refused by FR-485 `lookup_callable` as `Unknown`,
    `Foreign(package_id)` or `WrongKind` before selection or Begin, never as a Begin
    input refusal. The schedule records ordered arguments without inventing a refusal
    result. Agreement with the QSL value authority remains owned by I
- id: FR-021-AC-5
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: The schedule orders arity validation before per-argument validation, validates
    arguments in declared parameter order, then places `function.call` before body
    work; the FR-485 session is the target for the matching owner-path corpus run.
- id: FR-021-AC-6
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A function whose declared operator requirements name a capability no registered
    backend can discharge is marked `unsupported` at generation time, naming the capability,
    before any item naming it is applied; no oracle is emitted for such an item, and
    the disposition is never an `Outcome` variant, never an `InputRefusal`, and takes
    no `Meter`.
- id: FR-021-AC-7
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A tree-shaped 100,000-node V2 `expression`/`call` graph whose nested children
    are separate admitted nodes reached by `reference` produces a flat SSA instruction
    table on a 512 KiB stack when caller work and source-byte limits fit it; no CG
    analysis or emission depth cap participates. The schedule crate compiles. This
    compilation is not a claim that a concrete FR-485 oracle ran; no Contract Runtime
    `MAX_CALL_DEPTH` behavior is required.
- id: FR-021-AC-8
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: The schedule marks only linked-package admission as applicable and never
    marks a kernel-mode package applicable.
- id: FR-021-AC-9
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: The schedule preserves a `function.call` charge point and an incomplete
    stop before body work, and the denied charge is not applied; FR-485 session application
    is the target for the matching denied-charge vectors.
- id: FR-021-AC-10
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A declared parameter or result type reaching a `reference` composite form
    is refused as blocked on quire-spec-language#120, with no schedule root or concrete
    code for that function. No meter charge is admitted for it; runtime charge evidence
    for reference-typed callables remains Planned.
- id: FR-021-AC-11
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: With an admitted requested-root kind, model and relation lowering records
    map to the quire-spec-language#120 blocker; state, temporal nodes other than directly
    requested `formula` and `fairness` roots, and protocol lowering records map to
    quire-spec-language#121; the blockers remain distinct. A requested temporal clause
    keeps that blocker even if its first unsupported reachable node is a formula or
    fairness child. A plain-read V1 temporal-tag `Unsupported` record with `NotAdmitted`
    requested-root kind follows AC-41 instead of guessing from the first reachable
    tag. TC-031's defensive Model/Stat
- id: FR-021-AC-12
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: "A declared function whose own body fails to lower \u2014 an unlowerable\
    \ node, a form none of this generator's classifiers admits, or a nested `call`\
    \ naming a function absent from the request \u2014 refuses every item bound to\
    \ that function with a typed reason, without changing an unrelated package's items."
- id: FR-021-AC-13
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Schedule source and claim-map bytes are identical across runs and request
    permutations in either API; entries order by call-node id, that API's declaration
    key (the actual function node id for `generate_function_schedules`, the admitted
    body-root expression id for `generate_exact_function_oracles`), then source argument-node
    ids, with the name tie-break AC-24 specifies. The exact-oracle claim still records
    the actual function node id separately as AC-3 requires.
- id: FR-021-AC-14
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: The schedule crate declares `publish = false`, compiles without a Contract
    Runtime `exact` dependency, contains no charge amount or literal outcome/value
    standing in for owner execution, and stays within caller-selected source bytes
    and publication artifact limits.
- id: FR-021-AC-15
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Each scheduled call point has one constant-size location record with function
    origin, stable occurrence ordinal, parent ordinal, child ordinal and instruction
    slot. A charged query reconstructs the same path as an independent walk of the
    request V2 reference graph; no record embeds the entire root-to-point path. A
    100,000-link chain uses O(100,000) map storage and generation work.
- id: FR-021-AC-16
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: The schedule and its claim/location maps neither read nor depend on dynamic
    invocation `location` or `losses` fields, and make no assumption that QSL populates
    them. The concrete AC-38/AC-39 guards assign their statically bound result/declaration
    occurrence at those explicit failure boundaries; these writes do not derive schedule
    metadata from invocation fields. Forwarding an existing evaluation preserves its
    fields without reading them into the maps.
- id: FR-021-AC-17
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Each location record uses the request function binding and function-body
    ordering for its origin; the runtime cross-check uses the FR-485 session's location
    derived from the declaring checked graph's root call site, for own and imported
    callables.
- id: FR-021-AC-18
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: No test name, trace tag, test output line or doc comment of the function-application
    corpus, no TC-031 step and no oracle-matrix row claims, or presents as evidence,
    agreement in outcome, refusal, charge order or consumed counters with the QSL-owned
    application authority; text that states this boundary is permitted. That agreement
    is not claimed by CG and is owned by quire-integration IR-842 (FR-014-AC-47).
- id: FR-021-AC-19
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: The emitted schedule crate for scalar, equality and nested-call corpus
    cases contains zero `.unwrap(`, `.expect(`, `unreachable!`, `panic!`, `todo!`
    and `unimplemented!` tokens; the emitted concrete FR-485 scalar, equality and
    nested-call oracle source obeys the same prohibition.
- id: FR-021-AC-20
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: For each scalar and equality instruction, the target-neutral schedule preserves
    completed, undefined, refused, incomplete and checked-invariant result distinctions
    without folding one into another. Forwarding FR-485's owner result variants remains
    Planned through IR-595.
- id: FR-021-AC-21
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'A scalar `Negate` body is refused as `UnsupportedOperator` with no schedule
    root, whether or not it reaches a `quire.op.quantity.convert` node (a conversion
    is not refused by name, because FR-021 has no quantity body, operand or argument
    support: the generator refuses a body that is one as `UnsupportedOperator` and
    the schedules as `ScheduleRefusal::UnsupportedOperation`; an earlier check that
    fires first is the accepted result: `Unsupported` for a direct unsupported form
    of the root, `NotExpression` and the other root checks, `UnsupportedOperandType`
    for a quantity-typed parameter or result, '
- id: FR-021-AC-22
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'When two or more declarations share one declaring node id, none of them
    appears in the instruction table or in `location-map.json`, and every item naming
    one of them is refused with `ExactFunctionRefusal::DuplicateDeclaringNode { node_id
    }` (the shared node id; the smallest in node-id order when the item''s name is
    held by duplicate groups on several node ids) before Stage 1 classification: never
    `UnknownFunction`, never `AmbiguousFunctionName` (the node-id refusal takes precedence
    when the declarations also share a name), never `DuplicateRequest` (two items
    on one `call` node naming different '
- id: FR-021-AC-23
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: For an admitted FR-330 package carrying a `Failed` feature-accounting record,
    a function item retains that exact typed record, its `limit_kind`, `limit`, `consumed`
    and source occurrences in `PackageAccounting`, without relabelling bytes as work
    exhaustion. Under a ceiling one byte below the shorter of the separately measured
    body-root and call-root package lengths, Contract IR FR-038-AC-95 returns `Failed(Bytes)`
    for every root in each request, one shared `consumed` per request above the ceiling,
    and neither request exposes canonical package bytes or `wire()`. The two measured
    `consumed` valu
- id: FR-021-AC-24
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'Items on one `call` node, with equal argument operands, that name functions
    absent from the request''s declarations each get their own claim-map entry. Requesting
    `zz_unknown` and `aa_unknown` on one node yields two entries, in that order: `UnknownFunction
    { name: "aa_unknown" }`, then `UnknownFunction { name: "zz_unknown" }` (function
    name compared byte-wise over its UTF-8 bytes, case-sensitively), and no `DuplicateRequest`;
    the entries are identical under both request orders. Requesting `Zz_unknown` and
    `aa_unknown` yields `Zz_unknown` first, because `Z` (0x5A) sorts before `a` (0x61).
    Reques'
- id: FR-021-AC-25
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'Across one body, the counter charges each node entry, child edge, emitted
    instruction and location record once in source traversal order. A setting equal
    to the required units emits a complete schedule; one less refuses that function
    as `GenerationWorkExhausted { limit, consumed: limit + 1 }` before the denied
    action, with no partial root or location record and no change to an unrelated
    function. A failed Contract IR lowering record keeps its lowering refusal and
    incurs no generation charge. PLANNED (IR-511).'
- id: FR-021-AC-26
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: The target-neutral schedule's branch/stop edges order a denied left-operand
    charge to stop before the right operand charge, and, after the left completes,
    the right charge before the parent operation. Owner-path execution matching an
    equivalent shallow FR-485 body remains Planned through IR-595.
- id: FR-021-AC-27
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: An admitted nested body consists of separate V2 expression nodes connected
    only by `reference` leaves under QSpec FR-322, with each expression node reached
    from one parent in this CG subset; callee argument 0 binds an exact V2 function
    node, parameter references bind the declared parameter slots, and remaining arguments
    preserve source order. Missing, cyclic, shared-expression, unbound-parameter,
    ill-typed or wrong-operation edges refuse with a typed cause before schedule emission.
    PLANNED (IR-511).
- id: FR-021-AC-28
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Under the default 1,048,576-byte source limit, a 100,000-instruction body
    that needs more bytes returns `SourceTooLarge` naming the default; raising the
    caller limit to its measured flat source bytes emits a complete compilable schedule
    crate, with deterministic ordinal-range modules if one artifact would exceed its
    publication limit. One byte below the measured count refuses before append with
    no partial output. PLANNED (IR-511).
- id: FR-021-AC-29
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A 100,000-call linear body produces 100,000 constant-size parent-linked
    location records; requesting its deepest path under sufficient query limits returns
    exactly 100,000 child ordinals, and a one-unit-below query limit refuses without
    a partial path. Generation does not materialize all paths. PLANNED (IR-511).
- id: FR-021-AC-30
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A requested generation work limit of `u64::MAX` returns `InvalidGenerationWorkLimit`
    before lowering or charging; at `u64::MAX - 1` the first denied count is representable
    in the `u64` refusal and never wraps or panics. PLANNED (IR-511).
- id: FR-021-AC-31
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: No test name, trace tag, test output line or doc comment of the 100,000-node
    case, no TC-031 step and no oracle-matrix row claims, or presents as evidence,
    that a generated oracle compiled, ran on a 512 KiB stack or agreed with a shallow
    equivalent through FR-485 without an actual owner-path run; schedule-source compilation
    (AC-7) is not execution evidence. The planned FR-485 corpus remains distinct from
    QSL value-authority agreement (FR-014-AC-47).
- id: FR-021-AC-32
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Neither the schedule crate nor any concrete oracle requires `quire_contract_runtime::exact`,
    an alias, wrapper or compatibility layer; a concrete oracle uses `qsl_replay::acquire_checked_closure`,
    `CheckedClosureHandle::lookup_callable` and `registry()`, `ApplicationRegistry::register`
    and `begin_call`, and `CheckedApplicationSession::step` or `run` through the permitted
    FR-485 facade, without direct QSL evaluator or checker internals. CG and any emitted
    oracle crate hosting a generated body depend directly on `quire-eval`, `quire-exact`
    and `quire-semantic-value` beside `qsl-replay`, not on Q
- id: FR-021-AC-33
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'With an IR-admitted `temporal`/`formula`, `temporal`/`fairness` or `expression`/`case`
    node used directly as an item''s call-node id, and an authenticated requested-root
    kind for an IR `Unsupported` lowering record, the claim-map entry retains that
    id and is `Refused { refusal: ExactFunctionRefusal::Unsupported }`. For the separate
    declared-body-root case, an item names that function through a distinct, IR-admitted
    `expression`/`call` node that lowers successfully and has no duplicate, capability,
    or earlier item refusal; its claim retains the call-node id and carries the body''s
    `ExactFunctionR'
- id: FR-021-AC-34
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Given one authentic strict-reader-admitted `CheckedPackageV2` of less than
    1,048,576 bytes containing 129 distinct functions with two bound Integer parameters
    each, 128 acyclic `quire.op.function.call` body-root edges forming one chain and
    a terminating admitted Scalar body, the public function generator schedules all
    129 functions and all 128 call edges from that same checked graph. Use the default
    1,048,576-byte Contract IR reader/lowering ceilings and default 1,048,576-byte
    generated-source ceiling; only the caller-selected generation work limit may be
    raised to the measured required units.
- id: FR-021-AC-35
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: For a concrete FR-485 corpus request, the caller supplies all seven acquisition
    inputs and the separately admitted Contract IR package. CG calls `qsl_replay::acquire_checked_closure`
    once for the generated run and receives `CheckedClosureCompilation` with emitted
    package, admitted context and retained `CheckedClosureHandle`; it does not reconstruct
    a QSL checked closure from the V2 graph or emitted source. Acquisition failure
    remains `AdmittedRefusal` and yields no handle.
- id: FR-021-AC-36
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'CG selects the own or imported declaring package explicitly by its admitted
    structural library name and compares its complete canonical V2 bytes, including
    the typed term sidecar, with the bytes retained by the same QSL acquisition. Imported
    bytes come from `CheckedClosureCompilation::owner_package_bytes`; own-unit bytes
    come from `package()`. CG resolves the function identity in its owning V2 frame
    through `ContractSourceKeyV1::from_v2_request(source, request)` and mechanically
    represents that structural key as the Exact `NodeKey`; no rehash, cross-frame
    index comparison, fabricated identity '
- id: FR-021-AC-37
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'A mismatch between the acquired unit''s structural name or complete canonical
    V2 bytes and the CG request''s admitted unit refuses the item as `ExactFunctionRefusal::OwnerIdentityMismatch`
    before registration, Begin, body invocation or application charge. The same refusal
    follows a mismatch between `CallableSelection::owner_package_name()` and the selected
    function''s declaring admitted library name, or between the selection''s read-only
    checker-issued structural key and that function''s source-resolved structural
    identity. The selection comparisons run in a private post-lookup guard; TC-031
    tests '
- id: FR-021-AC-38
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: With a genuine selected callable declaring Boolean result, mutate only
    the test copy of the emitted body to return Completed(Value::Integer(...)); the
    unchanged shipped result guard returns Done(Evaluation) with GeneratedUnexpectedOutcome
    at the actual result occurrence and the admitted charge prefix. The unmutated
    registered body completes with its admitted Boolean. No failure injection, caller-chosen
    result type or production hook is used.
- id: FR-021-AC-39
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Native preparation consumes each selected owner record/tuple/union declaration
    once, preserving original keys, ordered member types, required/optional presence
    and sealed union identities. A test-only copy resubmits one unchanged authentic
    declaration to the same QSV constructor; its genuine duplicate-key Refused(InvalidDeclaration)
    yields GeneratedEnvironmentRejected at the declaration occurrence before body
    operations/charges, retaining the original typed diagnostic. No synthetic declaration,
    empty fallback, injected error or production hook is used. Actual constructor
    WorkUnits with tripped
- id: FR-021-AC-40
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'The generated guard uses the same checker-issued selection.signature().result()
    and actual owner QSV environment, never a replacement result type. Undefined,
    Refused and Incomplete retain their variants, payloads, locations and admitted
    charge prefixes. Err(EnvironmentFailure) remains BodyFailure::QsvEnvironment /
    CallFailure::Body, never false, Completed or a genuine invariant fault. Boolean/Integer
    result membership allocates no storage: Allocation/Capacity mapping has inspection
    evidence with no execution credit. The first shipped allocating-result membership
    family activates the actual-req'
- id: FR-021-AC-41
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: For a plain strict-reader-admitted V1 package, after existing duplicate,
    capability and item-precedence checks, an eligible selected call item or declaration
    body with an admitted `Unsupported` accounting record whose `node_tag` is `"temporal"`
    and whose requested-root-kind view is `NotAdmitted` causes whole-call `OracleGenerationError::UnauthenticatedRequestedRootKind
    { request_index, node_id }` before claim map, schedule, oracle or partial publication.
    These V1 facts trigger refusal without deciding whether the requested root is
    direct `formula`, direct `fairness` or a temporal clause. For a
- id: FR-021-AC-42
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: The emitted function-oracle crate, compiled with `cfg(kani)`, declares
    every `body_{symbol}` module `pub(crate)` with a `pub(crate)` `Factory` and omits
    every `oracle_{symbol}` host entry, and compiled without `cfg(kani)` is unchanged
    (all `oracle_{symbol}` entries present and behaving as AC-35 through AC-37 require).
    A source-level check of the emitted text proves the `#[cfg(not(kani))]` gate on
    each host entry and the module visibility; the Kani obligation and its explicit
    exclusions are as stated in Behavior, and no test, doc comment or matrix row credits
    the host glue to a Kani proof.
- id: FR-031-AC-1
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). For each of Add, Subtract and Multiply over a `reject`
    integer type, the generated source of the native oracle calls `exact::evaluate_integer_arithmetic`
    with the matching `IntegerArithmetic` variant and the type's interval as the bound,
    and the oracle's body holds no Rust arithmetic operator between operand expressions
    and none of `checked_`, `wrapping_`, `saturating_` or `overflowing_` method calls.
- id: FR-031-AC-2
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED in the IR-1045 native oracle branch (formerly PLANNED under
    canceled IR-602). For Divide and Remainder over a `reject` integer type, the generated
    source of the native oracle calls `exact::divide` with `DivisionProfile::Truncating`,
    with `DivisionMember::Quotient` for a divide and `DivisionMember::Remainder` for
    a remainder, and with `IntegerDomain::Bounded` over the node's interval as the
    domain, and the oracle's body holds no Rust `/` or `%` and none of `checked_div`,
    `checked_rem`, `wrapping_rem`, `div_euclid`, `rem_euclid` or `modulo`, and no
    division of a quotient and remainde
- id: FR-031-AC-3
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). A native oracle with an arithmetic node returns `Outcome<bool>`
    and takes a trailing `&mut Meter`; an oracle with none returns `bool` and takes
    no meter.
- id: FR-031-AC-4
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). Each of the six comparisons over two arithmetic-free
    operands is emitted as the one native operator the table names, and over the seven
    values `i64::MIN`, `i64::MIN + 1`, `-1`, `0`, `1`, `i64::MAX - 1` and `i64::MAX`,
    with two interior values added, taken pairwise in both orders, agrees with the
    exact kernel's own evaluation, as the test takes it from Contract Runtime today
    (`order_numbers` for the four orderings, `check_equality` then `CheckedEquality::evaluate`
    for equal and not-equal; after the migration of FR-031-AC-22, `quire_exact`'s
    `order_numbers` and `planned_equ
- id: FR-031-AC-5
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-596). Under a `reject` integer type, an add, subtract
    or multiply result outside the type''s interval evaluates to `Refused(IntegerOutOfDomain)`,
    and the oracle neither panics nor returns a wrapped, saturated or clamped value.
    The cases are O-1 `i64::MAX + 1`, O-2 `i64::MIN - 1` as a subtract, and O-4 `i64::MIN
    * 2`, each built as TC-044 step 4 gives. On the tree before the change each fails:
    a debug build panics with an overflow message, a release build returns a wrapped
    value, and no case returns the refusal.'
- id: FR-031-AC-6
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED in the IR-1045 native oracle branch (formerly PLANNED under
    canceled IR-602). Under a `reject` integer type, a zero divisor evaluates to `Undefined(DivisionByZero)`
    for both Divide and Remainder, for the cases Z-1 `x / 0`, Z-2 `0 / 0`, Z-3 `x
    % 0` and Z-4 `0 % 0`, each built as TC-044 step 5 gives (Z-3 and Z-4 reach the
    zero through the add `y + 11`, which completes with 0), and the oracle does not
    panic and does not return `Refused`. On the tree before the change each case is
    refused at generation with the interim code, and a raw operator would panic with
    a division-by-zero messag
- id: FR-031-AC-7
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). An add, subtract or multiply over a `saturate` integer
    type is refused with `UnsupportedSaturatingArithmetic` at the node's span, with
    no artifact, for the rows `x + 1` at `x = i64::MAX` and `x - 1` at `x = i64::MIN`
    over an integer type of `i64::MIN..=i64::MAX`, and `x * 2` at `x = i64::MAX` over
    `0..=10` (where an in-domain `x = 10` also gives the wrong value `20 <= 10` against
    the policy's `10 <= 10`); the refusal span is the first such node in authored
    preorder when a negation also occurs later, and the diagnostic message names the
    missing runtime saturating operation
- id: FR-031-AC-8
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-596). In a generated crate compiled and executed against
    `quire-contract-runtime`, the outcome of a native add, subtract or multiply oracle
    equals a plain-integer model of IR FR-015''s member-range semantics, computed
    in the test with `i128` independently of the emitter and of any `exact::` call:
    `Completed` with the exact result when it lies in the type''s interval and `Refused(IntegerOutOfDomain)`
    when it does not. The grid covers operand values inside the declared domain, at
    its edges and outside it, and the outcome kind is compared, so no vector returns
    `Completed` where the '
- id: FR-031-AC-9
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-596). The first non-completed outcome in evaluation order
    is the oracle''s outcome: an arithmetic stop in the left operand of a short-circuit
    connective is returned, a stop in a right operand the left decided is never reached,
    a total connective returns the first stop of its two operands, and a stop is never
    returned as `Completed(true)` or `Completed(false)`.'
- id: FR-031-AC-10
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'OPEN for tri-state and bound-strategy consumers; the V1 Kani clause-lowering
    consumer and its tagged test retire under the 2026-10-27 IR-348 ruling. Original
    criterion retained: IMPLEMENTED (IR-596). The tri-state harness generator, the
    bound strategy generators and the Kani obligation clause lowering each refuse
    a clause holding an arithmetic node with the refusal the Consumers section names
    for it, emit no raw arithmetic operator, and never read an `Outcome<bool>` as
    `bool`.'
- id: FR-031-AC-11
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'RETIRED (2026-10-27 owner ruling, IR-348; V1 bundle-only, original criterion
    retained): IMPLEMENTED IN CG (IR-596); THE QSL EXEMPLAR RUN IS PENDING. The clause
    that runs QSL''s IT-011 and IT-010-SC-05 against this branch is pending the quire-integration
    run; CG''s in-repo run of the same bundle shape under real Kani is the closest
    equivalent and does not stand for it. `generate_kani_bundle` over the postcondition
    `amount < 1000` implies `amount + 1 <= 1000`, with `amount` an integer of `0..=1000`
    under `reject`, generates a bundle whose postcondition oracle returns `bool`,
    holds exactly one synt'
- id: FR-031-AC-12
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). Apart from the generated Rust header lines required
    by NFR-002, the oracle generated for an expression with no arithmetic node is
    byte-identical to the oracle the generator produced for it before this requirement.
    This criterion passes on the tree before the change and is a held agreement, not
    a defect proof.
- id: FR-031-AC-13
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). Generating the oracle of an expression with an arithmetic
    node twice, and from a permuted request, yields identical bytes. This criterion
    passes on the tree before the change and is a held agreement, not a defect proof.
- id: FR-031-AC-14
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). The generated source of an oracle with an arithmetic
    node contains no `unwrap`, `expect` or panic macro. This criterion passes on the
    tree before the change and is a held agreement, not a defect proof.
- id: FR-031-AC-15
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). In the native oracle, each of the six comparisons
    that has an arithmetic operand is emitted as the exact-kernel call the table names
    (for Equal and NotEqual, until the migration of FR-031-AC-22, the checked equality
    of Contract Runtime that the Comparison section records), with no Rust comparison
    operator between its two operands.
- id: FR-031-AC-16
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED in the IR-1045 native oracle branch (formerly PLANNED under
    canceled IR-602). Under a `reject` integer type of `-5..=5`, the minimum divided
    by negative one is out of domain and the minimum remainder negative one is not,
    and neither panics: O-3 `i64::MIN / -1` evaluates to `Refused(DivisionOutOfDomain)`
    whose member is `Quotient` and whose domain is `-5..=5`, with the refusal''s code
    `division_out_of_domain` and cause `quotient-outside-domain`; O-5 `i64::MIN %
    -1` evaluates to `Completed(true)`, the remainder being the exact 0 and `0 <=
    5` true. Each is built as TC-044 step 4 gives.'
- id: FR-031-AC-17
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED in the IR-1045 native oracle branch (formerly PLANNED under
    canceled IR-602). In a generated crate, the outcome of a divide or remainder oracle
    equals a plain-integer model of IR''s member-only semantics under the truncating
    law, computed with `i128` independently of the emitter and of any `exact::` call
    (zero divisor: `Undefined(DivisionByZero)`; the selected member in the type''s
    interval: `Completed` with the comparison''s Boolean over that member; otherwise
    `Refused(DivisionOutOfDomain)` with that member), on a grid of dividends and divisors
    inside the declared domain, at its edge'
- id: FR-031-AC-18
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'PARTIALLY IMPLEMENTED: the active native-consumer portion is implemented
    by IR-1045; the V1 bundle-consumer portion is RETIRED under the 2026-10-27 IR-348
    owner ruling. The original two-consumer acceptance value, formerly PLANNED under
    canceled IR-602, is retained verbatim as historical provenance; this status makes
    no active bundle-coverage claim. A divide or remainder node over a `saturate`
    integer type is refused with `UnsupportedSaturatingArithmetic` at the node''s
    span, with no artifact and no raw `/` or `%` in any output, for the `saturate`
    rows S-1 to S-4 of TC-044 step 11, in both consu'
- id: FR-031-AC-27
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'OPEN under IR-348 for obsolete error-code cleanup after IR-1045 native
    replacement. Original criterion retained: PLANNED (IR-602) (it cannot land before
    the native division of AC-2 exists). The code `UnsupportedIntegerDivision` does
    not exist: the crate''s `GenerationErrorCode` has no such variant, nothing under
    `src/`, `tests/` or `schemas/` spells it, no stable code string of that name is
    emitted, and no diagnostic message that generation can emit names IR-601. On the
    tree before the change the variant exists and a divide or remainder over `reject`
    is refused with it.'
- id: FR-031-AC-28
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-664). The V1 Boolean oracle admits a typed IR integer domain
    or literal only when its value and both inclusive domain endpoints fit i64. A
    minimum of `i64::MIN - 1` or `i128::MIN`, or a maximum of `i64::MAX + 1` or `i128::MAX`,
    produces `UnsupportedExpression` at the first affected expression source span
    and no generated source; the same rule applies to an out-of-range literal. Values
    are never narrowed, clamped or wrapped to create an oracle.
- id: FR-031-AC-19
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'RETIRED (2026-10-27 owner ruling, IR-348; V1 bundle-only, original criterion
    retained): IMPLEMENTED (IR-596). The Kani bundle oracle of an expression whose
    arithmetic nodes are add, subtract and multiply renders them as the infix `+`,
    `-` and `*` on `i64` with one binary operator per arithmetic node, contains no
    `exact::` call and no `checked_`, `wrapping_`, `saturating_` or `overflowing_`
    method (a remainder node''s `wrapping_rem` is FR-031-AC-23''s, not this criterion''s),
    takes no meter, and its file states that its arithmetic is checked by Kani and
    is not a native evaluator.'
- id: FR-031-AC-20
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'RETIRED (2026-10-27 owner ruling, IR-348; V1 bundle-only, original criterion
    retained): IMPLEMENTED (IR-596). Under real Kani, a harness that calls a bundle
    oracle with unconstrained operands at which IR''s discharge does not hold fails
    with Kani''s arithmetic-overflow check and a counterexample: the oracle of `x
    < 5 && x * 2 <= 10` over `0..=10` under `reject`, called with `x` unconstrained,
    fails the "attempt to multiply with overflow" check, and the counterexample''s
    playback value satisfies `x < 5` and overflows `x * 2` (any such value is valid;
    the assertion does not name one); a hand-writte'
- id: FR-031-AC-21
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'RETIRED (2026-10-27 owner ruling, IR-348; V1 bundle-only, original criterion
    retained): IMPLEMENTED (IR-596). For each of add, subtract and multiply, the bool
    the Kani bundle oracle returns equals the `bool` inside `Completed` of the native
    oracle of the same typed expression, on every vector of a grid inside the declared
    domain at which the expression''s guards pass and IR''s discharge holds, including
    the domain''s edges, with the bundle oracle compiled and run natively in a generated
    crate against `quire-contract-runtime` in plain `cargo test`, with no Kani. This
    is the differential that holds'
- id: FR-031-AC-22
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED in the IR-1045 native oracle branch (formerly PLANNED under
    canceled IR-602). The generated source of a native oracle with an arithmetic node
    aliases `quire_exact` as `rt`, takes `&mut rt::Meter`, returns `rt::Outcome<bool>`,
    and contains no `quire_contract_runtime::exact` path; a generated crate that compiles
    and runs it, with divide and remainder nodes and with add, subtract and multiply
    nodes in the same oracle (the construction Z-3, `y <= 0 && x % (y + 11) <= 10`,
    is one such oracle), resolves `quire-exact`; CG's own `Cargo.toml` depends on
    `quire-exact` directly from `agent-ix
- id: FR-031-AC-26
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED in the IR-1045 native oracle branch (formerly PLANNED under
    canceled IR-602). The generated source of a native oracle holds no `quire_contract_runtime::operators`
    path and no `and_short_circuit`, `or_short_circuit` or `implies_short_circuit`;
    a short-circuit connective is an inline `match` over the left operand's `rt::Outcome`
    that returns the first stop and does not evaluate the right operand when the left
    decides, and a total connective evaluates both operands in order and returns the
    first stop. Each completed connective, including unary BooleanNot, calls `quire_exact::retain_bo
- id: FR-031-AC-23
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'RETIRED (2026-10-27 owner ruling, IR-348; V1 bundle-only, original criterion
    retained): PLANNED (IR-602). The Kani bundle oracle renders Divide over a `reject`
    type as the infix `/` and Remainder as `wrapping_rem` on `i64`, one operator or
    call per arithmetic node, contains no `exact::` call and no `modulo`, `div_euclid`,
    `rem_euclid`, `checked_`, `saturating_` or `overflowing_` method, no `wrapping_`
    method other than `wrapping_rem` on a remainder node, takes no meter, returns
    `bool`, and its file states that its arithmetic is checked by Kani. `generate_kani_bundle`
    over a divide or a remaind'
- id: FR-031-AC-24
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'RETIRED (2026-10-27 owner ruling, IR-348; V1 bundle-only, original criterion
    retained): PLANNED (IR-602). Under real Kani, a harness that calls the bundle
    oracle of the construction Z-1 (a divide) with unconstrained operands fails the
    checks "attempt to divide by zero" and "attempt to divide with overflow", each
    with a counterexample, and one that calls the bundle oracle of the construction
    Z-3 (a remainder) with unconstrained operands fails the check "attempt to calculate
    the remainder with a divisor of zero" and no other arithmetic check; a probe harness
    that assumes the declared domain and '
- id: FR-031-AC-25
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'RETIRED (2026-10-27 owner ruling, IR-348; V1 bundle-only, original criterion
    retained): PLANNED (IR-602) (its native side calls the kernel). For divide and
    for remainder, the bool the Kani bundle oracle returns equals the `bool` inside
    `Completed` of the native oracle of the same typed expression, on every vector
    of a grid inside the declared domain at which the expression''s guards pass and
    the divisor is non-zero and IR''s discharge holds, including the domain''s edges
    and R-1 and R-2, and including the remainder `y != 0 && x % y <= 0` over `i64::MIN..=i64::MAX`
    at `x = i64::MIN`, `y = -1` (nat'
- id: FR-035-AC-1
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: A generated caller-ingress oracle accepts `&TextPayload`, the declared
    `TextType` and `&mut Meter`, calls `rt::admit_text` once, and returns the runtime
    outcome, retained text, original runtime provenance, charge sequence and consumed
    counters unchanged for all six profiles over representative empty, ASCII, combining,
    composed and supplementary payloads.
- id: FR-035-AC-2
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: Invalid UTF-8 bytes make `TextPayload::from_utf8` return `InvalidUtf8`
    before admission and leave the admission meter unchanged; `admit_text` itself
    is never claimed to accept those bytes.
- id: FR-035-AC-3
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: For each profile, `Text[1,1]` admits or refuses according to the profile
    length of the retained sequence, including `e` plus U+0301 under NFC and NFD;
    a length refusal occurs before `text.result-retain`, and denial of each reached
    charge point yields the direct runtime stop, charges and consumed counters.
- id: FR-035-AC-4
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: For each of the six profiles and both `Text[0,8]` and `Text[1,1]`, a real
    Kani harness symbolically ranges over all 21 sequences of zero to two scalars
    from the four-scalar alphabet, constructs each `TextPayload` by `from_utf8`, compares
    the generated admission oracle with a separate direct runtime call including outcome
    and metering, and ends with one satisfied cover after its assertions. The proof
    identity names this finite class and its type.
- id: FR-035-AC-5
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: In a copy of the generated oracle with its admission call or declared `TextType`
    altered so the oracle accepts a payload the direct runtime call refuses, real
    Kani falsifies the admission agreement assertion with concrete playback; restoring
    the call verifies over the same symbolic class.
- id: FR-035-AC-6
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: A request outside the declared finite proof class has a typed unsupported
    or requires-bound disposition and no harness, and no caller-ingress claim is marked
    `ir_confirmed` or assigned a checked-package node id. The six attempted text-operand
    `numeric.convert` nodes remain Contract IR refusals and do not count as passing
    caller-admission coverage.
- id: FR-042-AC-1
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: CG's native Boolean, equality and function constructors use the exact eight
    CG-owned causes in the table, retaining `IllTypedCause` from `check_type` and
    `check_equality`; downstream replay consumers preserve the typed payload or match
    it explicitly, retain charge/failure behavior, and accept neither the former unit
    constructor nor an invented default.
- id: FR-042-AC-2
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: The `GeneratedUnexpectedOutcome` producer executes a generated body that
    returns `Completed(value)`, compares that value against its selected callable's
    checker-issued result type via QSV `admits`, and reports only `Ok(false)` as this
    cause at the result location. A test-only emitted-source mutation supplies a reachable
    wrong completed value to the unchanged shipped check; `Undefined`, `Refused` and
    `Incomplete` retain their original outcomes and charges, and `Err(EnvironmentFailure)`
    is never a mismatch. For current Boolean/Integer results, Allocation/Capacity
    mapping is inspected with no exe
- id: FR-042-AC-3
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: The `GeneratedEnvironmentRejected` producer uses only real nominal declarations
    supplied by the selected checked-closure owner, once each in unmutated production
    source. A test-only mutation of emitted pre-body assembly submits one unchanged
    owner declaration twice to the shipped QSV constructor; exactly its genuine duplicate-key
    `EnvironmentFailure::Refused(InvalidDeclaration)` yields this cause before body
    execution at the declaration occurrence, with the typed rejection retained in
    diagnostic evidence. Successful construction does not yield it; `Limit`, `Allocation`
    and `Capacity` are never
- id: FR-044-AC-1
  path: spec/oracle/functional/FR-044-export-generated-closure-evaluation-corpus.md
  role: examined
  excerpt: 'Given an admitted scalar, an equality and a nested generated-to-generated-to-interpreted
    source application, `make evaluator-corpus` writes one stable unique case id per
    application in `cases.jsonl` at `EVALUATOR_CORPUS_DIR`. Every line is a `quire-eval-corpus`
    `quire.evaluator-case/v1` record with FR-402''s exact members: four source-identity
    labels, display path and text, sorted dependency units, QSL-compiled `package_id`,
    callable function name, FR-071 arguments, decimal-string limits, `object_environment:
    null`, `generated_bodies: []` and `expected: null` while `quire-eval` has no registere'
- id: FR-044-AC-2
  path: spec/oracle/functional/FR-044-export-generated-closure-evaluation-corpus.md
  role: examined
  excerpt: Repeating export of the same cases is byte-identical. Loaded case ids equal
    an independent enumeration of the live QSL-admitted source applications; deleting
    a record fails that census. Perturbing a callable, argument, dependency or limit
    changes the loaded case or causes FR-402's stated load/admission refusal. A producer-side
    compile, case-construction, case-encoding or load failure leaves the previous
    complete bundle byte-identical, or the target absent when none existed.
- id: FR-044-AC-3
  path: spec/oracle/functional/FR-044-export-generated-closure-evaluation-corpus.md
  role: examined
  excerpt: Inspection finds CG uses `quire-eval-corpus` for case writing and loading,
    obtains `package_id` via `qsl_replay::compile_package` inside CG's replay module,
    defines no local case/compared-record codec or loader refusal, and computes no
    expectation from emitted source or Contract Runtime `exact`. CG uses no `qsl-eval`
    dependency or direct QSL evaluator call outside the existing replay facade.
- id: FR-044-AC-4
  path: spec/oracle/functional/FR-044-export-generated-closure-evaluation-corpus.md
  role: examined
  excerpt: Once `quire-eval` implements registered generated-body execution, each
    selected case records a non-null `ComparedRecord` from one run with its sorted
    unique generated-body function names and fresh meter; FR-402's expected/std check
    passes. Perturbing a generated-body name changes the loaded case or causes FR-402's
    stated refusal. Boolean and Integer completed values encode; a completed value
    of another kind refuses `unencodable_value`. The compared record includes the
    exact result arm, refusal, location, losses and all ten consumed counters, with
    `charges` present only under FR-402's non-trunc
- id: FR-016-AC-1
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: Every retained witness decodes into typed values or is refused as malformed.
- id: FR-016-AC-2
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: An out-of-domain witness is reported as such and never as a contract failure.
- id: FR-016-AC-3
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: An in-domain witness is reported as a failure only when native replay reproduces
    the same typed outcome.
- id: FR-016-AC-4
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A native replay that runs and disagrees with the harness result yields
    a typed mismatch, and never a failure or a replay error.
- id: FR-016-AC-5
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A witness bound to a different harness identity is reported as malformed
    and never replayed.
- id: FR-016-AC-6
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: Every witness decode receives required caller-supplied byte and value-count
    bounds at its entry point; replay inputs carry both as non-optional fields with
    no default, and an over-bound witness is reported malformed without reading beyond
    either bound.
- id: FR-016-AC-8
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A witness schema binds the obligation's persisted argument bindings, in
    the order the obligation persists them, position for position to the harness's
    symbolic arguments, and each decoded value is named by the binding at its position;
    a binding that is not an argument is refused with a typed schema refusal that
    reports no failure and is none of the replay results.
- id: FR-016-AC-9
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A decoded falsification whose witness makes the native function evaluate
    the clause to false settles as `reproduced-with-evaluated-witness` with category
    `violation`; the verdict depends on the witness value, so a witness at which the
    function holds does not settle so.
- id: FR-016-AC-10
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A decoded falsification at which the native function evaluates the clause
    to true settles `inconclusive` with the proved violation and the replayed success
    both named.
- id: FR-016-AC-11
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: The adapter refuses, with a distinct typed error each, a decoded value
    no replay parameter binds, a transcript field holding a delimiter, a transcript
    QSL does not admit, a request QSL refuses with any `ReplayRefusal` other than
    `Fault` (returned with its cause), and a witness-sourced request that settles
    on the input arm.
- id: FR-016-AC-12
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A `replay` call that returns `ReplayRefusal::Fault` is returned as `SpineReplayError::Refused`
    carrying that fault.
- id: FR-016-AC-13
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A `Witness` arm that settles `reproduced-with-evaluated-witness` with a
    category other than `violation` yields a typed mismatch, and never a reproduced
    failure.
- id: FR-016-AC-14
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A proved unit that imports a locked dependency is compiled by `qsl_replay::call_site`
    together with that dependency's lock source, and `qsl_replay::replay` settles
    the unit's falsification through the imported function.
- id: FR-016-AC-15
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: RETIRED, replaced by FR-016-AC-29; the selected QSL dependency wire has
    no `package_id` member.
- id: FR-016-AC-16
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A unit whose import no lock dependency selection supplies is refused at
    the call site.
- id: FR-016-AC-17
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: RETIRED, replaced by FR-016-AC-30; a dependency `package_id` mismatch cannot
    be represented by the selected QSL wire.
- id: FR-016-AC-18
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A lock selecting a library the unit does not import is refused by QSL as
    unselected.
- id: FR-016-AC-19
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: Lock libraries whose sources share a source owner are refused as no dependency
    input, before the call site compiles anything.
- id: FR-016-AC-20
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A lock library whose source has the unit's own source owner is refused
    by the call site.
- id: FR-016-AC-21
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: RETIRED, replaced by FR-016-AC-26; the former CG-minted O-09 digest preimage
    is obsolete and earns no implementation credit.
- id: FR-016-AC-22
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: RETIRED, replaced by FR-016-AC-27; the former kind/domain-inside-digest
    control is obsolete and earns no implementation credit.
- id: FR-016-AC-23
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: RETIRED, replaced by FR-016-AC-28; the former CG identity-derivation control
    is obsolete and earns no implementation credit.
- id: FR-016-AC-24
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A lock whose only defect is one library identity selected twice is refused
    by QSL's `DependencyInput::new` as `DuplicateIdentity`, code `invalid_package`
    with cause `conflicting-definition`, and reaches the caller as `DependencyLockError::Input`.
- id: FR-016-AC-25
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: In an installed-Kani run that classifies a generated harness as `Falsified`,
    the retained counterexample is byte-identical to the first eligible assertion
    playback for that harness in the same launcher's complete captured stdout order,
    excluding cover blocks, and native replay uses values decoded from that retained
    counterexample; a `Verified` run supplies no counterexample. A test oracle observing
    stdout independently compares the complete fence interior, trimmed only of surrounding
    whitespace, with the retained counterexample before decoding or replay. Substituting
    another valid, self-consis
- id: FR-016-AC-26
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: PLANNED (IR-1079; replaces AC-21). The function replay request carries
    the producer's exact QSL `ObligationIdentity(NodeKey, Option<ModelIdentity>, ordinal)`
    unchanged, with no CG hash, preimage, rederivation or transcript-sourced identity.
    A function harness missing a parameter argument, declaring an unbounded integer
    or bounding a Boolean refuses before replay with no substituted domain. The selected
    function occurrence, obligation kind and complete parameter bindings with declared
    domains remain on the record, sorted by declared parameter identifier, and are
    compared exactly before replay.
- id: FR-016-AC-27
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: PLANNED (IR-1079; replaces AC-22). Two function nodes with the same parameters
    have distinct structural obligation identities; comment, blank-line and source-span
    edits preserving node/model/ordinal leave identity unchanged. For two distinct
    obligations on one node/model, the producer assigns distinct ordinals. Changing
    kind or one argument domain with a held identity fails the field-by-field record/claim
    comparison before replay; one requested kind for a two-conjunct function remains
    one obligation, not one per conjunct.
- id: FR-016-AC-28
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: PLANNED (IR-1079; replaces AC-23). The function path obtains the producer's
    node/model/ordinal identity, without deriving one from package contents. Perturbing
    unrelated declarations or parameter order while the producer identity and sorted
    argument bindings stay equal leaves the carried identity and accepted comparison
    unchanged; changing the selected function node, model or assigned ordinal changes
    identity, while changing the occurrence key, kind or argument domain under the
    same identity refuses before replay.
- id: FR-016-AC-29
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: PLANNED (IR-1079; replaces AC-15). The replay request's `package.dependencies`
    carries exactly one `DependencyEntryWire { identity, sources }` per lock dependency
    selection, in ascending identity order. Each entry preserves that selection's
    exact dependency identity and complete selected source references; the byte provision
    supplies the bytes named by those references. The wire carries no dependency `package_id`
    or lock field.
- id: FR-016-AC-30
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: PLANNED (IR-1079; replaces AC-17). Before QSL replay, CG compares each
    replay request dependency's `identity` and complete exact `sources` references
    with the corresponding independently admitted package/dependency source selection.
    A missing, extra, reordered or unequal selection, including a changed but valid
    provisioned source under the same imported dependency identity, yields a typed
    CG dependency-selection mismatch refusal before dispatch and no replay verdict.
    CG never substitutes a same-identity source or fabricates a dependency `package_id`
    wire field; QSL's acceptance of a separately
- id: FR-024-AC-1
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: RETIRED, replaced by FR-024-AC-47; the former CG-minted O-09 digest scheme
    is obsolete and earns no implementation credit.
- id: FR-024-AC-2
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: Every backend-witness transcript the generator passes to `Witness::parse`
    is produced by the one adapter rendering function from decoded values, and that
    function takes no backend-native text as input.
- id: FR-024-AC-3
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A counterexample with any value outside its declared domain is reported
    out-of-domain and neither `replay`, `replay_frame` nor `replay_state_clause` is
    called. The endpoints of each declared domain are admitted, and the values immediately
    outside it are not.
- id: FR-024-AC-4
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: For each common required `WitnessPacket` member in turn, excluding the
    three alternative replay-target members of FR-024-AC-41, a packet missing that
    member is refused by `WitnessEnvelope::reconstruct` with QSL's `MissingMember`
    naming it, and no replay runs. A packet whose trace position is present with the
    value none is admitted.
- id: FR-024-AC-6
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A frame counterexample is submitted as a `WitnessEnvelope<FrameCounterexample>`
    through `qsl_replay::replay_frame`, and its clause, frame, anchor and occurrence
    identities reach QSL unchanged.
- id: FR-024-AC-7
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A reduced `Witness`-arm candidate is retained only when a backend re-run
    and native replay both preserve domain validity and the failure; a candidate that
    changes domain validity or the verdict is discarded.
- id: FR-024-AC-8
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A retained reduced `Witness`-arm envelope carries its own backend re-run's
    transcript, never the original counterexample's.
- id: FR-024-AC-10
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: No type named `Witness`, `ReplaySource`, `WitnessEnvelope`, `TerminalRecord`
    or `ObligationIdentity` is defined under `src/`, and no source file under `src/`
    imports any of them from `quire_contract_ir`.
- id: FR-024-AC-11
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: '`StateClauseReplay::replay` returns exactly the `StateClauseReplayResult`
    that `qsl_replay::replay_state_clause` returns when a test calls it directly with
    the same request and envelope, for a violating run and for a respecting run, and
    the two results differ from each other. `StateClauseReplay::replay_through` hands
    the executor the request and envelope it built and returns the executor''s value
    as it is: an executor returning a sentinel result changes the returned result
    to that sentinel.'
- id: FR-024-AC-12
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The state-clause envelope's `clause_node` equals the `ClauseSite` node,
    and its `occurrence_key` the `ClauseSite` occurrence, that `qsl_replay::call_site`
    returns for the clause name. Its packet and request carry `state_clause` as that
    clause's declared `QualifiedName`, with neither other replay target; its payload
    has no `clause` selection and its `observation` is the `Invocation` arm. For a
    unit with two postcondition clauses, each clause's envelope carries its own node,
    occurrence and declared name. A clause name the unit does not declare returns
    `StateClauseReplayError::CallSite` holding t
- id: FR-024-AC-13
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: RETIRED, replaced by FR-024-AC-52; the former document-digest address and
    changed-digest checks are obsolete and earn no implementation credit.
- id: FR-024-AC-14
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: RETIRED, replaced by FR-024-AC-53; the former document-digest equality
    check is obsolete and earns no implementation credit.
- id: FR-024-AC-15
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A playback that binds no value for a declared state field returns `StateClauseReplayError::MissingField`
    naming that field, the executor is not called, and no snapshot holds a default
    for it. A playback or post-state name outside the path's declared field list returns
    `StateClauseReplayError::UndeclaredField` and a field bound twice returns `DuplicateField`,
    each naming the field, in the playback and in the post state alike.
- id: FR-024-AC-16
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: For a subject mutated to violate the postcondition, the replay settles
    `reproduced-with-evaluated-witness` with category `violation` and an evaluated
    `false`. For the unmutated subject's run over the same pre state it settles `inconclusive`
    with cause `Verdicts` (`violation` proved, `success` replayed). The envelope is
    on the `Witness` arm with a payload `witness` of none in both cases.
- id: FR-024-AC-17
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A state field value outside its declared integer range returns `StateClauseReplayError::OutOfDomain`
    naming the field and the executor is not called. The range's two endpoints are
    admitted and the values one below and one above are not.
- id: FR-024-AC-18
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: With the installed backend, the falsified operation-contract harness of
    a postcondition state clause over a subject mutated to debit is replayed from
    its real Kani playback through `replay_state_clause` and settles `reproduced-with-evaluated-witness`,
    `violation`. The test is `tc_035_real_kani_state_clause_counterexample_replays_through_qsl`
    in the module `kani_obligations_state_clause_replay`, so the `kani_obligations`
    filter of `make kani` selects it.
- id: FR-024-AC-19
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'An operation that declares a parameter, and one that declares a result,
    each return `StateClauseReplayError::UnsupportedOperationShape` carrying the operation
    and the declaration (its parameters and result), build no document, and do not
    call the executor; an operation that declares neither is not refused for its shape.
    The shape check reads the admitted package the caller supplies, and QSL''s admission
    reads the documents in `ReplayRequestWire.documents`; where the two disagree,
    the disagreement surfaces as a QSL refusal and not as this error. The error is
    not read as a QSL refusal: FR-029-AC-'
- id: FR-024-AC-20
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: RETIRED, replaced by FR-024-AC-48; the former four-member O-09 preimage
    and encoder are obsolete and earn no implementation credit.
- id: FR-024-AC-21
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: RETIRED, replaced by FR-024-AC-49; the former kind/occurrence-inside-digest
    control is obsolete and earns no implementation credit.
- id: FR-024-AC-22
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: RETIRED, replaced by FR-024-AC-50; the former frame preimage member rule
    is obsolete and earns no implementation credit.
- id: FR-024-AC-23
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: '`StateFrameIdentity` and its persisted record carry `state_fields`, every
    field the harness draws in draw order, which `domains` (ranged fields only) and
    `property`''s granted and checked lists (unordered) do not give. `state_fields`
    is used for decoding and ordering only and is not minted. A field with no declared
    range is listed in `state_fields`, has no entry in `domains`, is decoded and is
    not range-checked or refused (the AD-003 owner may tighten that later). A regenerated
    harness from equal inputs has a byte-identical record, and a record without `state_fields`,
    or one that lists a state '
- id: FR-024-AC-24
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: RETIRED, replaced by FR-024-AC-51; the former frame identity minting rule
    is obsolete and earns no implementation credit.
- id: FR-024-AC-25
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'The frame witness is built from the real playback: the playback text decodes
    through the one decoder of `src/replay/witness.rs` against bindings built from
    the harness''s `state_fields` (each `i64`, in draw order), and the transcript
    passed to `Witness::parse` is rendered by the one adapter rendering function from
    the decoded values, with the harness path and the check text the decode names.
    The fixed per-operation assertion text that `frame.rs` passes to `Witness::parse`
    today is gone. A playback of another harness, a playback with the wrong number
    of values and one with a wrong-width value ea'
- id: FR-024-AC-26
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A decoded state-field value outside the field's declared range returns
    `FrameReplayError::OutOfDomain` naming the field and its value, and calls neither
    `call_site` nor `replay_frame`; the range's two endpoints are admitted and the
    values one below and one above are not. A field with no declared range is not
    range-checked.
- id: FR-024-AC-27
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'The decoded pre-state is tied to the invocation: for every state field,
    the integer the pre snapshot named by the invocation holds for the object the
    invocation''s `self` addresses equals the decoded value. A differing field returns
    `FrameReplayError::PreState` naming the field, decoded value and snapshot value.
    An invocation or pre snapshot absent from the provided documents, not in the state-clause
    path''s typed document shape, or lacking the object or a field returns `PreState`
    naming what is missing. None calls `replay_frame`. A forbidden-write playback
    replayed against an invocation of a di'
- id: FR-024-AC-28
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A harness whose `scope.operation` is not the operation requested returns
    `FrameReplayError::ScopeMismatch` naming `operation` before `call_site` is called;
    one whose `scope.anchor` or `scope.frame` is not the anchor or frame `call_site`
    names returns `ScopeMismatch` naming that member and does not call `replay_frame`.
    The harness's module and harness symbols name the generated artifact, are not
    members of the identity, and are checked only by the decode (FR-024-AC-25).
- id: FR-024-AC-29
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The frame envelope's `declared_domains` is the empty list, no source of
    `src/replay/frame.rs` builds a `DeclaredDomain` or a `DomainKey`, and the module's
    header states, once, that the declaration is empty until QSL-345 settles the declared-domain
    key and refuses an empty declaration. The frame path adopts no key shape before
    then, whatever shape another path builds.
- id: FR-024-AC-30
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: PLANNED (IR-1086, IR-624). With the installed backend, the real playback
    of the falsified frame harness of a subject that writes a forbidden field is replayed
    through `FrameReplay::new` and `replay`, the caller supplying the harness's `StateFrameIdentity`,
    the parsed playback produced from the falsified outcome's complete counterexample
    text by the bounded Kani output reader, and the inputs of FR-024's Inputs list
    but no obligation identity and no transcript, and settles a reproduced violation
    naming the written field. The caller parses under byte and value limits no wider
    than the component-w
- id: FR-024-AC-31
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. For a QSL-emitted package, `StateClauseReplay` takes names and order
    from its inputs'' `state_fields` and validates each name with `model_object_fields(&object_id)`:
    for the twin''s `balance` and unread `audit`, both are present with the accessor''s
    0 to 1000 inclusive range. A playback or post state omitting one returns `MissingField`;
    a bound name outside the list returns `UndeclaredField`; a listed name absent
    from the accessor returns `ModelFields { object, cause: Absent { field } }`, and
    an accessor er'
- id: FR-024-AC-32
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. A frame harness generated from the package QSL emits for the twin is
    replayed through `FrameReplay::new` and `replay`: its generated `scope.anchor`
    and `scope.frame` equal the ids `call_site` names, asserted before replay; `Twin::aligned`
    is gone from test support; a forbidden write settles `reproduced-with-evaluated-witness`
    with category `violation` naming the written field; its `state_fields` retain
    request order and its `domains` equal the accessor''s ranges for listed fields,
    including an unread rang'
- id: FR-024-AC-33
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. A postcondition harness generated from the package QSL emits for the
    twin, with no identity alignment, is replayed through `StateClauseReplay::new`
    and `replay`: the debit mutation settles `reproduced-with-evaluated-witness` with
    category `violation` and evaluated `false`; the unmutated subject over the same
    pre state settles `inconclusive` with cause `Verdicts` (FR-024-AC-16); the clause
    id is the one `call_site` names for `BalanceNeverDrops`, and the declared field
    ranges equal those returned by the ac'
- id: FR-024-AC-34
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. With the installed backend, the real playback of the falsified frame
    harness of a forbidden write (FR-024-AC-30) and the falsified postcondition harness
    of a debit mutation (FR-024-AC-18) replay and settle as those criteria state,
    each harness generated from QSL's emitted package using accessor-derived ranges.
    The modules `kani_obligations_state_frame` and `kani_obligations_state_clause_replay`
    are selected by the `kani_obligations` filter of `make kani`.
- id: FR-024-AC-35
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. A falsified frame or postcondition harness draws a present model field
    whose accessor type has no `i64` range (for example `IntRange` with a lower endpoint
    below `i64::MIN` and an upper endpoint of -1), and playback binds an `i64` value
    outside that model range. QSL refuses the pre state and settlement is `Inconclusive`
    with `ReplayRefused` (FR-029-AC-16), naming the field and its persisted `TypeNotRange`
    reason; it never reports `Verified` or a violation. Playback inside the model
    range is unaffected, a
- id: FR-024-AC-36
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'Two distinct test backend adapters each produce a typed outcome: success
    carries equivalent assignments, each adapter''s backend identity, the same opaque
    obligation identity, declared domains, source/provenance and replay packet members;
    decode refusal carries only backend identity, obligation identity and typed cause.
    Native text is observed only within its own adapter; the common intake neither
    parses native text nor remints the obligation identity, and QSL receives each
    successful adapter''s original backend and source identity in a complete packet.'
- id: FR-024-AC-37
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The common intake maps an adapter's decode-refusal arm to a typed decode
    evidence failure retaining its cause, with no envelope or replay. Before native
    replay, an out-of-domain assignment yields a distinct typed domain evidence failure;
    an incomplete or QSL-refused witness/packet yields a distinct typed QSL admission
    evidence failure retaining its cause. A replay verdict disagreement remains a
    distinct typed evidence failure. None settles as contract success or contract
    failure, and none of the pre-replay failures invokes replay. Kani's exact decode
    and refusal partition remain governed by FR
- id: FR-024-AC-38
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'Starting from one admitted original failure, one valid reduced candidate
    that preserves the failure becomes a new CG lineage revision referring to its
    exact admitted QSL envelope, its exact parent revision and the unchanged opaque
    obligation identity. After retention, the original QSL envelope compares equal
    to its pre-reduction clone by `WitnessEnvelope<P>: Eq`, and the original CG record''s
    revision identity, parent, obligation and typed evidence fields read back equal
    to their captured values. No lineage member is added to QSL''s envelope or packet.'
- id: FR-024-AC-39
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: From a retained failure revision, an out-of-domain reduction and a domain-valid
    reduction that changes the native verdict each create no revision. After each
    rejection, every prior QSL envelope compares equal to its pre-proposal clone and
    every prior CG record's revision identity, parent, obligation and typed evidence
    fields equal their captured values; a later valid reduction names the actual retained
    parent, with no skipped or substituted parent.
- id: FR-024-AC-40
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A retained reduced `Witness` envelope holds a transcript from its own backend
    re-run, bound to its own reduced assignments; it cannot reuse any ancestor's transcript.
- id: FR-024-AC-41
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A function counterexample packet and its admitted `WitnessEnvelope` carry
    `function` as the selected function's `QualifiedName`, with no `frame_operation`
    or `state_clause`; a frame counterexample packet and envelope carry `frame_operation`
    as `{object, operation}` from QSL's validated typed call-site result, with `object`
    naming the operation's declaring object type, process or state machine (also when
    the caller selected an inherited operation) and `operation` its `Identifier`,
    and no `function` or `state_clause`; a postcondition state-clause counterexample
    packet and envelope carry `state_c
- id: FR-024-AC-42
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A function, frame or state-clause packet with none of `function`, `frame_operation`
    and `state_clause` is refused as a missing replay target; one with any two or
    all three is refused as a conflicting replay target. Each refusal is typed and
    distinguishable, `WitnessEnvelope::reconstruct` admits no envelope, and no replay
    executor is called, even when every common member is present. An absent selection
    is not replaced by an empty name, an inferred payload operation or clause, or
    `selected_function`.
- id: FR-024-AC-43
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'A function-contract adapter supplies a completed `WitnessPacket<NoPayload>`
    carrying its decoded witness and retained proving-run members. CG admits it through
    `WitnessEnvelope::reconstruct` before request construction and requires its `Witness`
    source arm. The request builder accepts the admitted envelope and request-only
    members, and copies exactly its function `declaration` occurrence key, `function`
    target and opaque `obligation_identity`; it accepts no bare key or selected-site
    substitute. Omitting the producer packet, substituting a different admitted function
    occurrence or changing its '
- id: FR-024-AC-44
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: An input-arm parity claim producer supplies a completed `WitnessPacket<NoPayload>`
    before its request builder. CG admits it through `WitnessEnvelope::reconstruct`
    and requires its `Input` source arm. The operator-parity request copies that packet's
    checked scalar application occurrence key, and the value-parity request copies
    its function `declaration` occurrence key, target and opaque obligation identity.
    Typed request serialization and readback preserve each full key, including two
    admitted occurrences of one node and an `expression` occurrence with a nonzero
    ordinal. No request builder mint
- id: FR-024-AC-45
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A `function` replay request missing `occurrence_key` refuses through QSL's
    request reader as `invalid_runtime_input`/`missing-member` at `/replay/occurrence_key`
    before byte provision and recompile; the same request with a byte-provision entry
    also removed reports the same refusal. Neither refusal invokes replay or becomes
    a contract verdict.
- id: FR-024-AC-46
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: Before constructing a function-contract, operator-parity or value-parity
    request, CG compares the admitted packet's `function` target, complete occurrence
    key, subject `clause_node`, `package_id`, source arm and opaque obligation identity
    with the independently selected claim from the admitted package and proving-run
    identity or original checked parity claim, never from that packet. A wrong target,
    key node, role or ordinal, subject node, package, source arm or obligation identity
    returns a typed claim-binding refusal naming the mismatched member, with no request
    or replay. A different valid o
- id: FR-024-AC-47
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'PLANNED (IR-1079; replaces AC-1). The envelope and replay request carry
    the exact producer-issued QSL `ObligationIdentity(NodeKey, Option<ModelIdentity>,
    ordinal)` for the selected function or frame, unchanged by CG. Source-span, harness
    symbol, solver and options changes leave that structural identity unchanged. Changing
    node, model or producer-assigned ordinal changes it. Kind, complete occurrence
    and sorted argument domains remain distinct obligation-record members; changing
    any one under a held structural identity refuses the record/selected-claim match
    before replay. The producer assigns '
- id: FR-024-AC-48
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: PLANNED (IR-1079; replaces AC-20). The function and frame paths use the
    QSL producer identity as a typed three-component value, with no CG preimage object,
    identity encoder or digest calculation. The function obligation record retains
    the selected function occurrence, kind and complete parameter bindings sorted
    by declared identifier, each with its declared Boolean or bounded-integer domain;
    the frame record has its frame occurrence, `frame` kind and empty argument list.
    Equal record and claim members admit the same identity; a member mismatch refuses
    before replay.
- id: FR-024-AC-49
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: PLANNED (IR-1079; replaces AC-21). Structural identity changes for a different
    frame node, owning model or assigned ordinal and remains equal for unchanged components
    across comments, blank lines, source-span, symbols, solver, options and state-field
    display changes. A change of frame grant that rekeys its producer node changes
    identity; if the node remains unchanged, full grant and field comparison detects
    a changed body. A changed kind or occurrence under held identity refuses before
    replay. Distinct postcondition claims of one operation retain their own complete
    occurrences even if they sha
- id: FR-024-AC-50
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: PLANNED (IR-1079; replaces AC-22). A frame record and envelope carry the
    exact `OperationSite` frame node and frame occurrence; the obligation identity
    carries that node, the producer-supplied optional model and producer-assigned
    ordinal. The record kind is `frame` and its arguments are empty. A postcondition
    property returns `FrameReplayError::NotAFrame`; a frame whose granted and checked
    fields differ from its state fields returns `FrameReplayError::FieldSetMismatch`;
    neither calls `call_site` or `replay_frame`. Field and range controls remain checked
    outside structural identity.
- id: FR-024-AC-51
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: PLANNED (IR-1079; replaces AC-24). `FrameReplayInputs` has no `obligation_identity`
    member. The frame request and envelope carry the same producer-issued typed identity
    from the selected frame claim; no caller stand-in, CG mint or hash reaches either.
    A changed grant that rekeys the producer frame node changes both identities; a
    grant/body mismatch under a held node is refused by the full record comparison.
    The state-clause path remains outside this criterion.
- id: FR-024-AC-52
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: PLANNED (IR-1133; replaces AC-13). The pre snapshot holds, for every declared
    field of the state object, the integer Kani playback bound to that field, and
    the post snapshot holds the integer post-state value the caller supplied for it;
    each has one `complete` population with one object and no defaulted field. The
    invocation document holds every member stated in Behavior, with `model` naming
    the admitted owner by identity only, `parameters` empty, `result` null, `created`
    and `deleted` empty, and `pre` and `post` each naming the respective snapshot
    by its four identity labels alone. CG appends
- id: FR-024-AC-53
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'PLANNED (IR-1133; replaces AC-14). For a document value with members in
    non-sorted source order, text containing a quote, backslash, control character
    and non-ASCII letter, and numeric members containing a large integer, a fractional
    number and an exponent form, the state-clause builder''s encoded bytes equal `core::canonical`''s
    bytes for that value exactly. Separately, take the valid builder output, change
    only JSON object-member order and whitespace, and supply those raw bytes directly
    to QSL''s `ReplayRequestWire.documents` with the same self-declared identity:
    QSL admits the same typed docum'
- id: FR-032-AC-1
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    A public consumer passes the routed harness's `ScalarObligationIdentity` directly
    to the scalar builder; its typed call constructs neither `KaniObligationIdentity`
    nor V1 `ReplayPackage`. After the ordered setup checks, an otherwise valid request
    returns unsupported-capability only while a named step-5 capability is unavailable;
    when all are available it returns an admitted plan, and authenticated completion
    enables replay/conversion. Conversion of a capability refusal returns no settlement
    rather than a synthe
- id: FR-032-AC-2
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631). The builder decodes actual routed-harness playback in
    persisted symbolic call order through the Kani output adapter; another harness
    name, missing/extra operand, wrong byte width, playback above the builder-owned
    8 MiB per-harness ceiling or malformed assertion block yields the distinct typed
    refusal under the stated precedence before replay.
- id: FR-032-AC-3
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    Each operand at its own lower or upper bound is admitted and the adjacent out-of-domain
    value is refused before replay. The admitted QSL inputs preserve the catalogued
    operator, operands, operand ranges, result range and harness limits. Another node/operation,
    altered operand schema or replay limit differing from the retained original generation
    context refuses; operand correspondence uses [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
    AC-16.
- id: FR-032-AC-4
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    The driver's actual native observation of the proved generated oracle is required
    and tied to this operand vector, limits and retained staged artifact bytes; checked
    completion requires driver authentication. A bare outcome echo, missing observation,
    another artifact's observation or fresh regeneration that removes the arithmetic
    mutation refuses before QSL evaluation. Playback bytes and Kani falsification
    alone cannot supply a generated outcome.
- id: FR-032-AC-5
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    The public consumer executes the same proved generated artifact and invokes QSL
    between the builder and converter. QSL compares its actual native observation
    with authoritative exact operator evaluation using the four-operator projection
    owned by [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-37,
    without adding refusal-cause or accounting-counter equality; changing the generated
    arithmetic while keeping original node/operator identity changes the measured
    parity result. `Undefined`, `Inco
- id: FR-032-AC-6
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    For every `Diverged`, `Agrees`, `GeneratedFault`, `RefusedInput`, exact `Incomplete`
    and other `Refused` outcome, the converter compares the report's full claim to
    the retained sent claim and actual sent wire against the original proving context
    before conversion; a changed package/function/source/dependency or original limit
    refuses even with an equal claim. At the immutable-report/retained-binding boundary,
    an absent report, another result/run, or changing only the obligation, node, occurrence,
    kind, operator
- id: FR-032-AC-7
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    QSL's divergence between generated and authoritative exact outcomes becomes a
    CG lowering fault and `Failed`; agreement despite Kani falsification becomes `Inconclusive(ScalarAgrees)`;
    native `Incomplete`/execution fault becomes `GeneratedFault` and `Failed` while
    exact `Incomplete` becomes `Incomplete(ResourceExhausted)`. `RefusedInput` preserves
    `invalid_runtime_input`; other typed QSL refusals follow the existing refusal
    map and non-fault refusals retain their code. This scalar route never returns
    `Reproduce
- id: FR-032-AC-8
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    The real QSL-emitted `x + 1` with input `Int[0,9]` and result `Int[0,10]` exercises
    routed generation, actual Kani arithmetic-mutation playback and same-proved-artifact
    native observation. A separate bounded-addition fixture with both operands and
    result in `[-1000,1000]` exercises an admitted operand pair whose exact result
    is outside the result range. The expected exact result is derived from the actual
    retained playback, and its correct refusal is never labelled a source violation.
    The fixture ranges and wit
- id: FR-032-AC-9
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: ~~Retired hash-preimage criterion.~~ Its authoritative member-selection,
    positional-order, distinct-draw, missing-provenance and changed-member checks
    are carried by FR-032-AC-15. The retired identity-digest oracle is not reused.
- id: FR-032-AC-10
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: ~~Retired shared-mint criterion.~~ IR-reader structural identity and actual
    QSL route conformance are carried by FR-032-AC-16. The retired parity encoder
    is not called.
- id: FR-032-AC-11
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: ~~Retired claimed/recomputed-digest criterion.~~ Exact identity and retained-body
    mismatch refusal before comparison is carried by FR-032-AC-17.
- id: FR-032-AC-12
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (typed scalar-family/metadata retention and
    actual CG/QSL route). When a structurally self-consistent claim invents an application
    occurrence or operand child, or misstates an inline literal's value/singleton
    range, the consumer observes the corresponding merged typed membership refusal
    before structural claim comparison; a self-consistent string cannot admit invented
    metadata.
- id: FR-032-AC-13
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: ~~Retired preimage-encoder failure criterion.~~ Missing or malformed authoritative
    identity/body refusal with no fabricated identity is carried by FR-032-AC-18.
- id: FR-032-AC-14
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (actual CG scalar builder/converter). Adding
    the scalar builder/converter preserves existing function/clause carry-only behavior
    and function/frame identity encoding; none is redirected through scalar parity
    or changed by its positional argument rule.
- id: FR-032-AC-15
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. The admitted IR reader supplies the original application
    structural identity and authentic expression occurrence. CG typed generation supplies
    and retains the scalar kind and one argument for each operand position in actual
    draw order, with typed graph-child or inline-literal provenance and the harness
    range. Two draws of one parameter remain two entries. Changing a package, path
    or producer ordinal changes the structural identity; changing an included body
    member while the identity stays equal is caught by complete retained-body comparison
    and refuses. A source-span or n
- id: FR-032-AC-16
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. An independent fixture obtains the same application
    `NodeKey`/`CheckedNodeId` from IR admission and CG retention, checks the exact
    structural identity and complete positional body, and the actual QSL operator
    route accepts the matching claim. No CG-local identity derivation, copied encoder,
    stand-in verifier or synthetic acceptance passes.
- id: FR-032-AC-17
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. With a valid original claim retained, altering only
    the claimed package, path or producer ordinal, or changing one otherwise membership-valid
    retained body member while reusing the old structural identity, produces a typed
    stale-dependency/revision-mismatch refusal before scalar comparison. A body-only
    edit cannot pass because the node identity is equal. No claimed or recomputed
    digest is required.
- id: FR-032-AC-18
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. If the admitted IR read lacks a valid application
    identity or authentic occurrence, or CG typed generation lacks a required retained
    body member, the CG builder returns typed setup refusal without replay or fabricated
    identity. A caller string or usable sentinel cannot replace the absent authority.
- id: FR-033-AC-1
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PARTIAL (IR-635 verified Eq graph-child constructor controls and Ne parameter/parameter
    and parameter/literal controls, including operator mismatch refusal; Ne literal/literal
    is AC-14 and inline operands are AC-16 and AC-17, all PLANNED under IR-814; falsified
    and generated-artifact routes planned, IR-806). A public consumer builds the equality-parity
    request directly from the composite node and original proving context. CG submits
    the original source/package and node selector to QSL and accepts only its admitted
    same-package/node result under QSpec FR-322. CG validates admitted node/occurren
- id: FR-033-AC-2
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. Canonical round trips preserve every listed leaf family,
    including beyond-i64 composite integer leaves as ExactInteger (while top-level
    Integer remains i64), IEEE NaN and signed-zero bits, normalized rational, retained
    decimal, delimiter-containing Unicode text, enum member, exact quantity/unit and
    reference identity, and every listed composite kind. Mixed nested values retain
    declaration/slot/member/presence/arity/order/multiplicity. No decoder claim depends
    on a currently supported Boolean/i64 harness alone.
- id: FR-033-AC-3
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: 'PLANNED/GATED. Noncanonical JCS/tag/scalar/escape spelling, unknown declaration
    or unit/reference identity, wrong enum/union member, missing/extra or wrongly
    present record slot, wrong tuple arity, excess cardinality, out-of-domain leaf,
    duplicate set/ordered-set element under authoritative equality or altered canonical
    element order yields a typed operand/decode refusal before exact evaluation when
    that stage is reached, after common claim checks and same-valid-claim Disagreed
    precedence. An admitted claim whose operand field x is declared Int[0, 9] but
    supplied as x: 12 with Disagreed yields'
- id: FR-033-AC-4
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. Public caller input exactly at the configured encoded-byte
    limit is admitted and one byte over refuses before parsing; occurrence/work exhaustion
    refuses with checked accounting. A 100,000-link admitted recursive value decodes,
    clones, compares, formats redacted Debug and drops on a 512 KiB native stack without
    a depth refusal. Its one-byte-too-small budget refuses truthfully; launcher capture
    limits alone cannot satisfy this check.
- id: FR-033-AC-5
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. The retained actual assertion playback reconstructs operand
    values by the FR-025 original-node/leaf bindings; another harness or persisted
    binding schema refuses claim binding. For the same admitted claim, swapped playback
    leaf order, missing/extra playback leaf or wrong primitive width refuses reconstruction
    only after Disagreed precedence; retained Disagreed remains Failed/CgDefect. Literal
    payloads remain their singleton value and absent/null/present states remain distinct.
    Removing or changing a leaf binding changes reconstruction or refuses, rather
    than silently decoding an
- id: FR-033-AC-6
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: ~~Retired content-identity claim criterion.~~ FR-033-AC-19 carries forward
    actual same-artifact native observation, original limits, mutation-preserving
    authentication and Disagreed precedence as an independent CG pre-invocation check.
    The removed `CompositeParityClaim.content_identity` is not replaced by another
    digest or claim member.
- id: FR-033-AC-7
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PARTIAL (IR-666 direct public F-7 converter controls; IR-806 original-artifact
    path planned). For an identity-valid claim reaching row F-7, actual QSL exact
    equality/count divergence from a falsified shadow maps to CG-defect `Failed`;
    an assertion-only mutation with otherwise agreeing equality/count maps to `Inconclusive(ScalarAgrees)`
    retaining the CompositeEquality claim/outcome. Neither maps to `Refuted`. Independently
    mutating equality result and occurrence-pair count is detected, including no early
    exit after an unequal pair.
- id: FR-033-AC-8
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. CG submits the original claim's operand declarations and
    literal singleton domains and consumes QSL-derived bound keys without a CG-authored
    declared-bound list. Omitting a bounded position or using an empty list with declared/unbounded/recursive
    keys leaves it uncovered; unknown/duplicate/kind-mismatched keys refuse. Exact
    declared coverage and a wider legal harness range cover; a tightened range/cardinality/depth
    does not silently cover under ADR-021 TX3. Enum Variants bounds cover only when
    every source-declared variant is included; a Variants bound naming an undeclared
    varia
- id: FR-033-AC-9
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: ~~Retired content-identity report-binding criterion.~~ FR-033-AC-20 carries
    forward full actual-sent QSL claim/report binding, every remaining claim and evidence
    member, refusal mapping and outcome precedence. FR-033-AC-19 separately carries
    the same-artifact check; the retired field is not reused.
- id: FR-033-AC-10
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. Each lawful generated family exercised by the completed
    route has a real verify and falsify control with retained backend evidence and
    mutation-killed same-artifact replay; all canonical families have the AC-2/3 decode
    checks. Families awaiting shadow generation or upstream admission remain explicitly
    unsupported/gated and are reported as gaps, rather than counted as passing end-to-end
    coverage. Open real Text-profile, nested set/bag performance and source-unbounded
    coverage limitations remain explicit; API presence or rational substitution supplies
    no generated-family acceptanc
- id: FR-033-AC-11
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: ~~Retired O-09 parity-preimage criterion.~~ FR-033-AC-18 carries forward
    positional graph-child arguments, repeated and literal operands, key ordering,
    membership refusal, complete retained-body comparison, separate proving-record
    metadata and independent artifact authentication. The old hash oracle is not reused.
- id: FR-033-AC-12
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: 'PARTIAL (IR-666 direct QSL F-1 precedence controls; IR-635 Eq-only verified
    constructor and QSL membership/unknown-bound refusals before supplied Disagreed;
    falsified and cross-outcome integration planned, IR-806). Both paths carry the
    same four-state Refinement. Claim/source/node/occurrence/bounds/structural-identity-or-body
    refusal precedes Disagreed; for a valid claim Disagreed wins over operand refusal
    (including an operand field x declared Int[0, 9] but supplied as x: 12), missing
    native observation, native fault, exact limit, refinement ceiling and agreeing
    replay. No early operand/nativ'
- id: FR-033-AC-13
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: 'PARTIAL (IR-666 direct public F-2 to F-6 controls; IR-806 must reach the
    same rows from a CG-built request over the original proving context). On a valid
    falsified claim without Disagreed, a native Incomplete or ExecutionFault wins
    over an invalid operand, too-small admission limit, too-small exact limit and
    CeilingReached: CG observes GeneratedFault/Failed retaining its NativeCause. With
    a Completed native, an operand outside its declared domain yields RefusedInput/ReplayRefused
    even with CeilingReached; a request limit reached during admission yields Incomplete/Admission
    with its counter, an'
- id: FR-033-AC-14
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-814; literal operands are admitted one-field `Integer` records,
    with bounded-field narrowing literals (IR-691) and collection-constructor literals
    gated). When the original application node is a `structural.ne` whose two operands
    are both composite literals and the requested operator is NotEqual, the verified
    request carries NotEqual and a retained checked claim body with two positional
    `graph_child` arguments, each the literal's own node with empty Bounds and no
    parameter position.
- id: FR-033-AC-15
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-814). When the original application node is a `structural.ne`
    comparing one parameter with itself and the requested operator is NotEqual, the
    verified request carries NotEqual and a retained checked claim body with two positional
    `graph_child` arguments with the same node identity, each retaining the drawn
    Node-keyed bounds naming that parameter and neither deduplicated.
- id: FR-033-AC-16
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-814; verified by Inspection of the single conversion that AC-17
    tests). If the owning operand projection refuses an inline integer literal operand
    (`InlineInteger`), then the builder returns that refusal through `CompositeBuildError::Operands`
    before any QSL invocation, with no report or terminal value and no built argument.
    The owning contract treats this branch as defensive because reader admission already
    refuses an integer operand of `structural.eq` and `structural.ne`, so no admitted
    fixture can reach it. Both inline refusals pass through the same `CompositeBuildError::Operand
- id: FR-033-AC-17
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-814). If the owning operand projection refuses an inline non-integer
    literal operand (`InlineNonInteger`), then the builder returns that refusal through
    `CompositeBuildError::Operands` before any QSL invocation, with no report or terminal
    value and no new CG error variant. No test of this path exists yet.
- id: FR-033-AC-18
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. CG obtains the original equality/inequality application
    structural identity from IR admission and retains the authentic occurrence, kind
    and one argument per operand position in original order. A self-comparison keeps
    two graph-child entries with each drawn Node-keyed bound; distinct parameters
    and composite literals keep their own graph-child entries, with empty Bounds for
    literals. Population-keyed metadata stays in the full proving record; an admitted
    QSL claim containing it refuses as HarnessUnknownKey before identity/body comparison.
    Bounds key ordering and duplicate
- id: FR-033-AC-19
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED (IR-806). Before falsified parity invocation, CG authenticates
    the retained proved generated artifact and original source/package/context, reconstructed
    operands and original execution limits against the driver's actual native observation;
    it preserves the owning same-valid-claim Disagreed precedence before treating
    a missing or failed observation as a refusal. Without a same-valid-claim Disagreed,
    a different artifact, changed retained bytes or context, altered operands or limits,
    missing observation, or fresh regeneration that removes a mutation refuses with
    a typed CG p
- id: FR-033-AC-20
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED for the current QSL FR-358 claim shape; earlier IR-666
    converter and IR-635 verified-constructor controls remain partial evidence only
    for unchanged members. Before projecting any QSL outcome, CG compares a genuine
    `report.claim()` against `CompositeIdentity::new` over exactly the actual sent
    obligation, all remaining `CompositeParityClaim` members (node, occurrence, operator,
    obligation kind, harness bounds and limits) and complete `CompositeEvidence` (falsified
    operands, shadow verdict and pair count, native observation and cause, refinement;
    or verified SUCCESS count an
- id: FR-019-AC-1
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Settlement dispatches one arm per variant of the closed backend kind, and
    every variant reaches an arm that settles rather than falling through.
- id: FR-019-AC-2
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Every requested item receives exactly one of `supported`, `requires-bound`,
    `unsupported` and `invalid-request`, settled from its `candidates` and extent
    classification, and `candidates` is read from the request rather than computed
    here.
- id: FR-019-AC-3
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: On the Kani arm, an unbounded extent against a `bounded`-only advertisement
    settles `requires-bound` when a finite bound is available, rather than `unsupported`;
    with no finite bound available it settles `unsupported`, warned, with `unsupported_projection`/`unbounded-extent`;
    and it never settles `supported`.
- id: FR-019-AC-4
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An item with more than one candidate and no named backend settles `invalid-request`
    with `invalid_capability`/`ambiguous-backend`, naming every candidate in candidate
    order, identically under every registration order.
- id: FR-019-AC-10
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Only an item settled `supported` routes.
- id: FR-019-AC-45
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'A manifest with a second descriptor of one backend identity causes a whole-envelope
    `EnvelopeRefusal::ManifestRepeatsBackendIdentity { identity: String }`, whether
    the descriptors are identical, differ in `ProviderOrigin` alone, or differ in
    any other CG descriptor field, and whether adjacent or non-adjacent. The call
    returns no item settlements. The variant has no string cause, manifest or digest
    payload; `negotiate_backend_provider` keeps its return signature.'
- id: FR-019-AC-46
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: With several repeated identities, the refusal names the bytewise-first
    repeated identity independent of manifest order; identities differing only in
    case are distinct. An invalid contract version or capability vocabulary is refused
    as `EnvelopeRefusal::InvalidCapability` before repeat detection or any kind read.
- id: FR-019-AC-47
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: QSL's registry snapshot holds each backend identity at most once (FR-290:258-265;
    FR-331:57), so no driver-built FR-331 manifest repeats an identity.
- id: FR-019-AC-48
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: RETIRED, replaced by FR-019-AC-54; its old path and expected/actual digest
    projection is obsolete and earns no implementation credit.
- id: FR-019-AC-49
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'An `Incomplete(incomplete)` outcome from the current Kani oracle-function
    preflight read produces `Err(EnvelopeRefusal::ContractPackageReadIncomplete {
    incomplete: ContractPackageIncompleteProjection })` for the whole request, with
    the reader''s `limit_kind`, `limit`, `consumed` and `path` copied into the named
    CG projection. No item settlement or artifact is returned.'
- id: FR-019-AC-50
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An `Admitted` Kani function-application preflight read whose exact generator
    produces a claim for an otherwise supported item retains the existing `supported`
    settlement.
- id: FR-019-AC-51
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Among several request items with non-`Admitted` preflight outcomes, the
    first failing item in manifest/item request order determines the whole-request
    `ContractPackageReadRefused` or `ContractPackageReadIncomplete` variant and projection,
    irrespective of outcome kind. Earlier successful items contribute no returned
    settlement or artifact.
- id: FR-019-AC-52
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An `Admitted` Kani function-application read whose exact generator emits
    no claim, and a structural preflight rejection before any read, each retain item-level
    `KaniFormRefused { OperationNotRendered }` without a reader envelope refusal.
- id: FR-019-AC-53
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Given an admitted package with bound driver items at driver indexes 0 and
    2 around an excluded accounting-only position at index 1 whose accounting is `lowered`
    with a Kani-supported form, an FR-331 envelope with exactly two items settles
    the bound items without an `EnvelopeItemCount` refusal. Both `ContractItemSettlement.request_index`
    and its nested `ItemSettlement.request_index` equal 0 and 2 respectively, not
    envelope indexes 0 and 1. The excluded position produces no settlement or claim,
    and its own classifier-call and Kani-preflight-read counts are both zero. An envelope
    with fewer or mo
- id: FR-019-AC-54
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'A `Refused(refusal)` outcome from the Kani oracle-function preflight reader
    produces `Err(EnvelopeRefusal::ContractPackageReadRefused { refusal: ContractPackageRefusalProjection
    })` for the whole request, copying the reader''s `code`, nonempty ordered typed
    `loci` including paths and source regions, `cause`, `locus`, `contract_version`,
    `document_pointer`, `extension_detail`, `protocol_detail` and `terminal_duplicate`
    without loss or reinterpretation. The obsolete `path`, `expected` and `actual`
    projection members are absent. No item settlement or artifact is returned. Distinct
    extension, proto'
- id: FR-019-AC-7
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An envelope whose `capability_vocabulary` is not exactly `quire.capability-kind/v1`,
    or is absent, is refused as `invalid_capability`/`unsupported-version` with none
    of its kinds read.
- id: FR-019-AC-8
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An item with an absent extent classification settles `invalid-request`
    with `invalid_capability`/`absent-extent`, and an absent or unknown kind settles
    before the candidate table is consulted.
- id: FR-019-AC-9
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'A backend kind added without a negotiation arm does not compile: the dispatch
    is an exhaustive `match` over the closed kind with no catch-all arm.'
- id: FR-019-AC-11
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). `BackendKind` has `Process(id)` beside `Kani`; `ALL`
    lists finite built-in kinds only, while CG enumerates process candidates from
    descriptors. CG's descriptor-to-kind conversion returns `Process(id)` exactly
    when a descriptor has process origin, including identity text `kani`. A named,
    registered process backend reaches that arm and receives one disposition; no process
    item disappears for being absent from `ALL`.
- id: FR-019-AC-12
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629; non-empty bounded rows depend on QSL-654's producer and
    admit-side enforcement). A bounded process item with an uncovered `extent.bounds[].kind`
    settles `unsupported`, warned, with `unsupported_projection`/`unsupported-requested-capability`
    naming the offending domain kind and candidate (QSpec FR-290-AC-13). An item that
    passes the domain check, including one with empty `bounds`, settles `supported`
    exactly when its descriptor advertises `bounded` for its kind; otherwise it settles
    `unsupported`, warned, with `unsupported_projection`/`unsupported-requested-capability`.
- id: FR-019-AC-16
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). An unbounded process item settles `supported` when its
    descriptor advertises `unbounded` for its kind, regardless of whether any `extent.domains[].kind`
    belongs to manifest `domains`.
- id: FR-019-AC-17
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). On a `bounded`-only process descriptor, an unbounded
    item settles `requires-bound` exactly when `finite_bound_available` is true and
    every boundable `extent.domains[].kind` is in manifest `domains` (QSpec FR-290-AC-13).
- id: FR-019-AC-18
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). Given the same process advertisements and item extent,
    changing only backend identity, manifest position, manifest run-limit defaults
    or ambient state leaves the disposition and cause unchanged apart from the backend
    named in the output.
- id: FR-019-AC-19
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629; bounded row depends on QSL-654's producer). The process
    arm reads each explicit `extent.bounds[].kind` and never derives a domain kind
    from `DomainKey`; two otherwise equal bounded items with the same `DomainKey`
    and different explicit kinds can settle differently under one manifest `domains`
    set.
- id: FR-019-AC-20
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629; driver projection remains paired work). CG's process descriptor
    retains the FR-331 manifest's advertised (kind, mode) pairs, `domains` and optional
    `bounds` beside the IR-633 `id` and `origin` projection.
- id: FR-019-AC-42
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Two otherwise equal CG Process descriptors with absent and present-empty
    manifest `bounds` remain distinct typed values. Changing only between those forms
    leaves process settlement's disposition and request-validation precedence unchanged;
    run-limit defaults are not proof bounds.
- id: FR-019-AC-21
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). On a `bounded`-only process descriptor, an unbounded
    item with any unadvertised boundable `extent.domains[].kind` settles `unsupported`,
    warned, with `unsupported_projection`/`unsupported-requested-capability` naming
    the offending domain kind and candidate, regardless of `finite_bound_available`;
    non-boundable kinds are not compared (QSpec FR-290-AC-13).
- id: FR-019-AC-22
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). On a `bounded`-only process descriptor, an unbounded
    item with every boundable `extent.domains[].kind` advertised and the item's `finite_bound_available=false`
    settles `unsupported`, warned, with `unsupported_projection`/`unbounded-extent`.
    CG reads that flag from the item and does not derive it from the domain kinds
    (QSpec FR-290-AC-13).
- id: FR-019-AC-23
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629; bounded row depends on QSL-654). Changing only a bounded
    item's proof-bound numeric maximum leaves its process-provider `supported` or
    `unsupported` disposition and cause unchanged.
- id: FR-019-AC-13
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'Settling a process-provider item reaches no plugin: a descriptor whose
    identity names a non-existent executable settles identically to one with an ordinary
    identity, apart from the backend each names, and starts nothing, and a descriptor
    whose identity names an executable that records its own start leaves no record.
    A mutant arm that starts or resolves the identity as a process either changes
    the first disposition or leaves the record, and fails this.'
- id: FR-019-AC-14
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: The process-provider arm's return type is `Disposition`, which has no terminal-value,
    verification-result or artifact member, so settling returns none of them. A change
    that returns one does not compile against that type.
- id: FR-019-AC-15
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-633). CG's `BackendDescriptor` has a typed `ProviderOrigin`
    projecting exactly QSL layer R's `Linked` and `Process` meanings (QSL FR-288,
    ADR-029 PV-1); layer R owns that vocabulary. The driver projects each descriptor
    from `Registry::descriptors()` once into CG's descriptor, copying `id` and advertised
    pairs unchanged and mapping `origin()` exhaustively, `Linked` to `Linked` and
    `Process` to `Process`, with no wildcard or origin inference from identity, manifest
    or provider bytes or a side map. Two registry descriptors identical except for
    origin produce CG descriptors identical e
- id: FR-019-AC-24
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'PLANNED (IR-338). Given QSL FR-335''s `value-validity` claim rooted at
    an unbounded `s: Set<Int[0,9999]>`, an empty candidate registry settles `unsupported`
    with a warning naming `value-validity` and emits no artifact or Kani outcome;
    one linked Kani candidate advertising (`value-validity`, `bounded`) with `finite_bound_available=true`
    settles the unbounded item `requires-bound` without substituting a bound or emitting
    an artifact; a separately requested bounded `ProofBound::Cardinality{maximum:
    8}` item with that same sole candidate settles `supported` under its own request
    index, while the un'
- id: FR-019-AC-25
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'OPEN for surviving typed V2 forms; deprecated `V1BoundClause` roles retire
    under the 2026-10-27 owner ruling on IR-348. Original criterion retained for traceability:
    The normative Kani form set is the complete V1 proof set defined by IR-269 and
    FR-015: scalar families Boolean, integer, rational, decimal, IEEE float, text,
    enum and quantity; record and tuple composite equality; function application;
    precondition, postcondition and invariant `BoundClause` roles; and bounded collection
    and graph forms. V1 `ClauseKind` has no frame variant. State before/after and
    frame obligations are typed V2 `St'
- id: FR-019-AC-26
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'OPEN for surviving forms; a `V1BoundClause` candidate is no longer supported
    after IR-348. Original criterion retained for traceability: For any form in FR-019-AC-25
    with one validated Kani candidate, a bounded extent settles `supported` exactly
    when the form''s FR-015 lowering succeeds and the candidate advertises the bounded
    mode; negotiation creates no artifact as part of settlement.'
- id: FR-019-AC-27
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: With one Kani candidate, a typed form outside the normative set in FR-019-AC-25
    settles `unsupported`, warned, with `unsupported_projection`/`unsupported-requested-capability`
    naming its typed tag/form and Kani; no artifact is emitted and no generator is
    invoked. A malformed in-set form settles with the applicable typed FR-015 validation
    refusal.
- id: FR-019-AC-33
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'OPEN for surviving Planned forms; V1-only subpaths retire under the 2026-10-27
    owner ruling on IR-348. Original criterion retained for traceability: For a valid
    bounded typed form inside FR-019-AC-25 that matches one specific Planned subpath
    in TC-030''s status register and does not match an Implemented subpath, negotiation
    returns `unsupported` with that subpath''s single exact typed refusal cause, emits
    no artifact, and never settles `supported` before its renderer is implemented;
    this refusal remains distinct from the out-of-V1 refusal in AC-27.'
- id: FR-019-AC-28
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: A form refusal under FR-019-AC-27 is distinct from an empty candidate set,
    which retains the missing-capability cause, and from multiple candidates without
    a named backend, which retains `invalid_capability`/`ambiguous-backend` naming
    each candidate in candidate order.
- id: FR-019-AC-29
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: After request and candidate/manifest validation select one Kani candidate,
    Kani checks the typed IR form before its extent and advertised-mode rules; if
    both the form and mode/extent are unsupported, the form refusal of FR-019-AC-27
    settles the item.
- id: FR-019-AC-30
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: For the same typed IR tag/form, changing only its display or debug text
    leaves Kani's disposition, cause and named form identity unchanged.
- id: FR-019-AC-31
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: For a Kani-supported unbounded form against a `bounded`-only advertisement,
    `finite_bound_available=true` settles `requires-bound`, with no substituted bound
    and no artifact.
- id: FR-019-AC-32
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: When a caller supplies a finite bound in a new bounded request for a Kani-supported
    form, negotiation settles `supported` and preserves the exact supplied proof-bound
    value, domain kind and domain identity.
- id: FR-019-AC-34
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). With one EN-1 candidate, an advertised admitted TP-1
    through TP-4, TP-5, SG-1 through SG-3, or HP-1 model/HP-2/HP-3/HP-6 form settles
    through its explicit-state arm with the recorded form and model-or-protocol subject
    intact; an unadvertised form or HP-1 code claim does not settle `supported` through
    EN-1.
- id: FR-019-AC-35
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). With one SMT temporal candidate advertising the item's
    kind and form, TP-1 through TP-4 route only to an EN-2 or EN-3 method ADR-018
    assigns to that form and subject; TP-4 does not route to EN-3, and a form absent
    from the candidate's advertised form set settles `unsupported` with no engine
    request.
- id: FR-019-AC-36
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). A TP-5 whole-refinement `temporal-satisfaction` item
    retains its safety and optional liveness halves on the concrete subject through
    EN-1, and the same whole item reaches an advertising SMT arm for RE-3 refutation;
    a separately requested RE-2 `operation-contract` item reaches the SMT arm for
    per-step safety checking, but its disposition cannot substitute for the whole-refinement
    item. An arm lacking the required TP-5 form settles the item `unsupported`.
- id: FR-019-AC-37
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). HP-1 model, HP-2, HP-3 and HP-6 reach an advertising
    EN-1 arm; only HP-2 reaches an advertising SMT arm under its applicable EN-2 or
    EN-3 method. HP-4 and an unadvertised HP form settle unsupported before an engine,
    HP-5 stays on its possible-property path, and an HP-1 code claim is not settled
    as an EN-1 model claim.
- id: FR-019-AC-38
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). An advertised SG-1, SG-2 or SG-3 item reaches EN-1 with
    its recorded form; only advertised SG-1 reaches an SMT arm. SG-2 or SG-3 against
    an SMT-only candidate settles unsupported and reaches no SMT engine.
- id: FR-019-AC-39
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). A selected reduction reaches only a candidate advertising
    that exact reduction, with the selection preserved in the routed request. Symmetry
    declarations and POR enter the obligation identity, so their reduced and unreduced
    requests for one claim have distinct obligation identities; a state constraint
    changes request identity but leaves obligation identity unchanged. An unadvertised
    selection or POR on a subject with no footprints settles `unsupported` without
    an artifact, and no arm drops or adds a reduction.
- id: FR-019-AC-40
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). For an item over a weak `parallel`, a candidate advertising
    the subject's resolved `tso` or `ra` model receives that exact model; a candidate
    lacking it settles `unsupported` with no artifact, and no arm weakens the resolved
    model. An `sc` subject is a separate strong-model control and does not take the
    weak-memory branch.
- id: FR-019-AC-41
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). With otherwise equal form and kind advertisement, model
    and protocol subject classes are read separately; an arm that discharges only
    one class settles the other `unsupported`, while a supported protocol item retains
    its protocol transition-system subject identity through routing.
- id: FR-019-AC-43
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Two driver items selecting distinct `expression` occurrences of one checked
    `integer.add` node and different direct `numeric.narrow` occurrences in one admitted
    ContractPackage settle independently under the same Kani descriptor. Each preflight
    result is classified from its own selected result context; changing the selected
    narrow changes that item's typed result identity or refusal without changing the
    other. A node-only classifier or package-wide choice of either bound fails this
    control.
- id: FR-019-AC-44
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: For a V2 scalar claim, an absent `OwnResult`, an application occurrence
    absent from the admitted source, a conversion occurrence from another source or
    unrelated to the application, a missing conversion context root, or a ContractPackage
    source identity different from the checked source settles `invalid-request` with
    a typed form/context cause and no artifact or harness. Request and candidate/manifest
    defects retain their earlier precedence; for a valid candidate this selected-form
    refusal precedes extent and advertised-mode refusal. Neither a source message
    nor a caller maximum repairs the se
- id: FR-022-AC-1
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Generation dispatches one arm per variant of the closed `BackendKind` through
    an exhaustive `match` with no catch-all, and `GenerationContexts::has` handles
    each variant; adding a variant without either arm does not compile. `Process`
    requires no context.
- id: FR-022-AC-2
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: For a set of routed Kani items, each record and harness in the output equals
    what `negotiate_kani_obligations` returns for the same items in ascending request-index
    order with the same context. The one difference is that every index inside a record
    is the driver's request index, and each harness is paired with the record whose
    `harness_symbol` names it.
- id: FR-022-AC-3
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: The entry point accepts no backend registry, candidate set, extent or capability
    kind, and constructs no FR-019 `Disposition`.
- id: FR-022-AC-4
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629 internal-invariant test migration). A crate-internal malformed
    routed Kani item whose private backend identity has no CG built-in kind, or converts
    to a kind other than Kani, refuses the whole call as `BackendKindDisagrees`, naming
    the request index, backend, routed kind and converted kind or its absence, with
    no artifact.
- id: FR-022-AC-5
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Two routed items with one request index refuse as `DuplicateRequestIndex`
    naming it. A routed Kani item without Kani context refuses as `MissingKindContext`
    naming Kani. Each returns no artifact.
- id: FR-022-AC-6
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: A Kani group-level refusal (for example an unwind outside `1..=1024`, or
    an unparsable subject path) is returned as `Kani` carrying the unchanged `KaniObligationError`,
    with no artifact.
- id: FR-022-AC-7
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: A Kani group containing an `invalid_request` item is listed in `rejected`,
    and every Kani item's record is returned with no harness. A `DuplicateItem` reported
    by FR-015 has `first_index` rewritten to the driver's request index of the first
    occurrence. On the FR-330 package path, an identical selected scalar occurrence
    pair also retains FR-039's earlier typed `DuplicateRequest` claim refusal at both
    positions.
- id: FR-022-AC-8
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Every routed item appears exactly once in `items`, in ascending request
    index, under its routed backend and the `KindOutput` variant of its routed kind.
    An item the arm refuses at lowering keeps its typed FR-015 refusal and no harness.
    An empty routed set returns an empty result.
- id: FR-022-AC-9
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Regeneration from equal inputs is byte-identical, and a permutation of
    the routed-item slice yields an identical result. The same node routed at a different
    request index or under another candidate of the same backend yields a byte-identical
    harness when the admitted FR-330 package and selected occurrence are the same.
- id: FR-022-AC-10
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: The Kani arm derives each item's operation and domain from the package
    alone; the caller supplies no claim map, and `x + 1` over a parameter `Int[0,
    9]` routes to a supported `quire.op.integer.add` harness with arguments `[0, 9]`
    and `[1, 1]` (the literal at its own value, FR-015-AC-16).
- id: FR-022-AC-11
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'An item with no derivable descriptor keeps its typed refusal and no harness,
    and its siblings'' records and harnesses are unaffected. On the FR-330 package
    path, an absent root retains its exact admitted `InvalidInput` accounting and
    `PackageRefused` output before Kani. On a direct FR-015 request, a node absent
    from the graph is `invalid_request` and rejects the group. A node that does not
    lower or has a refused bound keeps the FR-015 disposition it has when its claim
    is generated by FR-014 (`requires_bound`, `blocked_on_upstream`, `unsatisfiable_bound`).
    A repeated selected pair on the FR-330 '
- id: FR-022-AC-12
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: '`RoutedGeneration.claim_map` is `Some` after a Kani group and contains
    one claim per selected node/occurrence admitted past FR-039 accounting, retaining
    any earlier typed `DuplicateRequest` refusal; an underivable node has one refused
    claim of provenance `underived`, ordered by node id and then selected occurrence
    key.'
- id: FR-022-AC-13
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'The Kani arm routes an integer node over bounded parameters to a supported
    harness whose arguments are one range per operand and whose result assertion uses
    the result bound: `a + b` over `[0, 9]` and `[10, 20]` with result `[0, 29]` has
    arguments `[0, 9]` and `[10, 20]`; `-e` over `[1, 9]` with result `[-9, -1]` and
    `e * f` over `[1, 9]` and `[100, 200]` with result `[100, 1800]` are supported;
    a node with a plain-typed `reference` operand beside bounded typing (FR-014-AC-25)
    settles `requires_bound` and has no harness.'
- id: FR-022-AC-14
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: '`RoutedGeneration.oracle_artifacts` is `Some` after a Kani group and `None`
    when nothing is routed. Its `claim-map.json` equals the complete package-derived
    claim map, including typed refused items; a group with no generated claim still
    returns its crate. For `x + 1` over a parameter `Int[0, 9]`, each `Generated`
    claim''s oracle symbol is defined in the returned `src/lib.rs` and appears in
    the supported harness''s Rust source.'
- id: FR-022-AC-15
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'The Kani arm routes the packages QSL emits for `x + 1` over `x: Int[0,
    9]` into `Int[0, 10]`, `x + y` over `Int[0, 9]` and `Int[10, 20]` into `Int[10,
    29]`, and `-z` over `Int[0, 9]` into `Int[-9, 0]` (a literal as a reference to
    its own `value` node, the declared bound on a narrowing `conversion` consuming
    the plain-typed arithmetic node) to supported harnesses with arguments `[0, 9]`
    and `[1, 1]`; `[0, 9]` and `[10, 20]`; and `[0, 9]`, each asserting the result
    against the conversion''s bound (`[0, 10]`, `[10, 29]`, `[-9, 0]`). In that shape
    a plain-Integer parameter beside a bounded one, and'
- id: FR-022-AC-16
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Two Kani harnesses with one `harness_symbol` refuse the call as `DuplicateHarness`
    naming the second harness's `module::harness` path, and no harness is overwritten
    or dropped.
- id: FR-022-AC-17
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). A routed `Process(id)` item appears exactly once with
    its own identity and empty `KindOutput::Process`, without a generation context
    or membership in `BackendKind::ALL`.
- id: FR-022-AC-18
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). A crate-internal routed `Process(id)` item whose private
    backend and kind disagree refuses the whole call as `BackendKindDisagrees` with
    `converted=Some(Process(item.backend.identity))` and no generated output.
- id: FR-022-AC-19
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). Permuting a routed-item slice containing Kani and multiple
    process identities leaves its request-index-ordered output unchanged.
- id: FR-022-AC-20
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). `RoutedGenerationItem.backend` and `.kind` are private
    while `request_index` and `node_id` remain public; external callers cannot create
    an item with a hand-built `Process(id)` or reassign its backend after `from_descriptor`
    construction.
- id: FR-022-AC-21
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). `from_descriptor` constructs Kani for a linked `kani`
    descriptor and `Process(id)` for a process descriptor with the same identity text.
    An unknown linked identity never produces Process.
- id: FR-022-AC-22
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). `from_descriptor` rejects a backend identity different
    from the descriptor identity as `DescriptorBackendMismatch`, naming both; it rejects
    an unknown linked identity as `UnknownLinkedBackend`, naming the backend. Neither
    refusal produces a routed item.
- id: FR-022-AC-23
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: After FR-019 authenticates two `integer.add` driver items on one admitted
    ContractPackage with distinct selected application/direct-`numeric.narrow` pairs,
    routed generation retains the pair for each item in its claim and harness identity
    and asserts against its own selected result bound. The two generated harnesses
    have distinct identities and expected ranges; a fresh settlement of a changed
    selection changes or refuses only that item, without borrowing the other bound.
- id: FR-022-AC-24
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: A routed scalar item's retained authenticated selection and its driver
    lowering position must name the same application/conversion pair in the same admitted
    ContractPackage. A changed pair, missing conversion context root or occurrence
    absent from that package refuses the whole call with a typed selection error before
    any harness. No package-wide bound search, source-text reparse or copied QSL context
    supplies a replacement.
- id: FR-022-AC-25
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: The public CG API exposes the concrete `ContractScalarError` carried by
    `RoutedGenerationError::ContractScalar`, so an external driver can match `ContractScalarError::Oracle(OracleGenerationError::SourceTooLarge
    { bytes, limit, setting })` and distinguish the scalar source ceiling from request-accounting
    and other oracle failures without parsing text or naming a private module.
- id: FR-022-AC-26
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Given the checked QSL RR-16 package with two `x + 1` occurrences of one
    application node, directly narrowed to `Int[0, 10]` and `Int[0, 20]`, the driver
    supplies two distinct request indexes and occurrence keys. CG matches each to
    its own requirement record and selected narrow; both supported scalar outputs
    use the shared application operation and their respective result bounds, and their
    obligation and harness identities differ by application occurrence. Reversing
    routed-item order leaves these pairings unchanged.
- id: FR-022-AC-27
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'For `if p then x + 1 else x + 1`, each occurrence''s generated scalar
    claim is conditional on its own QSL path guard: the same guard occurrence with
    `holds=true` for the then item and `holds=false` for the else item. CG preserves
    the full outermost-first guard sequence and uses it when forming the proof obligation;
    a result under `p=false` cannot prove the then claim and one under `p=true` cannot
    prove the else claim. An empty path condition remains unguarded.'
- id: FR-022-AC-28
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: If a routed scalar item's occurrence has no entry in the supplied requirement
    records, its key's node disagrees with the authenticated application selection,
    or the keyed record disagrees with the item's result bound, direct conversion
    occurrence or ordered path condition, CG returns a typed refusal identifying the
    request index and occurrence before emitting an oracle, harness or obligation
    identity for that request; it does not use another record with the same node or
    reconstruct missing values from the IR graph or source text.
- id: FR-022-AC-29
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'Given an admitted package whose complete lowering sequence has bound driver
    items at indexes 0 and 2 and an excluded accounting-only position at index 1,
    a routed item with `request_index` 1 and the excluded position''s exact `node_id`
    refuses the whole call as `ExcludedDriverIndex { request_index: 1, node_id }`
    before any generation arm, oracle or harness. The same routed backend and descriptor
    at indexes 0 and 2 remain valid controls, and CG never substitutes an envelope
    position or the next bound item''s selection for index 1.'
- id: FR-022-AC-30
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: For a plain reader-admitted V1 package, a routed item requiring an operation
    mode, law-definition or member-target view whose accessor returns `NotAdmitted`
    makes `generate_routed` refuse the whole call as `UnauthenticatedOperationView
    { request_index, node_id }`, identifying the lowest `request_index` among such
    routed items regardless of routed-item slice permutation, with no oracle, settlement
    or harness output. A view-independent integer route over the plain read retains
    its existing result. The same view-dependent route over the matching source-authenticated
    V1 package retains its ordinar
- id: FR-026-AC-1
  path: spec/routed/functional/FR-026-backend-adapter-contract.md
  role: examined
  excerpt: The Kani adapter implements the adapter trait, and a function generic over
    that trait reaches the Kani adapter's generation arm, execution, transcript parser
    and witness renderer through it.
- id: FR-026-AC-4
  path: spec/routed/functional/FR-026-backend-adapter-contract.md
  role: examined
  excerpt: The execution evidence type is an associated type of the adapter trait,
    and the Kani adapter's is `KaniExecutionEvidence`.
- id: FR-026-AC-5
  path: spec/routed/functional/FR-026-backend-adapter-contract.md
  role: examined
  excerpt: PLANNED (IR-629). `Process(id)` has no CG adapter trait implementation
    or CG execution path; an inspection of all adapter implementations and execution
    dispatch arms finds no process arm.
- id: FR-045-AC-1
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: 'For an admitted HP-2 safety clause and `k = 1`, CG requests QSL''s base
    and step queries using FR-315''s clause, ordered subjects, instance and Meter
    shape, submits each canonical byte sequence unchanged to the selected solver,
    and retains the product, item, instance, depth, kind and exact bytes in each query
    record. Matching `Unsat`/`Unsat` records are eligible for QSL-owned `Inductive
    { depth: 1 }`; QSL FR-182-AC-7, rather than CG, settles that outcome as `proved`,
    `decisive-witness`, `Uncertified`.'
- id: FR-045-AC-2
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: A base `Sat` response remains only a candidate for QSL reconstruction and
    replay; base `Unsat` with step `Sat` is not induction; solver `Unknown`, QSL encoding
    refusal, resource stop, missing or malformed response, process failure and a response
    paired with another product, item, instance, depth, kind or query bytes remain
    distinct typed non-proof evidence. A resource stop is handed to FR-182 V-7 as
    incomplete. Every branch retains its available identities and depth, and no CG
    branch claims terminal refutation or certification.
- id: FR-045-AC-3
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: 'CG invokes the selected solver executable directly, one process per QSL
    query, with exact query bytes on stdin and finite per-query input-byte, wall-clock,
    memory and captured-output limits; it retains the executable, arguments, limits,
    completion, status, captured output and any emitted model/proof bytes in a CG-owned
    run record. An input-byte excess refuses before spawn, a missing executable gives
    `MissingTool`, and a reached limit gives `Stopped` with that limit; none becomes
    `Unsat` or a QSL outcome. Separate attempt and query identities prevent a stale
    or swapped response from satisfying '
- id: FR-045-AC-4
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: 'For a refinement with seven step rows, CG retains one `Base` and exactly
    seven typed `Step { row }` records numbered 0 through 6 from the recompiled `steps`
    array in authored declaration order (QSpec FR-375-AC-22), even when run records
    arrive out of order. Permuting the authored source rows correspondingly permutes
    the recompiled step rows: each named row''s `Step { row }` label and exact FR-315
    query association follow its new position, and a record bound to the wrong row
    refuses handoff. Each record binds the refinement item, subjects, distinct invocation
    attempt, role and row, and exact FR-'
- id: FR-045-AC-5
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: 'Given seven completed `Unsat` step records and one completed `Unsat` base
    record with present Alethe proofs, CG emits one deterministic handoff sequence
    in `Base`, `Step { row: 0 }` through `Step { row: 6 }` order for either input
    permutation. Every entry has separate length-delimited query and proof byte fields
    equal to the original FR-315 query and emitted Alethe proof, including a present
    empty proof. Repeating serialization of the same typed fields gives identical
    bytes; changing a proof byte changes only that entry''s proof field. The QSL adapter
    can construct `RefinementSmtCertificate` wi'
- id: FR-045-AC-6
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: Starting from AC-5's collection, removing `Base`, removing row 3, duplicating
    `Base` or row 3, or adding row 7 yields a typed shape refusal and no certificate
    handoff. Replacing row 3 with a record from another item, subject or attempt,
    or swapping row 3's record with row 4's while retaining the old row labels, yields
    a typed association refusal and no handoff.
- id: FR-045-AC-7
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: Starting from AC-5's collection, making row 3's query or proof field absent
    yields a typed missing-field refusal and no handoff; making the proof present
    but empty preserves the empty bytes in the handoff. An FR-315 refusal, a `Sat`
    or `Unknown` answer, a missing or malformed response, a failed solver invocation
    or a reached limit remains typed non-proof evidence and produces no proof-bearing
    handoff.
- id: FR-002-AC-1
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Rejected inputs remain distinct from passing cases in generated reports.
- id: FR-002-AC-2
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Boundary strategies exercise immediately inside and outside supported constraints.
- id: FR-002-AC-3
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Supported correlated constraints do not rely on improbable filtering.
- id: FR-002-AC-4
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Shrinking preserves generated constraints or records a residual rejection.
- id: FR-002-AC-5
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: The generated campaign runner owns execution, explicit-discard accounting,
    and conclusion, returns a typed terminal result, and rejects campaigns below either
    configured coverage floor or above the explicit-discard ceiling; exhausted searches
    retain their reason and any missed floor without becoming a completed floor failure.
- id: FR-002-AC-6
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Integer and generated Boolean harness campaign cases bind exact values
    to executable accepted/rejected expectations; enum cases execute an all-admitted
    membership expectation.
- id: FR-002-AC-7
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). For one precondition and one postcondition selected from
    an admitted `CheckedPackageV2`, the generated harness embeds each selected clause's
    FR-014 V2 clause oracle.
- id: FR-002-AC-8
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). In the V2 harness, an instrumented snapshot, precondition
    oracle, subject and postcondition oracle record that the snapshot precedes precondition
    evaluation, which precedes subject invocation, which precedes postcondition evaluation,
    for every case.
- id: FR-002-AC-9
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). Source inspection shows the V2 harness generator opens
    a `CheckedPackageV2` body only through the `core/ir/` layer of AD-004, and names
    no `BoundPackage`, `ClauseRef`, `quire_contract_model::TypedExpression` or `oracle::boolean_v1`
    item, each of which today's V1 harness names or uses.
- id: FR-002-AC-10
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A selected clause identity that is unknown, of the wrong
    clause kind, or the same for both selections yields a typed, located diagnostic
    naming the selection and no artifact.
- id: FR-002-AC-11
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A selected clause body outside the FR-014-AC-38 shape
    yields a structured unsupported diagnostic and no artifact, and the generator
    emits no filtering strategy for it.
- id: FR-002-AC-12
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). Repeated V2 harness generation for the same admitted
    package, selected checked clause ids, authentic `claim` occurrences, and campaign
    policy is byte-identical. The repeated selections include the same FR-011 identity
    inputs; the generated header, runtime identities, source map, symbols, and remaining
    bytes all agree across repetitions.
- id: FR-002-AC-13
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which both oracles complete true
    runs the subject once and is a pass.
- id: FR-002-AC-14
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes
    false is a rejected precondition and the subject is not invoked.
- id: FR-002-AC-15
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes
    true and the postcondition oracle completes false is a failed postcondition.
- id: FR-002-AC-16
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: 'PLANNED (IR-364). For each selected precondition and postcondition oracle,
    a seeded `Ok(Outcome::Undefined)`, `Ok(Outcome::Refused)`, `Ok(Outcome::Incomplete)`,
    `Err(ClauseStop::Oracle(OracleStop))`, and `Err(ClauseStop::InvalidConstant)`
    returns CG-owned `V2HarnessCaseResult::NonCompleted` retaining the exact typed
    stop cause, selected claim identity, and selected accepted typed `ExecutionPoint`.
    The generated campaign returns `V2HarnessCampaignResult::NonCompleted` carrying
    that case and a summary whose `attempted` includes this stopped case once and
    whose `non_completed` is one. No runtime '
- id: FR-002-AC-17
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns
    an outcome that does not complete does not invoke the subject.
- id: FR-002-AC-18
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns
    an outcome that does not complete does not evaluate the postcondition oracle.
- id: FR-002-AC-19
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). Each V2 selection preserves the accepted QSL `ClauseBinding.execution_point`
    for its exact admitted package, checked clause id, and authentic `claim` occurrence
    through the injective runtime rendering; precondition and postcondition points
    may differ. Two requests selecting different authentic claims on the same precondition
    node each retain that node's accepted `Pre` operation point while selecting the
    correct occurrence. A rejection carries the precondition point; a failed or passed
    verdict after completed postcondition evaluation carries the postcondition point;
    a non-comp
- id: FR-002-AC-20
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). After FR-002-AC-10's clause id, kind, and distinctness
    checks, a wrong-role, wrong-ordinal, or other-node `claim` occurrence for either
    selected V2 clause yields `HarnessErrorCode::MissingClaimOccurrence` and no artifact
    even if a sibling occurrence has a point; an authentic selected tuple with no
    accepted point yields `HarnessErrorCode::MissingExecutionPoint` and no artifact
    even if another authentic occurrence of that node has a point; a render mutant
    that changes only that selection's generated harness or verdict point yields `HarnessErrorCode::ExecutionPointMismatch`
    befo
- id: FR-002-AC-21
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: RETIRED, replaced by FR-002-AC-25; the former digest-shaped identity rendering
    is obsolete and earns no implementation credit.
- id: FR-002-AC-22
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). For either selection, an out-of-driver missing or non-claim
    occurrence returns `HarnessErrorCode::MissingClaimOccurrence`, a referenced model
    element absent from the admitted V2 relation returns `HarnessErrorCode::UnboundModelElement`
    naming it, and a process FR-331 owner differing from the admitted package returns
    `HarnessErrorCode::OwnerIdentityMismatch`. Each produces no artifact. A linked
    request uses one driver-exported/E4-emitted/E5-admitted package and a selected
    claim checked against that package; a foreign claim absent from it returns `HarnessErrorCode::MissingClaimO
- id: FR-002-AC-23
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: RETIRED, replaced by FR-002-AC-26; the former digest-shaped text control
    is obsolete and earns no implementation credit.
- id: FR-002-AC-24
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-364). With a supplied runtime report whose snapshot is `at_limit`
    before a seeded non-completed V2 oracle case, `V2HarnessCampaignResult::NonCompleted`
    retains that case and a summary with `non_completed == 1` and exact `attempted
    == None`; it does not infer an exact prior count from the saturated report. The
    report's existing counters and `at_limit` signal remain unchanged, and no runtime
    verdict is constructed or recorded for the stopped case.
- id: FR-002-AC-25
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815; replaces AC-21). The V2 harness header, `RequirementId`,
    `RevisionId`, source map and symbols carry FR-011's canonical structural `qocc2_`/`qpkg2_`
    renderings of each driver's selected claim's complete node, role and ordinal and
    the same admitted package's name and complete lock. Changing any one structural
    identity component changes its rendered text. An unchanged node and lock identity
    never excuses a changed admitted body from the existing body comparison.
- id: FR-002-AC-26
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815; replaces AC-23). At a V2 harness identity text comparison
    boundary, malformed, truncated, case-folded or otherwise noncanonical structural
    `qocc2_`/`qpkg2_` text, including an invalid byte count or malformed nested path
    or lock encoding, returns `HarnessErrorCode::InvalidIdentityRendering` before
    publication and without normalizing or mutating a report. A different but canonical
    occurrence returns `MissingClaimOccurrence`; a different but canonical package
    name or lock returns `OwnerIdentityMismatch`, likewise before publication. Exact
    typed and canonical-text comparisons agre
- id: FR-008-AC-1
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED (IR-364). A fresh QSL-produced, IR-admitted ConfigVersion V2 postcondition
    with direct `self.versionNumber = pre(self.versionNumber)` yields two `State`
    census entries in authored order, `Post` then `Pre`, each 0..=1000, with the same
    project id, `quire.op.integer.eq`, and the selected clause and authentic claim
    retained. The let-bound alias form yields a typed operand refusal and no bundle.
- id: FR-008-AC-2
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED (IR-364). An IR-admitted V2 `amount < 7` invariant with a typed
    current read of range 0..=1000 yields operator `Less`, primary `amount`, literal
    7 and that inclusive census domain. Reversing the two authored operands preserves
    the reversed positions and corresponding operator semantics.
- id: FR-008-AC-3
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED (IR-364). A valid selected comparison generates despite a comparison-oracle-refused
    sibling claim in the same package; requesting the refused claim returns its typed
    selected-claim cause and locus with no bundle. An oracle-construction failure
    for the selected claim returns no bundle. Each of integer.eq/ne/lt/le/gt/ge has
    an executable selected-claim comparison oracle whose result agrees with the ordered
    relation and generated tags. Arithmetic, alias, Boolean and literal-only shapes
    refuse at their own structural loci.
- id: FR-008-AC-4
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED (IR-364). Unknown clause, absent claim, unsupported clause kind
    and malformed comparison report distinct typed causes in the stated first-defect
    order. Diagnostics carry selected clause and claim, structural path and reached
    child id when available, without claiming a unique child source region.
- id: FR-008-AC-5
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED (IR-364). A request with several applicable defects returns the
    first only. Every refusal returns zero artifacts; a successful request returns
    the complete source, census and runner bundle.
- id: FR-008-AC-6
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED (IR-364). Either read endpoint or a literal at i64::MIN - 1, i64::MAX
    + 1, i128::MIN or i128::MAX refuses `UnsupportedRelation` at that operand without
    narrowing. Two individually i64-representable reads with unequal ranges also refuse;
    equal ranges admit when other rules hold.
- id: FR-008-AC-7
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. Beyond the per-operator agreement stated in FR-008-AC-3, which
    this criterion does not repeat, for each of integer.eq, ne, lt, le, gt and ge
    the selected claim's V2 comparison oracle returns, for every valuation of a 1-
    to 4-member domain (read-versus-read and read-versus-literal, the literal at each
    member and in either authored position), the result of an independently evaluated
    ordered relation. For every such valuation, the FR-014 clause oracle's result
    for the same direct comparison is identical to the selected oracle's result and
    to that independent relation.
- id: FR-008-AC-8
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. The V2 comparison oracle takes only the accessor's typed result
    for the selected claim. Source inspection finds exactly one `lower_typed_integer_comparison`
    function under `src/oracle` used by both the selected strategy oracle and FR-014's
    clause oracle for all six operators and the ordered direct read/read and read/literal
    forms; neither path has a second CG comparison lowerer or evaluator module, and
    no V1 bound oracle, `BoundPackage` or V1-to-V2 adapter is an input to the V2 path.
- id: FR-008-AC-9
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. A CG integration test calls IR's public clause-context accessor
    through its public API only, and the generator's census, ordered relation and
    diagnostics equal the accessor's operator, ordinals, provenance, observation,
    bounds and refusal fields for each closed operator, provenance and observation
    variant.
- id: FR-008-AC-10
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. Source inspection shows the generator reads no `CheckedSemanticNodeV2.body`
    field and decodes no body JSON (the verification of FR-008-CON-1).
- id: FR-008-CON-1
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: The generator SHALL read V2 operand identity, provenance, observation,
    operator and bounds only through IR's public typed accessor. A private decoder
    or copied schema is forbidden.
- id: FR-008-CON-2
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: Admission SHALL finish before any strategy, population, census or runner
    source is published; a refusal returns no artifact or partial bundle.
- id: FR-008-CON-3
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: The generator SHALL check each read endpoint for i64 representability before
    constructing the shared-domain relation.
- id: FR-008-CON-4
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: The generator SHALL check each comparison literal for i64 representability
    before conversion.
- id: FR-009-AC-1
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: "PLANNED (IR-364). For the QSL-produced, IR-admitted V2 `VersionUnchanged`\
    \ selected claim (FR-008-AC-1) over 0..=1000, every `Satisfying` case has post\
    \ = pre, every `Violating` case has post \u2260 pre, and every value lies in 0..=1000."
- id: FR-009-AC-2
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: 'PLANNED (IR-364). For every V2 comparison operator, every domain of 1,
    2, 3, and 4 members, and both a two-read relation and a read-versus-literal relation
    with the literal at each domain member, the generatable set of each population
    equals the independently enumerated satisfying or violating set: none missing,
    none extra. An empty side is refused with `EmptyPopulation` and never produces
    an empty strategy.'
- id: FR-009-AC-3
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: PLANNED (IR-364). Generated population source contains no filter, assume,
    reject, or discard call, and a seeded 10,000-case run of each population records
    zero discards and zero proptest global rejects.
- id: FR-009-AC-4
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: PLANNED (IR-364). For `amount < 0` over 0..=1000, `Satisfying` and `Broad`
    are refused with `EmptyPopulation` naming `Satisfying`, and `Violating` generates;
    for `amount >= 0`, `Violating` and `Broad` are refused naming `Violating`; no
    refused request emits an artifact.
- id: FR-009-AC-5
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: PLANNED (IR-364). A domain of `i64::MIN..=i64::MAX` with `NotEqual`, `Less`,
    and `Equal` compiles as generated Rust and generates 10,000 seeded cases without
    overflow or panic, every case in domain and on its side.
- id: FR-009-AC-6
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: PLANNED (IR-364). Repeated generation for the same `CheckedPackageV2`,
    selected clause id, authentic claim occurrence, and population is byte-identical.
- id: FR-009-AC-7
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: PLANNED. For each noncommutative operator, a comparison written with the
    literal in the first position produces the same satisfying and violating sets
    as its mirrored read-first form, enumerated independently.
- id: FR-010-AC-1
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: PLANNED (IR-364). For `amount < 7` over 0..=1000, the in-domain census
    is exactly [0 Holds, 1 Holds, 6 Holds, 7 Violated, 8 Violated, 999 Violated, 1000
    Violated], and the out-of-domain array is exactly [-1, 1001]; for `amount < 1`,
    where the literal edges coincide with the domain edges, the in-domain census is
    exactly [0 Holds, 1 Violated, 2 Violated, 999 Violated, 1000 Violated] with no
    duplicates, and the out-of-domain array is exactly [-1, 1001].
- id: FR-010-AC-2
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: 'PLANNED (IR-364). For the QSL-produced, IR-admitted V2 `VersionUnchanged`
    selected claim (primary post, partner pre; FR-008-AC-1) over 0..=1000, the in-domain
    census is exactly the 10 pairs (post, pre): (0,0 H), (0,1 V), (1,0 V), (1,1 H),
    (1,2 V), (999,998 V), (999,999 H), (999,1000 V), (1000,999 V), (1000,1000 H);
    the out-of-domain array is exactly the 10 pairs (-1,0), (0,-1), (0,1001), (1,-1),
    (1,1001), (999,-1), (999,1001), (1000,-1), (1000,1001), (1001,1000).'
- id: FR-010-AC-3
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: PLANNED (IR-364). A selected claim whose accessor domain is exactly `i64::MIN..=i64::MAX`
    (representable under FR-008-AC-6) compiles as generated Rust without overflow
    or panic, its out-of-domain array holds no case for either outer edge, and its
    unrepresentable-edge array lists both.
- id: FR-010-AC-4
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: PLANNED (IR-364). `amount <= 1000` over 0..=1000 refuses `Boundary` with
    `UnsupportedCampaignConstraint` and emits no artifact for that request, while
    its `Satisfying` bundle still carries the out-of-domain array [-1, 1001].
- id: FR-010-AC-5
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: PLANNED (IR-364). Every in-domain census tag equals an independent evaluation
    of the relation, every out-of-domain case has at least one value outside the domain,
    and repeated generation produces byte-identical arrays.
- id: FR-011-AC-1
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED (IR-364). Over V2 selected claims, a QSL-produced `VersionUnchanged`
    (`Postcondition`) `Broad` campaign records `Holds` cases as `Passed` and `Violated`
    cases as `FailedPostcondition` and passes; a `Precondition` fixture records `Violated`
    cases as `RejectedPrecondition` in `rejected` and passes with zero proptest global
    rejects; the `amount < 7` `Invariant` records `Violated` cases as `FailedPostcondition`.
- id: FR-011-AC-2
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED (IR-364). A V2 comparison oracle deliberately swapped for its negation
    fails each campaign with `ConformanceMismatch` carrying the case values, expected
    tag, and observed verdict kind.
- id: FR-011-AC-3
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED (IR-364). The census runner evaluates each of the 10 `VersionUnchanged`
    in-domain census cases exactly once, in census order, and evaluates none of the
    out-of-domain cases.
- id: FR-011-AC-4
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED (IR-364). A campaign over a report seeded with fixed prior counts
    returns `discard_rate() == Some((discarded, attempted))` and `rejection_rate()
    == Some((rejected, attempted))` with those exact values, a fresh zero-attempt
    summary returns `None` for both, and a runtime snapshot whose counters are `at_limit`
    returns `None` for both through the generated snapshot-summary operation.
- id: FR-011-AC-5
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED (IR-364). A fresh 10,000-case `Broad` campaign for each fixture
    returns `discard_rate() == Some((0, attempted))` with `attempted > 0`, passes
    a zero discard ceiling, and does not end `Exhausted`.
- id: FR-011-AC-6
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: 'SUPERSEDED by FR-011-AC-14 under QSpec #449 / IR-1070; no implementation
    credit. Original planned text: For a selected checked clause id and authentic
    `claim` occurrence in one admitted V2 package, every constructed verdict carries
    the runtime point rendered from the accepted QSL `ClauseBinding.execution_point`
    of that exact tuple, with variant and anchor name preserved. Invariant fixtures
    at both initialization and handler anchors, and two distinct handler anchors,
    produce distinct runtime point strings; `Precondition` and `Postcondition` fixtures
    preserve their own binding points. Two authen'
- id: FR-011-AC-7
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: 'SUPERSEDED by FR-011-AC-15 under QSpec #449 / IR-1070; no implementation
    credit. Original planned text: An absent checked clause maps Contract IR FR-058
    `UnknownClause` to `StrategyErrorCode::UnknownClause`; a non-state-clause maps
    `NotStateClause` to `StrategyErrorCode::UnsupportedClauseKind`, before occurrence
    checks and with no artifact. A wrong-role, wrong-ordinal, or other-node `claim`
    occurrence returns `StrategyErrorCode::MissingClaimOccurrence` and no artifact,
    even when another authentic occurrence of the selected node has a point. An authentic
    selected tuple lacking an accepted bindi'
- id: FR-011-AC-8
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: RETIRED, replaced by FR-011-AC-19; the former digest-shaped rendering is
    obsolete and earns no implementation credit.
- id: FR-011-AC-9
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: RETIRED, replaced by FR-011-AC-20; the former domain/algorithm/digest owner
    comparison is obsolete and earns no implementation credit.
- id: FR-011-AC-10
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED. The QSL driver forwards only FR-331 items accepted by `export_bindings`;
    its `UnknownItem`, `Unbound`, and `Fault` results stay driver-side. An out-of-driver
    non-claim or absent occurrence returns CG `StrategyErrorCode::MissingClaimOccurrence`,
    and a selected claim with an unbound referenced model element returns `StrategyErrorCode::UnboundModelElement`
    naming it. Neither produces an artifact or verdict, and a matched sibling remains
    unaffected.
- id: FR-011-AC-11
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: RETIRED, replaced by FR-011-AC-21; the former digest-shaped text control
    is obsolete and earns no implementation credit.
- id: FR-011-AC-12
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED. Repeated numeric runner generation with the same admitted package,
    selected driver-bound FR-331 claim, checked clause, and campaign policy yields
    byte-identical source and map.
- id: FR-011-AC-13
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: 'PLANNED (IR-1072). The public V2 selected-claim entry accepts an admitted
    `CheckedPackageV2` and a typed complete claim key (checked clause id, claim owner
    node id, authentic `claim` `CheckedOccurrence`) without a second caller-supplied
    owner; a matching key selects only that claim and its exact checked point. With
    the same clause and occurrence but a different claim owner, the entry returns
    a distinct typed V2 diagnostic retaining Contract IR `CheckedExecutionPointRefusal::MissingClaimOccurrence`
    and the supplied clause, owner, and occurrence, and publishes no artifact. The
    V1 `BoundPackage` '
- id: FR-011-AC-14
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: 'PLANNED (IR-1070). For selected authentic `claim` occurrences of distinct
    V2 invariant clause nodes whose condition and anchor are equal but whose accepted
    points are respectively `initialization{name: "startup"}` and `handler{name: "tick"}`,
    each constructed verdict carries the exact point of its selected node through
    the FR-058 accessor and injective runtime rendering. The nodes and otherwise identical
    packages have distinct identities; changing only the accepted handler name changes
    its rederived node and package identities and runtime point. Precondition and
    postcondition claims preserve t'
- id: FR-011-AC-15
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED (IR-1070). An absent checked clause returns `StrategyErrorCode::UnknownClause`,
    a non-state clause returns `StrategyErrorCode::UnsupportedClauseKind`, and a wrong-role,
    wrong-ordinal, or other-node occurrence returns `StrategyErrorCode::MissingClaimOccurrence`,
    each before artifact publication and without substituting another claim or node's
    point. An authentic claim of a separate V2 clause node whose optional point is
    absent returns `StrategyErrorCode::MissingExecutionPoint` without borrowing a
    pointed sibling node's point. A renderer mutant changing only the selected point
    returns `S
- id: FR-011-AC-16
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED (IR-1124; depends on IR-1072 and FR-008). A public V2 strategy
    request over an admitted checked package and one authentic selected claim produces
    a complete source, map, census and numeric runner bundle whose selected clause,
    owner, occurrence, package revision, comparison and runner identities agree exactly.
    The runner executes the FR-009 constructive population and FR-010 census with
    the selected V2 oracle; source inspection finds no V1 `BoundPackage`, V1 `ClauseRef`,
    `generate_bound_strategy` call or V1-to-V2 adapter in this V2 path, while the
    selected V2 checked clause key is retai
- id: FR-011-AC-17
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED (IR-1124). Independently injected unknown clause, unsupported kind,
    wrong owner/role/ordinal, missing accepted point, FR-008 comparison refusal, empty
    population, identity mismatch, point-render mutant, invalid generated syntax and
    resource overflow each return a typed diagnostic with the selected key and available
    structural locus, publish zero artifacts and start no campaign. Pairwise overlapping
    defects return the first cause in Behavior order; a valid sibling still publishes
    its complete bundle.
- id: FR-011-AC-18
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED (IR-1124; FR-004 live binding remains separate). A campaign run
    from the emitted V2 runner retains the selected claim and package identity with
    its own source/map and result for same-invocation producer binding. Published
    bytes or a detached/copied result alone yield no authenticated campaign, measured
    coverage or proof credit. A refused selection yields no campaign evidence, including
    when another claim of the same package succeeds.
- id: FR-011-AC-19
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED (IR-1079; replaces AC-8). The numeric runner's `RequirementId`,
    `RevisionId`, generated header, source map and symbol suffix retain the selected
    driver-bound FR-331 claim's complete structural node, role and ordinal and the
    same-owner admitted package's complete name and lock through the canonical `qocc2_`/`qpkg2_`
    renderings. Changing any one admitted identity component changes the corresponding
    text; a body-only edit retaining the same structural node and lock leaves that
    identity text unchanged and is still compared through the admitted body.
- id: FR-011-AC-20
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED (IR-1079; replaces AC-9). For a process request, a FR-331 request
    whose package name or complete lock differs from the separately admitted package
    returns `StrategyErrorCode::OwnerIdentityMismatch` before any bundle, verdict
    or report mutation, while a matched sibling retains its disposition. A package-byte/lock
    mismatch stays the FR-331 reader's typed refusal. For a linked request, E4/E5
    package disagreement stays the driver's handoff refusal; CG checks the selected
    claim against the one held E5-admitted package without a second owner field.
- id: FR-011-AC-21
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED (IR-1079; replaces AC-11). Malformed, truncated, case-folded or
    otherwise noncanonical structural `qocc2_`/`qpkg2_` text, including a wrong byte
    count or malformed nested path or lock encoding, yields `StrategyErrorCode::InvalidIdentityRendering`
    before bundle publication or verdict/report mutation, without normalization. A
    different but canonical node, role or ordinal yields `MissingClaimOccurrence`;
    a different but canonical package name or lock yields `OwnerIdentityMismatch`,
    at the same no-publication boundary. Exact typed and canonical-text comparisons
    agree on the accepted pair.
- id: FR-012-AC-1
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: PLANNED (IR-364). For every V2 comparison operator and every domain of
    1 to 4 members from FR-009-AC-2, an exhaustive walk of every valid `simplify`
    and `complicate` path of `Satisfying`, `Violating`, and `Broad` value trees visits
    only in-domain candidates that keep their original tag; a valid path calls `complicate`
    only after at least one successful `simplify`.
- id: FR-012-AC-2
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: PLANNED (IR-364). For 1,000 seeded `VersionUnchanged` value trees of each
    population over 0..=1000, every candidate on the greedy shrink path stays in domain
    and keeps its tag.
- id: FR-012-AC-3
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: PLANNED (IR-364). A `Violating` V2 `VersionUnchanged` campaign whose comparison
    oracle is replaced by `post == pre
- id: FR-012-AC-4
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: PLANNED (IR-364). After a shrinking campaign, `attempted` equals the number
    of oracle evaluations the runner performed, including shrink replays.
- id: FR-013-AC-1
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: PLANNED (IR-364). A consumer fixture crate that includes the generated
    Rust source and whose manifest depends only on `proptest` and `quire-contract-runtime`
    compiles under denied warnings, draws `VersionUnchanged` cases, and reads the
    selected `versionNumber` StateField at `"pre"` and `"post"` through the generated
    typed-identity, path and observation constants.
- id: FR-013-AC-2
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: PLANNED (IR-364). The generated bundle contains no serialized case, census,
    or summary file, and the change adds no file under `schemas/`.
- id: FR-013-AC-4
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: PLANNED (IR-364). The generated Rust header carries the selected checked
    clause id and authentic claim occurrence, and generating two distinct selected
    claims from one package produces different headers and artifact paths.
- id: FR-013-AC-5
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: PLANNED (IR-364). interface-001 `bound_strategy_slice.consumer` records
    the case, census, and runner surface a downstream consumer compiles against.
- id: NFR-004-AC-1
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: PLANNED (IR-364). Fresh 10,000-case and 256-case campaigns of every population
    over the V2 selected claims `VersionUnchanged`, `amount < 7`, and a `Precondition`
    fixture record zero discards, zero proptest global rejects, and no `Exhausted`
    conclusion.
- id: NFR-004-AC-2
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: PLANNED (IR-364). For every admitted fixture in TC-019, the in-domain census
    plus out-of-domain array holds at most 20 cases.
- id: NFR-004-M-1
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: Discarded / attempted on a fresh 10,000-case campaign per population and
    fixture
- id: NFR-004-M-2
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: Proptest global rejects on the same campaigns
- id: NFR-004-M-3
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: Campaigns ending `Exhausted` on the same campaigns, and on 256-case default-config
    campaigns
- id: NFR-004-M-4
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: In-domain plus out-of-domain census cases per admitted clause
bindings:
- test_id: src/publication/publish.rs::tests::publication_is_order_independent_and_replaces_only_its_destination
  ac_id: FR-005-AC-1
  trace: correct
- test_id: src/publication/publish.rs::tests::every_injected_failure_preserves_old_and_developer_owned_bytes
  ac_id: FR-005-AC-1
  trace: correct
- test_id: src/publication/publish.rs::tests::failed_rollback_reports_unknown_and_preserves_complete_recovery_bundles
  ac_id: FR-005-AC-1
  trace: correct
- test_id: src/publication/publish.rs::tests::post_commit_cleanup_failure_reports_that_the_new_bundle_is_published
  ac_id: FR-005-AC-1
  trace: correct
- test_id: src/publication/publish.rs::tests::unsafe_and_duplicate_artifact_paths_are_refused
  ac_id: FR-005-AC-1
  trace: correct
- test_id: src/publication/publish.rs::tests::cleanup_removes_a_sibling_symlink_without_following_it
  ac_id: FR-005-AC-1
  trace: correct
- test_id: src/publication/publish.rs::tests::publication_is_order_independent_and_replaces_only_its_destination
  ac_id: FR-005-AC-2
  trace: correct
- test_id: src/publication/publish.rs::tests::bundle_construction_refuses_more_than_the_bounded_artifact_count
  ac_id: FR-005-AC-5
  trace: correct
- test_id: src/publication/publish.rs::tests::bundle_construction_refuses_an_artifact_over_the_bounded_size
  ac_id: FR-005-AC-5
  trace: correct
- test_id: src/publication/publish.rs::tests::bundle_construction_refuses_a_complete_bundle_over_the_bounded_size
  ac_id: FR-005-AC-5
  trace: correct
- test_id: src/publication/publish.rs::tests::unsafe_and_duplicate_artifact_paths_are_refused
  ac_id: FR-005-AC-6
  trace: correct
- test_id: src/publication/publish.rs::tests::every_injected_failure_preserves_old_and_developer_owned_bytes
  ac_id: FR-005-AC-7
  trace: correct
- test_id: src/publication/publish.rs::tests::real_backup_cleanup_failure_reports_published_with_both_new_artifacts
  ac_id: FR-005-AC-8
  trace: correct
- test_id: src/publication/publish.rs::tests::failed_rollback_reports_unknown_and_preserves_complete_recovery_bundles
  ac_id: FR-005-AC-9
  trace: correct
- test_id: src/publication/publish.rs::tests::unsafe_and_duplicate_artifact_paths_are_refused
  ac_id: FR-005-AC-10
  trace: correct
- test_id: tests/composite_equality_support/package.rs::fixture_identity_tests::composite_application_keeps_producer_assigned_node_id
  ac_id: FR-037-AC-6
  trace: correct
- test_id: tests/exact_scalar_support/package.rs::fixture_identity_tests::source_assigned_application_and_enum_member_ids_survive_admission
  ac_id: FR-037-AC-6
  trace: correct
- test_id: tests/composite_equality_support/package.rs::fixture_identity_tests::grouped_application_keeps_each_structural_member_and_refuses_broken_inputs
  ac_id: FR-037-AC-7
  trace: correct
- test_id: tests/composite_equality_support/package.rs::fixture_identity_tests::enum_member_case_uses_the_producers_structural_id
  ac_id: FR-037-AC-8
  trace: correct
- test_id: tests/exact_scalar_support/package.rs::fixture_identity_tests::source_assigned_application_and_enum_member_ids_survive_admission
  ac_id: FR-037-AC-8
  trace: correct
- test_id: tests/composite_equality_support/package.rs::fixture_identity_tests::grouped_application_keeps_each_structural_member_and_refuses_broken_inputs
  ac_id: FR-037-AC-9
  trace: correct
- test_id: tests/composite_equality_support/package.rs::fixture_identity_tests::changed_body_and_unused_ordinal_are_measured_separately_from_identity
  ac_id: FR-037-AC-9
  trace: correct
- test_id: tests/exact_scalar_support/package.rs::fixture_identity_tests::bounded_corpus_cohorts_admit_every_requested_scalar_once
  ac_id: FR-037-AC-9
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_052_reader_refuses_declaration_path_mismatch_in_admitted_positive_wire
  ac_id: FR-037-AC-9
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_052_changed_application_body_is_not_credited_as_original_fixture
  ac_id: FR-037-AC-9
  trace: correct
- test_id: src/kani/generate/frame.rs::tests::state_projection_uses_catalogued_comparison_symbols
  ac_id: FR-038-AC-1
  trace: correct
- test_id: src/oracle/operator_catalog.rs::tests::catalog_entries_preserve_identity_family_arity_result_and_consumer_projection
  ac_id: FR-038-AC-1
  trace: correct
- test_id: src/oracle/operator_catalog.rs::tests::catalog_entries_preserve_identity_family_arity_result_and_consumer_projection
  ac_id: FR-038-AC-2
  trace: correct
- test_id: src/kani/generate/frame.rs::tests::state_projection_uses_catalogued_comparison_symbols
  ac_id: FR-038-AC-3
  trace: correct
- test_id: src/kani/generate/scalar.rs::tests::scalar_projection_selects_arithmetic_and_boolean_implication
  ac_id: FR-038-AC-3
  trace: correct
- test_id: src/oracle/operator_catalog.rs::tests::catalog_entries_preserve_identity_family_arity_result_and_consumer_projection
  ac_id: FR-038-AC-3
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_operator_confusion_within_one_shape_is_not_silently_confirmed
  ac_id: FR-038-AC-3
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_catalog_consumer_negative_cases_refuse_wrong_shapes_and_preserve_order_and_target
  ac_id: FR-038-AC-3
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_a_caller_declared_operation_is_refused_with_no_harness
  ac_id: FR-038-AC-3
  trace: correct
- test_id: src/kani/generate/scalar.rs::tests::scalar_projection_selects_arithmetic_and_boolean_implication
  ac_id: FR-038-AC-4
  trace: correct
- test_id: src/kani/generate/frame.rs::tests::state_projection_uses_catalogued_comparison_symbols
  ac_id: FR-038-AC-6
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::admitted_ordering_variants_change_routed_oracle_bytes_and_keep_refused_sibling
  ac_id: FR-039-AC-1
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::admitted_bounded_integer_operation_generates_from_reached_bounds
  ac_id: FR-039-AC-1
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::admitted_ordering_variants_change_routed_oracle_bytes_and_keep_refused_sibling
  ac_id: FR-039-AC-2
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::scalar_own_result_without_conversion_differs_from_not_applicable
  ac_id: FR-039-AC-2
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::duplicate_expression_uses_each_selected_conversion_context
  ac_id: FR-039-AC-2
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::scalar_selected_occurrence_refusals_preserve_generated_sibling
  ac_id: FR-039-AC-2
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::one_routed_profile_admits_function_and_equality_beside_scalar_bound_refusal
  ac_id: FR-039-AC-2
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::selected_function_targets_keep_reader_identity_and_order
  ac_id: FR-039-AC-3
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::checked_reader_refuses_forged_same_key_local_function
  ac_id: FR-039-AC-3
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::admitted_function_settlement_does_not_read_again
  ac_id: FR-039-AC-5
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::admitted_function_settlement_with_context_root_is_supported
  ac_id: FR-039-AC-5
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::admitted_failed_accounting_refuses_without_an_oracle
  ac_id: FR-039-AC-6
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::scalar_selected_occurrences_remain_package_local
  ac_id: FR-039-AC-7
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::genuine_qsl_function_claim_uses_admitted_package_identity_and_reached_keys
  ac_id: FR-039-AC-7
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::one_routed_profile_admits_function_and_equality_beside_scalar_bound_refusal
  ac_id: FR-039-AC-7
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::genuine_qsl_function_claim_uses_admitted_package_identity_and_reached_keys
  ac_id: FR-039-AC-8
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::excluded_position_keeps_accounting_and_driver_index
  ac_id: FR-039-AC-9
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::source_name_and_lock_must_match_admitted_accounting
  ac_id: FR-039-AC-9
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::excluded_position_refuses_function_oracle_before_claim_or_artifact
  ac_id: FR-039-AC-9
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::excluded_position_refuses_equality_oracle_before_claim_or_artifact
  ac_id: FR-039-AC-9
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::excluded_position_refuses_scalar_oracle_before_claim_or_artifact
  ac_id: FR-039-AC-9
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::routed_excluded_driver_index_is_refused_before_generation
  ac_id: FR-039-AC-9
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::excluded_position_ends_context_span_and_orphans_following_root
  ac_id: FR-039-AC-10
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::duplicate_expression_uses_each_selected_conversion_context
  ac_id: FR-039-AC-10
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::admitted_bounded_integer_operation_generates_from_reached_bounds
  ac_id: FR-039-AC-11
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::scalar_selected_occurrences_remain_package_local
  ac_id: FR-039-AC-11
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::genuine_qsl_function_claim_uses_admitted_package_identity_and_reached_keys
  ac_id: FR-039-AC-11
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::one_routed_profile_admits_function_and_equality_beside_scalar_bound_refusal
  ac_id: FR-039-AC-11
  trace: correct
- test_id: src/publication/publish.rs::tests::publication_is_order_independent_and_replaces_only_its_destination
  ac_id: NFR-001-AC-1
  trace: correct
- test_id: tests/it/oracle_generation.rs::tc_002_integer_and_state_comparisons_are_deterministic_compile_and_match_the_model
  ac_id: NFR-001-AC-1
  trace: correct
- test_id: src/publication/publish.rs::tests::publication_is_order_independent_and_replaces_only_its_destination
  ac_id: NFR-001-AC-2
  trace: correct
- test_id: src/publication/publish.rs::tests::every_injected_failure_preserves_old_and_developer_owned_bytes
  ac_id: NFR-001-AC-2
  trace: correct
- test_id: src/publication/publish.rs::tests::failed_rollback_reports_unknown_and_preserves_complete_recovery_bundles
  ac_id: NFR-001-AC-2
  trace: correct
- test_id: src/publication/publish.rs::tests::post_commit_cleanup_failure_reports_that_the_new_bundle_is_published
  ac_id: NFR-001-AC-2
  trace: correct
- test_id: src/publication/publish.rs::tests::cleanup_removes_a_sibling_symlink_without_following_it
  ac_id: NFR-001-AC-2
  trace: correct
- test_id: src/publication/publish.rs::tests::publication_is_order_independent_and_replaces_only_its_destination
  ac_id: NFR-001-AC-3
  trace: correct
- test_id: src/publication/publish.rs::tests::every_injected_failure_preserves_old_and_developer_owned_bytes
  ac_id: NFR-001-AC-3
  trace: correct
- test_id: src/publication/publish.rs::tests::failed_rollback_reports_unknown_and_preserves_complete_recovery_bundles
  ac_id: NFR-001-AC-3
  trace: correct
- test_id: src/publication/publish.rs::tests::post_commit_cleanup_failure_reports_that_the_new_bundle_is_published
  ac_id: NFR-001-AC-3
  trace: correct
- test_id: src/publication/publish.rs::tests::unsafe_and_duplicate_artifact_paths_are_refused
  ac_id: NFR-001-AC-3
  trace: correct
- test_id: src/publication/publish.rs::tests::cleanup_removes_a_sibling_symlink_without_following_it
  ac_id: NFR-001-AC-3
  trace: correct
- test_id: src/publication/publish.rs::tests::every_injected_failure_preserves_old_and_developer_owned_bytes
  ac_id: NFR-001-M-2
  trace: correct
- test_id: src/publication/publish.rs::tests::every_injected_failure_preserves_old_and_developer_owned_bytes
  ac_id: NFR-001-M-3
  trace: correct
- test_id: src/core/provenance.rs::tests::newline_in_subject_stays_inside_header_comment
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: src/kani/generate/render.rs::tests::renderer_emits_only_fixed_structure_and_supplied_data_in_order
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::extra_equality_module_has_its_own_header
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::scalar_own_result_without_conversion_differs_from_not_applicable
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: tests/it/bound_generation.rs::complete_public_binding_preserves_identity_population_and_derivation
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: tests/it/bound_populations.rs::tc_018_generated_sources_are_constructive_and_byte_identical
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::bound_strategy_header_retains_its_clause_identity
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::public_family_and_corpus_results_have_direct_ir_errors_and_unchanged_successes
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac10_generation_is_repeatable_from_a_fresh_package
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac34_public_strict_reader_chain_generates_compilable_flat_schedule
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac7_admitted_hundred_thousand_call_nodes_generate_on_a_512_kib_stack
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac13_generation_is_deterministic_across_runs_and_permutations
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_every_scalar_family_generates_one_oracle_calling_its_runtime_operator
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: tests/it/harness_generation.rs::tc_004_generated_harness_binds_clauses_and_executes_all_three_terminal_paths
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_scalar_harness_asserts_the_native_arithmetic_relation
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_a_postcondition_yields_a_contract_harness_and_a_scoped_frame_harness
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::negotiation_renders_the_selected_sequence_claims_deterministically
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: tests/it/oracle_generation.rs::tc_001_boolean_oracle_bundle_is_deterministic_traceable_and_schema_valid
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: tests/it/strategy_generation.rs::enum_strategy_header_retains_input_identity
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: tests/it/strategy_generation.rs::tc_004_supported_populations_are_directly_shaped_and_shrink_inside_constraints
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: src/kani/generate/render.rs::tests::renderer_emits_only_fixed_structure_and_supplied_data_in_order
  ac_id: NFR-002-AC-2
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::extra_equality_module_has_its_own_header
  ac_id: NFR-002-AC-2
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::scalar_own_result_without_conversion_differs_from_not_applicable
  ac_id: NFR-002-AC-2
  trace: correct
- test_id: tests/it/bound_generation.rs::complete_public_binding_preserves_identity_population_and_derivation
  ac_id: NFR-002-AC-2
  trace: correct
- test_id: tests/it/bound_populations.rs::tc_018_generated_sources_are_constructive_and_byte_identical
  ac_id: NFR-002-AC-2
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::bound_strategy_header_retains_its_clause_identity
  ac_id: NFR-002-AC-2
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::public_family_and_corpus_results_have_direct_ir_errors_and_unchanged_successes
  ac_id: NFR-002-AC-2
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac10_generation_is_repeatable_from_a_fresh_package
  ac_id: NFR-002-AC-2
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac34_public_strict_reader_chain_generates_compilable_flat_schedule
  ac_id: NFR-002-AC-2
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac7_admitted_hundred_thousand_call_nodes_generate_on_a_512_kib_stack
  ac_id: NFR-002-AC-2
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac13_generation_is_deterministic_across_runs_and_permutations
  ac_id: NFR-002-AC-2
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_every_scalar_family_generates_one_oracle_calling_its_runtime_operator
  ac_id: NFR-002-AC-2
  trace: correct
- test_id: tests/it/harness_generation.rs::tc_004_generated_harness_binds_clauses_and_executes_all_three_terminal_paths
  ac_id: NFR-002-AC-2
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_scalar_harness_asserts_the_native_arithmetic_relation
  ac_id: NFR-002-AC-2
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_a_postcondition_yields_a_contract_harness_and_a_scoped_frame_harness
  ac_id: NFR-002-AC-2
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::negotiation_renders_the_selected_sequence_claims_deterministically
  ac_id: NFR-002-AC-2
  trace: correct
- test_id: tests/it/oracle_generation.rs::tc_001_boolean_oracle_bundle_is_deterministic_traceable_and_schema_valid
  ac_id: NFR-002-AC-2
  trace: correct
- test_id: tests/it/strategy_generation.rs::enum_strategy_header_retains_input_identity
  ac_id: NFR-002-AC-2
  trace: correct
- test_id: tests/it/strategy_generation.rs::tc_004_supported_populations_are_directly_shaped_and_shrink_inside_constraints
  ac_id: NFR-002-AC-2
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::wide_ir_domain_refuses_the_i64_strategy
  ac_id: NFR-002-AC-3
  trace: correct
- test_id: tests/it/harness_generation.rs::tc_004_invalid_execution_point_is_a_structured_failure_without_artifact
  ac_id: NFR-002-AC-3
  trace: correct
- test_id: tests/it/harness_generation.rs::tc_004_multiple_state_bindings_fail_closed
  ac_id: NFR-002-AC-3
  trace: correct
- test_id: tests/it/oracle_generation.rs::wide_ir_integer_bounds_refuse_an_i64_oracle_without_generated_source
  ac_id: NFR-002-AC-3
  trace: correct
- test_id: tests/it/oracle_generation.rs::tc_003_unsupported_dependency_reports_the_first_reference_span
  ac_id: NFR-002-AC-3
  trace: correct
- test_id: tests/it/oracle_generation.rs::tc_003_source_size_limit_rejects_without_a_partial_bundle
  ac_id: NFR-002-AC-3
  trace: correct
- test_id: tests/it/strategy_generation.rs::tc_004_invalid_shapes_fail_with_structured_diagnostics
  ac_id: NFR-002-AC-3
  trace: correct
- test_id: src/kani/generate/corpus/bounded_kani_corpus.rs::tests::tc_023_unencodable_case_identity_refuses_without_claiming
  ac_id: NFR-005-AC-1
  trace: correct
- test_id: tests/it/no_generation_panics.rs::tc_042_ac1_the_literal_free_scan_keeps_code_and_drops_literals_comments_and_test_items
  ac_id: NFR-005-AC-1
  trace: correct
- test_id: tests/it/no_generation_panics.rs::tc_042_ac1_src_holds_no_panic_token
  ac_id: NFR-005-AC-1
  trace: correct
- test_id: src/kani/generate/scalar.rs::tests::tc_042_ac2_a_claim_without_a_derivable_first_bound_has_no_renderer
  ac_id: NFR-005-AC-2
  trace: correct
- test_id: tests/it/no_generation_panics.rs::tc_042_ac2_a_claim_map_this_generator_did_not_produce_is_operation_not_rendered
  ac_id: NFR-005-AC-2
  trace: correct
- test_id: tests/it/no_generation_panics.rs::tc_042_ac3_an_unsupported_settlement_warns_only_for_an_unsupported_projection_cause
  ac_id: NFR-005-AC-3
  trace: correct
- test_id: src/evidence/bound_coverage.rs::tests::tc_042_ac4_a_region_without_a_probe_is_a_map_mismatch_not_a_panic
  ac_id: NFR-005-AC-4
  trace: correct
- test_id: src/routed/generate.rs::tests::tc_042_ac5_a_record_count_other_than_the_item_count_is_a_typed_refusal
  ac_id: NFR-005-AC-5
  trace: correct
- test_id: src/routed/generate.rs::tests::tc_042_ac6_a_duplicate_position_outside_the_group_is_a_typed_refusal
  ac_id: NFR-005-AC-6
  trace: correct
- test_id: src/routed/generate.rs::tests::tc_042_ac6_an_out_of_range_duplicate_position_refuses_the_whole_routing
  ac_id: NFR-005-AC-6
  trace: correct
- test_id: src/evidence/bound_coverage.rs::tests::tc_042_ac7_a_map_shorter_than_two_regions_is_a_census_mismatch_not_a_panic
  ac_id: NFR-005-AC-7
  trace: correct
- test_id: tests/it/no_generation_panics.rs::tc_042_ac8_the_ir_577_bodies_hold_no_index_or_subtraction
  ac_id: NFR-005-AC-8
  trace: correct
- test_id: tests/it/no_generation_panics.rs::tc_042_ac8_the_body_check_flags_index_and_subtraction_and_passes_their_lookalikes
  ac_id: NFR-005-AC-8
  trace: correct
- test_id: tests/it/no_generation_panics.rs::tc_042_ac1_src_holds_no_panic_token
  ac_id: NFR-005-M-1
  trace: correct
- test_id: src/evidence/bound_coverage.rs::tests::tc_042_ac4_a_region_without_a_probe_is_a_map_mismatch_not_a_panic
  ac_id: NFR-005-M-2
  trace: correct
- test_id: src/evidence/bound_coverage.rs::tests::tc_042_ac7_a_map_shorter_than_two_regions_is_a_census_mismatch_not_a_panic
  ac_id: NFR-005-M-2
  trace: correct
- test_id: src/kani/generate/scalar.rs::tests::tc_042_ac2_a_claim_without_a_derivable_first_bound_has_no_renderer
  ac_id: NFR-005-M-2
  trace: correct
- test_id: src/routed/generate.rs::tests::tc_042_ac5_a_record_count_other_than_the_item_count_is_a_typed_refusal
  ac_id: NFR-005-M-2
  trace: correct
- test_id: src/routed/generate.rs::tests::tc_042_ac6_a_duplicate_position_outside_the_group_is_a_typed_refusal
  ac_id: NFR-005-M-2
  trace: correct
- test_id: tests/it/no_generation_panics.rs::tc_042_ac2_a_claim_map_this_generator_did_not_produce_is_operation_not_rendered
  ac_id: NFR-005-M-2
  trace: correct
- test_id: tests/it/no_generation_panics.rs::tc_042_ac3_an_unsupported_settlement_warns_only_for_an_unsupported_projection_cause
  ac_id: NFR-005-M-2
  trace: correct
- test_id: tests/it/no_generation_panics.rs::tc_042_ac8_the_ir_577_bodies_hold_no_index_or_subtraction
  ac_id: NFR-005-M-3
  trace: correct
- test_id: tests/it/bound_coverage.rs::complete_bound_package_is_observed_against_actual_native_llvm
  ac_id: FR-004-AC-1
  trace: correct
- test_id: tests/it/bound_coverage.rs::complete_bound_package_is_observed_against_actual_native_llvm
  ac_id: FR-004-AC-2
  trace: correct
- test_id: tests/it/bound_coverage.rs::complete_bound_package_is_observed_against_actual_native_llvm
  ac_id: FR-004-AC-3
  trace: correct
- test_id: tests/it/bound_coverage.rs::complete_population_is_measured_but_never_run_qualified
  ac_id: FR-004-AC-3
  trace: correct
- test_id: tests/it/bound_coverage.rs::aggregate_consequent_cannot_outcount_its_loop_free_oracle
  ac_id: FR-004-AC-3
  trace: correct
- test_id: tests/it/bound_coverage.rs::strict_domain_schema_refuses_qualification_and_erased_identity
  ac_id: FR-004-AC-4
  trace: correct
- test_id: tests/it/bound_coverage.rs::inexact_artifact_inventory_fails_without_classifications
  ac_id: FR-004-AC-4
  trace: correct
- test_id: tests/it/bound_coverage.rs::normalized_mapping_root_is_retained_and_bounded
  ac_id: FR-004-AC-4
  trace: correct
- test_id: src/evidence/bound_coverage.rs::tests::output_bound_counts_exact_bytes_before_allocating_output
  ac_id: FR-004-AC-5
  trace: correct
- test_id: tests/it/bound_coverage.rs::strict_domain_schema_refuses_qualification_and_erased_identity
  ac_id: FR-004-AC-5
  trace: correct
- test_id: tests/it/bound_coverage.rs::resource_profile_and_path_refusals_preserve_no_classifications
  ac_id: FR-004-AC-5
  trace: correct
- test_id: tests/it/bound_coverage.rs::inexact_artifact_inventory_fails_without_classifications
  ac_id: FR-004-AC-5
  trace: correct
- test_id: tests/it/bound_coverage.rs::missing_observations_are_not_zero_and_foreign_generated_files_refuse
  ac_id: FR-004-AC-5
  trace: correct
- test_id: tests/it/bound_coverage.rs::normalized_mapping_root_is_retained_and_bounded
  ac_id: FR-004-AC-5
  trace: correct
- test_id: tests/it/bound_coverage.rs::aggregate_consequent_cannot_outcount_its_loop_free_oracle
  ac_id: FR-004-AC-5
  trace: correct
- test_id: tests/it/vacuity_primitives.rs::tc_006_invalid_probe_components_refuse_independently
  ac_id: FR-004-AC-5
  trace: correct
- test_id: tests/it/vacuity_primitives.rs::tc_006_relative_source_root_refuses_even_with_matching_manifest
  ac_id: FR-004-AC-5
  trace: correct
- test_id: tests/it/bound_coverage.rs::resource_profile_and_path_refusals_preserve_no_classifications
  ac_id: FR-004-AC-7
  trace: correct
- test_id: tests/it/bound_coverage.rs::complete_bound_package_is_observed_against_actual_native_llvm
  ac_id: FR-004-AC-7
  trace: correct
- test_id: tests/it/bound_coverage.rs::complete_population_is_measured_but_never_run_qualified
  ac_id: FR-004-AC-7
  trace: correct
- test_id: tests/it/bound_coverage.rs::inexact_artifact_inventory_fails_without_classifications
  ac_id: FR-004-AC-7
  trace: correct
- test_id: tests/it/bound_coverage.rs::strict_domain_schema_refuses_qualification_and_erased_identity
  ac_id: FR-004-AC-9
  trace: correct
- test_id: tests/it/bound_coverage.rs::complete_bound_package_is_observed_against_actual_native_llvm
  ac_id: FR-004-AC-9
  trace: correct
- test_id: tests/it/bound_coverage.rs::complete_population_is_measured_but_never_run_qualified
  ac_id: FR-004-AC-9
  trace: correct
- test_id: tests/it/bound_coverage.rs::missing_observations_are_not_zero_and_foreign_generated_files_refuse
  ac_id: FR-004-AC-9
  trace: correct
- test_id: tests/it/bound_coverage.rs::informational_only_is_no_executable_not_invalid_or_exercised
  ac_id: FR-004-AC-9
  trace: correct
- test_id: src/evidence/v2_llvm.rs::request_view_tests::request_view_keeps_complete_population_and_report_bytes_under_input_permutation
  ac_id: FR-004-AC-16
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::admitted_clause_body_sets_the_population_and_implication_count
  ac_id: FR-004-AC-16
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::one_admitted_population_retains_all_four_coverage_classes
  ac_id: FR-004-AC-16
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::every_admitted_clause_requires_its_own_generated_source_map_and_export
  ac_id: FR-004-AC-16
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::admitted_v2_oracle_is_vacuous_under_real_llvm_when_consequent_is_skipped
  ac_id: FR-004-AC-16
  trace: correct
- test_id: src/evidence/v2_llvm.rs::request_view_tests::request_view_keeps_population_and_artifact_refusal_precedence_without_partial_rows
  ac_id: FR-004-AC-18
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::admitted_clause_body_sets_the_population_and_implication_count
  ac_id: FR-004-AC-19
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::one_admitted_population_retains_all_four_coverage_classes
  ac_id: FR-004-AC-19
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::entered_clause_classifies_zero_some_and_all_consequent_probes
  ac_id: FR-004-AC-19
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::admitted_v2_oracle_is_vacuous_under_real_llvm_when_consequent_is_skipped
  ac_id: FR-004-AC-19
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::admitted_clause_body_sets_the_population_and_implication_count
  ac_id: FR-004-AC-20
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::one_admitted_population_retains_all_four_coverage_classes
  ac_id: FR-004-AC-20
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::admitted_clause_body_sets_the_population_and_implication_count
  ac_id: FR-004-AC-21
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::one_admitted_population_retains_all_four_coverage_classes
  ac_id: FR-004-AC-21
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::entered_clause_classifies_zero_some_and_all_consequent_probes
  ac_id: FR-004-AC-21
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::admitted_clause_body_sets_the_population_and_implication_count
  ac_id: FR-004-AC-22
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::one_admitted_population_retains_all_four_coverage_classes
  ac_id: FR-004-AC-22
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::entered_clause_classifies_zero_some_and_all_consequent_probes
  ac_id: FR-004-AC-22
  trace: correct
- test_id: src/evidence/v2_llvm.rs::request_view_tests::request_view_keeps_complete_population_and_report_bytes_under_input_permutation
  ac_id: FR-004-AC-23
  trace: correct
- test_id: src/evidence/v2_llvm.rs::request_view_tests::request_view_keeps_population_and_artifact_refusal_precedence_without_partial_rows
  ac_id: FR-004-AC-23
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::every_admitted_clause_requires_its_own_generated_source_map_and_export
  ac_id: FR-004-AC-23
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::changed_map_census_and_missing_probe_refuse_all_classifications
  ac_id: FR-004-AC-23
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::removing_source_map_or_complete_clause_inventory_prevents_success
  ac_id: FR-004-AC-23
  trace: correct
- test_id: src/evidence/v2_llvm.rs::request_view_tests::request_view_keeps_population_and_artifact_refusal_precedence_without_partial_rows
  ac_id: FR-004-AC-24
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::malformed_export_and_wrong_source_root_never_invent_a_classification
  ac_id: FR-004-AC-24
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::each_llvm_input_class_refuses_without_invented_probe_counts
  ac_id: FR-004-AC-24
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::foreign_checked_node_in_source_map_refuses_before_observation
  ac_id: FR-004-AC-24
  trace: correct
- test_id: tests/it/v2_llvm_coverage.rs::admitted_v2_oracle_is_vacuous_under_real_llvm_when_consequent_is_skipped
  ac_id: FR-004-AC-24
  trace: correct
- test_id: tests/it/v2_point_renderer.rs::distinct_pointed_invariants_render_distinct_verdict_points_and_identities
  ac_id: FR-004-AC-26
  trace: correct
- test_id: tests/it/v2_point_renderer.rs::two_authentic_claims_of_one_pointed_node_share_the_point
  ac_id: FR-004-AC-26
  trace: correct
- test_id: tests/it/v2_point_renderer.rs::changed_or_unpointed_selection_returns_typed_no_artifact_refusal
  ac_id: FR-004-AC-26
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_symbolic_bounds_equal_ir_domains_and_every_option_is_identity
  ac_id: FR-015-AC-2
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_unbounded_non_finite_and_blocked_items_are_refused_without_harnesses
  ac_id: FR-015-AC-3
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_scalar_source_lookup_preserves_multiple_occurrences_per_node
  ac_id: FR-015-AC-3
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_lowering_and_bound_refusals_keep_their_fr015_dispositions
  ac_id: FR-015-AC-3
  trace: correct
- test_id: src/kani/generate/scalar.rs::tests::tc_025_derive_domain_reads_binding_shaped_range_members
  ac_id: FR-015-AC-5
  trace: correct
- test_id: src/kani/generate/scalar.rs::tests::tc_025_range_members_are_read_by_name_not_position
  ac_id: FR-015-AC-5
  trace: correct
- test_id: src/kani/generate/scalar.rs::tests::tc_025_inverted_ranges_are_unsatisfiable_and_ordered_ranges_are_not
  ac_id: FR-015-AC-5
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_lowering_and_bound_refusals_keep_their_fr015_dispositions
  ac_id: FR-015-AC-5
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_every_confirmed_operation_is_rendered_or_honestly_refused
  ac_id: FR-015-AC-6
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_a_caller_declared_operation_is_refused_with_no_harness
  ac_id: FR-015-AC-6
  trace: correct
- test_id: tests/it/cover_last.rs::tc_025_every_emitted_harness_ends_with_exactly_one_cover
  ac_id: FR-015-AC-7
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_scalar_harness_asserts_the_native_arithmetic_relation
  ac_id: FR-015-AC-7
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_a_postcondition_yields_a_contract_harness_and_a_scoped_frame_harness
  ac_id: FR-015-AC-7
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_real_kani_a_violation_at_the_only_valuation_is_a_counterexample_not_the_covers_playback
  ac_id: FR-015-AC-7
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::precondition_collection_source_is_admitted
  ac_id: FR-015-AC-7
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::real_kani_collection_precondition_distinguishes_totality_from_satisfiability
  ac_id: FR-015-AC-7
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_symbolic_bounds_equal_ir_domains_and_every_option_is_identity
  ac_id: FR-015-AC-9
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_symbolic_bounds_equal_ir_domains_and_every_option_is_identity
  ac_id: FR-015-AC-10
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_symbolic_bounds_equal_ir_domains_and_every_option_is_identity
  ac_id: FR-015-AC-11
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_every_item_is_accounted_before_any_harness_is_exposed
  ac_id: FR-015-AC-12
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_a_present_node_with_an_unrecognized_kind_is_refused_rather_than_silently_supported
  ac_id: FR-015-AC-14
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_a_claim_naming_a_node_absent_from_the_graph_is_refused_not_silently_supported
  ac_id: FR-015-AC-14
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_lowering_and_bound_refusals_keep_their_fr015_dispositions
  ac_id: FR-015-AC-14
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_selected_division_preserves_exact_renderer_refusal
  ac_id: FR-015-AC-15
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_a_quantity_conversion_claim_is_unrendered_with_no_harness
  ac_id: FR-015-AC-15
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_an_underivable_item_keeps_its_refusal_and_its_siblings_are_unaffected
  ac_id: FR-015-AC-15
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_quantity_conversion_has_an_oracle_and_no_harness
  ac_id: FR-015-AC-15
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_qsl_shaped_increment_sum_and_negation_are_supported
  ac_id: FR-015-AC-16
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_qsl_shaped_literal_and_consumer_refusals_have_no_harness
  ac_id: FR-015-AC-17
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_qsl_shaped_literal_and_consumer_refusals_have_no_harness
  ac_id: FR-015-AC-18
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::v2_clause_census_refuses_invalid_declared_dependencies
  ac_id: FR-015-AC-22
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::v2_clause_census_is_order_independent_and_records_readiness
  ac_id: FR-015-AC-25
  trace: correct
- test_id: src/kani/generate/frame.rs::tests::tc_025_an_operation_name_with_braces_cannot_break_an_assertion
  ac_id: FR-015-AC-26
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_a_postcondition_yields_a_contract_harness_and_a_scoped_frame_harness
  ac_id: FR-015-AC-26
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_two_clauses_of_one_operation_never_share_a_frame_record_path
  ac_id: FR-015-AC-26
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_a_postcondition_yields_a_contract_harness_and_a_scoped_frame_harness
  ac_id: FR-015-AC-27
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_each_field_is_assumed_in_its_own_declared_range
  ac_id: FR-015-AC-27
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_a_postcondition_yields_a_contract_harness_and_a_scoped_frame_harness
  ac_id: FR-015-AC-28
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_the_frame_harness_follows_the_frame_node_not_the_caller
  ac_id: FR-015-AC-28
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_frame_grants_rekey_to_admitted_order_without_dropping_effects
  ac_id: FR-015-AC-29
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_shapes_without_a_finite_encoding_are_refused_by_name
  ac_id: FR-015-AC-29
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_malformed_requests_and_non_clause_nodes_are_refused
  ac_id: FR-015-AC-29
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_the_single_clause_entry_keeps_its_first_refusal_when_both_roles_refuse
  ac_id: FR-015-AC-29
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_real_kani_proves_the_state_postcondition_and_a_mutated_subject_falsifies_it
  ac_id: FR-015-AC-30
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_real_kani_proves_allowed_and_forbidden_frame_effects_and_a_mutated_frame_falsifies
  ac_id: FR-015-AC-31
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_real_kani_frame_counterexample_replays_through_qsl
  ac_id: FR-015-AC-32
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_real_kani_the_allowed_frame_effect_replays_as_a_respected_frame
  ac_id: FR-015-AC-32
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_the_frame_replay_envelope_names_the_payloads_frame_and_occurrence
  ac_id: FR-015-AC-33
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_the_frame_replay_envelope_names_the_payloads_frame_and_occurrence
  ac_id: FR-015-AC-34
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_an_operation_with_no_frame_is_refused_when_the_request_is_built
  ac_id: FR-015-AC-35
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_frame_replay_settles_a_forbidden_and_a_granted_write
  ac_id: FR-015-AC-36
  trace: correct
- test_id: src/kani/generate/scalar.rs::tests::scalar_projection_selects_arithmetic_and_boolean_implication
  ac_id: FR-015-AC-37
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_scalar_harness_asserts_the_native_arithmetic_relation
  ac_id: FR-015-AC-37
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_every_rendered_operation_states_its_own_native_relation
  ac_id: FR-015-AC-37
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic
  ac_id: FR-015-AC-37
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::operation_context_retains_every_field_and_parameter_in_declared_order
  ac_id: FR-015-AC-38
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::operation_context_retains_every_field_and_parameter_in_declared_order
  ac_id: FR-015-AC-39
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::operation_context_executes_exact_state_and_rejects_every_bad_produced_axis
  ac_id: FR-015-AC-39
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::operation_context_requires_and_assumes_same_anchor_preconditions
  ac_id: FR-015-AC-43
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::unsupported_sibling_refusal_names_the_exact_admitted_precondition
  ac_id: FR-015-AC-43
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::v2_clause_census_refuses_invalid_declared_dependencies
  ac_id: FR-015-AC-44
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::v2_clause_census_is_order_independent_and_records_readiness
  ac_id: FR-015-AC-45
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::qsl_v2_precondition_harness_identity_tracks_request_facts
  ac_id: FR-015-AC-47
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_a_byte_ceiling_and_an_unrecognised_lowering_refusal_are_oracle_refused_unchanged
  ac_id: FR-015-AC-50
  trace: correct
- test_id: tests/it/cover_last.rs::tc_025_every_emitted_harness_ends_with_exactly_one_cover
  ac_id: FR-015-AC-53
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::precondition_collection_source_is_admitted
  ac_id: FR-015-AC-53
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::real_kani_collection_precondition_distinguishes_totality_from_satisfiability
  ac_id: FR-015-AC-53
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::tc_023_kani_reads_a_corpus_harness_without_its_cover_as_missing_the_cover_summary
  ac_id: FR-015-AC-55
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::tc_023_graph_corpus_ends_its_harness_with_one_cover_after_its_assertion
  ac_id: FR-015-AC-55
  trace: correct
- test_id: tests/it/cover_last.rs::tc_025_every_emitted_harness_ends_with_exactly_one_cover
  ac_id: FR-015-AC-55
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::tc_023_kani_executes_the_generated_graph_harness
  ac_id: FR-015-AC-57
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::tc_023_kani_falsifies_a_false_graph_harness
  ac_id: FR-015-AC-57
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::tc_023_real_corpus_playback_replays_through_its_qsl_source
  ac_id: FR-015-AC-57
  trace: correct
- test_id: tests/it/cover_last.rs::tc_025_every_emitted_harness_ends_with_exactly_one_cover
  ac_id: FR-015-AC-58
  trace: correct
- test_id: tests/it/cover_last.rs::tc_025_the_cover_inspection_rejects_a_misplaced_duplicated_or_missing_cover
  ac_id: FR-015-AC-58
  trace: correct
- test_id: tests/it/cover_last.rs::tc_025_the_proof_spelling_counter_sees_split_formatted_and_continued_attributes
  ac_id: FR-015-AC-58
  trace: correct
- test_id: tests/it/cover_last.rs::tc_025_the_literal_scan_skips_a_test_item_with_an_array_type
  ac_id: FR-015-AC-58
  trace: correct
- test_id: tests/it/cover_last.rs::tc_025_no_proof_attribute_is_spelled_outside_what_the_inspection_drives
  ac_id: FR-015-AC-58
  trace: correct
- test_id: tests/it/cover_last.rs::tc_025_frame_renderer_owns_every_non_corpus_kani_spelling
  ac_id: FR-015-AC-58
  trace: correct
- test_id: tests/it/kani_generation.rs::kani_generation_selects_an_admitted_v2_scalar_harness
  ac_id: FR-015-AC-58
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_every_state_frame_item_has_a_record_whatever_an_earlier_item_refused
  ac_id: FR-015-AC-59
  trace: correct
- test_id: src/kani/generate/role_parity_tests.rs::state_frame_both_roles_match_direct_generation
  ac_id: FR-015-AC-60
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_a_state_frame_item_with_no_bound_is_requires_bound
  ac_id: FR-015-AC-61
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_a_state_frame_item_outside_the_encoding_has_a_named_reason
  ac_id: FR-015-AC-62
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_a_state_frame_item_the_arm_does_not_render_carries_the_engines_refusal
  ac_id: FR-015-AC-63
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_an_invalid_state_frame_item_rejects_the_request_with_every_record
  ac_id: FR-015-AC-64
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_a_state_frame_request_is_refused_whole_for_the_requests_own_faults
  ac_id: FR-015-AC-65
  trace: correct
- test_id: src/kani/generate/outcome.rs::tests::tc_025_every_state_frame_refusal_maps_to_the_disposition_the_table_gives
  ac_id: FR-015-AC-66
  trace: correct
- test_id: src/kani/generate/outcome.rs::tests::tc_025_the_state_frame_mapping_has_no_wildcard_or_binding_arm
  ac_id: FR-015-AC-66
  trace: correct
- test_id: src/kani/generate/outcome.rs::declaring_node_error_tests::declaring_node_refusals_preserve_distinct_kinds_and_original_field_identity
  ac_id: FR-015-AC-66
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_an_unselected_model_body_never_supplies_a_range
  ac_id: FR-015-AC-66
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_a_non_identifier_graph_field_name_is_a_malformed_clause
  ac_id: FR-015-AC-67
  trace: correct
- test_id: src/kani/generate/role_parity_tests.rs::state_frame_one_role_refused_matches_direct_generation
  ac_id: FR-015-AC-68
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_a_source_model_range_within_jcs_safe_integer_replays_without_override
  ac_id: FR-015-AC-77
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_the_generator_uses_model_fields_of_the_package_qsl_emits
  ac_id: FR-015-AC-77
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_model_field_table_preserves_the_requested_draw_order
  ac_id: FR-015-AC-77
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_an_unselected_model_body_never_supplies_a_range
  ac_id: FR-015-AC-78
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_model_member_types_without_i64_ranges_remain_unranged
  ac_id: FR-015-AC-78
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::selected_model_variant_field_is_unranged_in_generated_frame_identity
  ac_id: FR-015-AC-78
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_the_generator_uses_model_fields_of_the_package_qsl_emits
  ac_id: FR-015-AC-79
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_model_field_table_preserves_the_requested_draw_order
  ac_id: FR-015-AC-79
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_real_kani_proves_the_state_postcondition_and_a_mutated_subject_falsifies_it
  ac_id: FR-015-AC-80
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_real_kani_proves_allowed_and_forbidden_frame_effects_and_a_mutated_frame_falsifies
  ac_id: FR-015-AC-80
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_unranged_fields_have_a_reason_and_no_duplicate_record_names
  ac_id: FR-015-AC-81
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_model_member_types_without_i64_ranges_remain_unranged
  ac_id: FR-015-AC-81
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::selected_model_variant_field_is_unranged_in_generated_frame_identity
  ac_id: FR-015-AC-81
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_a_field_with_no_declared_range_is_decoded_and_not_checked
  ac_id: FR-015-AC-81
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_wide_model_override_preserves_unranged_context_without_a_replay_claim
  ac_id: FR-015-AC-81
  trace: correct
- test_id: src/kani/generate/lower/bounded_kani_profile.rs::tests::tc_023_census_reports_every_construct_including_ones_after_an_early_refusal
  ac_id: FR-015-AC-82
  trace: correct
- test_id: src/kani/generate/lower/bounded_kani_profile.rs::tests::profile_reader_rejects_legacy_inconclusive_before_census_admission
  ac_id: FR-015-AC-82
  trace: correct
- test_id: src/kani/generate/lower/mod.rs::tests::profile_refusals_preserve_original_causes_and_context
  ac_id: FR-015-AC-82
  trace: correct
- test_id: src/kani/generate/lower/mod.rs::tests::supported_profile_keeps_unowned_and_mismatched_dispatch_refusals
  ac_id: FR-015-AC-82
  trace: correct
- test_id: src/kani/generate/lower/bounded_kani_profile.rs::tests::tc_023_missing_construct_is_rejected_with_kani_capability_missing
  ac_id: FR-015-AC-83
  trace: correct
- test_id: src/kani/generate/lower/bounded_kani_profile.rs::tests::empty_or_duplicate_construct_name_refuses_without_a_partial_census
  ac_id: FR-015-AC-83
  trace: correct
- test_id: src/kani/generate/lower/finite_reference_graphs.rs::tests::branching_graph_uses_a_stable_bounded_discovery_order
  ac_id: FR-015-AC-86
  trace: correct
- test_id: src/kani/generate/lower/mod.rs::tests::supported_profile_still_prepares_exact_graph_result
  ac_id: FR-015-AC-86
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::tc_023_pre_run_case_cannot_expose_a_certifiable_kani_outcome
  ac_id: FR-015-AC-91
  trace: correct
- test_id: src/kani/generate/corpus/bounded_kani_corpus.rs::tests::tc_023_case_identity_is_independent_of_emission_order_and_run
  ac_id: FR-015-AC-94
  trace: correct
- test_id: src/kani/generate/corpus/bounded_kani_corpus.rs::tests::tc_023_distinct_requests_get_distinct_names_paths_and_proof_symbols
  ac_id: FR-015-AC-94
  trace: correct
- test_id: src/kani/generate/corpus/bounded_kani_corpus.rs::tests::tc_023_every_graph_request_field_changes_the_identity
  ac_id: FR-015-AC-94
  trace: correct
- test_id: src/kani/generate/corpus/bounded_kani_corpus.rs::tests::tc_023_every_input_and_profile_field_changes_the_identity
  ac_id: FR-015-AC-94
  trace: correct
- test_id: src/kani/generate/corpus/bounded_kani_corpus.rs::tests::tc_023_identity_is_canonical_over_the_input_population_order
  ac_id: FR-015-AC-94
  trace: correct
- test_id: src/kani/generate/collection.rs::tests::unique_capacity_matches_independent_wide_oracle
  ac_id: FR-015-AC-96
  trace: correct
- test_id: src/kani/generate/collection.rs::tests::unique_capacity_includes_the_full_i64_interval
  ac_id: FR-015-AC-96
  trace: correct
- test_id: src/kani/generate/collection.rs::tests::finite_element_domain_refuses_an_empty_integer_range
  ac_id: FR-015-AC-96
  trace: correct
- test_id: src/kani/generate/collection.rs::tests::capacity_refusal_follows_existing_representation_unwind_and_memory_refusals
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::operation_context_retains_every_field_and_parameter_in_declared_order
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::operation_context_executes_exact_state_and_rejects_every_bad_produced_axis
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::operation_context_requires_and_assumes_same_anchor_preconditions
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::operation_context_checks_result_domain_and_the_authored_postcondition
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::unsupported_whole_object_axes_have_typed_refusals_and_no_harness
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::whole_object_precondition_totality_and_playback_keep_all_state_and_parameters
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::unread_collection_bounds_and_full_width_integer_domains_are_not_replaced
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::retained_prestate_order_is_an_authored_obligation_separate_from_collection_domain
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::whole_object_identity_regenerates_and_changes_with_unread_domains
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::boolean_ordered_set_operation_draws_both_values_and_enforces_distinct_slots
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::inherited_fields_retain_the_actual_declaring_node_in_whole_object_identity
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::postcondition_nonvacuity_accepts_intermediate_lengths_selected_by_sibling_preconditions
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::unsupported_sibling_refusal_names_the_exact_admitted_precondition
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::identity_binds_nonquantifier_conjuncts_and_assumed_precondition_meaning
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::boolean_literal_conjuncts_preserve_the_complete_collection_clause
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::admitted_state_clause_collection_preserves_order_and_uniqueness
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::admitted_state_clause_sequence_uses_its_full_symbolic_domain
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::negated_state_clause_quantifier_is_not_an_independent_collection_claim
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::operation_and_constraint_clauses_do_not_get_standalone_collection_proofs
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::qsl_emits_and_ir_admits_the_authored_collection_and_reference_shapes
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::negotiation_renders_the_selected_sequence_claims_deterministically
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::collection_claims_nested_under_not_or_disjunction_are_refused
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::out_of_range_integer_literal_is_rendered_as_a_falsifiable_claim
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::boolean_sequence_uses_symbolic_slots_for_both_boolean_values
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::nonzero_sequence_bounds_are_inclusive_in_the_symbolic_domain
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::collection_precondition_generated_oracle_agrees_with_native_clause_evaluation
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::precondition_collection_source_is_admitted
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::precondition_collection_source_controls_quantifier_element_and_census
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::precondition_collection_source_refusals_emit_no_harness
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::unsupported_set_form_precedes_unique_capacity_refusal
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::nonunique_boolean_sequence_keeps_maximum_above_unique_capacity
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::admitted_full_width_integer_ordered_set_retains_its_domain
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::admitted_unrepresentable_element_endpoint_precedes_unique_capacity
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::feasible_ordered_set_emits_symbolic_ordered_unique_slots
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::boolean_ordered_set_keeps_both_values_and_distinct_active_slots
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::integer_ordered_set_at_distinct_capacity_keeps_its_declared_maximum
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::ordered_set_over_capacity_names_the_exact_capacity_without_a_harness
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::boolean_ordered_set_minimum_above_capacity_is_refused
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::tc_025_real_kani_state_clause_collection_claim_and_mutant
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::tc_025_real_kani_ordered_set_claim_and_mutant_keep_distinct_active_values
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::tc_025_real_kani_ordered_set_uniqueness_changes_the_verdict
  ac_id: FR-015-AC-96
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::unsupported_whole_object_axes_have_typed_refusals_and_no_harness
  ac_id: FR-015-AC-97
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::negated_state_clause_quantifier_is_not_an_independent_collection_claim
  ac_id: FR-015-AC-97
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::operation_and_constraint_clauses_do_not_get_standalone_collection_proofs
  ac_id: FR-015-AC-97
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::unsupported_set_form_precedes_unique_capacity_refusal
  ac_id: FR-015-AC-97
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::unsupported_collection_predicate_is_refused_without_harness
  ac_id: FR-015-AC-97
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::qsl_admitted_collection_without_an_upper_bound_requires_bound_without_harness
  ac_id: FR-015-AC-97
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::unbounded_sequence_of_reference_elements_is_unsupported_not_requires_bound
  ac_id: FR-015-AC-97
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::qsl_rejects_a_missing_upper_bound_with_a_positive_minimum
  ac_id: FR-015-AC-97
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::finite_sequence_of_unbounded_integer_elements_requires_bound_when_qsl_admits_it
  ac_id: FR-015-AC-97
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::bounded_sequences_of_string_uuid_and_timestamp_require_element_bounds
  ac_id: FR-015-AC-97
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::collection_negotiation_accounts_for_each_supported_or_refused_item
  ac_id: FR-015-AC-97
  trace: correct
- test_id: src/kani/generate/spec.rs::tests::collection_constructor_refuses_invalid_domains_and_bindings
  ac_id: FR-015-AC-99
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::operation_context_retains_every_field_and_parameter_in_declared_order
  ac_id: FR-015-AC-99
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::operation_context_executes_exact_state_and_rejects_every_bad_produced_axis
  ac_id: FR-015-AC-99
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::operation_context_requires_and_assumes_same_anchor_preconditions
  ac_id: FR-015-AC-99
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::operation_context_checks_result_domain_and_the_authored_postcondition
  ac_id: FR-015-AC-99
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::whole_object_precondition_totality_and_playback_keep_all_state_and_parameters
  ac_id: FR-015-AC-99
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::retained_prestate_order_is_an_authored_obligation_separate_from_collection_domain
  ac_id: FR-015-AC-99
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::postcondition_nonvacuity_accepts_intermediate_lengths_selected_by_sibling_preconditions
  ac_id: FR-015-AC-99
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::boolean_literal_conjuncts_preserve_the_complete_collection_clause
  ac_id: FR-015-AC-99
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::negotiation_renders_the_selected_sequence_claims_deterministically
  ac_id: FR-015-AC-99
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::out_of_range_integer_literal_is_rendered_as_a_falsifiable_claim
  ac_id: FR-015-AC-99
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::nonzero_sequence_bounds_are_inclusive_in_the_symbolic_domain
  ac_id: FR-015-AC-99
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::collection_precondition_generated_oracle_agrees_with_native_clause_evaluation
  ac_id: FR-015-AC-99
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::collection_totality_playback_binds_the_generated_assertion_in_the_default_lane
  ac_id: FR-015-AC-99
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::precondition_collection_source_is_admitted
  ac_id: FR-015-AC-99
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::precondition_collection_source_controls_quantifier_element_and_census
  ac_id: FR-015-AC-99
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::real_kani_collection_precondition_distinguishes_totality_from_satisfiability
  ac_id: FR-015-AC-99
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::native_collection_replay_evaluates_the_authored_healthy_and_mutated_functions
  ac_id: FR-015-AC-99
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies
  ac_id: FR-015-AC-99
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::tc_025_real_kani_ordered_set_claim_and_mutant_keep_distinct_active_values
  ac_id: FR-015-AC-99
  trace: correct
- test_id: src/kani/generate/collection.rs::tests::every_identity_component_changes_the_rendered_harness_path
  ac_id: FR-015-AC-100
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::whole_object_identity_regenerates_and_changes_with_unread_domains
  ac_id: FR-015-AC-100
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::inherited_fields_retain_the_actual_declaring_node_in_whole_object_identity
  ac_id: FR-015-AC-100
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::identity_binds_nonquantifier_conjuncts_and_assumed_precondition_meaning
  ac_id: FR-015-AC-100
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::negotiation_renders_the_selected_sequence_claims_deterministically
  ac_id: FR-015-AC-100
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::boolean_sequence_uses_symbolic_slots_for_both_boolean_values
  ac_id: FR-015-AC-100
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::nonzero_sequence_bounds_are_inclusive_in_the_symbolic_domain
  ac_id: FR-015-AC-100
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::collection_identity_tracks_domains_claim_subject_and_unwind
  ac_id: FR-015-AC-100
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::collection_run_evidence_preserves_every_ordered_identity_option
  ac_id: FR-015-AC-100
  trace: correct
- test_id: src/kani/generate/collection.rs::tests::capacity_refusal_follows_existing_representation_unwind_and_memory_refusals
  ac_id: FR-015-AC-101
  trace: correct
- test_id: src/kani/generate/collection.rs::tests::domain_representability_failures_keep_the_typed_reason
  ac_id: FR-015-AC-101
  trace: correct
- test_id: src/kani/generate/collection.rs::tests::rendering_refuses_an_unrepresentable_element_domain_without_a_harness
  ac_id: FR-015-AC-101
  trace: correct
- test_id: src/kani/generate/collection.rs::tests::a_declaration_without_listable_fields_or_the_selected_field_is_field_not_admitted
  ac_id: FR-015-AC-101
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::unread_collection_bounds_and_full_width_integer_domains_are_not_replaced
  ac_id: FR-015-AC-101
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::collection_claims_nested_under_not_or_disjunction_are_refused
  ac_id: FR-015-AC-101
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::collection_unwind_must_cover_maximum_length_and_loop_exit
  ac_id: FR-015-AC-101
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::precondition_collection_source_refusals_emit_no_harness
  ac_id: FR-015-AC-101
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::admitted_unrepresentable_element_endpoint_precedes_unique_capacity
  ac_id: FR-015-AC-101
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::collection_domain_larger_than_the_request_memory_ceiling_is_refused_without_harness
  ac_id: FR-015-AC-101
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::canonical_model_wire_rejects_cardinality_above_i64
  ac_id: FR-015-AC-101
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::unsupported_whole_object_axes_have_typed_refusals_and_no_harness
  ac_id: FR-015-AC-102
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::unsupported_sibling_refusal_names_the_exact_admitted_precondition
  ac_id: FR-015-AC-102
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::unsupported_collection_predicate_is_refused_without_harness
  ac_id: FR-015-AC-102
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::qsl_admitted_collection_without_an_upper_bound_requires_bound_without_harness
  ac_id: FR-015-AC-102
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::finite_sequence_of_unbounded_integer_elements_requires_bound_when_qsl_admits_it
  ac_id: FR-015-AC-102
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::collection_negotiation_accounts_for_each_supported_or_refused_item
  ac_id: FR-015-AC-102
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_qsl_emitted_nested_integer_operands_are_bounded_or_require_bounds
  ac_id: FR-015-AC-103
  trace: correct
- test_id: src/oracle/scalar/lower/tests.rs::tc_024_nested_inference_enforces_depth_before_reusing_memoized_nodes
  ac_id: FR-015-AC-104
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_routed_operand_refusals_leave_healthy_sibling_unchanged
  ac_id: FR-015-AC-104
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_qsl_emitted_nested_integer_operands_are_bounded_or_require_bounds
  ac_id: FR-015-AC-104
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_qsl_nested_operand_depth_limit_is_typed_and_keeps_sibling_supported
  ac_id: FR-015-AC-104
  trace: correct
- test_id: tests/it/kani_obligations.rs::admitted_integer_division_is_unrendered_and_emits_no_harness
  ac_id: FR-015-AC-109
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_an_underivable_claim_is_underived_and_carries_the_nodes_identity
  ac_id: FR-015-AC-109
  trace: correct
- test_id: src/kani/generate/corpus/bounded_kani_corpus.rs::tests::tc_023_retired_collection_corpus_leaves_only_the_graph_request
  ac_id: FR-015-AC-110
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::tc_023_proof_graph_schema_rejects_retired_corpus_families
  ac_id: FR-015-AC-110
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::collection_claim_uses_the_v2_symbolic_harness_without_a_finite_corpus_request
  ac_id: FR-015-AC-110
  trace: correct
- test_id: src/kani/generate/spec.rs::tests::constructor_refuses_cover_call_and_statement_order_violations
  ac_id: FR-015-AC-112
  trace: correct
- test_id: src/kani/generate/spec.rs::tests::constructor_checks_names_domains_messages_and_unwind_boundaries
  ac_id: FR-015-AC-112
  trace: correct
- test_id: src/kani/generate/spec.rs::tests::expression_validation_descends_into_matches_arguments_and_patterns
  ac_id: FR-015-AC-112
  trace: correct
- test_id: src/kani/generate/spec.rs::tests::expression_namespace_checks_use_identifier_identity_and_path_position
  ac_id: FR-015-AC-112
  trace: correct
- test_id: src/kani/generate/spec.rs::tests::raw_identifiers_cannot_evade_duplicate_binding_checks
  ac_id: FR-015-AC-112
  trace: correct
- test_id: src/kani/generate/spec.rs::tests::own_input_shadow_accepts_raw_alias_once_and_preserves_source_spelling
  ac_id: FR-015-AC-112
  trace: correct
- test_id: src/kani/generate/render.rs::tests::renderer_emits_only_fixed_structure_and_supplied_data_in_order
  ac_id: FR-015-AC-113
  trace: correct
- test_id: src/kani/generate/render.rs::tests::renderer_emits_only_fixed_structure_and_supplied_data_in_order
  ac_id: FR-015-AC-114
  trace: correct
- test_id: src/kani/generate/render.rs::tests::rendering_is_identical_in_another_process
  ac_id: FR-015-AC-114
  trace: correct
- test_id: src/kani/generate/render.rs::tests::renderer_refuses_size_before_invalid_syntax_with_exact_byte_count
  ac_id: FR-015-AC-115
  trace: correct
- test_id: src/kani/generate/scalar.rs::tests::scalar_port_is_differentially_identical_for_all_operations_and_boundaries
  ac_id: FR-015-AC-115
  trace: correct
- test_id: src/kani/generate/scalar.rs::tests::scalar_port_preserves_resource_and_syntax_refusals
  ac_id: FR-015-AC-115
  trace: correct
- test_id: src/kani/generate/render.rs::tests::renderer_emits_only_fixed_structure_and_supplied_data_in_order
  ac_id: FR-015-AC-116
  trace: correct
- test_id: src/kani/generate/spec.rs::tests::own_input_shadow_accepts_raw_alias_once_and_preserves_source_spelling
  ac_id: FR-015-AC-116
  trace: correct
- test_id: tests/it/cover_last.rs::tc_025_every_emitted_harness_ends_with_exactly_one_cover
  ac_id: FR-015-AC-116
  trace: correct
- test_id: tests/it/kani_render_private.rs::unsatisfiable_assume_is_cover_unsatisfied_under_real_kani
  ac_id: FR-015-AC-117
  trace: correct
- test_id: src/kani/generate/scalar.rs::tests::scalar_projection_selects_arithmetic_and_boolean_implication
  ac_id: FR-015-AC-118
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_boolean_implication_harness_keeps_subject_oracle_and_native_independent
  ac_id: FR-015-AC-118
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_boolean_implication_with_lazy_right_calls_its_oracle_with_an_outcome
  ac_id: FR-015-AC-118
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_boolean_implication_subject_mutation_is_falsified_with_false_left
  ac_id: FR-015-AC-118
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::public_family_and_corpus_results_have_direct_ir_errors_and_unchanged_successes
  ac_id: FR-015-AC-123
  trace: correct
- test_id: src/kani/generate/corpus/bounded_kani_corpus.rs::tests::tc_023_large_dependency_graph_refuses_canonical_projection
  ac_id: FR-015-AC-124
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::public_family_and_corpus_results_have_direct_ir_errors_and_unchanged_successes
  ac_id: FR-015-AC-124
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::direct_graph_errors_preserve_profile_dispatch_and_bound_precedence
  ac_id: FR-015-AC-124
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::direct_corpus_refusals_preserve_earliest_failure_and_identity_transactions
  ac_id: FR-015-AC-124
  trace: correct
- test_id: src/core/native_integer.rs::tests::native_interval_identity_retains_full_i128_endpoints_as_exact_decimal
  ac_id: FR-015-AC-125
  trace: correct
- test_id: src/kani/generate/clause/v2.rs::native_probe_tests::bounded_add_feeding_multiply_uses_the_typed_completed_interval
  ac_id: FR-015-AC-125
  trace: correct
- test_id: src/kani/generate/scalar.rs::native_selection_tests::native_selection_uses_owner_plan_for_all_arithmetic_operations
  ac_id: FR-015-AC-125
  trace: correct
- test_id: src/kani/generate/scalar.rs::native_selection_tests::native_selection_refuses_complete_multiply_population_outside_signed32
  ac_id: FR-015-AC-125
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::native_observer_preserves_native_operands_result_and_admission_stop
  ac_id: FR-015-AC-125
  trace: correct
- test_id: src/oracle/clause/native.rs::tests::completed_prefix_bounds_preserve_the_original_semantic_refusal
  ac_id: FR-015-AC-125
  trace: correct
- test_id: src/oracle/clause/native.rs::tests::a_fitting_semantic_result_or_zero_product_cannot_repair_multiply_capacity
  ac_id: FR-015-AC-125
  trace: correct
- test_id: src/oracle/scalar/render.rs::native_publication_tests::grouped_native_publication_keeps_each_proof_body_and_source_map_aligned
  ac_id: FR-015-AC-125
  trace: correct
- test_id: tests/it/kani_obligations.rs::native_scalar_increment_keeps_original_profile_and_checked_capacity
  ac_id: FR-015-AC-125
  trace: correct
- test_id: tests/it/kani_obligations.rs::native_scalar_generated_proof_body_typechecks
  ac_id: FR-015-AC-125
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::native_v2_precondition_keeps_original_parameter_bounds_and_ordering
  ac_id: FR-015-AC-125
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::native_v2_chained_probe_uses_selected_intermediate_interval
  ac_id: FR-015-AC-125
  trace: correct
- test_id: tests/it/kani_obligations.rs::native_scalar_false_bound_control_keeps_original_draws
  ac_id: FR-015-AC-126
  trace: correct
- test_id: tests/it/kani_obligations.rs::native_original_tc027_increment_verifies
  ac_id: FR-015-AC-126
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::native_v2_postcondition_keeps_guarded_arithmetic_and_independent_expectation
  ac_id: FR-015-AC-126
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::native_v2_postcondition_oracle_typechecks_with_direct_dependencies
  ac_id: FR-015-AC-126
  trace: correct
- test_id: tests/it/kani_scalar_native_private.rs::native_tc025_arithmetic_mutant_falsifies
  ac_id: FR-015-AC-126
  trace: correct
- test_id: tests/it/kani_scalar_native_private.rs::native_original_tc027_false_bound_falsifies
  ac_id: FR-015-AC-126
  trace: correct
- test_id: tests/it/native_v2_helper_playback.rs::native_v2_helper_mutant_playback_decodes_typed_original_witness
  ac_id: FR-015-AC-126
  trace: correct
- test_id: tests/it/kani_scalar_native_meter.rs::routed_native_oracle_preserves_denial_cancellation_counters_and_refusal
  ac_id: FR-015-AC-127
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::operation_and_constraint_clauses_do_not_get_standalone_collection_proofs
  ac_id: FR-015-AC-128
  trace: correct
- test_id: src/kani/run/tool.rs::tests::tc_027_the_launcher_resolves_through_cargo_home_then_path
  ac_id: FR-017-AC-2
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_a_missing_launcher_is_refused_before_anything_runs
  ac_id: FR-017-AC-2
  trace: correct
- test_id: src/kani/classify.rs::synthetic::tc_027_run_classification_never_defaults_to_verified
  ac_id: FR-017-AC-4
  trace: correct
- test_id: src/kani/classify.rs::real_capture::tc_027_the_console_banner_never_decides_the_verdict
  ac_id: FR-017-AC-4
  trace: correct
- test_id: src/kani/classify.rs::real_capture::tc_027_a_success_report_listing_a_failed_check_is_refused_never_verified
  ac_id: FR-017-AC-4
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_a_routed_scalar_harness_run_classifies_like_a_contract_harness
  ac_id: FR-017-AC-4
  trace: correct
- test_id: src/kani/classify.rs::synthetic::tc_027_run_classification_never_defaults_to_verified
  ac_id: FR-017-AC-5
  trace: correct
- test_id: src/kani/classify.rs::synthetic::tc_027_an_exhausted_unwind_bound_is_inconclusive_not_falsified
  ac_id: FR-017-AC-5
  trace: correct
- test_id: src/kani/classify.rs::real_capture::tc_027_real_kani_failure_carries_the_assertion_playback_not_the_cover_one
  ac_id: FR-017-AC-5
  trace: correct
- test_id: src/kani/classify.rs::real_capture::tc_027_real_kani_unwinding_failure_is_inconclusive_not_falsified
  ac_id: FR-017-AC-5
  trace: correct
- test_id: src/kani/output/playback.rs::tests::tc_027_playback_scanning_returns_the_property_block_and_stops_at_an_unterminated_fence
  ac_id: FR-017-AC-5
  trace: correct
- test_id: src/kani/run/execute.rs::tests::tc_027_the_launch_exports_the_report_after_the_harness_options
  ac_id: FR-017-AC-6
  trace: correct
- test_id: tests/it/kani_generation.rs::kani_generation_real_v2_scalar_verifies
  ac_id: FR-017-AC-6
  trace: correct
- test_id: tests/it/kani_generation.rs::kani_generation_real_v2_scalar_verifies
  ac_id: FR-017-AC-7
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified
  ac_id: FR-017-AC-7
  trace: correct
- test_id: tests/it/kani_generation.rs::kani_generation_real_v2_scalar_verifies
  ac_id: FR-017-AC-11
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_a_routed_scalar_harness_run_classifies_like_a_contract_harness
  ac_id: FR-017-AC-11
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified
  ac_id: FR-017-AC-11
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic
  ac_id: FR-017-AC-11
  trace: correct
- test_id: src/kani/classify.rs::real_capture::tc_027_real_kani_success_with_a_satisfied_cover_is_verified
  ac_id: FR-017-AC-12
  trace: correct
- test_id: src/kani/classify.rs::real_capture::tc_027_real_kani_failure_carries_the_assertion_playback_not_the_cover_one
  ac_id: FR-017-AC-12
  trace: correct
- test_id: src/kani/classify.rs::real_capture::tc_027_real_kani_unwinding_failure_is_inconclusive_not_falsified
  ac_id: FR-017-AC-12
  trace: correct
- test_id: src/kani/classify.rs::real_capture::tc_027_real_kani_a_run_with_no_successful_check_is_a_vacuous_proof
  ac_id: FR-017-AC-12
  trace: correct
- test_id: src/kani/classify.rs::real_capture::tc_027_real_kani_partly_satisfied_covers_are_cover_unsatisfied
  ac_id: FR-017-AC-12
  trace: correct
- test_id: src/kani/classify.rs::real_capture::tc_027_real_kani_success_without_a_cover_is_inconclusive
  ac_id: FR-017-AC-12
  trace: correct
- test_id: src/kani/classify.rs::real_capture::tc_027_the_console_banner_never_decides_the_verdict
  ac_id: FR-017-AC-12
  trace: correct
- test_id: src/kani/classify.rs::real_capture::tc_027_an_unnamed_check_category_is_a_property
  ac_id: FR-017-AC-12
  trace: correct
- test_id: src/kani/output/playback.rs::tests::tc_027_playback_scanning_returns_the_property_block_and_stops_at_an_unterminated_fence
  ac_id: FR-017-AC-12
  trace: correct
- test_id: src/kani/run/execute.rs::tests::tc_043_a_launch_stopped_over_a_stream_is_refused_with_its_stable_code
  ac_id: FR-017-AC-14
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_an_over_limit_stream_refuses_a_single_run_and_a_batch_by_its_member_count
  ac_id: FR-017-AC-14
  trace: correct
- test_id: src/kani/run/launch.rs::tests::tc_043_a_capture_over_its_limit_is_refused_and_one_at_its_limit_is_whole
  ac_id: FR-017-AC-14
  trace: correct
- test_id: src/kani/run/launch.rs::tests::tc_043_a_launcher_stream_over_the_limit_is_refused_and_one_at_the_limit_completes
  ac_id: FR-017-AC-14
  trace: correct
- test_id: src/kani/run/launch.rs::tests::tc_043_a_batch_stream_is_bounded_by_the_limit_times_the_member_count
  ac_id: FR-017-AC-14
  trace: correct
- test_id: src/kani/run/launch.rs::tests::tc_043_an_over_limit_run_is_stopped_and_kills_the_launcher_group
  ac_id: FR-017-AC-14
  trace: correct
- test_id: src/kani/run/launch.rs::tests::a_timeout_of_duration_max_never_elapses_and_does_not_panic
  ac_id: FR-017-AC-15
  trace: correct
- test_id: src/kani/run/launch.rs::tests::a_process_orphaned_just_before_the_kill_does_not_block_this_calls_own_return
  ac_id: FR-017-AC-16
  trace: correct
- test_id: src/kani/run/launch.rs::tests::a_capture_thread_told_to_stop_returns_what_is_already_in_the_pipe
  ac_id: FR-017-AC-16
  trace: correct
- test_id: src/kani/run/launch.rs::tests::a_capture_thread_blocked_on_an_open_idle_pipe_stops_when_the_flag_is_set
  ac_id: FR-017-AC-16
  trace: correct
- test_id: src/kani/run/launch.rs::tests::a_capture_thread_stops_reading_when_its_drain_limit_has_passed
  ac_id: FR-017-AC-16
  trace: correct
- test_id: src/kani/run/launch.rs::tests::a_capture_thread_stops_within_the_drain_limit_while_a_straggler_keeps_writing
  ac_id: FR-017-AC-16
  trace: correct
- test_id: src/kani/run/launch.rs::tests::a_run_exceeding_its_budget_kills_a_real_grandchild_not_only_the_direct_child
  ac_id: FR-017-AC-17
  trace: correct
- test_id: src/kani/classify.rs::synthetic::tc_027_an_unreadable_or_missing_report_is_refused_never_inconclusive
  ac_id: FR-017-AC-18
  trace: correct
- test_id: src/kani/classify.rs::real_capture::tc_027_a_report_that_changed_shape_is_refused_not_classified
  ac_id: FR-017-AC-18
  trace: correct
- test_id: src/kani/classify.rs::real_capture::tc_027_a_report_without_exactly_one_harness_is_refused
  ac_id: FR-017-AC-18
  trace: correct
- test_id: src/kani/classify.rs::real_capture::tc_027_a_success_report_listing_a_failed_check_is_refused_never_verified
  ac_id: FR-017-AC-18
  trace: correct
- test_id: src/kani/run/execute.rs::tests::tc_027_execution_reads_only_the_report_its_own_run_exported
  ac_id: FR-017-AC-18
  trace: correct
- test_id: src/kani/classify.rs::real_capture::tc_027_real_kani_the_per_check_view_carries_id_class_location_and_status
  ac_id: FR-017-AC-20
  trace: correct
- test_id: src/kani/classify.rs::real_capture::tc_027_an_unknown_line_is_none_and_a_non_numeric_line_is_refused
  ac_id: FR-017-AC-20
  trace: correct
- test_id: src/kani/output/report.rs::tests::tc_027_a_class_spelled_cover_or_unwind_is_never_other
  ac_id: FR-017-AC-20
  trace: correct
- test_id: src/kani/output/report.rs::tests::tc_027_the_per_check_view_has_one_serialized_wire_shape
  ac_id: FR-017-AC-20
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::unequal_memory_ceilings_plan_separate_groups_with_their_own_selections
  ac_id: FR-017-AC-21
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_compatible_harnesses_start_one_process_with_one_selection_each_in_request_order
  ac_id: FR-017-AC-21
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_harnesses_group_by_option_vector_and_by_request_timeout
  ac_id: FR-017-AC-21
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_the_member_timeout_is_whole_seconds_rounded_up_and_omitted_above_kani_maximum
  ac_id: FR-017-AC-21
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_requests_naming_another_launcher_crate_or_target_directory_are_not_grouped
  ac_id: FR-017-AC-21
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_a_batch_may_run_longer_than_t_when_it_is_inside_n_times_t
  ac_id: FR-017-AC-21
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::a_batch_finishes_between_one_fewer_members_bound_and_its_full_bound
  ac_id: FR-017-AC-21
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_the_outer_bound_is_member_count_times_timeout_and_overflow_never_elapses
  ac_id: FR-017-AC-21
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_a_batch_still_running_at_its_outer_bound_is_refused_as_timed_out
  ac_id: FR-017-AC-21
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_a_duration_max_batch_runs_without_a_member_timeout_and_the_maximum_runs_with_one
  ac_id: FR-017-AC-21
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_a_batch_with_no_report_is_refused_after_a_clean_exit_and_no_verdict_after_a_failed_one
  ac_id: FR-017-AC-21
  trace: correct
- test_id: tests/it/kani_batching.rs::tc_043_real_kani_one_process_runs_n_harnesses_where_n_ran_before
  ac_id: FR-017-AC-21
  trace: correct
- test_id: tests/it/kani_batching.rs::tc_043_real_kani_batch_keeps_each_members_own_result_and_times_out_one_member
  ac_id: FR-017-AC-21
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_compatible_harnesses_start_one_process_with_one_selection_each_in_request_order
  ac_id: FR-017-AC-22
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_members_are_matched_by_path_and_classified_from_their_own_entry
  ac_id: FR-017-AC-22
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_a_member_evidence_carries_its_own_fields_and_the_batch_invocation
  ac_id: FR-017-AC-22
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_a_success_beside_a_non_zero_exit_is_verified_only_when_an_entry_states_failure
  ac_id: FR-017-AC-22
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_a_timed_out_entry_is_timed_out_naming_t_and_a_bare_failure_has_no_counterexample
  ac_id: FR-017-AC-22
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_a_member_failing_two_checks_takes_its_first_block_and_keeps_its_neighbour
  ac_id: FR-017-AC-22
  trace: correct
- test_id: tests/it/kani_batching.rs::tc_043_real_kani_batch_keeps_each_members_own_result_and_times_out_one_member
  ac_id: FR-017-AC-22
  trace: correct
- test_id: src/kani/output/playback.rs::tests::tc_043_a_playback_is_taken_by_the_path_it_is_headed_for
  ac_id: FR-017-AC-23
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_a_report_that_lacks_repeats_or_adds_a_harness_refuses_the_batch
  ac_id: FR-017-AC-23
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_a_member_missing_from_the_crate_refuses_the_batch_and_starts_nothing
  ac_id: FR-017-AC-23
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_a_member_is_given_the_playback_headed_for_its_own_path
  ac_id: FR-017-AC-23
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_a_member_failing_two_checks_takes_its_first_block_and_keeps_its_neighbour
  ac_id: FR-017-AC-23
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_an_unattributable_playback_refuses_the_whole_group
  ac_id: FR-017-AC-23
  trace: correct
- test_id: tests/it/kani_batching.rs::tc_043_real_kani_batch_keeps_each_members_own_result_and_times_out_one_member
  ac_id: FR-017-AC-23
  trace: correct
- test_id: src/kani/run/launch.rs::tests::tc_043_a_launcher_that_exits_on_its_own_has_its_grandchild_killed
  ac_id: FR-017-AC-24
  trace: correct
- test_id: src/kani/run/namespace.rs::tests::completed_monitor_cleanup_kills_an_orphan_and_its_fork_after_the_last_sample
  ac_id: FR-017-AC-24
  trace: correct
- test_id: src/kani/run/execute.rs::tests::tc_043_a_launch_stopped_over_a_stream_is_refused_with_its_stable_code
  ac_id: FR-017-AC-25
  trace: correct
- test_id: src/kani/run/launch.rs::tests::tc_043_a_capture_that_panics_or_whose_poll_or_read_errs_is_unread_not_empty
  ac_id: FR-017-AC-25
  trace: correct
- test_id: src/kani/run/launch.rs::tests::tc_043_an_unread_stream_names_the_stream_in_the_launch_outcome
  ac_id: FR-017-AC-25
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::tc_027_staged_native_observer_compiles_with_locked_kernel_and_charge_log
  ac_id: FR-017-AC-27
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  ac_id: FR-017-AC-27
  trace: correct
- test_id: src/kani/run/namespace.rs::tests::prepared_source_bind_is_read_only_to_the_backend
  ac_id: FR-017-AC-28
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::tc_027_changed_staged_bytes_and_unexpected_files_refuse_snapshot
  ac_id: FR-017-AC-28
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  ac_id: FR-017-AC-28
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_rejects_preflight_mismatches_without_launch
  ac_id: FR-017-AC-28
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::every_retained_staged_path_and_byte_is_checked_without_a_digest
  ac_id: FR-017-AC-29
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::companion_staging_retains_every_original_artifact_and_one_canonical_adapter
  ac_id: FR-017-AC-29
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::native_command_selects_only_the_companion_and_preserves_ordered_inputs
  ac_id: FR-017-AC-29
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::snapshot_rejects_same_bytes_replaced_by_a_symlink
  ac_id: FR-017-AC-29
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  ac_id: FR-017-AC-29
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_rejects_preflight_mismatches_without_launch
  ac_id: FR-017-AC-29
  trace: correct
- test_id: tests/it/kani_scalar_native_private.rs::staged_routed_native_companion_retains_custody_and_dev_only_feature_boundary
  ac_id: FR-017-AC-29
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::tc_027_staged_native_observer_compiles_with_locked_kernel_and_charge_log
  ac_id: FR-017-AC-30
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  ac_id: FR-017-AC-30
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_subtraction_fixture_has_two_bounded_operands
  ac_id: FR-017-AC-31
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders
  ac_id: FR-017-AC-31
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_027_a_silent_launcher_with_no_report_is_no_verdict_with_phase_launcher
  ac_id: FR-017-AC-32
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_027_evidence_of_every_other_outcome_has_no_diagnostics_member
  ac_id: FR-017-AC-32
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_043_over_limit_and_unread_streams_stay_refused_with_no_record
  ac_id: FR-017-AC-32
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_043_the_terminal_value_of_no_verdict_evidence_ignores_its_diagnostics
  ac_id: FR-017-AC-32
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_real_kani_non_compiling_crate_is_no_verdict_with_phase_compile
  ac_id: FR-017-AC-32
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_a_solver_line_outside_the_members_segment_does_not_make_it_solver
  ac_id: FR-017-AC-33
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_parallel_jobs_make_segment_phases_unknown_but_not_compile_or_launcher
  ac_id: FR-017-AC-33
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_a_marker_before_the_excerpt_window_still_decides_the_phase
  ac_id: FR-017-AC-33
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_027_marker_stand_ins_decide_the_failure_phase
  ac_id: FR-017-AC-33
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_ansi_escapes_go_and_controls_and_invalid_bytes_become_replacement_characters
  ac_id: FR-017-AC-34
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_a_stream_at_the_limit_is_whole_and_one_over_keeps_a_line_aligned_suffix
  ac_id: FR-017-AC-34
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_a_final_line_over_the_limit_is_cut_on_a_character_boundary
  ac_id: FR-017-AC-34
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_a_long_line_with_its_line_feed_is_never_an_empty_excerpt
  ac_id: FR-017-AC-34
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_a_line_feed_only_stream_is_kept_and_an_empty_one_is_empty
  ac_id: FR-017-AC-34
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_a_stream_at_the_eight_mebibyte_limit_still_yields_a_bounded_excerpt
  ac_id: FR-017-AC-34
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_every_redaction_input_yields_its_stated_output
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_known_directories_match_whole_components_and_unusable_homes_redact_nothing
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_the_longest_known_directory_wins_and_a_tie_goes_crate_target_home
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_every_listed_path_opener_starts_a_token_and_no_other_character_does
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_every_listed_path_closer_ends_a_token_and_no_other_character_does
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_duration_pid_and_timestamp_rules_respect_their_boundaries
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_the_record_and_its_parts_implement_no_deserialize
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_the_record_serializes_with_the_stated_member_names
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_027_a_runs_streams_are_redacted_against_the_requests_directories
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_027_runs_differing_only_in_redacted_values_serialize_identical_records
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_027_the_same_run_under_two_process_homes_gives_the_same_record
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_043_each_member_takes_the_phase_of_its_own_segment
  ac_id: FR-017-AC-36
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_043_over_limit_and_unread_streams_stay_refused_with_no_record
  ac_id: FR-017-AC-36
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_043_stream_text_never_changes_the_classification
  ac_id: FR-017-AC-36
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_043_group_members_take_their_own_phase_and_share_the_groups_excerpts
  ac_id: FR-017-AC-36
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_043_a_group_report_decides_between_unknown_and_no_record
  ac_id: FR-017-AC-36
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_043_requests_with_different_home_directories_are_not_grouped
  ac_id: FR-017-AC-36
  trace: correct
- test_id: src/kani/run/launch.rs::tests::the_merged_text_is_the_lossy_streams_joined_by_one_line_feed
  ac_id: FR-017-AC-36
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_043_the_terminal_value_of_no_verdict_evidence_ignores_its_diagnostics
  ac_id: FR-017-AC-36
  trace: correct
- test_id: src/kani/output/report.rs::tests::tc_027_the_exit_status_vocabulary_maps_to_its_causes
  ac_id: FR-017-AC-37
  trace: correct
- test_id: src/kani/output/report.rs::tests::tc_027_an_unknown_exit_status_is_redacted_and_cut_to_64_bytes_on_a_character
  ac_id: FR-017-AC-37
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_single_reports_project_resource_causes_and_keep_other_failures
  ac_id: FR-017-AC-37
  trace: correct
- test_id: src/kani/output/report.rs::tests::tc_027_the_exit_status_is_read_only_from_the_one_entry_naming_the_harness
  ac_id: FR-017-AC-38
  trace: correct
- test_id: src/kani/output/report.rs::tests::tc_027_a_number_or_lone_surrogate_exit_status_is_still_malformed
  ac_id: FR-017-AC-38
  trace: correct
- test_id: src/kani/output/report.rs::tests::tc_027_the_cause_implements_no_deserialize
  ac_id: FR-017-AC-38
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::failure_cause::tc_027_the_cause_is_kept_for_no_other_outcome
  ac_id: FR-017-AC-38
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::failure_cause::tc_027_a_malformed_exit_status_still_refuses_the_report
  ac_id: FR-017-AC-38
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_failed_check_reports_keep_both_absent_cause_shapes_failed
  ac_id: FR-017-AC-38
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::failure_cause::tc_043_two_entries_for_a_member_leave_the_timeout_rule_unchanged
  ac_id: FR-017-AC-39
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_five_member_batch_keeps_classifications_and_agrees_on_timeouts
  ac_id: FR-017-AC-39
  trace: correct
- test_id: tests/it/kani_obligations.rs::cbmc_failure_terminal_value_reads_the_retained_cause
  ac_id: FR-017-AC-40
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_real_kani_unknown_cbmc_option_keeps_exit_code_64_as_the_failure_cause
  ac_id: FR-017-AC-40
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_cause_changes_only_terminal_projection_and_retains_evidence
  ac_id: FR-017-AC-40
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::frozen_outer_code_and_class_table_has_exactly_ten_distinct_codes
  ac_id: FR-017-AC-41
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::staged_locks_match_owning_repositories_and_versions
  ac_id: FR-017-AC-42
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::frozen_outer_code_and_class_table_has_exactly_ten_distinct_codes
  ac_id: FR-017-AC-42
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::frozen_evidence_classification_covers_all_outcomes_reasons_and_causes_unchanged
  ac_id: FR-017-AC-42
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::frozen_nested_refusal_table_preserves_nested_output_codes
  ac_id: FR-017-AC-42
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::native_launch_failure_is_an_environment_fault_without_a_receipt
  ac_id: FR-017-AC-42
  trace: correct
- test_id: tests/it/kani_scalar_native_private.rs::bounded_single_run_preserves_cancelled_with_start_signal_and_timeout_without_signal
  ac_id: FR-017-AC-42
  trace: correct
- test_id: tests/it/kani_scalar_native_private.rs::seeded_false_bound_harness_produces_a_paired_completion_refusal
  ac_id: FR-017-AC-42
  trace: correct
- test_id: tests/it/kani_scalar_native_private.rs::bounded_paired_runner_propagates_the_native_helpers_oracle_stop
  ac_id: FR-017-AC-42
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::native_decoder_retains_actual_limits_and_rejects_each_mismatch
  ac_id: FR-017-AC-43
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::regression_bare_native_result_records_are_counted_and_refused
  ac_id: FR-017-AC-43
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::staged_locks_match_owning_repositories_and_versions
  ac_id: FR-017-AC-44
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::frozen_outer_code_and_class_table_has_exactly_ten_distinct_codes
  ac_id: FR-017-AC-44
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::frozen_evidence_classification_covers_all_outcomes_reasons_and_causes_unchanged
  ac_id: FR-017-AC-44
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::frozen_nested_refusal_table_preserves_nested_output_codes
  ac_id: FR-017-AC-44
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  ac_id: FR-017-AC-45
  trace: correct
- test_id: tests/it/kani_scalar_native_private.rs::cleanup_success_and_failure_preserve_real_receipt_after_snapshot_removal
  ac_id: FR-017-AC-45
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::every_retained_staged_path_and_byte_is_checked_without_a_digest
  ac_id: FR-017-AC-46
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::owned_measurement_survives_explicit_cleanup_and_reports_invalid_utf8
  ac_id: FR-017-AC-46
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::regression_non_utf8_lock_is_a_snapshot_io_fault
  ac_id: FR-017-AC-46
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::companion_staging_retains_every_original_artifact_and_one_canonical_adapter
  ac_id: FR-017-AC-46
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  ac_id: FR-017-AC-46
  trace: correct
- test_id: tests/it/kani_scalar_native_private.rs::staged_routed_native_companion_retains_custody_and_dev_only_feature_boundary
  ac_id: FR-017-AC-46
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  ac_id: FR-017-AC-47
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders
  ac_id: FR-017-AC-47
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::every_retained_staged_path_and_byte_is_checked_without_a_digest
  ac_id: FR-017-AC-48
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::owned_measurement_survives_explicit_cleanup_and_reports_invalid_utf8
  ac_id: FR-017-AC-48
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::companion_staging_retains_every_original_artifact_and_one_canonical_adapter
  ac_id: FR-017-AC-48
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  ac_id: FR-017-AC-48
  trace: correct
- test_id: tests/it/kani_scalar_native_private.rs::cleanup_success_and_failure_preserve_real_receipt_after_snapshot_removal
  ac_id: FR-017-AC-48
  trace: correct
- test_id: tests/it/kani_scalar_native_private.rs::staged_routed_native_companion_retains_custody_and_dev_only_feature_boundary
  ac_id: FR-017-AC-48
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::native_decoder_retains_actual_limits_and_rejects_each_mismatch
  ac_id: FR-017-AC-50
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::regression_bare_native_result_records_are_counted_and_refused
  ac_id: FR-017-AC-50
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  ac_id: FR-017-AC-50
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders
  ac_id: FR-017-AC-50
  trace: correct
- test_id: tests/it/kani_scalar_native_private.rs::cleanup_success_and_failure_preserve_real_receipt_after_snapshot_removal
  ac_id: FR-017-AC-50
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_scalar_limits_is_available_at_codegen_root_with_request_type_identity
  ac_id: FR-017-AC-51
  trace: correct
- test_id: src/core/profile.rs::tests::observer_is_an_isolated_dev_only_example_over_the_canonical_adapter
  ac_id: FR-017-AC-52
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::sealed_scalar_launch_selects_the_library_without_changing_proof_options
  ac_id: FR-017-AC-52
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::companion_staging_retains_every_original_artifact_and_one_canonical_adapter
  ac_id: FR-017-AC-52
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::native_command_selects_only_the_companion_and_preserves_ordered_inputs
  ac_id: FR-017-AC-52
  trace: correct
- test_id: tests/it/kani_scalar_native_private.rs::staged_routed_native_companion_retains_custody_and_dev_only_feature_boundary
  ac_id: FR-017-AC-52
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::companion_staging_retains_every_original_artifact_and_one_canonical_adapter
  ac_id: FR-017-AC-53
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::snapshot_rejects_same_bytes_replaced_by_a_symlink
  ac_id: FR-017-AC-53
  trace: correct
- test_id: tests/it/kani_scalar_native_private.rs::staged_routed_native_companion_retains_custody_and_dev_only_feature_boundary
  ac_id: FR-017-AC-53
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::omitted_namespace_confirmation_refuses_public_evidence
  ac_id: FR-017-AC-55
  trace: correct
- test_id: src/kani/run/namespace.rs::tests::empty_namespace_confirmation_refuses_before_later_real_confirmation
  ac_id: FR-017-AC-55
  trace: correct
- test_id: src/kani/classify.rs::synthetic::a_report_with_no_successful_check_is_inconclusive_not_verified_even_with_every_cover_satisfied
  ac_id: FR-017-AC-61
  trace: correct
- test_id: src/kani/classify.rs::real_capture::tc_027_real_kani_a_run_with_no_successful_check_is_a_vacuous_proof
  ac_id: FR-017-AC-61
  trace: correct
- test_id: src/kani/run/execute.rs::tests::collection_totality_binding_rejects_foreign_or_missing_checks
  ac_id: FR-017-AC-61
  trace: correct
- test_id: src/kani/run/execute.rs::tests::a_precondition_harness_with_no_checks_is_decided_by_its_cover_not_the_zero_checks_rule
  ac_id: FR-017-AC-61
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::collection_totality_controls_single_readonly_and_batch_evidence
  ac_id: FR-017-AC-61
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::real_kani_collection_precondition_distinguishes_totality_from_satisfiability
  ac_id: FR-017-AC-61
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::tc_027_staged_native_observer_compiles_with_locked_kernel_and_charge_log
  ac_id: FR-017-AC-62
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::staged_locks_match_owning_repositories_and_versions
  ac_id: FR-017-AC-62
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_a_routed_scalar_harness_run_classifies_like_a_contract_harness
  ac_id: FR-017-CON-1
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_unbounded_non_finite_and_blocked_items_are_refused_without_harnesses
  ac_id: FR-017-CON-2
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_every_item_is_accounted_before_any_harness_is_exposed
  ac_id: FR-017-CON-2
  trace: correct
- test_id: src/kani/generate/corpus/bounded_kani_corpus.rs::tests::either_ceiling_changes_the_canonical_corpus_identity
  ac_id: FR-028-AC-1
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::tc_023_proof_graph_artifact_validates_against_its_published_schema
  ac_id: FR-028-AC-1
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::tc_023_proof_graph_schema_rejects_retired_corpus_families
  ac_id: FR-028-AC-1
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::tc_023_proof_graph_with_a_declared_dependency_validates_against_its_published_schema
  ac_id: FR-028-AC-1
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::both_state_obligation_identities_change_with_either_required_ceiling
  ac_id: FR-028-AC-1
  trace: correct
- test_id: tests/it/routed_generation.rs::routed_scalar_identity_records_both_required_ceilings
  ac_id: FR-028-AC-1
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::identity_ceilings_govern_execution_and_successful_evidence_records_observed_memory
  ac_id: FR-028-AC-2
  trace: correct
- test_id: src/kani/run/launch.rs::tests::zero_wall_ceiling_cannot_accept_an_already_exited_launcher
  ac_id: FR-028-AC-2
  trace: correct
- test_id: src/kani/run/launch.rs::tests::expired_dispatch_deadline_starts_no_helper
  ac_id: FR-028-AC-2
  trace: correct
- test_id: src/kani/run/namespace.rs::tests::startup_cap_refuses_without_claiming_the_identity_wall_ceiling_elapsed
  ac_id: FR-028-AC-2
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::child_memory_overage_is_inconclusive_and_kills_the_entire_backend_tree
  ac_id: FR-028-AC-3
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::identity_ceilings_govern_execution_and_successful_evidence_records_observed_memory
  ac_id: FR-028-AC-4
  trace: correct
- test_id: src/kani/run/harness.rs::tests::unranged_state_draws_record_the_full_i64_domain
  ac_id: FR-028-AC-4
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_the_member_timeout_is_whole_seconds_rounded_up_and_omitted_above_kani_maximum
  ac_id: FR-028-AC-12
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_a_batch_may_run_longer_than_t_when_it_is_inside_n_times_t
  ac_id: FR-028-AC-12
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::a_batch_finishes_between_one_fewer_members_bound_and_its_full_bound
  ac_id: FR-028-AC-12
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_the_outer_bound_is_member_count_times_timeout_and_overflow_never_elapses
  ac_id: FR-028-AC-12
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_a_batch_still_running_at_its_outer_bound_is_refused_as_timed_out
  ac_id: FR-028-AC-12
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_a_duration_max_batch_runs_without_a_member_timeout_and_the_maximum_runs_with_one
  ac_id: FR-028-AC-12
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_a_timed_out_entry_is_timed_out_naming_t_and_a_bare_failure_has_no_counterexample
  ac_id: FR-028-AC-12
  trace: correct
- test_id: tests/it/kani_batching.rs::tc_043_real_kani_batch_keeps_each_members_own_result_and_times_out_one_member
  ac_id: FR-028-AC-12
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::child_memory_overage_is_inconclusive_and_kills_the_entire_backend_tree
  ac_id: FR-028-AC-21
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::batch_memory_overage_refuses_every_member_without_classifying_a_partial_report
  ac_id: FR-028-AC-21
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::identity_ceilings_govern_execution_and_successful_evidence_records_observed_memory
  ac_id: FR-028-AC-21
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::unequal_identity_memory_ceilings_run_in_separate_backend_processes
  ac_id: FR-028-AC-21
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::unequal_memory_ceilings_plan_separate_groups_with_their_own_selections
  ac_id: FR-028-AC-21
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::unsupported_platform_refuses_single_and_batch_before_backend_dispatch
  ac_id: FR-028-AC-21
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::tc_043_an_over_limit_stream_refuses_a_single_run_and_a_batch_by_its_member_count
  ac_id: FR-028-AC-21
  trace: correct
- test_id: src/kani/run/launch.rs::tests::unavailable_tree_memory_observation_refuses_before_spawn
  ac_id: FR-028-AC-21
  trace: correct
- test_id: src/kani/run/launch.rs::tests::observation_failure_after_spawn_refuses_a_valid_report_and_stops_the_run
  ac_id: FR-028-AC-21
  trace: correct
- test_id: src/kani/run/memory.rs::tests::released_sighand_signed_group_preserves_owned_rss_and_identity_checks
  ac_id: FR-028-AC-21
  trace: correct
- test_id: src/kani/run/memory.rs::tests::unavailable_children_observation_refuses_and_only_disappeared_tasks_are_skipped
  ac_id: FR-028-AC-21
  trace: correct
- test_id: src/kani/run/memory.rs::tests::resident_memory_of_a_reused_pid_is_excluded_from_the_backend_sample
  ac_id: FR-028-AC-21
  trace: correct
- test_id: src/kani/run/memory.rs::tests::released_address_space_is_observed_before_zombie_status_but_missing_rss_is_refused
  ac_id: FR-028-AC-21
  trace: correct
- test_id: src/kani/run/memory.rs::tests::available_observer_does_not_invent_a_peak_before_observing_a_tree
  ac_id: FR-028-AC-21
  trace: correct
- test_id: src/kani/run/memory.rs::tests::missing_task_ancestry_refuses_a_zombie_leader_with_live_workers
  ac_id: FR-028-AC-21
  trace: correct
- test_id: src/kani/run/memory.rs::tests::released_leader_mm_uses_live_worker_rss_or_refuses_observation
  ac_id: FR-028-AC-21
  trace: correct
- test_id: src/kani/run/memory.rs::tests::vanished_process_during_worker_listing_is_skipped_but_live_missing_task_is_refused
  ac_id: FR-028-AC-21
  trace: correct
- test_id: src/kani/run/namespace.rs::tests::startup_cap_refuses_without_claiming_the_identity_wall_ceiling_elapsed
  ac_id: FR-028-AC-21
  trace: correct
- test_id: src/kani/run/namespace.rs::tests::gated_startup_abort_kills_init_before_gate_eof_and_never_dispatches_backend
  ac_id: FR-028-AC-21
  trace: correct
- test_id: src/kani/run/namespace.rs::tests::completed_monitor_cleanup_kills_an_orphan_and_its_fork_after_the_last_sample
  ac_id: FR-028-AC-21
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_verified_proves_its_checks_and_a_reproduced_falsification_refutes
  ac_id: FR-029-AC-1
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_a_vacuous_proof_and_an_unsatisfied_cover_are_proved_with_zero_checks
  ac_id: FR-029-AC-2
  trace: correct
- test_id: tests/it/terminal_map.rs::observed_memory_exhaustion_maps_to_incomplete_resource_exhausted
  ac_id: FR-029-AC-3
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_a_run_with_no_verdict_is_failed
  ac_id: FR-029-AC-4
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_a_missing_counterexample_or_cover_summary_is_failed
  ac_id: FR-029-AC-5
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_no_outcome_maps_to_tested
  ac_id: FR-029-AC-6
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_a_replay_disagreement_carries_its_cause
  ac_id: FR-029-AC-8
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_a_replay_refusal_carries_its_code
  ac_id: FR-029-AC-9
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_a_fault_in_any_replay_wrapper_is_failed
  ac_id: FR-029-AC-10
  trace: correct
- test_id: tests/it/kani_collection_operation.rs::whole_object_precondition_totality_and_playback_keep_all_state_and_parameters
  ac_id: FR-029-AC-11
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::ordinary_v2_clause_cannot_settle_as_a_collection_totality_failure
  ac_id: FR-029-AC-11
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::collection_totality_playback_binds_the_generated_assertion_in_the_default_lane
  ac_id: FR-029-AC-11
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_domains.rs::real_kani_collection_precondition_distinguishes_totality_from_satisfiability
  ac_id: FR-029-AC-11
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_each_failure_this_repository_raises_is_failed
  ac_id: FR-029-AC-11
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_only_a_reproduced_replay_refutes
  ac_id: FR-029-AC-12
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_a_setup_refusal_after_a_refutation_is_replay_refused_never_declined
  ac_id: FR-029-AC-13
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_a_repeated_dependency_identity_is_replay_refused_with_invalid_package
  ac_id: FR-029-AC-14
  trace: correct
- test_id: tests/it/terminal_map.rs::direct_cancelled_run_maps_exact_source_and_rejects_settlement
  ac_id: FR-029-AC-15
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_a_settlement_accompanies_a_falsified_outcome_only
  ac_id: FR-029-AC-15
  trace: correct
- test_id: tests/it/terminal_map.rs::cause_matrix_preserves_literal_terminal_rows_and_pair_refusals
  ac_id: FR-029-AC-15
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_the_state_clause_replay_reads_as_fr029_ac16
  ac_id: FR-029-AC-16
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_040_out_of_range_debit_post_state_is_exact_and_refuted
  ac_id: FR-029-AC-18
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_040_real_kani_out_of_range_debit_replays_as_refuted
  ac_id: FR-029-AC-18
  trace: correct
- test_id: tests/it/composite_parity_converter.rs::tc_041_f1_and_f2_preserve_precedence_and_native_causes
  ac_id: FR-029-AC-20
  trace: correct
- test_id: tests/it/composite_parity_converter.rs::tc_041_f3_to_f6_keep_operand_and_limit_stages_distinct
  ac_id: FR-029-AC-20
  trace: correct
- test_id: tests/it/composite_parity_converter.rs::tc_041_f7_compares_verdict_and_pair_count_then_preserves_agreement
  ac_id: FR-029-AC-20
  trace: correct
- test_id: tests/it/composite_parity_converter.rs::tc_041_bound_prepare_refusal_precedes_disagreed
  ac_id: FR-029-AC-28
  trace: correct
- test_id: tests/it/terminal_map.rs::direct_cancelled_run_maps_exact_source_and_rejects_settlement
  ac_id: FR-029-AC-29
  trace: correct
- test_id: tests/it/kani_obligations.rs::cbmc_failure_terminal_value_reads_the_retained_cause
  ac_id: FR-029-AC-32
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_single_reports_project_resource_causes_and_keep_other_failures
  ac_id: FR-029-AC-32
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_failed_check_reports_keep_both_absent_cause_shapes_failed
  ac_id: FR-029-AC-32
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_cause_changes_only_terminal_projection_and_retains_evidence
  ac_id: FR-029-AC-32
  trace: correct
- test_id: tests/it/terminal_map.rs::cause_matrix_preserves_literal_terminal_rows_and_pair_refusals
  ac_id: FR-029-AC-32
  trace: correct
- test_id: tests/it/kani_obligations.rs::cbmc_failure_terminal_value_reads_the_retained_cause
  ac_id: FR-029-AC-33
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_single_reports_project_resource_causes_and_keep_other_failures
  ac_id: FR-029-AC-33
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_five_member_batch_keeps_classifications_and_agrees_on_timeouts
  ac_id: FR-029-AC-33
  trace: correct
- test_id: tests/it/terminal_map.rs::cause_matrix_preserves_literal_terminal_rows_and_pair_refusals
  ac_id: FR-029-AC-33
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_single_reports_project_resource_causes_and_keep_other_failures
  ac_id: FR-029-AC-34
  trace: correct
- test_id: tests/it/terminal_map.rs::cause_matrix_preserves_literal_terminal_rows_and_pair_refusals
  ac_id: FR-029-AC-34
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_cause_changes_only_terminal_projection_and_retains_evidence
  ac_id: FR-029-AC-35
  trace: correct
- test_id: tests/it/terminal_map.rs::cause_matrix_preserves_literal_terminal_rows_and_pair_refusals
  ac_id: FR-029-AC-35
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_041_every_expressible_pair_maps_to_one_value
  ac_id: FR-030-AC-1
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_041_a_proof_carries_its_check_count_and_a_reproduced_counterexample_refutes
  ac_id: FR-030-AC-1
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_041_a_refusal_before_the_run_is_declined_with_its_kind_and_ir_code
  ac_id: FR-030-AC-2
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_041_a_limit_or_a_cancellation_is_incomplete_with_its_cause
  ac_id: FR-030-AC-3
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_041_a_proof_carries_its_check_count_and_a_reproduced_counterexample_refutes
  ac_id: FR-030-AC-4
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_041_a_vacuous_proof_is_proved_with_zero_checks_and_other_inconclusive_is_refused
  ac_id: FR-030-AC-5
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_041_no_outcome_maps_to_tested
  ac_id: FR-030-AC-6
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_041_the_map_is_one_match_over_the_pair_with_no_wildcard_arm
  ac_id: FR-030-AC-7
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_041_an_absent_solver_or_backend_is_unsupported_with_its_cause
  ac_id: FR-030-AC-8
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_041_a_counterexample_disagreement_or_refusal_is_inconclusive_with_its_cause
  ac_id: FR-030-AC-9
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_041_a_counterexample_with_a_fault_or_a_cg_defect_is_failed
  ac_id: FR-030-AC-10
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_041_only_a_reproduced_replay_refutes_a_counterexample
  ac_id: FR-030-AC-11
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_041_a_setup_refusal_after_a_counterexample_is_replay_refused_never_declined
  ac_id: FR-030-AC-12
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_041_a_repeated_dependency_identity_after_a_counterexample_is_replay_refused
  ac_id: FR-030-AC-13
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_041_a_settlement_accompanies_a_counterexample_only
  ac_id: FR-030-AC-14
  trace: correct
- test_id: tests/it/composite_parity_converter.rs::tc_041_f1_and_f2_preserve_precedence_and_native_causes
  ac_id: FR-030-AC-15
  trace: correct
- test_id: tests/it/composite_parity_converter.rs::tc_041_f3_to_f6_keep_operand_and_limit_stages_distinct
  ac_id: FR-030-AC-16
  trace: correct
- test_id: tests/it/composite_parity_converter.rs::tc_041_f7_compares_verdict_and_pair_count_then_preserves_agreement
  ac_id: FR-030-AC-17
  trace: correct
- test_id: tests/it/composite_parity_converter.rs::tc_041_bound_prepare_refusal_precedes_disagreed
  ac_id: FR-030-AC-17
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_041_a_proof_carries_its_check_count_and_a_reproduced_counterexample_refutes
  ac_id: FR-030-AC-18
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_041_a_vacuous_proof_is_proved_with_zero_checks_and_other_inconclusive_is_refused
  ac_id: FR-030-AC-18
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::cancellation_before_spawn_has_no_backend_or_report
  ac_id: FR-040-AC-1
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::live_cancellation_preserves_first_event_through_ir_and_qsl
  ac_id: FR-040-AC-1
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::live_cancellation_preserves_first_event_through_ir_and_qsl
  ac_id: FR-040-AC-2
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::child_memory_overage_is_inconclusive_and_kills_the_entire_backend_tree
  ac_id: FR-040-AC-3
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::output_over_limit_is_not_relabelled_as_cancellation
  ac_id: FR-040-AC-3
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::controlled_caller_order_distinguishes_cancelled_and_completed_batch
  ac_id: FR-040-AC-3
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::completed_and_elapsed_runs_do_not_acquire_late_cancellation
  ac_id: FR-040-AC-3
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::child_memory_overage_is_inconclusive_and_kills_the_entire_backend_tree
  ac_id: FR-040-AC-4
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::measured_memory_overage_competing_with_cancellation_keeps_resource_outcome
  ac_id: FR-040-AC-4
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::output_over_limit_is_not_relabelled_as_cancellation
  ac_id: FR-040-AC-4
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::memory_observation_failure_competing_with_cancellation_refuses_public_evidence
  ac_id: FR-040-AC-4
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::unread_capture_competing_with_cancellation_refuses_public_evidence
  ac_id: FR-040-AC-4
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::cleanup_nonconfirmation_competing_with_cancellation_refuses_public_evidence
  ac_id: FR-040-AC-4
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::completed_and_elapsed_runs_do_not_acquire_late_cancellation
  ac_id: FR-040-AC-4
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::cancelled_batch_retains_first_source_and_independent_group
  ac_id: FR-040-AC-5
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::controlled_caller_order_distinguishes_cancelled_and_completed_batch
  ac_id: FR-040-AC-5
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::distinct_cancellation_handles_have_distinct_launchers
  ac_id: FR-040-AC-5
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::cancellation_before_spawn_has_no_backend_or_report
  ac_id: FR-040-AC-6
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::live_cancellation_preserves_first_event_through_ir_and_qsl
  ac_id: FR-040-AC-6
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::cancellation_before_spawn_has_no_backend_or_report
  ac_id: FR-040-AC-7
  trace: correct
- test_id: tests/it/terminal_map.rs::direct_cancelled_run_maps_exact_source_and_rejects_settlement
  ac_id: FR-040-AC-7
  trace: correct
- test_id: tests/it/constrained_method.rs::authentic_method_selection_is_exact_and_deterministic
  ac_id: FR-048-AC-1
  trace: correct
- test_id: tests/it/constrained_method.rs::complete_census_retains_unread_fields_and_uncalled_operations
  ac_id: FR-048-AC-1
  trace: correct
- test_id: tests/it/constrained_method.rs::boolean_and_partial_conditions_use_the_owner_oracle_and_unbounded_domains_refuse
  ac_id: FR-048-AC-1
  trace: correct
- test_id: tests/it/constrained_method.rs::equivalent_package_instances_cannot_mix_handles_and_census_refusal_retains_owner_payload
  ac_id: FR-048-AC-1
  trace: correct
- test_id: tests/it/constrained_method.rs::structurally_identical_constraint_occurrences_require_an_explicit_selection
  ac_id: FR-048-AC-1
  trace: correct
- test_id: tests/it/constrained_method.rs::authentic_method_selection_is_exact_and_deterministic
  ac_id: FR-048-AC-2
  trace: correct
- test_id: tests/it/constrained_method.rs::equivalent_package_instances_cannot_mix_handles_and_census_refusal_retains_owner_payload
  ac_id: FR-048-AC-2
  trace: correct
- test_id: tests/it/constrained_method.rs::same_named_general_clause_bodies_and_preconditions_cannot_reuse_a_method
  ac_id: FR-048-AC-2
  trace: correct
- test_id: tests/it/constrained_method.rs::installed_small_method_controls_retain_boundary_safety_and_no_boundary_success
  ac_id: FR-048-AC-3
  trace: correct
- test_id: tests/it/constrained_method.rs::installed_small_method_controls_retain_boundary_safety_and_no_boundary_success
  ac_id: FR-048-AC-4
  trace: correct
- test_id: tests/it/cover_last.rs::constrained_method_covers_have_distinct_purposes_and_the_boundary_is_last
  ac_id: FR-048-AC-4
  trace: correct
- test_id: tests/it/constrained_method.rs::installed_small_method_controls_retain_boundary_safety_and_no_boundary_success
  ac_id: FR-048-AC-5
  trace: correct
- test_id: tests/it/cover_last.rs::constrained_method_covers_have_distinct_purposes_and_the_boundary_is_last
  ac_id: FR-048-AC-5
  trace: correct
- test_id: src/kani/terminal/constrained/tests.rs::conversion_preserves_original_causes_and_rejects_malformed_or_cross_method_evidence
  ac_id: FR-048-AC-6
  trace: correct
- test_id: tests/it/constrained_method.rs::boolean_and_partial_conditions_use_the_owner_oracle_and_unbounded_domains_refuse
  ac_id: FR-048-AC-6
  trace: correct
- test_id: src/kani/terminal/constrained/tests.rs::conversion_preserves_original_causes_and_rejects_malformed_or_cross_method_evidence
  ac_id: FR-048-AC-7
  trace: correct
- test_id: src/kani/terminal/constrained/tests.rs::conversion_preserves_original_causes_and_rejects_malformed_or_cross_method_evidence
  ac_id: FR-048-AC-8
  trace: correct
- test_id: tests/it/constrained_method.rs::installed_small_method_controls_retain_boundary_safety_and_no_boundary_success
  ac_id: FR-048-AC-8
  trace: correct
- test_id: tests/it/constrained_method.rs::equivalent_package_instances_cannot_mix_handles_and_census_refusal_retains_owner_payload
  ac_id: FR-048-AC-8
  trace: correct
- test_id: tests/it/constrained_method.rs::same_named_general_clause_bodies_and_preconditions_cannot_reuse_a_method
  ac_id: FR-048-AC-8
  trace: correct
- test_id: tests/it/constrained_method.rs::ordinary_execution_cannot_publish_a_constrained_success
  ac_id: FR-048-AC-9
  trace: correct
- test_id: tests/it/constrained_method.rs::installed_small_method_controls_retain_boundary_safety_and_no_boundary_success
  ac_id: FR-048-AC-9
  trace: correct
- test_id: tests/it/constrained_method.rs::qsl_owner_evaluates_the_complete_small_state_domain
  ac_id: FR-048-AC-10
  trace: correct
- test_id: tests/it/constrained_method.rs::installed_small_method_controls_retain_boundary_safety_and_no_boundary_success
  ac_id: FR-048-AC-10
  trace: correct
- test_id: tests/it/constrained_method.rs::complete_census_retains_unread_fields_and_uncalled_operations
  ac_id: FR-048-AC-11
  trace: correct
- test_id: tests/it/constrained_method.rs::boolean_and_partial_conditions_use_the_owner_oracle_and_unbounded_domains_refuse
  ac_id: FR-048-AC-11
  trace: correct
- test_id: tests/it/constrained_method.rs::a_standalone_collection_constraint_remains_unsupported_without_a_harness
  ac_id: FR-048-AC-11
  trace: correct
- test_id: tests/it/kani_gate.rs::rust_gate_rejects_a_kani_reaching_path_without_silently_skipping
  ac_id: NFR-006-AC-1
  trace: correct
- test_id: tests/it/kani_gate.rs::scope_classifies_changed_paths_and_both_rename_directions
  ac_id: NFR-006-AC-1
  trace: correct
- test_id: tests/it/kani_gate.rs::scope_classifies_changed_paths_and_both_rename_directions
  ac_id: NFR-006-AC-2
  trace: correct
- test_id: tests/it/kani_gate.rs::scope_classifies_changed_paths_and_both_rename_directions
  ac_id: NFR-006-AC-3
  trace: correct
- test_id: tests/it/kani_gate.rs::make_targets_share_one_real_lane_and_ci_excludes_it
  ac_id: NFR-006-AC-4
  trace: correct
- test_id: tests/it/kani_generation.rs::kani_generation_selects_an_admitted_v2_scalar_harness
  ac_id: NFR-006-AC-4
  trace: correct
- test_id: tests/it/kani_generation.rs::kani_generation_real_v2_scalar_verifies
  ac_id: NFR-006-AC-4
  trace: correct
- test_id: tests/it/kani_gate.rs::gate_runs_once_under_lock_and_refuses_missing_tests_or_failures
  ac_id: NFR-006-AC-5
  trace: correct
- test_id: tests/it/kani_gate.rs::gate_waits_for_the_existing_lock_holder
  ac_id: NFR-006-AC-5
  trace: correct
- test_id: tests/it/kani_gate.rs::rust_gate_refuses_an_empty_kani_test_list
  ac_id: NFR-006-AC-6
  trace: correct
- test_id: tests/it/kani_gate.rs::gate_runs_once_under_lock_and_refuses_missing_tests_or_failures
  ac_id: NFR-006-AC-6
  trace: correct
- test_id: tests/it/kani_gate.rs::gate_runs_once_under_lock_and_refuses_missing_tests_or_failures
  ac_id: NFR-006-AC-7
  trace: correct
- test_id: tests/it/kani_gate.rs::gate_refuses_missing_launcher_and_dirty_tree
  ac_id: NFR-006-AC-8
  trace: correct
- test_id: tests/it/kani_gate.rs::gate_refuses_missing_launcher_and_dirty_tree
  ac_id: NFR-006-AC-9
  trace: correct
- test_id: tests/it/kani_gate.rs::gate_refuses_missing_launcher_and_dirty_tree
  ac_id: NFR-006-AC-10
  trace: correct
- test_id: tests/it/kani_gate.rs::ignored_lane_selection_and_transitive_include_targets_match_scope
  ac_id: NFR-006-AC-11
  trace: correct
- test_id: tests/it/kani_gate.rs::make_targets_share_one_real_lane_and_ci_excludes_it
  ac_id: NFR-006-AC-12
  trace: correct
- test_id: tests/it/kani_gate.rs::ignored_lane_selection_and_transitive_include_targets_match_scope
  ac_id: NFR-006-AC-18
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_every_scalar_family_generates_one_oracle_calling_its_runtime_operator
  ac_id: FR-014-AC-1
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_refused_items_are_typed_emit_no_code_and_leave_siblings_unchanged
  ac_id: FR-014-AC-1
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_every_exact_scalar_refusal_variant_is_matched_exhaustively
  ac_id: FR-014-AC-1
  trace: correct
- test_id: src/kani/generate/collection.rs::tests::a_quantity_conversion_predicate_is_an_unsupported_shape
  ac_id: FR-014-AC-2
  trace: correct
- test_id: src/kani/generate/frame.rs::tests::a_quantity_conversion_is_not_a_state_comparison
  ac_id: FR-014-AC-2
  trace: correct
- test_id: src/kani/generate/scalar.rs::tests::a_quantity_conversion_has_no_scalar_harness_renderer
  ac_id: FR-014-AC-2
  trace: correct
- test_id: src/replay/composite_builder.rs::tests::a_quantity_conversion_is_an_unsupported_node_operation
  ac_id: FR-014-AC-2
  trace: correct
- test_id: tests/it/capability_settlement.rs::a_selected_quantity_conversion_settles_as_operation_not_rendered
  ac_id: FR-014-AC-2
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_text_admission_corpus_is_refused_by_ir_today
  ac_id: FR-014-AC-2
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_every_scalar_family_generates_one_oracle_calling_its_runtime_operator
  ac_id: FR-014-AC-2
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_function_generator_refuses_a_function_whose_body_is_a_quantity_conversion
  ac_id: FR-014-AC-2
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_equality_generator_refuses_a_quantity_conversion_node_before_any_artifact
  ac_id: FR-014-AC-2
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_function_schedules_refuse_quantity_bodies_per_function
  ac_id: FR-014-AC-2
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_function_generator_refuses_a_call_passing_a_quantity_conversion_as_an_argument
  ac_id: FR-014-AC-2
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_function_schedule_item_with_a_conversion_argument_is_refused
  ac_id: FR-014-AC-2
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_equality_over_a_conversion_operand_is_refused_as_an_unsupported_quantity
  ac_id: FR-014-AC-2
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::unit_function_signatures_refuse_per_function_and_preserve_supported_siblings
  ac_id: FR-014-AC-2
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_a_quantity_conversion_claim_is_unrendered_with_no_harness
  ac_id: FR-014-AC-2
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_quantity_conversion_has_an_oracle_and_no_harness
  ac_id: FR-014-AC-2
  trace: correct
- test_id: src/oracle/scalar/lower/tests.rs::hybrid_family_is_unsupported_and_preserves_node_identity
  ac_id: FR-014-AC-3
  trace: correct
- test_id: src/oracle/scalar/lower/tests.rs::tc_024_invalid_and_incomplete_bodies_are_typed_refusals
  ac_id: FR-014-AC-3
  trace: correct
- test_id: src/oracle/scalar/lower/tests.rs::tc_024_catalogued_operation_identity_refuses_a_node_whose_operation_has_no_identity
  ac_id: FR-014-AC-3
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_refused_items_are_typed_emit_no_code_and_leave_siblings_unchanged
  ac_id: FR-014-AC-3
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_lowering_work_exhaustion_is_a_typed_refusal
  ac_id: FR-014-AC-3
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_every_exact_scalar_refusal_variant_is_matched_exhaustively
  ac_id: FR-014-AC-3
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_literal_operands_are_classified_by_value_kind_and_constants_stop_typed
  ac_id: FR-014-AC-3
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_corpus_crate_artifacts_are_complete_and_self_consistent
  ac_id: FR-014-AC-4
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_generation_is_byte_deterministic_across_runs_and_request_orders
  ac_id: FR-014-AC-4
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_claim_map_entries_ascend_by_structural_node_key
  ac_id: FR-014-AC-4
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::tc_024_tc192_division_and_modulus_oracles_agree
  ac_id: FR-014-AC-6
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::tc_024_tc185_decimal_oracles_agree
  ac_id: FR-014-AC-6
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::tc_024_tc186_text_and_enum_oracles_agree
  ac_id: FR-014-AC-6
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::tc_024_tc187_quantity_oracles_agree
  ac_id: FR-014-AC-6
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::tc_024_tc187_u12_compound_unit_products_agree
  ac_id: FR-014-AC-6
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::tc_024_tc187_u02_dimension_normalization_agrees
  ac_id: FR-014-AC-6
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::tc_024_tc187_u05_affine_point_arithmetic_is_refused_alike
  ac_id: FR-014-AC-6
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::tc_024_tc187_u09_a_foreign_dimension_key_is_ill_typed
  ac_id: FR-014-AC-6
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::tc_024_tc187_u14_static_refusals_charge_nothing
  ac_id: FR-014-AC-6
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::tc_024_tc187_u20_composed_offsets_decide_affinity
  ac_id: FR-014-AC-6
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::tc_024_tc187_u27_division_by_zero_precedes_every_edge
  ac_id: FR-014-AC-6
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::tc_024_tc187_u06_u11_unit_graph_admission_refuses_each_malformed_graph
  ac_id: FR-014-AC-6
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::tc_024_tc193_ieee_oracles_agree
  ac_id: FR-014-AC-6
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::tc_024_no_vector_here_has_an_authority_leg_and_all_agree_with_direct_runtime
  ac_id: FR-014-AC-6
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::exact_quantity_conversion_vectors_agree
  ac_id: FR-014-AC-6
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::denied_conversion_agreement_detects_non_result_meter_mutation
  ac_id: FR-014-AC-6
  trace: correct
- test_id: tests/it/exact_scalar_agreement.rs::tc_024_generated_scalar_crate_agrees_with_direct_runtime
  ac_id: FR-014-AC-6
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_refused_items_are_typed_emit_no_code_and_leave_siblings_unchanged
  ac_id: FR-014-AC-7
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_admitted_protocol_node_is_unsupported
  ac_id: FR-014-AC-7
  trace: correct
- test_id: src/core/profile.rs::tests::runtime_boolean_outcome_has_the_kernel_type
  ac_id: FR-014-AC-8
  trace: correct
- test_id: src/core/profile.rs::tests::only_equality_manifests_acquire_the_admitted_context_facade
  ac_id: FR-014-AC-8
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::tc_027_staged_native_observer_compiles_with_locked_kernel_and_charge_log
  ac_id: FR-014-AC-8
  trace: correct
- test_id: tests/it/exact_function_generation.rs::generated_manifest_exposes_semantic_value_owner_source
  ac_id: FR-014-AC-8
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_catalog_consumer_negative_cases_refuse_wrong_shapes_and_preserve_order_and_target
  ac_id: FR-014-AC-8
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_generated_crate_is_unpublished_charge_free_and_compiles
  ac_id: FR-014-AC-8
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_025_v2_precondition_fixture_compiles_with_its_direct_dependencies
  ac_id: FR-014-AC-8
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::tc_024_operand_width_mismatch_is_a_typed_stop_before_any_charge
  ac_id: FR-014-AC-9
  trace: correct
- test_id: tests/it/exact_scalar_agreement.rs::tc_024_generated_scalar_crate_agrees_with_direct_runtime
  ac_id: FR-014-AC-9
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_generated_source_over_the_ceiling_is_refused_whole
  ac_id: FR-014-AC-9
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_literal_operands_are_classified_by_value_kind_and_constants_stop_typed
  ac_id: FR-014-AC-9
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_generated_crate_is_unpublished_charge_free_and_compiles
  ac_id: FR-014-AC-9
  trace: correct
- test_id: tests/exact_scalar_support/package.rs::bound_tests::anonymous_integer_authority_admits_one_integer_range
  ac_id: FR-014-AC-10
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_052_aggregate_scalar_operand_is_refused_by_reader
  ac_id: FR-014-AC-10
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_052_quantity_literal_operand_is_refused_by_reader
  ac_id: FR-014-AC-10
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_052_noncanonical_integer_endpoint_is_refused_by_reader
  ac_id: FR-014-AC-10
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_refused_items_are_typed_emit_no_code_and_leave_siblings_unchanged
  ac_id: FR-014-AC-10
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_052_aggregate_integer_operand_is_refused_by_reader
  ac_id: FR-014-AC-10
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_unreadable_integer_bound_is_a_strict_reader_refusal
  ac_id: FR-014-AC-10
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_missing_rounding_mode_is_a_strict_reader_refusal
  ac_id: FR-014-AC-10
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_anonymous_integer_range_generates_one_addition
  ac_id: FR-014-AC-10
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_corpus_reader_admits_anonymous_integer_ranges
  ac_id: FR-014-AC-10
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::wrong_bound_form_is_rejected_before_scalar_generation
  ac_id: FR-014-AC-10
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_claim_map_carries_identity_source_bounds_and_operation_per_item
  ac_id: FR-014-AC-11
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_every_exact_scalar_refusal_variant_is_matched_exhaustively
  ac_id: FR-014-AC-11
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_operator_confusion_within_one_shape_is_not_silently_confirmed
  ac_id: FR-014-AC-12
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_a_mislabelled_descriptor_is_refused_where_bounds_disagree_and_marked_otherwise
  ac_id: FR-014-AC-13
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_catalog_consumer_negative_cases_refuse_wrong_shapes_and_preserve_order_and_target
  ac_id: FR-014-AC-13
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_claim_map_carries_identity_source_bounds_and_operation_per_item
  ac_id: FR-014-AC-14
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_every_exact_scalar_refusal_variant_is_matched_exhaustively
  ac_id: FR-014-AC-14
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_work_exhaustion_shape_is_reader_admitted_at_small_size
  ac_id: FR-014-AC-15
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_lowering_work_exhaustion_is_a_typed_refusal
  ac_id: FR-014-AC-15
  trace: correct
- test_id: src/oracle/scalar/lower/tests.rs::tc_024_integer_range_is_read_from_binding_members_and_refuses_bare_literals
  ac_id: FR-014-AC-16
  trace: correct
- test_id: src/oracle/scalar/lower/tests.rs::tc_024_bound_members_are_looked_up_by_name_in_any_order
  ac_id: FR-014-AC-16
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_bounded_increment_over_a_bounded_parameter_is_supported_with_a_scalar_harness
  ac_id: FR-014-AC-16
  trace: correct
- test_id: src/oracle/scalar/lower/tests.rs::tc_024_reference_typed_by_a_bounded_domain_is_classified_by_its_base_scalar_type
  ac_id: FR-014-AC-17
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac17_an_operand_typed_by_a_bounded_domain_generates
  ac_id: FR-014-AC-17
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_bounded_increment_over_a_bounded_parameter_is_supported_with_a_scalar_harness
  ac_id: FR-014-AC-17
  trace: correct
- test_id: src/oracle/scalar/derive/tests.rs::tc_024_the_identity_inverse_matches_the_forward_catalogue_for_every_descriptor
  ac_id: FR-014-AC-18
  trace: correct
- test_id: src/oracle/scalar/derive/tests.rs::tc_024_derivation_checks_the_graph_and_the_lowering_before_the_identity
  ac_id: FR-014-AC-18
  trace: correct
- test_id: src/oracle/scalar/derive/tests.rs::tc_024_derivation_refuses_a_missing_identity_a_non_application_and_a_foreign_identity
  ac_id: FR-014-AC-18
  trace: correct
- test_id: src/oracle/scalar/derive/tests.rs::tc_024_derivation_reads_the_division_profile_from_the_laws_and_nowhere_else
  ac_id: FR-014-AC-18
  trace: correct
- test_id: src/oracle/scalar/derive/tests.rs::tc_024_derivation_requires_the_mode_to_agree_with_the_bound_rounding
  ac_id: FR-014-AC-18
  trace: correct
- test_id: src/oracle/scalar/derive/tests.rs::tc_024_the_refused_catalogue_identities_have_no_family
  ac_id: FR-014-AC-18
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_catalog_consumer_negative_cases_refuse_wrong_shapes_and_preserve_order_and_target
  ac_id: FR-014-AC-18
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_derivation_equals_every_golden_corpus_descriptor
  ac_id: FR-014-AC-18
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_each_overloaded_identity_derives_by_its_own_selector
  ac_id: FR-014-AC-18
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_derivation_refuses_what_it_cannot_derive_with_a_typed_reason
  ac_id: FR-014-AC-18
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_an_underivable_claim_is_underived_and_carries_the_nodes_identity
  ac_id: FR-014-AC-18
  trace: correct
- test_id: src/oracle/scalar/derive/tests.rs::tc_024_the_identity_inverse_matches_the_forward_catalogue_for_every_descriptor
  ac_id: FR-014-AC-19
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_derivation_equals_every_golden_corpus_descriptor
  ac_id: FR-014-AC-19
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac20_two_distinct_bounded_parameters_generate_against_the_result_bound
  ac_id: FR-014-AC-20
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_bounded_parameters_are_supported_with_a_per_operand_scalar_harness
  ac_id: FR-014-AC-20
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac21_a_scalar_typed_result_takes_the_one_bound_no_operand_types
  ac_id: FR-014-AC-21
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac22_a_scalar_typed_result_with_only_operand_bounds_is_ambiguous
  ac_id: FR-014-AC-22
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_derivation_refuses_what_it_cannot_derive_with_a_typed_reason
  ac_id: FR-014-AC-22
  trace: correct
- test_id: src/oracle/scalar/lower/tests.rs::wrong_form_and_wrong_base_bounds_keep_typed_defensive_refusals
  ac_id: FR-014-AC-23
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac23_an_operand_or_result_typed_by_a_bound_of_another_form_is_refused
  ac_id: FR-014-AC-23
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac24_operand_bounds_need_not_lie_inside_the_result_bound
  ac_id: FR-014-AC-24
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_bounded_parameters_are_supported_with_a_per_operand_scalar_harness
  ac_id: FR-014-AC-24
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac25_a_plain_typed_reference_operand_beside_bounded_typing_requires_a_bound
  ac_id: FR-014-AC-25
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_a_plain_typed_reference_operand_is_requires_bound_with_no_harness
  ac_id: FR-014-AC-25
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac26_a_reference_to_a_literal_value_node_is_a_literal_operand
  ac_id: FR-014-AC-26
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_qsl_shaped_increment_sum_and_negation_are_supported
  ac_id: FR-014-AC-26
  trace: correct
- test_id: src/oracle/scalar/lower/tests.rs::tc_024_a_conversion_narrows_only_its_one_referenced_argument
  ac_id: FR-014-AC-27
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac27_a_scalar_typed_result_takes_the_bound_of_its_narrowing_conversion
  ac_id: FR-014-AC-27
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_qsl_shaped_increment_sum_and_negation_are_supported
  ac_id: FR-014-AC-27
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac28_a_literal_form_node_without_a_literal_body_is_not_a_literal
  ac_id: FR-014-AC-28
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_qsl_shaped_plain_operands_refuse_and_selected_narrowing_generates
  ac_id: FR-014-AC-28
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac29_a_plain_parameter_beside_a_bounded_one_requires_a_bound
  ac_id: FR-014-AC-29
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_qsl_shaped_plain_operands_refuse_and_selected_narrowing_generates
  ac_id: FR-014-AC-29
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac30_plain_operands_under_a_narrowing_require_a_bound
  ac_id: FR-014-AC-30
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac31_a_non_integer_literal_operand_is_an_operand_type_mismatch
  ac_id: FR-014-AC-31
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_qsl_shaped_literal_and_consumer_refusals_have_no_harness
  ac_id: FR-014-AC-31
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac32_two_distinct_narrowings_are_ambiguous
  ac_id: FR-014-AC-32
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_qsl_shaped_plain_operands_refuse_and_selected_narrowing_generates
  ac_id: FR-014-AC-32
  trace: correct
- test_id: src/oracle/scalar/lower/tests.rs::wrong_form_and_wrong_base_bounds_keep_typed_defensive_refusals
  ac_id: FR-014-AC-33
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac33_a_narrowing_of_another_form_or_base_type_is_missing_bound
  ac_id: FR-014-AC-33
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac34_a_node_narrowed_and_consumed_otherwise_is_ambiguous
  ac_id: FR-014-AC-34
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_qsl_shaped_literal_and_consumer_refusals_have_no_harness
  ac_id: FR-014-AC-34
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_derivation_refuses_what_it_cannot_derive_with_a_typed_reason
  ac_id: FR-014-AC-35
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_boolean_and_integer_equality_nodes_generate_confirmed_runtime_oracles
  ac_id: FR-014-AC-35
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_boolean_descriptor_rejects_a_wrong_right_evaluation_mode
  ac_id: FR-014-AC-35
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_boolean_oracle_source_map_probes_the_entry_and_implication_consequent
  ac_id: FR-014-AC-36
  trace: correct
- test_id: tests/exact_scalar_support/boolean_agreement_cases.rs::boolean_connectives_agree_in_outcome_charges_counters_and_denials
  ac_id: FR-014-AC-37
  trace: correct
- test_id: tests/exact_scalar_support/boolean_agreement_cases.rs::typed_scalar_equalities_agree_in_outcome_charges_counters_and_denials
  ac_id: FR-014-AC-37
  trace: correct
- test_id: tests/it/exact_scalar_agreement.rs::tc_024_v2_boolean_and_equality_oracles_agree_with_direct_runtime
  ac_id: FR-014-AC-37
  trace: correct
- test_id: src/kani/generate/role_parity_tests.rs::clause_node_index_preserves_first_node_and_reuses_one_walk
  ac_id: FR-014-AC-38
  trace: correct
- test_id: src/kani/generate/role_parity_tests.rs::clause_planning_and_reading_share_one_graph_walk
  ac_id: FR-014-AC-38
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::qsl_emitted_state_clause_derives_a_confirmed_field_backed_oracle
  ac_id: FR-014-AC-38
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::clause_claim_map_carries_the_derived_descriptor_and_typed_disposition
  ac_id: FR-014-AC-38
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::admitted_clause_refuses_non_clause_node
  ac_id: FR-014-AC-38
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::clause_oracle_refuses_a_clause_from_another_admitted_package
  ac_id: FR-014-AC-38
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::admitted_inline_clause_keeps_the_clause_identity
  ac_id: FR-014-AC-38
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::qsl_emitted_pre_of_an_expression_is_refused
  ac_id: FR-014-AC-38
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::qsl_emitted_negated_clause_composes_the_boolean_oracle
  ac_id: FR-014-AC-38
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::qsl_model_field_range_is_the_clause_input_range
  ac_id: FR-014-AC-38
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::qsl_bounded_operation_parameter_becomes_a_clause_input
  ac_id: FR-014-AC-38
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::qsl_emitted_clause_composes_boolean_and_integer_operations
  ac_id: FR-014-AC-38
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::skipped_v2_clause_consequent_has_a_measured_zero_native_probe
  ac_id: FR-014-AC-38
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac39_the_panic_scan_names_every_banned_token_and_no_lookalike
  ac_id: FR-014-AC-39
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac39_non_test_code_strips_test_items_by_structure
  ac_id: FR-014-AC-39
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac39_scalar_generator_and_generated_crates_hold_no_panic_site
  ac_id: FR-014-AC-39
  trace: correct
- test_id: src/oracle/scalar/lower/tests.rs::tc_024_a_failed_record_is_refused_by_its_limit_kind
  ac_id: FR-014-AC-40
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_a_byte_ceiling_lowering_failure_is_refused_per_item_as_its_own_refusal
  ac_id: FR-014-AC-40
  trace: correct
- test_id: src/oracle/mod.rs::tests::tc_024_the_classifier_reads_each_limit_kind_once
  ac_id: FR-014-AC-41
  trace: correct
- test_id: src/oracle/scalar/lower/tests.rs::tc_024_a_failed_record_is_refused_by_its_limit_kind
  ac_id: FR-014-AC-41
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac42_only_the_shared_classifier_reads_a_failed_records_limit_kind
  ac_id: FR-014-AC-42
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac44_direct_temporal_formula_is_unsupported
  ac_id: FR-014-AC-44
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac44_direct_control_case_is_unsupported
  ac_id: FR-014-AC-44
  trace: correct
- test_id: src/oracle/scalar/lower/tests.rs::tc_024_nested_inference_accepts_inline_canonical_integer_literals
  ac_id: FR-014-AC-45
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_qsl_emitted_nested_integer_operands_are_bounded_or_require_bounds
  ac_id: FR-014-AC-45
  trace: correct
- test_id: src/oracle/scalar/lower/tests.rs::tc_024_nested_inference_reports_missing_cycle_unsupported_and_bad_literals
  ac_id: FR-014-AC-46
  trace: correct
- test_id: src/oracle/scalar/lower/tests.rs::tc_024_nested_inference_accepts_inline_canonical_integer_literals
  ac_id: FR-014-AC-46
  trace: correct
- test_id: src/oracle/scalar/lower/tests.rs::tc_024_nested_inference_enforces_depth_before_reusing_memoized_nodes
  ac_id: FR-014-AC-46
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_routed_operand_refusals_leave_healthy_sibling_unchanged
  ac_id: FR-014-AC-46
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_qsl_emitted_nested_integer_operands_are_bounded_or_require_bounds
  ac_id: FR-014-AC-46
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_qsl_nested_operand_depth_limit_is_typed_and_keeps_sibling_supported
  ac_id: FR-014-AC-46
  trace: correct
- test_id: src/oracle/scalar/derive/tests.rs::quantity_conversion_confirmation_requires_identity_and_exact_mode
  ac_id: FR-014-AC-48
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::exact_quantity_conversion_vectors_agree
  ac_id: FR-014-AC-48
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::denied_conversion_agreement_detects_non_result_meter_mutation
  ac_id: FR-014-AC-48
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::quantity_conversion_derives_and_preserves_target_provenance
  ac_id: FR-014-AC-48
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::caller_declared_quantity_conversion_is_confirmed
  ac_id: FR-014-AC-48
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::quantity_conversion_ill_typed_preserves_meter
  ac_id: FR-014-AC-49
  trace: correct
- test_id: src/oracle/scalar/derive/tests.rs::quantity_conversion_rejects_scalar_targets_after_mode
  ac_id: FR-014-AC-50
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::exact_quantity_conversion_vectors_agree
  ac_id: FR-014-AC-50
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::admitted_compound_conversion_target_refuses_without_changing_sibling
  ac_id: FR-014-AC-50
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::unit_function_signatures_refuse_per_function_and_preserve_supported_siblings
  ac_id: FR-014-AC-50
  trace: correct
- test_id: tests/it/capability_settlement.rs::a_selected_quantity_conversion_settles_as_operation_not_rendered
  ac_id: FR-014-AC-51
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_025_a_quantity_conversion_claim_is_unrendered_with_no_harness
  ac_id: FR-014-AC-51
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_quantity_conversion_has_an_oracle_and_no_harness
  ac_id: FR-014-AC-51
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_quantity_conversion_requires_authenticated_operation_views
  ac_id: FR-014-AC-51
  trace: correct
- test_id: tests/exact_scalar_support/agreement_cases.rs::quantity_paired_result_units_survive_stops_and_feed_following_operations
  ac_id: FR-014-AC-52
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::wrong_bound_form_is_rejected_before_scalar_generation
  ac_id: FR-014-AC-52
  trace: correct
- test_id: src/oracle/clause.rs::tests::canonical_i64_leaves_use_the_public_primitive_constructor
  ac_id: FR-014-AC-53
  trace: correct
- test_id: src/oracle/clause.rs::tests::every_ineligible_leaf_keeps_the_original_parse_and_typed_failure
  ac_id: FR-014-AC-53
  trace: correct
- test_id: tests/it/v2_clause_integer_literal_agreement.rs::public_v2_integer_literals_preserve_owner_semantics_and_every_denial_prefix
  ac_id: FR-014-AC-53
  trace: correct
- test_id: tests/it/capability_settlement.rs::a_selected_quantity_conversion_settles_as_operation_not_rendered
  ac_id: FR-014-AC-55
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_selected_division_preserves_exact_renderer_refusal
  ac_id: FR-014-AC-55
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_quantity_conversion_has_an_oracle_and_no_harness
  ac_id: FR-014-AC-55
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_quantity_conversion_requires_authenticated_operation_views
  ac_id: FR-014-AC-55
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_unauthenticated_view_refusal_uses_lowest_request_index
  ac_id: FR-014-AC-55
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::hybrid_family_is_unsupported_and_preserves_node_identity
  ac_id: FR-018-AC-1
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::duplicate_member_private_defense_retains_exact_cause_and_healthy_sibling
  ac_id: FR-018-AC-1
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac1_every_item_gets_one_disposition_refused_siblings_unchanged
  ac_id: FR-018-AC-1
  trace: correct
- test_id: tests/admitted_composite_support/agreement_cases.rs::genuine_nested_composites_match_outcomes_charges_counters_and_each_denial
  ac_id: FR-018-AC-2
  trace: correct
- test_id: tests/composite_equality_support/cases/collection.rs::original_shape_vectors_collection_3
  ac_id: FR-018-AC-2
  trace: correct
- test_id: tests/composite_equality_support/cases/conv.rs::original_free_conversion_vectors
  ac_id: FR-018-AC-2
  trace: correct
- test_id: tests/composite_equality_support/cases/conv_dec_dec.rs::original_conv_dec_dec_vectors
  ac_id: FR-018-AC-2
  trace: correct
- test_id: tests/composite_equality_support/cases/conv_dec_int.rs::original_conv_dec_int_vectors
  ac_id: FR-018-AC-2
  trace: correct
- test_id: tests/composite_equality_support/cases/conv_dec_rat.rs::original_conv_dec_rat_vectors
  ac_id: FR-018-AC-2
  trace: correct
- test_id: tests/composite_equality_support/cases/conv_rat_int.rs::original_conv_rat_int_vectors
  ac_id: FR-018-AC-2
  trace: correct
- test_id: tests/composite_equality_support/cases/conv_rat_rat.rs::original_conv_rat_rat_vectors
  ac_id: FR-018-AC-2
  trace: correct
- test_id: tests/composite_equality_support/cases/enum.rs::original_shape_vectors_enum_8
  ac_id: FR-018-AC-2
  trace: correct
- test_id: tests/composite_equality_support/cases/nested.rs::original_shape_vectors_nested_5
  ac_id: FR-018-AC-2
  trace: correct
- test_id: tests/composite_equality_support/cases/option.rs::original_shape_vectors_option_2
  ac_id: FR-018-AC-2
  trace: correct
- test_id: tests/composite_equality_support/cases/record.rs::original_shape_vectors_record_0
  ac_id: FR-018-AC-2
  trace: correct
- test_id: tests/composite_equality_support/cases/record.rs::original_shape_vectors_record_6
  ac_id: FR-018-AC-2
  trace: correct
- test_id: tests/composite_equality_support/cases/recursive.rs::original_shape_vectors_recursive_4
  ac_id: FR-018-AC-2
  trace: correct
- test_id: tests/composite_equality_support/cases/text.rs::original_shape_vectors_text_7
  ac_id: FR-018-AC-2
  trace: correct
- test_id: tests/composite_equality_support/cases/tuple.rs::original_shape_vectors_tuple_1
  ac_id: FR-018-AC-2
  trace: correct
- test_id: tests/it/admitted_composite_equality.rs::genuine_record_tuple_closure_and_native_oracles_use_original_owner_keys
  ac_id: FR-018-AC-2
  trace: correct
- test_id: tests/it/admitted_composite_equality.rs::genuine_composite_host_generation_preserves_closure_and_capability_ordering
  ac_id: FR-018-AC-2
  trace: correct
- test_id: tests/it/composite_equality_agreement.rs::tc_029_generated_composite_equality_crate_agrees_with_direct_runtime
  ac_id: FR-018-AC-2
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac3_claim_descriptor_matches_the_request
  ac_id: FR-018-AC-3
  trace: correct
- test_id: tests/it/admitted_enum_equality.rs::authentic_text_option_and_primitive_sequence_generate_with_real_bounds
  ac_id: FR-018-AC-4
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_admitted_one_reference_sequence_resolves_its_separate_bound
  ac_id: FR-018-AC-4
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac4_schedule_matches_checked_equality_for_every_shape
  ac_id: FR-018-AC-4
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_a_sequence_without_a_separate_collection_bound_is_refused
  ac_id: FR-018-AC-4
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac5_a_disallowed_conversion_refuses_at_generation_time
  ac_id: FR-018-AC-5
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_nested_ieee_equality_is_refused_by_reader
  ac_id: FR-018-AC-6
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac6_ieee_at_any_depth_is_operator_ineligible
  ac_id: FR-018-AC-6
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac16_a_float_rounding_member_is_refused_as_operator_ineligible
  ac_id: FR-018-AC-6
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac7_each_family_gets_its_own_distinct_blocker
  ac_id: FR-018-AC-7
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac7_a_direct_reference_operand_is_refused_by_ir_today
  ac_id: FR-018-AC-7
  trace: correct
- test_id: src/oracle/equality/admitted.rs::tests::actual_composite_lookup_refusals_preserve_kind_identity_and_foreign_owner
  ac_id: FR-018-AC-8
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::duplicate_member_private_defense_retains_exact_cause_and_healthy_sibling
  ac_id: FR-018-AC-8
  trace: correct
- test_id: tests/admitted_composite_support/agreement_cases.rs::genuine_nested_composites_match_outcomes_charges_counters_and_each_denial
  ac_id: FR-018-AC-8
  trace: correct
- test_id: tests/composite_equality_support/cases/record.rs::tc_029_ac8_check_type_guards_the_oracle
  ac_id: FR-018-AC-8
  trace: correct
- test_id: tests/it/admitted_composite_equality.rs::genuine_record_tuple_closure_and_native_oracles_use_original_owner_keys
  ac_id: FR-018-AC-8
  trace: correct
- test_id: tests/it/admitted_composite_equality.rs::genuine_composite_host_generation_preserves_closure_and_capability_ordering
  ac_id: FR-018-AC-8
  trace: correct
- test_id: tests/it/composite_equality_agreement.rs::tc_029_generated_composite_equality_crate_agrees_with_direct_runtime
  ac_id: FR-018-AC-8
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac8_a_duplicate_field_is_refused_with_its_declaration_cause
  ac_id: FR-018-AC-8
  trace: correct
- test_id: tests/admitted_composite_support/agreement_cases.rs::genuine_nested_composites_match_outcomes_charges_counters_and_each_denial
  ac_id: FR-018-AC-9
  trace: correct
- test_id: tests/composite_equality_support/cases/collection.rs::original_shape_vectors_collection_3
  ac_id: FR-018-AC-9
  trace: correct
- test_id: tests/composite_equality_support/cases/conv_charge.rs::original_charged_conversion_vectors
  ac_id: FR-018-AC-9
  trace: correct
- test_id: tests/composite_equality_support/cases/enum.rs::original_shape_vectors_enum_8
  ac_id: FR-018-AC-9
  trace: correct
- test_id: tests/composite_equality_support/cases/nested.rs::original_shape_vectors_nested_5
  ac_id: FR-018-AC-9
  trace: correct
- test_id: tests/composite_equality_support/cases/option.rs::original_shape_vectors_option_2
  ac_id: FR-018-AC-9
  trace: correct
- test_id: tests/composite_equality_support/cases/record.rs::original_shape_vectors_record_0
  ac_id: FR-018-AC-9
  trace: correct
- test_id: tests/composite_equality_support/cases/record.rs::original_shape_vectors_record_6
  ac_id: FR-018-AC-9
  trace: correct
- test_id: tests/composite_equality_support/cases/record.rs::tc_029_ac9_a_denied_charge_is_incomplete_never_a_completed_boolean
  ac_id: FR-018-AC-9
  trace: correct
- test_id: tests/composite_equality_support/cases/recursive.rs::original_shape_vectors_recursive_4
  ac_id: FR-018-AC-9
  trace: correct
- test_id: tests/composite_equality_support/cases/text.rs::original_shape_vectors_text_7
  ac_id: FR-018-AC-9
  trace: correct
- test_id: tests/composite_equality_support/cases/tuple.rs::original_shape_vectors_tuple_1
  ac_id: FR-018-AC-9
  trace: correct
- test_id: tests/it/admitted_composite_equality.rs::genuine_record_tuple_closure_and_native_oracles_use_original_owner_keys
  ac_id: FR-018-AC-9
  trace: correct
- test_id: tests/it/composite_equality_agreement.rs::tc_029_generated_composite_equality_crate_agrees_with_direct_runtime
  ac_id: FR-018-AC-9
  trace: correct
- test_id: tests/composite_equality_support/cases/collection.rs::original_shape_vectors_collection_3
  ac_id: FR-018-AC-10
  trace: correct
- test_id: tests/composite_equality_support/cases/enum.rs::original_shape_vectors_enum_8
  ac_id: FR-018-AC-10
  trace: correct
- test_id: tests/composite_equality_support/cases/nested.rs::original_shape_vectors_nested_5
  ac_id: FR-018-AC-10
  trace: correct
- test_id: tests/composite_equality_support/cases/option.rs::original_shape_vectors_option_2
  ac_id: FR-018-AC-10
  trace: correct
- test_id: tests/composite_equality_support/cases/record.rs::original_shape_vectors_record_0
  ac_id: FR-018-AC-10
  trace: correct
- test_id: tests/composite_equality_support/cases/record.rs::original_shape_vectors_record_6
  ac_id: FR-018-AC-10
  trace: correct
- test_id: tests/composite_equality_support/cases/recursive.rs::original_shape_vectors_recursive_4
  ac_id: FR-018-AC-10
  trace: correct
- test_id: tests/composite_equality_support/cases/text.rs::original_shape_vectors_text_7
  ac_id: FR-018-AC-10
  trace: correct
- test_id: tests/composite_equality_support/cases/tuple.rs::original_shape_vectors_tuple_1
  ac_id: FR-018-AC-10
  trace: correct
- test_id: tests/it/admitted_composite_equality.rs::same_shaped_record_and_tuple_names_keep_distinct_repeatable_owner_identities
  ac_id: FR-018-AC-10
  trace: correct
- test_id: tests/it/admitted_enum_equality.rs::authentic_enum_request_order_repeatability_and_dual_operator_symbols
  ac_id: FR-018-AC-10
  trace: correct
- test_id: tests/it/composite_equality_agreement.rs::tc_029_generated_composite_equality_crate_agrees_with_direct_runtime
  ac_id: FR-018-AC-10
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac10_bytes_are_deterministic_across_request_permutations
  ac_id: FR-018-AC-10
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac10_claim_map_entries_ascend_by_the_descriptor_key
  ac_id: FR-018-AC-10
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac10_generation_is_repeatable_from_a_fresh_package
  ac_id: FR-018-AC-10
  trace: correct
- test_id: tests/composite_equality_support/cases/record.rs::tc_029_ac11_operator_variants_produce_complementary_outcomes
  ac_id: FR-018-AC-11
  trace: correct
- test_id: tests/it/admitted_composite_equality.rs::same_shaped_record_and_tuple_names_keep_distinct_repeatable_owner_identities
  ac_id: FR-018-AC-11
  trace: correct
- test_id: tests/it/admitted_enum_equality.rs::authentic_enum_request_order_repeatability_and_dual_operator_symbols
  ac_id: FR-018-AC-11
  trace: correct
- test_id: tests/it/composite_equality_agreement.rs::tc_029_generated_composite_equality_crate_agrees_with_direct_runtime
  ac_id: FR-018-AC-11
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac11_two_operators_over_one_node_get_distinct_symbols_and_are_caller_declared
  ac_id: FR-018-AC-11
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac12_integer_helper_omitted_when_unreached
  ac_id: FR-018-AC-12
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac12_manifest_is_unpublished_and_charge_free
  ac_id: FR-018-AC-12
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac13_declaration_keys_are_the_reached_v2_node_ids
  ac_id: FR-018-AC-13
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::tc_029_bound_members_are_read_from_named_bindings
  ac_id: FR-018-AC-14
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::tc_029_bare_literal_bound_members_are_refused
  ac_id: FR-018-AC-14
  trace: correct
- test_id: tests/it/admitted_enum_equality.rs::authentic_text_option_and_primitive_sequence_generate_with_real_bounds
  ac_id: FR-018-AC-14
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac15_a_reference_to_a_conversion_is_read_as_the_type_it_converts
  ac_id: FR-018-AC-15
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac15_nested_conversions_are_read_through_to_the_innermost_operand
  ac_id: FR-018-AC-15
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac15_a_non_conversion_application_operand_is_read_as_its_own_type
  ac_id: FR-018-AC-15
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_equality_over_a_conversion_operand_is_refused_as_an_unsupported_quantity
  ac_id: FR-018-AC-15
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::legacy_numeric_member_alias_equivalence_is_a_private_resolution_control
  ac_id: FR-018-AC-16
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::wrong_bound_form_private_defense_keeps_exact_missing_integer_range
  ac_id: FR-018-AC-16
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::unsupported_private_bound_bases_name_the_original_bound_and_preserve_sibling
  ac_id: FR-018-AC-16
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_a_sequence_without_a_separate_collection_bound_is_refused
  ac_id: FR-018-AC-16
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac16_a_text_bounds_member_reconstructs_its_own_text_type
  ac_id: FR-018-AC-16
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac16_numeric_bounded_domain_members_read_as_their_bounded_scalars
  ac_id: FR-018-AC-16
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac16_a_bounded_domain_member_never_reads_a_sibling_bound
  ac_id: FR-018-AC-16
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac16_a_bound_form_that_does_not_fit_its_base_is_refused
  ac_id: FR-018-AC-16
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac16_a_float_rounding_member_is_refused_as_operator_ineligible
  ac_id: FR-018-AC-16
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac16_a_bounded_domain_over_a_base_without_a_bound_form_is_refused
  ac_id: FR-018-AC-16
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac17_emitted_source_has_no_panic_site_fallback_or_index_expression
  ac_id: FR-018-AC-17
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac17_emitted_reconstruction_is_typed_and_refused_never_unwrapped
  ac_id: FR-018-AC-17
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac17_the_structural_checker_names_each_departure
  ac_id: FR-018-AC-17
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::tc_029_ac18_render_value_type_refuses_quantity_and_reference_with_a_typed_error
  ac_id: FR-018-AC-18
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::tc_029_ac18_the_item_boundary_refuses_the_item_and_fails_the_call_on_a_generation_error
  ac_id: FR-018-AC-18
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::tc_029_ac18_a_render_failure_refuses_only_its_item_and_never_renames_a_sibling
  ac_id: FR-018-AC-18
  trace: correct
- test_id: tests/it/admitted_quantity_units.rs::genuine_quantity_items_keep_their_refusal_and_healthy_sibling
  ac_id: FR-018-AC-18
  trace: correct
- test_id: tests/it/admitted_quantity_units.rs::nested_quantity_stays_unsupported_with_owner_metadata_and_healthy_unions
  ac_id: FR-018-AC-18
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac19_the_equality_generator_source_holds_no_panic_token
  ac_id: FR-018-AC-19
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac39_the_panic_scan_names_every_banned_token_and_no_lookalike
  ac_id: FR-018-AC-19
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_024_ac39_non_test_code_strips_test_items_by_structure
  ac_id: FR-018-AC-19
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac20_byte_ceiling_has_failed_records_and_no_contract_package
  ac_id: FR-018-AC-20
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::tc_029_unclosed_option_and_sequence_cycles_refuse_at_the_repeated_node
  ac_id: FR-018-AC-24
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac24_recursive_list_and_tree_items_emit_one_oracle_each
  ac_id: FR-018-AC-24
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_public_qsl_recursive_items_generate_through_reader
  ac_id: FR-018-AC-24
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_authoritative_qspec_recursive_items_generate_through_reader
  ac_id: FR-018-AC-24
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::tc_029_type_resolution_charges_each_entry_and_the_first_over_limit_entry
  ac_id: FR-018-AC-25
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::tc_029_deep_option_resolution_and_runtime_type_operations_fit_a_small_stack
  ac_id: FR-018-AC-25
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::tc_029_inline_collection_bound_is_charged_before_lookup
  ac_id: FR-018-AC-25
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::tc_029_qsl_recursive_package_uses_the_reader_shapes
  ac_id: FR-018-AC-26
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::tc_029_recursive_list_and_tree_shapes_close_at_record_keys
  ac_id: FR-018-AC-26
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::tc_029_optional_wrapper_malformed_shapes_refuse_without_affecting_tree
  ac_id: FR-018-AC-26
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::tc_029_direct_option_field_is_required_not_silently_optional
  ac_id: FR-018-AC-26
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_public_qsl_recursive_items_generate_through_reader
  ac_id: FR-018-AC-26
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_authoritative_qspec_recursive_items_generate_through_reader
  ac_id: FR-018-AC-26
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::tc_029_qsl_recursive_package_uses_the_reader_shapes
  ac_id: FR-018-AC-27
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::tc_029_recursive_list_and_tree_shapes_close_at_record_keys
  ac_id: FR-018-AC-27
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::tc_029_sequence_bound_shape_mutants_refuse_without_affecting_list
  ac_id: FR-018-AC-27
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_public_qsl_recursive_items_generate_through_reader
  ac_id: FR-018-AC-27
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_authoritative_qspec_recursive_items_generate_through_reader
  ac_id: FR-018-AC-27
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::deep_acyclic_closure_generates_flat_compilable_source_on_a_small_stack
  ac_id: FR-018-AC-28
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::cycle_refusal_follows_its_charged_entry_and_work_refusal_wins_at_the_boundary
  ac_id: FR-018-AC-29
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::source_limit_refuses_before_a_function_or_module_fragment_is_appended
  ac_id: FR-018-AC-30
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::deep_acyclic_closure_generates_flat_compilable_source_on_a_small_stack
  ac_id: FR-018-AC-30
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::maximum_work_setting_refuses_and_last_denied_count_is_representable
  ac_id: FR-018-AC-31
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::invalid_type_resolution_work_limit_refuses_before_lowering
  ac_id: FR-018-AC-31
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_ac32_direct_control_case_refusal_preserves_healthy_equality_sibling
  ac_id: FR-018-AC-32
  trace: correct
- test_id: src/oracle/equality/mod.rs::tests::deep_acyclic_closure_generates_flat_compilable_source_on_a_small_stack
  ac_id: FR-018-AC-33
  trace: correct
- test_id: tests/admitted_enum_support/agreement_cases.rs::wrong_original_package_and_wrong_prepared_item_stop_before_lookup_or_charge
  ac_id: FR-018-AC-33
  trace: correct
- test_id: tests/it/admitted_enum_equality.rs::genuine_enum_generated_native_abi_agrees_with_owner
  ac_id: FR-018-AC-33
  trace: correct
- test_id: tests/it/admitted_enum_equality.rs::public_generator_rejects_another_authentic_source_context
  ac_id: FR-018-AC-33
  trace: correct
- test_id: tests/it/admitted_enum_equality.rs::primitive_only_generation_rejects_same_name_context_with_different_checked_body
  ac_id: FR-018-AC-33
  trace: correct
- test_id: tests/it/composite_original_eq.rs::original_request_retains_the_actual_compilation_context
  ac_id: FR-018-AC-33
  trace: correct
- test_id: tests/admitted_enum_support/agreement_cases.rs::complete_genuine_enum_pairs_match_outcomes_ordered_charges_counters_and_every_denial
  ac_id: FR-018-AC-34
  trace: correct
- test_id: tests/admitted_enum_support/agreement_cases.rs::same_case_of_distinct_genuine_declarations_is_not_an_alias
  ac_id: FR-018-AC-34
  trace: correct
- test_id: tests/it/admitted_enum_equality.rs::genuine_enum_generated_native_abi_agrees_with_owner
  ac_id: FR-018-AC-34
  trace: correct
- test_id: src/replay/composite_builder.rs::admitted_failure_tests::boundary_mapping_retains_a_genuine_malformed_source_package_refusal
  ac_id: FR-018-AC-35
  trace: correct
- test_id: src/replay/composite_builder.rs::admitted_failure_tests::actual_boundary_mapping_retains_public_fault_payload
  ac_id: FR-018-AC-35
  trace: correct
- test_id: tests/it/composite_original_eq.rs::original_request_preserves_the_owning_reader_limit
  ac_id: FR-018-AC-35
  trace: correct
- test_id: tests/it/composite_original_eq.rs::original_request_preserves_the_above_reader_resource_refusal
  ac_id: FR-018-AC-35
  trace: correct
- test_id: tests/admitted_enum_support/agreement_cases.rs::complete_genuine_enum_pairs_match_outcomes_ordered_charges_counters_and_every_denial
  ac_id: FR-018-AC-36
  trace: correct
- test_id: tests/admitted_enum_support/agreement_cases.rs::same_case_of_distinct_genuine_declarations_is_not_an_alias
  ac_id: FR-018-AC-36
  trace: correct
- test_id: tests/admitted_enum_support/agreement_cases.rs::wrong_original_package_and_wrong_prepared_item_stop_before_lookup_or_charge
  ac_id: FR-018-AC-36
  trace: correct
- test_id: tests/it/admitted_enum_equality.rs::genuine_enum_generated_native_abi_agrees_with_owner
  ac_id: FR-018-AC-36
  trace: correct
- test_id: tests/it/admitted_enum_equality.rs::authentic_enum_request_order_repeatability_and_dual_operator_symbols
  ac_id: FR-018-AC-36
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_nominal_owner_lookups_use_only_structural_wire_identities
  ac_id: FR-018-AC-36
  trace: correct
- test_id: src/core/profile.rs::tests::only_equality_manifests_acquire_the_admitted_context_facade
  ac_id: FR-018-AC-37
  trace: correct
- test_id: tests/it/admitted_enum_equality.rs::genuine_enum_generated_native_abi_agrees_with_owner
  ac_id: FR-018-AC-37
  trace: correct
- test_id: tests/it/admitted_enum_equality.rs::authentic_text_option_and_primitive_sequence_generate_with_real_bounds
  ac_id: FR-018-AC-38
  trace: correct
- test_id: tests/it/admitted_quantity_units.rs::genuine_quantity_items_keep_their_refusal_and_healthy_sibling
  ac_id: FR-018-AC-38
  trace: correct
- test_id: tests/it/admitted_quantity_units.rs::nested_quantity_stays_unsupported_with_owner_metadata_and_healthy_unions
  ac_id: FR-018-AC-38
  trace: correct
- test_id: src/oracle/equality/admitted.rs::tests::actual_composite_lookup_refusals_preserve_kind_identity_and_foreign_owner
  ac_id: FR-018-AC-39
  trace: correct
- test_id: tests/it/admitted_composite_equality.rs::genuine_record_tuple_closure_and_native_oracles_use_original_owner_keys
  ac_id: FR-018-AC-39
  trace: correct
- test_id: tests/it/admitted_composite_equality.rs::genuine_composite_host_generation_preserves_closure_and_capability_ordering
  ac_id: FR-018-AC-39
  trace: correct
- test_id: tests/it/admitted_composite_equality.rs::source_context_comparison_retains_a_present_checked_term_sidecar
  ac_id: FR-018-AC-39
  trace: correct
- test_id: tests/it/admitted_enum_equality.rs::optional_capability_absence_is_typed_and_wrong_presence_is_global
  ac_id: FR-018-AC-39
  trace: correct
- test_id: tests/it/admitted_enum_equality.rs::nominal_capability_demand_follows_nested_option_and_collection_elements
  ac_id: FR-018-AC-39
  trace: correct
- test_id: tests/admitted_integer_support/agreement_cases.rs::context_free_constructor_rejects_explicit_wrong_context
  ac_id: FR-018-AC-40
  trace: correct
- test_id: tests/it/admitted_enum_equality.rs::optional_capability_absence_is_typed_and_wrong_presence_is_global
  ac_id: FR-018-AC-40
  trace: correct
- test_id: tests/it/admitted_enum_equality.rs::optional_capability_absence_is_typed_and_wrong_presence_is_global
  ac_id: FR-018-AC-41
  trace: correct
- test_id: tests/it/admitted_enum_equality.rs::nominal_capability_demand_follows_nested_option_and_collection_elements
  ac_id: FR-018-AC-41
  trace: correct
- test_id: tests/admitted_integer_support/agreement_cases.rs::context_free_constructor_rejects_explicit_wrong_context
  ac_id: FR-018-AC-42
  trace: correct
- test_id: tests/it/admitted_composite_equality.rs::genuine_record_tuple_closure_and_native_oracles_use_original_owner_keys
  ac_id: FR-018-AC-42
  trace: correct
- test_id: tests/it/admitted_composite_equality.rs::genuine_composite_host_generation_preserves_closure_and_capability_ordering
  ac_id: FR-018-AC-42
  trace: correct
- test_id: tests/it/admitted_composite_equality.rs::source_context_comparison_retains_a_present_checked_term_sidecar
  ac_id: FR-018-AC-42
  trace: correct
- test_id: tests/admitted_integer_support/agreement_cases.rs::unbounded_right_values_outside_sibling_interval_still_complete
  ac_id: FR-018-AC-43
  trace: correct
- test_id: tests/it/admitted_enum_equality.rs::genuine_bounded_parameter_keeps_other_integer_and_target_unbounded
  ac_id: FR-018-AC-43
  trace: correct
- test_id: src/oracle/equality/union_tests.rs::union_closure_seals_owner_members_and_admits_each_nested_payload
  ac_id: FR-018-AC-44
  trace: correct
- test_id: src/oracle/equality/union_tests.rs::union_lookup_and_seal_faults_keep_typed_causes_before_equality
  ac_id: FR-018-AC-44
  trace: correct
- test_id: tests/it/admitted_composite_equality.rs::genuine_union_host_generation_closes_nested_owner_declarations
  ac_id: FR-018-AC-44
  trace: correct
- test_id: tests/it/admitted_composite_equality.rs::genuine_union_capability_refusals_preserve_the_healthy_sibling
  ac_id: FR-018-AC-44
  trace: correct
- test_id: src/oracle/equality/units.rs::tests::preparation_preserves_the_complete_owner_table_and_returned_unit_ids
  ac_id: FR-018-AC-45
  trace: correct
- test_id: tests/it/admitted_quantity_units.rs::genuine_quantity_items_keep_their_refusal_and_healthy_sibling
  ac_id: FR-018-AC-45
  trace: correct
- test_id: tests/it/admitted_quantity_units.rs::nested_quantity_stays_unsupported_with_owner_metadata_and_healthy_unions
  ac_id: FR-018-AC-45
  trace: correct
- test_id: src/oracle/equality/admitted.rs::tests::actual_composite_lookup_refusals_preserve_kind_identity_and_foreign_owner
  ac_id: FR-018-AC-46
  trace: correct
- test_id: tests/it/composite_equality_generation.rs::tc_029_nominal_owner_lookups_use_only_structural_wire_identities
  ac_id: FR-018-AC-46
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::operand_type_refusal_retains_the_original_v2_frame
  ac_id: FR-021-AC-1
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac1_every_item_gets_one_disposition_and_refusal_does_not_cascade
  ac_id: FR-021-AC-1
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac1_duplicate_request_refuses_every_copy
  ac_id: FR-021-AC-1
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_arity_mismatch_refuses_when_item_argument_count_disagrees
  ac_id: FR-021-AC-1
  trace: correct
- test_id: tests/exact_function_support/agreement_cases.rs::generated_and_independent_fr485_session_match_in_outcome_and_charges
  ac_id: FR-021-AC-2
  trace: correct
- test_id: tests/exact_function_support/imported_cases.rs::generated_library_body_matches_importer_facade_application
  ac_id: FR-021-AC-2
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_031_repeated_source_operand_refuses_positional_add_body
  ac_id: FR-021-AC-2
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_031_reordered_source_call_refuses_positional_nested_body
  ac_id: FR-021-AC-2
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_031_source_callee_refuses_requested_alternate_and_keeps_sibling
  ac_id: FR-021-AC-2
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_031_source_equal_refuses_requested_not_equal_body
  ac_id: FR-021-AC-2
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_031_independent_fr485_session_runs_all_three_body_forms_without_registration
  ac_id: FR-021-AC-2
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_031_generated_function_crate_matches_independent_fr485_session
  ac_id: FR-021-AC-2
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac3_claim_records_function_and_origin_from_the_request
  ac_id: FR-021-AC-3
  trace: correct
- test_id: tests/exact_function_support/agreement_cases.rs::qsl_input_refusals_precede_application_charges_and_arity_precedes_kind
  ac_id: FR-021-AC-4
  trace: correct
- test_id: tests/exact_function_support/agreement_cases.rs::qsl_input_refusals_precede_application_charges_and_arity_precedes_kind
  ac_id: FR-021-AC-5
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac6_undischargeable_capability_refuses_before_any_item
  ac_id: FR-021-AC-6
  trace: correct
- test_id: src/oracle/function/schedule.rs::tests::ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack
  ac_id: FR-021-AC-7
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac7_admitted_hundred_thousand_call_nodes_generate_on_a_512_kib_stack
  ac_id: FR-021-AC-7
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac8_generated_source_never_names_check_mode_kernel
  ac_id: FR-021-AC-8
  trace: correct
- test_id: tests/exact_function_support/agreement_cases.rs::function_call_denial_is_incomplete_without_an_admitted_charge
  ac_id: FR-021-AC-9
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac10_reference_parameter_outside_package_is_unknown_type_node
  ac_id: FR-021-AC-10
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac11_model_and_state_are_distinct_blockers
  ac_id: FR-021-AC-11
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac12_dangling_callee_refuses_only_its_own_items
  ac_id: FR-021-AC-12
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac12_form_mismatch_refuses_only_its_own_item
  ac_id: FR-021-AC-12
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_signature_mismatch_refuses_arity_and_result_type_disagreement
  ac_id: FR-021-AC-12
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac13_generation_is_deterministic_across_runs_and_permutations
  ac_id: FR-021-AC-13
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac14_manifest_and_source_shape
  ac_id: FR-021-AC-14
  trace: correct
- test_id: src/oracle/function/schedule.rs::tests::ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack
  ac_id: FR-021-AC-15
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ambiguous_function_name_refuses_both_and_does_not_corrupt_indices
  ac_id: FR-021-AC-15
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac15_location_map_round_trips_to_the_request_structurally
  ac_id: FR-021-AC-15
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac16_no_generated_code_reads_location_or_losses
  ac_id: FR-021-AC-16
  trace: correct
- test_id: src/oracle/function/owner_path.rs::tests::nested_call_site_comes_from_the_selected_checked_body
  ac_id: FR-021-AC-17
  trace: correct
- test_id: src/oracle/function/owner_path.rs::tests::own_and_imported_callers_charge_their_checked_nested_site
  ac_id: FR-021-AC-17
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_031_nested_static_origin_matches_the_qsl_checked_call_site
  ac_id: FR-021-AC-17
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_031_imported_static_origin_matches_the_qsl_checked_call_site
  ac_id: FR-021-AC-17
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::omitted_earlier_function_does_not_renumber_the_checked_origin
  ac_id: FR-021-AC-17
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ambiguous_function_name_refuses_both_and_does_not_corrupt_indices
  ac_id: FR-021-AC-17
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac19_emitted_function_oracle_source_has_no_panicking_path
  ac_id: FR-021-AC-19
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac20_closed_kernel_outcomes_are_exhaustive
  ac_id: FR-021-AC-20
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac21_unsupported_operator_refuses_unary_negate_scalar_body
  ac_id: FR-021-AC-21
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::oracle_generators_have_no_panicking_arms
  ac_id: FR-021-AC-21
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_031_ac21_the_whole_file_scan_counts_a_test_module_and_not_a_comment
  ac_id: FR-021-AC-21
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac22_fixture_i_both_bodies_admissible
  ac_id: FR-021-AC-22
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac22_fixture_ii_one_body_refused_and_sibling_admissible
  ac_id: FR-021-AC-22
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac22_fixture_iii_shared_name_takes_the_node_id_refusal
  ac_id: FR-021-AC-22
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac22_fixture_iv_nested_call_to_a_refused_duplicate_is_unknown_callee
  ac_id: FR-021-AC-22
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac22_fixture_v_two_duplicate_groups_report_the_smallest_node_id
  ac_id: FR-021-AC-22
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac22_fixture_vi_same_call_node_items_over_a_pair_each_keep_the_node_refusal
  ac_id: FR-021-AC-22
  trace: correct
- test_id: src/oracle/function/mod.rs::tests::schedule_call_item_lowering_failure_keeps_its_own_limit_details
  ac_id: FR-021-AC-23
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac23_byte_ceiling_has_failed_records_and_no_contract_package
  ac_id: FR-021-AC-23
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac23_small_shared_chain_lowers_with_complete_closures
  ac_id: FR-021-AC-23
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac24_case_i_one_unknown_name_is_one_entry
  ac_id: FR-021-AC-24
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac24_case_ii_two_unknown_names_are_two_entries_in_name_order
  ac_id: FR-021-AC-24
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac24_case_iii_the_same_unknown_name_twice_is_one_duplicate_request
  ac_id: FR-021-AC-24
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac24_case_iv_known_and_unknown_order_unknown_first_and_match_solo
  ac_id: FR-021-AC-24
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac24_case_v_names_order_by_bytes_case_sensitively
  ac_id: FR-021-AC-24
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac24_case_vi_duplicate_node_pair_members_order_by_name
  ac_id: FR-021-AC-24
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac24_case_vii_differing_declaring_node_ids_order_before_names
  ac_id: FR-021-AC-24
  trace: correct
- test_id: src/oracle/function/schedule.rs::tests::ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack
  ac_id: FR-021-AC-25
  trace: correct
- test_id: src/oracle/function/schedule.rs::tests::ac25_nested_add_schedule_is_postorder_and_budget_refusal_publishes_nothing
  ac_id: FR-021-AC-25
  trace: correct
- test_id: src/oracle/function/schedule.rs::tests::ac27_integer_literal_is_an_ordered_inline_schedule_operand
  ac_id: FR-021-AC-25
  trace: correct
- test_id: src/oracle/function/schedule.rs::tests::ac25_max_limit_is_rejected_before_any_graph_action
  ac_id: FR-021-AC-25
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac7_admitted_hundred_thousand_call_nodes_generate_on_a_512_kib_stack
  ac_id: FR-021-AC-25
  trace: correct
- test_id: src/oracle/function/schedule.rs::tests::ac25_nested_add_schedule_is_postorder_and_budget_refusal_publishes_nothing
  ac_id: FR-021-AC-26
  trace: correct
- test_id: src/oracle/function/mod.rs::tests::ac27_inline_scalar_literal_is_retained_with_its_admitted_type
  ac_id: FR-021-AC-27
  trace: correct
- test_id: src/oracle/function/schedule.rs::tests::ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack
  ac_id: FR-021-AC-27
  trace: correct
- test_id: src/oracle/function/schedule.rs::tests::ac25_nested_add_schedule_is_postorder_and_budget_refusal_publishes_nothing
  ac_id: FR-021-AC-27
  trace: correct
- test_id: src/oracle/function/schedule.rs::tests::ac27_integer_literal_is_an_ordered_inline_schedule_operand
  ac_id: FR-021-AC-27
  trace: correct
- test_id: src/oracle/function/schedule.rs::tests::equality_operation_identity_preserves_its_scalar_domain
  ac_id: FR-021-AC-27
  trace: correct
- test_id: src/oracle/function/schedule.rs::tests::ac27_call_uses_exact_declaration_identity_and_rejects_mismatched_binding
  ac_id: FR-021-AC-27
  trace: correct
- test_id: src/oracle/function/schedule.rs::tests::ac27_unknown_operation_and_mismatched_operand_types_refuse_before_schedule
  ac_id: FR-021-AC-27
  trace: correct
- test_id: src/oracle/function/schedule.rs::tests::ac27_cycle_shared_node_and_unbound_parameter_refuse_without_a_schedule
  ac_id: FR-021-AC-27
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac7_admitted_hundred_thousand_call_nodes_generate_on_a_512_kib_stack
  ac_id: FR-021-AC-27
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_source_function_call_binds_callee_and_ordered_arguments
  ac_id: FR-021-AC-27
  trace: correct
- test_id: src/oracle/function/schedule.rs::tests::ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack
  ac_id: FR-021-AC-28
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac34_public_strict_reader_chain_generates_compilable_flat_schedule
  ac_id: FR-021-AC-28
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac7_admitted_hundred_thousand_call_nodes_generate_on_a_512_kib_stack
  ac_id: FR-021-AC-28
  trace: correct
- test_id: src/oracle/function/schedule.rs::tests::ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack
  ac_id: FR-021-AC-29
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac7_admitted_hundred_thousand_call_nodes_generate_on_a_512_kib_stack
  ac_id: FR-021-AC-29
  trace: correct
- test_id: src/core/profile.rs::tests::function_manifest_uses_the_checked_closure_facade_and_direct_leaves
  ac_id: FR-021-AC-32
  trace: correct
- test_id: src/oracle/function/owner_render.rs::tests::scalar_equality_and_nested_call_render_through_one_facade
  ac_id: FR-021-AC-32
  trace: correct
- test_id: tests/exact_function_support/agreement_cases.rs::generated_and_independent_fr485_session_match_in_outcome_and_charges
  ac_id: FR-021-AC-32
  trace: correct
- test_id: tests/exact_function_support/imported_cases.rs::generated_library_body_matches_importer_facade_application
  ac_id: FR-021-AC-32
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_031_genuine_qsl_source_generates_three_direct_function_claims
  ac_id: FR-021-AC-32
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_031_generated_function_crate_matches_independent_fr485_session
  ac_id: FR-021-AC-32
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac33_direct_case_call_node_is_unsupported
  ac_id: FR-021-AC-33
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac33_genuine_temporal_roots_leave_a_callable_sibling
  ac_id: FR-021-AC-33
  trace: correct
- test_id: tests/it/exact_scalar_generation.rs::tc_031_ac33_direct_control_case_is_unsupported
  ac_id: FR-021-AC-33
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac34_public_strict_reader_chain_generates_compilable_flat_schedule
  ac_id: FR-021-AC-34
  trace: correct
- test_id: src/oracle/function/owner_path.rs::tests::checked_owner_path_applies_a_genuine_source_callable
  ac_id: FR-021-AC-35
  trace: correct
- test_id: src/oracle/function/owner_path.rs::tests::checked_owner_path_applies_a_locked_imported_callable
  ac_id: FR-021-AC-35
  trace: correct
- test_id: src/oracle/function/owner_path.rs::tests::own_and_imported_callers_charge_their_checked_nested_site
  ac_id: FR-021-AC-35
  trace: correct
- test_id: src/oracle/function/owner_path.rs::tests::malformed_source_stops_at_acquisition
  ac_id: FR-021-AC-35
  trace: correct
- test_id: src/oracle/function/owner_path.rs::tests::unrelated_declaring_package_cannot_authorize_a_callable
  ac_id: FR-021-AC-35
  trace: correct
- test_id: tests/exact_function_support/agreement_cases.rs::generated_and_independent_fr485_session_match_in_outcome_and_charges
  ac_id: FR-021-AC-35
  trace: correct
- test_id: tests/exact_function_support/imported_cases.rs::generated_library_body_matches_importer_facade_application
  ac_id: FR-021-AC-35
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_031_genuine_qsl_source_generates_three_direct_function_claims
  ac_id: FR-021-AC-35
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_031_independent_fr485_session_runs_all_three_body_forms_without_registration
  ac_id: FR-021-AC-35
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_031_generated_function_crate_matches_independent_fr485_session
  ac_id: FR-021-AC-35
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_031_generated_imported_owner_matches_independent_fr485_session
  ac_id: FR-021-AC-35
  trace: correct
- test_id: src/oracle/function/owner_path.rs::tests::checked_owner_path_applies_a_genuine_source_callable
  ac_id: FR-021-AC-36
  trace: correct
- test_id: src/oracle/function/owner_path.rs::tests::checked_owner_path_applies_a_locked_imported_callable
  ac_id: FR-021-AC-36
  trace: correct
- test_id: src/oracle/function/owner_path.rs::tests::nested_call_site_comes_from_the_selected_checked_body
  ac_id: FR-021-AC-36
  trace: correct
- test_id: src/oracle/function/owner_path.rs::tests::own_and_imported_callers_charge_their_checked_nested_site
  ac_id: FR-021-AC-36
  trace: correct
- test_id: tests/exact_function_support/agreement_cases.rs::wrong_unit_or_owner_refuses_before_registration_and_charge
  ac_id: FR-021-AC-36
  trace: correct
- test_id: tests/exact_function_support/imported_cases.rs::generated_library_body_matches_importer_facade_application
  ac_id: FR-021-AC-36
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_031_generated_imported_owner_matches_independent_fr485_session
  ac_id: FR-021-AC-36
  trace: correct
- test_id: src/oracle/function/owner_path.rs::tests::genuine_other_unit_identity_stops_before_lookup_or_registration
  ac_id: FR-021-AC-37
  trace: correct
- test_id: src/oracle/function/owner_path.rs::tests::unrelated_declaring_package_cannot_authorize_a_callable
  ac_id: FR-021-AC-37
  trace: correct
- test_id: src/oracle/function/owner_path.rs::tests::post_lookup_guard_refuses_a_genuine_second_selection
  ac_id: FR-021-AC-37
  trace: correct
- test_id: src/oracle/function/owner_path.rs::tests::typed_facade_outcomes_keep_distinct_serializable_cause_codes
  ac_id: FR-021-AC-37
  trace: correct
- test_id: tests/exact_function_support/agreement_cases.rs::wrong_unit_or_owner_refuses_before_registration_and_charge
  ac_id: FR-021-AC-37
  trace: correct
- test_id: tests/exact_function_support/invariant_cases.rs::result_membership_uses_real_signature_and_preserves_stopped_arms_and_charges
  ac_id: FR-021-AC-38
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_057_generated_checked_invariant_guards_use_checked_owner_inputs
  ac_id: FR-021-AC-38
  trace: correct
- test_id: tests/exact_function_support/invariant_cases.rs::authentic_owner_declarations_construct_once_and_duplicate_keeps_original_diagnostic
  ac_id: FR-021-AC-39
  trace: correct
- test_id: tests/exact_function_support/invariant_cases.rs::real_constructor_work_and_storage_stops_never_become_declaration_rejections
  ac_id: FR-021-AC-39
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_057_generated_checked_invariant_guards_use_checked_owner_inputs
  ac_id: FR-021-AC-39
  trace: correct
- test_id: tests/exact_function_support/invariant_cases.rs::result_membership_uses_real_signature_and_preserves_stopped_arms_and_charges
  ac_id: FR-021-AC-40
  trace: correct
- test_id: tests/exact_function_support/invariant_cases.rs::primitive_membership_has_no_storage_failure_execution_credit
  ac_id: FR-021-AC-40
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_057_generated_checked_invariant_guards_use_checked_owner_inputs
  ac_id: FR-021-AC-40
  trace: correct
- test_id: src/oracle/function/mod.rs::tests::selected_imported_call_is_ineligible_for_temporal_body_guard
  ac_id: FR-021-AC-41
  trace: correct
- test_id: tests/it/exact_function_generation.rs::tc_031_ac33_direct_case_call_node_is_unsupported
  ac_id: FR-021-AC-41
  trace: correct
- test_id: src/oracle/function/owner_render.rs::tests::kani_build_keeps_body_modules_and_drops_host_entries
  ac_id: FR-021-AC-42
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_native_arithmetic_is_the_runtime_exact_operation_over_the_types_interval
  ac_id: FR-031-AC-1
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_native_division_uses_one_exact_member_and_domain
  ac_id: FR-031-AC-2
  trace: correct
- test_id: tests/it/oracle_generation.rs::tc_044_native_proven_division_uses_exact_member
  ac_id: FR-031-AC-2
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_an_arithmetic_oracle_returns_an_outcome_and_takes_a_meter
  ac_id: FR-031-AC-3
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_an_arithmetic_free_comparison_keeps_the_native_operator
  ac_id: FR-031-AC-4
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_arithmetic_free_comparisons_agree_with_the_runtime
  ac_id: FR-031-AC-4
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_overflow_under_reject_is_the_runtimes_refusal
  ac_id: FR-031-AC-5
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_native_division_zero_and_minimum_vectors
  ac_id: FR-031-AC-6
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_saturating_arithmetic_is_refused_at_its_node
  ac_id: FR-031-AC-7
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_the_refusal_locus_is_the_first_unsupported_node_in_preorder
  ac_id: FR-031-AC-7
  trace: correct
- test_id: tests/it/oracle_generation.rs::tc_003_unsupported_expression_and_root_map_to_declared_terminal_states
  ac_id: FR-031-AC-7
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_arithmetic_outcomes_equal_an_independent_plain_integer_model
  ac_id: FR-031-AC-8
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_the_first_stop_in_evaluation_order_is_the_outcome
  ac_id: FR-031-AC-9
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_pure_connective_subtrees_retain_within_a_metered_root
  ac_id: FR-031-AC-9
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_017_bound_admission_uses_the_public_clause_and_domain
  ac_id: FR-031-AC-10
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_a_refused_sibling_clause_refuses_the_strategy_of_every_clause_in_its_package
  ac_id: FR-031-AC-10
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_boolean_sinks_refuse_arithmetic
  ac_id: FR-031-AC-10
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_an_arithmetic_free_oracle_is_byte_identical_to_the_one_before
  ac_id: FR-031-AC-12
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_an_arithmetic_oracle_is_deterministic_and_independent_of_declaration_order
  ac_id: FR-031-AC-13
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_an_arithmetic_oracle_holds_no_unwrap_expect_or_panic_macro
  ac_id: FR-031-AC-14
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_a_comparison_with_an_arithmetic_operand_is_the_runtime_call
  ac_id: FR-031-AC-15
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_native_division_zero_and_minimum_vectors
  ac_id: FR-031-AC-16
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_native_division_matches_independent_member_model
  ac_id: FR-031-AC-17
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_017_bound_admission_uses_the_public_clause_and_domain
  ac_id: FR-031-AC-18
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_saturating_divide_and_remainder_are_refused_by_the_native_oracle
  ac_id: FR-031-AC-18
  trace: correct
- test_id: tests/it/oracle_generation.rs::wide_ir_integer_bounds_refuse_an_i64_oracle_without_generated_source
  ac_id: FR-031-AC-28
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_native_division_uses_one_exact_member_and_domain
  ac_id: FR-031-AC-22
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_native_division_zero_and_minimum_vectors
  ac_id: FR-031-AC-22
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_mixed_division_connective_retain_denial_is_typed
  ac_id: FR-031-AC-22
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_native_division_zero_and_minimum_vectors
  ac_id: FR-031-AC-26
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_mixed_division_connective_retain_denial_is_typed
  ac_id: FR-031-AC-26
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_native_connectives_retain_only_completed_results
  ac_id: FR-031-AC-26
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_pure_connective_subtrees_retain_within_a_metered_root
  ac_id: FR-031-AC-26
  trace: correct
- test_id: src/oracle/boolean_v1.rs::checked_invariant_tests::generated_interval_guard_retains_the_exact_constructor_cause
  ac_id: FR-042-AC-1
  trace: correct
- test_id: tests/composite_equality_support/cases/record.rs::tc_029_ac8_check_type_guards_the_oracle
  ac_id: FR-042-AC-1
  trace: correct
- test_id: tests/composite_equality_support/cases/record.rs::mismatched_real_prepared_bindings_use_descriptor_cause_before_charging
  ac_id: FR-042-AC-1
  trace: correct
- test_id: tests/composite_equality_support/cases/record.rs::actual_equality_check_rejection_preserves_its_original_typed_payload
  ac_id: FR-042-AC-1
  trace: correct
- test_id: tests/exact_function_support/invariant_cases.rs::native_argument_shape_guard_uses_its_exact_cause_without_a_body_charge
  ac_id: FR-042-AC-1
  trace: correct
- test_id: tests/exact_function_support/invariant_cases.rs::native_operand_kind_guard_uses_its_exact_cause_without_a_body_charge
  ac_id: FR-042-AC-1
  trace: correct
- test_id: tests/exact_function_support/invariant_cases.rs::result_membership_uses_real_signature_and_preserves_stopped_arms_and_charges
  ac_id: FR-042-AC-2
  trace: correct
- test_id: tests/exact_function_support/invariant_cases.rs::primitive_membership_has_no_storage_failure_execution_credit
  ac_id: FR-042-AC-2
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_057_generated_checked_invariant_guards_use_checked_owner_inputs
  ac_id: FR-042-AC-2
  trace: correct
- test_id: tests/exact_function_support/invariant_cases.rs::authentic_owner_declarations_construct_once_and_duplicate_keeps_original_diagnostic
  ac_id: FR-042-AC-3
  trace: correct
- test_id: tests/exact_function_support/invariant_cases.rs::real_constructor_work_and_storage_stops_never_become_declaration_rejections
  ac_id: FR-042-AC-3
  trace: correct
- test_id: tests/it/exact_function_agreement.rs::tc_057_generated_checked_invariant_guards_use_checked_owner_inputs
  ac_id: FR-042-AC-3
  trace: correct
- test_id: src/kani/output/playback.rs::tests::assertion_reader_types_rendered_values_and_refusals
  ac_id: FR-016-AC-1
  trace: correct
- test_id: src/replay/witness.rs::tests::decode_falsification_reads_a_multi_line_check_text
  ac_id: FR-016-AC-1
  trace: correct
- test_id: tests/it/native_v2_helper_playback.rs::native_v2_helper_mutant_playback_decodes_typed_original_witness
  ac_id: FR-016-AC-1
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_malformed_counterexamples_stop_at_output_or_decode
  ac_id: FR-016-AC-1
  trace: correct
- test_id: src/replay/witness.rs::tests::first_out_of_domain_is_inclusive_at_both_bounds
  ac_id: FR-016-AC-2
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::native_v2_decoder_enforces_original_inclusive_bounds
  ac_id: FR-016-AC-2
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_an_out_of_domain_counterexample_is_evidence_failure
  ac_id: FR-016-AC-2
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_an_in_domain_counterexample_the_twin_falsifies_is_reproduced
  ac_id: FR-016-AC-3
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_a_healthy_native_twin_does_not_reproduce_the_violation
  ac_id: FR-016-AC-4
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_a_counterexample_the_twin_holds_is_evidence_failure
  ac_id: FR-016-AC-4
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_a_counterexample_for_another_harness_is_never_replayed
  ac_id: FR-016-AC-5
  trace: correct
- test_id: src/replay/witness.rs::tests::witness_decode_bounds_have_no_default
  ac_id: FR-016-AC-6
  trace: correct
- test_id: src/replay/witness.rs::tests::decode_refuses_a_poisoned_suffix_past_each_bound
  ac_id: FR-016-AC-6
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::tc_026_corpus_replay_refuses_poisoned_suffixes_at_both_decode_bounds
  ac_id: FR-016-AC-6
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_026_state_clause_decode_refuses_poisoned_suffixes_at_both_bounds
  ac_id: FR-016-AC-6
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_026_frame_replay_refuses_poisoned_suffixes_at_both_decode_bounds
  ac_id: FR-016-AC-6
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_function_replay_refuses_poisoned_suffixes_at_both_decode_bounds
  ac_id: FR-016-AC-6
  trace: correct
- test_id: src/replay/witness.rs::tests::argument_types_preserve_order_and_type
  ac_id: FR-016-AC-8
  trace: correct
- test_id: src/replay/witness.rs::tests::argument_types_refuse_a_non_argument_binding
  ac_id: FR-016-AC-8
  trace: correct
- test_id: src/replay/witness.rs::tests::decode_falsification_names_each_value_by_its_binding_position
  ac_id: FR-016-AC-8
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::state_clause_kani_playback_adapter_refuses_wrong_evidence
  ac_id: FR-016-AC-8
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_a_falsifying_input_replays_through_qsl_to_the_same_violation
  ac_id: FR-016-AC-9
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_the_replayed_verdict_depends_on_the_witness_value
  ac_id: FR-016-AC-9
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_a_boolean_value_replays_as_zero_or_one
  ac_id: FR-016-AC-9
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_the_replay_verdict_is_decided_by_the_exact_witness_values
  ac_id: FR-016-AC-9
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_the_replayed_verdict_depends_on_the_witness_value
  ac_id: FR-016-AC-10
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_a_healthy_native_twin_does_not_reproduce_the_violation
  ac_id: FR-016-AC-10
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_a_value_with_no_parameter_is_refused
  ac_id: FR-016-AC-11
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_each_adapter_refusal_is_its_own_typed_error
  ac_id: FR-016-AC-11
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_an_executor_fault_remains_a_typed_replay_refusal
  ac_id: FR-016-AC-12
  trace: correct
- test_id: src/replay/function.rs::tests::only_a_reproduced_violation_reproduces
  ac_id: FR-016-AC-13
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_a_unit_importing_a_locked_dependency_replays_to_a_reproduced_verdict
  ac_id: FR-016-AC-14
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_an_import_with_no_locked_dependency_is_refused
  ac_id: FR-016-AC-16
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_qsl_refuses_a_dependency_the_unit_does_not_select
  ac_id: FR-016-AC-18
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_libraries_sharing_a_source_owner_are_refused
  ac_id: FR-016-AC-19
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_a_library_sharing_the_units_source_owner_is_refused
  ac_id: FR-016-AC-20
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_a_lock_repeating_a_dependency_is_refused
  ac_id: FR-016-AC-24
  trace: correct
- test_id: tests/it/kani_stdout_capture.rs::tests::tc_026_observing_launcher_retains_actual_stdout_and_exit
  ac_id: FR-016-AC-25
  trace: correct
- test_id: tests/it/kani_stdout_capture.rs::tests::tc_026_observing_installed_launcher_preserves_version_output
  ac_id: FR-016-AC-25
  trace: correct
- test_id: tests/it/kani_stdout_capture.rs::tests::tc_026_observed_stdout_selects_only_the_named_assertion_block
  ac_id: FR-016-AC-25
  trace: correct
- test_id: src/replay/function.rs::tests::tc_026_the_identity_ignores_the_declared_order_of_the_site_parameters
  ac_id: FR-016-AC-26
  trace: correct
- test_id: src/replay/obligation.rs::tests::large_declaration_ordinal_preserves_producer_identity
  ac_id: FR-016-AC-26
  trace: correct
- test_id: src/replay/function.rs::tests::tc_026_producer_identity_tracks_the_selected_function_node
  ac_id: FR-016-AC-27
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_a_held_producer_identity_rejects_changed_claim_members
  ac_id: FR-016-AC-27
  trace: correct
- test_id: src/replay/function.rs::tests::tc_035_function_request_requires_its_producer_packet
  ac_id: FR-016-AC-28
  trace: correct
- test_id: src/replay/function.rs::tests::tc_026_the_identity_ignores_the_declared_order_of_the_site_parameters
  ac_id: FR-016-AC-28
  trace: correct
- test_id: src/replay/function.rs::tests::tc_026_producer_identity_tracks_the_selected_function_node
  ac_id: FR-016-AC-28
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_a_held_producer_identity_rejects_changed_claim_members
  ac_id: FR-016-AC-28
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_the_request_package_reference_carries_the_lock_dependencies
  ac_id: FR-016-AC-29
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_026_a_changed_dependency_selection_is_refused_before_replay
  ac_id: FR-016-AC-30
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_the_frame_witness_is_rendered_from_the_decoded_playback
  ac_id: FR-024-AC-2
  trace: correct
- test_id: tests/it/layout.rs::witness_parse_calls_stay_in_the_one_adapter_renderer
  ac_id: FR-024-AC-2
  trace: correct
- test_id: src/replay/function.rs::tests::tc_035_function_domain_boundaries_stop_before_replay
  ac_id: FR-024-AC-3
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_a_decoded_value_outside_its_declared_range_is_refused_at_the_endpoints
  ac_id: FR-024-AC-3
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_035_public_function_domain_boundaries_stop_before_replay
  ac_id: FR-024-AC-3
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_035_missing_packet_members_stop_before_replay
  ac_id: FR-024-AC-4
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_frame_packet_reaches_qsl_with_call_site_identities
  ac_id: FR-024-AC-6
  trace: correct
- test_id: tests/it/layout.rs::qsl_replay_types_are_neither_defined_nor_imported_from_contract_ir
  ac_id: FR-024-AC-10
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_035_replay_returns_the_direct_result_and_replay_through_returns_the_executors_value
  ac_id: FR-024-AC-11
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_035_the_envelope_carries_the_call_sites_clause_identities
  ac_id: FR-024-AC-12
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_035_the_envelope_declares_each_ranged_field_and_the_transcript_names_the_playback
  ac_id: FR-024-AC-12
  trace: correct
- test_id: src/kani/output/playback.rs::tests::generated_postcondition_check_is_interpreted_in_output_reader
  ac_id: FR-024-AC-13
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::state_clause_kani_playback_adapter_refuses_wrong_evidence
  ac_id: FR-024-AC-13
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::state_clause_kani_playback_adapter_refuses_wrong_evidence
  ac_id: FR-024-AC-15
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_035_a_missing_field_is_refused_by_name
  ac_id: FR-024-AC-15
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_035_an_undeclared_or_repeated_binding_is_refused_by_name
  ac_id: FR-024-AC-15
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_035_a_violating_run_reproduces_and_a_respecting_run_is_inconclusive
  ac_id: FR-024-AC-16
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::state_clause_kani_playback_adapter_refuses_wrong_evidence
  ac_id: FR-024-AC-17
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_035_a_value_outside_its_declared_range_is_out_of_domain
  ac_id: FR-024-AC-17
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_035_real_kani_state_clause_counterexample_replays_through_qsl
  ac_id: FR-024-AC-18
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_035_an_operation_declaring_a_parameter_or_a_result_is_refused_by_shape
  ac_id: FR-024-AC-19
  trace: correct
- test_id: src/replay/obligation.rs::tests::frame_kind_is_derived_from_property
  ac_id: FR-024-AC-22
  trace: correct
- test_id: src/replay/witness.rs::tests::decode_falsification_refuses_a_repeated_binding_name
  ac_id: FR-024-AC-23
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_the_record_carries_the_draw_order_and_a_record_without_it_is_not_read
  ac_id: FR-024-AC-23
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_the_playback_is_decoded_in_the_harnesss_draw_order
  ac_id: FR-024-AC-23
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_a_repeated_state_field_is_refused_by_the_decoder_and_the_record
  ac_id: FR-024-AC-23
  trace: correct
- test_id: src/replay/frame.rs::tests::tc_035_the_declared_domains_are_empty_and_the_header_names_qsl_345_once
  ac_id: FR-024-AC-25
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_the_playback_is_decoded_in_the_harnesss_draw_order
  ac_id: FR-024-AC-25
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_the_frame_witness_is_rendered_from_the_decoded_playback
  ac_id: FR-024-AC-25
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_a_playback_that_does_not_decode_is_refused_before_the_call_site
  ac_id: FR-024-AC-25
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_a_decoded_value_outside_its_declared_range_is_refused_at_the_endpoints
  ac_id: FR-024-AC-26
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_the_invocation_of_the_playbacks_own_pre_state_replays_and_another_is_refused
  ac_id: FR-024-AC-27
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_a_pre_state_that_cannot_be_read_from_the_invocation_is_refused_by_name
  ac_id: FR-024-AC-27
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_unrelated_document_bytes_leave_the_pre_state_absent
  ac_id: FR-024-AC-27
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_a_harness_scope_that_is_not_the_requested_operation_is_refused_by_member
  ac_id: FR-024-AC-28
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_the_node_ids_of_the_package_qsl_emits_are_the_ids_call_site_names
  ac_id: FR-024-AC-28
  trace: correct
- test_id: src/replay/frame.rs::tests::tc_035_the_declared_domains_are_empty_and_the_header_names_qsl_345_once
  ac_id: FR-024-AC-29
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_the_frame_envelope_declares_no_domain
  ac_id: FR-024-AC-29
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_frame_replay_rechecks_parsed_playback_bounds
  ac_id: FR-024-AC-30
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_the_node_ids_of_the_package_qsl_emits_are_the_ids_call_site_names
  ac_id: FR-024-AC-30
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_real_kani_frame_counterexample_replays_through_qsl
  ac_id: FR-024-AC-30
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_035_emitted_model_fields_bind_replay_before_playback
  ac_id: FR-024-AC-31
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_a_source_model_range_within_jcs_safe_integer_replays_without_override
  ac_id: FR-024-AC-32
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_real_kani_frame_counterexample_replays_through_qsl
  ac_id: FR-024-AC-32
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_035_emitted_model_fields_bind_replay_before_playback
  ac_id: FR-024-AC-33
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_035_real_kani_state_clause_counterexample_replays_through_qsl
  ac_id: FR-024-AC-34
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_real_kani_frame_counterexample_replays_through_qsl
  ac_id: FR-024-AC-34
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_035_the_envelope_carries_the_call_sites_clause_identities
  ac_id: FR-024-AC-41
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_frame_packet_reaches_qsl_with_call_site_identities
  ac_id: FR-024-AC-41
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_035_conflicting_replay_targets_stop_before_replay
  ac_id: FR-024-AC-42
  trace: correct
- test_id: src/replay/function.rs::tests::tc_035_function_request_requires_its_producer_packet
  ac_id: FR-024-AC-43
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_035_function_replay_refuses_another_proving_run_identity_before_execution
  ac_id: FR-024-AC-43
  trace: correct
- test_id: tests/it/composite_original_eq.rs::tc_035_original_composite_request_requires_its_own_packet
  ac_id: FR-024-AC-44
  trace: correct
- test_id: tests/it/composite_original_eq.rs::tc_035_original_composite_packets_bind_distinct_real_occurrences
  ac_id: FR-024-AC-44
  trace: correct
- test_id: src/replay/function.rs::tests::tc_035_missing_function_request_occurrence_refuses_before_byte_provision
  ac_id: FR-024-AC-45
  trace: correct
- test_id: src/replay/function.rs::tests::tc_035_function_request_requires_its_producer_packet
  ac_id: FR-024-AC-46
  trace: correct
- test_id: tests/it/composite_original_eq.rs::tc_035_original_composite_request_requires_its_own_packet
  ac_id: FR-024-AC-46
  trace: correct
- test_id: tests/it/composite_original_eq.rs::tc_035_original_composite_packets_bind_distinct_real_occurrences
  ac_id: FR-024-AC-46
  trace: correct
- test_id: tests/it/skeleton_spine.rs::tc_035_function_replay_refuses_another_proving_run_identity_before_execution
  ac_id: FR-024-AC-46
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_the_request_and_envelope_carry_the_producer_frame_claim
  ac_id: FR-024-AC-47
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_a_changed_envelope_identity_refuses_before_frame_replay
  ac_id: FR-024-AC-47
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_the_frame_identity_follows_what_call_site_names
  ac_id: FR-024-AC-47
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_a_clause_on_an_earlier_sorting_operation_moves_the_frame_occurrence
  ac_id: FR-024-AC-47
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_the_frame_identity_names_none_of_the_harness_members
  ac_id: FR-024-AC-47
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_the_request_and_envelope_carry_the_producer_frame_claim
  ac_id: FR-024-AC-48
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_the_request_and_envelope_carry_the_producer_frame_claim
  ac_id: FR-024-AC-50
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::tc_035_a_non_frame_and_a_mismatched_field_set_are_refused_before_the_call_site
  ac_id: FR-024-AC-50
  trace: correct
- test_id: src/replay/state_clause.rs::tests::tc_035_the_model_header_is_the_one_owner_or_a_distinct_error
  ac_id: FR-024-AC-52
  trace: correct
- test_id: src/replay/state_clause.rs::tests::tc_035_duplicate_document_labels_refuse_before_request
  ac_id: FR-024-AC-52
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_035_the_documents_hold_the_playback_and_post_state
  ac_id: FR-024-AC-52
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_035_changed_document_values_change_the_replay_verdict
  ac_id: FR-024-AC-52
  trace: correct
- test_id: src/replay/state_clause.rs::tests::tc_035_the_builder_returns_what_core_canonical_returns_for_the_vector
  ac_id: FR-024-AC-53
  trace: correct
- test_id: src/replay/state_clause.rs::tests::tc_035_the_module_encodes_only_through_core_canonical
  ac_id: FR-024-AC-53
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_035_qsl_reads_reformatted_raw_documents_by_identity
  ac_id: FR-024-AC-53
  trace: correct
- test_id: tests/it/scalar_playback.rs::routed_draw_order_is_preserved_with_distinct_domains
  ac_id: FR-032-AC-2
  trace: correct
- test_id: tests/it/scalar_playback.rs::negate_and_literal_singleton_keep_every_draw
  ac_id: FR-032-AC-2
  trace: correct
- test_id: tests/it/scalar_playback.rs::subtraction_decodes_signed_extreme_operands_without_evaluating_them
  ac_id: FR-032-AC-2
  trace: correct
- test_id: tests/it/scalar_playback.rs::harness_arity_width_and_comment_failures_are_distinct
  ac_id: FR-032-AC-2
  trace: correct
- test_id: tests/it/scalar_playback.rs::assertion_selection_and_malformed_blocks_use_adapter_refusals
  ac_id: FR-032-AC-2
  trace: correct
- test_id: tests/it/scalar_playback.rs::scalar_decoder_rejects_playback_above_its_limit_after_a_loose_parse
  ac_id: FR-032-AC-2
  trace: correct
- test_id: tests/it/scalar_literal_operands.rs::add_keeps_the_plain_literal_inline_through_scalar_generation
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_literal_operands.rs::negate_keeps_the_plain_literal_inline_through_scalar_generation
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_playback.rs::routed_draw_order_is_preserved_with_distinct_domains
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_playback.rs::negate_and_literal_singleton_keep_every_draw
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_playback.rs::subtraction_decodes_signed_extreme_operands_without_evaluating_them
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_playback.rs::inclusive_endpoints_and_adjacent_values_use_each_operands_range
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_playback.rs::typed_schema_byte_ceiling_decode_and_domain_refusals
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_playback.rs::unsupported_operator_and_invalid_persisted_schema_refuse_typed_playback
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_playback.rs::original_renderer_limits_are_retained_with_genuine_routed_source
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_playback.rs::original_limits_refuse_changed_retained_identity_source_or_record
  ac_id: FR-032-AC-3
  trace: correct
- test_id: tests/it/scalar_replay_binding.rs::scalar_actual_sent_context_and_every_driver_tie_are_independently_checked
  ac_id: FR-032-AC-6
  trace: correct
- test_id: tests/it/scalar_literal_operands.rs::add_keeps_the_plain_literal_inline_through_scalar_generation
  ac_id: FR-032-AC-9
  trace: correct
- test_id: tests/it/scalar_literal_operands.rs::equality_with_a_plain_integer_peer_keeps_the_literal_inline
  ac_id: FR-032-AC-9
  trace: correct
- test_id: tests/it/scalar_literal_operands.rs::negate_keeps_the_plain_literal_inline_through_scalar_generation
  ac_id: FR-032-AC-9
  trace: correct
- test_id: tests/it/scalar_literal_operands.rs::equality_with_a_bounded_peer_keeps_the_narrowed_literal_as_a_graph_child
  ac_id: FR-032-AC-9
  trace: correct
- test_id: tests/it/scalar_replay_binding.rs::original_scalar_driver_executes_independent_repeated_draws_and_binds_real_qsl
  ac_id: FR-032-AC-15
  trace: correct
- test_id: tests/it/scalar_replay_binding.rs::original_scalar_driver_executes_independent_repeated_draws_and_binds_real_qsl
  ac_id: FR-032-AC-16
  trace: correct
- test_id: tests/it/scalar_replay_binding.rs::original_scalar_driver_executes_independent_repeated_draws_and_binds_real_qsl
  ac_id: FR-032-AC-17
  trace: correct
- test_id: tests/it/scalar_replay_binding.rs::scalar_actual_sent_context_and_every_driver_tie_are_independently_checked
  ac_id: FR-032-AC-17
  trace: correct
- test_id: src/replay/scalar_authority.rs::tests::absent_or_ambiguous_scalar_authority_never_invents_an_occurrence
  ac_id: FR-032-AC-18
  trace: correct
- test_id: tests/it/scalar_replay_binding.rs::original_scalar_driver_executes_independent_repeated_draws_and_binds_real_qsl
  ac_id: FR-032-AC-18
  trace: correct
- test_id: tests/it/scalar_replay_binding.rs::scalar_actual_sent_context_and_every_driver_tie_are_independently_checked
  ac_id: FR-032-AC-18
  trace: correct
- test_id: tests/it/composite_original_eq.rs::original_parameter_operands_build_positional_identity_and_public_verified_report
  ac_id: FR-033-AC-1
  trace: correct
- test_id: tests/it/composite_original_eq.rs::original_literal_operands_retain_graph_children_and_empty_bounds
  ac_id: FR-033-AC-1
  trace: correct
- test_id: tests/it/composite_original_eq.rs::original_ne_parameter_operands_keep_order_and_operator
  ac_id: FR-033-AC-1
  trace: correct
- test_id: tests/it/composite_original_eq.rs::original_ne_literal_operand_keeps_empty_bounds
  ac_id: FR-033-AC-1
  trace: correct
- test_id: tests/it/composite_original_eq.rs::original_operator_mismatch_refuses_with_typed_operators
  ac_id: FR-033-AC-1
  trace: correct
- test_id: tests/it/composite_original_eq.rs::repeated_requests_keep_each_nodes_original_operation
  ac_id: FR-033-AC-1
  trace: correct
- test_id: tests/it/composite_original_eq.rs::wrong_source_refuses_before_invocation
  ac_id: FR-033-AC-1
  trace: correct
- test_id: tests/it/composite_original_eq.rs::different_semantic_package_refuses_before_invocation
  ac_id: FR-033-AC-1
  trace: correct
- test_id: tests/it/composite_original_eq.rs::retained_selection_and_recompiled_context_mismatch_refuse
  ac_id: FR-033-AC-1
  trace: correct
- test_id: tests/it/composite_original_eq.rs::wrong_node_and_occurrence_refuse_before_invocation
  ac_id: FR-033-AC-1
  trace: correct
- test_id: tests/it/composite_original_eq.rs::another_functions_node_reaches_binding_checked_qsl_refusal
  ac_id: FR-033-AC-1
  trace: correct
- test_id: tests/it/composite_original_eq.rs::original_operand_projection_obeys_work_ceiling
  ac_id: FR-033-AC-1
  trace: correct
- test_id: tests/it/composite_original_eq.rs::original_stage_limit_stops_recompile_before_invocation
  ac_id: FR-033-AC-1
  trace: correct
- test_id: tests/it/composite_original_eq.rs::original_imported_context_refuses_before_invocation
  ac_id: FR-033-AC-1
  trace: correct
- test_id: tests/it/composite_original_eq.rs::retained_context_kr_families_preserve_complete_request_outputs_and_refused_siblings
  ac_id: FR-033-AC-1
  trace: correct
- test_id: tests/it/composite_parity_converter.rs::tc_041_f7_compares_verdict_and_pair_count_then_preserves_agreement
  ac_id: FR-033-AC-7
  trace: correct
- test_id: tests/it/composite_original_eq.rs::original_literal_operands_retain_graph_children_and_empty_bounds
  ac_id: FR-033-AC-11
  trace: correct
- test_id: tests/it/composite_original_eq.rs::original_ne_parameter_operands_keep_order_and_operator
  ac_id: FR-033-AC-11
  trace: correct
- test_id: tests/it/composite_original_eq.rs::original_ne_literal_operand_keeps_empty_bounds
  ac_id: FR-033-AC-11
  trace: correct
- test_id: tests/it/composite_original_eq.rs::self_comparison_keeps_two_positional_arguments
  ac_id: FR-033-AC-11
  trace: correct
- test_id: tests/it/composite_original_eq.rs::unrelated_node_bound_reaches_binding_checked_qsl_refusal
  ac_id: FR-033-AC-11
  trace: correct
- test_id: tests/it/composite_original_eq.rs::original_operand_projection_obeys_work_ceiling
  ac_id: FR-033-AC-11
  trace: correct
- test_id: tests/it/composite_original_eq.rs::duplicate_positional_bound_refuses_without_an_obligation
  ac_id: FR-033-AC-11
  trace: correct
- test_id: tests/it/composite_original_eq.rs::another_functions_node_reaches_binding_checked_qsl_refusal
  ac_id: FR-033-AC-12
  trace: correct
- test_id: tests/it/composite_original_eq.rs::unrelated_node_bound_reaches_binding_checked_qsl_refusal
  ac_id: FR-033-AC-12
  trace: correct
- test_id: tests/it/composite_parity_converter.rs::tc_041_f1_and_f2_preserve_precedence_and_native_causes
  ac_id: FR-033-AC-12
  trace: correct
- test_id: tests/it/composite_parity_converter.rs::tc_041_f1_and_f2_preserve_precedence_and_native_causes
  ac_id: FR-033-AC-13
  trace: correct
- test_id: tests/it/composite_parity_converter.rs::tc_041_f3_to_f6_keep_operand_and_limit_stages_distinct
  ac_id: FR-033-AC-13
  trace: correct
- test_id: tests/it/composite_artifact_binding.rs::original_composite_body_and_actual_emitted_bytes_have_independent_authority
  ac_id: FR-033-AC-18
  trace: correct
- test_id: tests/it/composite_original_eq.rs::original_parameter_operands_build_positional_identity_and_public_verified_report
  ac_id: FR-033-AC-18
  trace: correct
- test_id: tests/it/composite_parity_converter.rs::structural_obligation_does_not_admit_a_changed_operator_body
  ac_id: FR-033-AC-18
  trace: correct
- test_id: tests/it/composite_parity_converter.rs::tc_048_falsified_report_requires_every_sent_identity_member
  ac_id: FR-033-AC-18
  trace: correct
- test_id: tests/it/capability_settlement.rs::tc_030_every_backend_kind_has_a_dispatched_arm
  ac_id: FR-019-AC-1
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_bounded_settlements_correlate_each_request_bound_by_index
  ac_id: FR-019-AC-2
  trace: correct
- test_id: tests/it/capability_settlement.rs::tc_030_two_candidates_with_no_named_backend_settle_ambiguous
  ac_id: FR-019-AC-4
  trace: correct
- test_id: tests/it/capability_settlement.rs::tc_030_three_candidates_settle_ambiguous_in_candidate_order
  ac_id: FR-019-AC-4
  trace: correct
- test_id: tests/it/capability_settlement.rs::tc_030_repeated_backend_identity_refuses_in_both_orders
  ac_id: FR-019-AC-45
  trace: correct
- test_id: tests/it/capability_settlement.rs::tc_030_repeated_descriptors_refuse_the_whole_envelope
  ac_id: FR-019-AC-45
  trace: correct
- test_id: tests/it/capability_settlement.rs::tc_030_domains_only_change_refuses_without_settlements
  ac_id: FR-019-AC-45
  trace: correct
- test_id: tests/it/capability_settlement.rs::tc_030_backend_identities_that_differ_only_in_case_are_distinct
  ac_id: FR-019-AC-46
  trace: correct
- test_id: tests/it/capability_settlement.rs::tc_030_repeated_identity_precedence_and_bytewise_selection
  ac_id: FR-019-AC-46
  trace: correct
- test_id: tests/it/capability_settlement.rs::incomplete_adapter_preserves_present_count_limit_path
  ac_id: FR-019-AC-49
  trace: correct
- test_id: tests/it/capability_settlement.rs::reader_incomplete_projection_preserves_the_exhausted_limit
  ac_id: FR-019-AC-49
  trace: correct
- test_id: tests/it/capability_settlement.rs::oversized_admitted_function_read_stops_the_envelope
  ac_id: FR-019-AC-49
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::admitted_function_settlement_does_not_read_again
  ac_id: FR-019-AC-50
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::admitted_function_settlement_with_context_root_is_supported
  ac_id: FR-019-AC-50
  trace: correct
- test_id: tests/it/capability_settlement.rs::admitted_function_read_keeps_supported_settlement
  ac_id: FR-019-AC-50
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::first_reader_failure_wins_after_an_earlier_supported_item
  ac_id: FR-019-AC-51
  trace: correct
- test_id: tests/it/capability_settlement.rs::genuine_function_backend_refusals_stay_item_level
  ac_id: FR-019-AC-52
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::excluded_position_consumes_no_envelope_item_and_preserves_both_indices
  ac_id: FR-019-AC-53
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::kani_function_after_excluded_position_settles_at_driver_index
  ac_id: FR-019-AC-53
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::excluded_position_before_function_body_preserves_body_driver_index
  ac_id: FR-019-AC-53
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::imported_function_reader_refusal_is_whole_envelope
  ac_id: FR-019-AC-54
  trace: correct
- test_id: tests/it/capability_settlement.rs::reader_refusal_projection_preserves_every_public_field
  ac_id: FR-019-AC-54
  trace: correct
- test_id: tests/it/capability_settlement.rs::named_refusal_adapter_preserves_unknown_version
  ac_id: FR-019-AC-54
  trace: correct
- test_id: tests/it/capability_settlement.rs::refusal_adapter_preserves_selected_document_pointer
  ac_id: FR-019-AC-54
  trace: correct
- test_id: tests/it/capability_settlement.rs::refusal_adapter_preserves_reader_node_locus
  ac_id: FR-019-AC-54
  trace: correct
- test_id: tests/it/capability_settlement.rs::tc_030_a_foreign_capability_vocabulary_refuses_the_carrier
  ac_id: FR-019-AC-7
  trace: correct
- test_id: tests/it/capability_settlement.rs::process_manifest_bounds_keep_absence_distinct_from_present_empty
  ac_id: FR-019-AC-8
  trace: correct
- test_id: tests/it/capability_settlement.rs::process_kind_uses_origin_manifest_modes_and_explicit_bound_kinds
  ac_id: FR-019-AC-11
  trace: correct
- test_id: tests/it/capability_settlement.rs::process_kind_uses_origin_manifest_modes_and_explicit_bound_kinds
  ac_id: FR-019-AC-12
  trace: correct
- test_id: tests/it/capability_settlement.rs::process_unbounded_settlement_checks_only_boundable_domains
  ac_id: FR-019-AC-16
  trace: correct
- test_id: tests/it/capability_settlement.rs::process_unbounded_settlement_checks_only_boundable_domains
  ac_id: FR-019-AC-17
  trace: correct
- test_id: tests/it/capability_settlement.rs::process_manifest_bounds_keep_absence_distinct_from_present_empty
  ac_id: FR-019-AC-18
  trace: correct
- test_id: tests/it/capability_settlement.rs::process_settlement_does_not_resolve_provider_identity_as_an_executable
  ac_id: FR-019-AC-18
  trace: correct
- test_id: tests/it/capability_settlement.rs::process_bounded_coverage_reads_explicit_kind_with_fixed_domain_key
  ac_id: FR-019-AC-19
  trace: correct
- test_id: tests/it/capability_settlement.rs::process_kind_uses_origin_manifest_modes_and_explicit_bound_kinds
  ac_id: FR-019-AC-20
  trace: correct
- test_id: tests/it/capability_settlement.rs::process_manifest_bounds_keep_absence_distinct_from_present_empty
  ac_id: FR-019-AC-42
  trace: correct
- test_id: tests/it/capability_settlement.rs::process_unbounded_settlement_checks_only_boundable_domains
  ac_id: FR-019-AC-21
  trace: correct
- test_id: tests/it/capability_settlement.rs::process_unbounded_settlement_checks_only_boundable_domains
  ac_id: FR-019-AC-22
  trace: correct
- test_id: tests/it/capability_settlement.rs::process_bounded_maximum_does_not_change_domain_coverage
  ac_id: FR-019-AC-23
  trace: correct
- test_id: tests/it/capability_settlement.rs::process_settlement_does_not_resolve_provider_identity_as_an_executable
  ac_id: FR-019-AC-13
  trace: correct
- test_id: src/routed/capability.rs::scalar_cache_tests::two_scalar_families_share_one_package_walk_without_swapping_identity
  ac_id: FR-019-AC-25
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::function_preflight_graph_index_is_shared_only_within_one_package_instance
  ac_id: FR-019-AC-25
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_form_preflight_gates_support_and_preserves_extent_rules
  ac_id: FR-019-AC-25
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_source_call_preflight_binds_callee_and_ordered_arguments
  ac_id: FR-019-AC-25
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_selected_division_preserves_exact_renderer_refusal
  ac_id: FR-019-AC-25
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_state_frame_planned_selectors_keep_the_exact_fr015_refusal
  ac_id: FR-019-AC-25
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_state_frame_refusal_identity_keeps_operation_and_contract_frame_role
  ac_id: FR-019-AC-25
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_supported_bounded_request_keeps_exact_proof_bound_input
  ac_id: FR-019-AC-26
  trace: correct
- test_id: tests/it/capability_settlement.rs::tc_030_the_advertised_mode_table_settles_each_row
  ac_id: FR-019-AC-26
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_form_preflight_gates_support_and_preserves_extent_rules
  ac_id: FR-019-AC-26
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_source_call_preflight_binds_callee_and_ordered_arguments
  ac_id: FR-019-AC-26
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_state_frame_planned_selectors_keep_the_exact_fr015_refusal
  ac_id: FR-019-AC-26
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_form_preflight_gates_support_and_preserves_extent_rules
  ac_id: FR-019-AC-27
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_state_frame_refusal_identity_keeps_operation_and_contract_frame_role
  ac_id: FR-019-AC-27
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_form_refusal_keeps_candidate_and_mode_precedence
  ac_id: FR-019-AC-27
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_state_frame_planned_selectors_keep_the_exact_fr015_refusal
  ac_id: FR-019-AC-33
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_state_frame_refusal_identity_keeps_operation_and_contract_frame_role
  ac_id: FR-019-AC-33
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_form_refusal_keeps_candidate_and_mode_precedence
  ac_id: FR-019-AC-28
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_form_preflight_gates_support_and_preserves_extent_rules
  ac_id: FR-019-AC-29
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_source_call_preflight_binds_callee_and_ordered_arguments
  ac_id: FR-019-AC-29
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_selected_division_preserves_exact_renderer_refusal
  ac_id: FR-019-AC-29
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_form_refusal_keeps_candidate_and_mode_precedence
  ac_id: FR-019-AC-29
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_state_frame_refusal_identity_keeps_operation_and_contract_frame_role
  ac_id: FR-019-AC-30
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_form_identity_does_not_depend_on_display_or_debug_text
  ac_id: FR-019-AC-30
  trace: correct
- test_id: tests/it/capability_settlement.rs::tc_030_the_advertised_mode_table_settles_each_row
  ac_id: FR-019-AC-31
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_form_preflight_gates_support_and_preserves_extent_rules
  ac_id: FR-019-AC-31
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_supported_bounded_request_keeps_exact_proof_bound_input
  ac_id: FR-019-AC-32
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_bounded_settlements_correlate_each_request_bound_by_index
  ac_id: FR-019-AC-32
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::duplicate_expression_uses_each_selected_conversion_context
  ac_id: FR-019-AC-43
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::duplicate_expression_uses_each_selected_conversion_context
  ac_id: FR-019-AC-44
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_form_preflight_gates_support_and_preserves_extent_rules
  ac_id: FR-019-AC-44
  trace: correct
- test_id: tests/it/capability_settlement.rs::a_selected_quantity_conversion_settles_as_operation_not_rendered
  ac_id: FR-019-AC-44
  trace: correct
- test_id: tests/it/capability_settlement.rs::selected_scalar_context_refusal_keeps_validation_precedence
  ac_id: FR-019-AC-44
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_selected_division_preserves_exact_renderer_refusal
  ac_id: FR-019-AC-44
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_routed_preflight_requires_selected_result_occurrences
  ac_id: FR-019-AC-44
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_scalar_family_selectors_require_selected_result
  ac_id: FR-019-AC-44
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_boolean_operator_selector_requires_selected_result
  ac_id: FR-019-AC-44
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_scalar_path_requires_selected_result_for_nonderivable_nodes
  ac_id: FR-019-AC-44
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_routed_kani_items_equal_fr015_output_keyed_by_request_index
  ac_id: FR-022-AC-2
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_bounded_increment_over_a_bounded_parameter_is_supported_with_a_scalar_harness
  ac_id: FR-022-AC-2
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_the_entry_point_takes_no_settlement_input
  ac_id: FR-022-AC-3
  trace: correct
- test_id: src/routed/generate.rs::tests::mismatched_private_kinds_refuse_before_generation
  ac_id: FR-022-AC-4
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_duplicate_index_and_missing_context_refuse_the_call
  ac_id: FR-022-AC-5
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_a_kani_group_refusal_is_returned_unchanged
  ac_id: FR-022-AC-6
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_an_invalid_kani_item_rejects_the_group_with_driver_indexes
  ac_id: FR-022-AC-7
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_a_selected_pair_routed_twice_retains_its_duplicate_refusal
  ac_id: FR-022-AC-7
  trace: correct
- test_id: src/oracle/scalar/derive/tests.rs::tc_024_derivation_checks_the_graph_and_the_lowering_before_the_identity
  ac_id: FR-022-AC-8
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_routed_kani_items_equal_fr015_output_keyed_by_request_index
  ac_id: FR-022-AC-8
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_an_empty_routed_set_returns_an_empty_result
  ac_id: FR-022-AC-8
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_lowering_and_bound_refusals_keep_their_fr015_dispositions
  ac_id: FR-022-AC-8
  trace: correct
- test_id: src/core/naming.rs::tests::equal_stems_take_ordinals_in_key_order_not_arrival_order
  ac_id: FR-022-AC-9
  trace: correct
- test_id: src/core/naming.rs::tests::a_suffixed_name_meeting_another_stem_is_disambiguated_again
  ac_id: FR-022-AC-9
  trace: correct
- test_id: src/core/naming.rs::tests::readable_stems_that_coincide_still_yield_distinct_names
  ac_id: FR-022-AC-9
  trace: correct
- test_id: tests/it/harness_generation.rs::tc_004_clauses_sharing_a_readable_stem_get_distinct_oracle_names
  ac_id: FR-022-AC-9
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_output_is_deterministic_and_independent_of_the_request_index
  ac_id: FR-022-AC-9
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_a_node_gets_identical_bytes_whatever_its_differently_named_siblings
  ac_id: FR-022-AC-9
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_nodes_sharing_a_stem_take_stable_ordinals_in_node_order
  ac_id: FR-022-AC-9
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_bounded_increment_over_a_bounded_parameter_is_supported_with_a_scalar_harness
  ac_id: FR-022-AC-10
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_an_underivable_item_keeps_its_refusal_and_its_siblings_are_unaffected
  ac_id: FR-022-AC-11
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_quantity_conversion_has_an_oracle_and_no_harness
  ac_id: FR-022-AC-11
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_quantity_conversion_requires_authenticated_operation_views
  ac_id: FR-022-AC-11
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_the_returned_claim_map_orders_package_claims_and_refusals
  ac_id: FR-022-AC-12
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_a_selected_pair_routed_twice_retains_its_duplicate_refusal
  ac_id: FR-022-AC-12
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_an_underivable_claim_is_underived_and_carries_the_nodes_identity
  ac_id: FR-022-AC-12
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_bounded_parameters_are_supported_with_a_per_operand_scalar_harness
  ac_id: FR-022-AC-13
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_a_plain_typed_reference_operand_is_requires_bound_with_no_harness
  ac_id: FR-022-AC-13
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_the_oracle_crate_is_returned_and_defines_the_symbol_the_harness_references
  ac_id: FR-022-AC-14
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_an_all_underivable_group_returns_package_crate_and_no_group_returns_none
  ac_id: FR-022-AC-14
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_the_returned_claim_map_file_keeps_underivable_refusals
  ac_id: FR-022-AC-14
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_qsl_shaped_increment_sum_and_negation_are_supported
  ac_id: FR-022-AC-15
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_qsl_shaped_plain_operands_refuse_and_selected_narrowing_generates
  ac_id: FR-022-AC-15
  trace: correct
- test_id: src/routed/generate.rs::tests::tc_033_two_harnesses_sharing_a_symbol_are_a_typed_error_not_a_silent_overwrite
  ac_id: FR-022-AC-16
  trace: correct
- test_id: tests/it/routed_generation.rs::process_items_generate_empty_outputs_in_request_order
  ac_id: FR-022-AC-17
  trace: correct
- test_id: src/routed/generate.rs::tests::mismatched_private_kinds_refuse_before_generation
  ac_id: FR-022-AC-18
  trace: correct
- test_id: tests/it/routed_generation.rs::process_items_generate_empty_outputs_in_request_order
  ac_id: FR-022-AC-19
  trace: correct
- test_id: tests/it/routed_generation.rs::routed_process_construction_preserves_optional_manifest_bounds
  ac_id: FR-022-AC-21
  trace: correct
- test_id: tests/it/routed_generation.rs::process_items_generate_empty_outputs_in_request_order
  ac_id: FR-022-AC-21
  trace: correct
- test_id: tests/it/routed_generation.rs::routed_item_constructor_refuses_mismatched_and_unknown_linked_descriptors
  ac_id: FR-022-AC-22
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::duplicate_expression_uses_each_selected_conversion_context
  ac_id: FR-022-AC-23
  trace: correct
- test_id: tests/it/routed_generation.rs::routed_scalar_selection_must_match_admitted_driver_position
  ac_id: FR-022-AC-23
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::duplicate_expression_uses_each_selected_conversion_context
  ac_id: FR-022-AC-24
  trace: correct
- test_id: tests/it/routed_generation.rs::routed_scalar_selection_must_match_admitted_driver_position
  ac_id: FR-022-AC-24
  trace: correct
- test_id: tests/it/routed_generation.rs::routed_scalar_source_ceiling_is_publicly_matchable
  ac_id: FR-022-AC-25
  trace: correct
- test_id: src/routed/contract_package_tests.rs::tests::routed_excluded_driver_index_is_refused_before_generation
  ac_id: FR-022-AC-29
  trace: correct
- test_id: tests/it/capability_settlement.rs::kani_selected_division_preserves_exact_renderer_refusal
  ac_id: FR-022-AC-30
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_quantity_conversion_has_an_oracle_and_no_harness
  ac_id: FR-022-AC-30
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_quantity_conversion_requires_authenticated_operation_views
  ac_id: FR-022-AC-30
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_unauthenticated_view_refusal_uses_lowest_request_index
  ac_id: FR-022-AC-30
  trace: correct
- test_id: src/routed/smt/pair_tests.rs::two_real_unsat_processes_need_distinct_exact_associations
  ac_id: FR-045-AC-2
  trace: correct
- test_id: src/routed/smt/pair_tests.rs::responses_remain_nonterminal_evidence
  ac_id: FR-045-AC-2
  trace: correct
- test_id: src/routed/smt/pair_tests.rs::completed_run_retains_exact_selection_limits_and_raw_output
  ac_id: FR-045-AC-2
  trace: correct
- test_id: src/routed/smt/pair_tests.rs::excess_input_and_malformed_output_never_become_unsat
  ac_id: FR-045-AC-2
  trace: correct
- test_id: src/kani/run/process.rs::tests::exact_input_reaches_one_direct_child_and_returns_whole_output
  ac_id: FR-045-AC-3
  trace: correct
- test_id: src/kani/run/process.rs::tests::input_excess_refuses_before_selected_program_can_start
  ac_id: FR-045-AC-3
  trace: correct
- test_id: src/kani/run/process.rs::tests::missing_selected_executable_refuses_without_starting_child
  ac_id: FR-045-AC-3
  trace: correct
- test_id: src/kani/run/process.rs::tests::relative_executable_is_a_typed_selection_error
  ac_id: FR-045-AC-3
  trace: correct
- test_id: src/kani/run/process.rs::tests::output_excess_stops_with_only_a_bounded_raw_prefix
  ac_id: FR-045-AC-3
  trace: correct
- test_id: src/kani/run/process.rs::tests::caller_cancellation_refuses_before_selected_program_starts
  ac_id: FR-045-AC-3
  trace: correct
- test_id: src/kani/run/process.rs::tests::nonzero_exit_keeps_status_and_whole_captured_streams
  ac_id: FR-045-AC-3
  trace: correct
- test_id: src/kani/run/process.rs::tests::child_exit_before_consuming_input_refuses_as_unread_stdin
  ac_id: FR-045-AC-3
  trace: correct
- test_id: src/kani/run/process.rs::tests::running_child_stops_at_its_wall_clock_limit
  ac_id: FR-045-AC-3
  trace: correct
- test_id: src/kani/run/process.rs::tests::overflowing_wall_clock_limit_refuses_before_spawn
  ac_id: FR-045-AC-3
  trace: correct
- test_id: src/kani/run/process.rs::tests::timed_out_child_retains_bounded_raw_output_without_a_success_status
  ac_id: FR-045-AC-3
  trace: correct
- test_id: src/kani/run/process.rs::tests::owned_child_tree_stops_when_memory_limit_is_reached
  ac_id: FR-045-AC-3
  trace: correct
- test_id: src/routed/smt/pair_tests.rs::two_real_unsat_processes_need_distinct_exact_associations
  ac_id: FR-045-AC-3
  trace: correct
- test_id: src/routed/smt/pair_tests.rs::completed_run_retains_exact_selection_limits_and_raw_output
  ac_id: FR-045-AC-3
  trace: correct
- test_id: src/routed/smt/pair_tests.rs::excess_input_and_malformed_output_never_become_unsat
  ac_id: FR-045-AC-3
  trace: correct
- test_id: src/routed/smt/refinement.rs::tests::canonical_roles_and_exact_fields_are_order_independent
  ac_id: FR-045-AC-4
  trace: correct
- test_id: src/routed/smt/refinement.rs::tests::shape_association_and_missing_fields_refuse_handoff
  ac_id: FR-045-AC-4
  trace: correct
- test_id: src/routed/smt/refinement.rs::tests::shape_association_and_missing_fields_refuse_handoff
  ac_id: FR-045-AC-6
  trace: correct
- test_id: src/routed/smt/refinement.rs::tests::shape_association_and_missing_fields_refuse_handoff
  ac_id: FR-045-AC-7
  trace: correct
- test_id: src/routed/smt/refinement.rs::tests::caller_supplied_proof_cannot_be_handed_off_as_emitted_proof
  ac_id: FR-045-AC-7
  trace: correct
- test_id: tests/it/harness_generation.rs::tc_004_generated_harness_binds_clauses_and_executes_all_three_terminal_paths
  ac_id: FR-002-AC-1
  trace: correct
- test_id: tests/it/strategy_generation.rs::tc_004_boundary_campaigns_tag_inside_and_immediately_outside_values
  ac_id: FR-002-AC-2
  trace: correct
- test_id: tests/it/strategy_generation.rs::tc_004_supported_populations_are_directly_shaped_and_shrink_inside_constraints
  ac_id: FR-002-AC-3
  trace: correct
- test_id: tests/it/strategy_generation.rs::tc_004_supported_populations_are_directly_shaped_and_shrink_inside_constraints
  ac_id: FR-002-AC-4
  trace: correct
- test_id: tests/it/harness_generation.rs::tc_004_generated_harness_binds_clauses_and_executes_all_three_terminal_paths
  ac_id: FR-002-AC-5
  trace: correct
- test_id: tests/it/harness_generation.rs::tc_004_generated_harness_binds_clauses_and_executes_all_three_terminal_paths
  ac_id: FR-002-AC-6
  trace: correct
- test_id: tests/it/strategy_generation.rs::tc_004_supported_populations_are_directly_shaped_and_shrink_inside_constraints
  ac_id: FR-002-AC-6
  trace: correct
- test_id: tests/it/strategy_generation.rs::tc_004_boundary_pinned_and_no_event_bodies_compile_and_execute_with_denied_warnings
  ac_id: FR-002-AC-6
  trace: correct
- test_id: tests/it/strategy_generation.rs::tc_004_customer_enum_memberships_are_directly_shaped_and_validated
  ac_id: FR-002-AC-6
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_017_bound_admission_uses_the_public_clause_and_domain
  ac_id: FR-008-AC-1
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_017_bound_admission_uses_the_public_clause_and_domain
  ac_id: FR-008-AC-2
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::wide_ir_domain_refuses_the_i64_strategy
  ac_id: FR-008-AC-3
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_017_bound_admission_uses_the_public_clause_and_domain
  ac_id: FR-008-AC-3
  trace: correct
- test_id: tests/it/oracle_arithmetic.rs::tc_044_a_refused_sibling_clause_refuses_the_strategy_of_every_clause_in_its_package
  ac_id: FR-008-AC-3
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_017_bound_admission_uses_the_public_clause_and_domain
  ac_id: FR-008-AC-4
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_017_bound_admission_uses_the_public_clause_and_domain
  ac_id: FR-008-AC-5
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::wide_ir_domain_refuses_the_i64_strategy
  ac_id: FR-008-AC-6
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_017_bound_admission_uses_the_public_clause_and_domain
  ac_id: FR-008-CON-2
  trace: correct
- test_id: src/strategy/bound/generation.rs::tests::tc_017_direct_relation_admission_refuses_wide_integer_domain
  ac_id: FR-008-CON-3
  trace: correct
- test_id: src/strategy/bound/generation.rs::tests::tc_017_direct_literal_conversion_refuses_outside_i64
  ac_id: FR-008-CON-4
  trace: correct
- test_id: tests/it/bound_populations.rs::tc_018_tc_021_generated_populations_compile_draw_and_shrink_inside_their_sides
  ac_id: FR-009-AC-1
  trace: correct
- test_id: tests/it/bound_populations.rs::tc_018_value_sets_equal_independent_enumeration_and_empty_sides_refuse
  ac_id: FR-009-AC-2
  trace: correct
- test_id: tests/it/bound_populations.rs::tc_018_tc_021_generated_populations_compile_draw_and_shrink_inside_their_sides
  ac_id: FR-009-AC-2
  trace: correct
- test_id: tests/it/bound_populations.rs::tc_018_generated_sources_are_constructive_and_byte_identical
  ac_id: FR-009-AC-3
  trace: correct
- test_id: tests/it/bound_populations.rs::tc_018_tc_021_generated_populations_compile_draw_and_shrink_inside_their_sides
  ac_id: FR-009-AC-3
  trace: correct
- test_id: tests/it/bound_populations.rs::tc_018_amount_below_zero_and_at_or_above_zero_refuse_the_empty_side
  ac_id: FR-009-AC-4
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_018_public_generation_refuses_every_population_with_an_empty_required_side
  ac_id: FR-009-AC-4
  trace: correct
- test_id: tests/it/v2_numeric_population.rs::selected_v2_empty_side_returns_typed_no_population_refusal
  ac_id: FR-009-AC-4
  trace: correct
- test_id: tests/it/bound_populations.rs::tc_018_full_i64_domain_value_sets_use_checked_edges_without_overflow
  ac_id: FR-009-AC-5
  trace: correct
- test_id: tests/it/bound_populations.rs::tc_018_tc_021_generated_populations_compile_draw_and_shrink_inside_their_sides
  ac_id: FR-009-AC-5
  trace: correct
- test_id: tests/it/bound_populations.rs::tc_018_generated_sources_are_constructive_and_byte_identical
  ac_id: FR-009-AC-6
  trace: correct
- test_id: tests/it/bound_census.rs::tc_019_literal_census_for_amount_less_than_seven_is_exact
  ac_id: FR-010-AC-1
  trace: correct
- test_id: tests/it/bound_census.rs::tc_019_literal_edges_coinciding_with_domain_edges_are_deduplicated
  ac_id: FR-010-AC-1
  trace: correct
- test_id: tests/it/bound_census.rs::tc_019_rendered_census_constants_compile_with_denied_warnings
  ac_id: FR-010-AC-1
  trace: correct
- test_id: tests/it/bound_census.rs::tc_019_two_read_census_for_version_unchanged_is_exact
  ac_id: FR-010-AC-2
  trace: correct
- test_id: tests/it/bound_census.rs::tc_019_rendered_census_constants_compile_with_denied_warnings
  ac_id: FR-010-AC-2
  trace: correct
- test_id: tests/it/bound_census.rs::tc_019_full_i64_domain_lists_outer_edges_as_unrepresentable
  ac_id: FR-010-AC-3
  trace: correct
- test_id: tests/it/bound_census.rs::tc_019_rendered_census_constants_compile_with_denied_warnings
  ac_id: FR-010-AC-3
  trace: correct
- test_id: tests/it/bound_census.rs::tc_019_single_tagged_census_refuses_boundary_but_keeps_edge_arrays
  ac_id: FR-010-AC-4
  trace: correct
- test_id: tests/it/bound_census.rs::tc_019_exhaustive_sweep_tags_domains_order_size_and_determinism
  ac_id: FR-010-AC-5
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_020_tc_021_tc_022_generated_consumer_runs_sampled_and_census_campaigns
  ac_id: FR-011-AC-1
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_020_all_clause_kinds_and_populations_run_without_discards
  ac_id: FR-011-AC-1
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_020_tc_021_negated_oracle_returns_a_structured_minimal_mismatch
  ac_id: FR-011-AC-2
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_020_tc_021_runner_rates_census_and_replay_surface_are_generated
  ac_id: FR-011-AC-3
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_020_tc_021_tc_022_generated_consumer_runs_sampled_and_census_campaigns
  ac_id: FR-011-AC-3
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_020_tc_021_runner_rates_census_and_replay_surface_are_generated
  ac_id: FR-011-AC-4
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_020_tc_021_tc_022_generated_consumer_runs_sampled_and_census_campaigns
  ac_id: FR-011-AC-4
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_020_all_clause_kinds_and_populations_run_without_discards
  ac_id: FR-011-AC-4
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_020_tc_021_tc_022_generated_consumer_runs_sampled_and_census_campaigns
  ac_id: FR-011-AC-5
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_020_all_clause_kinds_and_populations_run_without_discards
  ac_id: FR-011-AC-5
  trace: correct
- test_id: tests/it/v2_selected_claim_entry.rs::v2_selected_claim_entry_keeps_point_and_exact_refusal_key
  ac_id: FR-011-AC-13
  trace: correct
- test_id: tests/it/v2_point_renderer.rs::distinct_pointed_invariants_render_distinct_verdict_points_and_identities
  ac_id: FR-011-AC-14
  trace: correct
- test_id: tests/it/v2_point_renderer.rs::pre_and_post_claims_render_their_accepted_operation_points
  ac_id: FR-011-AC-14
  trace: correct
- test_id: tests/it/v2_point_renderer.rs::two_authentic_claims_of_one_pointed_node_share_the_point
  ac_id: FR-011-AC-14
  trace: correct
- test_id: tests/it/v2_point_renderer.rs::changed_or_unpointed_selection_returns_typed_no_artifact_refusal
  ac_id: FR-011-AC-15
  trace: correct
- test_id: tests/it/v2_numeric_strategy.rs::v2_numeric_bundle_keeps_selected_identity_and_runs_both_modes
  ac_id: FR-011-AC-16
  trace: correct
- test_id: tests/it/v2_numeric_strategy.rs::v2_numeric_bundle_admits_all_six_authored_comparisons
  ac_id: FR-011-AC-16
  trace: correct
- test_id: src/strategy/v2_numeric.rs::tests::renderer_mutants_refuse_before_any_artifact_exists
  ac_id: FR-011-AC-17
  trace: correct
- test_id: tests/it/v2_numeric_strategy.rs::v2_numeric_bundle_keeps_selected_identity_and_runs_both_modes
  ac_id: FR-011-AC-17
  trace: correct
- test_id: tests/it/v2_numeric_strategy.rs::v2_numeric_refusals_keep_first_typed_cause
  ac_id: FR-011-AC-17
  trace: correct
- test_id: tests/it/bound_populations.rs::tc_018_tc_021_generated_populations_compile_draw_and_shrink_inside_their_sides
  ac_id: FR-012-AC-1
  trace: correct
- test_id: tests/it/bound_populations.rs::tc_018_tc_021_generated_populations_compile_draw_and_shrink_inside_their_sides
  ac_id: FR-012-AC-2
  trace: correct
- test_id: tests/it/bound_populations.rs::tc_018_tc_021_generated_populations_compile_draw_and_shrink_inside_their_sides
  ac_id: FR-012-AC-3
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_020_tc_021_runner_rates_census_and_replay_surface_are_generated
  ac_id: FR-012-AC-4
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_020_tc_021_tc_022_generated_consumer_runs_sampled_and_census_campaigns
  ac_id: FR-012-AC-4
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_020_tc_021_negated_oracle_returns_a_structured_minimal_mismatch
  ac_id: FR-012-AC-4
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_020_tc_021_tc_022_generated_consumer_runs_sampled_and_census_campaigns
  ac_id: FR-013-AC-1
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_022_bundle_is_typed_rust_with_bound_identity
  ac_id: FR-013-AC-1
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_022_bundle_is_typed_rust_with_bound_identity
  ac_id: FR-013-AC-2
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_022_bundle_is_typed_rust_with_bound_identity
  ac_id: FR-013-AC-4
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_022_bundle_is_typed_rust_with_bound_identity
  ac_id: FR-013-AC-5
  trace: correct
- test_id: tests/it/bound_strategy_generation.rs::tc_020_all_clause_kinds_and_populations_run_without_discards
  ac_id: NFR-004-AC-1
  trace: correct
- test_id: tests/it/bound_census.rs::tc_019_exhaustive_sweep_tags_domains_order_size_and_determinism
  ac_id: NFR-004-AC-2
  trace: correct
```
