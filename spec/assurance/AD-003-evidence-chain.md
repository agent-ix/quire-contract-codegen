---
id: AD-003
title: "Evidence chain across IR, CG and QSL: obligation identity, corpus identity, launch evidence and the terminal value"
type: ArchitectureDescription
status: proposed
owner: codegen-maintainers
system: quire-contract-codegen Kani and Evidence subsystems, and the identities and outcomes they join from Contract IR and QSL
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-001
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AD-002
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-030
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-030
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-013
    type: references
---
# Evidence chain across IR, CG and QSL

This AD governs the Kani and Evidence subsystems and the Replay subsystem's identity members. CG
owns it because CG is the one repository that depends on both IR and QSL; IR must not depend on
QSL, so no other repository can join the chain. IR and QSL each hold their own link and are cited,
not copied. It is one of the seam descriptions of IR-324.

## System Boundary

A proof claim reaches a user as a terminal record whose value says what a Kani run established.
That claim is only as good as the chain from the checked package to the run to the record. This AD
names every link, who owns it, how each link is bound to the one before it, and which links are
missing.

Scope: from an admitted `CheckedPackageV2` to one QSL terminal value per run item. Out of scope:
oracle generation, strategy generation, Kani's internals, QSL's evaluation and the replay
request itself (AD-002).

## Views

The chain is described link by link, then by what crosses each join and who reports a failure,
then by how identity is asserted.

### The chain

| # | Link | Owner | Identity or content binding |
| --- | --- | --- | --- |
| 1 | Checked package, `package_id` (`quire.package.semantic/v2`) | QSpec wire, QSL emits, IR re-derives on read | digest over the package's identity preimage |
| 2 | Finite profile and input (`ProfileSelection`, `ValidatedFiniteInput`, `KaniOutcome` for refusals) | IR | profile revision selected with the ABI and module revisions |
| 3 | Obligation: harness, bound domains, ceilings, oracle symbols (`KaniObligationIdentity`, `ScalarObligationIdentity`, and for frame harnesses `StateFrameIdentity`, `state_frame.rs`) | CG | JSON record persisted beside the harness; no digest is computed from it (gap E-1) |
| 4 | Corpus case: `CaseIdentity` over construct, profile, input, request, dependencies | CG | lowercase SHA-256 over its deterministic JSON, which also names the artifact paths; collisions refused by `EmittedCorpusIdentities` (`bounded_kani_corpus.rs`, `kani_corpus_identity_collision`) |
| 5 | Launch: argument vector, ceilings, launcher, harness-in-crate check (`execute_kani_obligation`, `kani_launch_command`, `launch_evidence`) | CG | harness source must appear byte for byte in the crate before launch (`kani_execution.rs`, FR-017 step 1) |
| 6 | Run classification: `classify_kani_run` over the typed transcript to `KaniRunOutcome` | CG | one parser module, `kani_transcript.rs`; Kani's wording read nowhere else |
| 7 | Terminal value: (`KaniRunOutcome` or `KaniOutcome`, replay settlement) to `qsl_replay::TerminalValue`. The input is the pair, not the Kani outcome alone. For a counterexample the second member is the E9 replay result, as ADR-013 C-09 states it (QSL-354, merged): (1) settled, with parity settled inside `replay`: a reproduced replay gives `Refuted`, and a disagreement (or a replay that completes no value) gives `Inconclusive(InconclusiveCause::ReplayParity)` (code `replay_parity`), carrying the replay's `DisagreementCause`. (2) A non-fault `ReplayRefusal` (identity mismatch, decode refusal, stale dependency, limit reached) gives `Inconclusive(InconclusiveCause::ReplayRefused)` (code `replay_refused`) carrying the refusal's catalog code: only QSL's closed set of `ReplayRefusal` codes, never a CG code. (3) A fault gives `TerminalValue::Failed`, so a defect stays loud. A refuted Kani outcome never becomes `Refuted` without a reproduced replay. A fault is a fault wherever it sits in the chain, not only as the variant `ReplayRefusal::Fault`: QSL also reports `ReplayRefusal::Admission(AdmissionFailure::Fault(_))` (catalog code `runtime_invariant`, `execute.rs`) and `CallSiteRefusal::Fault` (`call_site.rs`), which reaches CG directly and wrapped in `ReplayPackageError::CallSite` and `FrameReplayError::CallSite`; the map classifies by walking the whole error, not by its top variant. CG's own failures before any replay runs also need a value (below). Merged QSL text agrees: ADR-011 T-13 and ADR-013 O-16, O-24 and C-09 say the driver runs the Kani obligation and the replay, passes both to CG's map, and writes each item's FR-331 terminal record. The two inconclusive causes `replay_parity` and `replay_refused` are named in the merged C-09 text; the `TerminalValue` variants that carry them land with QSL-351 (not yet in QSL's `main`). CG provides the map and its typed inputs; the driver builds the terminal record (ADR-011 T-13; quire-driver PR 11) | CG (map), QSL (type) | one total `match` each over the outcome, no wildcard arm (FR-030-AC-7; FR-029 and FR-030 are `Planned`, written over the outcome alone today, and FR-029's unconditional `falsified` to `Refuted` row is to be amended) |
| 8 | Counterexample join: decoded playback to QSL replay, with `ObligationIdentity` | CG builds, QSL consumes | see AD-002; `package_id` recomputed by QSL |
| 9 | FR-331 `results` record | QSL type (`TerminalRecord`, `ProofResultEnvelope`), QSpec wire; the driver builds the record from CG's map and typed inputs (quire-driver PR 11) | none in CG: CG builds no record and is not required to (gap E-3) |

Dependency direction: IR to nothing of QSL; CG to IR and `qsl-replay`; QSL's replay reads what CG
builds and never calls CG. QSL depends on no CG, normal or test-time (QSL ADR-011 FB-11), so a
test that needs both lives in a repository above both, not in QSL; QSL's review names
quire-integration (QSL-342).

### What crosses each join, and who reports a failure

| Join | Crosses | Failure outcome | Reporter |
| --- | --- | --- | --- |
| 1 to 2 | admitted package, finite input | IR typed `KaniOutcome` `Refused`, `InvalidInput`, `IncompleteInput`, or `unsupported` at negotiation (no outcome) | IR |
| 2 to 3 | one lowered claim per routed item | `GenerationErrorCode::UnsupportedObligations`, a `NoDerivableClaim` claim; no harness | CG |
| 3 to 5 | harness source and options | `HarnessNotInCrate`; any argument vector is the identity's `options` verbatim | CG |
| 5 to 6 | process output | `KaniInconclusiveReason::{NoVerdict, TimedOut, ...}`; a run with no verdict is not a proof | CG |
| 6 to 7 | outcome, SUCCESS-check count and, for a falsified run, the replay result | none: the map is total over (outcome, replay result) once every pre-replay failure below is classified; a replay disagreement is `Inconclusive(ReplayParity)`, a replay refusal `Inconclusive(ReplayRefused)`, a fault anywhere `Failed` | CG |
| 6 to 7, falsified run stops in CG before any replay | `EvidenceFailureCause::{Decode, Domain}`, `SpineReplayError::{UnboundArgument, FieldDelimiter, Transcript, WrongArm}`, `ReplayPackageError::{InvalidFunction, Dependencies, CallSite}`, `FrameReplayError::{Dependencies, CallSite, Name, Transcript, Envelope}` | Decided here, with QSL's closed-set rule (as relayed): the replay did not run, so `Refuted` is withheld, and `ReplayRefused` carries only QSL's `ReplayRefusal` codes, a closed set QSL owns. The first group maps to `Failed` so a CG defect stays loud, because each is CG's own defect: `Domain` (a playback outside the harness proof bound, which CG's pre-check exists to keep visible), `UnboundArgument` (CG's harness bindings and QSL's `call_site` disagree), `FieldDelimiter` (harness text CG generated), `Decode` (a playback that does not type against the bindings CG persisted), `Transcript`, `Envelope`, `WrongArm`, and any fault, a `CallSite` fault included. The second group is replay setup refused on data after Kani refuted: `Dependencies` (a caller lock refused at replay setup, `DependencyLockError`, is this case), a non-fault `CallSite` refusal (for example `Compile`, `ModelIntake`, `DependencyInput`, `Import`, `Dependency`, `UnknownFunction`, `UnknownOperation`, `UnknownClause`), `InvalidFunction`, `Name`. Decided by QSL (relayed; the when-rule, R-Q1): it maps to `Inconclusive(ReplayRefused)` with a QSL catalog code, the same as an E9 refusal. QSL adds those codes in QSL-352 (relayed), landing with CG layout AD-004 step 5. Interim, until QSL-352 assigns them: these are `Failed`, a safe default, because `CallSiteRefusal` exposes no code at this base and `Dependencies`, `InvalidFunction` and `Name` are CG's own errors. Faults stay `Failed` throughout. | CG |
| 7 to 9 | terminal value and item identity | the driver's step; CG builds no record | driver |

Each failure state stays a distinct typed state (AD-001 Failure view). No state is converted to
success.

### Versioned contracts and how identity is asserted

- The identity that binds a proof to its content is the obligation identity: one canonical
  content-identity digest, minted by CG over the obligation's identity members, carried in the
  envelope as QSL's `ObligationIdentity` (E-1). It is the only digest CG mints for a proof, and no
  other digest, pin, SHA or version record is added to this chain.
- The other digests on the chain already exist and bind content, not tools or versions:
  `package_id` (QSL recomputes it on replay), the byte digests QSL checks on provided source and
  the `CaseIdentity` name of a corpus case. The `ByteDigest` of the transcript that the function path
  once put in the request's identity slot was not a content identity of the proof; it is replaced
  by the O-09 function-contract obligation identity (R-Q7, IR-553). QSL does not check the slot
  against anything (`execute.rs`, it is passed to witness decoding as a label).
- There is no pin, SHA or digest over a file, version or tool, and CG proposes none. In
  particular the Kani version is not pinned; classification reads Kani's output through the one
  transcript parser. QSL's `BackendProviderSource` has a public `tool_pin` string and ADR-013
  O-24 says the envelope carries a tool pin; whether CG must supply one or QSL derives it is
  routed to QSL (R-Q9), not decided here.
- The seam to QSL is the `qsl-replay` Rust API, asserted by compilation. The seam to IR is IR's
  crate API, asserted by compilation. CG reads model items through IR's root-crate glob today;
  IR-347 (as relayed by the IR planner) moves the Kani family lowerings (`lower_checked_arithmetic`,
  `lower_query`, `lower_reaches`) out of IR into CG and has IR delete them after, and CG then
  reads the model crate for model items, so the glob goes away.
- Cause codes cross IR to CG as strings (`KaniOutcome.code`). IR spells `kani_vacuous_proof` only
  inside `KaniOutcome::proved_from_checks`; `kani_solver_absent` and `kani_backend_absent` are
  named in IR's and CG's specs (FR-030) and defined by no IR constant or enum. CG code spells `kani_vacuous_proof` as a literal
  (`kani_execution.rs:690`) and the other two nowhere yet, so CG's map will spell them itself. A
  typo is not a compile error.
  IR-347 (reopened) already covers the free-string cause codes; no new ticket is filed.

## Decisions

CG owns the map into QSL's terminal value and the preimage of the obligation identity; QSL owns
both types. The statements below are ones a test can check, followed by the three questions
raised in IR-324 from parked CG PR 210 and their recommendations. PR 210 (draft, open, laid out
before the subsystem restructure) is input here and is not at this base.

### Invariants a test can check

Candidate statements (local labels; the repository assigns requirement ids when one is
authored).

- E-1. CG computes QSL's `ObligationIdentity` by one function over the ADR-013 O-09 members
  (the clause or application node id, its occurrence key, the obligation kind, and the
  arguments each as parameter node id and declared domain), excluding the source span, encoded
  as RFC 8785 JSON in CG's one canonical-encoding place by `quire_canonical`, never by
  `serde_json`;
  the value changes when any included member changes and does not when the span changes. The
  V1 contract path, the scalar path and the frame path (`StateFrameIdentity`) all use it. The
  function path (`call_site` over a `QualifiedName`, ADR-013 O-09 as amended by QSL-352) uses it
  too, with the checked function node id and its `declaration` occurrence key in place of the
  clause's, both read from `FunctionSite` and never derived by CG, and the existing `ObligationKind` of the
  harness replayed (O-09 adds no subject tag and no new kind: one identity per kind the function
  requests; FR-016-AC-21 to AC-23, implemented for the function path only: the V1 contract, scalar
  and frame paths do not compute it yet). The function's `arguments` are all of its parameters
  (O-09): a harness that leaves a parameter without an argument, or declares an integer with no
  bound (AD-016 arrow 5: `requires-bound`, never narrowed implicitly) or a Boolean with bounds,
  has no identity and is refused with a typed error. The retained per-argument bound is the
  per-argument domain AD-016 puts in the identity; it is not a subset of the parameters.
  The closed preimage, CG's own spelling (O-09 fixes the members, and no QSL or QSpec text pins
  the member names, the domain encoding, the node-id text form or a digest label as of this
  revision; AD-016 TK-05, the seed vector `obligationIdentitySha256`, is open): one RFC 8785
  object with members `function` (the function node id as 64 lowercase hex digits),
  `declaration` (`node` as 64 lowercase hex digits, `role` as the role string, `ordinal` as a
  number), `kind` (the `ObligationKind` in snake case), and `arguments`, an array ascending by
  the parameter's declared identifier, each `{domain, parameter}` with `parameter` the parameter
  node id in lowercase hex and `domain` either `{"type":"boolean"}` or
  `{"type":"integerRange","minimum":"<decimal>","maximum":"<decimal>"}` (bounds are decimal
  strings because RFC 8785 integers above 2^53 are not exact). The digest is the plain SHA-256 of
  that text, with no domain label: interim, until QC-4 / TK-07 may add an FR-201 domain, which
  would change every identity. The golden text of the FR-016-AC-21 tests
  (`tests/it/skeleton_spine.rs`, `recomputed`) is the vector; it is written by hand, not by CG's
  encoder. If QSL later recomputes or compares the identity, QSL pins the spelling and CG
  follows in a follow-up; until then the spelling is CG's own and interim.
- E-2. Two obligations with identical identity members have the same `ObligationIdentity`;
  regeneration is byte-identical (NFR-001).
- E-3. Every run item that reaches the map has exactly one terminal value, and the map from
  `KaniRunOutcome` and from `KaniOutcomeKind` is one `match` with no wildcard arm (FR-030-AC-7 is
  the existing form; FR-029-AC-1's unconditional `falsified` to `Refuted` is not, and is to be
  amended). A falsified run's value is a function of the outcome and the replay result together
  (link 7): a replay disagreement is `Inconclusive(ReplayParity)`, a replay refusal is
  `Inconclusive(ReplayRefused)`, a fault anywhere in the chain is `Failed`, and a refuted
  outcome without a reproduced replay is never `Refuted`. Every CG failure before replay is
  classified by the link 7 rule (when replay setup is refused on data: `Inconclusive(ReplayRefused)`
  with a QSL code once QSL-352 assigns it, `Failed` until then; a CG defect or a fault:
  `Failed`), so no falsified run is left without a value. A refusal of the obligation's own
  input before Kani runs, as IR reports it, is `Declined(ProofRefusalCause)` through the
  IR-outcome rows of C-09 (FR-030; R-Q1) and is outside the replay (counterexample) rows of C-09.
- E-4. No outcome maps to `Tested`.
- E-5. A run whose SUCCESS-check count is zero maps to a value QSL reads as non-success. A
  precondition harness counts its satisfied cover as its one SUCCESS check (question b).
- E-6. A run is classified only from the output of the launch it made: a transcript or report
  left from an earlier run is never read. Test: with a stale artifact in place from a run of
  another verdict, a launcher that prints nothing classifies `NoVerdict`, not the stale verdict.
  Today the reader takes captured process output; PR 210's JSON report must be read from a path
  fresh to this launch.
- E-7. The terminal value of a run is derived from the run's own transcript or report, never
  from generation-time classification (FR-017-CON-2).
- E-8. No source in CG or IR defines `TerminalValue`, `TerminalRecord` or `ObligationIdentity`.
- E-9. The `proof_category` of a run is `TerminalValue::category()` of its terminal value and
  nothing else (so there is no second derivation).

### (a) `TerminalValue` has no non-vacuous inconclusive value

Measured. QSL `TerminalValue` has seven variants (`Proved{success_checks}`, `Tested`, `Refuted`,
`Declined`, `Unsupported`, `Incomplete`, `Failed`; `proof_result.rs`). QSpec FR-331 has eight
result values including `inconclusive`, and FR-331-AC-8 says a zero-check proof records
`inconclusive` with a typed vacuity cause, not a new result value. QSL encodes that as
`Proved{0}` with `vacuous_proof_cause()` returning `KaniVacuousProof`. ADR-013 O-16's proof column
maps IR `Inconclusive` to the result `inconclusive`, and bound exhaustion to `incomplete`.

- Unwind-exhausted to `Incomplete(ResourceExhausted)` (FR-029): intended by O-16 ("bound
  exhaustion"). Keep, and cite the O-16 row in the map table.
- Cover-unsatisfied to `Proved{0}` (FR-029): not an O-16 row; a CG approximation. It is
  non-success and reads as `inconclusive`, which is the right category, but with cause
  `KaniVacuousProof`, which is the wrong cause (the checks were real; the assumptions were
  unsatisfiable).
- IR `Inconclusive` with a cause other than the vacuous one (FR-030): maps to `Failed` (a tool
  failure), because no other value fits. O-16 and IR's own `provider_result` say `inconclusive`.

Options and costs:

| Option | What changes | Cost |
| --- | --- | --- |
| A. Keep the seven variants; document CG's rows as an approximation | CG spec only | cover-unsatisfied misnames its cause; IR `Inconclusive` is reported as a tool failure; two encodings of the inconclusive category stay |
| B (recommended). QSL adds `TerminalValue::Inconclusive(InconclusiveCause)` with causes for the vacuous proof, the unsatisfied cover and no-qualified-interpretation, and `Proved` carries a non-zero count | QSL type, reader and FR-069; QSpec FR-331-AC-8 wording; CG's three rows (vacuous, cover-unsatisfied, `Inconclusive`-other) and TC-040 and TC-041 | one QSL change plus one CG follow-up; removes `Proved{0}` as a magic value |
| C. CG reports unsatisfied cover as `Failed` | CG only | blames the tool for a property of the model |

Option B also gives a home to the replay results of link 7: the inconclusive value
carries `InconclusiveCause::ReplayParity` (code `replay_parity`, carrying `DisagreementCause`) or
`InconclusiveCause::ReplayRefused` (code `replay_refused`, carrying the `ReplayRefusal` code), which no existing `TerminalValue` variant can. This is QSL's and
QSpec's decision (routed R-Q1, R-S2); QSL's review says option B arrives as QSL-351. Until QSL decides, CG keeps option A's
rows and states them as interim in FR-029 and FR-030; it adds nothing that would need to be
removed (no shim).

Who owns the map (C-09): CG. QSL's ADR-013 as merged (QSL-354) says so throughout: O-16's table
calls the proof column "mapped by one exhaustive CG-owned function (C-09)", the invariants
paragraph says "CG implements the map over the Kani outcome and the E9 replay result", and C-09
takes the Kani outcome and the replay result. IR's `KaniOutcomeKind::provider_result` doc still
says the map is "implemented here". IR cannot own it (IR must not depend on QSL) and retired its
map (IR FR-031-AC-5, IR-358); CG depends on both and owns it. Once option B lands, collapse FR-029 and FR-030 into one table over IR's `KaniOutcome` plus
one total conversion from `KaniRunOutcome` to `(kind, code)`, so there is one C-09 table and one
test. Cost of keeping two: they already disagree with IR's own map on the `Inconclusive` row.

### (b) Does a satisfied cover of a precondition harness count as its one SUCCESS check?

Measured. A precondition harness "asserts nothing: its one property is its non-vacuity cover"
(doc on `classify_kani_run`, `kani_execution.rs`). At this base the zero-check rule is skipped for
that kind (`classify_transcript`, `kani_execution.rs:676`) and the cover decides `Verified`; no
count is carried on `KaniExecutionEvidence`, and the count that is read (Kani's printed
`** failed of total` line) is used only as a vacuity gate. PR 210 replaces the printed line with
Kani's `--export-json` report, carries the count on the evidence and, for the precondition kind
only, adds satisfied covers to it. FR-029's input is "the SUCCESS check count its transcript
reports", so the counting rule is open in the spec.

Recommendation: yes. The harness proves satisfiability of the precondition over the IR bounds; the
cover is that proof. Without it every verified precondition maps to `Proved{0}` and QSL reports it
vacuous, contradicting CG's own `Verified`. State the counting rule once, in CG (FR-017 and
FR-029): "SUCCESS checks are the harness's non-cover checks that hold, plus, for a precondition
harness, its satisfied cover". QSL needs no code change: it reads only zero versus non-zero.
QSpec FR-331-AC-8 should say "SUCCESS checks as the backend adapter counts them" (R-S2). Cost of
the alternatives: not counting leaves the contradiction; counting covers for every kind turns a
contract harness whose only passing check is its non-vacuity cover into a proof.

### (c) `KaniCheckResult`, `terminal_value`, `proof_category`: owner and the contract QSL-342 reads

None of the three exists at this base; they are PR 210's. Measured there: `KaniCheckResult{id,
class, location{file, line}, status}` lives in `kani_transcript.rs` and derives `Deserialize`
through `RawCheck` (field `category`, line as a string, `unknown` to `None`) and `Serialize` from
its own fields (field `class`). It is therefore a view of Kani's report and not a wire contract,
and it does not round-trip.

Ownership: CG owns all three. They are views of Kani's report and CG's map. QSL owns
`TerminalValue` and `TerminalValue::category`. PR 210's `proof_category` is
`terminal_value(..).category()` and adds a second public entry for one expression.

The contract QSL-342 reads (from its ticket text, untrusted, and consistent with the code at PR
210):

1. `terminal_value(&KaniRunOutcome, success_checks: u32) -> qsl_replay::TerminalValue`, replacing
   a hand-built `TerminalRecord` in its Kani exemplar.
2. The classified run: `outcome`, `success_checks` and `checks: Vec<KaniCheckResult>`, so QSL can
   attribute SUCCESS checks to each claimed module by `location.file` and `location.line`. The
   file path is relative to CG's generated crate and stable because generation is byte-identical.
3. `decode_falsification` (public, `kani_witness_join.rs:138`) and the transcript classifier, so
   QSL does not scrape Kani prose.

Recommendation: CG exposes exactly these; callers use `.category()` rather than a
`proof_category` function; `KaniCheckResult` gets one shape (serialize-only, or one wire struct
with matching names). Cost of leaving: a public `proof_category` can drift from QSL's category
function, and a struct that serialises differently from how it deserialises will be persisted by
someone. This is a recommendation for PR 210 and the CG spec follow-up, not a change in this PR.

## Risks

What is measured today, what is open and with whom, and what is routed. Measured on this
repository at the IR-321 subsystem layout, IR at its `main` branch and QSL at the `qsl-replay`
crate CG's lock selects.

### Current state and gaps

- E-1 gap, function path closed (IR-553): `ReplayPackage::obligation_identity` computes the
  function-contract identity through `core::canonical` (`quire-canonical`, a direct
  `branch = "main"` dependency) and the function path's request carries it. The V1 contract, scalar
  and frame paths still compute none, and the rest of this entry describes them.
  No code on those paths computes `ObligationIdentity`. QSL's type says QSL never hashes it.
  ADR-013 O-09 defines the preimage: the clause (or application) node id, its occurrence key,
  the obligation kind and the arguments (parameter node id and declared domain), source span
  excluded. CG's frame envelope takes a caller `[u8; 32]` (`frame_replay.rs:47`); the function
  path puts the transcript's byte digest in the request's obligation-identity slot at this base
  (AD-002); that digest is replaced by the function-contract identity, not retained.
  The work is larger than one missing field. Three identity structs exist and none carries what
  O-09 needs. `KaniObligationIdentity` holds a `ClauseRef`, not the clause node id, and no
  occurrence key; its `ObligationBinding` (`identifier`, `role`, `primitive_type`,
  `integer_bounds`, `dependencies`) has no parameter node id. `ScalarObligationIdentity` holds a
  node id but no occurrence key, and its `ScalarObligationArgument` (`identifier`, `minimum`,
  `maximum`) has no parameter node id. `StateFrameIdentity` (`state_frame.rs`) holds the clause
  node id and no occurrence key, and it is the identity of the frame harness whose
  counterexample goes into an envelope today (`frame_replay.rs:47`). Each must gain the
  missing members before one function can compute the O-09 value. Recommendation below.
- Encoder gap (measured at `main`): CG has no obligation-identity digest code and no
  `quire_canonical` use. The content digest of a corpus case is `serde_json::to_vec` plus a
  newline in `deterministic_json` (`kani.rs:1045-1046`; a second copy at `oracle.rs:1111-1112`),
  then `ByteDigest::of` from the `qsl-replay` API (`bounded_kani_corpus.rs:340-342`). CG also
  hashes bytes with `ByteDigest::of` for source files (`spine_replay.rs:149-152`) and the
  transcript (`spine_replay.rs:440`); those hash given bytes and canonicalise nothing.
  `serde_json` is not RFC 8785: key order is struct field order, and integers above 2^53, floats
  and negative zero are not canonicalised. `sha2` is a dev-dependency only. The obligation
  identity must not copy this. Target, ruled by QSL (as relayed): CG's one canonical-encoding
  place (`core::canonical`, CG layout AD step 1a) depends on `quire-canonical` directly (its own
  repository, `agent-ix/quire-canonical`, spelled `branch = "main"`; `to_vec(v, Limits)` and
  `sha256`, RFC 8785), with no `qsl-replay` re-export. The O-09 preimage is encoded there, never
  by `serde_json`, and the corpus case digest moves to the same place. `ByteDigest` stays only
  where QSL's own types require it (`qsl-replay` API surfaces such as source digests). This is a
  CG-owned decision, not a routed need. The CG layout AD (CG PR 215, open)
  already adopts this target: `core/canonical.rs` is the one caller of `quire-canonical`, both
  `serde_json` `deterministic_json` copies are deleted, and `ContentDigest` is a CG type built
  only by `core::canonical` over the RFC 8785 preimage. This AD only relies on it. Two encoders for one identity is the
  tangle IR has with its own digest; no ticket is filed here.
- Coupling note: QSL still pins `quire-canonical` by tag (`quire-canonical-v0.3.0`), so a lock
  that pulls QSL together with CG or IR on `main` (quire-integration, the driver) holds two
  copies of it until QSL moves to `branch = "main"` (waiting on the owner). CG step 1a and
  IR-274 must not merge into a two-copy lock; check the driver's one-copy gate first.
- E-3 gap: no CG code builds a `TerminalRecord` or the data an FR-331 `results` record needs,
  and CG is not required to. The driver owns the execute, replay and terminal-record chain
  (quire-driver PR 11); CG provides the map and its typed inputs. QSL exposes
  `TerminalRecord::new` and `BackendProviderSource` publicly. QSL's review says
  `TerminalRecord.item` (a string today) becomes a typed request index; the driver follows when
  it lands.
- Resolved decision, preimage: AD-001's Decisions section and FR-024 once gave the
  `ObligationIdentity` preimage as "every `KaniObligationIdentity` member except `source_span`",
  which conflicted with ADR-013 O-09's member list that E-1 uses (node id, occurrence key,
  kind, arguments as parameter node id and domain). Decision: O-09 wins. AD-001 and FR-024 now
  state the O-09 preimage (IR-553).
- Sequencing (QSL's review, not assumed): QSL-351 (an inconclusive value with a typed cause,
  and a non-zero count in `Proved`) and QSL-352 change `qsl-replay` types CG builds, so they
  land in step with CG, and QSL-351 lands before the IR-465 terminal map is written so that map
  is written once.
- At this base there is no terminal map at all: `TerminalValue` is named in no `src` file; FR-029
  and FR-030 and TC-040 and TC-041 are `Planned`. PR 210 (draft, unmerged) adds the map. The
  questions above read PR 210 as input only.
- The SUCCESS-check count is not carried on `KaniExecutionEvidence`
  (`kani_execution.rs`, the struct and `classify_transcript`); see question (b).
- IR still exports `KaniProviderResult` and `KaniProviderRecord` and maps every kind itself
  (`src/kani/outcome.rs`, `src/kani/mod.rs:23`), although IR's spec says the map moved to CG and
  FR-039 lists those items as not in IR's interface. Its map sends `Inconclusive` to
  `inconclusive`; CG's FR-030 sends `Inconclusive` with any other cause to `Failed`. Two maps of
  one row disagree. The removal is in IR-347's reopened scope; this AD files nothing new.
- CG imports model types through IR's root-crate glob re-export (for example
  `routed_generation.rs:20`, `generation.rs:10`, `kani.rs:8`; `Cargo.toml` has no
  `quire-contract-model` entry). IR's AD-001 says the root crate re-exports nothing and
  consumers import from the model crate; IR `src/lib.rs:12` still has
  `pub use quire_contract_model::*`. Removing the glob needs CG to add the model dependency: two
  repositories changing one import path, scheduled together, not a compatibility layer. Also in
  IR-347's reopened scope.

### Open questions

| Question | Owner | Recommendation |
| --- | --- | --- |
| What is the obligation-identity preimage, now that three CG identity structs exist and none carries the O-09 members? | CG proposes, QSL and QSpec confirm | State it by ADR-013 O-09's member list, not by struct name: the node id, occurrence key, kind and arguments (parameter node id and domain), source span excluded, RFC 8785 encoded by `quire_canonical`, one implementation in CG used by all three identities. This is the one canonical content-identity digest that binds a proof to its content; CG adds no other. QSL's `ObligationIdentity` stays opaque (it carries 32 bytes). CG writes E-1. QSL's doc says the digest domain is not in the closed FR-201 set; QSpec decides whether it needs a domain name (R-S3). |
| Replace IR's `KaniProviderResult` map | IR | Delete the type and its map as FR-039 already says, together with the CG import change above (IR-347). |
| IR cause codes as strings | IR | Export constants (or a typed cause enum) for the three codes (IR-347). |
| One map instead of two | CG after QSL | After option B of (a). |

No compatibility layer is proposed. If a seam above would need one (for example a CG copy of
QSL's terminal type), it is a design smell and the answer is to put the code with its owner.

### Routed gaps

Needs stated to owners, not decisions. Ids are the routing ids of IR-324; they are not
requirement ids.

To QSL (QSL reviews these rows):

| Id | Stated need |
| --- | --- |
| R-Q1 | An inconclusive `TerminalValue` with typed causes including `ReplayParity` and `ReplayRefused`, and a non-zero count in `Proved` (option B of (a); QSL-351, ahead of IR-465), and a typed request index in `TerminalRecord` (as relayed). `ReplayRefused` carries only QSL's catalogued codes (QSL's rule, as relayed). Decided by QSL (relayed), keyed on when a refusal happens. (a) Before Kani runs, when the obligation's own input is refused (nothing was proved): `Declined(ProofRefusalCause)`. The only CG producer of such a refusal is IR: its `Refused`, `InvalidInput` and `IncompleteInput` outcomes (join 1 to 2), which the IR-outcome rows of C-09 (FR-030) map. Those rows are outside the replay (counterexample) rows, so the replay rows stay about counterexamples only. QSL's example, "a refused caller lock", is QSL's and not a CG case: in CG a refused lock is `DependencyLockError` reached through `ReplayPackageError::Dependencies` (interface-001; FR-016-AC-16 to AC-19), only after Kani refuted, so it is case (b); IR checks the lock at checked-package admission, not as a `KaniOutcome`. Whether a second CG producer of (a) exists is for QSL to say only if it knows one. (b) After Kani refuted, when replay setup is refused on data (for example `CallSiteRefusal` `Compile`, `ModelIntake`, `DependencyInput`, `Import`, `Dependency`, `UnknownFunction`, `UnknownOperation`, `UnknownClause`; `InvalidFunction`; `Name`; a dependency-lock refusal): `Inconclusive(ReplayRefused)` with a QSL code, the same as an E9 refusal. Faults stay `Failed`. CG's own refusals before Kani get no terminal value today, and are stated here, not decided: generation refusals (FR-015 refusals; an item whose claim is a `NoDerivableClaim` refusal settles `unsupported`, FR-015-AC-15; and a generation that ends in the `GenerationErrorCode::UnsupportedObligations` error, interface-001's terminal state `unsupported`, `oracle.rs`; each has no run, no artifact and no terminal value, FR-029 scope); `HarnessNotInCrate` from `execute_kani_obligation` (FR-022) is a refusal to run, and no requirement gives it a terminal value. The latter is open for CG with the driver; a driver-side defect that refuses to run is a candidate for `Failed`. QSL adds the catalogued codes for the non-fault refusals in QSL-352 (relayed), landing with CG layout AD-004 step 5; until then the `Failed` mapping of link 7 is the interim. |
| R-Q2 | Edit QSL's `ObligationIdentity` doc to point at ADR-013 O-09's member list instead of naming `KaniObligationIdentity`. Move QSL's `quire-canonical` dependency from the tag to `branch = "main"` (waiting on the owner) so locks hold one copy. |
| R-Q3 | Resolved by QSL-354 as merged: ADR-013 O-16 and C-09 now say CG owns the map. Kept so the id is not reused. |
| R-Q9 | The tool pin. `BackendProviderSource` has a public `tool_pin` string and ADR-013 O-24 says the envelope carries a tool pin; QSL-351's deletion of `ToolPin` (relayed) removes it, a QSL-only change that needs nothing from CG. Agreed with QSL; kept so the question is not reopened. CG mints and proposes no pin. |

To QSpec:

| Id | Stated need |
| --- | --- |
| R-S2 | FR-331-AC-8 wording of "SUCCESS check": the count is the backend adapter's. |
| R-S3 | Whether the obligation identity needs a digest domain name. |

To IR (the AD owner; listed so they are not lost; wording as in IR PR 239; no new ticket is
filed):

| Id | Stated need |
| --- | --- |
| R-I1 | Delete `KaniProviderResult`, `KaniProviderRecord` and the `provider_result` map from the root crate (`src/kani/outcome.rs`, `src/kani/mod.rs:23`); they duplicate the terminal map CG owns. In IR-347's reopened scope. |
| R-I2 | Export cause-code constants or a typed cause enum: the Kani cause codes cross as bare strings that consumers re-spell. Overlaps IR-347's free-string cause codes. |
| R-I3 | Remove `pub use quire_contract_model::*` (`src/lib.rs:12`) together with CG adding a direct `quire-contract-model` dependency; CG imports model types through the glob. In IR-347's reopened scope. |
