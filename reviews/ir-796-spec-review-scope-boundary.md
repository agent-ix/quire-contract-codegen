---
id: SR-12006
title: "spec-review/scope-boundary review of IR-796 TC-050 proof-bound disposition"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen; spec/oracle/matrix/TC-050-caller-text-admission.md, spec/oracle/functional/FR-035-caller-text-admission.md, spec/oracle/matrix/tests.md, spec/kani/matrix/tests.md @ f9164277dccc"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-050
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-035
    type: references
---
# SR-12006: spec-review/scope-boundary, IR-796 disposition (CG #412)

## Summary

Ticket: IR-796. PR quire-contract-codegen#412 at f9164277dcccf08c03d2a7d23adbae4aaa6b5fde, diff against codex/ir-494-caller-text-admission.

Allocation of the next owner. Runtime main 6036f70 (IR-349 #95) deleted src/exact/text.rs and the exact Meter and Integer, and Text admission and Meter now live in quire-exact. quire-exact 389a21ae, already in CG's lock, has the byte scalar_count (IR-797) and InlineSizes charge (IR-705). quire_exact::admit_text('e') still exhausts 1 GiB, and a direct CBMC diagnostic is dominated by num_bigint BigUint::from(u128) unwinding inside quire_exact Meter::charge. Meter equality is avoidable in the CG generator.

## Scope

- examined: TC-050 Measured Proof-Bound Disposition (next owner) (spec/oracle/matrix/TC-050-caller-text-admission.md:148-155): The next owner is Contract Runtime: the proof cost of the exact Text admission and metering path (`prepare` scalar counting, `Meter::charge` and `Meter` equality) has to come within the published ceiling under `--unwind 8` before CG can regenerate the twelve harnesses and the mutation control. That change is a separate runtime slice, followed by a CG generator slice that rebinds TC-050 step 3 to the reduced runtime. A Kani or CBMC change that keeps the concrete payload length constant through `Box<str>` construction is an alternative upstream route. Neither route alters this test's criteria.
- examined: TC-050 Measured Proof-Bound Disposition (counts) (spec/oracle/matrix/TC-050-caller-text-admission.md:130-135): Symbolic execution unwinds `core::str::count::do_count_chars` for a one-scalar payload, which is reached through the scalar counts in the runtime's Text `prepare` path and `TextProfile::length`. With those counts rewritten in a scratch runtime copy, symbolic execution of one admission finishes in about 5 seconds. Propositional conversion then needs about 2.6 GiB, and the full single-literal agreement harness exceeded 3.6 GiB.
- examined: TC-050 Measured Proof-Bound Disposition (memcmp) (spec/oracle/matrix/TC-050-caller-text-admission.md:136-138): Separately, `assert_eq!` on two `Meter` values compares the ten-counter array through CBMC's `memcmp` model, whose loop fails its unwinding assertion at `--unwind 8` even with no admission call.
- examined: FR-035 Dependencies Proof cost (spec/oracle/functional/FR-035-caller-text-admission.md:121-127): FR-035-AC-4 and FR-035-AC-5 depend on Contract Runtime's Text admission and metering path completing in Kani within the published 1 GiB ceiling under `--unwind 8`. IR-796 measured that it does not, for any harness shape that keeps these criteria. The disposition and its next owner are recorded in TC-050's Measured Proof-Bound Disposition section. The criteria are unchanged and stay planned and unrun until that dependency is met.
- examined: Oracle matrix FR-035 row (spec/oracle/matrix/tests.md): AC-4/5 are unrun and have no real-Kani proof credit. IR-796 measured that no criterion-preserving harness shape fits the published 1 GiB / `--unwind 8` bound, because one concrete `admit_text` call already exceeds it. TC-050 records the disposition and names Contract Runtime as next owner; twelve verified harnesses and the falsifying mutation control remain open.
- examined: Oracle matrix TC-050 row (spec/oracle/matrix/tests.md): PR #381 candidate exists, but the measured NFC `Text[1,1]` run is inconclusive, IR-796's measured proof-bound disposition places the cost floor in runtime admission and metering (next owner: Contract Runtime), and the twelve-harness proof and mutation control are unrun and uncredited
- examined: Kani matrix FR-035 row (spec/kani/matrix/tests.md): IR-796 measured that per-literal, partitioned and byte-table shapes all exhaust the published bound, so TC-050 records a proof-bound disposition with Contract Runtime as next owner.
- examined: Kani matrix TC-050 row (spec/kani/matrix/tests.md): one original NFC `Text[1,1]` real-Kani run is inconclusive after `MemoryExhausted`, and IR-796's measured disposition shows no criterion-preserving shape fits the published bound (next owner: Contract Runtime); twelve verified harnesses, falsifying control and production proof evidence remain open

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | Contract Runtime is named as next owner of 'prepare scalar counting, Meter::charge and Meter equality', but runtime main (6036f70, IR-349 #95) removed that code, and Text admission and Meter are now owned by quire-exact. quire-exact 389a21ae, already pinned in CG's lock, has the IR-797 byte scalar_count and the IR-705 InlineSizes charge. Reviewer run: quire_exact::admit_text('e') still OOMs at 1 GiB (11.9 s) and 4 GiB (64.7 s), with BigUint::from(u128) dominating symex in Meter::charge. Failure scenario: a runtime slice is opened against code that no longer exists on runtime main, while the residual cost sits in quire-exact Integer accounting and CG stays on a stale runtime pin. | spec/oracle/matrix/TC-050-caller-text-admission.md:148-155 | wrong-requirement |
| FND-002 | medium | Meter equality is assigned to the runtime, but it is a CG generator defect. Both meters come from unlimited_meter() with no injected denial, and admit() only advances denial_point_seen when denial is Some, so the private fields are None/0 in both by construction. Comparing limits(), consumed(kind) x10, admitted_charges() and charge_log_truncated() covers every field that can differ and is not narrower than AC-4's 'admitted charges and consumed counters'. Reviewer run: the accessor comparison VERIFIES at unwind 8 (0.51 s), while assert_eq! fails memcmp unwinding. Failure scenario: CG waits on a runtime API change it does not need. | spec/oracle/matrix/TC-050-caller-text-admission.md:136-138 | wrong-requirement |
| FND-003 | medium | The shared root with IR-705 is neither stated nor linked: the Integer encoding in Meter::charge exhausting Kani memory. The author's patched diagnostic shows Meter::charge, drop glue of (LimitKind, Integer) and BigUint::to_u64 loops. On quire-exact the cost is BigUint::from(u128) in Meter::charge, the class IR-705 owns. Failure scenario: two lanes fix the same Integer/Meter cost independently, or the TC-050 owner never learns that IR-705's fix did not cover this path. | spec/oracle/matrix/TC-050-caller-text-admission.md:130-135 | missing-requirement |

## Verdict

Changes requested. Re-allocate: the residual kernel cost (Integer encoding in Meter::charge) goes to quire-exact under or linked to IR-705. The runtime pin and route moving off the deleted runtime text API, plus the accessor-based Meter comparison, go to CG.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | FR-035:21 still says the oracle 'calls Contract Runtime's own `exact::admit_text` API', which contradicts the new Upstream bullet (FR-035:114-116, quire-exact). Behavior and ACs (FR-035:69, 86, 105) use a bare `rt::admit_text` with nothing binding `rt`. The generator binds `rt` to `quire_contract_runtime::exact` (PR #381 src/oracle/caller_text.rs:143), and at the locked runtime revision that is a separate copy, not quire-exact. Failure scenario: an IR-837 implementer follows the Description and keeps the deleted runtime path. Or a reader takes `rt` as quire_exact and concludes the current harness already exercises the kernel. | spec/oracle/functional/FR-035-caller-text-admission.md:21, 69, 86, 105 |

## Dispositions

Round 1, reviewed at b94bdd4e7f8d15e1d317391ac0fe07fd93bc9a02.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |

Round 2, reviewed at bed849da9fbc39b9fcef4a4e654e6362d2baae65 (IR-705 reopened, In Progress).

| FND | outcome | reason |
| --- | --- | --- |
| FND-004 | fixed | - |


## Standalone follow-up

This is a standalone historical measured disposition, not implementation acceptance for draft #381. No caller_text.rs exists on main. The six FR-035 criteria, symbolic length/two alphabet-index encoding, TC-050 procedure and expected results are unchanged from main. All six criteria compute untagged; this pre-existing planned implementation gap is owned by IR-494/#381 and is not new coverage or proof credit. No Cargo, Kani or historical measurement was run. No applicable AssuranceProfile exists in spec/.

PASS for the changed boundary context, subject to SR-12003 FND-004. The exact API is specified as quire_exact, the separate runtime binding is explicitly attributed to draft #381, and its generator is expressly absent on main. CG owns future generation/binding/accessor work under IR-837; quire-exact owns the measured Meter integer cost under IR-705. External kernel behavior is assumed here, with eventual real agreement tests required by TC-050; not presently guaranteed. Original FND-001..004 remain fixed.

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
