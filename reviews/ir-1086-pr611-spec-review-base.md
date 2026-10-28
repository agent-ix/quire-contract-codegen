---
id: SR-611030
title: "Base spec review of IR-1086 PR 611"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/replay/functional/FR-024-counterexample-envelope-intake.md (FR-024-AC-30); spec/replay/matrix/TC-035-counterexample-envelope-intake.md (step 27)"
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-024
  type: references
- target: ix://agent-ix/quire-contract-codegen/TC-035
  type: references
---

## Summary

Ticket: IR-1086. PR: quire-contract-codegen#611 (draft), exact head
fda70b9fd76841a15a0f91681984a5ca48b48379, merge base and origin/main fb50acdc.
Spec-only diff: one phrase in FR-024-AC-30 and the matching clause of TC-035 step 27.

Before: "the caller supplying the harness's `StateFrameIdentity`, the playback text and the
inputs of FR-024's Inputs list but no obligation identity and no transcript".
After: "the caller supplying the harness's `StateFrameIdentity`, the parsed playback produced
from the complete retained backend output by the bounded Kani output reader, and the inputs of
FR-024's Inputs list but no obligation identity and no transcript". A byte comparison confirmed
this is the only change in the AC-30 row; the installed-backend (`make kani`) requirement, the
forbidden-write reproduced violation naming the written field, the granted-field verified/no
playback clause, the QSL emitted-package scope node identity clause, the `Twin::aligned` removal,
the no-obligation-identity and no-transcript clauses, and the named test all remain verbatim.

Lenses: base checklist here; integrity in SR-611031. EARS conformance was not run: no FR, NFR or
StR statement changed, only an AC row and a TC step. Code review and gap analysis were not run:
no production or test code changed. No AssuranceProfile document exists in the repository, so no
review policy applies. Validation: `make spec` passed (pre-existing advisory EARS warnings in
FR-017 and FR-014 only); `quoin validate --repo . --json` reported no findings.

## Verdict

**CHANGES REQUESTED (owner decision item).** The amendment does not narrow AC-30's verdict,
identity or no-transcript obligations, and it moves the parse to the caller in the direction
AD-004 L-6 and its Interim exception for `FrameReplayInputs::playback` already prescribe.
Two new phrases are ambiguous as written: what "the complete retained backend output" is, and
which bounds "the bounded Kani output reader" applies and whether `FrameReplay` re-enforces them.
The sibling state-clause slice resolved both explicitly; AC-30 does not.

## Scope

```yaml
scope:
- id: FR-024-AC-30
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: "With the installed backend, the real playback of the falsified frame harness of a subject that writes a forbidden field is replayed through `FrameReplay::new` and `replay`, the caller supplying the harness's `StateFrameIdentity`, the parsed playback produced from the complete retained backend output by the bounded Kani output reader, and the inputs of FR-024's Inputs list but no obligation identity and no transcript, and settles a reproduced violation naming the written field."
- id: TC-035-step-27
  path: spec/replay/matrix/TC-035-counterexample-envelope-intake.md
  role: examined
  excerpt: "With the installed backend (`make kani`), replay the real playback of the falsified frame harness of a subject that writes a forbidden field, supplying the harness's identity, the parsed playback produced from the complete retained backend output by the bounded Kani output reader and the Inputs list, but no obligation identity and no transcript."
- id: TC-035-expected-27
  path: spec/replay/matrix/TC-035-counterexample-envelope-intake.md
  role: examined
  excerpt: "The forbidden-write playback settles a reproduced violation naming the written field (FR-024-AC-30)."
- id: AD-004-L-6
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: context_only
  excerpt: "Public or replay-facing entries in `replay/function.rs` and `replay/frame.rs` still accept transcript text and call the output parser before decoding. ... L-6 remains partial until each entry takes parsed playback at its boundary, as itemized under Interim exceptions."
- id: FR-024-current-state-state-clause
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: context_only
  excerpt: "It passes the full falsified counterexample text to `parse_assertion_playback` in `kani/output/playback.rs`. The parser receives the smaller requested and run byte and value limits before reading the text, preserving its multiple-assertion refusal and full-text `source_len`. ... The public method recomputes those effective limits from its requested limits and `run.decode_limits`, even if supplied a `Playback` parsed under wider bounds."
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | "the complete retained backend output" is defined nowhere; it can mean the full console capture or `KaniRunOutcome::Falsified.counterexample`, which holds only the selected playback test | spec/replay/functional/FR-024-counterexample-envelope-intake.md:481; src/kani/classify.rs:64-67; src/kani/classify.rs:251-253 |
| FND-002 | medium | "the bounded Kani output reader" names no bounds and does not require `FrameReplay` to re-enforce them, so a caller parse under wider limits satisfies AC-30 | spec/replay/functional/FR-024-counterexample-envelope-intake.md:481; src/replay/frame.rs:339-363 |

## Detail

FND-001. Code at the merge base: `KaniRunOutcome::Falsified { counterexample }` is documented as
"The concrete-playback test Kani printed for the failed check, verbatim" and is filled from
`counterexample.test()` after classification selects a block. The state-clause slice words its
input as "the falsified outcome's complete counterexample text". AC-30's new "complete retained
backend output" is a new term. One reading parses the full stdout capture, which can hold more
than one assertion block and hits `parse_assertion_playback`'s multiple-assertion refusal;
the other parses `Falsified.counterexample`. Both implementers would claim to satisfy AC-30.
Suggested fix: name the source, for example "the falsified outcome's complete counterexample
text", matching the state-clause wording.

FND-002. Today `decoded_state` parses under `run.decode_limits`, so `FrameReplay::new` enforces
the run's byte and value ceilings itself. Once the caller parses, AC-30 says only "bounded"; no
FR-024 AC states the frame path's limits, and AC-30 does not say whether `FrameReplay::new`
recomputes the minimum of requested and `run.decode_limits` and rechecks `source_len` and value
count on a supplied `Playback`, as the state-clause method must. IR-1086's own acceptance text
(untrusted ticket data) expects byte-limit refusals and `source_len` to be preserved; the AC as
amended does not oblige either. This is the one place the amendment can weaken the effective
guarantee. Suggested fix: state the effective limits and that `FrameReplay::new` re-enforces
them on the supplied playback, or cite the AC that does.
