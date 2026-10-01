---
id: AD-002
title: "CG to QSL replay seam: call_site, replay, replay_frame and the counterexample envelope"
type: ArchitectureDescription
status: proposed
owner: codegen-maintainers
system: quire-contract-codegen replay adapter (spine_replay, frame_replay, kani_witness_join) and the qsl-replay facade it calls
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-001
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AD-003
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-011
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-013
    type: references
---
# CG to QSL replay seam

The Replay subsystem row of the Subsystem Registry lists this AD beside AD-001. It is a view of
one boundary of AD-001's "CG to QSL" seam rows, not a second architecture, and it is one of the
seam descriptions of IR-324. The evidence chain that joins this seam to IR and to the terminal
value is [AD-003](AD-003-evidence-chain.md).

## System Boundary

A Kani falsification becomes evidence only if QSL's native evaluation reproduces it. This AD
states what CG hands QSL for that, what QSL returns, and who is responsible for each part. It
covers `qsl_replay::call_site`, `replay`, `replay_frame`, the backend-witness transcript, the
counterexample envelope and the replay request.

Out of scope: how Kani output is read (CG's transcript parser), QSL's evaluation, the terminal
value of a run (AD-003) and the driver.

## Views

The seam is described as what crosses it, how identity is asserted, which way dependencies point
and who reports each failure.

### What crosses the seam

| Item | Direction | Owner of the type | Who builds / reads |
| --- | --- | --- | --- |
| `call_site::<FunctionSite \| OperationSite \| ClauseSite>` result: `package_id`, lowered `quire.checked-package/v2` bytes, parameter, anchor, frame and clause node ids and occurrence keys | QSL to CG | QSL (`CallSiteSelection` is sealed) | QSL computes every node id; CG never derives one |
| Source bytes, dependency libraries (`SuppliedLibrary`, `DependencyInput`), domain package documents | CG to QSL | QSL types; bytes are the proving run's | CG supplies them from the proved lock |
| Backend-witness transcript, admitted by `Witness::parse` | CG to QSL | QSL grammar | CG renders it from decoded values, never from Kani text |
| `ReplayRequestWire` (and `ReplaySource::{Witness, Input}`) | CG to QSL | QSL | CG fills it; `replay` reads it |
| `WitnessEnvelope` / `WitnessPacket` (frame counterexamples) | CG to QSL | QSL | CG builds through `WitnessEnvelope::reconstruct` |
| `ObligationIdentity` (32 bytes), in the envelope and in the request's obligation-identity slot | CG to QSL | QSL type; CG is to mint the value | see AD-003 |
| `ReplayResult`, `FrameReplayResult`, `WitnessSettlement`, `ProofCategory` | QSL to CG | QSL | CG reads; CG defines its own verdict only as a partition of QSL's (`ReplayVerdict`) |
| `DeclaredDomain(ProofBound{DomainKey, FiniteBound})` | CG to QSL (in the envelope) | QSL (re-exported by `qsl-replay`) | CG is to build it from its argument bindings' bounds |

Nothing else crosses. CG holds no copy of an envelope, witness, replay source, terminal record or
obligation identity type, and neither does IR. CG's only QSL dependency is the `qsl-replay` crate
(the one QSL entry in `Cargo.toml`).

### Versioned contracts and how identity is asserted

| Contract | Spelled by | Asserted by |
| --- | --- | --- |
| `qsl-replay` Rust API | QSL | The compiler. CG's `Cargo.toml` names QSL's `main` branch and `Cargo.lock` selects the revision it builds against; the seam asserts nothing about that revision beyond compilation, and no digest or version check is needed: a changed type or removed function fails CG's build. |
| Request `contract_version` `quire.native-runtime/v1`, `capability_vocabulary` `quire.capability-kind/v1`, `package_contract_version` `quire.checked-package/v2` | QSL's reader (private constants in `request.rs`) | QSL refuses a spelling it does not know. CG writes the literals itself (see Current state). |
| `package_id` (`quire.package.semantic/v2`) | QSL | QSL recompiles the provided source and requires the recompiled `package_id` to equal the request's (`execute.rs`, "Rule 5"; `execute/frame.rs` for the frame path). This is the one canonical content-identity digest the whole replay seam rests on: it binds a replay to the content that was proved. |
| Source bytes | CG computes the digest with QSL's `ByteDigest` | QSL checks each provided byte against its digest (`stale_dependency/byte-digest-mismatch`). |
| Request obligation-identity slot (`originating_counterexample_identity` today) | CG fills it; today with the `ByteDigest` of the transcript | QSL does not check it against any content: `replay` passes it into witness decoding as a label (`execute.rs`, `arguments`). R-Q7 retires the transcript digest in favour of the obligation identity. |
| Backend identity | request `backend` member | carried; QSL records it, does not interpret it |

The obligation identity is the other content identity on the replay path; it is discussed in
AD-003 and is the only digest CG mints for a proof. No pin, SHA or version record is asserted
anywhere on this seam, and none is proposed.

### Dependency direction

- IR never depends on QSL and defines no replay type (IR FR-037, IR AD-001). IR has no `replay`
  or `witness` module and no QSL dependency.
- CG depends on `qsl-replay` and on IR; `qsl-replay` depends on the model crate through
  `qsl-package`. No cycle.
- CG calls `qsl_replay::replay` and `replay_frame` only. No path takes a caller-supplied
  executor.
- The replay verdict is QSL's evaluation. CG supplies no value that decides it.

### Failure outcomes and who reports them

| Condition | Reporter | Outcome |
| --- | --- | --- |
| A harness argument has no bound parameter, delimiter in harness or check text, transcript not admitted | CG (`SpineReplayError`) | typed refusal, no replay |
| Dependency selections not admitted, call site not located (`CallSiteRefusal`, a fault among them) | CG wrapping QSL (`ReplayPackageError`, `FrameReplayError`) | typed refusal, no replay; a wrapped fault is still a fault (AD-003, link 7) |
| A qualified name built from the operation's identifiers is not admitted (`FrameReplayError::Name`) | CG | typed refusal, no replay |
| The frame witness transcript or the envelope CG built is not admitted (`FrameReplayError::Transcript`, `FrameReplayError::Envelope`, from `WitnessEnvelope::reconstruct`) | CG | typed refusal, no replay; CG built a value its own contract says QSL admits, so AD-003 maps it to a failure |
| `replay` / `replay_frame` refuses the request or envelope | QSL (`ReplayRefusal`) | CG carries it unchanged in `Refused`; it is not a verdict on the evidence |
| Decoded value outside its declared domain (function path) | CG (`EvidenceFailureCause::Domain`) before any request is built | evidence failure; AD-003 link 7 maps it to a failure value, since it is CG's own defect and not a `ReplayRefusal` |
| Replay ran and did not settle `ReproducedWithEvaluatedWitness` in category `violation` | QSL settles; CG partitions (`verdict_of`) | evidence failure with QSL's settlement and category |
| A witness-sourced request settles on the input arm | CG (`WrongArm`) | typed refusal |

A counterexample that is not reproduced is never a clause success or a clause failure: it is
evidence failure whatever the cause. An input-arm settlement (`reproduced-without-witness`) is
never backend evidence.

## Decisions

CG builds what QSL reads and QSL decides the verdict; each statement below is one a test can
check.

### Invariants a test can check

Candidate statements (local labels; the repository assigns requirement ids when one is
authored).

- R-1. CG imports no QSL item except from `qsl-replay`, and no CG or IR source defines a
  `Witness`, `ReplaySource`, `WitnessEnvelope`, `TerminalValue` or `ObligationIdentity`.
  Test: cargo metadata on CG's normal dependencies, and a grep gate on type definitions.
- R-2. Every parameter, anchor, frame and clause node id CG puts in a request or envelope is the
  value `call_site` returned. Test: a mutation that rewrites an id turns the replay red.
- R-3. A transcript is built by one function and admitted only through `Witness::parse`.
- R-4. A value outside its declared domain yields evidence failure before `replay` is called
  (test: a stand-in that panics if called).
- R-5. Only a `ReproducedWithEvaluatedWitness` settlement in category `violation` reproduces;
  every other settlement, including `Inconclusive` in any category, is evidence failure.
- R-6. Every envelope and every request CG builds carries, as its obligation identity, a QSL `ObligationIdentity` computed by CG from the
  obligation's identity members (blocked on AD-003, E-1).
- R-7. A request names the `package_id` returned by `call_site` for the same source, and the
  replay is refused (not reproduced) when one source byte changes.
- R-8. The contract spellings in a request are QSL's, not CG's own literals (blocked on a QSL
  export, see Routed gaps).

## Risks

What is measured today, what is open and with whom, and what is routed. Measured on this
repository at the IR-321 subsystem layout, against the `qsl-replay` crate CG's lock selects.

### Current state and gaps

- Function path: `spine_replay.rs` `ReplayPackage::new` calls `qsl_replay::call_site` with a
  `FunctionSite` (`spine_replay.rs:400`); `replay_falsification` calls `replay` (:125);
  `replay_counterexample` decodes, domain-checks (:498, through `first_out_of_domain`) and
  partitions (`verdict_of`, :517). This path builds no `WitnessEnvelope`: it sends
  `ReplaySource::Witness` in the request. The request has an obligation-identity slot (QSL's
  reader names it `originating_counterexample_identity` and types it `ObligationIdentity`), and
  CG fills it with the `ByteDigest` of the transcript (`spine_replay.rs:440`, `:357`): the slot
  exists and holds the wrong value, not the obligation identity. QSL's review says the slot is
  to be renamed `obligation_identity`; that is QSL's change and is not assumed here.
- Frame path: `frame_replay.rs` calls `call_site` with an `OperationSite` (:115), builds the
  envelope with `WitnessEnvelope::reconstruct` (:186) and calls `replay_frame` (:187).
  `obligation_identity` is a caller-supplied `[u8; 32]` field (:47); the only value in the
  repository is `[1; 32]` in a test (`tests/state_frame_support/native_twin.rs:406`). No code
  computes it.
- No domain check precedes the frame path, and the frame envelope carries
  `declared_domains: Some(Vec::new())` (`frame_replay.rs:169`): the proof bound is not in the
  envelope even though QSL's `DeclaredDomain` is buildable through the facade. QSL's review
  says an empty declared-domain list will be refused once its QSL-345 part 2 lands; CG's frame
  path must then supply the bounds.
- The contract spellings are written as literals in `spine_replay.rs:345-352`; the same string
  for `quire.capability-kind/v1` exists as a constant at `capability.rs:22` and is not used
  there. QSL keeps its own spellings private. QSL's review says QSL removes the three version
  and vocabulary members from the request (QSL-352), after which CG drops its copies; until
  then CG's literals are required.
- FR-024 requires the envelope for every replay and a domain check before it; only the frame
  path builds an envelope. The function path is therefore either outside FR-024 or short of it.
  This AD records the gap; it does not decide it. FR-024 is `Planned` in the replay matrix.
- `first_out_of_domain` is crate-private; `decode_falsification` is public
  (`kani_witness_join.rs:138`) and decodes to `qsl_replay::WitnessValue`.
- QSL's `call_site` accepts a `ClauseSite` selection and QSL has a state-clause replay entry;
  CG has no clause-replay entry in `src`. Not a gap against this AD; recorded so nobody assumes
  it.
- AD-001's Replay view and Current state describe the same function-path facts. AD-001 is not
  edited here.

### Open questions

| Question | Owner | Recommendation | Cost of the alternative |
| --- | --- | --- | --- |
| What does the function path put in the request's obligation-identity slot, and does it also build a `WitnessEnvelope`, as FR-024 says? | CG | Put the obligation identity (AD-003, E-1) in the slot instead of the transcript digest. QSL's review says no separate envelope is wanted for the function path; if so, amend FR-024 to scope the envelope to frame and state-clause counterexamples. Decide in the CG spec PR. | Today the function path's request carries no obligation join, so a counterexample cannot be tied to the harness that produced it. |
| Request contract spellings: QSL removes them (QSL-352) | QSL | CG drops its literals when that lands. | A second spelling of a QSL contract is a copy that drifts. |
| `call_site` selection for state clauses (`ClauseSite`) has no CG consumer | CG | Leave until a state-clause harness needs it. | none now |
| The domain check before replay exists on the function path only | CG with QSL | CG keeps its own pre-check: a playback outside the harness's proof bound is a CG harness defect, and a QSL-side check of admitted values against `DeclaredDomain`, if QSL adds one, would report it as an invalid input and hide the defect. Do not drop the CG check on QSL's account. | Without CG's check a QSL refusal would hide a CG defect. |

No compatibility layer is proposed or needed. If QSL changes the facade, CG changes its calls.

### Routed gaps

Needs stated to owners, not decisions. Ids are the routing ids of IR-324; they are not
requirement ids.

To QSL (QSL reviews these rows):

| Id | Stated need |
| --- | --- |
| R-Q5 | QSL removes the three version and vocabulary members from the request (QSL-352); CG drops its hand-written copies. |
| R-Q7 | Rename the request's `originating_counterexample_identity` to `obligation_identity` and have it hold the obligation-identity digest; no separate envelope for the function path (QSL's review, to be confirmed by QSL). CG then fills the slot with its obligation identity. |
| R-Q8 | CG's frame envelope sends an empty declared-domain list, which QSL-345 part 2 will refuse; CG must supply the bounds. CG keeps its own pre-check regardless. |
