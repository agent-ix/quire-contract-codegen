---
id: SR-580001
title: "IR-595 PR 580 checked-closure facade re-point base spec review"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@08c68e3ac71b47e1fedae1c017891ac150053a89; spec/oracle/functional/FR-021-function-application-oracles.md, spec/assurance/AD-004-cg-crate-layout.md, spec/oracle/matrix/TC-031-function-application-oracles.md"
---

## Summary

Ticket: IR-595. PR: quire-contract-codegen#580 (draft), frozen head 08c68e3a over base
e8bde645 (CG main at review time). The diff re-points FR-021-AC-32, AC-35, AC-36 and AC-37,
one new FR-021 Behavior bullet, AD-004's external evaluation seam paragraph and TC-031 step 12b
to the concrete checked-closure facade in draft QSL PR #1483 (quire-spec-language, head
6ee417f64d40d13b59676c544e8659597a31f7aa, `qsl-replay/src/application.rs` and
`qsl-replay/src/lib.rs`). Checked against QSL main FR-485 (merged in #1470), the #1483
public API, and CG main code.

## Examined scope

- API names and arguments against #1483: examined. `acquire_checked_closure` takes the seven
  inputs (source, path, bytes, packages, dependencies, limits, replay_limits) and returns
  `Result<CheckedClosureCompilation, AdmittedRefusal>`. `CheckedClosureCompilation` has
  `package()`, `context()`, `handle()` and `into_parts()`. `CheckedClosureHandle` has
  `package_id()`, `lookup_callable(owner_package_id, wire_node_id)` returning
  `Result<CallableSelection, CallableLookupRefusal>`, and `registry()`. `CallableSelection`
  has `owner_package_id()`, `key()` and `signature()`. `CallableLookupRefusal` is `Unknown`,
  `Foreign(DigestRecord)` and `WrongKind`. `ApplicationRegistry::register(&selection,
  &signature, factory)` returns `Result<Admission, SelectionRefusal>`. `begin_call(&selection,
  Vec<Value>, &ApplicationEnvironment, &mut Meter)` returns
  `Result<CheckedApplicationSession, ApplicationBeginRefusal>`.
  `ApplicationEnvironment::new(ObjectClosure)` exists, as do `SelectionRefusal::WrongAcquisition`,
  `ApplicationBeginRefusal::{WrongAcquisition, Call}`, `step`, `run` and `is_sealed`. Every
  name, argument set and refusal the diff cites matches #1483 exactly.
- FR-021-AC-32 against main: examined. It is strengthened. It names the facade entries and
  adds that `qsl_replay::replay` stays the separate Kani witness entry. It does not narrow
  main. The pending gate moves from IR-1007 to QSL #1483 (FND-003).
- FR-021-AC-35 against main: examined. It is strengthened: the entry is named, the returned
  parts are named, and acquisition failure is added. "Once for the generated run" makes main's
  "once" precise without narrowing it, because TC-031 already acquires separately for the
  direct run. The new failure clause has no TC step (SR-580002).
- FR-021-AC-36 against main: examined. Main's "QSL lookup must admit the callable before CG
  registers a body" is kept as the explicit lookup, register and `begin_call` sequence, and
  "a raw key alone is never authority" is added. The mapping text is unchanged.
- FR-021-AC-37 against main: examined. The same two mismatch classes as main. The
  lookup-refusal clause now names `CallableLookupRefusal`, and wrong-acquisition refusals are
  added. FND-002, plus EARS wording in SR-580003.
- New FR-021 Behavior bullet ("For a matching `CallableSelection` ..."): examined. It matches
  FR-485 S4-S7 and #1483.
- AD-004 external evaluation seam paragraph: examined. FND-001.
- TC-031 step 12b: examined. It matches the #1483 API.
- Truthfulness of the planned cutover: examined. `src/oracle/function/mod.rs:155` still has
  `use quire_contract_runtime::exact as rt;`. `Cargo.toml:38` keeps the RT `exact` feature.
  `src/replay/function.rs` sends a decoded Kani falsification to `qsl_replay::replay`
  (module doc, lines 1-9). AD-004's "The current concrete function oracle still imports RT's
  `exact` port" and "`replay` module keeps using `qsl_replay::replay` for Kani witness
  settlement" are both accurate. Everything stays PLANNED.
- No duplicate evaluator, no adapter or compatibility layer: examined. The diff forbids
  expression dispatch, a second evaluator and an evaluator copy. It keeps main's
  alias/wrapper/compatibility-layer ban. CG imports from QSL only `qsl_replay`.
- Drift containment: examined. The #1483 API names now appear in four ACs, one Behavior bullet,
  AD-004 and TC-031. All are PLANNED and gated on #1483 merging, and the author has committed
  to a recheck after merge. A rename at merge is a bounded spec-only edit in three files.
  Drift is contained. FND-003 is about how the gate is named.
- FR-485 (QSL main): context only.

## Verdict

CHANGES REQUESTED. The API transcription is exact. No AC is narrowed. The description of the
current RT-exact import and the separate `qsl_replay::replay` role is truthful, and every
re-pointed criterion stays PLANNED. Two medium gaps remain. First, AD-004 leaves out the
direct `quire-eval` dependency that the facade forces. Second, the selection half of AC-37
cannot be triggered through #1483, because the selection echoes CG's own lookup inputs.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new path names `quire_eval::BodyFactory`, `Admission::Accepted`, `Signature`, `Evaluation` and `CallFailure`. FR-485 S4 and AC-8 forbid `qsl_replay` from re-exporting leaf types, and #1483's CG-shaped consumer fixture depends directly on `quire-eval`, `quire-exact` and `quire-semantic-value`. CG's `Cargo.toml` has no `quire-eval` dependency today. AD-004's seam paragraph says only that the leaf "provides the evaluation interface". It never says CG (and the emitted oracle crate, if it hosts the `BodyFactory`) adds a direct `quire-eval` dependency beside `qsl-replay`. AC-32 says it closes on inspecting "dependencies" but gives no expected dependency set. | spec/assurance/AD-004-cg-crate-layout.md:359-375; Cargo.toml:38-42 |
| FND-002 | medium | In #1483, `lookup_callable` builds the `CallableSelection` from the caller's own `owner_package_id` argument, and returns a `key` only when `key.as_bytes() == wire_node_id.as_bytes()` (application.rs:79-131). So after a successful lookup, the AC-37 comparison of `selection.owner_package_id()` and the key bytes with the mapped request ids is equal by construction. Through the public CG API only the `handle.package_id()` branch can fail. TC-031's "separately select a different admitted callable in A's checked closure" needs a test seam that injects a selection, and neither AC-37 nor TC-031 specifies one. As written, the selection branch cannot fail. This is inherited from main's wording, but this PR is what binds it to the concrete API. | spec/oracle/functional/FR-021-function-application-oracles.md:331; spec/oracle/matrix/TC-031-function-application-oracles.md:173-182 |
| FND-003 | low | The pending gate in AC-32/35/36/37, the Status paragraph and TC-031 12b changed from "IR-1007/IR-595" to "QSL #1483 and IR-595". That keys the gate on one draft PR number rather than the owning ticket or FR-485 delivery. If #1483 is closed or superseded, the gate names a dead PR. Naming IR-1007 (with #1483 as the current candidate in prose) keeps the gate stable. | spec/oracle/functional/FR-021-function-application-oracles.md:326-331 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | TC-031 12b exercises the selection-identity guard with 'a different admitted callable in A's same acquired closure'. That callable may be owned by the unit, which tests only the key-byte comparison. AC-37 also requires the owner comparison (`CallableSelection::owner_package_id()` against the declaring package). No vector requires a genuine second selection from an imported dependency's owner, so dropping the owner comparison from the guard survives the corpus. | spec/oracle/matrix/TC-031-function-application-oracles.md:183-188 |
| FND-005 | low | AD-004's seam paragraph still keys the precondition on the PR number: 'Before a concrete function-application oracle replaces the historical RT-port witness, QSL #1483 and the IR-595 CG consumer must land'. FR-021 and TC-031 now gate on IR-1007. This is the same class of defect as FND-003, left in AD-004. | spec/assurance/AD-004-cg-crate-layout.md:371-373 |

## Dispositions

Round 1, checked against fix-round head 1249c63ad436f113d26c402bd15ef429f1346dd2.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AD-004 now states the direct quire-eval, quire-exact and quire-semantic-value dependencies beside qsl-replay and no QSL re-export; AC-32 states the expected set and TC-031 12b inspects it. |
| FND-002 | fixed | The guard is a private post-lookup function that production feeds only its own lookup result. TC-031 feeds it a genuine second selection from a successful public lookup_callable in the same acquisition. #1483's lookup_callable is public and CallableSelection is Clone, so this is reachable without forging a selection and without changing the facade. |
| FND-003 | fixed | AC-32/35/36/37, Status and TC-031 12b now gate on IR-1007, with #1483 named in prose as the current candidate. The residual AD-004 line 372 is recorded as new FND-005. |
