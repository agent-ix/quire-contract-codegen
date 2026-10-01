---
id: "SR-670"
title: "CG PR 214 spec review: AD-002 replay seam and AD-003 evidence chain"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@80243027bcda41f1d81e83718bd29738926b6972; spec/assurance/AD-002-cg-qsl-replay-seam.md, spec/assurance/AD-003-evidence-chain.md, spec/spec.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-002
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AD-003
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AD-001
    type: references
---

# SR-670: CG PR 214 spec review

## Summary

Ticket: IR-324. PR: agent-ix/quire-contract-codegen#214 at 8024302, base main 2fad745. The PR
changes spec files only. `src/`, `tests/`, `Cargo.toml` and `Cargo.lock` are byte-identical
to origin/main, so every code claim was checked against origin/main.

This review covers integrity, ArchitectureDescription structure, traceability, and
consistency with the code, with AD-001, and with the sibling seam ADs (CG #215 AD-004, IR #239
and #241, driver #11 AD-001, RT #92 AD-003). QSL claims were checked read-only at QSL
origin/main d81193f and again at a28a557, the `qsl-replay` revision CG's `Cargo.lock` selects
(`Cargo.lock:1313`). Both revisions agree on every point checked.

Re-verified as stated:

- `spine_replay.rs` :125 `replay`, :345-352 literals, :357 and :440 transcript `ByteDigest`
  into `originating_counterexample_identity`, :400 `call_site`, :498 `first_out_of_domain`,
  :517 `verdict_of`.
- `frame_replay.rs` :47, :115, :169, :186 and :187.
- `native_twin.rs:406` `[1; 32]`, `capability.rs:22`, `kani_witness_join.rs:138`.
- `kani_execution.rs:676`, the precondition exemption. The `classify_kani_run` doc says this
  too, and no count is carried on `KaniExecutionEvidence` (:344-361).
- No `TerminalValue`, `TerminalRecord`, `ObligationIdentity` definition or `ToolPin` in CG
  `src`. The one QSL dependency is `qsl-replay`.
- IR `src/lib.rs:12` glob, `src/kani/mod.rs:23`, and `provider_result`'s "implemented here".
- QSL `TerminalValue` has 7 variants and `InconclusiveCause` has `KaniVacuousProof` only.
- `ReplayRefusal::Fault(InternalFault)` is at `execute.rs:225`, and Rule 5 is at `execute.rs:678`.
- The request slot is typed `ObligationIdentity`, and the private contract constants are in
  `request.rs`.
- ADR-013 O-09, O-16 (including the "IR implements the map" sentence at :416), O-24, O-27 and
  C-09. ADR-011 FB-11 and OBS-040. QSpec FR-331-AC-8.

The head moved during the review, from d8d55ba to 8024302, which adds the encoder-gap bullet,
the E-1 encoder clause and the R-Q2 export ask. Those cites were re-verified at origin/main:

- `kani.rs:1045-1046` and `oracle.rs:1111-1112` (`serde_json::to_vec` plus a newline);
- `bounded_kani_corpus.rs:340-342` (`ByteDigest::of`);
- `sha2` is a dev-dependency only (`Cargo.toml:35`), and `src` has no `quire_canonical` use;
- QSL `qsl-replay` depends on `quire-canonical` but re-exports no canonical digest at main;
- CG #215 step 1a exists.

The earlier findings carry over unchanged, with line numbers moved by the inserted bullet.

`make spec` with TRUSTED_HOME set to the scratchpad home exits 0 with the 3 baseline warnings
(FR-017:137, FR-014:278 twice). The PR adds none.

Confidentiality: neither AD names a quire-research branch, path or SHA, and neither holds a
roadmap. Cross-repo items are cited by public ticket id. Neither AD adds a pin, SHA or version
record. The obligation identity is the only digest minted, and the others are listed as
existing (see FND-009).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AD-003 calls the terminal map "total over the replay result, which is one of three" (settled, refused, fault). A falsified run can also end with no QSL replay result, because CG stops it before `replay`. Examples: decode failure or an out-of-domain value (`EvidenceFailureCause::{Decode, Domain}`); `SpineReplayError::{UnboundArgument, FieldDelimiter, Transcript, WrongArm}`; `ReplayPackageError` (including a wrapped `CallSiteRefusal::Fault`); `FrameReplayError::{Name, Transcript, Envelope}`. AD-002's failure table lists these, but AD-003 gives them no terminal value. So E-3 ("every run item has exactly one terminal value") and the "total" claim are not true as written. Needed: a fourth arm (CG-side refusal or fault before replay) with its value, or a routed question | spec/assurance/AD-003-evidence-chain.md:59, :76, :122-125; spec/assurance/AD-002-cg-qsl-replay-seam.md:87-94 |
| FND-002 | medium | "CG separates (2) from (3) by variant" catches only the top-level `ReplayRefusal::Fault`. Two more faults are nested. `ReplayRefusal::Admission(AdmissionFailure::Fault(_))` maps to `Code::RuntimeInvariant`, the same code as `Fault` (QSL `execute.rs:222`, `:252`, at d81193f and a28a557). `CallSiteRefusal::Fault(InternalFault)` (`call_site.rs:215`) reaches CG through `ReplayPackageError::CallSite`. A split by top-level variant sends an admission fault to `Inconclusive(ReplayRefused)`. That is the hidden defect the fault arm exists to prevent ("so a defect stays loud"). Needed: say that every internal fault maps to `Failed`, whatever variant carries it, for example by its `RuntimeInvariant` code or by explicit nested arms | spec/assurance/AD-003-evidence-chain.md:59 |
| FND-003 | medium | The E-1 gap understates the work. AD-003 says each identity struct "must gain" the occurrence key. But ADR-013 O-09 (:280) also needs the clause or application **node id** and, for each argument, its **parameter node id** and declared domain. `KaniObligationIdentity` holds a `ClauseRef` (requirement plus clause id, not a node id). `ObligationBinding` and `ScalarObligationArgument` hold no parameter node id (`kani_obligations.rs:440-451`, `:468-494`, `:513-520`). E-1 also says "the scalar and the V1 obligation paths both use it" and omits the frame path. `StateFrameIdentity` (`state_frame.rs:204-226`) is a third identity struct. The frame path is the only one that puts an obligation identity in an envelope today (`frame_replay.rs:47`, `:160`). PR body R-Q2 repeats the occurrence-key-only wording | spec/assurance/AD-003-evidence-chain.md:115-119, :240-242 |
| FND-004 | medium | The tool pin is left out of the AD. AD-003 says no digest over a tool is proposed and "the Kani version is not pinned" (:92-94). It also says CG "can build" `TerminalRecord` and `BackendProviderSource` (:256-257). But `BackendProviderSource` has `pub tool_pin: String` (QSL `proof_result.rs:283`), `ProofResultEnvelope` carries a `ToolPin` (:205-209), and ADR-013 O-24 (:834) says the envelope carries "the tool pin". So the E-3 path the AD recommends needs a tool pin that the AD says nobody proposes. PR body F3 says "the ADs make no claim", but AD-003's E-3 text does make one. Needed: route the question to QSL as an R-Q row with a stated need (drop `tool_pin`, or define it as the app version a report names), without deciding it here | spec/assurance/AD-003-evidence-chain.md:92-94, :256-259 |
| FND-005 | medium | AD-003 does not match the driver's ownership ruling. Driver #11 AD-001 (:41-43, :152-158) says the driver owns execute, then replay, then the terminal record: it calls CG's map and the `qsl-replay` facade. AD-003 instead frames building the `TerminalRecord` as a CG gap ("no CG code builds a `TerminalRecord`"; "CG can build both"). It says "which component writes the `results` wire is not established here", and AD-002 lists "the driver" only as out of scope. R-Q4 also diverges: the PR #214 body withdraws it, while driver #11 AD-001:183 still routes "R-Q4: a writer for the FR-331 `results`". Needed: cite the driver AD for the terminal-record step, keep CG's ownership to the C-09 map, and reconcile R-Q4 across both PRs | spec/assurance/AD-003-evidence-chain.md:61, :77, :256-259; spec/assurance/AD-002-cg-qsl-replay-seam.md:36-37 |
| FND-006 | medium | AD-001's Decisions conflict with AD-003, and no AD records it. AD-001:160-161 says "CG builds QSL's `ObligationIdentity` from every `KaniObligationIdentity` member except `source_span`". That set includes oracles, symbols, solver, unwind and options. AD-003 E-1 and its open question set the preimage by the O-09 member list, "not by struct name". AD-002:166 mentions only AD-001's Current state, which is stale separately: AD-001:195-197 says no domain check and no `WitnessEnvelope`, but `spine_replay.rs:498` checks the domain and `frame_replay.rs:186` builds the envelope. The PR body says that will be fixed separately. Needed: name the AD-001 Decisions conflict in AD-003 so the AD-001 refresh also covers line 160 | spec/assurance/AD-003-evidence-chain.md:115-119, :286; spec/assurance/AD-001-codegen-architecture.md:160-161 |
| FND-007 | low | E-3 cites "FR-029-AC-1 and FR-030-AC-7 are the existing form" of a single `match` with no wildcard arm. FR-029-AC-1 is instead the mapping AC "`falsified` maps to `Refuted`" (FR-029:82). That unconditional mapping is exactly what E-3's replay rule contradicts. For FR-029, the no-wildcard rule is in its statement (:48) and in TC-040 | spec/assurance/AD-003-evidence-chain.md:122-124 |
| FND-008 | low | One row reads "Source bytes, counterexample identity: QSL checks each provided byte against its digest". QSL does not check the counterexample identity. `replay` passes it through only as the obligation label of `ReplayRefusal::Witness` (QSL `execute.rs:476`; `execute/tests.rs:264-271`) | spec/assurance/AD-002-cg-qsl-replay-seam.md:68 |
| FND-009 | low | AD-003 lists the transcript `ByteDigest` ("counterexample identity") among existing digests that "bind content". AD-002 and R-Q7 say this value sits in the slot in error and will be replaced by the obligation identity. After R-Q7 the digest has no role. Under the repository's digest rule it should be marked for removal, not listed as a legitimate digest | spec/assurance/AD-003-evidence-chain.md:88-91; spec/assurance/AD-002-cg-qsl-replay-seam.md:139-142 |
| FND-010 | low | "CG tracks QSL's main branch; there is no revision or digest assertion". This reads as if no revision is selected, but `Cargo.lock:1313` records one (a28a557) and the gates run `--locked`. The AD itself measures "against the `qsl-replay` crate CG's lock selects" (:130). Say the lock selects a revision as ordinary dependency resolution and that it guards nothing beyond the build | spec/assurance/AD-002-cg-qsl-replay-seam.md:65 |
| FND-011 | low | `git merge-tree` of 8024302 with CG #215 (bfaaa84) reports a content conflict in `spec/spec.md` under `## References`, because both PRs insert at the same line. The fix is trivial: keep both inserted lines. The registry rows do not conflict | spec/spec.md:105-109 |
| FND-012 | low | "`kani_solver_absent` and `kani_backend_absent` ... so CG spells them itself". No CG source spells either one. They appear only in FR-030 and TC-041, because the map is not built. Say "CG's spec spells them" | spec/assurance/AD-003-evidence-chain.md:97-100 |
| FND-013 | low | The encoder-gap bullet (added at 8024302) says "The one content digest CG mints in `src` is the corpus case's". CG `src` also mints source-byte `DigestRecord`s with `ByteDigest::of` (`spine_replay.rs:149-152`) and the transcript digest (`spine_replay.rs:440`). AD-003:88-91 lists both. The claim holds only for digests over a JSON encoding. Say "the one digest CG mints over a JSON encoding" | spec/assurance/AD-003-evidence-chain.md:244-246 |
| FND-014 | medium | The new encoder text disagrees with CG #215 (AD-004 at bfaaa84). AD-003 R-Q2 asks QSL to export "a `ContentDigest` wrapping `ByteDigest`" through `qsl-replay`. E-1 requires `quire_canonical` "never `serde_json`", and the AD names "the CG layout AD's step 1a" as the fix. But AD-004 defines `ContentDigest` as a CG type in `core/identity.rs`, "built only by `core::canonical`". In AD-004, `core/canonical.rs` keeps `deterministic_json` (the `serde_json::to_vec` encoder) as the one helper, and step 1a merges the two `deterministic_json` copies with no RFC 8785 encoder (AD-004 :349-358, :443-445). The two PRs disagree on who owns `ContentDigest` (QSL export or CG newtype) and which encoder backs it. Needed: align both ADs before either merges. One owner for `ContentDigest`, and step 1a states that it calls `quire_canonical` through the facade | spec/assurance/AD-003-evidence-chain.md:118, :243-255, :304 |

## Verdict

These are structurally sound ArchitectureDescriptions: System Boundary, Views, Decisions and
Risks are in place, `quire validate` passes, and the registry rows and References are correct.
Almost every measured file:line claim reproduces. The answers to owner questions (a), (b) and
(c) are well argued and stay within the brief. C-09 belongs to CG, unwind exhaustion is
intended per O-16, cover-unsatisfied is a CG approximation, a satisfied precondition cover
counts as one SUCCESS check, and CG owns `terminal_value`. The relayed QSL items are labelled
as relayed, not yet in QSL main (QSL-351, QSL-352, QSL-354, QSL-345, QSL-342). No requirement
ids are minted.

Not mergeable yet. Fix the medium findings first: the terminal-map totality (FND-001, FND-002),
the full E-1 gap (FND-003), the unrouted tool pin (FND-004), consistency with the driver
ruling and AD-001 (FND-005, FND-006), and the `ContentDigest` and encoder conflict with CG #215
(FND-014). The PR is also still a GitHub draft. Its own body says
it is not ready until the R-Q rows are reviewed and R-S1 to R-S8 are routed.
