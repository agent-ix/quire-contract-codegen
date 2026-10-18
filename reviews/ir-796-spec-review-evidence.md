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

Round 2, reviewed at bed849da9fbc39b9fcef4a4e654e6362d2baae65 (IR-705 reopened, In Progress).

| FND | outcome | reason |
| --- | --- | --- |
| FND-003 | fixed | - |


## New findings (standalone follow-up)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | Historical measurements are presented as a result for the current kernel/toolchain after the standalone rebase. TC-050 says no shape fits with the 'current kernel and toolchain' and its boundary calls this an established impossibility for the 'current kernel path'; the measured SR-12003 receipt used an older quire-exact dependency than this branch's Cargo.lock. FR-035's proof-cost paragraph and both matrix summaries inherit that present-tense scope. This can wrongly rule out proof work against an unmeasured dependency. Attribute these conclusions to the kernel/toolchain measured in IR-796, without a new pin, experiment or current impossibility claim; preserve the historical observations, bounds and planned/uncredited criteria. | spec/oracle/matrix/TC-050-caller-text-admission.md:85-87, 128-130, 144-146; spec/oracle/functional/FR-035-caller-text-admission.md:123-129; both FR-035/TC-050 matrix rows |


## Standalone follow-up

This is a standalone historical measured disposition, not implementation acceptance for draft #381. No caller_text.rs exists on main. The six FR-035 criteria, symbolic length/two alphabet-index encoding, TC-050 procedure and expected results are unchanged from main. All six criteria compute untagged; this pre-existing planned implementation gap is owned by IR-494/#381 and is not new coverage or proof credit. No Cargo, Kani or historical measurement was run. No applicable AssuranceProfile exists in spec/.

FAIL for the refreshed publication context: one new evidence-scope defect, FND-004. Original FND-001..003 latest fixed outcomes remain unchanged. Historical measured-section bytes and original receipts are retained; they were not reproduced. The changed dependency invalidates an unqualified present-tense inference, not the historical observations. Advisor output is retained as method-selection context only; no authored Verification cells changed.

### Follow-up scope

- {excerpt: 'When a caller requests runtime Text admission, the code generator shall emit an

    oracle and a separately bounded Kani obligation for a caller-owned

    `TextPayload` and declared `TextType`. This is a caller-ingress claim, not a

    claim about an expression node in an admitted `CheckedPackageV2`. The oracle

    calls the exact kernel''s `quire_exact::admit_text` API and returns its

    outcome and metering unchanged. Its evidence identifies the caller-declared

    profile, inclusive length bounds, payload class and proof configuration,

    without `ir_confirmed` provenance, a checked node id or a QSL term identity.


    I', id: FR-035/Description, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}
- {excerpt: "- A caller-supplied `TextType::new(min, max, profile)` with nonempty inclusive\n  bounds and one of the six `TextProfile` values.\n- A runtime `TextPayload` made by the public `TextPayload::from_utf8(&[u8])`\n  constructor, and a caller-supplied `Meter`. Invalid UTF-8 bytes are refused\n  by that constructor before a `TextPayload` exists or admission is called.\n- For proof, a finite symbolic source class: zero, one or two Unicode scalars\n  chosen independently from `e`, U+0301 COMBINING ACUTE ACCENT, U+00E9 LATIN\n  SMALL LETTER E WITH ACUTE, and U+1F600 GRINNING FACE. The harness encodes\n  the sel", id: FR-035/Inputs, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}
- {excerpt: "- When given a valid `TextPayload` and `TextType`, the emitted oracle shall\n  call `rt::admit_text(&payload, &text_type, meter)` and return its `Outcome<Text>`\n  unchanged. It shall neither pre-normalize the payload nor substitute an\n  unmetered local bound check.\n- When `TextPayload::from_utf8` rejects invalid bytes, the caller-ingress\n  route shall expose that constructor refusal before it can call admission;\n  the meter shall not be charged by admission. The generated oracle takes a\n  `TextPayload`, so this refusal is not an `admit_text` outcome.\n- When an admitted payload violates the decl", id: FR-035/Behavior, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}
- {excerpt: "- **Upstream**: quire-exact, the exact kernel this repository locks, which\n  defines and exports `quire_exact::admit_text`, `quire_exact::TextPayload`,\n  `quire_exact::TextType` and `quire_exact::Meter`.\n- **Execution**: [FR-017](../../kani/functional/FR-017-kani-execution-evidence.md)\n  classifies and retains evidence for the generated caller harness.\n- **Related**: [FR-014](./FR-014-exact-scalar-oracles.md) for checked-package\n  text comparison. QSL-694 explores text-to-number parsing inside a term; it\n  does not gate this caller-ingress operation.\n- **Proof cost**: With the current kernel a", id: FR-035/Dependencies, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}
- {excerpt: 'A generated caller-ingress oracle accepts `&TextPayload`, the declared `TextType` and `&mut Meter`, calls `rt::admit_text` once, and returns the runtime outcome, retained text, original runtime provenance, charge sequence and consumed counters unchanged for all six profiles over representative empty, ASCII, combining, composed and supplementary payloads.', id: FR-035-AC-1, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}
- {excerpt: 'Invalid UTF-8 bytes make `TextPayload::from_utf8` return `InvalidUtf8` before admission and leave the admission meter unchanged; `admit_text` itself is never claimed to accept those bytes.', id: FR-035-AC-2, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}
- {excerpt: 'For each profile, `Text[1,1]` admits or refuses according to the profile length of the retained sequence, including `e` plus U+0301 under NFC and NFD; a length refusal occurs before `text.result-retain`, and denial of each reached charge point yields the direct runtime stop, charges and consumed counters.', id: FR-035-AC-3, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}
- {excerpt: 'For each of the six profiles and both `Text[0,8]` and `Text[1,1]`, a real Kani harness symbolically ranges over all 21 sequences of zero to two scalars from the four-scalar alphabet, constructs each `TextPayload` by `from_utf8`, compares the generated admission oracle with a separate direct runtime call including outcome and metering, and ends with one satisfied cover after its assertions. The proof identity names this finite class and its type.', id: FR-035-AC-4, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}
- {excerpt: 'In a copy of the generated oracle with its admission call or declared `TextType` altered so the oracle accepts a payload the direct runtime call refuses, real Kani falsifies the admission agreement assertion with concrete playback; restoring the call verifies over the same symbolic class.', id: FR-035-AC-5, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}
- {excerpt: 'A request outside the declared finite proof class has a typed unsupported or requires-bound disposition and no harness, and no caller-ingress claim is marked `ir_confirmed` or assigned a checked-package node id. The six attempted text-operand `numeric.convert` nodes remain Contract IR refusals and do not count as passing caller-admission coverage.', id: FR-035-AC-6, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}
- {excerpt: "1. For every `TextProfile`, generate the caller-ingress oracle with\n   `Text[0,8]` and `Text[1,1]`. Construct runtime-provenance payloads through\n   `TextPayload::from_utf8` for empty, `e`, U+0301, U+00E9, U+1F600, and\n   `e` followed by U+0301. Run the generated oracle and a separate direct\n   runtime `admit_text` call with fresh equal meters. Compare the result kind,\n   retained sequence, source payload and provenance on completion, admitted\n   charges and every consumed counter (FR-035-AC-1, AC-3).\n2. Present malformed UTF-8 to `TextPayload::from_utf8`; assert\n   `InvalidUtf8`, no payload, ", id: TC-050/Test Procedure, path: spec/oracle/matrix/TC-050-caller-text-admission.md, role: examined}
- {excerpt: 'The generated caller-ingress oracle and direct runtime call agree on each

    admitted payload, profile, bound and injected charge failure. Invalid UTF-8

    stops at payload construction before admission. The twelve real-Kani harnesses

    cover exactly the stated 21-sequence finite class and verify with satisfied

    covers; the changed admission call is falsified with concrete playback and the

    restored call verifies. The production evidence retains the caller identity;

    unsatisfied covers and infrastructure failures cannot become verified caller

    claims. Unsupported proof requests produce no harness. This

    te', id: TC-050/Expected Results, path: spec/oracle/matrix/TC-050-caller-text-admission.md, role: examined}
- {excerpt: 'A `MemoryExhausted` Kani result is inconclusive and gives no proof credit for

    FR-035-AC-4 or FR-035-AC-5. TC-050 remains open until all twelve harnesses

    verify over the full symbolic 21-sequence class through the public

    `TextPayload::from_utf8` constructor, with generated-versus-direct runtime

    agreement and a satisfied final cover, and the changed-call control is

    falsified with concrete playback before the restored call verifies over that

    same class. A fixed literal, a static enumeration, a branch-local control or

    a higher resource ceiling does not discharge these criteria. An inconclusive

    run', id: TC-050/Proof Boundary and Current Disposition, path: spec/oracle/matrix/TC-050-caller-text-admission.md, role: examined}
- {excerpt: "IR-796 measured whether a different harness shape could satisfy FR-035-AC-4\nand FR-035-AC-5 under the unchanged published bound of 1 GiB per obligation,\n600 seconds, `--unwind 8` and Cadical. The toolchain and dependency revisions\nof each run are recorded in the IR-796 ticket receipts, not in this test.\nThree shapes were measured in a scratch copy of the generated harness, each\nkeeping the public `TextPayload::from_utf8` constructor, the\ngenerated-versus-direct agreement assertions and the final cover:\n\n- one concrete literal per harness, so the harness set rather than a selector\n  ranges over", id: TC-050/Measured Proof-Bound Disposition (IR-796), path: spec/oracle/matrix/TC-050-caller-text-admission.md, role: examined}
- {excerpt: "| FR-035 | FR-035-AC-1 through FR-035-AC-6 | TC-050 | \U0001F6A7 Planned (IR-494, IR-796); PR #381 has a generated caller-ingress oracle and symbolic harness candidate, but the original NFC `Text[1,1]` Kani run ended `MemoryExhausted`. AC-4/5 are unrun and have no real-Kani proof credit. IR-796 measured that no criterion-preserving harness shape fits the published 1 GiB / `--unwind 8` bound, because one concrete `admit_text` call already exceeds it. TC-050 records the disposition; the next owners are IR-705 (quire-exact metering integer cost) and IR-837 (CG harness on quire-exact admission, accessor me", id: FR-035/status, path: spec/oracle/matrix/tests.md, role: examined}
- {excerpt: "| TC-050 | Verify caller TextPayload admission and bounded Kani agreement | Integration | P0 | FR-035-AC-1, FR-035-AC-2, FR-035-AC-3, FR-035-AC-4, FR-035-AC-5, FR-035-AC-6, FR-017-AC-26 | \U0001F6A7 Planned (IR-494, IR-796); PR #381 candidate exists, but the measured NFC `Text[1,1]` run is inconclusive, IR-796's measured proof-bound disposition places the cost floor in one concrete admission and its exact metering (next owners: IR-705, IR-837), and the twelve-harness proof and mutation control are unrun and uncredited |", id: TC-050/status, path: spec/oracle/matrix/tests.md, role: examined}
- {excerpt: "| FR-035 | FR-035-AC-4, FR-035-AC-5, FR-035-AC-6 | TC-050 | \U0001F6A7 Planned (IR-494, IR-796); PR #381 has a symbolic caller `TextPayload` harness candidate, but the original NFC `Text[1,1]` Kani run ended `MemoryExhausted` and AC-4/5 remain unrun and uncredited. IR-796 measured that per-literal, partitioned and byte-table shapes all exhaust the published bound, so TC-050 records a proof-bound disposition with IR-705 and IR-837 as next owners. This route retains its own caller-ingress identity, separate from FR-015's checked-package obligations and FR-025's subject ABI |", id: FR-035/status, path: spec/kani/matrix/tests.md, role: examined}
- {excerpt: "| TC-050 | Verify caller TextPayload admission and bounded Kani agreement | Integration | P0 | FR-035-AC-4, FR-035-AC-5, FR-035-AC-6, FR-017-AC-26 | \U0001F6A7 Planned (IR-494, IR-796); one original NFC `Text[1,1]` real-Kani run is inconclusive after `MemoryExhausted`, and IR-796's measured disposition shows no criterion-preserving shape fits the published bound (next owners: IR-705, IR-837); twelve verified harnesses, falsifying control and production proof evidence remain open |", id: TC-050/status, path: spec/kani/matrix/tests.md, role: examined}


## Dispositions (standalone follow-up)

Round 3. Rechecked only unresolved FND-004 against the coder fix.

| FND | outcome | reason |
| --- | --- | --- |
| FND-004 | fixed | TC-050 limits the observations and conclusions to the IR-796 measured kernel/toolchain and expressly excludes later dependencies/toolchains; FR-035 proof-cost wording and all four matrix status rows have matching historical attribution. Numeric bounds, observations, control descriptions, planned/uncredited criteria and current main procedure remain unchanged. No experiments or current proof credit were added. |

## Final standalone verdict

PASS for publication of this historical disposition. The code, EARS, base, integrity and scope-boundary contexts were inspected for impact from the four-file prose fix; no new defect or normative change was found, so no unchanged method or historical measurement was repeated. All twelve original terminal findings remain preserved. This is not acceptance of draft #381 or completed IR-796; required implementation and full CI/Kani gates remain open. No branch edits, Cargo or Kani runs.
