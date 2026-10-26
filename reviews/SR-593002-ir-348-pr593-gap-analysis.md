---
id: SR-593002
title: "IR-348 PR #593 gap-analysis (L-6 typed witness and scalar replay slice)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; PR #593 head 3ee11b66a9ed8ec6abd3d59b1d154a06e993844d on merge-base 68fafff7 (origin/main 8db3554e); src/replay/scalar.rs, src/replay/witness.rs, src/lib.rs, tests/it/scalar_playback.rs, tests/it/kani_obligations_state_frame.rs, tests/it/kani_scalar_native_private.rs against spec/assurance/AD-004-cg-crate-layout.md (L-6, Interim exceptions) and spec/replay/functional/FR-032-routed-scalar-replay-binding.md (AC-2, Setup Refusal Precedence); IR-348"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
---

## Summary

Ticket: IR-348. This is a planless gap analysis of the PR diff, so plan completion was not
assessed. The PR closes two of the five L-6 residual rows in the AD-004 Interim exceptions
table: the V2 clause witness entry and the scalar replay entry. It does not claim to close L-6
or IR-348, and this review does not assess the IR-348 criteria as a whole.

Inventory check, measured at head:

- The only `parse_assertion_playback` call sites in `src/replay/` outside tests are
  `function.rs:1327`, `function.rs:1412`, `state_clause.rs:534` and `frame.rs:356`. Those are
  exactly the three residual rows that remain. No text-taking replay entry is missing from the
  inventory, and none of the remaining rows is already closed.
- No code without an owning requirement was added. The diff only removes code or changes a
  parameter type.
- Traceability: `quire trace --id FR-032-AC-2` resolves seven `#[trace]` claims in
  `tests/it/scalar_playback.rs`, including the two tests this PR renamed. FR-032 marks every AC
  PLANNED (IR-631).

The method ran on Quire tooling only (`quire trace`, `quire validate`). No `quoin matrix` or
Cargo run was made, per the brief.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Two renamed tests still carry `#[trace("FR-032-AC-2", ...)]`, but no longer test the precedence that AC-2 requires ("the distinct typed refusal under the stated precedence"). FR-032's Setup Refusal Precedence puts schema coherence (1) before the byte ceiling (2), and the byte ceiling before adapter parsing (3). Before this PR, `schema_then_byte_ceiling_then_decode_then_domain_precedence` checked that an invalid schema plus an oversized transcript refuses with the schema code. `unsupported_operator_and_invalid_persisted_schema_refuse_before_parsing` checked that a schema refusal beats an unparseable `"no block"` input. Both inputs are now replaced by valid at-limit transcripts, so neither ordering is asserted. With the caller parsing first, an invalid schema plus an oversized transcript now yields `cg_witness_decode_bytes_limit`. The precedence is an IR-631 builder obligation, and the PR body says so, but FR-032 and AD-004 do not record that this slice removed the interim evidence. Either retag the two tests so they no longer claim AC-2's precedence clause, or record in FR-032's implementation status that schema-before-parse ordering has no implementation or test until the IR-631 builder lands. | tests/it/scalar_playback.rs:429-451; tests/it/scalar_playback.rs:453-487; spec/replay/functional/FR-032-routed-scalar-replay-binding.md:355-364; spec/replay/functional/FR-032-routed-scalar-replay-binding.md:383 |

## Verdict

The AD-004 inventory edit matches the code exactly. The two closed residual rows are genuinely
closed, and the three that remain are genuinely open. One medium trace/evidence gap: FR-032-AC-2's
precedence clause loses its only interim tests, and nothing records this. Mergeable after FND-001
is fixed or explicitly recorded. SR-593001 FND-001 (the decoder's untested own ceiling) is the
related code-level gap.

## Coverage

```yaml
scope:
- id: AD-004:L-6
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "Public or replay-facing entries in `replay/function.rs`, `replay/state_clause.rs` and `replay/frame.rs` still accept transcript text and call the output parser before decoding. L-6 remains partial until each entry takes parsed playback at its boundary, as itemized under Interim exceptions."
- id: AD-004:interim-exceptions
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "Public function replay entries ... | State-clause replay entry re-parses `KaniRunOutcome::Falsified` text ... | The public frame replay input carries transcript text ..."
- id: FR-032-AC-2
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: "PLANNED (IR-631). The builder decodes actual routed-harness playback in persisted symbolic call order through the Kani output adapter; another harness name, missing/extra operand, wrong byte width, playback above the builder-owned 8 MiB per-harness ceiling or malformed assertion block yields the distinct typed refusal under the stated precedence before replay."
- id: FR-032:setup-refusal-precedence
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  excerpt: "1. Original proof-context, node/operator/schema ... coherence ... 2. Selected playback byte ceiling, checked before parsing through the Kani output adapter. 3. Adapter selection and decoding ..."
bindings:
- {test_id: typed_schema_byte_ceiling_decode_and_domain_refusals, ac_id: FR-032-AC-2, trace: wrong}
- {test_id: unsupported_operator_and_invalid_persisted_schema_refuse_typed_playback, ac_id: FR-032-AC-2, trace: wrong}
- {test_id: harness_arity_width_and_comment_failures_are_distinct, ac_id: FR-032-AC-2, trace: correct}
- {test_id: assertion_selection_and_malformed_blocks_use_adapter_refusals, ac_id: FR-032-AC-2, trace: correct}
```
