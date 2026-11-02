---
id: SR-202610310181
title: "IR-1125 PR709 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; tests/it/kani_obligations_state_frame.rs"
review_set: subset
relationships:
  - {target: "ix://agent-ix/quire-contract-codegen/FR-015", type: reviews}
---

## Summary

Reviewed frozen fd5a5cf10ec622f9a65c7f0acdf7b197088d1af7 against 2256d0e4cb864c38b748ff757eed25034802c59a. Ticket: IR-1125. Scope is only the changed private test fixture and two installed controls in tests/it/kani_obligations_state_frame.rs. No production, API, spec or CI change. Existing generation helper retains 16 GiB/600s; only two controls select 4 GiB/120s. Domains remain balance/audit 0..1000 and unwind 4. No 16 GiB proof was run by this reviewer.

Inspected commands-and-results.txt, fmt/clippy/focused/controls logs, control-summary.json and all five execution/source/generated-source/identity/manifest receipt sets under /home/peter/dev/qualification-logs/titan-ir1125. Full compiled source contains its generated source. Frame outcomes are Verified/Falsified/Falsified (allowed/audit mutation/empty modifies); postcondition outcomes are Verified/Falsified (healthy/debit). Assertions name the actual violated obligation. The public generator reaches frame.rs HarnessSpec::new_frame/render_spec for both roles. Receipt names separate serial calls per named test; existing make kani serial lane applies. No new duplicate generator or copied upstream implementation was introduced; searched generation and proof helper definitions/callers in the file and tests/common/proof_ceilings.rs.

Recorded gates: fmt and all-target/all-feature Clippy passed; focused suite raw exit 101 with 79 passed, 5 ignored and only permitted known TC-025 ModelIntake baseline tc_025_model_member_types_without_i64_ranges_remain_unranged. Actual installed controls passed 2 tests/5 requests. Reviewer ran no Cargo or Kani per dispatch. Full ordinary integration gate remains owed after this pass and is owned by the lead. No applicable AssuranceProfile was found. FR-015-AC-58 and AD-004 L-3 remain open for other emitters/corpus; IR-1123 replay is separate. IR-1255 PR706 and its selected-target/native-twin work are outside scope.

## Verdict

PASS for the scoped change; no defects found. Aggregate integration gate remains outstanding and is not waived. No new traceability, reverse-ownership or hollow-evidence gap found in the scoped delta.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed

Computed quire matrix inspected for FR-015-AC-30, FR-015-AC-31 and FR-015-AC-80: all tagged-by-ignored-test, as intended for the installed backend lane. The retained explicit ignored-test run supplies actual backend evidence for this slice; ignored status is not treated as ordinary-suite coverage. No new trace tags or requirements are introduced. Reverse ownership and stub checks cover changed helpers only; this subset review makes no repository-wide coverage claim. Optional standalone semantic gap review was not requested and was skipped. Spec review/sub-analyses do not apply because no spec/plan changed.

## Examined scope

```yaml
scope:
- id: FR-015-AC-30
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: With the installed backend, the operation contract verifies for a healthy
    subject and is falsified, naming the postcondition, for a subject mutated to debit.
- id: FR-015-AC-31
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: With the installed backend, a frame-allowed effect verifies, a frame-forbidden
    effect is falsified naming the forbidden field, and regenerating the frame from
    a package whose `modifies` is mutated to grant nothing falsifies the allowed subject.
- id: FR-015-AC-80
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. With the installed backend, the cases of FR-015-AC-30 and FR-015-AC-31
    run over harnesses generated from the package QSL emits: the healthy subject verifies,
    the subject mutated to debit is falsified naming the postcondition, a granted
    write verifies and a write to an ungranted field is falsified naming that field.'
- id: generation fixture
  path: tests/it/kani_obligations_state_frame.rs
  role: examined
  excerpt: fn generated_from_twin_with_ceilings(
- id: resource ceiling
  path: tests/it/kani_obligations_state_frame.rs
  role: examined
  excerpt: 'const SMALL_FRAME_CEILINGS: quire_contract_codegen::ProofCeilings ='
- id: receipt retention
  path: tests/it/kani_obligations_state_frame.rs
  role: examined
  excerpt: let prefix = format!("{attempt:03}");
bindings:
- test_id: tc_025_real_kani_proves_the_state_postcondition_and_a_mutated_subject_falsifies_it
  ac_id: FR-015-AC-30
  trace: correct
- test_id: tc_025_real_kani_proves_allowed_and_forbidden_frame_effects_and_a_mutated_frame_falsifies
  ac_id: FR-015-AC-31
  trace: correct
- test_id: tc_025_real_kani_proves_the_state_postcondition_and_a_mutated_subject_falsifies_it
  ac_id: FR-015-AC-80
  trace: correct
- test_id: tc_025_real_kani_proves_allowed_and_forbidden_frame_effects_and_a_mutated_frame_falsifies
  ac_id: FR-015-AC-80
  trace: correct
```
