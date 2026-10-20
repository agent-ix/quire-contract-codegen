---
id: SR-14180
title: IR631 decoder code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; IR-631 PR480; src/replay/scalar.rs, src/lib.rs,
  src/replay/mod.rs, src/kani/output/playback.rs, src/replay/witness.rs, src/kani/identity.rs,
  src/kani/generate/scalar.rs, src/oracle/operator_catalog.rs, tests/it/layout.rs,
  tests/it/main.rs
review_set: subset
---

## Summary

Ticket IR-631, PR480, bounded scalar decoder foundation. Two current-delta blocking findings.

## Verdict

FAIL

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new cfg(test) fixture imports routed from replay, violating AD-004 L-2 and causing l_2_every_import_follows_the_dependency_direction to fail in make ci: may_import("replay") allows only kani/core, and edges() scans cfg(test) bodies. Move the real QSL-to-IR-to-routed/public-decoder tests to a permitted integration or routed boundary without weakening the scanner or identity assertions. | src/replay/scalar.rs:129 |
| FND-002 | high | The decoder adds a second string-to-operator and operand-arity map for the four integer scalar operations. src/oracle/operator_catalog.rs already owns those identities, ScalarKaniOperator and OperandOrder, consumed by src/kani/generate/scalar.rs::ScalarOperation::of/operand_names. A catalog identity/arity change can update generation while leaving replay stale. Reuse the owning typed scalar projection through a dependency-direction-compliant seam instead of maintaining this parallel match. | src/replay/scalar.rs:41 |

## Coverage

Reviewed the whole three-file delta (586 added lines), full production decoder and all eight new tests against actual playback selection/reader, existing function decoder, scalar identity and renderer. No dependency/lock, spec, unsafe, lints, CI, proof domain/cap or existing parser changes. The public free function takes ScalarObligationIdentity directly and returns existing DecodeFailure plus ordered i64 values; no function-identity coercion, local arithmetic evaluator, QSL replay, new wire/receipt or copied parser. Production access follows replay->kani and core. Operation/schema checks precede the fixed 8 MiB byte ceiling; adapter parsing precedes domain validation. Required arity bounds identity iteration to one/two arguments before BTreeSet allocation. All transcript-derived adapter allocations are bounded by 8 MiB; usize::MAX value count preserves extra-draw arity refusal under that byte cap. Exact 8-byte conversion uses try_into; signed values use from_le_bytes. Decimal comments must agree, all entries decode before first persisted-order domain refusal, and inclusive original argument bounds are used. No new request-path unwrap/index panic, casts, unsafe, async/blocking lifecycle or mutable global state is introduced.

The fixture compiles authored source with qsl_replay::compile_package, admits CheckedPackageV2, selects authentic expression occurrences, lowers and admits ContractPackageWireV1, calls actual generate_routed and extracts the returned harness identity. No hand-built identity-only happy path substitutes for generation. Independent literal expected operands and distinct ranges discriminate swapped draw order, inclusive endpoints, wrong width/comment, wrong harness, missing/extra draws, unsupported operator/arity/range/duplicate identifier, byte limit and precedence. Literal singleton has its own draw; negate and full signed subtraction endpoints decode without evaluating the arithmetic. Repeated source-parameter operands are not a separate fixture here; inspection of the owning renderer confirms it persists distinct left/right draw identifiers for each operand position, so the new duplicate-generated-name guard does not deduplicate graph parameter identities. The tests are synthetic playback-text parser tests, not actual Kani scalar mutation/replay acceptance. New trace attributes apply only to actual tests.

Two actionable current-delta defects remain. First, the genuine fixture sits at the wrong architectural boundary; same tests must survive its move. Second, the new operator spelling/arity map duplicates the typed catalog source. Searched src/oracle/operator_catalog.rs, src/kani/generate/scalar.rs and related consumers; Operator::entry has the shared identities, OperandOrder and ScalarKaniOperator, and ScalarOperation::of/operand_names consumes it. Do not fix either defect with a new allow, root-import dodge, synthetic-only fixture, weakened test or AD-004 exception.

Full static module inventory includes all 93 src Rust paths and 1,126 lexical crate-path edges, cfg(test) bodies, grouped imports and macro tokens (module-graph.json). The new fixture adds eight routed symbol edges resolving to routed/capability.rs and routed/generate.rs. Transitive routed generation reaches kani/oracle/core and has no return path to replay/scalar; no resolved file cycle is found. This is a direction violation, not an invented cycle. super::* in the inline scalar tests resolves to its parent production decoder; there are no new #[path] source inclusions or alias tricks. Existing path-included fixture modules were inspected via tests/it/main.rs and the AD004 scanner; no extra module, allocator, visibility or duplicate-mod suppression is added. The static scan is not compiler expansion evidence.

Frozen source: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2; actual base: 4b578b4557b0e31f42880c09d63d7d58d2db4581; ticket IR-631; PR quire-contract-codegen#480. Runtime reviewer model: gpt-6.1-sol. Source, spec, tests and branch remained read-only; only external review artifacts were written. No applicable AssuranceProfile exists under spec/. No subagents were used. No normative specification or plan file changes: spec-review and normative subanalyses are inapplicable and skipped; no padding SRs.

Heavy checks by this reviewer: UNRUN (explicit NO HEAVY allocation). Actual author logs were inspected: focused decoder plus existing function playback tests PASS 23/23 (8 scalar, 15 existing); final make lint fmt-check PASS. Actual author make kani-gate PASS at the exact frozen source, 37/37, 878s, Kani 0.68.0, clean tree. Initial duplicate-mod and invalid full-range result fixture failures were fixed before these receipts; the current fixture really compiles authored QSL and uses emitted typed IR/routed generation. Those receipts do not include aggregate make ci. Root must schedule aggregate and distinct final-source full Kani after fixes and custody; this review does not run or predict compiler success.

This is the independently useful decoder/domain foundation only. No public scalar request builder, original-context/proving-record retention/authentication, falsified-artifact native producer, driver checked observation, shared QSL parity mint/actual replay, immutable full-report comparison, converter, settlement or TC-047 end-to-end acceptance is delivered. FR-032-AC-2/3 tags are partial evidence: the builder boundary and QSL input/context obligations remain unimplemented. IR-631 stays In Progress. Verified-only scalar native receipts are not reused for Falsified. No completed scalar proof/replay is claimed.

All scope excerpts are nonempty verbatim source slices at the recorded source_head/path/excerpt_start_line; slices longer than 600 characters are explicitly marked verbatim_prefix. Source inventory and structural bindings establish ownership/presence only. They are not wholesale semantic acceptance or proof discharge. The expanded optional whole-repository semantic review was not commissioned; focused changed decoder intent/fixture review was commissioned and performed.

## Examined units and bindings

```yaml
scope:
- id: FR-032-AC-2
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: PLANNED (IR-631). The builder decodes actual routed-harness playback in persisted symbolic call orde
  excerpt_start_line: 383
  excerpt: PLANNED (IR-631). The builder decodes actual routed-harness playback in persisted symbolic call order through the
    Kani output adapter; another harness name, missing/extra operand, wrong byte width, playback above the builder-owned 8
    MiB per-harness ceiling or malformed assertion block yields the distinct typed refusal under the stated precedence before
    replay.
  type: criterion
- id: FR-032-AC-3
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: 'PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). Each operand at '
  excerpt_start_line: 384
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). Each operand at its own lower
    or upper bound is admitted and the adjacent out-of-domain value is refused before replay. The admitted QSL inputs preserve
    the catalogued operator, operands, operand ranges, result range and harness limits. Another node/operation, altered operand
    schema or replay limit differing from the retained original generation context refuses; operand correspondence uses [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
    AC-16.
  type: criterion
- id: FR-038-AC-1
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: A source inventory compares every identity in [FR-014](../../oracle/functional/FR-014-exact-scalar-o
  excerpt_start_line: 68
  excerpt: 'A source inventory compares every identity in [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md)''s
    specified derivable set with the implemented scalar, scalar Kani and state/frame selections, recording the unimplemented
    scalar integer `eq/ne` and Boolean variants as open FR-014 work. For the four integer arithmetic identities shared by
    scalar oracle and scalar Kani, and the four integer ordering identities shared by scalar oracle and state/frame Kani,
    the implemented consumers resolve the same typed identity, family, arity and ordered operands; no second string-to-operator
    match '
  excerpt_kind: verbatim_prefix
  type: criterion
- id: FR-032-AC-1
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). A public consume
  excerpt_start_line: 382
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). A public consumer passes the
    routed harness's `ScalarObligationIdentity` directly to the scalar builder; its typed call constructs neither `KaniObligationIdentity`
    nor V1 `ReplayPackage`. After the ordered setup checks, an otherwise valid request returns unsupported-capability only
    while a named step-5 capability is unavailable; when all are available it returns an admitted plan, and authenticated
    completion enables replay/conversion. Conversion of a capability refusal returns no settlement rather than a synthe
  excerpt_kind: verbatim_prefix
  type: criterion
- id: FR-032-AC-4
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). The driver's act
  excerpt_start_line: 385
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). The driver's actual native
    observation of the proved generated oracle is required and tied to this operand vector, limits and canonical proof-content
    identity; the checked completion requires driver authentication; a bare outcome/digest echo, missing observation, another
    artifact's observation or fresh regeneration that removes the arithmetic mutation refuses before QSL evaluation. Playback
    bytes and Kani falsification alone cannot supply a generated outcome.
  type: criterion
- id: FR-032-AC-5
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). The public consu
  excerpt_start_line: 386
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). The public consumer executes
    the same proved generated artifact and invokes QSL between the builder and converter. QSL compares its actual native observation
    with authoritative exact operator evaluation using the four-operator projection owned by [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
    AC-37, without adding refusal-cause or accounting-counter equality; changing the generated arithmetic while keeping original
    node/operator identity changes the measured parity result. `Undefined`, `Inco
  excerpt_kind: verbatim_prefix
  type: criterion
- id: FR-032-AC-6
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). For every `Diver
  excerpt_start_line: 387
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). For every `Diverged`, `Agrees`,
    `GeneratedFault`, `RefusedInput`, exact `Incomplete` and other `Refused` outcome, the converter compares the report's
    full claim to the retained sent claim and actual sent wire against the original proving context before conversion; a changed
    package/function/source/dependency or original limit refuses even with an equal claim. At the immutable-report/retained-binding
    boundary, an absent report, another result/run, or changing only the obligation, node, occurrence, kind, canonica
  excerpt_kind: verbatim_prefix
  type: criterion
- id: FR-032-AC-8
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). The real QSL-emi
  excerpt_start_line: 389
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). The real QSL-emitted `x + 1`
    with input `Int[0,9]` and result `Int[0,10]` exercises routed generation, actual Kani arithmetic-mutation playback and
    same-proved-artifact native observation. A separate bounded-addition fixture with both operands and result in `[-1000,1000]`
    exercises an admitted operand pair whose exact result is outside the result range. The expected exact result is derived
    from the actual retained playback, and its correct refusal is never labelled a source violation. The fixture ranges and
    wit
  excerpt_kind: verbatim_prefix
  type: criterion
- id: FR-032-AC-9
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: PLANNED (IR-631) / CODE-GATED (authoritative metadata access and CG retention). Independent canonica
  excerpt_start_line: 390
  excerpt: PLANNED (IR-631) / CODE-GATED (authoritative metadata access and CG retention). Independent canonical-preimage
    checks include only the claimed application node, authentic expression occurrence, authoritative scalar kind and one argument
    per operand position in actual draw order, each with position ordinal, typed `OperandIdentity` and harness operand range.
    Actual graph references use `GraphChild { node_id }`; inline literals use `InlineLiteral { node_id, occurrence_key, position
    }` from the authentic application/occurrence and position, with their value in the singleton range. QSL emits no ext
  excerpt_kind: verbatim_prefix
  type: criterion
- id: FR-017-AC-14
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: A launcher whose stdout or stderr carries more than 8 MiB times its harness count is stopped, its pr
  excerpt_start_line: 837
  excerpt: A launcher whose stdout or stderr carries more than 8 MiB times its harness count is stopped, its process group
    is killed, and the run is refused with `kani_output_over_limit` naming the stream, the limit and the count, with no outcome
    and no retained text; a launcher whose stream is exactly the limit completes with its real exit status and the whole stream
    retained. No stream is ever truncated and then classified.
  type: criterion
- id: NFR-006-AC-1
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: context_only
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: '`make kani-scope` prints `required` and the path for each of `src/kani/generate/negotiate.rs`, `src/'
  excerpt_start_line: 179
  excerpt: '`make kani-scope` prints `required` and the path for each of `src/kani/generate/negotiate.rs`, `src/oracle/boolean_v1.rs`,
    `src/routed/generate.rs`, `src/replay/witness.rs`, `src/core/identity.rs`, `src/publication/mod.rs`, `tests/it/kani_batching.rs`,
    `tests/it/skeleton_spine.rs`, `tests/it/bounded_kani_corpus.rs`, `tests/it/kani_obligations_state_frame.rs`, `tests/it/scratch_crate.rs`,
    `tests/exact_scalar_support/package.rs`, `tests/checked_package_support/base.rs`, `tests/checked_package_support/rekey.rs`,
    `tests/state_frame_support/model.rs`, `schemas/kani-proof-graph-v2.schema.json`, `sch'
  excerpt_kind: verbatim_prefix
  type: criterion
- id: src/kani/generate/scalar.rs
  path: src/kani/generate/scalar.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: '//! The scalar family: lowering an IR-confirmed V2 exact-scalar claim and rendering its Kani'
  excerpt_start_line: 1
  excerpt: '//! The scalar family: lowering an IR-confirmed V2 exact-scalar claim and rendering its Kani'
  owning_requirements: &id001
  - FR-015
  - FR-017
  - FR-025
  - FR-028
  - FR-029
  - FR-030
  - FR-034
  - FR-040
  - FR-041
  examination: module ownership and stub inventory
- id: src/kani/generate/scalar.rs::visible-function-1
  path: src/kani/generate/scalar.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: 'pub(super) fn scalar_stem(operation: &str) -> String {'
  excerpt_start_line: 180
  excerpt: 'pub(super) fn scalar_stem(operation: &str) -> String {'
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/generate/scalar.rs::visible-function-2
  path: src/kani/generate/scalar.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: pub(super) fn lower_scalar_claim(
  excerpt_start_line: 223
  excerpt: pub(super) fn lower_scalar_claim(
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/generate/scalar.rs::visible-function-3
  path: src/kani/generate/scalar.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: 'pub(super) fn derive_domain(bound: &CheckedSemanticNodeV2) -> DerivedDomain {'
  excerpt_start_line: 333
  excerpt: 'pub(super) fn derive_domain(bound: &CheckedSemanticNodeV2) -> DerivedDomain {'
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/generate/scalar.rs::visible-function-4
  path: src/kani/generate/scalar.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: 'pub(super) fn unsatisfiable(bound: &CheckedSemanticNodeV2) -> Option<(String, String)> {'
  excerpt_start_line: 360
  excerpt: 'pub(super) fn unsatisfiable(bound: &CheckedSemanticNodeV2) -> Option<(String, String)> {'
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/generate/scalar.rs::visible-function-5
  path: src/kani/generate/scalar.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: pub(super) fn render_scalar(
  excerpt_start_line: 409
  excerpt: pub(super) fn render_scalar(
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/generate/scalar.rs::visible-function-6
  path: src/kani/generate/scalar.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: pub(super) fn false_bound_harness_for_test(
  excerpt_start_line: 642
  excerpt: pub(super) fn false_bound_harness_for_test(
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/identity.rs
  path: src/kani/identity.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: //! The harness and identity record types of a generated Kani obligation.
  excerpt_start_line: 1
  excerpt: //! The harness and identity record types of a generated Kani obligation.
  owning_requirements: *id001
  examination: module ownership and stub inventory
- id: src/kani/identity.rs::visible-function-1
  path: src/kani/identity.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: pub(crate) fn generated(
  excerpt_start_line: 322
  excerpt: pub(crate) fn generated(
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/identity.rs::visible-function-2
  path: src/kani/identity.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: pub(crate) fn has_valid_provenance(&self) -> bool {
  excerpt_start_line: 339
  excerpt: pub(crate) fn has_valid_provenance(&self) -> bool {
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/identity.rs::visible-function-3
  path: src/kani/identity.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: pub(crate) fn generated(
  excerpt_start_line: 379
  excerpt: pub(crate) fn generated(
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/identity.rs::visible-function-4
  path: src/kani/identity.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: pub(crate) fn has_valid_provenance(&self) -> bool {
  excerpt_start_line: 397
  excerpt: pub(crate) fn has_valid_provenance(&self) -> bool {
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/identity.rs::visible-function-5
  path: src/kani/identity.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: 'pub(crate) fn bind_artifact_package(&mut self, artifacts: &[Artifact]) {'
  excerpt_start_line: 404
  excerpt: 'pub(crate) fn bind_artifact_package(&mut self, artifacts: &[Artifact]) {'
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/identity.rs::visible-function-6
  path: src/kani/identity.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: 'pub(crate) fn matches_artifact_package(&self, artifacts: &[Artifact]) -> bool {'
  excerpt_start_line: 412
  excerpt: 'pub(crate) fn matches_artifact_package(&self, artifacts: &[Artifact]) -> bool {'
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/identity.rs::visible-function-7
  path: src/kani/identity.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: pub fn harness_path(&self) -> HarnessPath {
  excerpt_start_line: 425
  excerpt: pub fn harness_path(&self) -> HarnessPath {
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/identity.rs::visible-function-8
  path: src/kani/identity.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: pub fn harness_path(&self) -> HarnessPath {
  excerpt_start_line: 436
  excerpt: pub fn harness_path(&self) -> HarnessPath {
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/identity.rs::visible-function-9
  path: src/kani/identity.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: pub fn harness_path(&self) -> HarnessPath {
  excerpt_start_line: 631
  excerpt: pub fn harness_path(&self) -> HarnessPath {
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/identity.rs::visible-function-10
  path: src/kani/identity.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: 'pub fn from_record(record: &str) -> Result<Self, StateFrameRecordError> {'
  excerpt_start_line: 646
  excerpt: 'pub fn from_record(record: &str) -> Result<Self, StateFrameRecordError> {'
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/output/playback.rs
  path: src/kani/output/playback.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: //! The concrete-playback block Kani prints on the console.
  excerpt_start_line: 1
  excerpt: //! The concrete-playback block Kani prints on the console.
  owning_requirements: *id001
  examination: module ownership and stub inventory
- id: src/kani/output/playback.rs::visible-function-1
  path: src/kani/output/playback.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: 'pub(crate) fn playback_blocks(text: &str) -> Vec<PlaybackBlock<''_>> {'
  excerpt_start_line: 32
  excerpt: 'pub(crate) fn playback_blocks(text: &str) -> Vec<PlaybackBlock<''_>> {'
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/output/playback.rs::visible-function-2
  path: src/kani/output/playback.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: 'pub(crate) fn counterexample_playback(text: &str, harness: Option<&str>) -> Option<String> {'
  excerpt_start_line: 64
  excerpt: 'pub(crate) fn counterexample_playback(text: &str, harness: Option<&str>) -> Option<String> {'
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/output/playback.rs::visible-function-3
  path: src/kani/output/playback.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: 'pub(crate) fn new(code: &str, source_id: &str, context: &str) -> Self {'
  excerpt_start_line: 87
  excerpt: 'pub(crate) fn new(code: &str, source_id: &str, context: &str) -> Self {'
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/output/playback.rs::visible-function-4
  path: src/kani/output/playback.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: pub(crate) fn select_assertion_block<'a>(
  excerpt_start_line: 150
  excerpt: pub(crate) fn select_assertion_block<'a>(
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/output/playback.rs::visible-function-5
  path: src/kani/output/playback.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: pub(crate) fn read_block<'a>(
  excerpt_start_line: 201
  excerpt: pub(crate) fn read_block<'a>(
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: //! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.
  excerpt_start_line: 1
  excerpt: //! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.
  owning_requirements:
  - AD-004
  - interface-001
  examination: module ownership and stub inventory
- id: src/oracle/operator_catalog.rs
  path: src/oracle/operator_catalog.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: //! Closed typed identities consumed by admitted V2 generators.
  excerpt_start_line: 1
  excerpt: //! Closed typed identities consumed by admitted V2 generators.
  owning_requirements: &id002
  - FR-014
  - FR-018
  - FR-021
  - FR-035
  - FR-038
  - FR-039
  examination: module ownership and stub inventory
- id: src/oracle/operator_catalog.rs::visible-function-1
  path: src/oracle/operator_catalog.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: pub(crate) const fn rust(self) -> &'static str {
  excerpt_start_line: 78
  excerpt: pub(crate) const fn rust(self) -> &'static str {
  owning_requirements: *id002
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/oracle/operator_catalog.rs::visible-function-2
  path: src/oracle/operator_catalog.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: pub(crate) const fn entry(self) -> Entry {
  excerpt_start_line: 112
  excerpt: pub(crate) const fn entry(self) -> Entry {
  owning_requirements: *id002
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/oracle/operator_catalog.rs::visible-function-3
  path: src/oracle/operator_catalog.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: 'pub(crate) fn from_identity(identity: &str) -> Option<Self> {'
  excerpt_start_line: 237
  excerpt: 'pub(crate) fn from_identity(identity: &str) -> Option<Self> {'
  owning_requirements: *id002
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/oracle/operator_catalog.rs::visible-function-4
  path: src/oracle/operator_catalog.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: 'pub(crate) fn from_ordering_operator(operator: OrderingOperator) -> Option<Self> {'
  excerpt_start_line: 243
  excerpt: 'pub(crate) fn from_ordering_operator(operator: OrderingOperator) -> Option<Self> {'
  owning_requirements: *id002
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/replay/mod.rs
  path: src/replay/mod.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: '//! The replay subsystem: decoding a Kani witness and replaying it natively (AD-004, FR-016,'
  excerpt_start_line: 1
  excerpt: '//! The replay subsystem: decoding a Kani witness and replaying it natively (AD-004, FR-016,'
  owning_requirements: &id003
  - FR-016
  - FR-024
  - FR-032
  - FR-033
  examination: module ownership and stub inventory
- id: src/replay/scalar.rs
  path: src/replay/scalar.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: //! Bounded scalar playback decoding and operand-domain validation (FR-032-AC-2/3).
  excerpt_start_line: 4
  excerpt: //! Bounded scalar playback decoding and operand-domain validation (FR-032-AC-2/3).
  owning_requirements: *id003
  examination: module ownership and stub inventory
- id: src/replay/scalar.rs::visible-function-1
  path: src/replay/scalar.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: pub fn decode_scalar_falsification(
  excerpt_start_line: 34
  excerpt: pub fn decode_scalar_falsification(
  owning_requirements: *id003
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/replay/witness.rs
  path: src/replay/witness.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: //! Reads a real Kani concrete-playback transcript against the generator's own persisted
  excerpt_start_line: 1
  excerpt: //! Reads a real Kani concrete-playback transcript against the generator's own persisted
  owning_requirements: *id003
  examination: module ownership and stub inventory
- id: src/replay/witness.rs::visible-function-1
  path: src/replay/witness.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: pub(crate) const fn unbounded_decode_limits() -> WitnessDecodeLimits {
  excerpt_start_line: 61
  excerpt: pub(crate) const fn unbounded_decode_limits() -> WitnessDecodeLimits {
  owning_requirements: *id003
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/replay/witness.rs::visible-function-2
  path: src/replay/witness.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: pub fn decode_falsification(
  excerpt_start_line: 145
  excerpt: pub fn decode_falsification(
  owning_requirements: *id003
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/replay/witness.rs::visible-function-3
  path: src/replay/witness.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: pub(crate) fn decode_playback<'a>(
  excerpt_start_line: 172
  excerpt: pub(crate) fn decode_playback<'a>(
  owning_requirements: *id003
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/replay/witness.rs::visible-function-4
  path: src/replay/witness.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: pub(crate) fn first_out_of_domain<'a>(
  excerpt_start_line: 290
  excerpt: pub(crate) fn first_out_of_domain<'a>(
  owning_requirements: *id003
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: AD-004-L2-test-direction
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: '- A `#[cfg(test)]` module obeys the same direction as the file it sits in.'
  excerpt_start_line: 377
  excerpt: "- A `#[cfg(test)]` module obeys the same direction as the file it sits in. A test that needs a\n  generator as\
    \ a fixture lives in `tests/it/`, not in the runner's file."
- id: scalar-decoder-routed-test-imports
  path: src/replay/scalar.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: '        routed::{'
  excerpt_start_line: 129
  excerpt: "        routed::{\n            capability::{BackendDescriptor, Candidate, ProviderOrigin},\n            generate::{\n\
    \                generate_routed, GenerationContexts, KaniGenerationContext, KindOutput,\n                RoutedGenerationItem,\n\
    \            },\n        },\n    };"
- id: scalar-decoder-string-operator-map
  path: src/replay/scalar.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: '    let arity = match identity.operation_identity.as_str() {'
  excerpt_start_line: 41
  excerpt: "    let arity = match identity.operation_identity.as_str() {\n        \"quire.op.integer.add\" | \"quire.op.integer.sub\"\
    \ | \"quire.op.integer.mul\" => 2,\n        \"quire.op.integer.negate\" => 1,\n        operation => {\n            return\
    \ Err(failure(\n                \"cg_scalar_witness_operation_unsupported\",\n                operation,\n           \
    \ ))\n        }\n    };"
- id: typed-scalar-operation-source
  path: src/oracle/operator_catalog.rs
  role: examined
  source_head: 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2
  anchor: '            O::Add => ('
  excerpt_start_line: 124
  excerpt: "            O::Add => (\n                \"quire.op.integer.add\",\n                Family::IntegerArithmetic,\n\
    \                OperandOrder::LeftThenRight,\n                ResultKind::Integer,\n                Some(IntegerOperator::Add),\n\
    \                Some(ScalarKaniOperator::Add),\n                None,\n                None,\n            ),\n      \
    \      O::Subtract => (\n                \"quire.op.integer.sub\",\n                Family::IntegerArithmetic,\n     \
    \           OperandOrder::LeftThenRight,\n                ResultKind::Integer,\n                Some(IntegerOperator::Subtract),\n\
    \                So"
  excerpt_kind: verbatim_prefix
bindings:
- test_id: src/replay/scalar.rs::tests::routed_draw_order_is_preserved_with_distinct_domains
  ac_id: FR-032-AC-2
  trace: correct
  path: src/replay/scalar.rs
  line: 314
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: src/replay/scalar.rs::tests::negate_and_literal_singleton_keep_every_draw
  ac_id: FR-032-AC-2
  trace: correct
  path: src/replay/scalar.rs
  line: 353
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: src/replay/scalar.rs::tests::subtraction_decodes_signed_extreme_operands_without_evaluating_them
  ac_id: FR-032-AC-2
  trace: correct
  path: src/replay/scalar.rs
  line: 395
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: src/replay/scalar.rs::tests::harness_arity_width_and_comment_failures_are_distinct
  ac_id: FR-032-AC-2
  trace: correct
  path: src/replay/scalar.rs
  line: 438
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: src/replay/scalar.rs::tests::assertion_selection_and_malformed_blocks_use_adapter_refusals
  ac_id: FR-032-AC-2
  trace: correct
  path: src/replay/scalar.rs
  line: 482
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: src/replay/scalar.rs::tests::schema_then_byte_ceiling_then_decode_then_domain_precedence
  ac_id: FR-032-AC-2
  trace: correct
  path: src/replay/scalar.rs
  line: 529
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: src/replay/scalar.rs::tests::unsupported_operator_and_invalid_persisted_schema_refuse_before_parsing
  ac_id: FR-032-AC-2
  trace: correct
  path: src/replay/scalar.rs
  line: 553
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: src/replay/scalar.rs::tests::routed_draw_order_is_preserved_with_distinct_domains
  ac_id: FR-032-AC-3
  trace: correct
  path: src/replay/scalar.rs
  line: 314
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: src/replay/scalar.rs::tests::negate_and_literal_singleton_keep_every_draw
  ac_id: FR-032-AC-3
  trace: correct
  path: src/replay/scalar.rs
  line: 353
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: src/replay/scalar.rs::tests::subtraction_decodes_signed_extreme_operands_without_evaluating_them
  ac_id: FR-032-AC-3
  trace: correct
  path: src/replay/scalar.rs
  line: 395
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: src/replay/scalar.rs::tests::inclusive_endpoints_and_adjacent_values_use_each_operands_range
  ac_id: FR-032-AC-3
  trace: correct
  path: src/replay/scalar.rs
  line: 412
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: src/replay/scalar.rs::tests::schema_then_byte_ceiling_then_decode_then_domain_precedence
  ac_id: FR-032-AC-3
  trace: correct
  path: src/replay/scalar.rs
  line: 529
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: src/replay/scalar.rs::tests::unsupported_operator_and_invalid_persisted_schema_refuse_before_parsing
  ac_id: FR-032-AC-3
  trace: correct
  path: src/replay/scalar.rs
  line: 553
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
```

## Dispositions

Round 1.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | All eight genuine source-emission/IR/routed tests moved to the existing integration binary and invoke the public decoder. The production replay file has no routed edge; the actual author layout lane passes unchanged. |
| FND-002 | fixed | Replay delegates to the internal Kani ABI seam; it resolves the existing owning Operator catalog, filters typed ScalarKaniOperator variants exhaustively and reads typed OperandOrder. No duplicate identity-string map remains. |

## Disposition round 1 coverage

Current delta disposition: PASS; both current-delta findings fixed, no new findings. Original FAIL verdict remains in its historical section above.

Inspected the complete six-file final PR and fix delta against both original source and rebased main. All eight original test functions and all 16 textual assertion invocations, including helper assertions, survive byte-for-byte modulo indentation/formatting, plus two extra unsupported known catalog operators lt/eq. The authored QSL compile -> CheckedPackageV2 admission -> routed lower/admit -> actual generate_routed -> returned ScalarObligationIdentity path remains intact and calls the public decode_scalar_falsification export in tests/it/scalar_playback.rs, registered once in tests/it/main.rs. Shared proof ceilings come from the established integration common helper. No #[path] fixture source inclusion, duplicate-mod suppression, allocator change, root-import dodge, gate/lint weakening or synthetic-only downgrade was introduced.

The new internal Kani ABI seam imports only oracle/operator_catalog, permitted by kani->oracle; replay imports only Kani. Operator::from_identity/entry remains the sole identity map; scalar_kani? refuses known integer comparisons, and the exhaustive typed Add/Subtract/Multiply/Negate projection retains the four-operation profile. OperandOrder::Unary/LeftThenRight owns one/two positions. Boolean implication, integer div, known catalog lt/eq and unknown identities are tested as unsupported. Existing generation scalar projection/catalog controls remain meaningful. No public API/protocol or free-string identity mapping was added by the fix. The current parser body otherwise retains its precedence, bounded adapter parsing, qualified harness, arity/width/comment checks, signed LE decode and inclusive per-position domain validation.

Fresh full source graph inventory includes all 93 source files and 1,118 lexical crate-path edges including cfg(test), grouped uses and macro tokens; no resolved file cycle appears. The old eight routed symbol edges from replay/scalar are gone. New replay->kani/abi->oracle/operator_catalog terminates in the independent owning catalog and creates no backedge. Existing layout scanner is unchanged and its actual author eight-test lane PASS is attributed correctly. No source compiler or graph expansion was run by reviewer.

Disposition round 1; frozen source 1f49bed0469407016bb50f01d3e3c9732995df4e; actual rebased main 22a6ccf5dfe9fe1b9a1c5fe90521285914fb17ef; original reviewed source 0e9623e7cd887616b903cc0a6e2ab68ad6b1d3c2. Same independent reviewer/model gpt-6.1-sol, new disposition run fb5928bc-a3ec-4bcd-bf12-0cc56fe99c84. Original Findings, Verdicts, complete artifact prefixes and original 23 private review comments are preserved unchanged. No new finding was found in this round. No source/spec/test/branch edits or GitHub comments were made by reviewer.

Reviewer compiler/backend checks: UNRUN, explicit NO HEAVY. Actual author receipts inspected at this fixed source: make lint fmt-check PASS; actual integration scalar+layout PASS 16/16 (all eight genuine scalar parser tests plus eight layout tests); existing witness/catalog/generation-profile library controls PASS 17/17. The prior full author make kani-gate PASS 37/37, 878s, belongs only to original 0e9623e and is not attributed to this fixed head. No refreshed full Kani or aggregate has run at this stage. Root alone allocates final aggregate and separate full Kani after latest exact custody.

This remains partial IR631. Public builder/original proving-context retention and authentication, falsified native observation, QSL mint/actual execution/full-report converter/settlement and TC047 remain unimplemented; full AC2/3 is not accepted from parser tags. IR-631 remains In Progress. No normative spec/plan delta against rebased main; normative review methods remain inapplicable. Optional expanded whole-repository semantic acceptance was not commissioned. Full scope/bindings below records static ownership/presence and the focused changed behavior, not all historical semantics or proof discharge. All excerpts are current-source nonempty verbatim slices <=600 characters with exact path/head/line; longer units use a declared verbatim prefix.

## Disposition round 1 full scope, bindings and outcomes

```yaml
scope:
- id: FR-032-AC-2
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: PLANNED (IR-631). The builder decodes actual routed-harness playback in persisted symbolic call orde
  excerpt_start_line: 383
  excerpt: PLANNED (IR-631). The builder decodes actual routed-harness playback in persisted symbolic call order through the
    Kani output adapter; another harness name, missing/extra operand, wrong byte width, playback above the builder-owned 8
    MiB per-harness ceiling or malformed assertion block yields the distinct typed refusal under the stated precedence before
    replay.
  type: criterion
- id: FR-032-AC-3
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: 'PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). Each operand at '
  excerpt_start_line: 384
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). Each operand at its own lower
    or upper bound is admitted and the adjacent out-of-domain value is refused before replay. The admitted QSL inputs preserve
    the catalogued operator, operands, operand ranges, result range and harness limits. Another node/operation, altered operand
    schema or replay limit differing from the retained original generation context refuses; operand correspondence uses [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
    AC-16.
  type: criterion
- id: FR-038-AC-1
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: A source inventory compares every identity in [FR-014](../../oracle/functional/FR-014-exact-scalar-o
  excerpt_start_line: 68
  excerpt: 'A source inventory compares every identity in [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md)''s
    specified derivable set with the implemented scalar, scalar Kani and state/frame selections, recording the unimplemented
    scalar integer `eq/ne` and Boolean variants as open FR-014 work. For the four integer arithmetic identities shared by
    scalar oracle and scalar Kani, and the four integer ordering identities shared by scalar oracle and state/frame Kani,
    the implemented consumers resolve the same typed identity, family, arity and ordered operands; no second string-to-operator
    match '
  excerpt_kind: verbatim_prefix
  type: criterion
- id: src/kani/abi.rs
  path: src/kani/abi.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: '//! The Kani subject ABI vocabulary: binding roles, primitive types, integer bounds, the solver'
  excerpt_start_line: 1
  excerpt: '//! The Kani subject ABI vocabulary: binding roles, primitive types, integer bounds, the solver'
  owning_requirements: &id001
  - FR-015
  - FR-017
  - FR-025
  - FR-028
  - FR-029
  - FR-030
  - FR-034
  - FR-040
  - FR-041
  examination: module ownership and stub inventory
- id: src/kani/abi.rs::visible-function-1
  path: src/kani/abi.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: 'pub(crate) fn integer_scalar_operand_count(identity: &str) -> Option<usize> {'
  excerpt_start_line: 13
  excerpt: 'pub(crate) fn integer_scalar_operand_count(identity: &str) -> Option<usize> {'
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/abi.rs::visible-function-2
  path: src/kani/abi.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: pub(crate) const fn source_name(self) -> &'static str {
  excerpt_start_line: 48
  excerpt: pub(crate) const fn source_name(self) -> &'static str {
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/abi.rs::visible-function-3
  path: src/kani/abi.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: 'pub(crate) fn from_model(value: &IntegerType) -> Option<Self> {'
  excerpt_start_line: 72
  excerpt: 'pub(crate) fn from_model(value: &IntegerType) -> Option<Self> {'
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/abi.rs::visible-function-4
  path: src/kani/abi.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: pub(crate) const fn as_str(self) -> &'static str {
  excerpt_start_line: 91
  excerpt: pub(crate) const fn as_str(self) -> &'static str {
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/abi.rs::visible-function-5
  path: src/kani/abi.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: 'pub(crate) fn i64_literal(value: i64) -> String {'
  excerpt_start_line: 98
  excerpt: 'pub(crate) fn i64_literal(value: i64) -> String {'
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/abi.rs::visible-function-6
  path: src/kani/abi.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: pub(crate) fn adapter_options(
  excerpt_start_line: 106
  excerpt: pub(crate) fn adapter_options(
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/abi.rs::visible-function-7
  path: src/kani/abi.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: 'pub(crate) fn readable_component(value: &str) -> String {'
  excerpt_start_line: 135
  excerpt: 'pub(crate) fn readable_component(value: &str) -> String {'
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/generate/scalar.rs
  path: src/kani/generate/scalar.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: '//! The scalar family: lowering an IR-confirmed V2 exact-scalar claim and rendering its Kani'
  excerpt_start_line: 1
  excerpt: '//! The scalar family: lowering an IR-confirmed V2 exact-scalar claim and rendering its Kani'
  owning_requirements: *id001
  examination: module ownership and stub inventory
- id: src/kani/generate/scalar.rs::visible-function-1
  path: src/kani/generate/scalar.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: 'pub(super) fn scalar_stem(operation: &str) -> String {'
  excerpt_start_line: 180
  excerpt: 'pub(super) fn scalar_stem(operation: &str) -> String {'
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/generate/scalar.rs::visible-function-2
  path: src/kani/generate/scalar.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: pub(super) fn lower_scalar_claim(
  excerpt_start_line: 223
  excerpt: pub(super) fn lower_scalar_claim(
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/generate/scalar.rs::visible-function-3
  path: src/kani/generate/scalar.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: 'pub(super) fn derive_domain(bound: &CheckedSemanticNodeV2) -> DerivedDomain {'
  excerpt_start_line: 333
  excerpt: 'pub(super) fn derive_domain(bound: &CheckedSemanticNodeV2) -> DerivedDomain {'
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/generate/scalar.rs::visible-function-4
  path: src/kani/generate/scalar.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: 'pub(super) fn unsatisfiable(bound: &CheckedSemanticNodeV2) -> Option<(String, String)> {'
  excerpt_start_line: 360
  excerpt: 'pub(super) fn unsatisfiable(bound: &CheckedSemanticNodeV2) -> Option<(String, String)> {'
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/generate/scalar.rs::visible-function-5
  path: src/kani/generate/scalar.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: pub(super) fn render_scalar(
  excerpt_start_line: 409
  excerpt: pub(super) fn render_scalar(
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/kani/generate/scalar.rs::visible-function-6
  path: src/kani/generate/scalar.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: pub(super) fn false_bound_harness_for_test(
  excerpt_start_line: 642
  excerpt: pub(super) fn false_bound_harness_for_test(
  owning_requirements: *id001
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: //! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.
  excerpt_start_line: 1
  excerpt: //! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.
  owning_requirements:
  - AD-004
  - interface-001
  examination: module ownership and stub inventory
- id: src/oracle/operator_catalog.rs
  path: src/oracle/operator_catalog.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: //! Closed typed identities consumed by admitted V2 generators.
  excerpt_start_line: 1
  excerpt: //! Closed typed identities consumed by admitted V2 generators.
  owning_requirements: &id002
  - FR-014
  - FR-018
  - FR-021
  - FR-035
  - FR-038
  - FR-039
  examination: module ownership and stub inventory
- id: src/oracle/operator_catalog.rs::visible-function-1
  path: src/oracle/operator_catalog.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: pub(crate) const fn rust(self) -> &'static str {
  excerpt_start_line: 78
  excerpt: pub(crate) const fn rust(self) -> &'static str {
  owning_requirements: *id002
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/oracle/operator_catalog.rs::visible-function-2
  path: src/oracle/operator_catalog.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: pub(crate) const fn entry(self) -> Entry {
  excerpt_start_line: 112
  excerpt: pub(crate) const fn entry(self) -> Entry {
  owning_requirements: *id002
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/oracle/operator_catalog.rs::visible-function-3
  path: src/oracle/operator_catalog.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: 'pub(crate) fn from_identity(identity: &str) -> Option<Self> {'
  excerpt_start_line: 237
  excerpt: 'pub(crate) fn from_identity(identity: &str) -> Option<Self> {'
  owning_requirements: *id002
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/oracle/operator_catalog.rs::visible-function-4
  path: src/oracle/operator_catalog.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: 'pub(crate) fn from_ordering_operator(operator: OrderingOperator) -> Option<Self> {'
  excerpt_start_line: 243
  excerpt: 'pub(crate) fn from_ordering_operator(operator: OrderingOperator) -> Option<Self> {'
  owning_requirements: *id002
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: src/replay/mod.rs
  path: src/replay/mod.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: '//! The replay subsystem: decoding a Kani witness and replaying it natively (AD-004, FR-016,'
  excerpt_start_line: 1
  excerpt: '//! The replay subsystem: decoding a Kani witness and replaying it natively (AD-004, FR-016,'
  owning_requirements: &id003
  - FR-016
  - FR-024
  - FR-032
  - FR-033
  examination: module ownership and stub inventory
- id: src/replay/scalar.rs
  path: src/replay/scalar.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: //! Bounded scalar playback decoding and operand-domain validation (FR-032-AC-2/3).
  excerpt_start_line: 4
  excerpt: //! Bounded scalar playback decoding and operand-domain validation (FR-032-AC-2/3).
  owning_requirements: *id003
  examination: module ownership and stub inventory
- id: src/replay/scalar.rs::visible-function-1
  path: src/replay/scalar.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: pub fn decode_scalar_falsification(
  excerpt_start_line: 35
  excerpt: pub fn decode_scalar_falsification(
  owning_requirements: *id003
  examination: textual public/crate-visible function ownership inventory; not semantic or runtime acceptance
- id: integration-scalar-public-imports
  path: tests/it/scalar_playback.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: use quire_contract_codegen::{
  excerpt_start_line: 9
  excerpt: "use quire_contract_codegen::{\n    decode_scalar_falsification, generate_routed, routed_contract_lowering_profile,\n\
    \    BackendDescriptor, Candidate, ContractDriverResultSelection, ContractRequestPosition,\n    DecodeFailure, GenerationContexts,\
    \ KaniGenerationContext, KindOutput, ProviderOrigin,\n    RoutedGenerationItem, ScalarObligationIdentity,\n};"
- id: replay-scalar-imports-fixed
  path: src/replay/scalar.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: use crate::kani::{
  excerpt_start_line: 8
  excerpt: "use crate::kani::{\n    abi::integer_scalar_operand_count,\n    identity::ScalarObligationIdentity,\n    output::playback::{read_block,\
    \ select_assertion_block, DecodeFailure},\n};"
- id: scalar-operation-count-call-fixed
  path: src/replay/scalar.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: '    let arity = integer_scalar_operand_count'
  excerpt_start_line: 42
  excerpt: "    let arity = integer_scalar_operand_count(&identity.operation_identity).ok_or_else(|| {\n        failure(\n\
    \            \"cg_scalar_witness_operation_unsupported\",\n            &identity.operation_identity,\n        )\n    })?;"
- id: catalog-backed-operand-count
  path: src/kani/abi.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: pub(crate) fn integer_scalar_operand_count
  excerpt_start_line: 13
  excerpt: "pub(crate) fn integer_scalar_operand_count(identity: &str) -> Option<usize> {\n    let entry = Operator::from_identity(identity)?.entry();\n\
    \    match entry.scalar_kani? {\n        ScalarKaniOperator::Add\n        | ScalarKaniOperator::Subtract\n        | ScalarKaniOperator::Multiply\n\
    \        | ScalarKaniOperator::Negate => {}\n    }\n    Some(match entry.operands {\n        OperandOrder::Unary => 1,\n\
    \        OperandOrder::LeftThenRight => 2,\n    })\n}"
- id: integration-test-registration
  path: tests/it/main.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: mod scalar_playback;
  excerpt_start_line: 80
  excerpt: mod scalar_playback;
- id: tests/it/scalar_playback.rs::routed_draw_order_is_preserved_with_distinct_domains
  path: tests/it/scalar_playback.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: fn routed_draw_order_is_preserved_with_distinct_domains() {
  excerpt_start_line: 192
  excerpt: fn routed_draw_order_is_preserved_with_distinct_domains() {
- id: tests/it/scalar_playback.rs::negate_and_literal_singleton_keep_every_draw
  path: tests/it/scalar_playback.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: fn negate_and_literal_singleton_keep_every_draw() {
  excerpt_start_line: 231
  excerpt: fn negate_and_literal_singleton_keep_every_draw() {
- id: tests/it/scalar_playback.rs::subtraction_decodes_signed_extreme_operands_without_evaluating_them
  path: tests/it/scalar_playback.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: fn subtraction_decodes_signed_extreme_operands_without_evaluating_them() {
  excerpt_start_line: 273
  excerpt: fn subtraction_decodes_signed_extreme_operands_without_evaluating_them() {
- id: tests/it/scalar_playback.rs::harness_arity_width_and_comment_failures_are_distinct
  path: tests/it/scalar_playback.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: fn harness_arity_width_and_comment_failures_are_distinct() {
  excerpt_start_line: 316
  excerpt: fn harness_arity_width_and_comment_failures_are_distinct() {
- id: tests/it/scalar_playback.rs::assertion_selection_and_malformed_blocks_use_adapter_refusals
  path: tests/it/scalar_playback.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: fn assertion_selection_and_malformed_blocks_use_adapter_refusals() {
  excerpt_start_line: 360
  excerpt: fn assertion_selection_and_malformed_blocks_use_adapter_refusals() {
- id: tests/it/scalar_playback.rs::schema_then_byte_ceiling_then_decode_then_domain_precedence
  path: tests/it/scalar_playback.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: fn schema_then_byte_ceiling_then_decode_then_domain_precedence() {
  excerpt_start_line: 407
  excerpt: fn schema_then_byte_ceiling_then_decode_then_domain_precedence() {
- id: tests/it/scalar_playback.rs::unsupported_operator_and_invalid_persisted_schema_refuse_before_parsing
  path: tests/it/scalar_playback.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: fn unsupported_operator_and_invalid_persisted_schema_refuse_before_parsing() {
  excerpt_start_line: 431
  excerpt: fn unsupported_operator_and_invalid_persisted_schema_refuse_before_parsing() {
- id: tests/it/scalar_playback.rs::inclusive_endpoints_and_adjacent_values_use_each_operands_range
  path: tests/it/scalar_playback.rs
  role: examined
  source_head: 1f49bed0469407016bb50f01d3e3c9732995df4e
  anchor: fn inclusive_endpoints_and_adjacent_values_use_each_operands_range() {
  excerpt_start_line: 290
  excerpt: fn inclusive_endpoints_and_adjacent_values_use_each_operands_range() {
bindings:
- test_id: tests/it/scalar_playback.rs::routed_draw_order_is_preserved_with_distinct_domains
  ac_id: FR-032-AC-2
  trace: correct
  path: tests/it/scalar_playback.rs
  line: 192
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: tests/it/scalar_playback.rs::negate_and_literal_singleton_keep_every_draw
  ac_id: FR-032-AC-2
  trace: correct
  path: tests/it/scalar_playback.rs
  line: 231
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: tests/it/scalar_playback.rs::subtraction_decodes_signed_extreme_operands_without_evaluating_them
  ac_id: FR-032-AC-2
  trace: correct
  path: tests/it/scalar_playback.rs
  line: 273
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: tests/it/scalar_playback.rs::harness_arity_width_and_comment_failures_are_distinct
  ac_id: FR-032-AC-2
  trace: correct
  path: tests/it/scalar_playback.rs
  line: 316
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: tests/it/scalar_playback.rs::assertion_selection_and_malformed_blocks_use_adapter_refusals
  ac_id: FR-032-AC-2
  trace: correct
  path: tests/it/scalar_playback.rs
  line: 360
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: tests/it/scalar_playback.rs::schema_then_byte_ceiling_then_decode_then_domain_precedence
  ac_id: FR-032-AC-2
  trace: correct
  path: tests/it/scalar_playback.rs
  line: 407
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: tests/it/scalar_playback.rs::unsupported_operator_and_invalid_persisted_schema_refuse_before_parsing
  ac_id: FR-032-AC-2
  trace: correct
  path: tests/it/scalar_playback.rs
  line: 431
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: tests/it/scalar_playback.rs::routed_draw_order_is_preserved_with_distinct_domains
  ac_id: FR-032-AC-3
  trace: correct
  path: tests/it/scalar_playback.rs
  line: 192
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: tests/it/scalar_playback.rs::negate_and_literal_singleton_keep_every_draw
  ac_id: FR-032-AC-3
  trace: correct
  path: tests/it/scalar_playback.rs
  line: 231
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: tests/it/scalar_playback.rs::subtraction_decodes_signed_extreme_operands_without_evaluating_them
  ac_id: FR-032-AC-3
  trace: correct
  path: tests/it/scalar_playback.rs
  line: 273
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: tests/it/scalar_playback.rs::inclusive_endpoints_and_adjacent_values_use_each_operands_range
  ac_id: FR-032-AC-3
  trace: correct
  path: tests/it/scalar_playback.rs
  line: 290
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: tests/it/scalar_playback.rs::schema_then_byte_ceiling_then_decode_then_domain_precedence
  ac_id: FR-032-AC-3
  trace: correct
  path: tests/it/scalar_playback.rs
  line: 407
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
- test_id: tests/it/scalar_playback.rs::unsupported_operator_and_invalid_persisted_schema_refuse_before_parsing
  ac_id: FR-032-AC-3
  trace: correct
  path: tests/it/scalar_playback.rs
  line: 431
  kind: test_function
  coverage: decoder/domain subset only; no completed builder/QSL route credit
findings: []
dispositions:
- fnd: FND-001
  outcome: fixed
  after_excerpt: "use crate::kani::{\n    abi::integer_scalar_operand_count,\n    identity::ScalarObligationIdentity,\n  \
    \  output::playback::{read_block, select_assertion_block, DecodeFailure},\n};"
  after_path: src/replay/scalar.rs
  after_excerpt_start_line: 8
  reason: All eight genuine source-emission/IR/routed tests moved to the existing integration binary and invoke the public
    decoder. The production replay file has no routed edge; the actual author layout lane passes unchanged.
- fnd: FND-002
  outcome: fixed
  after_excerpt: "    let arity = integer_scalar_operand_count(&identity.operation_identity).ok_or_else(|| {\n        failure(\n\
    \            \"cg_scalar_witness_operation_unsupported\",\n            &identity.operation_identity,\n        )\n    })?;"
  after_path: src/replay/scalar.rs
  after_excerpt_start_line: 42
  reason: Replay delegates to the internal Kani ABI seam; it resolves the existing owning Operator catalog, filters typed
    ScalarKaniOperator variants exhaustively and reads typed OperandOrder. No duplicate identity-string map remains.
```
