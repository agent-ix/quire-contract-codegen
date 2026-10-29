---
id: TC-061
title: "Verify current-V2 function proof producer and replay binding"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-047
    type: verifies
---
# TC-061: Verify current-V2 function proof producer and replay binding

## Description

Planned production controls for
[FR-047](../functional/FR-047-v2-function-proof-producer.md).
Use one genuinely admitted checked V2 source and its original request, not a
test-constructed `KaniObligationIdentity` or `V2FunctionProofIdentity`, V1 claim, scalar conversion or
hand-built replay package. The independent assertion playback comes from the
installed backend's complete captured transcript.
The current QSL FR-485 facade is available; CG's concrete FR-021 consumer is
still Planned under IR-595. The V2 identity/binding API and imported-owner
function-site selector are Planned; current V1 replay APIs do not back them.

## Test Procedure

1. Compile one source-authored Boolean function with finite parameters into a
   checked V2 package; retain the original source, selected dependencies,
   function declaration, claim occurrence and request index. Run the real CG
   producer. Assert its emitted Rust is executable, its persisted identity
   names that exact function/occurrence and all arguments, and the original
   source/dependency lock and limits are retained (AC-1, AC-4, AC-7).
2. Run the unmodified harness through installed Kani within its recorded
   ceilings. Require `Verified` and a satisfied cover. Mutate only the selected
   function body in a separately admitted same-claim source, regenerate through
   the same producer, and require `Falsified`. Independently retain the first
   eligible assertion playback from complete stdout. Check its bytes against
   the runner's retained counterexample, decode only in-domain values and
   replay through the planned V2 typed function entry with that mutant's source and
   identity. A synthetic outcome, a test-built harness or a mutated assertion
   is insufficient (AC-1, AC-5).
3. Place two distinct claim occurrences on one selected node. Verify two
   separate identities, request indices, contexts and harnesses. Cross-swap
   either occurrence or request index and require typed refusal; repeat with
   an equal function name and body in another declaring package (AC-2, AC-4).
4. Request an unknown claim, absent declaration, ambiguous occurrence and
   non-function claim, each beside a healthy sibling. Inspect the refusal
   variant and absence of emitted Rust, runner invocation and replay decode;
   compare the healthy sibling's bytes with its solo request (AC-3).
5. Change one binding at a time: package, unit source, dependency owner or
   selection, declaration, occurrence, request index, obligation kind,
   parameter identity and bound, generated artifact and proof ceiling. Require
   a named refusal or distinct admitted identity; do not substitute a new
   caller-built identity. Feed the retained original `ReplayInputs` and
   `V2FunctionProofIdentity` to `bind_v2_function_replay`; check that only its
   matching `BoundV2FunctionReplay` reaches typed replay. For a function
   declared by a dependency, select its actual declaring package/function
   occurrence through QSL's planned imported-function site API and compare
   the original dependency lock; report the branch Planned until that API
   exists, without substituting the unit-only `call_site` (AC-4, AC-5).
6. Use FR-016-AC-6's required public decode bounds in the V2 entry. Test byte and physical value-count ceilings at the smaller of caller and
   capture limits and one unit over each, with a poisoned trailing entry to
   show that the decoder stops at the limit. Test a missing, extra and
   out-of-domain value independently; none may settle as a reproduced
   violation (AC-6).
7. Inspect the emitted oracle and harness call path for the selected owner
   callable, result and charge distinctions. Keep TC-057's wrong-completed
   result and genuine environment-construction controls separately pending
   until they execute their own shipped guards (AC-7).

## Expected Results

The healthy current-V2 function proof verifies, the body-only mutant falsifies
and replays with its own in-domain playback, and no changed association or
over-bound transcript receives proof or reproduced-failure credit. Every
selection failure is typed and leaves healthy sibling output unchanged.

## Status

PLANNED under IR-1121. No current-V2 function Kani producer exists on CG main;
the direct test-only function identity fixture does not back these criteria.
CG's concrete FR-021 oracle (IR-595) and QSL's imported-function site
selection remain prerequisites for the respective controls.
