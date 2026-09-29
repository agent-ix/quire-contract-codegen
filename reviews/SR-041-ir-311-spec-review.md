---
id: "SR-041"
title: "PR 186 spec review: ADR-001 to ADR-004 accepted, FR-001/FR-003/FR-007 retired, FR-026 to FR-029"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@a7c91f13bfb1f68f91a88c7a6c83c9184fd251cd; spec/assurance/AD-001-codegen-architecture.md, spec/decisions/ADR-001-overlapping-generators-and-input-models.md, spec/decisions/ADR-002-backend-adapter-boundary.md, spec/decisions/ADR-003-kani-tractability.md, spec/decisions/ADR-004-generated-subject-abi-open-decisions.md, spec/functional/FR-001-deterministic-oracles.md, spec/functional/FR-003-kani-lowering.md, spec/functional/FR-007-bounded-kani-profile-corpus.md, spec/functional/complete-v1/FR-014-exact-scalar-oracles.md, spec/functional/complete-v1/FR-015-bounded-kani-obligations.md, spec/functional/complete-v1/FR-017-pinned-kani-execution-evidence.md, spec/functional/complete-v1/FR-022-routed-generation.md, spec/functional/complete-v1/FR-024-counterexample-envelope-intake.md, spec/functional/complete-v1/FR-025-generated-subject-abi.md, spec/functional/complete-v1/FR-026-backend-adapter-contract.md, spec/functional/complete-v1/FR-027-single-version-profile.md, spec/functional/complete-v1/FR-028-bounded-proof-ceilings.md, spec/functional/complete-v1/FR-029-run-outcome-terminal-record.md, spec/index.md, spec/test-matrix.md, spec/test/TC-005-proof-dependencies.md, spec/test/TC-014-numeric-state-kani.md, spec/test/TC-023-bounded-kani-profile-corpus.md, spec/test/complete-v1/TC-024-exact-scalar-oracles.md, spec/test/complete-v1/TC-025-bounded-kani-obligations.md, spec/test/complete-v1/TC-036-generated-subject-abi.md, spec/test/complete-v1/TC-037-backend-adapter-contract.md, spec/test/complete-v1/TC-038-single-version-profile.md, spec/test/complete-v1/TC-039-bounded-proof-ceilings.md, spec/test/complete-v1/TC-040-run-outcome-terminal-record.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/ADR-001
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/ADR-002
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/ADR-003
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/ADR-004
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: reviews
---

# SR-041: PR 186 spec review

## Summary

Ticket: IR-311 (primary), IR-312 (ADR-002, ADR-003 and ADR-004 parts). PR:
agent-ix/quire-contract-codegen#186, head `a7c91f1`, base `23dcc3d`. Spec-only, 30 files under
`spec/`. Method: spec-review, with the integrity and EARS sub-analyses folded into this file, as
SR-039 and SR-040 did.

The PR was checked against the owner rulings recorded on IR-311 and IR-312, and against the
restated OQ-3 ruling on IR-313. The restated ruling says a profile capability-matrix Inconclusive is
a negotiation-stage disposition: QSpec FR-331 `dispositions`, settled `unsupported`, with no
artifact and no terminal value. A vacuous proof maps to QSL's inconclusive terminal value with the
vacuity cause once QSL has one. Until then it maps to a typed absence, never to a substitute. I
checked the QSpec text myself at `quire-specification` `origin/main`,
`spec/objects/interfaces/FR-331-backend-provider-envelope.md`:

- FR-331-AC-2: an item settled `unsupported`, `requires-bound` or `invalid-request` emits no
  artifact.
- FR-331-AC-4: an empty candidate set settles `unsupported`, with a warning naming the item's kind.
- FR-331-AC-8: a Kani proof run with zero SUCCESS checks records `inconclusive` with a typed
  vacuity cause, never `proved`.

At the pinned `qsl-replay` rev `20ba521`, `qsl-replay/src/proof_result.rs:104-127`, `TerminalValue`
has no inconclusive arm. It encodes vacuity as `Proved { success_checks: 0 }`, which is the
conformance bug QSL is fixing on its side.

Measured on this head (logs are in the reviewer scratchpad `cg186-review/logs/`):

- `quire coverage --scope . --strict` exits 1 on both sides. It reports 32 unbacked rows and 0
  contradicted on `origin/main`, and 59 unbacked and 0 contradicted on head. The +27 are the new
  FR-025-AC-7/8, FR-026 to FR-029 and TC-037 to TC-040 rows. The FR-025 Partial row that became
  Planned drops out of the count. The author's "32 -> 59, +27, 0 contradicted" is confirmed.
- `make spec` exits 2 on both sides. The only errors are the existing AP-001 and MP-001
  frontmatter failures. The only warning is the existing FR-014 EARS warning, which moved from line
  267 to line 269.
- Embedded spec files: `src/oracle.rs:39` includes FR-001 into `generator_implementation_digest()`,
  and `tests/it/oracle_generation.rs:112` recomputes that digest from the same bytes. `build.rs:112`
  only watches the file. `src/kani.rs:35` includes FR-003 into `kani_implementation_digest()`. No
  golden file pins either digest. `cargo test --locked --test it oracle_generation` exits 0 (12
  passed), and `cargo test --locked --test it kani` exits 0 (45 passed, 5 ignored). The edits change
  runtime digests only.
- Ids: FR-026 to FR-029 and TC-037 to TC-040 are free on every remote branch, and #186 is the only
  open PR.
- Public-repo rules: no private work-package tokens, no private research-repository path, no dates and no Linear ids in the added
  lines. ADR-003's `agent-ix/quire-contract-runtime#78` citation predates this PR and points to a
  public repository. There is no compatibility, legacy or versioning text.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-029 maps the vacuous-proof reason to `Proved { success_checks: 0 }`. That is the substitute the restated ruling forbids: FR-331-AC-8 requires `inconclusive` with a vacuity cause, never `proved`. Until QSL adds that value, the map must return a typed absence. `CoverUnsatisfied` is also a vacuity outcome (`src/kani_execution.rs:516-524`), so it belongs on the same vacuity path, not in a generic "undecided" bucket. AC-1, TC-040 step 1 and ADR-002 Q3 repeat the stale mapping. | spec/functional/complete-v1/FR-029-run-outcome-terminal-record.md:46-47, spec/functional/complete-v1/FR-029-run-outcome-terminal-record.md:64, spec/test/complete-v1/TC-040-run-outcome-terminal-record.md:25, spec/decisions/ADR-002-backend-adapter-boundary.md:88-93 |
| FND-002 | medium | FR-029, ADR-002 and AD-001 say every other inconclusive reason waits on a QSL "ADR-013 O-24 amendment". The restated ruling asks QSL for no new terminal value, so these typed absences have no path to becoming values. The reasons are `NoVerdict`, `FailedWithoutCounterexample`, `MissingCoverSummary` and `UnwindBoundExhausted`. The blanket "no outcome maps to `Failed`" also rules out the value QSL defines for a tool failure, which is what `NoVerdict` (build, launcher or solver failure) is. Each reason needs an existing value, or a stated reason why none fits. | spec/functional/complete-v1/FR-029-run-outcome-terminal-record.md:52-56, spec/functional/complete-v1/FR-029-run-outcome-terminal-record.md:65, spec/decisions/ADR-002-backend-adapter-boundary.md:29-30, spec/assurance/AD-001-codegen-architecture.md:235-236 |
| FND-003 | high | FR-015-AC-25 and its Behavior line require "exactly one supported, refused or inconclusive disposition". This was carried from FR-007's Contract IR profile vocabulary. It contradicts the FR-331 disposition set that FR-015's own `ObligationDisposition` implements (`Supported`, `RequiresBound`, `Unsupported`, `InvalidRequest`; `src/kani_obligations.rs:421-441`). It also contradicts the restated ruling: a capability-matrix Inconclusive settles `unsupported` at negotiation (FR-331-AC-2, FR-331-AC-4). TC-025 step 7 asks for "inconclusive items", which no generation input can produce. | spec/functional/complete-v1/FR-015-bounded-kani-obligations.md:158, spec/functional/complete-v1/FR-015-bounded-kani-obligations.md:191, spec/test/complete-v1/TC-025-bounded-kani-obligations.md:89 |
| FND-004 | high | FR-014 now derives integer `eq`/`ne` and Boolean `eq`/`ne` "as it does for every other derivable operation", which is an `exact` scalar kernel operation. FR-018 states that the runtime's `check_equality`/`CheckedEquality::evaluate` relation is the only equality and that generated oracles call it. The resolution the author intends (FR-014 owns the node but emits the runtime `check_equality` call) is written nowhere. FR-014 also does not say what runtime call or `Outcome` shape the Boolean connectives use. FR-018's mutation note still names the plain-`bool` Boolean closures of `src/oracle.rs`. | spec/functional/complete-v1/FR-014-exact-scalar-oracles.md:280-283, spec/functional/complete-v1/FR-014-exact-scalar-oracles.md:330, spec/functional/complete-v1/FR-018-composite-equality-oracles.md:49-54 |
| FND-005 | medium | ADR-001 Q3 says every generator reads `CheckedPackageV2`, with no second input path. AD-001 says the same ("Every generator reads it"). FR-008 to FR-013, FR-002, FR-004, FR-005 and interface-001 still specify the V1 `BoundPackage` as their target input, and the matrix keeps FR-008 to FR-013 `Covered`. The two sets of statements contradict each other, and nothing in the spec marks the strategy side as unresolved. This can be deferred to IR-364 if ADR-001's Consequences name it. | spec/decisions/ADR-001-overlapping-generators-and-input-models.md:84-87, spec/functional/strategies/FR-008-bound-domain-strategy-admission.md:29, spec/functional/strategies/FR-008-bound-domain-strategy-admission.md:56 |
| FND-006 | medium | FR-028's static refusal and shadow selection are ambiguous in four ways. (a) The "tractability record" has no stated source or owner: committed adapter data, or request input? (b) "The request's ceilings do not exceed those of the recorded run" is undefined for two ceilings: raising the memory ceiling unlocks a family whose recorded run timed out. (c) With a shadow present and a raised ceiling, the Behavior lines at 69-75 say production (AC-6) and shadow (AC-7) at once. (d) The shadow and refinement inputs have no owning requirement: Contract Runtime has none, and IR-340 tracks it. | spec/functional/complete-v1/FR-028-bounded-proof-ceilings.md:33-37, spec/functional/complete-v1/FR-028-bounded-proof-ceilings.md:69-75, spec/functional/complete-v1/FR-028-bounded-proof-ceilings.md:91-92 |
| FND-007 | medium | The matrix is not honest about TC-001 to TC-003. They stay `Covered`, and the new prose says "covered by their existing tests", although most of their criteria are now retired FR-001 and FR-003 criteria, tested only on the V1 path slated for deletion. TC-002 also lists FR-005-AC-1 and FR-005-AC-5, which the FR table marks Planned. TC-007 still lists the retired FR-003-AC-2. TC-005, TC-014 and TC-023 were marked retired on the same grounds. | spec/test-matrix.md:331-333, spec/test-matrix.md:337, spec/test-matrix.md:370 |
| FND-008 | low | ADR-001 Q1 says FR-003's "post-state result binding" is carried into FR-015. FR-015-AC-19 (under ADR-004 Q2) replaces the by-value result with a `&mut` state argument instead. The same `&mut` rule is also stated twice, in FR-015-AC-19 and FR-025-AC-8, so it has two owners. | spec/decisions/ADR-001-overlapping-generators-and-input-models.md:60-63, spec/functional/complete-v1/FR-015-bounded-kani-obligations.md:185, spec/functional/complete-v1/FR-025-generated-subject-abi.md:84 |
| FND-009 | low | Some retired FR-007 rows do not name a carrier criterion. CON-2, CON-4 and AC-5 restate a fact and name no carrier, and AC-3 names FR-028 as a whole instead of FR-028-AC-2 and FR-028-AC-3. | spec/functional/FR-007-bounded-kani-profile-corpus.md:45, spec/functional/FR-007-bounded-kani-profile-corpus.md:47, spec/functional/FR-007-bounded-kani-profile-corpus.md:55, spec/functional/FR-007-bounded-kani-profile-corpus.md:57 |
| FND-010 | low | Several new ACs are compound, each holding two or three independently failing claims: FR-015-AC-20 and AC-22, FR-028-AC-1, AC-6, AC-7 and AC-8, FR-014-AC-35 and AC-37 (package-name aliasing appended to the attestation claim), and FR-029-AC-2. | spec/functional/complete-v1/FR-015-bounded-kani-obligations.md:186, spec/functional/complete-v1/FR-028-bounded-proof-ceilings.md:91, spec/functional/complete-v1/FR-014-exact-scalar-oracles.md:332 |
| FND-011 | low | FR-026's "When a second backend is added, it shall register by…" describes a development procedure, not system behaviour. It has no AC and cannot be tested as written. It belongs in ADR-002 Q4 only, or as a compile-time exhaustiveness criterion. | spec/functional/complete-v1/FR-026-backend-adapter-contract.md:54-57 |

## Sub-analyses

### Integrity

Retired requirements keep their files and criterion numbers, and each retired criterion names what
carries it, with the FR-007 exceptions in FND-009. I spot-checked every carrier the matrix names:
FR-014-AC-1, AC-4 and AC-5, FR-015-AC-2, AC-3, AC-4, AC-9, AC-10 and AC-11, FR-024-AC-5 and
FR-025-AC-3. Each states the behaviour it is said to carry. The matrix marks TC-005, TC-014 and
TC-023 `⛔ Retired`, and the matrix rows for FR-001, FR-003 and FR-007 are `⛔` with carriers.
Every carried and new criterion is `🚧 Planned`, and no row was narrowed to fit the code.
FR-014-AC-18 and FR-015-AC-15 were correctly downgraded to Planned, because their tests assert
`integer.eq` is refused (`tests/it/exact_scalar_generation.rs:2003`,
`tests/it/routed_generation.rs:863`). The index area table and the directory table agree with the
new files. Findings: FND-005, FND-007, FND-009.

### EARS

The new Behavior lines in FR-026 to FR-029 and the additions to FR-014, FR-015 and FR-025 use
ubiquitous, event, unwanted and optional forms, and `make spec` raises no new EARS warning. The
exception is FR-026's second-backend line (FND-011). Findings: FND-010, FND-011.

### Rulings conformance

- ADR-001: Q1 names FR-015 as the one generator, carries FR-003 and FR-007, retires the injected
  closure and drops stubbing. Q2 gives FR-014 all oracles, Boolean and integer included. Q3 has one
  input model and no adapter. All three match the IR-311 ruling. Residual: FND-004, FND-005.
- ADR-002: one trait, closed-enum match, one version profile, one total map, and a second backend
  with its own adapter and evidence schema. This matches, except the Q3 map, which is stale
  against IR-313 (FND-001, FND-002).
- ADR-003: production code where it verifies, otherwise a bounded shadow plus refinement. Ceilings
  sit in the harness identity with their own reasons and a static refusal, and the stubbing ban
  stays. The owner principle (best coverage, every proof bounded, a tightened bound recorded so it
  never reads as a full proof) is stated in the Decision and in FR-028-AC-8. Residual: FND-006.
- ADR-004: a family gets a type row only when its witness decodes losslessly, state is passed by
  `&mut`, and the frame lowering is left pending upstream with nothing invented. It matches.
- Every ADR is `Accepted` with a plain Decision section.
- AD-001 stays `proposed`. That is right for now, because its execution view step 7 and its risk
  list carry the stale FR-029 text (FND-001, FND-002). It should become accepted once those are
  fixed.

### Author-listed contradictions, judged

1. V1 retirement reaching the strategy side: a real contradiction (FND-005). It can be deferred to
   IR-364 only if the spec says so.
2. Integer `eq`/`ne` against FR-018: unresolved in the text (FND-004).
3. The two tests contradicting the spec: handled honestly, because the rows are Planned. No finding.
4. FR-003's by-value post-state against `&mut`: the ruling settles it, and only ADR-001's wording
   is off (FND-008).
5. Per-family coverage against FR-017-AC-8: no contradiction. FR-028 only names the family on each
   record and computes no aggregate. No finding.
6. The shadow model with no Contract Runtime owner: a real gap (FND-006 part d), tracked on IR-340.

## Verdict

Not mergeable. There are three HIGH findings: the stale vacuous-proof mapping (FND-001), an
"inconclusive" generation disposition that contradicts FR-331 and the restated ruling (FND-003),
and an unresolved FR-014/FR-018 equality ownership conflict (FND-004). There are four MEDIUM
findings: FND-002 and FND-005 to FND-007. All are text fixes inside this PR. FND-005 may be
deferred to IR-364, and FND-006(d) to IR-340, provided the spec names the gap. The LOW findings are
editorial.

## Dispositions

Round 1, reviewed at `41a25a86e62b5833309787a0a73ce042a7622085`. The fix commits are `52e5e2d`
and `41a25a8`, which only rewords one FR-028 line into EARS form. The rulings in force at this
round: the IR-313 correction 2 comment keeps vacuous mapped to `Proved { success_checks: 0 }` and
asks QSL for nothing. I checked this at `qsl-replay` `20ba521`. `TerminalValue::category()` returns
`Inconclusive` for `Proved { success_checks: 0 }`, and `vacuous_proof_cause()` returns
`KaniVacuousProof`, which meets QSpec FR-331-AC-8. The capability-matrix `unsupported` disposition
is settled at negotiation, with a warning naming the capability kind.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed 52e5e2d | FR-029, ADR-002 Q3 and AD-001 map the vacuous-proof reason and `cover-unsatisfied` to `Proved { success_checks: 0 }`. Under the IR-313 correction 2 comment and at `qsl-replay` `20ba521`, QSL reads that value as category `inconclusive` with cause `KaniVacuousProof`, not as a substitute. FR-029 also states that an item settled `unsupported`, `requires-bound` or `invalid-request` at negotiation has no run and no terminal value, and that an `unsupported` item's warning names its `quire.capability-kind/v1` kind (FR-290). |
| FND-002 | fixed 52e5e2d | Every O-24-pending sentence is gone from FR-029, ADR-002 Status and Q3, and AD-001's risks. Each inconclusive reason now has a decided row: unwind exhaustion maps to `Incomplete(ResourceExhausted)`, no-verdict maps to `Failed`, and two reasons map to typed absences, each with a stated reason. The blanket ban on `Failed` is lifted, and only `Tested` stays banned. The unwind row is correct: QSL ADR-013's O-16 row "incomplete (timeout, cancellation, bound exhaustion)" maps to `ResourceExhausted`, its bound table lists the Kani unwind as a backend tool budget, and `IncompleteCause::ResourceExhausted` is "exhausted a configured resource bound before completing". The residual is recorded as FND-012. |
| FND-003 | fixed 52e5e2d | FR-015 Behavior and FR-015-AC-25 now require exactly one `ObligationDisposition` from the FR-331 set (`supported`, `requires-bound`, `unsupported`, `invalid-request`), matching `src/kani_obligations.rs:421-441`. TC-025 step 7 requests `supported`, `requires-bound` and `unsupported` items. |
| FND-004 | fixed 52e5e2d | FR-014 now states that the integer and Boolean `eq`/`ne` oracles call `TypeEnvironment::check_equality` and then `CheckedEquality::evaluate`. Connectives call `exact::evaluate_boolean` or `exact::evaluate_boolean_short_circuit`, each returning `Outcome<bool>`. FR-018 names FR-014 as the owner of these nodes, and its mutation note no longer cites `src/oracle.rs`. At Contract Runtime `ed0a04b`, `evaluate_boolean`, `evaluate_boolean_short_circuit`, `BooleanConnective`, `ShortCircuitConnective`, `TypeEnvironment::check_equality` and `CheckedEquality::evaluate` all exist and are exported from `exact` (`src/exact/mod.rs`, `numeric.rs:383-432`, `equality.rs:108` and `:162`). |
| FND-005 | fixed 52e5e2d | ADR-001's Consequences name FR-002, FR-004, FR-005, FR-008 to FR-013, NFR-004, interface-001 and their TCs as still stating the V1 input, and say they contradict the decision until restated. AD-001's generation view and the matrix prose say the same. Restating them is IR-364. |
| FND-006 | fixed 52e5e2d | The tractability record is committed data the Kani adapter owns, beside its version profile, and each entry records which ceiling was exceeded. An entry applies only while the request does not raise that ceiling. The shadow applies only while the entry applies, so raising the ceiling means production. FR-028-AC-11 tests the case where only the other ceiling is raised. The missing Contract Runtime shadow requirement is named (IR-340). `41a25a8` rewrites the refusal line into EARS form. |
| FND-007 | fixed 52e5e2d | TC-001, TC-002 and TC-003 are `🚧 Planned`, with the reasons in the row and in the prose. TC-007 no longer lists FR-003-AC-2, and its `verifies FR-003` edge is removed. |
| FND-008 | fixed 52e5e2d | ADR-001 Q1 now says the bounded state transition is carried "with its state bound by `&mut` reference as ADR-004 Q2 decides". FR-015 defers state binding to FR-025, and FR-015-AC-19 no longer restates `&mut`, so FR-025-AC-8 is its only owner. |
| FND-009 | fixed 52e5e2d | FR-007-CON-2, CON-4 and AC-5 now say "Not carried" with a reason. FR-007-AC-3 names FR-028-AC-2 and FR-028-AC-3, and the matrix agrees. |
| FND-010 | fixed 52e5e2d | The compound criteria are split. FR-015-AC-20 and AC-22 split into AC-26 and AC-27. FR-028-AC-1, AC-6 and AC-8 split into AC-9, AC-10 and AC-12. FR-014-AC-35 and AC-37 split into AC-38 and AC-39. FR-029 goes from 3 ACs to 7. TC-024, TC-025, TC-039 and TC-040 follow. |
| FND-011 | fixed 52e5e2d | The second-backend procedure is removed from FR-026 Behavior, and the Description points to ADR-002 Q4. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-012 | medium | FR-029 now maps two run outcomes, `inconclusive` with the failure-without-counterexample reason and with the missing-cover-summary reason, to a typed absence permanently. The QSL amendment that was once expected to give them values is no longer coming. A supported item whose run ends that way then has no FR-331 result at all. QSpec FR-331's `accounting` requires "exactly one terminal record for every requested item". Each of the two reasons needs one of the eight FR-331 result values, or a stated negotiation-side refusal. Candidates: `Failed`, because the tool broke its own output contract, or `inconclusive`, though that one carries only the vacuity cause today. | spec/functional/complete-v1/FR-029-run-outcome-terminal-record.md:62-63, spec/functional/complete-v1/FR-029-run-outcome-terminal-record.md:71-73, spec/functional/complete-v1/FR-029-run-outcome-terminal-record.md:86 |
| FND-013 | low | FR-028 states what is missing rather than what is: "No Contract Runtime requirement states them yet, so no family has a shadow until one does", and the same "yet" appears in Dependencies. That is status narration in requirement text. The rule it encodes, that no shadow exists unless one is supplied, is already FR-028's behaviour. | spec/functional/complete-v1/FR-028-bounded-proof-ceilings.md:40-41, spec/functional/complete-v1/FR-028-bounded-proof-ceilings.md:110 |

### Round 1 verdict

All 11 original findings are fixed in `52e5e2d` and `41a25a8`, and none is still open. The round
adds one MEDIUM finding (FND-012) and one LOW finding (FND-013).

The author judged unwind-bound exhaustion to map to `Incomplete(ResourceExhausted)`. That is
correct and should not be a typed absence. No-verdict mapping to `Failed` fits
`TerminalValue::Failed` ("the tool itself failed").

AD-001 `accepted` is warranted. Its FR-029 text now matches the correction 2 ruling, and its
Current-state section records what lags. `reviews/SR-041-ir-311-spec-review.md` equals the round-0
reviewer copy except one line, which has the two private-repo tokens redacted. The added non-review
text has no work-package tokens, no Linear ids and no dates.

Measured:

- `quire coverage --strict` exits 1 on both sides. It reports 32 unbacked and 0 contradicted on
  main, and 67 and 0 on head. The +35 are exactly FR-025-AC-7/8, FR-026 to FR-029 with every AC,
  and TC-037 to TC-040, all Planned. No row went from backed to unbacked.
- `make spec` exits 2 on both sides, from AP-001 and MP-001 only. The FR-014 EARS warning is the
  existing one, now at line 278.
- `cargo test --locked --test it oracle_generation` exits 0, 12 passed.
- `cargo test --locked --test it kani` exits 0, 45 passed and 5 ignored.

Not mergeable until FND-012 is fixed or deferred with a reason. FND-013 is editorial.
