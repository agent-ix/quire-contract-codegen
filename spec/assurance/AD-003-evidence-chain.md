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
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-033
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
request itself (AD-002). The existing source-predicate/function/frame joins below retain their
meaning; planned composite node parity uses FR-033/FR-029 and never produces source Refuted.
Its structural O-09 replacement and separate same-artifact content tie are explicitly scoped below.

## Views

The chain is described link by link, then by what crosses each join and who reports a failure,
then by how identity is asserted.

### The chain

| # | Link | Owner | Identity or content binding |
| --- | --- | --- | --- |
| 1 | Checked package, `package_id` (`quire.package.semantic/v2`) | QSpec wire, QSL emits, IR re-derives on read | digest over the package's identity preimage |
| 2 | Finite profile and input (`ProfileSelection`, `ValidatedFiniteInput`, `KaniOutcome` for refusals) | IR | profile revision selected with the ABI and module revisions |
| 3 | Obligation: harness, bound domains, ceilings, oracle symbols (`KaniObligationIdentity`, `ScalarObligationIdentity`, and for frame harnesses `StateFrameIdentity`, `state_frame.rs`) | CG | JSON record persisted beside the harness; no digest is computed from it (gap E-1) |
| 4 | Corpus case: `CaseIdentity` over ceilings, construct, profile, input, request, dependencies | CG | lowercase SHA-256 over its RFC 8785 canonical JSON without a trailing newline, which also names the artifact paths; unencodable preimages refuse as `kani_corpus_identity_unencodable` and collisions refuse as `kani_corpus_identity_collision` (`bounded_kani_corpus.rs`) |
| 5 | Launch: argument vector, ceilings, launcher, harness-in-crate check (`execute_kani_obligation`, `kani_launch_command`, `launch_evidence`) | CG | harness source must appear byte for byte in the crate before launch (`kani_execution.rs`, FR-017 step 1) |
| 6 | Run classification: `classify_kani_run` over the typed transcript to `KaniRunOutcome` | CG | one parser module, `kani_transcript.rs`; Kani's wording read nowhere else |
| 7 | Terminal value: (`KaniRunOutcome` or `KaniOutcome`, replay settlement) to `qsl_replay::TerminalValue`. The input is the pair, not the Kani outcome alone. For a counterexample the second member is the E9 replay result, as ADR-013 C-09 states it (QSL-354, merged): (1) settled, with parity settled inside `replay`: a reproduced replay gives `Refuted`, and a disagreement (or a replay that completes no value) gives `Inconclusive(InconclusiveCause::ReplayParity)` (code `replay_parity`), carrying the replay's `DisagreementCause`. (2) A non-fault `ReplayRefusal` (identity mismatch, decode refusal, stale dependency, limit reached) gives `Inconclusive(InconclusiveCause::ReplayRefused)` (code `replay_refused`) carrying the refusal's catalog code: only QSL's closed set of `ReplayRefusal` codes, never a CG code. (3) A fault gives `TerminalValue::Failed`, so a defect stays loud. A refuted Kani outcome never becomes `Refuted` without a reproduced replay. A fault is a fault wherever it sits in the chain, not only as the variant `ReplayRefusal::Fault`: QSL also reports `ReplayRefusal::Admission(AdmissionFailure::Fault(_))` (catalog code `runtime_invariant`, `execute.rs`) and `CallSiteRefusal::Fault` (`call_site.rs`), which reaches CG directly and wrapped in `ReplayPackageError::CallSite`, `FrameReplayError::CallSite` and `StateClauseReplayError::CallSite`, and a `ReplayRefusal` fault (either form) wrapped in `SpineReplayError::Refused`, `FrameReplayError::Refused` or `StateClauseReplayError::Refused`; the map classifies by walking the whole error, not by its top variant. CG's own failures before any replay runs also need a value (below). Merged QSL text agrees: ADR-011 T-13 and ADR-013 O-16, O-24 and C-09 say the driver runs the Kani obligation and the replay, passes both to CG's map, and writes each item's FR-331 terminal record. The two inconclusive causes `replay_parity` and `replay_refused` are named in the merged C-09 text; the `TerminalValue::Inconclusive` variant that carries them is merged in QSL `main` (02530e7). CG provides the map and its typed inputs; the driver builds the terminal record (ADR-011 T-13; quire-driver PR 11) | CG (map), QSL (type) | one total `match` each over the pair, no wildcard arm (FR-030-AC-7; the FR-029 map is built as `run_terminal_value` and the FR-030 map as `ir_outcome_terminal_value`, each over the pair (outcome, replay settlement), `Refuted` only with a reproduced replay, IR-465 and IR-358; the fault readings are asserted over every wrapper by tests built from QSL's constructible `InternalFault` (FR-029-AC-10, FR-030-AC-10); the 0 -> -1 post-state value outside its range is preserved exactly and settles as the ordinary `Refuted` terminal (FR-029-AC-18, QSL-634, IR-460)) |
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
| 6 to 7 | outcome, SUCCESS-check count and, for a falsified run, the replay result | none: the map is total over (outcome, replay result), once every pre-replay failure below is classified; a replay disagreement is `Inconclusive(ReplayParity)`, a replay refusal or a setup refusal that carries a QSL code `Inconclusive(ReplayRefused)`, a fault anywhere `Failed` | CG |
| 6 to 7, falsified run stops in CG before any replay | `EvidenceFailureCause::{Decode, Domain}`, `SpineReplayError::{UnboundArgument, FieldDelimiter, Transcript, WrongArm, Identity}`, `ReplayPackageError::{InvalidFunction, Dependencies, CallSite}`, `FrameReplayError::{Dependencies, CallSite, Name, Transcript, Envelope}` | Decided here, with QSL's closed-set rule (as relayed): the replay did not run, so `Refuted` is withheld, and `ReplayRefused` carries only QSL's catalogued codes, a closed set QSL owns. The first group maps to `Failed` so a CG defect stays loud, because each is CG's own defect, with no QSL code: `Domain` (a playback outside the harness proof bound, which CG's pre-check exists to keep visible), `UnboundArgument` (CG's harness bindings and QSL's `call_site` disagree), `FieldDelimiter` (harness text CG generated), `Decode` (a playback that does not type against the bindings CG persisted), `Transcript`, `Envelope`, `WrongArm`, `Identity`, `InvalidFunction` and `Name` (`InvalidFunction` wraps a discarded `InvalidIdentifier` from `Identifier::new`, which has no code; `Name` wraps QSL's `EmptyQualifiedName`, which has no code, and is unreachable as built because every `QualifiedName::new` call passes a non-empty list), and any fault, a `CallSite` fault included. The second group is replay setup refused on data after Kani refuted, by a QSL refusal that carries a code: a non-fault `CallSite` refusal (for example `Compile`, `ModelIntake`, `DependencyInput`, `Import`, `Dependency`, `UnknownFunction`, `UnknownOperation`, `UnknownClause`) and `DependencyLockError::Input`. Decided by QSL for the call-site refusal (the when-rule, R-Q1, relayed on IR-465, a QSL ruling recorded by the planner); `DependencyLockError::Input` is CG applying the same rule, sound because the same `DependencyInputRefusal` reaches the map as `CallSiteRefusal::DependencyInput`. It maps to `Inconclusive(ReplayRefused)` with the QSL catalog code from `CallSiteRefusal::code()` or `DependencyInputRefusal::code()`, the same as an E9 refusal, and never to `Declined`, which is for a refusal before any backend run. A lock that repeats a dependency identity is `DependencyLockError::Input` too: CG has no duplicate error of its own (FR-016-AC-24, built), and QSL's `DependencyInput::new` refuses it as `DuplicateIdentity`, `invalid_package` with cause `conflicting-definition` (QSL ruling, relayed on IR-465, a QSL ruling recorded by the planner; see R-Q1). Faults stay `Failed` throughout. | CG |
| 7 to 9 | terminal value and item identity | the driver's step; CG builds no record | driver |

Each failure state stays a distinct typed state (AD-001 Failure view). No state is converted to
success.

### Versioned contracts and how identity is asserted

- For scalar and composite parity, CG carries the admitted IR reader's structural application identity and retains the complete checked claim body for comparison (E-1). The identity names the application; the body comparison binds the claim and harness domains. Neither alone binds every byte or evidence field of the generated artifact. The full CG identity/evidence record remains a separate record.
- Planned composite parity separately authenticates the actual native observation against the
  retained proved generated artifact, original context, operands and limits under
  [FR-033-AC-19](../replay/functional/FR-033-composite-parity-replay-binding.md). Structural node
  identity and a matching QSL report cannot establish this tie. QSL FR-358 has no
  `CompositeParityClaim.content_identity` member; CG performs the same-artifact check before QSL
  invocation and adds no digest, substitute identity or tracking member. Driver authentication and
  consumer conformance remain CODE-gated.
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
  crate API, asserted by compilation. CG reads model items through its direct `quire-contract-model` dependency.
  IR-347 moved the Kani semantic-family lowerings and their request/result types into CG;
  IR export removal follows the consumer move. FR-015-AC-109 has since retired CG's arithmetic
  lowerer and corpus arm. The collection and graph lowerers remain in CG pending FR-015-AC-110
  and FR-015-AC-111. The validated finite-input/profile/dispatch interfaces and typed
  `KaniOutcome` remain IR-owned.
- Cause codes cross IR to CG as `quire_contract_model::Std001Code` (IR FR-044), not as strings:
  `KaniOutcome.code`, `KaniProviderRecord.cause` and the `CapabilityDisposition` codes carry the
  type, which guarantees the STD-001 code form and nothing else. A code STD-001 registers outside
  `DiagnosticCode` is a `Std001Code` constant (`Std001Code::KANI_VACUOUS_PROOF`, which CG's
  `classify_success` compares against), so a typo there is a compile error. A code CG mints
  itself (`kani_corpus_dependency_invalid`, `kani_corpus_identity_collision`,
  `kani_corpus_serialization_failed`, `kani_profile_input_mismatch`) is built with the
  `std001_code!` macro, which checks the form at compile time; STD-001 does not list those, and
  `Std001Code::is_registered` is false for them. Under the planned IR-621 CG adoption,
  the retained family/corpus paths use IR's public infallible `KaniOutcome::refused`,
  `invalid_input` and `resource_exhausted` constructors for their respective observed kinds.
  They pass the original typed cause, source identity and selected profile/bound context;
  CG remains responsible for selecting the actual refusal and does not infer a run result.
  The three public operations in `interface-001` return `Result<Success, KaniOutcome>`
  directly, without `FamilyLoweringError`, `BoundedCorpusError`, an `OutcomeConstruction`
  branch, or a replacement alias or wrapper. IR retains its fallible `non_success` API
  for callers holding a broad kind; IR's separate constructor validation and proof-count
  criteria are not accepted by this consumer change. Terminal cancellation pairing is
  outside this adoption, and AD-004's final corpus retirement remains separately gated.

## Decisions

CG owns the map into QSL's terminal value and the retained parity claim body; IR owns the structural application identity. QSL owns the terminal and replay types. The statements below are ones a test can check, followed by the three questions
raised in IR-324 from parked CG PR 210 and their recommendations. PR 210 (draft, open, laid out
before the subsystem restructure) is input here and is not at this base.

### Claim binding allocation

[FR-032](../replay/functional/FR-032-routed-scalar-replay-binding.md) owns scalar producer,
context-retention, driver and consumer duties;
[FR-033](../replay/functional/FR-033-composite-parity-replay-binding.md) owns those duties for
composite parity. Their positional checked claim bodies and full report claims have separate roles.
Repeated operand positions remain separate even when they name the same node; literal operands
retain their route's authentic identity/domain. Function/frame preimages remain unchanged.

Full sent-claim comparison on every outcome uses the actual retained claim sent by the driver.
The owning requirements also retain the original replay wire/context and proving limits for
independent source/package/context checks and actual same-artifact native observation
where required. An identity echo alone does not authenticate an artifact. Structural application
identity, native same-artifact authentication and refinement evidence retain distinct roles. Public transport
`ReplayLimits` remains a separate final facade argument, rather than a claim-identity member.

These CG producer, metadata, observation and consumer checks remain PLANNED/UNRUN and CODE-gated
under their owning requirements; API availability and deterministic regeneration close none of
them. E-2 below states only deterministic identity regeneration.

### Invariants a test can check

Candidate statements (local labels; the repository assigns requirement ids when one is
authored).

- E-1. CG carries QSL's producer-issued structural `ObligationIdentity` unchanged:
  the obligation node's `NodeKey`, optional owning `ModelIdentity`, and producer-assigned ordinal
  for that node/model pair. QSL ADR-013 O-09 and QC-14 were corrected in
  878381034d5ddda662def7cffe0096d0e3c5f0cb. The producer assigns distinct ordinals to
  distinct obligations at the same node/model. CG never computes a preimage, hashes an
  obligation identity or derives a node from a replay transcript. The complete source occurrence,
  obligation kind, parameter identities and declared domains stay on the obligation record and
  are compared field by field against the independently selected claim. A mismatch refuses before
  replay, even when structural identity is unchanged; a changed structural component changes
  identity. Function and frame paths retain their own typed record and replay controls in
  FR-016-AC-26 through AC-28 and FR-024-AC-47 through AC-51. This structural obligation identity
  remains distinct from the canonical content identity binding a proof to exactly what it proved.
- E-1 per-step refinement records (PLANNED/CODE-GATED, IR-942). The QSL
  ADR-013 O-07 occurrence of a QSV `StepSimulationRecord` is a locator on the
  owning refinement node: role `generated`, ordinal 0 for `Initial` and 1 +
  the zero-based `StepRowRef` index for `Step`. The index is the row's
  position in QSpec FR-375-AC-22's declaration-ordered `steps` array. Source
  permutation changes the index and occurrence ordinal of a moved row; ties
  on node key or member name do not introduce a separate sorting rule. The
  record *content* identity is a separate SHA-256 over one RFC 8785 object
  with exactly `subject_kind` equal to `protocol-refinement-step` and `record`
  equal to QSV's canonical
  `StepSimulationRecord` encoding. CG uses `quire-canonical` and QSV's owned
  type/encoder directly; it does not copy or reinterpret the record schema.
  The QSV record's `refinement`, `obligation` and supplied-order `hypotheses`
  all participate. CG mints this identity only after the selected owner and
  occurrence validate. Comparing different `refinement` values uses two
  separately valid selected refinements, each with its matching record and
  occurrence; a mismatch under one fixed selected owner refuses without an
  identity. The O-07 occurrence key, source regions, `source_span`, CG
  proof kind, CG `ObligationKind`, and harness execution settings do not enter
  that content identity. The per-step `KaniObligationIdentity` is a distinct
  persisted *harness* identity record: its per-step subject carries the exact
  QSV record, its content identity, and the separate O-07 occurrence key;
  its persisted harness-record digest covers every record member except
  `source_span`. This planned producer closes the current link-3 no-digest
  gap for per-step records. A CG kind needed by the harness remains a harness field,
  never a second discriminator in the record content preimage. QSpec FR-322-AC-88 through AC-91 permit generated owner occurrences only for proof-eligible refinements; an ineligible refinement carries its declaration occurrence alone and yields no per-step harness or proof credit. CG refuses
  before generating a harness when the record owner and selected refinement
  differ, a `StepRowRef` is out of range, the occurrence role or ordinal does
  not match the obligation, or a package repeats a `(refinement, obligation)`
  pair even with different hypotheses. A refusal has no content identity or
  proof credit. The existing V1 clause identity rule above does not define
  this per-step record shape; FR-015-AC-120 through AC-122 own its tests.
- E-1 scalar allocation (PLANNED/CODE-GATED, IR-631; replacement for the retired parity
  preimage). CG takes the original application `NodeKey`/`CheckedNodeId` and authentic `expression`
  occurrence from the admitted IR reader. CG typed generation supplies and retains the scalar
  kind and one argument per operand position in original draw order as the checked claim body. Every argument retains
  its position, typed `GraphChild` or `InlineLiteral` identity and actual harness range.
  An inline literal uses the application node/occurrence/position and its singleton range;
  an existing literal node is a graph child. Two independent draws of one parameter remain
  two entries, with no identifier sorting or node-id deduplication. Unrelated enclosing
  parameters are excluded. The scalar kind `operator_parity` comes from typed generation,
  not a caller-chosen string. Missing or ambiguous occurrence, positional provenance, kind
  or checked identity refuses before replay. CG validates graph-child ranges against the
  original proving schema and retains the original limits. A package/path/producer-ordinal
  change changes the structural identity; a body-only change keeps that identity but is
  detected by full retained-body comparison and refuses before scalar comparison. CG does
  not call `parity_obligation`, construct `ParityPreimage`, derive an obligation hash or copy
  an encoder. The actual QSL operator route must accept the matching original claim and
  refuse membership or body mismatch. The driver executes the same proved generated artifact
  at decoded operands and original renderer limits and compares the receipt's staged artifacts,
  harness and record byte for byte against the retained originals before checked completion.
  An outcome echo or freshly regenerated oracle cannot authenticate it. Before settlement CG
  compares every member of the report's full claim with the retained sent claim and the
  completed wire/run context against the original proving package, source, dependencies and
  limits, on every outcome including refusal and fault. QSL does not authenticate the artifact.
  Full-report binding and same-artifact observation remain separate obligations. [FR-032](../replay/functional/FR-032-routed-scalar-replay-binding.md)
  AC-15 through AC-18 replace the retired hash-specific AC-9/10/11/13 without dropping
  their member-change, negative-path and no-fabricated-identity controls.
- E-1 composite extension (PARTIAL: IR-635 Eq and some Ne controls; IR-814 and IR-806
  retain their planned controls). CG takes the original equality/inequality application
  structural identity from IR admission and retains its authentic occurrence, kind and one
  argument per operand position. A self-comparison contributes two graph-child entries for
  the same actual parameter, each with the drawn Node-keyed bounds. Distinct parameters and
  composite literal nodes retain separate graph-child entries; literals have empty Bounds.
  The owning operand projection refuses top-level inline literals before QSL invocation.
  Within each Bounds argument, canonical key order and duplicate-key refusal remain; repeated
  positions are not duplicate keys in one argument. Population-keyed metadata remains in
  the full CG proving record and a QSL claim carrying it refuses as `HarnessUnknownKey` during
  prepare, before structural identity/body comparison. CG pre-invocation refusal has no QSL
  report or terminal value. Full report-claim comparison and independent original-context/same-artifact
  authentication remain required on every outcome.
  A package/path/producer-ordinal change changes the structural identity; an included body
  change under the same identity refuses by full body comparison. No CG or QSL parity
  preimage mint or obligation digest is needed. [FR-033](../replay/functional/FR-033-composite-parity-replay-binding.md)
  AC-18 replaces the retired hash-specific AC-11 while preserving its positional, adverse,
  membership and evidence controls.
- E-2. Two obligations with identical node, model and producer ordinal have the same structural `ObligationIdentity`; distinct obligations at one node/model receive distinct ordinals. Equal identity does not bypass exact record/body comparison; regeneration from equal admitted inputs is byte-identical (NFR-001).
- E-3. Every run item that reaches the map has exactly one terminal value, and the map from
  `KaniRunOutcome` and from `KaniOutcomeKind` is one `match` with no wildcard arm, over the pair
  with the replay settlement (FR-030-AC-7; FR-029-AC-1 and FR-029-AC-12 state `Refuted` only with a
  reproduced replay). A falsified run's value is a function of the outcome and the replay result together
  (link 7): a replay disagreement is `Inconclusive(ReplayParity)`, a replay refusal is
  `Inconclusive(ReplayRefused)`, a fault anywhere in the chain is `Failed`, and a refuted
  outcome without a reproduced replay is never `Refuted`. Every CG failure before replay is
  classified by the link 7 rule (a CG defect or a fault: `Failed`; a replay setup refused on data by
  a QSL refusal: `Inconclusive(ReplayRefused)`), so no falsified run is left without a value.
  A refusal of the obligation's own
  input before Kani runs, as IR reports it, is `Declined(ProofRefusalCause)` through the
  IR-outcome rows of C-09 (FR-030; R-Q1) and is outside the replay (counterexample) rows of C-09.
- E-4. The ordinary production/source-predicate map returns no `Tested`; planned composite parity
  may consume `Tested` only under FR-029 AC-19 and FR-033. The owning QSL API is delivered;
  CG retention, driver authority, concrete representation and consumer conformance remain CODE-gated.
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
| B (partly declined by QSL, relayed on IR-465: the non-zero `Proved` count is declined and vacuity stays `Proved{0}`; the `Inconclusive` half is merged in QSL 02530e7). QSL adds `TerminalValue::Inconclusive(InconclusiveCause)` with causes for the vacuous proof, the unsatisfied cover and no-qualified-interpretation, and `Proved` carries a non-zero count | QSL type, reader and FR-069; QSpec FR-331-AC-8 wording; CG's three rows (vacuous, cover-unsatisfied, `Inconclusive`-other) and TC-040 and TC-041 | one QSL change plus one CG follow-up; removes `Proved{0}` as a magic value |
| C. CG reports unsatisfied cover as `Failed` | CG only | blames the tool for a property of the model |

Option B also gives a home to the replay results of link 7: the inconclusive value
carries `InconclusiveCause::ReplayParity` (code `replay_parity`, carrying `DisagreementCause`) or
`InconclusiveCause::ReplayRefused` (code `replay_refused`, carrying the `ReplayRefusal` code), which no existing `TerminalValue` variant can. This is QSL's and
QSpec's decision (routed R-Q1, R-S2); QSL's review said option B arrives as QSL-351, whose merged part is #551 (ToolPin) only; the inconclusive types have since merged in QSL 02530e7. QSL has since ruled (relayed on IR-465, a QSL ruling recorded by the planner) that vacuity stays `Proved{0}` with no `NonZero` count, so CG's FR-029 and FR-030 keep the vacuous and cover-unsatisfied rows as `Proved{0}`; the replay rows are built on the merged `Inconclusive` types, and CG adds nothing that would need to be removed (no shim).

Who owns the map (C-09): CG. QSL's ADR-013 as merged (QSL-354) says so throughout: O-16's table
calls the proof column "mapped by one exhaustive CG-owned function (C-09)", the invariants
paragraph says "CG implements the map over the Kani outcome and the E9 replay result", and C-09
takes the Kani outcome and the replay result. IR's `KaniOutcomeKind::provider_result` doc still
says the map is "implemented here". IR cannot own it (IR must not depend on QSL) and retired its
map (IR FR-031-AC-5, IR-358); CG depends on both and owns it. Once the pending inconclusive types land, collapse FR-029 and FR-030 into one table over IR's `KaniOutcome` plus
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
3. `decode_falsification` (public, in `src/replay/witness.rs`) and the transcript classifier, so
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

- E-1 structural migration (IR-1079): the former function/frame O-09 four-member
  digest and `core::canonical` encoder were implemented historically but are retired as
  obligation-identity mechanisms. QSL's authoritative value is the producer-issued
  `(NodeKey, Option<ModelIdentity>, ordinal)` tuple (ADR-013 O-09 and QC-14 correction
  878381034d5ddda662def7cffe0096d0e3c5f0cb). CG carries it unchanged for the
  function/frame replay request and envelope, compares occurrence, kind and sorted
  argument domains as separate record fields, and preserves all typed mismatch refusals.
  The V1 clause and scalar routes remain separately tracked; parity already uses its
  structural producer identity and complete checked-body comparison. `core::canonical`
  remains available for content identities that bind exact proved bytes, never for an
  obligation identity. The postcondition state-clause caller identity remains Q-1.
- E-3 gap: no CG code builds a `TerminalRecord` or the data an FR-331 `results` record needs,
  and CG is not required to. The driver owns the execute, replay and terminal-record chain
  (quire-driver PR 11); CG provides the map and its typed inputs. QSL exposes
  `TerminalRecord::new` and `BackendProviderSource` publicly. QSL's review says
  `TerminalRecord.item` (a string today) becomes a typed request index; the driver follows when
  it lands.
- Resolved decision, structural obligation identity: the historical AD-001/FR-024
  four-member preimage decision is retired. QSL ADR-013 O-09 and QC-14 now define
  node/model/producer ordinal; CG compares kind, occurrence and domains on the record.
- Sequencing (QSL's review, not assumed): an inconclusive value with a typed cause and a non-zero
  count in `Proved` change `qsl-replay` types CG builds. They have since merged in QSL
  (02530e7), before the IR-465 terminal map code was written, so that map was written once.
- At this base there is no terminal map at all: `TerminalValue` is named in no `src` file; FR-029
  and FR-030 and TC-040 and TC-041 are `Planned`. PR 210 merged without the map (IR-465 re-planned
  it to AD-004 step 5). The questions above read PR 210 as input only.
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
| ~~Retired parity obligation-preimage decision~~ | IR FR-322 structural identity; QSL-1293 removes parity preimages | The earlier O-09 parity digest decision is historical and no longer authorizes a parity mint. CG obtains the application `NodeKey`/`CheckedNodeId` from IR admission, retains the authentic occurrence, kind and ordered arguments, and compares the complete body even when structural identity is unchanged. Full FR-015-AC-76 evidence and FR-033 same-artifact authentication remain separate. Function/frame identity now follows the same structural tuple, with their own record comparisons. |
| Replace IR's `KaniProviderResult` map | IR | Delete the type and its map as FR-039 already says, together with the CG import change above (IR-347). |
| IR cause codes as strings | IR | Resolved for the code type: IR exports `Std001Code` with constants for the registered codes (IR FR-044). The family lowerings' own codes are typed `std001_code!` values that STD-001 does not register; they leave with the lowerings (IR-347). |
| One map instead of two | CG after QSL | After the pending inconclusive types of (a) land. |

No compatibility layer is proposed. If a seam above would need one (for example a CG copy of
QSL's terminal type), it is a design smell and the answer is to put the code with its owner.

### Routed gaps

Needs stated to owners, not decisions. Ids are the routing ids of IR-324; they are not
requirement ids.

To QSL (QSL reviews these rows):

| Id | Stated need |
| --- | --- |
| R-Q1 | An inconclusive `TerminalValue` with typed causes including `ReplayParity` and `ReplayRefused`, and a typed request index in `TerminalRecord` (merged in QSL `main`, 02530e7; QSL has ruled (relayed on IR-465, a QSL ruling recorded by the planner) that vacuity stays `Proved{0}` with no `NonZero` count, so option B of (a) is answered that way). `ReplayRefused` carries only QSL's catalogued codes (QSL's rule, as relayed). Decided by QSL (relayed on IR-465, a QSL ruling recorded by the planner), keyed on when a refusal happens. (a) Before Kani runs, when the obligation's own input is refused (nothing was proved): `Declined(ProofRefusalCause)`. The only CG producer of such a refusal is IR: its `Refused`, `InvalidInput` and `IncompleteInput` outcomes (join 1 to 2), which the IR-outcome rows of C-09 (FR-030) map. Those rows are outside the replay (counterexample) rows, so the replay rows stay about counterexamples only. QSL's example, "a refused caller lock", is QSL's and not a CG case: in CG a refused lock is `DependencyLockError` reached through `ReplayPackageError::Dependencies` (interface-001; FR-016-AC-16 to AC-19), only after Kani refuted, so it is case (b); IR checks the lock at checked-package admission, not as a `KaniOutcome`. Whether a second CG producer of (a) exists is for QSL to say only if it knows one. (b) After Kani refuted, when replay setup is refused on data by a QSL refusal that carries a code (for example `CallSiteRefusal` `Compile`, `ModelIntake`, `DependencyInput`, `Import`, `Dependency`, `UnknownFunction`, `UnknownOperation`, `UnknownClause`; `DependencyLockError::Input`, which is CG applying the same rule): `Inconclusive(ReplayRefused)` with that QSL code, the same as an E9 refusal; QSL has ruled (relayed) that `Declined` is only for a refusal before any backend run, and its FR-121 amendment is pending in QSL. `InvalidFunction` and `Name` carry no QSL code and are CG defects, `Failed`. CG has no `DependencyLockError::Duplicate` (FR-016-AC-24, built): a repeated dependency identity is refused by QSL's `DependencyInput::new` as `invalid_package` and arrives as `DependencyLockError::Input` (below). Faults stay `Failed`. CG's own refusals before Kani get no terminal value today, and are stated here, not decided: generation refusals (FR-015 refusals; an item whose claim is a `NoDerivableClaim` refusal settles `unsupported`, FR-015-AC-15; and a generation that ends in the `GenerationErrorCode::UnsupportedObligations` error, interface-001's terminal state `unsupported`, `oracle.rs`; each has no run, no artifact and no terminal value, FR-029 scope); `HarnessNotInCrate` from `execute_kani_obligation` (FR-022) is a refusal to run, and no requirement gives it a terminal value. The latter is open for CG with the driver; a driver-side defect that refuses to run is a candidate for `Failed`. The catalogued codes for the non-fault refusals are already in QSL `main`: `CallSiteRefusal::code()` and `DependencyInputRefusal::code()` are public. `TerminalValue::Inconclusive` with `ReplayParity` and `ReplayRefused(Code)` is merged in QSL. Repeated dependency identity (FR-029-AC-14, FR-030-AC-13). QSL ruled, relayed on IR-465 (a QSL ruling recorded by the planner): CG deletes its own duplicate pre-check in the lock admission and builds QSL's dependency input, so `DependencyInput::new` refuses `DuplicateIdentity` as `invalid_package` (cause `conflicting-definition`); it arrives as `DependencyLockError::Input` and settles by timing in QSL's terms: `Declined` before a backend run, `Inconclusive(ReplayRefused(invalid_package))` after a refutation. CG builds the lock only after a refutation, so for CG it is always `ReplayRefused`. A second copy of QSL's check in CG is rejected, and so is `Failed`, which would report an input defect as a fault. The builder behaviour is FR-016-AC-24 and is built. Because QSL checks libraries in supply order, a lock with several defects reports whichever QSL meets first, so the ACs cover a lock whose only defect is the repeated identity. The `Declined` code: `Declined { cause, code }` carries a `DeclineCode`, either a QSL catalog `Code` or a STD-001 registry code (QSL #634, merged; QSL records the registry and no issuer, and refuses no unregistered code). CG's map input, Contract IR's `KaniOutcome`, carries a `quire_contract_model::Std001Code` (IR-605, merged), which is not a QSL catalog `Code`; the map carries it unchanged as `DeclineCode::Std001`, built through the `Std001Code` that `qsl-replay` re-exports (FR-030-AC-2; see FR-030 Status). |
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
| R-I2 | Export cause-code constants or a typed cause enum: the Kani cause codes crossed as bare strings that consumers re-spelled. Met by IR FR-044 (`Std001Code` and its constants); the family lowerings' own codes are typed but unregistered `std001_code!` values that leave with the lowerings (IR-347). |
| R-I3 | Remove `pub use quire_contract_model::*` (`src/lib.rs:12`) together with CG adding a direct `quire-contract-model` dependency; CG imports model types through the glob. In IR-347's reopened scope. |
