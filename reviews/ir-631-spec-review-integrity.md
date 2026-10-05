---
id: SR-1604
title: "IR-631 spec review (integrity): FR-032 and TC-047 consistency and hidden assumptions"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen#290; spec/replay/functional/FR-032-routed-scalar-replay-binding.md, spec/replay/matrix/TC-047-routed-scalar-replay-binding.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-047
    type: reviews
---

# SR-1604: IR-631 integrity analysis

## Summary

Ticket: IR-631. Reviewer: claude-opus-5-5, run d75eb678-590e-4bb0-b33c-ca7f9c5bde50. I checked completeness, consistency, hidden assumptions and atomicity in FR-032 and TC-047. Four medium findings and one low finding remain: refusal precedence while the gate holds, the terminal reading of the interim unsupported-capability refusal, the settlement variant for a QSL-detected lowering fault, the unpersisted source of the accounting limits, and an unowned decode limit.

## Verdict

**CONDITIONAL**: medium and low findings only. Traceability is complete: FR-032 satisfies StR-001, and TC-047 verifies FR-032 and FR-029. The QSL-641 gates are explicit and preserved. No criterion turns a lowering-parity failure into a source-property refutation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The order of setup refusals is unspecified. AC-1 requires an unsupported-capability refusal until the upstream arm exists. AC-2 and AC-3 require distinct decode and domain refusals before replay. TC-047 step 3 says "until the upstream capability exists, require the unsupported-capability refusal. Try another harness name, missing/extra operands ...". Nothing orders the capability check against decoding and operand validation. While the gate holds, one reader expects every probe to return unsupported-capability; another expects the distinct decode and domain refusals. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:133, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:134, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:185, spec/replay/matrix/TC-047-routed-scalar-replay-binding.md:30 |
| FND-002 | medium | The terminal reading of the interim unsupported-capability setup refusal is not stated. FR-032 maps only "CG-only setup defects carrying no QSL catalog code" to `CgDefect`/`Failed`, and a missing upstream capability is not a CG defect. Readers can disagree on two possibilities: the converter returns `ReplaySettlement::CgDefect`, so every falsified routed scalar run reads `Failed` until QSL-641 lands; or it returns no settlement, giving `MissingSettlement` or the driver's own unavailable reading. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:61, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:123, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:133 |
| FND-003 | medium | The "harness accounting limits" have no persisted source. `ScalarObligationIdentity` persists the node, operation, oracle, module and harness symbols, arguments, solver, unwind and options, but no limits (src/kani/identity.rs:123-141). The renderer hard-codes a `u64::MAX` meter (src/kani/generate/scalar.rs:412). Even so, FR-032's Behavior section carries and binds limits; AC-3, AC-4 and AC-6 key on them; and TC-047 step 3 mutates "changed accounting limits". Neither names the input that supplies the original limits or what a changed value is compared against. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:88, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:138, spec/replay/matrix/TC-047-routed-scalar-replay-binding.md:33, src/kani/identity.rs:123, src/kani/generate/scalar.rs:412 |
| FND-004 | medium | The settlement variant for a QSL-detected lowering fault is unspecified. AC-7 maps QSL's generated-versus-exact divergence to "a CG lowering fault and `Failed`". Of the existing variants, `CgDefect` is documented as "a failure this repository raised that carries no QSL catalog code", and `Fault` is "an internal fault of QSL's call-site facade". This divergence is neither: QSL raises it. FR-032 says only the harness-defect reading needs a settlement extension, so it is unclear whether the lowering fault reuses `CgDefect` with its documented meaning widened, or needs a new variant. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:139, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:72, src/kani/terminal.rs:39 |
| FND-005 | low | The input "a bounded decode limit" and AC-2's "over-limit playback" name no owner, no value, and no relation to the existing Kani output-stream bound (`kani_output_over_limit`, TC-043). The existing `decode_playback` has no limit parameter. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:52, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:134, src/replay/witness.rs:139 |

## Dispositions

Round 1 re-check of the fix-round candidate of PR #290. The fixing commit is recorded in the private ticket marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | The new Setup Refusal Precedence section orders the refusals: context/identity coherence, then the playback byte ceiling, then adapter decode, then operand domain, then upstream capability. An absent capability never hides an earlier refusal. FR-032-AC-1, AC-2 and TC-047 step 3 follow the same order. |
| FND-002 | fixed | Outputs and the terminal paragraph now state it: the interim unsupported-capability refusal yields a typed converter refusal with no settlement. It is not relabelled `CgDefect` and gets no invented QSL code; absent-settlement behaviour stays with FR-029-AC-15. |
| FND-003 | fixed | Inputs now require the original scalar generation/proving context, which retains the renderer's own limits and is tied to the generated source by the canonical proof-content identity. This is stated as planned IR-631 producer work, with no current `ScalarObligationIdentity` limits member and no default. Replay limits are compared with that context, and TC-047 step 3 tests a missing context and altered limits. |
| FND-004 | fixed | FR-032 no longer assumes an existing variant. QSL-reported divergence is not automatically `CgDefect` or `Fault`; its legal settlement and terminal representation is upstream gate 3, and the existing variant documentation cannot be widened silently. |
| FND-005 | fixed | The builder now owns the decode ceiling. Its value is the 8 MiB per-harness capture ceiling FR-017 sets (FR-017 line 58, AC-14; `CAPTURE_LIMIT` in `src/kani/run/launch.rs`), checked on the selected playback before parsing, and the grouped launcher bound is explicitly not a larger allowance. FR-017 is added as a `depends_on` edge. |
