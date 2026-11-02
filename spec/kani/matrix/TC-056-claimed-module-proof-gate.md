---
id: TC-056
title: "Verify claimed-module discharge and real mutation controls"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-041
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/NFR-006
    type: verifies
---
# TC-056: Verify claimed-module discharge and real mutation controls

## Description

Verify that the existing real-Kani lane accepts a declared module claim only when the current
build and transcript locate a discharged proposition there and an injected mutation actually
falsifies it. This procedure adds the [FR-041](../functional/FR-041-claimed-module-proof-gate.md)
controls to [NFR-006](../non-functional/NFR-006-real-kani-lane-gating.md)'s `make kani-gate`;
it does not create another repository gate.

## Test Procedure

The [retirement record](../functional/FR-041-claimed-module-proof-gate.md#v1-claim-retirement)
provides no V2 proof credit. These procedures remain Open until production generated
modules provide a genuine current-build claim source and installed-Kani controls.
IR-1047's V2 implication and IR-1049's new V2 Boolean state-transition claim are
separate obligations, each with its own generated identity.

1. Obtain a nonempty set of claims from the current production generated artifact.
   Build the candidate and compare claimed identities with compiled modules. In isolated
   negative controls, omit one claimed module from the build and use an empty claim set;
   assert each fails and names the module or empty set (FR-041-AC-1). This procedure
   cannot be credited until a genuine current-build claim source exists.
2. Run the unmodified proofs with the installed Kani backend and read the typed per-check
   locations and statuses. Use two claimed modules occupying different line extents of the same
   `src/lib.rs`: one with an in-module SUCCESS proposition, the other with only an UNREACHABLE
   check. Require `unreached` for the second. Repeat with a SUCCESS inside both modules, then
   replace the second's check with a SATISFIED cover, a non-SUCCESS status and an unknown source
   location, one at a time. Require failure in every missing-SUCCESS case (FR-041-AC-2).
3. From the current build, compute each expectation's and subject's transitive reachable
   functions and their intersection. Compare it with the checked-in generated helper evidence,
   including the empty-set case; delete and add one helper in negative controls. Inspect the
   expectation's semantic source. If it proves a `quire-exact` operation or a runtime operation
   calling one, verify that it uses QSpec operation vectors or a specification model with no
   `quire-exact` call (FR-041-AC-3).
4. Verify each unmodified claimed proof first. For each claimed module and each computed shared
   helper, apply its distinct source mutation in an isolated build, compile it and run its proof
   with real Kani. Inspect a FAILURE check at the affected proposition and record the exact diff,
   command and failing check output. In negative controls delete one mutation, leave one mutant
   verifying, and make mutants fail to compile, time out, skip execution, return inconclusive, or
   trip only the clean-tree guard. Each negative control must fail acceptance. A stand-in launcher
   may test control parsing, but cannot satisfy the positive proof or mutation runs
   (FR-041-AC-4, FR-041-AC-5).
5. Invoke `make kani-gate` on a clean current candidate. Confirm it includes the module census
   and mutation controls in its one required real-Kani lane, keeps NFR-006's lock and
   test-count behavior, and exits non-zero if any step above fails (NFR-006-AC-19).
6. Select an admitted V2 Boolean implication scalar claim through the production
   `KaniObligationRequest` path and render it through the scalar generator. Assert the
   generated identity's module and harness match the genuine current-build claim source
   and compiled module census. Inspect the generated module's named
   `subject_boolean_implication` and `expected_boolean_implication` call-graph roots:
   subject calls the runtime implication helper, expectation computes `!left || right`
   from the same two drawn Booleans and reaches no subject, oracle or `quire-exact`
   function. Derive and compare their transitive shared-helper intersection; include
   an explicit empty set or a distinct mutation and positive/falsifying run for each
   observed shared helper. In separate controls, remove or rename the module, disconnect
   the expectation root, and make its source location `UNREACHABLE`; require the named
   claim or reachability refusal (FR-041-AC-6, FR-041-AC-7).
7. Run the unmodified generated V2 module with installed Kani and require a located
   in-module SUCCESS and satisfied final cover. Mutate only the parsed subject-call
   expression inside that module to `false`, preserve the package, generated oracle
   and named expectation, and run the same harness with installed Kani. Require a
   compiled, executed FAILURE at its subject-versus-expectation assertion and
   in-domain playback with `left` false. Require the current-run source diff,
   invocation and check output. A mutant that verifies, refuses, times out or fails
   compilation does not count (FR-041-AC-8).
8. Include the V2 implication claim in the current-build census after its positive
   and falsifying controls pass. Require the real-Kani lane to execute the claim test
   and fail for a missing, skipped or failing claim or shared-helper control. Do not
   credit the distinct Boolean transition obligation. This completes IR-1047's
   machinery delivery without requiring the later transition claim (FR-041-AC-9).
9. After that machinery lands, generate the Boolean transition obligation as a new
   V2 claim with its own stable module and obligation identities, precondition
   `input_current || state_pre`, postcondition `state_post`, actual subject,
   checked-package/source provenance and independent expectation. Register it in
   the current-build census without restoring or re-identifying a V1 row. Require
   installed-Kani healthy and falsifying controls on that module and every computed
   shared helper; a distinct implication proof or stored verdict does not discharge
   it (FR-041-AC-10).

## Expected Results

The required real-Kani lane passes only when every current-build claimed module
has an in-module SUCCESS proposition, the build-derived shared-helper set agrees
with the generated evidence, and each claimed module and shared helper has a
compiling, executed mutation with a real failed check. No build-only result,
stub, process refusal or stored verdict counts. A current V2 claim cannot be
removed or left uncompiled to satisfy these checks.

## Status

Open. A genuine current-build claim source and installed-Kani proof and mutation
controls are still needed for IR-1047. IR-1049's new V2 transition claim follows
that prerequisite and is not a condition for landing the machinery.
