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

The current claim-control fixture uses `kani_generation::guard_sources()[0]`, a V1 bundle
module named by `scripts/kani_claims.json`. Under the 2026-10-27 IR-348 retirement ruling,
that source is historical. The IR-348 immediate V1 deletion code slice must implement, in or
before its deletion PR, an operator hold for only that named V1 module claim row, with reason
"V1 deleted by owner ruling 2026-10-27; no genuine V2 named independent expectation/control
exists yet" and expiry "until the V2 named independent expectation + real positive/falsifying
control exists". Deleting the V1 module or dropping its claim supplies no proof evidence.
Execute the positive procedure on the production V2 Boolean implication scalar module
as an additional claim after its named expectation root and claim are reviewed. This
implication claim has a different proposition from the held FR-003@4 Boolean state-transition
claim; it does not remove that row or expire its hold. IR-1047 Acceptance 4 remains
unmet. At this spec head, the generator still inlines the expectation and the claim
test still selects V1 source. The hold is PLANNED at this spec head. The held V1 row remains Open, and
all non-held controls stay required.

1. Check in a reviewed nonempty claim declaration for the generated modules the gate says it
   proves. Build the current candidate and compare those identities with the compiled modules.
   In isolated negative controls, omit one declared module from the build, remove one claim
   relative to the merge-base declaration, and use an empty declaration. Assert each fails and
   names the affected module or empty list (FR-041-AC-1).
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
   and mutation controls in its one required real-Kani lane, keeps NFR-006's lock, freshness and
   test-count behavior, and exits non-zero if any step above fails (NFR-006-AC-19).
6. Select an admitted V2 Boolean implication scalar claim through the production
   `KaniObligationRequest` path and render it through the scalar generator. Assert the
   generated identity's module and harness match the reviewed claim declaration and
   compiled module census. Inspect the generated module's named
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
8. Add the reviewed V2 generated implication claim as a separate non-held row after
   the above controls pass. Retain the historical FR-003@4 V1 declaration row under
   its hold; assert that the V2 implication does not receive FR-003@4 credit or erase
   the held identity from the merge-base declaration. Run `make kani-gate` on the
   current candidate; it must execute this claim test in the one required lane and
   fail if any non-held claim or helper control fails, is missing or is skipped.
   Confirm the historical row remains reported held, not discharged
   (FR-041-AC-1, FR-041-AC-9, NFR-006-AC-19).
9. For the future held-claim expiry, generate a production V2 module from an admitted
   package carrying the original FR-003@4 Boolean transition: precondition
   `input_current || state_pre`, postcondition `state_post`, and the actual transition
   subject. Require the same reviewed module/claim identity in the generated module,
   retained manifest row and current compiled census, plus source provenance and a
   separately named independent expectation. Compute exact subject/expectation
   helper intersection from the current graph. Run installed Kani on the healthy
   module and an in-module source mutant; require an in-module SUCCESS and satisfied
   cover, then a compiling, executed in-module FAILURE with in-domain playback.
   Update the retained row's V2 source fields and clear `held` only after these
   same-claim controls pass. In negative controls, remove the row, omit its compiled
   module, change the obligation, substitute the V2 implication module, or provide
   a stored old verdict; require refusal and no hold expiry (FR-041-AC-1,
   FR-041-AC-10). This procedure is PLANNED under IR-1049 and does not claim
   IR-1047 Acceptance 4 met.
10. While the IR-348 hold is active, inspect the one reviewed V1 claim row and its exact reason
   and expiry. Require the claim test to validate the retained declaration and held output
   without treating its success as proof credit. Require separate `kani-gate:` lines for the held
   identity, reason, expiry and count, leaving the existing run-result line unchanged. Assert
   that a hold on another row, a missing reason or expiry, a removed declaration row, and any
   failed, missing or skipped non-held control make the gate fail. With the only current claim
   held, assert zero claimed modules, proofs and mutations discharged. The gate may exit zero
   only when hold validation, all non-held controls and other lane checks pass.

## Expected Results

The existing `make kani-gate` passes only when every non-held declared, compiled claim has an in-module
SUCCESS proposition, the fresh build-derived helper set agrees with its checked-in evidence, and
every claimed module and shared helper has a compiling, executed mutation with a real failed
check. A compiled module with no SUCCESS proposition is named `unreached`. No build-only result,
stub, process refusal or stored old verdict counts as a current proof or mutation result. The
operator-held V1 row is listed and counted separately on `kani-gate:` lines with its reason and
expiry and remains unproved. A successful separate V2 implication claim has its own
proof credit and does not discharge FR-003@4 or expire that hold. Hold expiry needs
FR-041-AC-10's same-claim V2 producer, retained identity and current-run controls.
The ordinary
absent-module and positive/mutation failures apply to every non-held row; only the valid V1 hold
excepts the absent V1 module and its suspended controls. With the only current claim held,
zero proofs and mutations are discharged, regardless of a passing claim test.

## Status

Open for IR-19 implementation. The hold itself is PLANNED for IR-348's immediate V1 deletion
code slice in or before its deletion PR. The current V1 module declaration and
`guard_sources()[0]` fixture are to be operator-held under IR-348 pending the named V2 independent
expectation and real positive/falsifying control for that same claim; the V2 implication
renderer owns only an additional distinct claim, and its inline expectation is not yet a
named call-graph root. The same-claim V2 producer and retained-row update are PLANNED
under IR-1049. IR-1047 Acceptance 4 remains unmet. No real-Kani result is
asserted by this specification artifact.
