---
id: FR-041
title: "Discharge every claimed Kani module with an independently checked proposition"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/NFR-006
    type: depends_on
---
# FR-041: Discharge every claimed Kani module with an independently checked proposition

## Description

When the repository claims proof coverage for a generated-harness module, its existing
[`make kani-gate`](../non-functional/NFR-006-real-kani-lane-gating.md) SHALL accept that claim only
after a real Kani run discharges a proposition in the module and an executed mutation inside the
module falsifies a proof check. The gate applies the proof-stage acceptance rule of QSL ADR-011
§2.3 to CG's claimed modules. It makes no claim about a module absent from its declared list or
about a semantic stage the test input did not traverse.

## Inputs

- The modules for which the current proof run claims coverage, identified from the compiled
  generated artifact. A source file may contain more than one claimed module. The current
  implementation has no qualifying claim source after retirement of its fixed V1 inventory.
- The current candidate's build, the source locations of its compiled modules, and the per-check
  location, class and status in each real Kani report exposed by [FR-017](./FR-017-kani-execution-evidence.md).
- The current build's call reachability from each proof expectation and function under proof; the
  mutation definitions and checked-in evidence of their exact source changes, commands and failed
  checks. The helper intersection is derived from this build, not maintained as a second list.

## Behavior

- The gate SHALL compare each module claimed by the current generated proof run with the
  modules in that build. It SHALL fail for an empty claimed set or a claimed module absent from
  the build, naming the condition or missing module. This obligation remains Open until a
  genuine current-build claim source exists; the retired fixed inventory grants no credit.
- For each compiled, claimed module, the gate SHALL attribute checks using their file **and line**
  against the current build's module source extents. It SHALL count only a check with backend status
  `SUCCESS` and a location inside that module. `UNREACHABLE`, a satisfied cover with status
  `SATISFIED`, an unknown location and every other status contribute zero. If the module has no
  counted proposition, then the gate SHALL report that module as `unreached` and fail, even if a
  different module in the same source file has a SUCCESS check or the whole harness verified.
- The gate SHALL derive the functions reachable from both an expectation and its function under
  proof from the current build. It SHALL compare that computed shared-helper set with the
  checked-in generated evidence set, including an explicitly empty set, and fail on a difference.
  The expectation SHALL be independent of the function under proof: for a `quire-exact` operation
  or a runtime operation that calls it, the expectation SHALL come from QSpec operation vectors or
  a checked-in specification model that calls no `quire-exact` operation.
- The gate SHALL first verify the unmodified claimed proof, then inject at least one distinct
  source mutation inside each claimed module and one in each computed shared helper. It SHALL run
  the affected proof through the installed Kani backend for every mutant. A control succeeds only
  when the changed source compiles, its proof executes, and a proposition's reported check is
  `FAILURE` because of that mutation. A compilation error, skipped run, timeout, inconclusive
  result, changed verdict without a failing check, or a mutant that still verifies SHALL fail the
  control; a dirty-tree or other process refusal alone is not a falsifying mutation. The gate SHALL
  retain the exact mutation diff, invocation and failing check output as current-run evidence.
- The gate SHALL accept no predetermined verdict or stubbed executor as proof evidence.

## V1 claim retirement

The V1-sourced FR-003@4 claim ended with the V1 generator deletion and retirement
of its inventory and hold in [CG #636](https://github.com/agent-ix/quire-contract-codegen/pull/636)
(IR-1105), as confirmed by the
[owner ruling on IR-1049](https://linear.app/agent-ix/issue/IR-1049#comment-813eda1b).
No V1 module identity, obligation identity or proof credit carries into a V2 claim.
The Boolean state-transition obligation is implemented by a new V2 claim under
AC-10. All current V2 claims remain subject to AC-1 through AC-5: an empty claim
set, absent compiled module, missing independent expectation or missing control
SHALL fail. Retirement supplies no exception for a current V2 claim.


## Current-build V2 claim machinery (IR-1047, planned)

- When an additional V2 claim is declared, the generator SHALL obtain an admitted V2 Boolean
  implication scalar claim from the production `KaniObligationRequest` path and render its
  module through `src/kani/generate/scalar.rs::render_boolean_implies`. A future
  current-build claim control SHALL assemble the generated Rust artifact, its required
  runtime dependency and its separate scratch-crate subject source; it SHALL
  derive the declared module and harness identities from the generated identity record,
  never from a copied V1 source or a hand-authored module.
- The generated module SHALL call a named `subject_boolean_implication` function rooted in
  the compiled call graph. That function SHALL call the production
  `quire_contract_runtime::operators::implies_short_circuit(left, || right)` subject. A
  separately named `expected_boolean_implication(left, right)` function SHALL compute
  `!left || right` directly from the same symbolic Boolean operands. The generated
  harness SHALL compare both the compiled subject and its embedded FR-014 oracle with
  that expectation and retain its final non-vacuity cover. Neither expectation path
  SHALL call the subject, the oracle or `quire-exact`; the two named roots and their
  transitive callees SHALL be inspectable in the current build's call graph.
- When the V2 claim is declared, the claim test SHALL use the generated module symbol,
  exact harness symbol, named subject root and named expectation root for the compiled
  census and shared-helper intersection. It SHALL reject a missing, renamed,
  uncompiled or `unreached` module under the ordinary named FR-041 causes. For every
  computed shared helper, it SHALL require its own reviewed source mutation and
  current-run real-Kani positive/falsifying control; it SHALL record an explicit empty
  set when the roots share none.
- The V2 module's falsifying mutation SHALL change only its parsed subject-call
  expression inside the generated module to `false`. The unmodified module SHALL
  produce an in-module `SUCCESS` check and a satisfied final cover under installed
  Kani; the mutant SHALL compile, execute, and report an in-module `FAILURE` at the
  subject-versus-expectation assertion with in-domain playback where `left` is false.
  This source mutation, not a stored verdict or a stand-in runner, SHALL be the
  claim's FR-041 control.
- Once those current-run controls pass, the generated V2 implication claim SHALL be
  included in the current-build module census. `make kani-gate` SHALL run the
  real-Kani claim control in its required lane and fail for a claimed module or
  shared-helper control that is absent, skipped or unsuccessful. This machinery
  and its independently qualified V2 implication claim complete the IR-1047
  delivery without depending on IR-1049. The distinct Boolean state-transition
  obligation remains a separate new V2 claim; implication proof credit SHALL NOT
  discharge it.

## New V2 Boolean state-transition claim (IR-1049, planned)

After the IR-1047 machinery is available, a production V2 module SHALL implement
the Boolean state-transition obligation originally described by FR-003@4:
precondition `input_current || state_pre`, postcondition `state_post`, and the
actual transition subject. It SHALL have its own stable V2 module and obligation
identities, authentic checked-package/source provenance, and a separately named
independent expectation rooted in the current compiled call graph. It SHALL be
registered as a new claim in the current-build census, without restoring a V1 row,
reusing a V1 identity or copying V1 generator/source code.

Installed Kani SHALL give this new module an in-module SUCCESS proposition and
satisfied cover. An in-module source mutation SHALL compile, execute and produce
an in-module FAILURE check with in-domain playback. Its computed shared helpers
remain subject to AC-3 and AC-4. No distinct implication proof, absent current V2
module or saved result discharges this obligation.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-041-AC-1 | A nonempty set of current-run generated module claims is checked against the current compiled-module census. An absent compiled module or empty set names the claim or condition and fails. Open pending a genuine current-build claim source. The former merge-base declaration comparison is retired by IR-1105. | Test |
| FR-041-AC-2 | With two claimed modules sharing one `src/lib.rs`, a SUCCESS location inside only the first and an UNREACHABLE check inside the second reports the second `unreached` and fails. A SUCCESS check inside each passes this census; a SATISFIED cover, unknown location or non-SUCCESS status cannot supply the missing check. | Test |
| FR-041-AC-3 | The current build's transitive expectation/subject reachability yields the shared-helper intersection; the gate fails when the checked-in generated evidence omits or adds a helper, and records an explicit empty set when none is shared. For an exact operation, the expectation uses QSpec vectors or a model with no `quire-exact` call. | Test and Inspection |
| FR-041-AC-4 | Each claimed module and computed shared helper has a positive unmodified real-Kani proof and a source mutation that compiles, executes, and reports a failed proposition check. Deleting a control or making its mutant verify, fail to compile, time out, skip execution, or merely trip the dirty-tree guard fails the gate; evidence names the exact diff, command and failed check from this run. | Test |
| FR-041-AC-5 | A stubbed executor, predetermined outcome or build-only run produces no qualifying module evidence and cannot pass the gate. | Test |
| FR-041-AC-6 | The V2 claim is produced by the admitted V2 Boolean implication request through the production scalar renderer; its generated module and harness identities match the current-build claim source and compiled census. A V1 source, hand-authored module, missing module, changed identity or uncompiled claim is rejected by name. Open pending a genuine current-build claim source. | Test |
| FR-041-AC-7 | The V2 module exposes separate named subject and expectation roots in the current compiled call graph. The subject reaches the runtime implication operation; the expectation evaluates `!left || right` over the same Boolean inputs and reaches neither subject, oracle nor `quire-exact`. The observed helper intersection exactly matches the declared helper mutations, including an explicit empty set. | Test and Inspection |
| FR-041-AC-8 | The unmodified generated V2 module has a location-attributed SUCCESS proposition and satisfied final cover with installed Kani. Replacing only its parsed subject-call expression with `false` compiles and executes the same harness, then yields an in-module FAILURE at the subject-versus-expectation assertion with in-domain playback where `left` is false; each computed shared helper has its own positive and falsifying control. | Test |
| FR-041-AC-9 | After AC-6 through AC-8 pass in the current lane, the V2 implication module is included in the current-build claim census. The required `make kani-gate` run executes its real-Kani control and fails on any missing, skipped or failing claim or helper control. This completes the IR-1047 machinery delivery independently of the new Boolean state-transition claim in AC-10; implication proof credit does not discharge that transition obligation. Open pending a genuine current-build claim source. | Test |
| FR-041-AC-10 | A production V2 module implements the Boolean state-transition obligation originally described by FR-003@4, with precondition `input_current \|\| state_pre`, postcondition `state_post`, the actual transition subject, authentic checked-package/source provenance and a separately named independent expectation. The new claim has its own stable V2 module and obligation identities and is registered in the current compiled census without restoring a V1 row or asserting V1 identity continuity. Installed Kani gives that module an in-module SUCCESS proposition and satisfied cover; its in-module source mutant compiles, executes and reports an in-module FAILURE check with in-domain playback, with per-helper controls under AC-3 and AC-4. A different obligation, V2 implication proof, absent current V2 claim or saved verdict does not discharge it. PLANNED (IR-1049), dependent on the IR-1047 machinery. | Test and Inspection |

## Status

Open. IR-1047 delivers the genuine current-build claim source, compiled-module
census, subject/expectation helper intersection and per-helper controls, using the
V2 implication claim. The new V2 Boolean state-transition claim follows in
IR-1049. Neither delivery receives proof credit from this specification correction;
current-run installed-Kani positive and falsifying evidence remains required.

## Dependencies

- **Upstream**: [FR-017](./FR-017-kani-execution-evidence.md) supplies typed per-check report evidence;
  [NFR-006](../non-functional/NFR-006-real-kani-lane-gating.md) owns the existing lane target and
  real-Kani lane execution.
- **Verification**: [TC-056](../matrix/TC-056-claimed-module-proof-gate.md).
