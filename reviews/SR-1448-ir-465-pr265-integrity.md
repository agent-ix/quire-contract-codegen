---
id: "SR-1448"
title: "CG PR 265 spec review (integrity): per-variant totality of the replay-settlement readings"
type: SpecReview
analysis: integrity
review_set: subset
date: "2026-10-04"
scope: "agent-ix/quire-contract-codegen@fd54a90fd3066845aea60064cbe96d433097222d; spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/functional/FR-030-ir-outcome-terminal-map.md, spec/kani/matrix/TC-040-run-outcome-terminal-record.md, spec/kani/matrix/TC-041-ir-outcome-terminal-map.md, spec/kani/matrix/tests.md, spec/assurance/AD-003-evidence-chain.md, spec/assurance/AD-004-cg-crate-layout.md (diff origin/main...HEAD); CG error enums read at src/replay/function.rs and src/replay/frame.rs; QSL qsl-replay read at origin/main 3dc4f522"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AD-003
    type: references
---

# SR-1448: CG PR 265 spec review (integrity)

## Summary

Ticket: IR-465. Each variant of every CG replay error enum was checked against the readings.
Each must land in exactly one row, except the held `Duplicate`.

- `SpineReplayError`: `UnboundArgument`, `FieldDelimiter`, `Transcript` and `Identity` are CG
  defects and map to `Failed` (AC-11). `Refused` takes its refused or fault reading from the
  walked content.
- `ReplayPackageError`: `InvalidFunction` maps to `Failed`. `Dependencies` splits into `Input`,
  which is `ReplayRefused`, and `Duplicate`, which is held. `CallSite` is `Failed` when it is
  `Fault` and `ReplayRefused` otherwise (AC-13).
- `FrameReplayError`: `Dependencies` and `CallSite` follow the same split. `Name`, `Transcript`
  and `Envelope` map to `Failed`. `Refused` takes its reading from the walked content.
- `DependencyLockError`: `Input` maps to `ReplayRefused` and `Duplicate` is held.
- `EvidenceFailureCause`: `Decode` and `Domain` map to `Failed`. `Verdict` is the disagreement
  reading.

QSL's `code()` methods were checked. `CallSiteRefusal`, `DependencyInputRefusal`, `ReplayRefusal`,
`ReplayRequestRefusal`, `CompileRefusal`, `ImportRefusal` and `RunRefusal` have one.
`EmptyQualifiedName`, `InvalidIdentifier`, `MalformedTranscript` and `WitnessRefusal` have none.
So the CG-defect rows that wrap a QSL type (Transcript, Envelope, Name) carry no code, as the spec
says.

The before-versus-after rule is applied consistently. CG calls `call_site` and `replay` only after
Kani refuted. `Declined` arises only from IR `Refused`, `InvalidInput` and `IncompleteInput`
(FR-030-AC-2), and FR-030 Status says honestly that the code QSL's pending `Declined` will carry
is open. The grep of spec/ for leftover HELD, "Failed interim", "option B" and `NonZero` finds
only the intended `Duplicate` holds and the AD-003 section (a) text (SR-1447 FND-003).

## Verdict

Totality holds with one hole in the wording. The FR-029 Behaviour statement "every failure ...
that carries no QSL catalog code" also captures the held `Duplicate`, so two shall statements
conflict (FND-001, medium). There are also two low findings. Fix FND-001 before merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Two FR-029 shall statements conflict over `DependencyLockError::Duplicate`. The first: "The Kani adapter shall map to `Failed` every failure this repository raises that carries no QSL catalog code: ...". The Description defines a CG defect the same way: "a failure this repository raised that carries no QSL catalog code". `Duplicate` is raised by CG in `ReplayInputs::admit` before QSL sees the lock, and it carries no QSL value: that is the PR's own reason for holding it. So both of those statements require `Failed` for it, while the next statement says "shall not map `DependencyLockError::Duplicate` to any value until QSL rules". FR-030's "classify ... a CG-raised failure that carries no QSL code as FR-029 states" inherits the conflict. An implementer who follows "every" settles the held question as `Failed`. Fix: make the CG-defect class "other than `DependencyLockError::Duplicate`", or make the list after the colon the closed definition. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:44-52,129-139; spec/kani/functional/FR-030-ir-outcome-terminal-map.md:98-100 |
| FND-002 | low | The AD-003 link 7 failure row leaves out `SpineReplayError::Identity`. Its error column lists `SpineReplayError::{UnboundArgument, FieldDelimiter, Transcript, WrongArm}`, and its first group names no `Identity`. FR-029-AC-11, the FR-029 Behaviour and AD-004 step 5 (edited in this PR to add `Identity`) all classify `Identity` as a CG defect that maps to `Failed`. The row was rewritten by this PR, so it is in scope. Fix: add `Identity` to the error column and to the first group. | spec/assurance/AD-003-evidence-chain.md:77 |
| FND-003 | low | AD-003 attributes the `DependencyLockError::Input` extension to QSL. The relayed QSL answer on IR-465 covers "a non-fault CallSiteRefusal" only. Link 7 says "Decided by QSL (...): it maps to `Inconclusive(ReplayRefused)` with the QSL catalog code from `CallSiteRefusal::code()` or `DependencyInputRefusal::code()`". R-Q1 (b), under "Decided by QSL", lists `DependencyLockError::Input` as a QSL-decided case. FR-029 Status states it honestly as CG's application of the rule ("The same rule covers `DependencyLockError::Input`"). The extension is sound: the same `DependencyInputRefusal` reaches the map as `CallSiteRefusal::DependencyInput` when QSL builds the input. Only the attribution is wrong. Fix: say in AD-003 that `Input` follows by CG's application of QSL's timing rule. | spec/assurance/AD-003-evidence-chain.md:77,379 |

## Dispositions

Round 1, reviewed at 808a3322ce2abf78ae32b00178a13cca51f59e29 (fix commit 808a332 on top of fd54a90). CG error enums re-read at that head.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 808a3322ce2abf78ae32b00178a13cca51f59e29: the FR-029 CG-defect reading is now "exactly the errors FR-029-AC-11 lists ... excluding the held `DependencyLockError::Duplicate`". The Failed bullet says "exactly these failures" and names `Duplicate` as the one CG-raised failure outside the list. The FR-030 table row and the classify bullet say the same. No statement now sweeps `Duplicate` into `Failed`. Partition re-checked against src: the CG-origin failures with no QSL value are `SpineReplayError::{UnboundArgument, FieldDelimiter, Transcript, WrongArm, Identity}`, `FrameReplayError::{Transcript, Envelope, Name}`, `ReplayPackageError::InvalidFunction`, `EvidenceFailureCause::{Domain, Decode}` (the out-of-bound playback and the decode failure) and `DependencyLockError::Duplicate`. That is the AC-11 list plus `Duplicate`. The other variants wrap a QSL value (`Refused`, `CallSite`, `Dependencies(Input)`, `Verdict`) and have their own readings. |
| FND-002 | fixed | 808a3322ce2abf78ae32b00178a13cca51f59e29: the AD-003 link 7 error column and the first group now include `SpineReplayError::Identity`. |
| FND-003 | fixed | 808a3322ce2abf78ae32b00178a13cca51f59e29: AD-003 link 7 says "Decided by QSL for the call-site refusal ...; `DependencyLockError::Input` is CG applying the same rule". R-Q1 (b) and AD-004 step 5 add "which is CG applying the same rule". |
