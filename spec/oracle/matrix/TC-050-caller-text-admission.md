---
id: TC-050
title: "Verify caller TextPayload admission and bounded Kani agreement"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-035
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: verifies
---
# TC-050: Verify caller TextPayload admission and bounded Kani agreement

## Description

Verify the caller Text admission oracle, its constructor boundary, runtime
accounting agreement and finite real-Kani claim separately from checked-package
text comparison and the six refused text-operand conversion nodes.

## Test Procedure

1. For every `TextProfile`, generate the caller-ingress oracle with
   `Text[0,8]` and `Text[1,1]`. Construct runtime-provenance payloads through
   `TextPayload::from_utf8` for empty, `e`, U+0301, U+00E9, U+1F600, and
   `e` followed by U+0301. Run the generated oracle and a separate direct
   runtime `admit_text` call with fresh equal meters. Compare the result kind,
   retained sequence, source payload and provenance on completion, admitted
   charges and every consumed counter (FR-035-AC-1, AC-3).
2. Present malformed UTF-8 to `TextPayload::from_utf8`; assert
   `InvalidUtf8`, no payload, no oracle call and no meter change. Deny each
   charge point reached by a valid payload in turn, including normalization
   points under a normalizing profile, and compare generated and direct stops,
   charges and counters. Check that bound refusal precedes
   `text.result-retain` (FR-035-AC-2, AC-3).
3. Generate twelve real-Kani harnesses: every profile crossed with both
   `Text[0,8]` and `Text[1,1]`. Draw length `0..=2` and two alphabet indices
   `0..4` symbolically. Encode zero, one or two selected scalars from
   `{e, U+0301, U+00E9, U+1F600}` as UTF-8, construct the public
   `TextPayload`, and compare the generated oracle with a separate direct
   runtime call. Check that each harness identity records its class, profile
   and bound, each has exactly one final cover, and run each through the
   FR-017 production executor against the installed Kani backend. Check
   `Verified` with the cover satisfied, evidence bound to the caller identity
   and selected harness path, and `None` for contract obligation kind
   (FR-035-AC-4, FR-017-AC-26).
4. Mutate only the generated admission call or its declared type in a copy so
   an out-of-bound member of the symbolic class is accepted. Rerun real Kani;
   require `Falsified` with a concrete playback for that harness's agreement
   assertion. Restore the generated call and verify with the same harness and
   options (FR-035-AC-5, FR-017-AC-26).
5. Through the same production executor, run the caller harness with a cover
   the bounded class cannot satisfy, a source-mismatched crate, an absent
   launcher and a malformed backend report. Require `CoverUnsatisfied` with
   counts for the cover, the established typed refusal for each setup/report
   failure, and no verified caller claim. A report with no cover and a failed
   assertion without matching playback are inconclusive. In a compatible
   mixed batch, bind each outcome and playback to its own harness path and
   caller identity (FR-017-AC-26).
6. Request an unbounded or unsupported payload class. Check its typed
   disposition, absent harness and absence of `ir_confirmed` or checked node
   identity. Retain the TC-024 check that all six text-operand
   `numeric.convert` attempts are refused by Contract IR; none is a successful
   TC-050 vector (FR-035-AC-6).

## Expected Results

The generated caller-ingress oracle and direct runtime call agree on each
admitted payload, profile, bound and injected charge failure. Invalid UTF-8
stops at payload construction before admission. The twelve real-Kani harnesses
cover exactly the stated 21-sequence finite class and verify with satisfied
covers; the changed admission call is falsified with concrete playback and the
restored call verifies. The production evidence retains the caller identity;
unsatisfied covers and infrastructure failures cannot become verified caller
claims. Unsupported proof requests produce no harness. This
test does not count refused checked-package conversions as admission proofs.

## Proof Boundary and Current Disposition

A `MemoryExhausted` Kani result is inconclusive and gives no proof credit for
FR-035-AC-4 or FR-035-AC-5. TC-050 remains open until all twelve harnesses
verify over the full symbolic 21-sequence class through the public
`TextPayload::from_utf8` constructor, with generated-versus-direct runtime
agreement and a satisfied final cover, and the changed-call control is
falsified with concrete playback before the restored call verifies over that
same class. A fixed literal, a static enumeration, a branch-local control or
a higher resource ceiling does not discharge these criteria. An inconclusive
run alone does not establish impossibility; the measured disposition below
records the impossibility IR-796 established for the current kernel path and
the tickets that own the next change.

Non-normative observation: IR-796 records that the original NFC `Text[1,1]`
candidate from PR #381 reached CBMC and ended `MemoryExhausted` under the
published 1 GiB ceiling after 72.021 seconds, at a measured peak of
1,089,802,240 bytes and with zero successful checks. Its phase diagnostics
place the observed cost in symbolic UTF-8 validation and scalar-count
unwinding before SAT. This one run does not settle the other eleven harnesses
or prove that a sound design cannot complete within the ceiling.

## Measured Proof-Bound Disposition (IR-796)

IR-796 measured whether a different harness shape could satisfy FR-035-AC-4
and FR-035-AC-5 under the unchanged published bound of 1 GiB per obligation,
600 seconds, `--unwind 8` and Cadical. The toolchain and dependency revisions
of each run are recorded in the IR-796 ticket receipts, not in this test.
Three shapes were measured in a scratch copy of the generated harness, each
keeping the public `TextPayload::from_utf8` constructor, the
generated-versus-direct agreement assertions and the final cover:

- one concrete literal per harness, so the harness set rather than a selector
  ranges over the 21 members;
- a symbolic selector over a partition of three members per harness;
- one symbolic selector over a constant 21-row byte table feeding a single
  `from_utf8` call.

Every measured harness of every shape, including the empty payload under
`binary-utf8` `Text[1,1]`, exhausted the 1 GiB ceiling. Where Kani reported a
verdict, it was `MemoryExhausted` in CBMC before any verification condition
was solved. Three runs, the empty per-literal `unicode-scalars` `Text[0,8]`
harness, the first three-member NFC `Text[1,1]` partition and the per-literal
`ee` mutation control, were killed by the cap with no verdict line, and their
phase was not retained. The widened-type mutation controls produced no
falsification or concrete playback. The cost floor is one admission call on
one concrete payload, not the 21-way selection:

- A single admission of the literal `e` under `unicode-scalars` `Text[0,8]`,
  with no agreement comparison, exhausts 1 GiB. The IR-796 review reproduced
  this for both the Contract Runtime admission path and the current
  `quire_exact::admit_text` kernel path, and at a 4 GiB diagnostic ceiling
  the single admission still ended `MemoryExhausted`, after 481 seconds on
  the runtime path and 65 seconds on the kernel path.
- On the kernel path, symbolic execution is dominated by `BigUint::from(u128)`
  unwinding inside the exact `Meter::charge`, the same integer-encoding cost
  class tracked by IR-705-AC-9.
- Separately, the generated harness's `assert_eq!` on two `Meter` values
  compares the ten-counter array through CBMC's `memcmp` model, whose loop
  fails its unwinding assertion at `--unwind 8` even with no admission call.
  Comparing the two meters through their public accessors verifies. This is
  a defect of the generated harness, not of the kernel.

No harness shape that keeps the public constructor, the direct admission call
and full outcome and metering agreement fits the published ceiling with the
current kernel and toolchain. A narrower assertion or a stub of admission or
metering would weaken FR-035-AC-4 or FR-035-AC-5. A raised memory ceiling, a
higher global unwind bound or a per-harness unwind override would change the
published bound rather than meet it. None of these is accepted as discharge.
FR-035-AC-4 and FR-035-AC-5 remain planned and unrun, and they have no proof
credit.

The next owners are tracked tickets. quire-exact owns the remaining integer
cost of exact metering under IR-705, whose AC-9 covers the same encoding
class, so a reduction there is a precondition for this proof. CG owns IR-837,
which moves the TC-050 harness onto the quire-exact admission path and
compares meters through public accessors instead of `Meter` equality. The
twelve harnesses and the mutation control are rerun under this test's
unchanged criteria once both have landed.
