---
id: SR-586006
title: 'spec-review/evidence review of IR-917 CG PR #586'
type: SpecReview
analysis: evidence
scope: agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md,
  spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/core/functional/interface-001-codegen-api.md
review_set: subset
---

## Summary

Independent Claude Opus 5.5 review of CG PR #586 at 322f78313264326b4e83918e7d81e876522334e5, base aa55d225. Scope: the three changed spec artifacts and the cited existing CG execution/provenance contracts; no applicable AssuranceProfile.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-027 checks only launcher stand-ins; TC-025's installed playback explicitly leaves the mechanism unspecified. No test must run execute_kani_concrete_playback against installed Kani, so an invalid CLI or missing Kani cfg/contract options could pass. Require an installed healthy and failed-assertion playback through this exact public API, with output and teardown evidence. | spec/kani/functional/FR-017-kani-execution-evidence.md:932 |

## Verdict

Not mergeable until the findings above are resolved in this PR.
