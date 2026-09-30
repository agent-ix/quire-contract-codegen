---
id: "SR-040"
title: "PR 185 spec review: FR-024, FR-025, ADR-001 to ADR-003, AD-001 rewrite, FR-016 AC split"
type: SpecReview
scope: "agent-ix/quire-contract-codegen; spec/assurance/AD-001-codegen-architecture.md, spec/assurance/MP-001-codegen-measurements.md, spec/decisions/ADR-001-overlapping-generators-and-input-models.md, spec/decisions/ADR-002-backend-adapter-boundary.md, spec/decisions/ADR-003-kani-tractability.md, spec/functional/FR-003-kani-lowering.md, spec/functional/FR-006-shared-assurance-intake.md, spec/functional/complete-v1/FR-016-witness-native-replay.md, spec/functional/complete-v1/FR-017-kani-execution-evidence.md, spec/functional/complete-v1/FR-018-composite-equality-oracles.md, spec/functional/complete-v1/FR-024-counterexample-envelope-intake.md, spec/functional/complete-v1/FR-025-generated-subject-abi.md, spec/index.md, spec/interface/interface-001-codegen-api.md, spec/stakeholder/StR-001-traceable-generation.md, spec/test-matrix.md, spec/test/complete-v1/TC-026-witness-native-replay.md, spec/test/complete-v1/TC-035-counterexample-envelope-intake.md, spec/test/complete-v1/TC-036-generated-subject-abi.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/AD-001
    type: reviews
---

# SR-040: PR 185 spec review

## Summary

Ticket: IR-92 (primary), IR-93. PR: agent-ix/quire-contract-codegen#185. Spec-only, 19 files
under `spec/`. Methods: spec-review with the integrity, EARS, evidence, dependency, scope-boundary,
failure-domain and risk sub-analyses folded into this one file, following the SR-039 precedent.

The PR was checked against the owner ruling for this session. QSL owns `Witness`, `ReplaySource`,
the counterexample envelope, the FR-331 terminal record and `ObligationIdentity` in `qsl-replay`.
IR deletes its copies. Kani transcript parsing stays in CG's adapter. Replay goes only through
`qsl_replay::replay` and `replay_frame`, and `runtime::execute` is retired. FR-024 and FR-025 follow
that ruling. The types they name exist in `qsl-replay`
(`WitnessEnvelope<P: FamilyPayload>`, `ObligationIdentity([u8; 32])`, `replay_frame(wire,
&WitnessEnvelope<FrameCounterexample>)`, `Witness::parse`, `TerminalRecord`). AD-016 arrow 5 and
QSL ADR-013 O-09 support the digest-except-`source_span` rule, ascending `arguments`, lossless
`i64` widening, requires-bound and the frame subject ABI. QSpec FR-197 and FR-312 support the
minimization lineage rules.

Measured:

- `make spec` exits 2 on both `origin/main` and head, and the two logs are byte-identical. The
  exit 2 comes from existing AP-001 and MP-001 frontmatter failures. Because `make spec` prints no
  per-file count, the new files were also validated one by one with `quire validate`, which exits 0.
- `quire coverage --json` finds `status_lies` empty on both sides. `unbacked_rows` goes from 11 to
  30, and every added row is a new FR-024/FR-025/TC-035/TC-036 row marked Planned or Partial.
- Test-embedded files: `tests/it/interface_001.rs` reads the yaml block line by
  line, only for `operations`, `required: [`, `results: [` and `terminal_states: [`. The edited
  `replay_boundary` line is a single scalar line outside those keys. The test was not run, because
  that would need a full crate build. `tests/it/shared_assurance.rs:1485` skips `.md` files, so the
  "FR-006-AC-4" mention in FR-006 cannot trip the deleted-names census.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AD-001 describes as current fact things that are false at this revision. The CG → QSL seam row says "No copies of these types exist in CG or in Contract IR", but `src/kani_witness_join.rs:35-36` imports IR `Witness`, and `src/bounded_kani_corpus.rs:14-16` and `src/bounded_kani_replay.rs:3,24` import IR `ReplaySource`. The same PR's TC-035 Status says so. Replay view step 2, the domain check before replay, is also unbuilt (FR-016-AC-2 is Planned). | spec/assurance/AD-001-codegen-architecture.md:147, spec/assurance/AD-001-codegen-architecture.md:120 |
| FND-002 | medium | FR-024 and AD-001 cite QSL ADR-013 O-25 to O-27 as the authority for QSL owning `Witness`, `ReplaySource` and the terminal record. On QSL `origin/main`, O-25 names IR `src/kani/witness.rs` as the backend-witness owner and IR `CounterexamplePacket` as the `ReplaySource` carrier. O-24 has IR map `KaniOutcomeKind` to the terminal record. ADR-002 Q3 in the same PR says O-24 needs amending. The ownership comes from the owner ruling and the `qsl-replay` API, not from ADR-013 as written. | spec/functional/complete-v1/FR-024-counterexample-envelope-intake.md:36, spec/assurance/AD-001-codegen-architecture.md:158 |
| FND-003 | medium | FR-024-AC-4 and its "any member absent" Behavior line do not match `qsl_replay::WitnessEnvelope`. Dependency entries are a `ReplayRequest` member (`DependencyEntry`), not an envelope member. The envelope's `clause_node`, `package_contract_version` and `source_digests` are missing from the per-member list. `trace_position` is `Option`, so "refuse if any member is absent" is wrong for a family that has no trace position. | spec/functional/complete-v1/FR-024-counterexample-envelope-intake.md:77, spec/functional/complete-v1/FR-024-counterexample-envelope-intake.md:102 |
| FND-004 | medium | FR-024-AC-2 lets exactly one adapter function read backend-native counterexample text. FR-017-AC-10 puts all Kani wording, including playback parsing (`src/kani_transcript.rs:26-29`, `concrete_playback_run`), in `src/kani_transcript.rs`, and `src/kani_witness_join.rs` also parses the playback. Both ACs can hold only if the renderer lives in `kani_transcript.rs`, and neither says so. TC-035 step 2's scan has no defined wording set, so it cannot be run as written. | spec/functional/complete-v1/FR-024-counterexample-envelope-intake.md:100, spec/functional/complete-v1/FR-017-kani-execution-evidence.md:144 |
| FND-005 | medium | The AC-4/AC-12 split is non-overlapping (replay ran and disagreed, or it did not run). But "cannot run" is undefined, and it overlaps AC-11 ("a request QSL refuses (returned with its cause)") and FR-024's `Witness::parse` refusal. A QSL refusal such as missing source bytes or a stale `package_id` fits both "unavailable" (AC-12) and "distinct typed error with cause" (AC-11), so the two ACs cannot both be tested. | spec/functional/complete-v1/FR-016-witness-native-replay.md:97-98 |
| FND-006 | medium | FR-024-AC-9 forbids importing `ReplaySource` from `quire_contract_ir`, and FR-024 says replay goes only through `qsl_replay`. FR-007 still requires packets "replayed by Contract IR's native" replay boundary (FR-007:45, :96), and its code imports IR `ReplaySource`. Until ADR-001 Q1 is ruled, the spec set contains two requirements that cannot both hold. The PR removed stale replay text from FR-003, FR-017 and MP-001, but left FR-007 unmarked. | spec/functional/complete-v1/FR-024-counterexample-envelope-intake.md:107, spec/functional/FR-007-bounded-kani-profile-corpus.md:96 |
| FND-007 | medium | Public-repo leak of a private delivery schedule. FR-025 Open items quotes AD-016's "`OPEN — decided in WP<n>`" and "`OPEN — decided in WP<n>`". quire-specification is PRIVATE and CG is PUBLIC. Work-package numbers are delivery order, and no CG spec on `main` carries a WP reference. | spec/functional/complete-v1/FR-025-generated-subject-abi.md:80, spec/functional/complete-v1/FR-025-generated-subject-abi.md:91 |
| FND-008 | low | FR-025 carries an "Open items" section of design questions with recommendations. Open questions belong in an ADR, and this PR adds three. The "While no Contract IR frame lowering exists" Behavior line and AC-5 tie a requirement to another repo's progress. | spec/functional/complete-v1/FR-025-generated-subject-abi.md:60, spec/functional/complete-v1/FR-025-generated-subject-abi.md:73 |
| FND-009 | low | FR-025 says its ABI is the one "AD-016 arrow 5 and QSL ADR-013 O-25 decide". AD-016 decides the `i64` decode and lossless widening. The Rust-type table (`bool`, `rt::Integer::from(i64)` passed by reference) is CG's own choice and should be stated as such. | spec/functional/complete-v1/FR-025-generated-subject-abi.md:25 |
| FND-011 | low | AD-001 says "arrows 3 to 5 (settlement, oracle and harness generation)". In AD-016, arrow 3 is IR → runtime-op selection, which is RT's side, and negotiation is settled at arrow 4. | spec/assurance/AD-001-codegen-architecture.md:33 |
| FND-012 | low | ADR-002 is `Proposed`, but its Decision section states a five-part adapter as the target design. The owner ruling covers only the transcript-parser and witness-renderer part. It also says "the owner ruling recorded in AD-001", but AD-001 records no ruling. | spec/decisions/ADR-002-backend-adapter-boundary.md:23, spec/decisions/ADR-002-backend-adapter-boundary.md:44 |
| FND-013 | low | Stale text left beside edited text. interface-001 `outcome_source` still says the playback is "decoded by the IR crate's witness parser (codegen#59)", the phrase this PR removed from FR-017. TC-035 Status leaves `src/bounded_kani_replay.rs` out of the step-8 failure list. FR-013:97-98 still names `runtime::execute`, which the owner ruling retires. | spec/interface/interface-001-codegen-api.md:307, spec/test/complete-v1/TC-035-counterexample-envelope-intake.md:60, spec/functional/strategies/FR-013-it010-consumable-output.md:98 |
| FND-014 | low | Compound ACs. FR-024-AC-2 holds three claims (parse-only admission, a single reader, refusal with no replay). FR-024-AC-7 holds retention, lineage and discard. FR-025-AC-2 holds naming plus two refusals. | spec/functional/complete-v1/FR-024-counterexample-envelope-intake.md:100, spec/functional/complete-v1/FR-024-counterexample-envelope-intake.md:105, spec/functional/complete-v1/FR-025-generated-subject-abi.md:68 |
| FND-015 | low | Wording rules. FR-018 says "which no requirement owns yet", a statement of what is not. FR-016 Dependencies keeps "Linear IR-309" in requirement text, although this PR removed the "#49/#50/#55" narration. AD-001 drops the "no program-wide structural convergence is claimed" disclaimer and the evidence-verifier rationale without saying so. | spec/functional/complete-v1/FR-018-composite-equality-oracles.md:258, spec/functional/complete-v1/FR-016-witness-native-replay.md:109 |

## Sub-analyses

### Integrity

The matrix is consistent. FR-016-AC-12 is in the Planned row and in the TC-026 registry row.
FR-024 and FR-025 have rows, and TC-035 and TC-036 sit in the TC registry table after TC-034, with
no malformed rows. TC-031 and TC-032 were added to the registry and match their FR rows. The
StR-001 VC-3 and VC-4 rows are in the Stakeholder table. StR-001 now has `satisfied_by` for every
FR except FR-006 and the unwritten FR-020, which matches the index text. The index area table
agrees with the directories. FR-006 retires AC-4 in a note and does not renumber, which matches
CLAUDE.md. Findings: FND-001, FND-006, FND-010, FND-011, FND-013.

### EARS

The new Behavior lines in FR-016, FR-024 and FR-025 use ubiquitous, event (`When`), unwanted
(`If … then`), optional (`Where`) and state (`While`) forms, and quire's EARS lint passes them.
"The subject of a frame obligation shall be …" is a definition, not a behaviour. Finding: FND-014.

### Evidence

Every new AC is Test-verified against TC-035 or TC-036, and each TC step maps to one AC. TC-035
step 2's wording scan and TC-035 step 4's member list are the unrunnable steps (FND-003, FND-004).

### Dependency

FR-024 depends on FR-016, FR-017 and FR-025. FR-025 depends on FR-015. There is no cycle. The
cross-repo `references` edges resolve at `origin/main` of quire-spec-language (ADR-011, ADR-013,
ADR-015, FR-116) and quire-specification (AD-016, FR-197, FR-312, TC-219). FR-025 lists FR-016 as
downstream, but FR-016 does not list FR-025 upstream (informational).

### Scope boundary

The boundary matches the owner ruling: CG builds QSL's types and defines none (FR-024-AC-9), and
the transcript parser stays in CG. The unresolved overlaps are FR-007 against FR-024 (FND-006) and
FR-024 against FR-017 over who reads playback text (FND-004).

### Failure domain

"Unavailable" has no defined trigger (FND-005), and neither does optional-member absence
(FND-003). Out-of-domain handling is covered twice, in FR-016-AC-2 and FR-024-AC-3 (informational).

### Risk

ADR-003 records the Kani tractability risk as open questions with options and cites the external
reports as unmeasured here. ADR-001 records the three-generator and two-input-model overlap as open.
Both are genuinely `proposed` and decide nothing. ADR-002 is the exception (FND-012).

## Clean

- FR-016 AC-4/AC-12 split: the two ACs do not overlap. "Runs and disagrees" and "cannot run" are
  disjoint, and each rules out the other's result.
- FR-003 Dependencies, FR-017 Behavior/AC-10, MP-001 and interface-001 `replay_boundary` no longer
  name IT-010 or `runtime::execute`.
- FR-024's frame path (`WitnessEnvelope<FrameCounterexample>` through `replay_frame`) and its
  minimization rules match QSL FR-116, QSpec FR-197 and FR-312.
- FR-025's ascending order, digest-except-`source_span`, requires-bound and frame subject ABI match
  AD-016 arrow 5 and the frame row, and QSL ADR-013 O-09.
- ADR-001 and ADR-003 are `proposed` and state open questions with options.
- `make spec` output is identical before and after. `status_lies` is empty.

## Verdict

Not mergeable yet. There are seven MEDIUM findings: two false or mis-cited authority claims
(FND-001, FND-002), three untestable or conflicting criteria (FND-003, FND-004, FND-005), one
unresolved conflict between two requirements (FND-006) and one private-schedule leak into a public
repo (FND-007). All are text fixes inside this PR. The LOW findings are editorial.

## Dispositions

Round 1.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The AD-001 seam row now reads "Target: no copy … see Current state". A new Current state section lists the IR `Witness` and `ReplaySource` imports, the IR replay path in `bounded_kani_replay.rs`, and the unbuilt domain check. |
| FND-002 | fixed | FR-024 and AD-001 now name the 2026-09-28 owner ruling and the `qsl-replay` API as the authority. They state that ADR-013 O-24/O-25 and AD-016 still name IR, and that the amendment is pending. |
| FND-003 | fixed | FR-024-AC-4 lists exactly the 14 `WitnessPacket` members, and each is refused by `WitnessEnvelope::reconstruct` with `MissingMember`. Trace position present with value none is admitted, which matches `Option<Option<TracePosition>>`. Dependency entries moved to the request members. |
| FND-004 | fixed | FR-024-AC-2 now requires only that the renderer take decoded values and no backend-native text, and it defers wording exclusivity to FR-017-AC-10. TC-035 names the seven-literal wording set, which matches `tc_027_no_other_source_file_contains_kani_prose_literals` (`src/kani_transcript.rs:710-718`). FR-016 adds a target bullet for reading playback only through `kani_transcript.rs`. |
| FND-005 | fixed | FR-016 has an ordered partition table. AC-12 is only `ReplayRefusal::Fault`, and AC-11 covers every other `ReplayRefusal`, so the two no longer overlap. Treating Fault as unavailable is consistent with QSL ADR-013 T-4 (an `InternalFault` maps to internal failure and is never a Refusal). One residual is recorded as FND-017. |
| FND-006 | fixed | FR-007's output, Behavior bullet and AC-4 now target the QSL `Input` arm (FR-024). A Replay target section states the current IR path, and the matrix marks FR-007-AC-4 Planned. One residual is recorded as FND-016. |
| FND-007 | fixed | `grep -rE 'WP[0-9]' spec reviews` finds nothing. FR-025 Open items is removed. |
| FND-008 | fixed | The open questions moved to the new ADR-004 (Proposed, with options and recommendations, deciding nothing). The While-clause became an unwanted-behaviour `If` bullet, and AC-5 no longer depends on external progress. |
| FND-009 | fixed | FR-025 now says which parts AD-016 and ADR-013 decide, and states that the Rust-type table is the generator's own choice. |
| FND-011 | fixed | AD-001 now places runtime-op selection at arrow 3, settlement and oracles at arrow 4, and harnesses at arrow 5. |
| FND-012 | fixed | The ADR-002 Decision is options-only (new Q0: five parts or parser only). The owner ruling it cites is now recorded in AD-001's Decisions. |
| FND-013 | fixed | interface-001 `outcome_source` now says "passed through verbatim to the FR-016 witness join". TC-035 Status lists `bounded_kani_replay.rs`. The `runtime::execute` mention is gone from FR-013. |
| FND-014 | fixed | FR-024-AC-2 is a single claim. AC-7 is split into AC-7 and AC-8 (the old AC-8 and AC-9 are now AC-9 and AC-10). FR-025-AC-2 is split into AC-2 and AC-6. The matrix, TC-035 and TC-036 match. |
| FND-015 | fixed | "Linear IR-309" is removed and the AD-001 no-convergence statement is restored. FR-018 drops "yet". The remaining "no requirement owns" is an Out of Scope fact, not a requirement statement. |

Round 2.

| FND | outcome | reason |
| --- | --- | --- |
| FND-016 | fixed | The fix changes exactly six `/// Trace:` doc-comment lines under `src` and `tests` and nothing else. No `FR-007-AC-4` tag remains under `src/` or `tests/`. `quire coverage` now reports FR-007-AC-4 `backed: false`, which matches the matrix row marking it Planned. `cargo fmt --check` and `cargo clippy --locked --all-targets -D warnings` both exit 0. |
| FND-017 | fixed | FR-016 adds a partition row and FR-016-AC-13: a `Witness` arm settling `reproduced-with-evaluated-witness` with any category other than `violation` is a mismatch. `WitnessSettlement` has two variants (`ReproducedWithEvaluatedWitness`, `Inconclusive`), and `settle` takes `category` as an independent input. The ordered rows now cover every `Err` and `Ok` return of `replay`, so each condition gives exactly one outcome. The matrix and TC-026 carry AC-13. |
| FND-018 | fixed | FR-024, FR-007 and AD-001 no longer carry the ruling date, `QSL-317` or "unassigned". They cite the `qsl-replay` API and AD-001, and say the upstream amendment is pending. The dated ruling appears only in ADR-002 (Context, and a pointer from Status), which is a decision record. The remaining Linear ids under `spec/` (FR-022, the test matrix, ADR-003's IR-241) are on `main` or are cited evidence, and this fix round did not add them. |

### Round 2 verdict

All 18 findings are fixed, and none has a latest outcome of still-open. This round adds no new
finding. `make spec` is byte-identical to `origin/main` (both exit 2 from AP-001/MP-001).
`status_lies` is empty. `grep -rE 'WP[0-9]' spec reviews src tests` finds nothing. The committed
SR-040 equals the round-1 reviewer copy, with line 62 still redacted. Mergeable.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-016 | low | FR-007-AC-4 now states the QSL `Input`-arm target, but six trace tags for FR-007-AC-4 remain on tests that exercise Contract IR's `replay_counterexample`. They back the rewritten criterion with tests of the retired behaviour, so `quire coverage` counts it as backed although the matrix says Planned. | src/bounded_kani_replay.rs:62, src/bounded_kani_corpus.rs:838, src/bounded_kani_corpus.rs:1162, src/bounded_kani_corpus.rs:1387, src/bounded_kani_corpus.rs:1409, tests/it/bounded_kani_corpus.rs:347 |
| FND-017 | low | The FR-016 partition does not cover every result `replay` can return. `WitnessArmResult::settle` takes `category` from its caller independently of the settlement, so a `Witness` arm that settles `ReproducedWithEvaluatedWitness` with a category other than `Violation` matches no row, and the claim "exactly one of the outcomes" fails for it. | spec/functional/complete-v1/FR-016-witness-native-replay.md:76-86 |
| FND-018 | low | The fix put process narration and a ticket id into requirement and architecture text: "the owner's ruling of 2026-09-28", "the `qsl-replay` facade work is QSL-317, and the AD-016 amendment is unassigned". This is status that will go stale. It belongs in the ADR or the ticket. | spec/functional/complete-v1/FR-024-counterexample-envelope-intake.md:35-40, spec/assurance/AD-001-codegen-architecture.md:158-165, spec/functional/FR-007-bounded-kani-profile-corpus.md:124 |

### Round 1 verdict

All 15 original findings are fixed, and none is still open. The fix round adds three
LOW findings and no MEDIUM or HIGH. `make spec` is byte-identical to `origin/main` (both exit 2 from
AP-001/MP-001). The changed files validate with `quire validate` (exit 0). `status_lies` is empty,
and unbacked rows go from 11 to 32, all new Planned rows. The interface-001 edit touches only the
single-line `outcome_source` scalar, which `tests/it/interface_001.rs` does not parse. Mergeable
from the spec side. FND-016 to FND-018 can be fixed in this PR or deferred at the leader's call.
