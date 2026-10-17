---
id: SR-12003
title: "spec-review/evidence review of IR-796 TC-050 proof-bound disposition"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen; spec/oracle/matrix/TC-050-caller-text-admission.md, spec/oracle/functional/FR-035-caller-text-admission.md, spec/oracle/matrix/tests.md, spec/kani/matrix/tests.md @ f9164277dccc"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-050
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-035
    type: references
---
# SR-12003: spec-review/evidence, IR-796 disposition (CG #412)

## Summary

Ticket: IR-796. PR quire-contract-codegen#412 at f9164277dcccf08c03d2a7d23adbae4aaa6b5fde, diff against codex/ir-494-caller-text-admission.

Checks whether the disposition's measured claims are backed by retained, reproducible evidence. Reviewer reproduction (independent crate rendered from the PR #381 template, CG lock pins: runtime 3cc88c48, quire-exact 389a21ae; Kani/CBMC from the installed toolchain; --unwind 8, Cadical, systemd scope MemoryMax=1G swap 0): original NFC Text[1,1] harness MemoryExhausted at 19.2 s, peak 1,073,741,824 B; single rt::admit_text('e') unicode-scalars Text[0,8] MemoryExhausted at 24.8 s at 1 GiB and 481.3 s at a 4 GiB diagnostic cap; TextPayload::from_utf8('e') alone VERIFIED in 0.45 s at 90 MB; assert_eq! on two fresh Meters FAILED 'unwinding assertion loop 0' in <builtin-library-memcmp> at 0.30 s. Same harnesses on quire_exact::admit_text (389a21ae): orig MemoryExhausted 12.4 s; single 'e' admission MemoryExhausted 11.9 s at 1 GiB and 64.7 s at 4 GiB; Meter assert_eq! unwinding failure reproduced; accessor comparison of two Meters VERIFIED (runtime 0.51 s, quire-exact 0.37 s). Two diagnostic statements lack a receipt or phase evidence.

## Scope

- examined: TC-050 Measured Proof-Bound Disposition (shapes) (spec/oracle/matrix/TC-050-caller-text-admission.md:118-123): Every measured harness of every shape, including the empty payload under `binary-utf8` `Text[1,1]`, ended `MemoryExhausted` inside CBMC before any verification condition was solved, and the widened-type mutation control of the per-literal and original shapes also ended `MemoryExhausted` without a falsification or playback. The cost floor is therefore one `rt::admit_text` call on one concrete payload, not the 21-way selection:
- examined: TC-050 Measured Proof-Bound Disposition (floor) (spec/oracle/matrix/TC-050-caller-text-admission.md:125-129): Without the agreement comparison, a single `rt::admit_text` on the literal `e` under `unicode-scalars` `Text[0,8]` exhausts 1 GiB in CBMC symbolic execution. Under a diagnostic 8 GiB ceiling, the concrete single-literal agreement harness was still in symbolic execution at the 600-second limit, at 3.4 GiB.
- examined: TC-050 Measured Proof-Bound Disposition (counts) (spec/oracle/matrix/TC-050-caller-text-admission.md:130-135): Symbolic execution unwinds `core::str::count::do_count_chars` for a one-scalar payload, which is reached through the scalar counts in the runtime's Text `prepare` path and `TextProfile::length`. With those counts rewritten in a scratch runtime copy, symbolic execution of one admission finishes in about 5 seconds. Propositional conversion then needs about 2.6 GiB, and the full single-literal agreement harness exceeded 3.6 GiB.
- examined: TC-050 Measured Proof-Bound Disposition (memcmp) (spec/oracle/matrix/TC-050-caller-text-admission.md:136-138): Separately, `assert_eq!` on two `Meter` values compares the ten-counter array through CBMC's `memcmp` model, whose loop fails its unwinding assertion at `--unwind 8` even with no admission call.
- examined: TC-050 Measured Proof-Bound Disposition (no shape fits) (spec/oracle/matrix/TC-050-caller-text-admission.md:140-146): No harness shape that keeps the public constructor, the direct runtime admission call and full outcome and metering agreement fits the published ceiling with this runtime and toolchain. A narrower assertion, stubbing of runtime admission or metering, a raised ceiling or unwind bound, or a per-harness unwind override would weaken FR-035-AC-4 or FR-035-AC-5, and none is accepted as discharge. FR-035-AC-4 and FR-035-AC-5 remain planned and unrun, and they have no proof credit.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | The '8 GiB diagnostic ... at 3.4 GiB' figure has no retained peak receipt: the author's runs/conc_us08_s01_v/cbmc-8g.time is empty, and 8 GiB exceeds the 4 GiB phase-diagnostic cap. Under the machine rules a reviewer cannot reproduce it. Failure scenario: a later owner cites 3.4 GiB as a measured floor that nobody can recheck. | spec/oracle/matrix/TC-050-caller-text-admission.md:125-129 | correct-requirement-no-evidence |
| FND-002 | low | 'ended MemoryExhausted inside CBMC before any verification condition was solved' is asserted for every measured harness. Three author runs (conc_us08_s00, part3_nfc11_p00, mut_conc_nfc11_s05) ended with cargo-kani killed (exit 137) and no Kani verdict line, so the phase evidence for those is missing. Failure scenario: a run that died in compilation or goto handling is recorded as a CBMC-phase stop. | spec/oracle/matrix/TC-050-caller-text-admission.md:118-123 | correct-requirement-no-evidence |

## Verdict

The central claims reproduce: the original harness OOMs at 1 GiB, one concrete admission OOMs at 1 GiB, the constructor alone verifies, and the Meter memcmp unwinding failure occurs. The impossibility also holds on the current kernel path (quire_exact::admit_text). Only low evidence gaps remain.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | Only two of the three named runs lack a verdict line. The per-literal `ee` mutation control (author run mut_conc_nfc11_s05, exit 137) does have one: its kani.log has `VERIFICATION:- FAILED` and the CBMC out-of-memory message. The miscount came from this reviewer's own SR-12003 FND-002, which listed that run wrongly. Failure scenario: the mutation control's MemoryExhausted verdict reads as unretained, which understates the evidence for the AC-5 control. | spec/oracle/matrix/TC-050-caller-text-admission.md:122-125 |

## Dispositions

Round 1, reviewed at b94bdd4e7f8d15e1d317391ac0fe07fd93bc9a02.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
