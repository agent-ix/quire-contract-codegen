---
id: "SR-13151"
title: "CG PR 452 gap analysis: FR-024-AC-10 backing, its verification method, the criteria left untagged, and the strict-coverage delta"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@88af6e6ea8fe51647515e5cd5e7d2c941e65a7ed; spec/replay/functional/FR-024-counterexample-envelope-intake.md (AC-2..AC-6, AC-10), spec/replay/functional/FR-016-witness-native-replay.md (AC-6, AC-12), spec/core/non-functional/NFR-002-provenance-boundary.md (AC-1, AC-2), spec/replay/matrix/tests.md, tests/it/layout.rs, src/replay/*.rs, src/kani/terminal.rs (diff origin/main...HEAD, base 87db2f7)"
---

# SR-13151: CG PR 452 gap analysis

## Summary

Ticket: IR-878. PR: agent-ix/quire-contract-codegen#452 at 88af6e6. Plan completion: not
assessed. Tools: quire 0.36.2 (engine 0.50.2), quoin 0.28.3.

**Strict coverage.** Measured with `quire coverage --strict --scope <worktree>` on
`origin/main` 87db2f7 and on the head:

- Backed rows went from 548/906 to 549/906. The FR-024 document went from 24/40 to 25/40.
  Bound tags went from 946 to 947.
- Unbacked rows: 229 before and 229 after. Contradicted statuses: 0 before and 0 after. The
  set of diagnostics is identical apart from one line-number shift.
- `quire matrix` moves FR-024-AC-10 from `untagged` to `tagged` (tests/it/layout.rs:83).
  Nothing else changes.

This matches the author's report. The footer does not move because it counts matrix rows, and
a row is backed when any id in it has a tagged symbol. The old FR-024 AC-1..AC-10 row carried
TC-035, which other tagged tests already back.

**FR-024-AC-10 as a Test.** The criterion names a fixed set of files (`src/`), a closed list
of five names, and two syntactic properties: definition and import. An automated syn scan that
fails on a violation is repeatable executed evidence. This repo already backs other
source-shape criteria (FR-024-AC-14, FR-024-AC-29, AD-004 L-1/L-2) as Tests this way. So
`Test` is an honest method for it, but only while the scan's scope matches the criterion's.
At this head it does not. SR-13150 FND-001 and FND-002 found surviving mutants that define a
listed type under `src/` (through a macro) or import one from `quire_contract_ir` (through a
submodule alias, an `extern crate` alias, or a `self::`/`super::` prefix). The matrix row still
says "✅ Covered" without stating either limit. That is FND-001 below. There are two honest
fixes:

- close the escapes, or
- state the limits in the test doc and the matrix row, as the L-2 header does for `super::`
  and macro paths.

The second leaves a partial Test. Only an Inspection or Analysis verification line would make
the residue explicit.

**The author's untagged list.** I read each criterion and the code:

- FR-024-AC-2: the reason given ("missing or partial assertions") is right. `render_witness`
  (src/replay/function.rs:192-204) is the one `Witness::parse` caller in `src/`, and no test
  asserts that. This is a genuine testable gap: FND-002.
- FR-024-AC-3: the reason is right for the function path. The function path checks the domain
  before replay (src/replay/function.rs:1063, 1114). The bounds test
  `first_out_of_domain_is_inclusive_at_both_bounds` (src/replay/witness.rs:503) is tagged
  FR-016-AC-2 and never shows that replay is not called. The frame half is backed by AC-26's
  test. Genuine gap: FND-003.
- FR-024-AC-4: the reason is right. `StateClauseReplay` has a public `packet` and a
  `replay_through` seam (src/replay/state_clause.rs:433-438, 736-747), so this is fully
  testable now. Genuine gap: FND-004.
- FR-024-AC-5: the reason is right, but this is not a test gap. The corpus `Input` arm is not
  built and is planned for removal (FR-024 lines 415-418). It is a spec problem, recorded in
  SR-13152.
- FR-024-AC-6: the reason given ("candidate in the Kani lane") is not right. Non-Kani frame
  tests already build the `FrameReplay` packet from `call_site`
  (tests/it/kani_obligations_state_frame.rs:2392-2408) and run `.replay()` through
  `replay_frame` (for example, line 2304). Genuine gap: FND-005.
- NFR-002-AC-1: the reason is right. This is a code gap, not a test gap. Generated headers
  name the obligation, not the generator, for example src/kani/generate/render.rs:45 and
  src/kani/generate/frame.rs:1095.
- NFR-002-AC-2: the reason is right that coverage across families is needed. Every renderer
  found emits the SPDX line (harness.rs:901, render.rs:45, frame.rs:1095, v1_bundle.rs:524,
  scalar.rs:632, negotiate.rs:1216, collection.rs:275, population.rs:613, generation.rs:500,
  boolean_v1.rs:349). It is testable now: FND-006.
- FR-016-AC-6: the reason is right. This is a code gap: `src/replay/witness.rs` has no decode
  size limit at all.
- FR-016-AC-12: the reason is right. It is a spec defect, not a test or code defect.
  SR-13152 FND-002 has the evidence.

## Verdict

The new tag backs FR-024-AC-10 with a real parser over the whole `src/` tree. The coverage
delta is correct and complete: +1 backed row, footer unchanged, 0 contradicted. The "Covered"
claim is broader than what the scan catches, so it over-claims until the escapes are closed
or written down (FND-001). That one blocks merge as is.

FND-002 to FND-006 are pre-existing testable gaps for the lead to assign next. They do not
block this PR:

1. FR-024-AC-4, state-clause `replay_through` test
2. FR-024-AC-3, function-path executor-not-called test
3. FR-024-AC-6, frame identity pass-through
4. FR-024-AC-2, single-renderer scan
5. NFR-002-AC-2, SPDX over every family

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-024-AC-10 is marked "✅ Covered" by a scan whose real scope is narrower than the criterion. Mutants defining a listed type in a macro body, or importing one through a submodule alias, an `extern crate` alias or a `self::`/`super::` path, all pass (SR-13150 FND-001/002). A Test tag is honest only if the escapes are closed or the limits are stated in the test doc and the matrix row. Introduced by this PR | spec/replay/matrix/tests.md:17, tests/it/layout.rs:81-139 |
| FND-002 | medium | FR-024-AC-2 has no test, although it is implemented by `render_witness`. Needed: a syn scan asserting that every `Witness::parse` call under `src/` sits inside `render_witness`. Also needed: a test that the transcript a replay passes equals `render_witness` over the decoded values, with no undecoded playback text. Note that `check_text` comes from the playback's assertion block, so the test should pin it as decoded text (FR-024-AC-25 wording). Pre-existing | src/replay/function.rs:183-204 |
| FND-003 | medium | FR-024-AC-3 (function path) has no test. Needed: drive `replay_harness_counterexample_through` with a counting executor. Values at the declared minimum and maximum should call it once each. Values one below and one above should return `EvidenceFailure(Domain { argument })` with zero calls. Pre-existing | src/replay/function.rs:1063-1069, src/replay/witness.rs:503 |
| FND-004 | medium | FR-024-AC-4 has no test, although it is directly testable. Needed: for each `WitnessPacket` member, set it to `None` on a built `StateClauseReplay.packet`. `replay_through` should then return `StateClauseReplayError::Envelope(WitnessRefusal::MissingMember(<member>))`, and the executor should not be called. Also check that `trace_position: Some(None)` is admitted. Pre-existing | src/replay/state_clause.rs:433-438, src/replay/state_clause.rs:736-747 |
| FND-005 | medium | FR-024-AC-6 is testable without Kani, contrary to the PR's reason. Needed: in the frame twin, assert that the packet's `family_payload` frame, anchor and occurrence, plus its `clause_node` and `occurrence_key`, equal what `call_site` returns. Then show `.replay()` through `replay_frame` succeeds. QSL refuses a disagreeing envelope (tc_025 at line 2253), so success shows the identities reached it unchanged. Pre-existing | tests/it/kani_obligations_state_frame.rs:2392-2408, src/replay/frame.rs:643-657 |
| FND-006 | low | NFR-002-AC-2 has no test, although every renderer found emits the SPDX line. Needed: one test that renders one artifact of each generated Rust family and asserts each carries `SPDX-License-Identifier: MIT OR Apache-2.0`. Pre-existing | src/kani/generate/render.rs:45, src/strategy/bound/generation.rs:500 |

## Dispositions

Round 1, reviewed at 2acbf09a9e1ecd76cbef094e44214bd53317ca08. Strict coverage was re-measured with `quire coverage --strict`: backed 548/906 on `origin/main` 87db2f7 and 549/906 at the head, 229 unbacked rows before and after, and 0 contradicted statuses; FR-024 is 25/40. The matrix row now lists the scan's limits: imports and qualified paths inside opaque macro tokens, macro expansion and constructed identifiers. That matches the test doc comment except for "generated files", which the doc names and the row omits. That omission is harmless, because the criterion is scoped to `src/`. The one residue the row does not state is the associated-item path (SR-13150 FND-006). The pre-existing code gaps behind NFR-002-AC-1 (header names no generator) and FR-016-AC-6 (no decode size limit), which this review's summary judged, are owned by IR-891 (Backlog).

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | The scan now kills every mutant that escaped in round 0, and the matrix row states its remaining limits; the narrow associated-path residue is tracked as SR-13150 FND-006 |
| FND-002 | deferred | pre-existing testable gap (FR-024-AC-2); assigned to the lead's next batch, no ticket id yet |
| FND-003 | deferred | pre-existing testable gap (FR-024-AC-3, function path); assigned to the lead's next batch, no ticket id yet |
| FND-004 | deferred | pre-existing testable gap (FR-024-AC-4); assigned to the lead's next batch, no ticket id yet |
| FND-005 | deferred | pre-existing testable gap (FR-024-AC-6, without Kani); assigned to the lead's next batch, no ticket id yet |
| FND-006 | deferred | pre-existing testable gap (NFR-002-AC-2 SPDX per family); assigned to the lead's next batch, no ticket id yet |
