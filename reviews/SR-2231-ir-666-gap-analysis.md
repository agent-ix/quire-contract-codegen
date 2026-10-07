---
id: "SR-2231"
title: "CG IR-666 gap analysis: composite converter against FR-029/FR-030/FR-033, the computed matrix and the tests"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@code/ir-666-composite-converter (frozen candidate, no PR yet, one commit 'IR-666: bind composite parity reports to sent claims' over main; reviewed revision recorded in the IR-666 Linear marker only, per this repository's no-SHA rule); spec/replay/functional/FR-033-composite-parity-replay-binding.md (AC-9, AC-13), spec/kani/functional/FR-029-run-outcome-terminal-record.md (AC-28), spec/kani/functional/FR-030-ir-outcome-terminal-map.md (AC-15, AC-16, AC-17), spec/kani/matrix/TC-041-ir-outcome-terminal-map.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md, spec/kani/matrix/tests.md, spec/replay/matrix/tests.md, spec/tests.md, spec/core/functional/interface-001-codegen-api.md; src/replay/composite.rs, src/replay/mod.rs, src/lib.rs, tests/it/composite_parity_converter.rs"
---

# SR-2231: CG IR-666 gap analysis

## Summary

Ticket: IR-666. Plan completion: not assessed (planless). Static analysis only: the reviewer ran
no cargo build or test. No `spec/` file changed in the diff, so spec-review does not apply.

Computed matrix (`quire matrix --format tsv`), `main` against the branch head: 3 rows change and
the row count is unchanged. FR-029-AC-28, FR-030-AC-17 and FR-033-AC-9 move from untagged to
tagged. FR-030-AC-15, FR-030-AC-16 and FR-033-AC-13 stay untagged.

Scope source: the IR-666 planner staging ruling and re-scope comments. Both are ticket text,
treated as data and re-measured against the merged spec. They assign IR-666 the converter, the
full claim check, FR-029-AC-28, FR-033-AC-9 and FR-030-AC-15/16/17, with QSL owning the F-1 to F-7
terminals. They keep the builder, parity invocation and original-artifact binding in IR-635. A
positive F-row that cannot run without the IR-635 builder "stays a named dependency in its TC
status row". The ACs' own text agrees: FR-030-AC-15/16/17, FR-033-AC-9/13 and FR-029-AC-28 each
read "PLANNED CG CONSUMER (IR-666)".

Coverage, per AC:

- FR-029-AC-28: covered. The test sends a decode-valid wire request through QSL's public facade
  with `ReplayLimits::default().with_input_bytes(1)` and passes the real report to the public
  converter with the same sent identity. It asserts `BoundExceeded`,
  `Inconclusive(ReplayRefused(code))` and pass-through equality, all with Disagreed retained. The
  fault split is an inspection of QSL's public `from_replay_refusal` plus CG's unchanged
  pass-through. Binding correct.
- FR-033-AC-9: covered for the converter half. The tests hold a genuine report fixed and change
  every top-level `CompositeIdentity` member and the observation members, and each returns
  `ClaimMismatch`. A missing report returns `MissingReport`. A request changed before send binds
  its actual sent identity and refuses the earlier unsent one. CG prechecks and the
  unavailable-capability path belong to the IR-635 builder. Binding correct.
- FR-030-AC-17: partly covered. See SR-2230 FND-001.
- FR-030-AC-15, FR-030-AC-16, FR-033-AC-13: not covered. See FND-001.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-030-AC-15, FR-030-AC-16 and FR-033-AC-13 are IR-666 consumer criteria with no test, and the PR does not record a named IR-635 dependency for them. These are the F-2 GeneratedFault with NativeCause, F-3 RefusedInput, F-4 Admission, F-5 ExactEvaluation and F-6 RefinementCeiling stages, and they stay untagged. Every report in the new tests is a `prepare` refusal, so no F-1 to F-7 or V-1 to V-5 row is ever produced through the converter. The staging ruling requires that a positive F-row that cannot run without the IR-635 builder stay a named dependency in its TC status row. This diff touches no status row. Failure scenario: IR-666 closes on this PR and the owned F-row criteria lose their owner, with nothing in the spec naming IR-635 as the blocker. Fix: cover the reachable rows through QSL's public `compile_package` and `parity_obligation`, which CG tests already use for `compile_package`. Otherwise mark each of these ACs Planned in TC-041/TC-048 and the matrix rows, with IR-635 named as the dependency | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:177-178, spec/replay/functional/FR-033-composite-parity-replay-binding.md:344, spec/kani/matrix/TC-041-ir-outcome-terminal-map.md:102-103, tests/it/composite_parity_converter.rs |
| FND-002 | medium | Spec status prose now contradicts the computed matrix. After this PR, `quire matrix` shows FR-029-AC-28, FR-030-AC-17 and FR-033-AC-9 tagged, but several status lines still say the opposite. kani/matrix/tests.md:53 says "CG consumer tests remain unbuilt", :55 says "no executable CG consumer coverage claimed" and :86 says AC-15 to AC-17 and FR-029-AC-28 "remain planned". replay/matrix/tests.md:21-22 and :36 say no executable CG consumer coverage. The TC-041 Status says steps 14 to 16 are planned. FR-033's Description (line 50) says "PLANNED/UNRUN", its Falsified Settlement section says "CG's consumer remains planned until IR-666 code", and Setup Refusal step 3 (:301-304) still describes the pre-delivery unavailable-capability gate. spec/tests.md:20 says FR-033 is "Gated on actual QSL-640 API delivery". Failure scenario: a reader or gate that uses the status rows treats delivered coverage as absent, and the AC-15/16 rows that really are absent get no different marking. Fix: update each row to the true per-AC state: covered for FR-029-AC-28 and FR-033-AC-9 (converter half), partial for FR-030-AC-17, and planned with the IR-635 dependency for the rest | spec/kani/matrix/tests.md:53, spec/kani/matrix/tests.md:55, spec/kani/matrix/tests.md:86, spec/replay/matrix/tests.md:21, spec/replay/matrix/tests.md:22, spec/replay/matrix/tests.md:36, spec/kani/matrix/TC-041-ir-outcome-terminal-map.md:102, spec/replay/functional/FR-033-composite-parity-replay-binding.md:50, spec/tests.md:20 |
| FND-003 | medium | No spec declares the new public API. `composite_parity_terminal_value`, `verified_shadow_terminal_value`, `CompositeParitySettlement`, `VerifiedShadowSettlement` and `CompositeReportError` (`MissingReport`, `ClaimMismatch`) are exported from lib.rs. interface-001's operations list and its Features table, which says "every operation the contract above declares", list `run_terminal_value` and `ir_outcome_terminal_value` but neither converter. FR-033 names only a "typed CG refusal". Failure scenario: the public names and the refusal vocabulary are unspecified. IR-635 will add precheck and unavailable-capability refusals, which may become new variants of the same enum, and no spec governs whether they do. Fix: add both operations, their inputs, outputs and refusal type to interface-001, and name the refusal variants in FR-033 | src/lib.rs:164-167, spec/core/functional/interface-001-codegen-api.md:139-152, spec/core/functional/interface-001-codegen-api.md:395 |
| FND-004 | low | The new module carries no `Implements:` tag, although each sibling module in src/replay/mod.rs has one (for example `// Implements: FR-015-AC-33` on `frame`). `quire trace --file src/replay/composite.rs` reports 0 claims and only a structural FR-033 mention. Inverse lookup therefore cannot tie the converter to FR-033-AC-9 or FR-029-AC-28 | src/replay/mod.rs:7-8, src/replay/composite.rs:1 |

## Verdict

The converter fully covers the binding half that IR-666 owns: FR-033-AC-9 for the converter and
FR-029-AC-28. It uses real QSL public reports and no fabricated F-row control, and it does not
build any IR-635 builder or invocation code. The gaps are in completing the ticket and its spec
bookkeeping. The F-row consumer criteria FR-030-AC-15/16 and FR-033-AC-13 have neither tests nor
a recorded IR-635 dependency (FND-001). Status prose across the matrix, TC and FR documents now
contradicts the computed matrix (FND-002). The new public API is in no interface contract
(FND-003). This is not merge-ready until FND-001 and FND-002 are resolved in this PR, and
FND-003 is resolved or explicitly ticketed.

## Dispositions

Round 1. Reviewed at the branch's second frozen head: fix commit "Bind composite reports and
cover public parity rows". The revision is recorded in the IR-666 Linear marker only. This is a
static re-check. The committed `reviews/SR-2231-ir-666-gap-analysis.md` is byte-identical to
this artifact's first-custody text.

Computed matrix (`quire matrix --format tsv`), merge base against head, comparing criterion and
status columns. The row count is unchanged. Eight criteria move from untagged to tagged:
FR-029-AC-28, FR-030-AC-15, FR-030-AC-16, FR-030-AC-17, FR-033-AC-7, FR-033-AC-9, FR-033-AC-12
and FR-033-AC-13. No other status changes. FR-029-AC-17 and FR-029-AC-19 to AC-27 stay untagged.
Current `main` has 10 more rows (FR-034-AC-41 to AC-50 from IR-682); they do not overlap this
branch.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | round-1 fix commit "Bind composite reports and cover public parity rows". `tc_041_f1_and_f2_preserve_precedence_and_native_causes` and `tc_041_f3_to_f6_keep_operand_and_limit_stages_distinct` drive real QSL F-1 to F-6 reports. The QSL package comes from public `compile_package` and the O-09 identity from public `parity_obligation`. The F-2 case for each native kind competes with an out-of-domain operand, both limits and CeilingReached. The criteria are marked PARTIAL, with IR-635 named for the original-artifact path. Counter not asserted: see SR-2230 FND-005 |
| FND-002 | fixed | round-1 fix commit: kani/matrix/tests.md:53/55/85-86, replay/matrix/tests.md:21-23/37, the TC-041 Status, FR-033 (Description, Falsified Settlement, Setup Refusal step 3) and spec/tests.md:20 now describe the delivered converter, and IR-635 is named for what remains. Two new status contradictions are recorded below as FND-005 and FND-006 |
| FND-003 | fixed | round-1 fix commit: interface-001 declares `composite_parity_terminal_value` and `verified_shadow_terminal_value` (inputs, outputs, `CompositeReportError (MissingReport \| ClaimMismatch)`) in operations and Features. FR-033 Behavior names both operations and both variants. The declared surface matches the five exported items |
| FND-004 | fixed | round-1 fix commit: `// Implements: FR-033-AC-9, FR-029-AC-28` in src/replay/mod.rs and in the module header |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | FR-033-AC-7 has contradictory status. Its own row now reads "PARTIAL (IR-666 direct public F-7 converter controls ...)", and the computed matrix tags it (`tc_041_f7_...`). The replay matrix row at line 21 still puts AC-7 under "Planned (IR-635) ... remain unbuilt". spec/tests.md:20 lists only "FR-033-AC-9/12/13" as Partial. Failure scenario: anyone reading the matrix files sees AC-7 as unbuilt while the computed matrix and the FR say otherwise. Fix: move FR-033-AC-7 into the line-22 Partial row and the spec/tests.md list | spec/replay/matrix/tests.md:21-22, spec/tests.md:20, spec/replay/functional/FR-033-composite-parity-replay-binding.md:350 |
| FND-006 | medium | The kani/matrix/tests.md:52 row for FR-029-AC-17 and AC-19 to AC-27 changed from "Planned (IR-635)" to "Partial (IR-666 direct public converter controls ...); QSL #645 reports exercise F-1 to F-7, V-1 to V-5 ...". The computed matrix shows all ten of those criteria untagged, with no binder. The row claims partial coverage that no traced test backs. FR-029-AC-20, the F-1 to F-7 consumption criterion IR-666 owns, is the one the new tests actually exercise; see SR-2234 FND-001. Failure scenario: the row overstates coverage for nine IR-635 criteria. Fix: keep the row Planned (IR-635), and give FR-029-AC-20 its own row once a test traces it | spec/kani/matrix/tests.md:52 |
