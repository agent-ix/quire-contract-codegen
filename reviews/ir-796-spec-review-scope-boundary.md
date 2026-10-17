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
