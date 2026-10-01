---
id: "SR-692"
title: "CG PR 217 spec review: TC-029 and TC-031 runtime dependency wording"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@757efc45d63ec97dddb83bbada83dacaeb908a76; spec/oracle/matrix/TC-029-composite-equality-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-029
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-031
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-018
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: references
---

# SR-692: CG PR 217 spec review

## Summary

Ticket: IR-344. PR: agent-ix/quire-contract-codegen#217 at 757efc4. Two spec files changed, one
procedure step each: TC-029 step 3 and TC-031 step 3. "The runtime revision" becomes "the
runtime dependency (git source, `branch = "main"`, no pinned `rev`)". Methods: spec-review
(consistency, traceability, integrity) on those steps and the criteria they verify.

Checked:

- The criteria. FR-018-AC-12 (`FR-018-composite-equality-oracles.md:205`) and FR-021-AC-14
  (`FR-021-function-application-oracles.md:220`) say the generated crate "names the Contract
  Runtime dependency this repository's `Cargo.toml` names with the `exact` feature". CG's
  `Cargo.toml` names the runtime by git URL and `branch = "main"`. The new step text matches the
  criteria. The old text ("the runtime revision", that is a `rev` pin) did not, because
  `Cargo.toml` never named a rev. The change brings the TCs into line with the ACs.
- The tests. `tc_029_ac12_manifest_is_unpublished_and_charge_free`
  (`tests/it/composite_equality_generation.rs:910`) and `tc_031_ac14_manifest_and_source_shape`
  (`tests/it/exact_function_generation.rs:666`) assert the whole dependency line with the
  `exact` feature and `!contains("rev =")`. The step text describes what they assert.
- No other spec text names the runtime revision. Grep of spec/ for `runtime revision`, `rev` and
  `RUNTIME_REVISION` outside AD-004 found none. No requirement id was minted, no TC count
  changed, so `tests.md` needs no edit. No pin or SHA was added, which matches the repository
  CLAUDE.md hash and pin rule.
- `make spec` exits 0 with the 3 baseline warnings (FR-017:137, FR-014:278 twice). The PR adds
  none.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. The two procedure edits are truthful and match FR-018-AC-12, FR-021-AC-14 and the tests
that trace them. The spec gaps around DuplicateHarness and AD-004 L-10 are recorded in the gap
analysis (SR-691), not here.
