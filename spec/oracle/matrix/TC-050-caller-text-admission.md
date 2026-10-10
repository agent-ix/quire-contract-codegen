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
