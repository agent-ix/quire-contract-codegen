---
id: SR-586001
title: 'code-review review of IR-917 CG PR #586'
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md,
  spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/core/functional/interface-001-codegen-api.md
review_set: subset
---

## Summary

Independent Claude Opus 5.5 review of CG PR #586 at 322f78313264326b4e83918e7d81e876522334e5, base aa55d225. Scope: the three changed spec artifacts and the cited existing CG execution/provenance contracts; no applicable AssuranceProfile.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-017-AC-57 accepts every KaniExecutionRequest harness but requires HarnessProvenanceMismatch for invalid sealed provenance. Existing KaniExecutableHarness::has_valid_provenance is unconditional true for Contract, Scalar and StateFrame; TC-027 starts from a 'contract harness'. Restrict playback to seal-validated variants with a named refusal for others, or first give all accepted variants authentic sealed provenance and test it. | spec/kani/functional/FR-017-kani-execution-evidence.md:931 |

## Verdict

Not mergeable until the findings above are resolved in this PR.
