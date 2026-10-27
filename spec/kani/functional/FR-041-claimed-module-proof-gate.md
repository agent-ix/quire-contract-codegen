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

- One checked-in, reviewed declaration of the modules whose proof coverage the gate claims, using
  stable module identities. A source file may contain more than one claimed module.
- The current candidate's build, the source locations of its compiled modules, and the per-check
  location, class and status in each real Kani report exposed by [FR-017](./FR-017-kani-execution-evidence.md).
- The current build's call reachability from each proof expectation and function under proof; the
  mutation definitions and checked-in evidence of their exact source changes, commands and failed
  checks. The helper intersection is derived from this build, not maintained as a second list.

## Behavior

- The gate SHALL compare the reviewed declaration with the modules in the current proof build and
  with the declaration at the candidate's merge base. It SHALL fail for an empty declaration, a
  declared module absent from the build, or a claim removed since that base, naming each missing or
  removed module. A generated module name change does not silently erase its former claim.
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
- While the 2026-10-27 IR-348 V1 retirement hold is active, the checked-in reviewed
  declaration SHALL mark only the `kani_fr_003_4_proof_boolean_transition_module` claim row
  in `scripts/kani_claims.json` as `held`. The gate SHALL accept a hold only on that row and
  validate it. While this hold is active, the reviewed declaration SHALL record reason
  "V1 deleted by owner ruling 2026-10-27; no genuine V2 named independent expectation/control
  exists yet" and expiry "until the V2 named independent expectation + real
  positive/falsifying control exists". The gate SHALL retain that exact row in the reviewed
  declaration and compare it with the merge-base declaration, even after the V1 module is absent
  from the compiled build. For this row alone, while the hold is valid, absence from the build is
  reported `held` rather than failed under the ordinary absent-module rule, and the positive
  proof and mutation controls are suspended as a narrow exception to AC-4. The gate SHALL print
  the held module identity, reason and expiry, count held rows separately from discharged rows,
  and validate the claim declaration and hold output in the claim test. While this sole current
  claim is held, zero claimed modules, proofs or mutations are discharged; a passing claim test
  grants no FR-041 proof credit. The gate SHALL run every non-held claim and shared-helper
  control. It SHALL exit zero only when the hold declaration and reporting are valid and all
  non-held controls and other required lane checks pass; any non-held failure, missing or skipped
  control SHALL make it exit non-zero. A missing hold reason or expiry, a held name absent from
  the declaration, a removed claim relative to the merge-base declaration, or a hold on any
  other module SHALL fail the gate. The exception expires under IR-1047 when the genuine V2
  named independent expectation and real positive/falsifying control exist. IR-348's immediate
  V1 deletion code slice SHALL implement the hold in or before its deletion PR.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-041-AC-1 | A nonempty reviewed declaration is checked against the current compiled-module census and the merge-base declaration. An absent compiled module, a removed claim, and an empty list each name the claim or condition and fail; adding a compiled claim does not erase an older one. | Test |
| FR-041-AC-2 | With two claimed modules sharing one `src/lib.rs`, a SUCCESS location inside only the first and an UNREACHABLE check inside the second reports the second `unreached` and fails. A SUCCESS check inside each passes this census; a SATISFIED cover, unknown location or non-SUCCESS status cannot supply the missing check. | Test |
| FR-041-AC-3 | The current build's transitive expectation/subject reachability yields the shared-helper intersection; the gate fails when the checked-in generated evidence omits or adds a helper, and records an explicit empty set when none is shared. For an exact operation, the expectation uses QSpec vectors or a model with no `quire-exact` call. | Test and Inspection |
| FR-041-AC-4 | Each claimed module and computed shared helper has a positive unmodified real-Kani proof and a source mutation that compiles, executes, and reports a failed proposition check. Deleting a control or making its mutant verify, fail to compile, time out, skip execution, or merely trip the dirty-tree guard fails the gate; evidence names the exact diff, command and failed check from this run. | Test |
| FR-041-AC-5 | A stubbed executor, predetermined outcome or build-only run produces no qualifying module evidence and cannot pass the gate. | Test |

## Status

Open; IR-348 implements the bounded hold for the retired V1 module claim row in
`scripts/kani_claims.json`. The claim test and gate validate the held declaration, report the
reason and expiry, and compare the claim names with the merge-base declaration. The V1 source was
removed; `kani_generation::guard_sources()[0]` now selects a surviving V2 scalar harness and
provides no replacement FR-041 module proof. A genuine generated V2 module, its reviewed claim,
current-build census and real-Kani proof and mutation controls must replace it. No qualifying V2
source or current-run evidence is identified here. The hold expires only when the V2 named
independent expectation and real positive/falsifying control exist. IR-1047 owns that V2
replacement and hold expiry. While the only current claim is held, no module proof or mutation
is discharged. FR-041-AC-1 through AC-5 remain unchanged and unfulfilled for the held claim;
non-held claims retain the full gate.

## Dependencies

- **Upstream**: [FR-017](./FR-017-kani-execution-evidence.md) supplies typed per-check report evidence;
  [NFR-006](../non-functional/NFR-006-real-kani-lane-gating.md) owns the existing lane target and
  candidate freshness checks.
- **Verification**: [TC-056](../matrix/TC-056-claimed-module-proof-gate.md).
