---
id: "SR-041"
title: "PR 186 spec review: ADR-001 to ADR-004 accepted, FR-001/FR-003/FR-007 retired, FR-026 to FR-029"
type: SpecReview
scope: "agent-ix/quire-contract-codegen; spec/assurance/AD-001-codegen-architecture.md, spec/decisions/ADR-001-overlapping-generators-and-input-models.md, spec/decisions/ADR-002-backend-adapter-boundary.md, spec/decisions/ADR-003-kani-tractability.md, spec/decisions/ADR-004-generated-subject-abi-open-decisions.md, spec/functional/FR-001-deterministic-oracles.md, spec/functional/FR-003-kani-lowering.md, spec/functional/FR-007-bounded-kani-profile-corpus.md, spec/functional/complete-v1/FR-014-exact-scalar-oracles.md, spec/functional/complete-v1/FR-015-bounded-kani-obligations.md, spec/functional/complete-v1/FR-017-kani-execution-evidence.md, spec/functional/complete-v1/FR-022-routed-generation.md, spec/functional/complete-v1/FR-024-counterexample-envelope-intake.md, spec/functional/complete-v1/FR-025-generated-subject-abi.md, spec/functional/complete-v1/FR-026-backend-adapter-contract.md, spec/functional/complete-v1/FR-027-single-version-profile.md, spec/functional/complete-v1/FR-028-bounded-proof-ceilings.md, spec/functional/complete-v1/FR-029-run-outcome-terminal-record.md, spec/index.md, spec/test-matrix.md, spec/test/TC-005-proof-dependencies.md, spec/test/TC-014-numeric-state-kani.md, spec/test/TC-023-bounded-kani-profile-corpus.md, spec/test/complete-v1/TC-024-exact-scalar-oracles.md, spec/test/complete-v1/TC-025-bounded-kani-obligations.md, spec/test/complete-v1/TC-036-generated-subject-abi.md, spec/test/complete-v1/TC-037-backend-adapter-contract.md, spec/test/complete-v1/TC-038-single-version-profile.md, spec/test/complete-v1/TC-039-bounded-proof-ceilings.md, spec/test/complete-v1/TC-040-run-outcome-terminal-record.md"
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
agent-ix/quire-contract-codegen#186. Spec-only, 30 files under
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

In `qsl-replay/src/proof_result.rs:104-127`, `TerminalValue`
has no inconclusive arm. It encodes vacuity as `Proved { success_checks: 0 }`, which is the
conformance bug QSL is fixing on its side.

Measured on this head:

- `quire coverage --scope . --strict` exits 1 on both sides. It reports 32 unbacked rows and 0
  contradicted on `origin/main`, and 59 unbacked and 0 contradicted on head. The +27 are the new
  FR-025-AC-7/8, FR-026 to FR-029 and TC-037 to TC-040 rows. The FR-025 Partial row that became
  Planned drops out of the count. The author's "32 -> 59, +27, 0 contradicted" is confirmed.
- `make spec` exits 2 on both sides. The only errors are the existing AP-001 and MP-001
  frontmatter failures. The only warning is the existing FR-014 EARS warning, which moved from line
  267 to line 269.
- `cargo test --locked --test it oracle_generation` exits 0 (12
  passed), and `cargo test --locked --test it kani` exits 0 (45 passed, 5 ignored).
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
| FND-010 | low | Several new ACs are compound, each holding two or three independently failing claims: FR-015-AC-20 and AC-22, FR-028-AC-1, AC-6, AC-7 and AC-8, FR-014-AC-35 and AC-37, and FR-029-AC-2. | spec/functional/complete-v1/FR-015-bounded-kani-obligations.md:186, spec/functional/complete-v1/FR-028-bounded-proof-ceilings.md:91, spec/functional/complete-v1/FR-014-exact-scalar-oracles.md:332 |
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

Round 1. The rulings in force at this
round: the IR-313 correction 2 comment keeps vacuous mapped to `Proved { success_checks: 0 }` and
asks QSL for nothing. I checked this against `qsl-replay`. `TerminalValue::category()` returns
`Inconclusive` for `Proved { success_checks: 0 }`, and `vacuous_proof_cause()` returns
`KaniVacuousProof`, which meets QSpec FR-331-AC-8. The capability-matrix `unsupported` disposition
is settled at negotiation, with a warning naming the capability kind.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-029, ADR-002 Q3 and AD-001 map the vacuous-proof reason and `cover-unsatisfied` to `Proved { success_checks: 0 }`. Under the IR-313 correction 2 comment and in `qsl-replay`, QSL reads that value as category `inconclusive` with cause `KaniVacuousProof`, not as a substitute. FR-029 also states that an item settled `unsupported`, `requires-bound` or `invalid-request` at negotiation has no run and no terminal value, and that an `unsupported` item's warning names its `quire.capability-kind/v1` kind (FR-290). |
| FND-002 | fixed | Every O-24-pending sentence is gone from FR-029, ADR-002 Status and Q3, and AD-001's risks. Each inconclusive reason now has a decided row: unwind exhaustion maps to `Incomplete(ResourceExhausted)`, no-verdict maps to `Failed`, and two reasons map to typed absences, each with a stated reason. The blanket ban on `Failed` is lifted, and only `Tested` stays banned. The unwind row is correct: QSL ADR-013's O-16 row "incomplete (timeout, cancellation, bound exhaustion)" maps to `ResourceExhausted`, its bound table lists the Kani unwind as a backend tool budget, and `IncompleteCause::ResourceExhausted` is "exhausted a configured resource bound before completing". The residual is recorded as FND-012. |
| FND-003 | fixed | FR-015 Behavior and FR-015-AC-25 now require exactly one `ObligationDisposition` from the FR-331 set (`supported`, `requires-bound`, `unsupported`, `invalid-request`), matching `src/kani_obligations.rs:421-441`. TC-025 step 7 requests `supported`, `requires-bound` and `unsupported` items. |
| FND-004 | fixed | FR-014 now states that the integer and Boolean `eq`/`ne` oracles call `TypeEnvironment::check_equality` and then `CheckedEquality::evaluate`. Connectives call `exact::evaluate_boolean` or `exact::evaluate_boolean_short_circuit`, each returning `Outcome<bool>`. FR-018 names FR-014 as the owner of these nodes, and its mutation note no longer cites `src/oracle.rs`. In Contract Runtime, `evaluate_boolean`, `evaluate_boolean_short_circuit`, `BooleanConnective`, `ShortCircuitConnective`, `TypeEnvironment::check_equality` and `CheckedEquality::evaluate` all exist and are exported from `exact` (`src/exact/mod.rs`, `numeric.rs:383-432`, `equality.rs:108` and `:162`). |
| FND-005 | fixed | ADR-001's Consequences name FR-002, FR-004, FR-005, FR-008 to FR-013, NFR-004, interface-001 and their TCs as still stating the V1 input, and say they contradict the decision until restated. AD-001's generation view and the matrix prose say the same. Restating them is IR-364. |
| FND-006 | fixed | The tractability record is committed data the Kani adapter owns, beside its version profile, and each entry records which ceiling was exceeded. An entry applies only while the request does not raise that ceiling. The shadow applies only while the entry applies, so raising the ceiling means production. FR-028-AC-11 tests the case where only the other ceiling is raised. The missing Contract Runtime shadow requirement is named (IR-340). The refusal line is rewritten into EARS form. |
| FND-007 | fixed | TC-001, TC-002 and TC-003 are `🚧 Planned`, with the reasons in the row and in the prose. TC-007 no longer lists FR-003-AC-2, and its `verifies FR-003` edge is removed. |
| FND-008 | fixed | ADR-001 Q1 now says the bounded state transition is carried "with its state bound by `&mut` reference as ADR-004 Q2 decides". FR-015 defers state binding to FR-025, and FR-015-AC-19 no longer restates `&mut`, so FR-025-AC-8 is its only owner. |
| FND-009 | fixed | FR-007-CON-2, CON-4 and AC-5 now say "Not carried" with a reason. FR-007-AC-3 names FR-028-AC-2 and FR-028-AC-3, and the matrix agrees. |
| FND-010 | fixed | The compound criteria are split. FR-015-AC-20 and AC-22 split into AC-26 and AC-27. FR-028-AC-1, AC-6 and AC-8 split into AC-9, AC-10 and AC-12. FR-014-AC-35 and AC-37 split into AC-38 and AC-39. FR-029 goes from 3 ACs to 7. TC-024, TC-025, TC-039 and TC-040 follow. |
| FND-011 | fixed | The second-backend procedure is removed from FR-026 Behavior, and the Description points to ADR-002 Q4. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-012 | medium | FR-029 now maps two run outcomes, `inconclusive` with the failure-without-counterexample reason and with the missing-cover-summary reason, to a typed absence permanently. The QSL amendment that was once expected to give them values is no longer coming. A supported item whose run ends that way then has no FR-331 result at all. QSpec FR-331's `accounting` requires "exactly one terminal record for every requested item". Each of the two reasons needs one of the eight FR-331 result values, or a stated negotiation-side refusal. Candidates: `Failed`, because the tool broke its own output contract, or `inconclusive`, though that one carries only the vacuity cause today. | spec/functional/complete-v1/FR-029-run-outcome-terminal-record.md:62-63, spec/functional/complete-v1/FR-029-run-outcome-terminal-record.md:71-73, spec/functional/complete-v1/FR-029-run-outcome-terminal-record.md:86 |
| FND-013 | low | FR-028 states what is missing rather than what is: "No Contract Runtime requirement states them yet, so no family has a shadow until one does", and the same "yet" appears in Dependencies. That is status narration in requirement text. The rule it encodes, that no shadow exists unless one is supplied, is already FR-028's behaviour. | spec/functional/complete-v1/FR-028-bounded-proof-ceilings.md:40-41, spec/functional/complete-v1/FR-028-bounded-proof-ceilings.md:110 |

### Round 1 verdict

All 11 original findings are fixed, and none is still open. The round
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

### Round 2

- `make spec` exits 0 on both.
- `quire coverage --strict` exits 1 on both: main backs 198 of 265 rows, head 197 of 291.
- The one row that lost backing is FR-017-AC-10, which was deleted (FND-017).

| FND | outcome | reason |
| --- | --- | --- |
| FND-012 | fixed | FR-029 maps the failure-without-counterexample and missing-cover-summary reasons to `Failed`, and states why (spec/functional/complete-v1/FR-029-run-outcome-terminal-record.md:60-72) |
| FND-013 | fixed | the "yet" status narration is gone from FR-028 |

## New findings (disposition pass 2)

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-014 | high | This is an id ledger (P0 case 1). ADR-001 rules that retired FR-001, FR-003 and FR-007 "keep their file and criterion numbers, marked retired, with each criterion naming the criterion that now carries it". The three FR files are now pure old-to-new maps: Inputs, Outputs and Behavior all read "None. The requirement is retired", and each AC row reads "Retired … carried by FR-0xx-AC-y". The matrix carries 18 "⛔ Retired; carried by …" rows, and TC-005, TC-014, TC-023 and index:75 repeat the map. Remedy: delete the three FRs and their ledger rows, then retag the code and tests to the carrying criteria (next row). | spec/decisions/ADR-001-overlapping-generators-and-input-models.md:71-72; spec/decisions/ADR-001-overlapping-generators-and-input-models.md:89-90; spec/functional/FR-001-deterministic-oracles.md:19-50; spec/functional/FR-003-kani-lowering.md:17-50; spec/functional/FR-007-bounded-kani-profile-corpus.md:17-59; spec/test-matrix.md:13-32; spec/index.md:75 |
| FND-015 | high | Code that is still live has no owning requirement. `src/lib.rs:6-39`, `bound.rs:102`, `oracle.rs:310` and `kani.rs:314` implement FR-001, FR-003 and FR-007. These were ✅ Covered on main and are now retired. Every carrying criterion (FR-014-AC-35..38, FR-015-AC-19..25) is 🚧 Planned and reads only `CheckedPackageV2`. Active requirements still name the retired FRs upstream: FR-002:6,70, FR-004:8,108, FR-005:8,54, FR-008:10,37,134, StR-001:6,10,16, NFR-001:7,11,53, NFR-002:7,9,54, TC-001, TC-002, TC-003 and interface-001:64-96. FR-001-AC-6 says "the V1 bound-package consumer has no successor", which these consumers contradict. ADR-001 Consequences records the contradiction, but the spec states no target for code that runs today. | src/lib.rs:6-39; spec/functional/FR-001-deterministic-oracles.md; spec/functional/strategies/FR-008-bound-domain-strategy-admission.md:37 |
| FND-016 | high | The symbol-naming text does not match `oracle::unique_names`. FR-018:174-175 and AC-11 (and TC-029:88-89) say every symbol is "the operator's readable name followed by its ordinal". In the code a lone stem is used bare, and only items that share a stem get `_{n}`, from 1 in key order. Equal and NotEqual have different stems, so AC-11's own scenario gets no ordinal. The AC-11 mutation row (FR-018:222) no longer fails anything, because colliding stems are now suffixed. TC-022:24-26 says every bound oracle symbol gets an ordinal, and TC-022:24,35 and FR-013:52 name `BoundGenerationError::NameCollision`, which does not exist (src/bound.rs has no such variant). | spec/functional/complete-v1/FR-018-composite-equality-oracles.md:174-175; spec/functional/complete-v1/FR-018-composite-equality-oracles.md:193; spec/functional/complete-v1/FR-018-composite-equality-oracles.md:222; spec/test/complete-v1/TC-029-composite-equality-oracles.md:88-89; spec/test/strategies/TC-022-it010-consumable-output.md:24-35; spec/functional/strategies/FR-013-it010-consumable-output.md:52 |
| FND-017 | medium | FR-017-AC-10 was deleted as a source-scan criterion, but it also carried the parser behaviour: real Kani captures parse to a typed transcript and classify, and the playback passes through verbatim. Ten tests in `src/kani_transcript.rs` (lines 264-552) now trace a criterion that no longer exists. The parse-level tests at :264, :309 and :328 back no criterion at all. Restore the non-scan half as an AC, or retag the tests to FR-017-AC-4/AC-5 and add an AC for transcript parsing. | src/kani_transcript.rs:264; spec/functional/complete-v1/FR-017-kani-execution-evidence.md:94-100 |
| FND-018 | medium | FR-014-AC-38 ("names built from two packages whose nodes share ids do not alias") cannot pass. Merged code puts no package identity in any name, so this criterion re-imports FR-001-AC-7's package-in-name rule, which the naming ruling removed. Delete it. | spec/functional/complete-v1/FR-014-exact-scalar-oracles.md:343; spec/test/complete-v1/TC-024-exact-scalar-oracles.md:159-160 |
| FND-019 | medium | Accepted ADRs keep history, "not" statements and open questions. ADR-001:27-30 is history; :66-69 and :83-85 say what is not carried; :92-93 is a migration procedure; and :94-98 leaves a contradiction open "until a separate spec change". ADR-001, ADR-002 and ADR-003 name rejected alternatives. ADR-002:33-36 has a "Where it lived" history table. ADR-003:32-58 narrates measured facts and "if those reports hold". ADR-004:17-18 and :55-56 are open questions under Accepted status ("QSpec has not answered it"). AD-001:71-72 and :161-163 leave contradictions pending. | spec/decisions/ADR-001-overlapping-generators-and-input-models.md:27-30; spec/decisions/ADR-004-generated-subject-abi-open-decisions.md:17-18; spec/assurance/AD-001-codegen-architecture.md:71-72 |
| FND-020 | medium | FR-017 Inputs says the identity carries "memory and wall-clock ceilings (FR-028) the run is held to". At head the run takes the caller's `KaniExecutionRequest::timeout` and has no memory ceiling, and FR-017's own open items admit this. The run's `VacuousProof` classification (zero SUCCESS checks gives inconclusive) is in no FR-017 criterion, yet FR-029-AC-2 relies on it. | spec/functional/complete-v1/FR-017-kani-execution-evidence.md:36-38; src/kani_execution.rs:216-220; src/kani_execution.rs:612-626 |
| FND-021 | low | Assurance-evidence and review-history prose remains. test-matrix:95-102, :111 and :326-333 carry SR-016/SR-017 review history and "local pre-review evidence". spec/evidence/suites.md is framed as an evidence registry (:27, :35, :38, :43-50). FR-017:26 says "turns a generated harness into an assurance claim". GitHub issue ids are cited in FR-017:134, ADR-003:49, the matrix and suites.md:46; the project rule is Linear ids only. | spec/test-matrix.md:95-111; spec/evidence/suites.md:22-50; spec/functional/complete-v1/FR-017-kani-execution-evidence.md:26 |
| FND-022 | low | Text that references the retired FR-003 and the removed naming is stale. test-matrix:331 says "the FR-003 portion of TC-007", but FR-003 was removed from TC-007. suites.md:22,46 call SUITE-008 "the FR-003 lane". interface-001:192,198,207,222 (outside the diff) describe a batch `NameCollision` and a "positional counter". | spec/test-matrix.md:331; spec/evidence/suites.md:22; spec/interface/interface-001-codegen-api.md:207 |

### Round 2 verdict

- **Clean:** check (1) no digest, pin, SHA, attestation, assurance-chain, publication-guard or vendoring text left in spec/ outside reviews/; check (4) OperationProvenance kept correctly (src/exact_scalar.rs:372-408 matches FR-014, FR-015, FR-018, FR-022, TC-024, TC-025, TC-029, TC-033 and the matrix); check (5) FR-004-AC-4 and AC-9 identical to main (FR-004:85,90 against main :90,95); check (6) `make spec` exit 0 on both.
- **Dangling references:** FR-006, TC-032, TC-028, SUITE-001, FR-017-AC-8/9, FR-019-AC-5, FR-026-AC-2/3, FR-029-AC-7, FR-027 and TC-038 have none. FR-017-AC-10 does (FND-017).

Not mergeable. FND-014, FND-015 and FND-016 are high.

### Round 3

- `make spec` exits 0 on both. The only warning is the existing FR-014 EARS warning, now at line 278.
- The FR-018 rewrite raises no EARS warning.
- `quire coverage --strict` exits 1 on both: main backs 198 of 265 rows, head 176 of 269, and `status_lies` is 0.
- 23 backed rows disappeared with their deleted criteria: FR-001-AC-1..8, FR-003-AC-1..8, FR-007-AC-1..3 and AC-5..7, and FR-017-AC-10. FR-017-AC-12 is newly backed. No defined row lost its backing.
- The branch is behind main, and GitHub reports it MERGEABLE.

| FND | outcome | reason |
| --- | --- | --- |
| FND-014 | fixed | FR-001, FR-003 and FR-007 are deleted. So are the retired matrix rows, the index.md retired row, and the "retired, carried by" text in TC-005, TC-014 and TC-023, which now verify FR-015. ADR-001 states Q1-Q3 as decisions only. No spec file outside reviews/ references FR-001, FR-003 or FR-007; the remaining hits are Contract Runtime and quire-driver ids. |
| FND-015 | fixed | spec/ no longer points active requirements at the deleted FRs: FR-002, FR-004, FR-005, FR-008, StR-001, NFR-001, NFR-002, TC-001..003 and interface-001 depends_on are all updated. AD-001 "Current state" (:188-206) names the V1 paths that code still carries. The code's trace tags are FND-023. |
| FND-016 | fixed | FR-018 now says "build each symbol from its operator's readable stem", "use a stem that one item holds bare", and "suffix items that share a stem with `_{n}`, numbered from 1 in ascending descriptor-key order", which matches `oracle::unique_names`. AC-11 and its mutation row are restated, and interface-001:207 matches. No `NameCollision` remains in spec/. |
| FND-017 | fixed | FR-017-AC-12 (capture parse and classify, playback verbatim) and FR-017-AC-13 (zero successful checks gives vacuous-proof inconclusive) exist, with a matrix row (:56), TC-027 steps (:29, :66, :70) and the TC-027 list (:162). The ten `src/kani_transcript.rs` tests trace FR-017-AC-12. See FND-025. |
| FND-018 | fixed | FR-014-AC-38 and TC-024's two-package aliasing step are gone. |
| FND-019 | fixed | ADR-001..004 now carry Status, Context, Decision (Q-rulings) and Consequences only, with no history tables, rejected alternatives, "not carried" text or open questions. AD-001 keeps a labelled Current-state section. |
| FND-020 | fixed | FR-017 Inputs names the caller's `KaniExecutionRequest::timeout` and "no memory ceiling"; the vacuous-proof classification is FR-017-AC-13. |
| FND-021 | still-open | The evidence-registry prose and SR-016/SR-017 history are gone from the matrix and suites.md, and the only issue ids left in the requirement texts are the three `UpstreamBlocker` wire values (src/generation.rs:18-25). But spec/index.md keeps GitHub issue links: frontmatter :10-11 and :17-20 (`ix://…/issues/10`, `…/issues/3`) and the list at :100-105 (github.com/agent-ix/…/issues/1, 3, 10, 3, 7). |
| FND-022 | fixed | the FR-003 lane and TC-007 text are gone. The interface-001 positional-counter and batch `NameCollision` text is restated. |

## New findings (disposition pass 3)

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-023 | high | Deleting FR-001, FR-003 and FR-007 left 99 code and test trace tags pointing at ids that no longer exist (`// Implements: FR-001/003/007`, `Trace: FR-00x-AC-n`), in 16 files. The counts are: src/bounded_kani_corpus.rs 19, tests/it/kani_generation.rs 19, tests/it/oracle_generation.rs 11, tests/it/bound_generation.rs 11, src/lib.rs 9, tests/it/bounded_kani_corpus.rs 7, src/bounded_kani_profile.rs 5, and 1-3 each in bound.rs, bounded_collections.rs, definedness_arithmetic.rs, kani.rs, kani_execution.rs, kani_obligations.rs, oracle.rs, tests/it/bound_coverage.rs and tests/it/kani_obligations.rs. This PR already retags src/kani_transcript.rs, so retagging these (or dropping tags whose criterion was not carried) is a comment-only change that fits here. The code these tags mark is still public, and interface-001 declares it with no owning FR. | src/lib.rs:6; src/bounded_kani_corpus.rs; tests/it/kani_generation.rs; tests/it/oracle_generation.rs |
| FND-024 | low | FR-018 Out of Scope says "Temporal and protocol oracles, which FR-020 will own". FR-020 does not exist, so this is a dangling id and a roadmap statement. FR-025:71 says QSpec "has not yet decided" frame lowering, which states what isn't. | spec/functional/complete-v1/FR-018-composite-equality-oracles.md:240; spec/functional/complete-v1/FR-025-generated-subject-abi.md:71 |
| FND-025 | low | The matrix marks FR-017-AC-12 and AC-13 `🚧 Planned`, but AC-12 is backed by ten passing unit tests. The vacuous-proof test `a_zero_total_checks_summary_is_inconclusive_not_verified_even_with_every_cover_satisfied` still says it "binds itself to no criterion" because FR-017-AC-4 does not cover the case; it should now trace FR-017-AC-13. | spec/test-matrix.md:56; src/kani_execution.rs:798-811 |

### Round 3 verdict

These checks are clean:
- Naming text matches `oracle::unique_names`.
- `NameCollision` is gone from spec/.
- FR-017-AC-12/13 are in place with the tests retagged.
- FR-014-AC-38 is gone.
- The ADRs state decisions only.
- interface-001 is consistent.
- The FR-018 rewrite passes EARS.
- There are no dangling spec references to deleted ids, except FR-020 (FND-024).

Not mergeable. FND-023 is high: 99 code and test tags point at the deleted FR-001, FR-003 and FR-007. FND-021 is still open for the index.md GitHub links. FND-024 and FND-025 are low.

### Round 4

Gates:

| Gate | Head | Main |
| --- | --- | --- |
| `make spec` | EXIT 0 | EXIT 0 |
| `make test` | EXIT 0 (80 / 213 passed, 5 ignored / 0) | EXIT 0 (same counts) |
| `quire coverage --strict` | EXIT 1, 186/269 rows backed | EXIT 1, 198/265 rows backed |

- `make spec` reports the same two FR-014 EARS warnings on both sides.
- `status_lies` is 0.
- The src/ and tests/ diff against main touches only comment lines; no code line changed.

| FND | outcome | reason |
| --- | --- | --- |
| FND-021 | fixed | spec/ outside reviews/ has no github.com or `/issues/` link |
| FND-023 | fixed | no `Implements:`/`Trace:` tag in src/ or tests/ names an id missing from this repo's spec. The only FR-001/FR-003 hits are fixture strings (`RequirementId::new("FR-001")`, `"FR-003"`) and the `oracle.rs` unit-test tuples. See FND-026 for how the retagged ids map. |
| FND-024 | fixed | no FR-020 mention is left; FR-025's "not yet decided" line is restated |
| FND-025 | fixed | the matrix row at spec/test-matrix.md:54 marks FR-017-AC-12/13 `✅ Covered`, and the vacuous-proof test traces FR-017-AC-13 (src/kani_execution.rs:802) |

## New findings (disposition pass 4)

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-026 | high | The retag binds V1-path tests to V2 criteria they do not verify, so quire now reports planned, unbuilt criteria as backed. Newly backed at head: FR-014-AC-35, AC-36, AC-37 and FR-015-AC-19, AC-20, AC-22, AC-23, AC-24, AC-25. The matrix itself says these are unbuilt. `test-matrix.md:42` records FR-014-AC-35..37 as "Planned; Boolean and integer eq/ne nodes are refused as OperationNotDerivable". Example: `tc_002_integer_and_state_comparisons_are_deterministic_compile_and_match_the_model` (tests/it/oracle_generation.rs:870-872) runs the V1 `BoundPackage` comparison oracle against a plain model. Yet it traces FR-014-AC-35 (`ir_confirmed` descriptors from CheckedPackageV2 nodes calling `evaluate_boolean`) and FR-014-AC-37 (charges, counters and QSL value authority). Likewise, the bounded Kani corpus and profile tests (`CapabilityEntry` Supported/Refused/Inconclusive, timed-out/exhausted) now trace FR-015-AC-23, whose vocabulary is `ObligationDisposition` (supported/requires-bound/unsupported/invalid-request) and has no inconclusive state. The kani_generation tests for `generate_kani_bundle` now trace FR-015-AC-19/20/24/25, which are about `negotiate_kani_obligations` harnesses. The retags that match the criterion text are `FR-007-AC-7`→`FR-015-AC-22` (census validation) and the transcript retags. The V1 tests elsewhere name behaviour no surviving criterion states, so drop their tags the way the FR-001-AC-6/7 and FR-007-AC-5/6 tags were dropped. The same applies to the module-level `// Implements: FR-014` / `FR-015` on src/oracle.rs, src/bound.rs, src/kani.rs and the corpus modules. | tests/it/oracle_generation.rs:870; tests/it/kani_generation.rs; tests/it/bounded_kani_corpus.rs; src/bounded_kani_profile.rs; src/bounded_kani_corpus.rs; src/lib.rs; spec/test-matrix.md:42 |
| FND-027 | low | Two retags are loose even allowing for the V1/V2 split. `FR-003-AC-3` (definedness, arithmetic and indirect-read refusals) became `FR-015-AC-3` (unbounded or non-finite refusal). `FR-001-AC-3` (requirement ids in symbols, failures and source maps) became `FR-014-AC-5` (claim-map entry fields). | tests/it/kani_generation.rs; tests/it/oracle_generation.rs |

### Round 4 verdict

FND-021 and FND-023 to FND-025 are fixed, and the code diff is comment-only. Not mergeable: FND-026 makes the coverage export claim nine planned V2 criteria are backed by V1-path tests. Fix it by dropping those tags rather than retagging them.

### Round 5


| Gate | Head | Main |
| --- | --- | --- |
| `make spec` | EXIT 0 | EXIT 0 |
| `make test` | EXIT 0 (80 / 213 passed, 5 ignored / 0) | EXIT 0 (same counts) |
| `quire coverage --strict` | EXIT 1, 177/269 backed, 56 unbacked rows | EXIT 1, 198/265 backed, 31 unbacked rows |

`status_lies` is 0 on both sides.

No criterion that was backed on main is unbacked at head. Every id that is newly unbacked was added by this PR, and the matrix marks each one 🚧 Planned. They are:
- FR-014-AC-35..37, at test-matrix.md:42
- FR-015-AC-19..25, at :50
- FR-025-AC-7/8, at :71
- FR-026-AC-1/4, FR-028-AC-1..9 and FR-029-AC-1..6, at :72-74
- TC-037, TC-039 and TC-040

The newly backed criteria are FR-017-AC-12 and AC-13.

FR-014-AC-1/4/5 are backed by the V2 tests in tests/it/exact_scalar_generation.rs and tests/exact_scalar_support/package.rs. FR-015-AC-3/9/10/11 are backed by tests/it/kani_obligations.rs, and AC-3 also by routed_generation.rs.

| FND | outcome | reason |
| --- | --- | --- |
| FND-026 | fixed | every V1-path tag is dropped, and the nine planned criteria (FR-014-AC-35/36/37, FR-015-AC-19/20/22/23/24/25) are unbacked, which matches their 🚧 Planned rows |
| FND-027 | fixed | the two loose retags went with the dropped V1 tags |

## New findings (disposition pass 5)

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-028 | medium | The src/lib.rs change is not comment-only. Dropping the `// Implements:` comments let rustfmt reorder the `mod` declarations. That leaves `// Implements: FR-016`, which main puts above `mod spine_replay`, above `mod bounded_collections`, so the FR-016 native-replay adapter module is now untagged and a V1 corpus module claims FR-016. The reorder changes no behaviour, since `mod` order has no semantics. The misplaced tag is still a wrong trace. Fix: move `// Implements: FR-016` back above `mod spine_replay`. | src/lib.rs:24-31 |

### Round 5 verdict

FND-026 and FND-027 are fixed. The coverage change is honest: no built or ✅ Covered criterion lost its backing. Not mergeable until FND-028, a one-line comment move, is fixed.
