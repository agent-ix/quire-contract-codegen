---
id: TC-047
title: "Check public routed scalar lowering replay and generated-content binding"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: verifies
---
# TC-047: Check public routed scalar lowering replay and generated-content binding

## Description

Planned public-consumer coverage of
[FR-032](../functional/FR-032-routed-scalar-replay-binding.md), distinguishing lowering parity
failure from source-property refutation. No implementation or passing replay is claimed. The
completed CG route is CODE-gated on QSL-641's amended full-claim reports, authoritative scalar
identity metadata, retained generation/proving context and checked driver-observation contract.
The measured QSL operator API and scalar terminal causes do not close those gates.

## Test Procedure

1. Obtain a real QSL-emitted checked package for `x + 1` with input `Int[0,9]` and result
   `Int[0,10]`; check those emitted ranges, then call public
   `generate_routed` for its scalar node and retain the returned identity and generated harness.
   Compile a public scalar builder call using those types directly, without a function identity.
2. In the named real-Kani lane, run the generated harness and an arithmetic mutation control.
   Retain its actual selected assertion playback and proved generated content. This increment
   has no admitted out-of-result-range exact value; do not invent one. Separately use bounded
   addition with both operands and result in `[-1000,1000]`, the descriptor already accepted by
   `kani_obligations::scalar_package`. Its QSL source counterpart must emit those checked ranges
   and retain its original `call_site` package/source tie before positive replay is admitted.
   `(600,600)` giving `1200` is an illustrative reachable out-of-result-range case; Kani may
   choose another pair. For an actual retained Kani falsification with correct generated
   refusal, seed a harness-assertion mutation that rejects otherwise correct refusal outside
   the result range, leaving the oracle intact. Decode the actual retained operand pair,
   assert that both operands are admitted and their exact sum is outside the result range,
   and derive the expected exact result from that pair rather than assume a solver choice.
   Retain that same mutated proof artifact and observe its oracle natively.
   This is the harness-defect agreement control, not a source violation. An arithmetic mutation
   instead supplies the divergence control. The observed ranges and native refusal must be
   asserted; source admission failure remains a gate, not a hand-built package replacement.
3. Build the scalar plan from the actual identity, decoded playback, proving package and
   canonical identity of that same proved generated oracle and the retained original generation
   limits context. Check setup precedence with one defect at a time: context/identity/schema,
   selected playback bytes above 8 MiB, adapter block/harness/arity/width, then operand domain.
   Those distinct refusals occur before an absent-capability check. Only an otherwise valid
   request reaches unsupported-capability; conversion of it returns no settlement.
   Try missing limits context and replay limits altered from the original retained context;
   no current `ScalarObligationIdentity.limits` member or default is assumed. The original
   context is recorded by the scalar generation authority alongside the proved source.
4. For the admitted upstream route, record the exact QSL request's operator, operands, operand
   ranges, result range and retained original limits, plus its QSL obligation identity and
   original `call_site` package/source tie required by AD-002 R-6/R-7. Check package/source and
   scalar-node membership mismatches through the upstream typed refusal. Operand correspondence
   remains governed by [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-16.
   Execute the same proved oracle artifact natively over those operands and limits, following the
   driver authentication contract, and retain the typed outcome and its checked completion.
   CG supplies the original generation/proving record; the driver authenticates the observation
   against that record's canonical proved content, decoded vector and retained limits. Attempt
   replay with a bare echoed outcome/digest, without authenticated observation, with another
   artifact's observation and with a regenerated correct oracle replacing the proved arithmetic
   mutation. Each must refuse before QSL evaluation; QSL receives no artifact and cannot be
   credited with authenticating the driver's execution.
5. Call QSL's scalar arm in the public consumer, then pass its result to the CG converter.
   Compare generated/exact divergence, agreement despite retained Kani falsification, a QSL
   non-fault refusal and an executor fault. Distinguish native `Incomplete`/execution fault
   (`GeneratedFault`, `Failed`) from exact limit exhaustion (`Incomplete(ResourceExhausted)`);
   verify `RefusedInput` keeps `invalid_runtime_input` and other QSL refusals follow the typed map.
   Check all four supported integer operators and
   refuse an undefined/incomplete generated outcome as agreement; those outcomes do not
   satisfy [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-37. Change the
   actual generated arithmetic, not the
   claimed replay verdict. Error-path seam observations may use a recording/failing executor;
   the positive evaluation must use QSL and the actual generated artifact.
6. For every `Diverged`, `Agrees`, `GeneratedFault`, `RefusedInput`, exact `Incomplete` and
   other `Refused` report, compare its full `claim()` to the exact retained sent claim before
   terminal conversion. Bind another result/run to the first binding and require no settlement;
   repeat by changing only obligation identity, node, canonical generated-content identity,
   operator, operand value/range, result range and limits. Keep the obligation digest equal
   for the other mutations to prove digest-only checking cannot pass. A missing report claim
   refuses too; agreement-only identity is insufficient. These tests await the actual upstream
   amended spec/API; do not fabricate a full report or substitute agreement for other outcomes. Pass
   valid converter
   settlements to `run_terminal_value`; repeat with no settlement to reuse
   [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-15. Trace executable
   tests to each criterion they actually assert; this document alone provides no coverage.
7. Independently inspect the scalar canonical preimage through CG's one canonical encoder:
   authenticate application node/occurrence and parameter-operand node ids from the proving
   package, retain the actual harness domains and existing kind, and order arguments by the
   authoritative declared parameter identifier. Check a literal remains a singleton operand
   without becoming a parameter argument and exclude unrelated enclosing-function parameters.
   Change each included member and require an identity change; change source span, native
   outcome and canonical generated-content identity and require the same obligation identity.
   Try absent occurrence/kind/parameter correspondence and require typed refusal with no
   identity. A repeated parameter drawn independently at two positions remains a CODE gate
   until its authoritative binding rule is resolved. No invented metadata closes that check.

## Expected Results

| Criterion | Planned observation | Mutation that must fail the check |
|---|---|---|
| FR-032-AC-1 | Public typed consumer uses scalar identity; an otherwise valid request reaches unsupported-capability after ordered setup checks, with no settlement | Require function identity or admit a synthetic predicate as the scalar route |
| FR-032-AC-2 | Adapter-decoded playback uses persisted order; builder rejects above 8 MiB before parsing | Decode in alphabetic order, ignore another harness name or discard extra bytes |
| FR-032-AC-3 | Operand endpoints and request operator/ranges agree with the harness/package; limits match original generation context, including AD-002 identities | Skip a bound check, replace operator/node, use result bounds for a literal or swap operands |
| FR-032-AC-4 | Driver authenticates same-proved-artifact observation against CG's retained record and checked completion ties operands/limits/content | Accept an unchecked outcome/digest echo, treat playback as a generated result or accept fresh regeneration that removes the mutation |
| FR-032-AC-5 | Driver executes the proved artifact and calls QSL between builder and converter; QSL measures the current four-operator completed-value/refused-outside-range projection | Execute inside the CG builder, substitute a local evaluator, stub the positive verdict, accept `Undefined`/`Incomplete` as agreement or add unproved refusal-cause/accounting-counter equality |
| FR-032-AC-6 | Full report claim and run match the retained sent claim on every outcome, or no settlement | Check only the obligation digest or agreement; omit node/content/operator/operand range/result range/limits on a refusal, exact Incomplete, generated fault or divergence |
| FR-032-AC-7 | Divergence/generated fault is `Failed`, agreement is `Inconclusive(ScalarAgrees)`, exact exhaustion is `Incomplete(ResourceExhausted)` and typed QSL refusal keeps its code; never `Refuted`/`Verified` | Convert every Kani falsification to `Reproduced`, fabricate a predicate disagreement or turn a fault into a data refusal |
| FR-032-AC-8 | Increment emission/result range is observed; the separate bounded-addition check derives the exact result from actual retained in-domain operands and asserts it is outside `[-1000,1000]`, with correct refusal/harness-defect agreement rather than source violation | Use a hand-built function harness, assume `(600,600)` instead of reading actual playback, substitute increment as the unreachable refusal case, relabel expected refusal as source falsification or accept wrong emitted fixture ranges |
| FR-032-AC-9 | Fixed O-09 members and authoritative ascending parameter order; literal is no parameter, missing metadata refuses, domains are those actually harnessed | Invent an occurrence/parameter id, include enclosing unrelated parameters or a content/outcome/tracking field, use generated-name order, or narrow a harness domain |

All checks are planned; completed-route observations have the same explicit upstream gates as
[FR-032](../functional/FR-032-routed-scalar-replay-binding.md). Scalar lowering replay never
produces a source-property `Refuted` or `Verified` terminal. The absent-settlement check verifies
[FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-15 directly, without
adding a duplicate FR-032 criterion. Missing upstream types/causes
must be reported as a gate, not implemented as local QSL lookalikes.
