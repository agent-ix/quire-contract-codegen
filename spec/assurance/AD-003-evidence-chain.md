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
| 3 | Obligation: harness, bound domains, ceilings, oracle symbols (`KaniObligationIdentity`, `ScalarObligationIdentity`) | CG | JSON record persisted beside the harness; no digest is computed from it (gap E-1) |
| 4 | Corpus case: `CaseIdentity` over construct, profile, input, request, dependencies | CG | lowercase SHA-256 over its deterministic JSON, which also names the artifact paths; collisions refused by `EmittedCorpusIdentities` (`bounded_kani_corpus.rs`, `kani_corpus_identity_collision`) |
| 5 | Launch: argument vector, ceilings, launcher, harness-in-crate check (`execute_kani_obligation`, `kani_launch_command`, `launch_evidence`) | CG | harness source must appear byte for byte in the crate before launch (`kani_execution.rs`, FR-017 step 1) |
| 6 | Run classification: `classify_kani_run` over the typed transcript to `KaniRunOutcome` | CG | one parser module, `kani_transcript.rs`; Kani's wording read nowhere else |
| 7 | Terminal value: (`KaniRunOutcome` or `KaniOutcome`, replay settlement) to `qsl_replay::TerminalValue`. The input is the pair, not the Kani outcome alone, and the map is total over the replay result, which is one of three. (1) Settled: a `Refuted` outcome whose replay disagrees becomes `Inconclusive(InconclusiveCause::ReplayParity)` (code `replay_parity`), carrying the replay's `DisagreementCause`. (2) Refused (identity mismatch, decode refusal, stale dependency, limit reached): `Inconclusive(InconclusiveCause::ReplayRefused)` (code `replay_refused`), carrying the `ReplayRefusal` code. (3) Fault (QSL `InternalFault`): `TerminalValue::Failed`, so a defect stays loud. A refuted Kani outcome never becomes `Refuted` without a reproduced replay. ADR-013 O-16's inconclusive row and O-27 support (1); both causes land with QSL-351 and their names are as QSL gave them, not yet in QSL's `main`. At this base `ReplayRefusal` has a `Fault(InternalFault)` variant beside the refusals, so CG separates (2) from (3) by variant | CG (map), QSL (type) | one total `match` each over the outcome, no wildcard arm (FR-029, FR-030; both `Planned`, and both written over the outcome alone today) |
| 8 | Counterexample join: decoded playback to QSL replay, with `ObligationIdentity` | CG builds, QSL consumes | see AD-002; `package_id` recomputed by QSL |
| 9 | FR-331 `results` record | QSL type (`TerminalRecord`, `ProofResultEnvelope`), QSpec wire | none in CG (gap E-3) |

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
| 6 to 7 | outcome, SUCCESS-check count and, for a falsified run, the replay settlement | none: the map is total (when built); a replay disagreement is `Inconclusive(ReplayParity)`, a replay refusal `Inconclusive(ReplayRefused)`, a replay fault `Failed` | CG |
| 7 to 9 | terminal value and item identity | not built (E-3) | n/a |

Each failure state stays a distinct typed state (AD-001 Failure view). No state is converted to
success.

### Versioned contracts and how identity is asserted

- The identity that binds a proof to its content is the obligation identity: one canonical
  content-identity digest, minted by CG over the obligation's identity members, carried in the
  envelope as QSL's `ObligationIdentity` (E-1). It is the only digest CG mints for a proof, and no
  other digest, pin, SHA or version record is added to this chain.
- The other digests on the chain already exist and bind content, not tools or versions:
  `package_id` (QSL recomputes it on replay), the byte digests QSL checks on provided source, the
  `CaseIdentity` name of a corpus case and the counterexample identity (`ByteDigest` of the
  transcript).
- There is no pin, SHA or digest over a file, version or tool, and none is proposed. In
  particular the Kani version is not pinned; classification reads Kani's output through the one
  transcript parser.
- The seam to QSL is the `qsl-replay` Rust API, asserted by compilation. The seam to IR is IR's
  root crate API, asserted by compilation.
- Cause codes cross IR to CG as strings (`KaniOutcome.code`). IR spells `kani_vacuous_proof` only
  inside `KaniOutcome::proved_from_checks`; `kani_solver_absent` and `kani_backend_absent` are
  named in IR's and CG's specs but defined by no IR constant or enum, so CG spells them itself. A
  typo is not a compile error. IR-347 (reopened) already covers the free-string cause codes; no
  new ticket is filed.

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
  as RFC 8785 JSON; the value changes when any included member changes and does not when the
  span changes. The scalar and the V1 obligation paths both use it.
- E-2. Two obligations with identical identity members have the same `ObligationIdentity`;
  regeneration is byte-identical (NFR-001).
- E-3. Every run item has exactly one terminal value, and the map from `KaniRunOutcome` and from
  `KaniOutcomeKind` is one `match` with no wildcard arm (FR-029-AC-1 and FR-030-AC-7 are the
  existing form). A falsified run's value is a function of the outcome and the replay
  settlement together (link 7): a replay disagreement is `Inconclusive(ReplayParity)`, a replay refusal is `Inconclusive(ReplayRefused)`, a replay fault is `Failed`, and a refuted outcome without a reproduced replay is never `Refuted`.
- E-4. No outcome maps to `Tested`.
- E-5. A run whose SUCCESS-check count is zero maps to a value QSL reads as non-success. A
  precondition harness counts its satisfied cover as its one SUCCESS check (question b).
- E-6. A stale report from a previous run cannot be read as this run's verdict.
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

Who owns the map (C-09): CG. QSL ADR-013 O-16's table and O-24 already say the proof column is
"mapped by one exhaustive CG-owned function (C-09)" and that CG maps IR's `KaniOutcome` into
QSL's `TerminalValue`. One sentence in O-16 still says "IR implements the map", and IR's
`KaniOutcomeKind::provider_result` doc says the map is "implemented here". IR cannot own it (IR
must not depend on QSL) and retired its map (IR FR-031-AC-5, IR-358); CG depends on both and owns
it. Once option B lands, collapse FR-029 and FR-030 into one table over IR's `KaniOutcome` plus
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

- E-1 gap: no code in CG computes `ObligationIdentity`. QSL's type says QSL never hashes it.
  ADR-013 O-09 defines the preimage: the clause (or application) node id, its occurrence key,
  the obligation kind and the arguments (parameter node id and declared domain), source span
  excluded. CG's frame envelope takes a caller `[u8; 32]` (`frame_replay.rs:47`); the function
  path puts the transcript's byte digest in the request's obligation-identity slot (AD-002).
  Neither CG identity struct carries the occurrence key: `KaniObligationIdentity` holds a
  `ClauseRef`, and `ScalarObligationIdentity` holds a node id and no occurrence key, so each
  must gain it before one function can compute the O-09 value. Recommendation below.
- E-3 gap: no CG code builds a `TerminalRecord` or the data an FR-331 `results` record needs.
  QSL exposes `TerminalRecord::new` and `BackendProviderSource` publicly, so CG can build both.
  QSL's review says `TerminalRecord.item` (a string today) becomes a typed request index; CG
  follows when it lands. Which component writes the `results` wire is not established here.
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
| What is the obligation-identity preimage, now that the V1 `KaniObligationIdentity` is superseded on the scalar path? | CG proposes, QSL and QSpec confirm | State it by ADR-013 O-09's member list, not by struct name: the node id, occurrence key, kind and arguments (parameter node id and domain), source span excluded, RFC 8785 encoded, one implementation in CG. This is the one canonical content-identity digest that binds a proof to its content; CG adds no other. QSL's `ObligationIdentity` stays opaque (it carries 32 bytes). CG writes E-1. QSL's doc says the digest domain is not in the closed FR-201 set; QSpec decides whether it needs a domain name (R-S3). |
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
| R-Q1 | An inconclusive `TerminalValue` with typed causes including `ReplayParity` and `ReplayRefused`, and a non-zero count in `Proved` (option B of (a); QSL-351, ahead of IR-465), and a typed request index in `TerminalRecord` (QSL-354 as relayed). |
| R-Q2 | Edit QSL's `ObligationIdentity` doc to point at ADR-013 O-09's member list instead of naming `KaniObligationIdentity`. |
| R-Q3 | The one sentence of ADR-013 O-16 that says IR implements the proof-column map; CG owns it. |

To QSpec:

| Id | Stated need |
| --- | --- |
| R-S2 | FR-331-AC-8 wording of "SUCCESS check": the count is the backend adapter's. |
| R-S3 | Whether the obligation identity needs a digest domain name. |

IR-owned items R-I1 and R-I3 are in IR-347's reopened scope and R-I2 overlaps IR-347's
free-string cause codes; no new ticket is filed for them.
