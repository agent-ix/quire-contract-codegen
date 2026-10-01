---
id: "SR-661"
title: "CG PR 215 spec review (integrity): AD-004 CG crate layout"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@bfaaa849a100aee4a49959554b4607f0410227b1 (review), b0c000866656cb3f4044acd10ed15ccb53b5d463 (disposition pass 1), fe571ba8cc34b33a7beea59c6c00c4f902a25a96 (disposition pass 2), 9d06673a824c498b6d1b447b7fbf5efe25e6b2c2 (disposition pass 3); spec/assurance/AD-004-cg-crate-layout.md, spec/spec.md (References); measured against src/ at origin/main 2fad745"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AD-001
    type: references
---

# SR-661: CG PR 215 spec review (integrity)

## Summary

Ticket: IR-344. PR: agent-ix/quire-contract-codegen#215 at bfaaa84 (re-read after the C-09
step-5 addition; the earlier head ccb4394 was also read). Base origin/main 2fad745.

Spec-only PR: one new ArchitectureDescription and one References line. This review checks
structure, internal consistency, consistency with spec/spec.md, AD-001 and the requirements
the AD cites, and every measured claim against the code at origin/main. The import edges were
measured by reading every `use crate::` block in `src/`, including the 17 blocks that import
through the crate root, and resolving each root item to the module that defines it (a
grep-based script plus a manual read of the blocks; no compiler graph).

Measured claims confirmed: 27 modules in `src/lib.rs`, all flat apart from `bound_strategy`;
26,687 lines; no strongly connected component among non-test edges; the only test back-edge in
the named pair is `kani_transcript.rs:261`; `kani_obligations` imports nothing from
`kani_execution`; 17 files import through the crate root; 9 one-line `fn artifact(` wrappers;
2 `deterministic_json` copies (`oracle.rs:1111`, `kani.rs:1045`); `RUNTIME_REVISION`
(`oracle.rs:13`) read only by the three manifest templates and by tests (no other caller in
the agent-ix GitHub code search); `CheckedNodeKind` unused; every renderer line cite
(`kani.rs:314`, `:814`, `kani_obligations.rs:2016/2057/2082/2187/2190/2285/2288`,
`state_frame.rs:1075/1099`, `bounded_kani_corpus.rs:588`) is exact; `kani.rs` and the corpus
emit no `kani::cover!`, as IR-464 says. `make spec` exits 0 with the 3 baseline warnings
(FR-017:137, FR-014:278 twice). AD-004 adds none. No requirement id is minted and none is
removed. FR-015-AC-22/AC-25 stay verbatim. The AD adds no pin, SHA, version record or vendored
file. `RUNTIME_REVISION` is deleted. The PR conflicts with PR #214 in `spec/spec.md`
References only (merge-tree).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The dependency direction is false against the code at the base, and a rename-only step cannot fix it, so L-2's layout test cannot pass at step 2e as planned. The AD says `evidence --> core` only and that strategy, evidence and kani are peers. Non-test edges that break it: (1) `bound_coverage` (evidence) imports `BoundOracleGeneration` and `GeneratedBoundOracles` from `bound` (strategy), an evidence-to-strategy peer edge (`bound_coverage.rs:8-11`). (2) `bound` (strategy) imports `PublicationDiagnostic` (`bound.rs:9`, `:94`), and the target tree keeps publication diagnostics in `publication/publish.rs`: an upward strategy-to-publication edge. (3) `ArtifactBundle::new` returns `PublicationDiagnostic` (`publication.rs:86`), so moving the bundle and its limits to `core/artifact.rs` while the diagnostic stays in `publication/` makes core import publication. (4) In the spec, FR-004 (evidence) `depends_on` FR-014 (oracle), so the direction diagram also contradicts the requirements' own edges. The AD needs an evidence-to-oracle edge (or an explicit V1 exception until step 6), a decision on where `PublicationDiagnostic` lives, and a statement of which of these edges step 2 is allowed to keep | spec/assurance/AD-004-cg-crate-layout.md:217-249, :409-414, :447-452 |
| FND-002 | medium | The order inside `kani/` says `run` imports `identity`, `output` and `classify`, not `generate`. Today `kani_execution` (target `kani/run/`, `kani/classify.rs`) imports `KaniObligationHarness`, `KaniScalarObligationHarness` and `ObligationKind` from `kani_obligations`, and `StateFrameHarness` and `StateFrameProperty` from `state_frame`, both of which become `kani/generate/` (`kani_execution.rs:45-52`). Its `#[cfg(test)]` module imports `state_frame` generators too (`:767`), and the AD says test modules obey the same direction. Step 2d is rename-only, so these harness types must move to `kani/identity.rs` in some step, and no step says so | spec/assurance/AD-004-cg-crate-layout.md:239-244, :449-452 |
| FND-003 | medium | L-1's layout test "compares with the registry" and lands with step 2e. The registry gains its `publication` row and its per-directory owning-module column only in step 7. At 2e the test either fails (`src/publication/` has no registry row; the Owning crates/modules column still names flat modules) or compares with something other than the registry. Either land the registry rows with 2e or make L-1 compare with the AD's map until step 7 | spec/assurance/AD-004-cg-crate-layout.md:214-215, :409-411, :452, :492-497 |
| FND-004 | medium | The C-09 addition to step 5 prescribes behaviour although the AD says "Out of scope: behaviour. Every requirement keeps its id and its criteria". FR-029 maps one Kani run outcome to one `TerminalValue` "in one match". Step 5 makes the map a function of the pair (outcome, replay result), with `ReplayParity`, `ReplayRefused` and `Failed` outcomes. Its spellings are relayed and marked "not verified here". That is a change to FR-029, and it should go to FR-029 by its ticket, with the AD citing it. The totality claim is also over QSL's replay result type only. CG's replay path has its own failure values before or around QSL: `SpineReplayError::{UnboundArgument, FieldDelimiter, Transcript, WrongArm}`, `DependencyLockError`, and the witness `DecodeFailure` that the AD itself moves to `replay/witness.rs`. The AD does not say who converts them into the QSL value the map takes, or which module pairs the two inputs (by the direction rules it can only be `routed`). The layering itself is acyclic and acceptable: `kani/terminal.rs` importing `qsl-replay` adds no kani-to-replay edge | spec/assurance/AD-004-cg-crate-layout.md:41-44, :318, :478-487 |
| FND-005 | low | Counts that do not match their own text. "Five renderers emit Kani source, not three" heads a list of four numbered items. Item 2 alone holds three templates, and item 3 holds two, so the count is four files or seven templates, not five. "63 `.get(\"..\")` calls" is the number of matching lines. The calls number 70 (`exact_scalar` 25, `state_frame` 31, `composite_equality` 11, `exact_function` 3) | spec/assurance/AD-004-cg-crate-layout.md:80-90, :105-107, :367 |
| FND-006 | low | Inconsistent V1 reader lists. Current state names `strategy` among the V1 readers. The module map marks `strategy` as plain "moved", not "V1 input", and step 6 does not list `strategy/campaign.rs`. The AD also does not address `pub mod bound_strategy` (`lib.rs:49`): it is the one public module path, and it changes when it moves to `strategy/bound/` | spec/assurance/AD-004-cg-crate-layout.md:94-96, :194, :488-491 |
| FND-007 | low | L-3's grep gate is both too broad and too narrow. `kani::any` already appears in doc comments of `kani_witness_join.rs` (`:5`, `:9`, `:12`, `:75`, ...), which becomes `replay/witness.rs`, and `kani::proof` appears in `state_frame.rs` docs. A text grep would fail on prose. It does not name `kani::requires`, `kani::ensures` or `proof_for_contract`, which the contract templates emit (`kani.rs:806-807`, `kani_obligations.rs:2285`), so a second contract renderer would pass the gate. Scope it to string literals and include the contract attributes | spec/assurance/AD-004-cg-crate-layout.md:261-263, :415-416 |
| FND-008 | low | The AD relies on AD-002 (R-Q5) and AD-003 (E-1, the obligation digest), which exist only in open PR #214, not at origin/main. If this merges first, those references dangle until #214 lands. AD-003 in #214 also fixes the digest preimage as RFC 8785 over the ADR-013 O-09 members, carried as QSL's opaque `ObligationIdentity`. AD-004 puts "deterministic JSON, the one content digest" in `core/canonical.rs`, where `deterministic_json` today is plain `serde_json::to_vec` (`kani.rs:1045`), not RFC 8785. It also adds `ContentDigest` beside QSL's `ObligationIdentity` without saying how the two relate | spec/assurance/AD-004-cg-crate-layout.md:270-271, :349-358, :376 |
| FND-009 | low | Public repo. The V2 strategy row gives another team's milestone label and delivery slot ("IR-364 (M3, the V2 strategy chain), IR team"). Cite the public ticket only and drop the milestone label | spec/assurance/AD-004-cg-crate-layout.md:532 |
| FND-010 | low | `core/ir` "becomes a thin re-export of IR's" decoder when IR exposes one. That is a forwarding layer kept for its call sites, which is the shim pattern the ruling and the repository forbid. Say instead that callers switch to IR's decoder and `core/ir` is deleted in the same change | spec/assurance/AD-004-cg-crate-layout.md:368-370 |
| FND-011 | low | Outside the diff, a separate fix: AD-001's Current state and Risks are stale against what AD-004 records. AD-001 lists `bounded_kani_corpus.rs` and `bounded_kani_profile.rs` as V1 paths, but neither names `BoundPackage`, `BoundClause` or `TypedExpression` at the base. AD-001's risk "Kani publishes no machine-readable verdict" contradicts AD-004's `--export-json` report decision | spec/assurance/AD-001-codegen-architecture.md:198-201, :210-211 |

## Verdict

Structure is sound: a valid ArchitectureDescription with System Boundary, Views, Decisions
(L-1 to L-12, each with a test), Migration order, Risks and an explicit "Not verified" list.
`make spec` is clean against the baseline, no id is minted or removed, the two spec.md
exceptions are settled consistently with the registry note, and no pin, SHA, version record,
vendored file or compatibility layer is proposed, apart from the `core/ir` re-export wording
(FND-010). Most measured claims are exact.

The central design claim does not hold: the downward-only dependency direction is
contradicted by existing edges that a rename cannot remove (FND-001, FND-002), and the layout
test is scheduled to land before it can pass (FND-003). The C-09 addition puts requirement
behaviour into an AD that declares behaviour out of scope (FND-004). Not mergeable as an
approved AD until FND-001 to FND-004 are fixed.

## New findings (disposition pass 1)

Re-reviewed at b0c0008 (fix commit 6303729 plus the quire-canonical edit b0c0008 over 83d8d97).
`make spec` exits 0 with the 3 baseline warnings. The committed `reviews/SR-661` and `SR-662` are
byte-identical to the review-pass files. The new direction rules were re-checked against every
non-test and test `use crate::` edge at the base. With `bound` in oracle, the bundle, its limits and
`PublicationDiagnostic` in core, and the harness records in `kani/identity.rs`, every edge points
downward. The quire-canonical API names `to_vec(value, Limits)` and `sha256` exist at
quire-canonical origin/main and at tag quire-canonical-v0.3.0. The digest type is `Sha256Digest`.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-012 | medium | Mapping every CG-side replay failure to `TerminalValue::Failed` is too loud in part, and it is inconsistent with the QSL cases the same step lists. QSL documents `Failed` as "the tool itself failed" and maps invalid or incomplete input to `Declined(ProofRefusalCause)` (`qsl-replay/src/proof_result.rs:118-126`). `DependencyLockError::{Duplicate, Input}` is a refused caller lock, which is invalid input, not a tool failure. `DecodeFailure` `cg_witness_harness_identity_mismatch` and the schema mismatches are the same conditions (identity mismatch, decode refusal) that case (2) maps to `Inconclusive(ReplayRefused)` when QSL detects them. So one condition gets two terminal values, depending on which side noticed it. Only malformed Kani output (`kani_witness_*` text codes) fits `Failed`. The "Decision" wording also contradicts "It adds no behaviour claim" two bullets earlier. Suggest: ask QSL-351 for a CG-side refusal cause, map the input refusals to `Declined` or to that cause, and keep `Failed` for a malformed backend output | spec/assurance/AD-004-cg-crate-layout.md:596-611 |
| FND-013 | medium | Step 1a's precondition names the driver's and quire-integration's locks. CG's own lock already resolves `quire-canonical` from QSL's tag (`Cargo.lock:1345-1347`, `?tag=quire-canonical-v0.3.0`, through `qsl-replay`). Adding a direct `branch = "main"` dependency therefore gives CG's own `Cargo.lock` two entries and fails CG's own `make deny` one-copy gate (`scripts/check_one_copy.awk`). 1a is blocked in this repository until QSL moves to `branch = "main"`, unless CG spells the same tag, which is a pin. The precondition "the driver's one-copy gate is checked first" also does not say what must hold. State it as: 1a waits until QSL resolves `quire-canonical` from `branch = "main"`, verified by `make deny` here | spec/assurance/AD-004-cg-crate-layout.md:421-433, :532-534, :642-643 |
| FND-014 | low | Step 2b's type list is incomplete. `KaniObligationIdentity` holds `EmbeddedOracle` (`kani_obligations.rs:456`, `:468`). `KaniScalarObligationHarness` holds `ScalarObligationIdentity`, which holds `ScalarObligationArgument` (`:513-550`). `StateFrameHarness` holds `StateFrameIdentity`, which holds `StateFrameScope`, `StateFieldDomain` and `StateFrameProperty`, and `StateFrameProperty` holds `StateComparison` (`state_frame.rs:95-218`). Unless these move to `identity` too, `kani/identity.rs` imports `generate`, which is the back-edge 2b exists to remove | spec/assurance/AD-004-cg-crate-layout.md:543-547 |
| FND-015 | low | Cross-PR consistency, report only. AD-003 at PR 214 head 9b3ea58 still routes the encoder and digest "through the `qsl-replay` facade" and asks QSL to export them (AD-003:127, :275-277, R-Q2 at :339). AD-004 now depends on `quire-canonical` directly, with no re-export. One of them must change before both merge. Also, IR PR 241's AD-006 Decision A has codegen add a direct `quire-contract-model` dependency and read IR model items from it before IR removes its root glob (R3-C2). AD-004's `core/ir` and import rules do not mention it | spec/assurance/AD-004-cg-crate-layout.md:421-425, :448-453 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 6303729 |
| FND-002 | fixed | 6303729 |
| FND-003 | fixed | 6303729 |
| FND-004 | fixed | 6303729 |
| FND-005 | fixed | 6303729 |
| FND-006 | fixed | 6303729 |
| FND-007 | fixed | 6303729 |
| FND-008 | fixed | 6303729 |
| FND-009 | fixed | 6303729 |
| FND-010 | fixed | 6303729 |
| FND-011 | deferred | AD-001 is outside this PR's diff; AD-004 step 7 now names the stale AD-001 text as a follow-up for the step-7 spec PR (AD-004:624-627) |

## New findings (disposition pass 2)

Re-reviewed at fe571ba (step 5 rewritten against QSL's merged ADR-013 C-09 and ADR-011 T-13).
The merged text was read at QSL origin/main b5ef6475 (QSL #550). It confirms the replay cases
as stated. It also says that the orchestrating driver (`quire-driver`) "runs the Kani obligation
and the replay and passes both" to CG's C-09 map, and that the map's first input is IR's
`KaniOutcome` (ADR-013:629, C-09 row :967; ADR-011 E9 :262). The closed-set rule for CG-origin
failures is relayed, not in the merged text. It is consistent with QSL's `Failed` ("the tool
itself failed"): each listed failure is produced by CG on bytes that CG or the driver made, so
none is a caller refusal. PR 214 at 0cbb45f now names `quire-canonical` directly, as AD-004 does.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-016 | medium | The pairing point contradicts the merged T-13 and C-09 text that step 5 itself now cites. QSL's ADR-013 C-09 says the orchestrating driver runs the Kani obligation and the E9 replay and passes both to the CG map. AD-004 still says `routed` "is the only place that can pair a run outcome with a replay result", and names `routed/adapter.rs` as the pairing point (tree, the run/report table and step 5). C-09's first input is IR's `KaniOutcome`, but step 5 says the C-09 map takes CG's `KaniRunOutcome` (that is FR-029's map; FR-030's takes IR's). Fix: `kani/terminal.rs` exports the C-09 map as a public entry over (IR `KaniOutcome`, QSL replay result), which the driver calls. Say where CG's own replay failures (`SpineReplayError`, `DependencyLockError`, `DecodeFailure`, all `replay/` types) become `Failed` for that caller. `kani` cannot import them, so either `replay/` exposes the conversion or the driver applies the closed-set rule | spec/assurance/AD-004-cg-crate-layout.md:278-282, :389, :590-595, :614-618 |

## Dispositions (round 2)

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-012 | fixed | fe571ba |
| FND-013 | still-open | fe571ba leaves step 1a's precondition unchanged: CG's own Cargo.lock resolves quire-canonical from QSL's tag, so a direct branch=main dependency fails CG's own make deny one-copy gate; the precondition still names only the driver's gate |
| FND-014 | still-open | Step 2b's type list is unchanged at fe571ba; EmbeddedOracle, ScalarObligationIdentity/Argument, StateFrameIdentity/Scope/FieldDomain and StateComparison are still not listed |
| FND-015 | still-open | The PR 214 half is resolved (AD-003 at 0cbb45f names quire-canonical directly); the IR PR 241 AD-006 decision A (direct quire-contract-model dependency, R3-C2) is still not reflected in AD-004 |

## New findings (disposition pass 3)

Re-reviewed at 9d06673. `make spec` exits 0 with the 3 baseline warnings at the head, and also
with merged AD-002 and AD-003 from origin/main d3acbe5 laid over it. Against origin/main
(PR 214 merged as d3acbe5), `git merge-tree` reports a conflict in `spec/spec.md` References only:
keep the AD-002, AD-003 and AD-004 lines.

Layering verified. `kani/terminal.rs` defining its replay-outcome type from `qsl-replay` types
makes `kani` depend on `qsl-replay`. That dependency is external to the crate, so the
crate-internal direction rules do not govern it. `qsl-replay` is the one QSL crate T12-A allows,
and `kani` already names `qsl_replay::TerminalValue` for FR-029 and FR-030. With `replay/`
converting into that type, `kani` imports nothing from `replay`.

Step 1d verified at IR origin/main: AD-006 is merged with R3-C2, and `src/lib.rs:12` holds
`pub use quire_contract_model::*;`.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-017 | low | PR 214 is merged (d3acbe5), but AD-004 still says AD-002 and AD-003 "are proposed in PR 214 and are not at this base ... PR 214 merges first", and cites them as "(PR 214, pending)" in four more places. The rebase should drop "pending", and may add AD-002 and AD-003 to `relationships` | spec/assurance/AD-004-cg-crate-layout.md:32-34, :325, :431, :474, :724 |
| FND-018 | low | Step 5's open question says it uses the "same wording as R-Q1 in CG PR 214", but merged AD-003 R-Q1 (AD-003:345) is the request for inconclusive causes, not this Declined-or-refusal question. QSL's answer, relayed by the leader and not in any merged text I could read: an obligation input refused before Kani gives `Declined(ProofRefusalCause)`; a replay setup refused on data after a refuted Kani run gives `Inconclusive(ReplayRefused)` with a QSL code catalogued in QSL-352, which lands with step 5; faults give `Failed`; `Failed` in the interim. `DependencyLockError` and the witness `DecodeFailure` arise after a refuted run, so step 5 should record that answer and move them out of the CG-defect variant when QSL-352's codes land, not leave the question open | spec/assurance/AD-004-cg-crate-layout.md:641-656 |
| FND-019 | low | Who calls `qsl_replay::replay` is left ambiguous. QSL's merged ADR-011 E9 (the T-13 driver row) says the driver calls `qsl_replay::replay` with the request CG's replay adapter builds. AD-004's table says the driver "calls `replay/` for the replay outcome", which reads as CG's `replay/` still running the replay, as `spine_replay` does today and as AD-001:162 says. State that `replay/` builds the request and converts QSL's result (and its own errors) into the C-09 input, or record the deviation from T-13 | spec/assurance/AD-004-cg-crate-layout.md:394-396, :657-666 |

## Dispositions (round 3)

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-013 | fixed | 9d06673 |
| FND-014 | fixed | 9d06673 |
| FND-015 | fixed | 9d06673 |
| FND-016 | fixed | 9d06673 |
