---
id: "SR-1529"
title: "CG PR 282 gap analysis: the QSL bcca433 lock move against FR-029, FR-030, FR-024 and AD-003 link 7"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@bd19f73acf6aa3830f8e0ce7b5abc6401516195a; Cargo.lock, src/replay/function.rs, src/replay/state_clause.rs, tests/it/terminal_map.rs, tests/it/kani_obligations_state_clause_replay.rs (diff 735e704...bd19f73); context: spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/functional/FR-030-ir-outcome-terminal-map.md, spec/kani/matrix/{TC-040,TC-041,tests}.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/assurance/AD-003-evidence-chain.md; QSL agent-ix/quire-spec-language at bcca433 and c8f0c28"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-030
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: references
---

# SR-1529: CG PR 282 gap analysis

## Summary

Ticket: IR-623. PR: agent-ix/quire-contract-codegen#282 at bd19f73. Its merge base is 735e704.
`main` has since moved to e3d3ab4 (#281, spec and reviews only), and the PR merges cleanly onto
it. Plan completion: not assessed.

I measured these myself and did not take them from the PR text:

- `quire coverage --strict` (quire 0.36.1) on the head: 44 unbacked rows, 0 contradicted. The diff
  adds, removes and renames no tagged test, and changes no `Trace:` line or spec file, so the
  count cannot move.
- Examined the setup-refusal row of FR-029 (Description: "a `CallSiteRefusal` other than `Fault`
  (code from `CallSiteRefusal::code()`)"; the `falsified` / setup refusal on data row), along with
  FR-029-AC-13, FR-030-AC-12 and AD-003 link 7 and R-Q1 (b). All of them read any non-fault
  `CallSiteRefusal` as a setup refusal on data that settles `Inconclusive(ReplayRefused)` with
  `code()`. The two new arms follow that rule exactly. At bcca433, QSL's own
  `TerminalValue::from_call_site_refusal` puts `UnknownField` and `UnknownPopulation` in the same
  non-fault arm as `UnknownClause`. That arm is added in the same QSL range (c8f0c28..bcca433,
  `proof_result.rs`). `CallSiteRefusal::code()` gives `Code::MissingDeclaration` for both. CG
  never calls `call_site` with a `FieldName` or `PopulationName` (no `src` file names either
  type), so the arms are needed only to make the match exhaustive. If a field or population name
  is ever unknown, it is a recompiled package that disagrees with the selection, the same case as
  `UnknownClause`. That is data, not a CG defect. A different reading would contradict the
  merged FR-029 table. AD-003 R-Q1 (b) lists the variants "for example", so leaving the two new
  ones out of that list makes nothing false.
- FR-024 state-clause path, statement "one `DeclaredDomain` per state field, from the field's
  declared integer range". The adapted key `DomainKey::Node { node: parameter, path: [position] }`
  has the same node and path as the c8f0c28 struct `DomainKey::new(parameter, vec![position])`.
  The adaptation therefore preserves behaviour. QSL bcca433 adds no check of declared-domain keys
  on replay: `declared_domains` is read only by the witness round trip, so the QSL-345 check
  still does not exist. But see FND-001 for the key QSL now defines for a state field.
- Spec text against the new lock: see FND-002 and FND-003.

## Verdict

CONDITIONAL. The code follows the merged FR-029 and FR-030 rows, and FR-029-AC-13 and FR-030-AC-12
are backed by failable tests (mutants in SR-1528). The lock move does two other things that the
PR does not record. It brings in QSL's state-field key definition (ADR-012 §15.4), which the
state-clause key CG builds does not follow (FND-001). It also brings in the `InternalFault`
re-export, which makes six spec statements false (FND-002). FND-002 can be fixed with text in this
PR, or deferred to a ticket that names those statements. FND-001 is a deferral to a ticket: it is
not a regression this PR makes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The state-clause declared domain is keyed `DomainKey::Node { node: <self parameter node>, path: [<field position in CG's field order>] }`. At the locked QSL bcca433, merged ADR-012 §15.4 (amended 2026-10-01, built by QSL-345 item 4, #637) defines a state field's domain key as `Node { node: <model/object_type node of the declaring type>, path: [<field ordinal in ascending UTF-8 name order>, ...] }`, and `call_site` returns it as `FieldSite.domain`. The PR keeps the pre-move key (same behaviour), and the updated `tc_035` now asserts the whole old key, which locks in a key that QSL's definition does not give for a state field. Nothing breaks today because QSL does not check declared-domain keys yet. FR-024 states no key shape for the state-clause path, and FR-024-AC-29 still says QSL-345 has not settled the key. Fix (a follow-up is acceptable): ticket the move to `FieldSite.domain` and record the divergence in FR-024 Status until then. | src/replay/state_clause.rs:829-860; tests/it/kani_obligations_state_clause_replay.rs:473-500; spec/replay/functional/FR-024-counterexample-envelope-intake.md:212-213,302 |
| FND-002 | medium | At bcca433, `qsl-replay` re-exports `InternalFault` (`lib.rs:69`, QSL #635), which has a public `const fn new`. These statements, read at the lock this PR writes, are now false: FR-029 Status ("`InternalFault`, which `qsl-replay` does not re-export ... stays planned until QSL exports a constructor or the type through `qsl-replay`"), FR-030 Status, TC-040 Status, TC-041 Status, `spec/kani/matrix/tests.md` rows for FR-029-AC-10 and FR-030-AC-10, and AD-003 link 7 ("FR-030-AC-10 stays planned until QSL re-exports `InternalFault`"). The stated condition for FR-029-AC-10 and FR-030-AC-10 to stay planned has been met, so those planned ACs now stay unbacked for a reason that no longer holds. Fix: correct the statements in this PR, or open a follow-up ticket that names them and backs AC-10. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:203-208; spec/kani/functional/FR-030-ir-outcome-terminal-map.md:170-175; spec/kani/matrix/TC-040-run-outcome-terminal-record.md:78-80; spec/kani/matrix/TC-041-ir-outcome-terminal-map.md:64-67; spec/kani/matrix/tests.md:42,44; spec/assurance/AD-003-evidence-chain.md:59 |
| FND-003 | low | FR-030 Status says the map is "built on `qsl-replay` at QSL `main` c8f0c28". The lock now reads bcca433. The revision is informational and gates nothing, so either drop the commit id or say it is informational. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:159 |

## Coverage

- `quire coverage --strict`: 44 unbacked, 0 contradicted on the head. The diff touches no tag.
- FR-029-AC-13 (`tc_040_a_setup_refusal_after_a_refutation_is_replay_refused_never_declined`) and
  FR-030-AC-12 (`tc_041_a_setup_refusal_after_a_counterexample_is_replay_refused_never_declined`)
  now enumerate all ten non-fault `CallSiteRefusal` variants, bare and wrapped in
  `ReplayPackageError` and `FrameReplayError`. Both bindings are correct.
- `tc_035_the_envelope_declares_each_ranged_field_and_the_transcript_names_the_playback` (Trace:
  FR-024-AC-12, TC-035) keeps its subject, and the assertion now covers the whole key. No FR-024
  AC states the state-clause `declared_domains` key; the FR-024 statement at line 212 is checked
  only by this test, under an AC about the clause identities. That gap predates the PR and is
  part of FND-001's follow-up.
- Plan completion: not assessed

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | Two test doc comments still make FND-002's false claim at 1b7c59c: `tc_040_a_fault_settlement_is_failed` ("QSL's `InternalFault` cannot be built here, so FR-029-AC-10 is not tagged") and `tc_041_a_counterexample_with_a_cg_defect_is_failed` ("QSL's `InternalFault` cannot be built here, so the fault half of FR-030-AC-10 is not tagged"). At bcca433, `qsl_replay::InternalFault::new` is public. Fix: reword to "not yet tested", as the spec now says, or rewrite them in the AC-10 follow-up. | tests/it/terminal_map.rs:381-382,1065-1066 |

## Dispositions

Round 1, reviewed at 1b7c59ca10065d65a47eb5310dd96cb8ff4e46b7 (fix commit 1b7c59c on bd19f73).

`git diff --stat bd19f73 1b7c59c` touches only seven `spec/` files: AD-003, FR-024, FR-029, FR-030,
TC-040, TC-041 and `spec/kani/matrix/tests.md`. Nothing under `src/`, `tests/`, `Cargo.*` or
`deny.toml` changes, so the green `make ci` on bd19f73 (SR-1528) still covers the code.
`make spec` on 1b7c59c exits 0 with no errors. Its warnings are the six EARS grammar warnings
that were already present on bd19f73. `quire coverage --strict` reports 44 unbacked and 0
contradicted, the same as before. No test tag changed, so FR-029-AC-10 and FR-030-AC-10 stay
untagged. Every rewritten line keeps them "Planned" and makes no claim that they are backed. The
PR body now records the spec edits and the quire-exact hold.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 1b7c59c: FR-024 Current state records the divergence accurately against QSL bcca433 ADR-012 §15.4 (declaring object_type node, ordinal in ascending UTF-8 name order, `FieldSite.domain`), says nothing breaks until QSL's check lands, and names the adoption. There is no code change, and the deferral is stated. |
| FND-002 | fixed | 1b7c59c: FR-029 Status, FR-030 Status, TC-040, TC-041, the tests.md AC-10 rows and AD-003 link 7 now say `qsl-replay` re-exports a constructible `InternalFault` (QSL bcca433, #635), with both ACs still planned and untagged. A grep of `spec/` finds no remaining "does not re-export". Two test doc comments outside `spec/` still make the claim; they are recorded as FND-004. |
| FND-003 | fixed | 1b7c59c: FR-030 Status reads "built on `qsl-replay` at QSL `main` (the commit is informational; the lock names it)". `c8f0c28` no longer appears in `spec/`. |
