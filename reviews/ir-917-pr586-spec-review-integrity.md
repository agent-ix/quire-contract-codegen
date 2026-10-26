---
id: SR-586004
title: 'spec-review/integrity review of IR-917 CG PR #586'
type: SpecReview
analysis: integrity
scope: agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md,
  spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/core/functional/interface-001-codegen-api.md
review_set: subset
---

## Summary

Independent Claude Opus 5.5 review of CG PR #586 at 322f78313264326b4e83918e7d81e876522334e5, base aa55d225. Scope: the three changed spec artifacts and the cited existing CG execution/provenance contracts; no applicable AssuranceProfile.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-015-AC-41/TC-025 say the appended playback test is outside the sealed harness; TC-027 places it in the generated harness's owning module. Because the generated module contains the private harness function, placing the test inside its braces changes the generated module bytes until removal. Specify one insertion location: inside the module but outside the harness function, and require exact restoration after removal. | spec/kani/functional/FR-017-kani-execution-evidence.md:931 |
| FND-002 | medium | AC-59 does not define how to interpret several libtest summaries or a passing test line interrupted by --nocapture output. Native playback can run several test targets and print zero-test summaries. Define whether the rule counts across all summaries or restricts execution to one target, and test multi-summary and interleaved-output cases. | spec/kani/functional/FR-017-kani-execution-evidence.md:933 |
| FND-003 | low | The exact appended byte span is undefined for attributes, doc comments and separators; source restoration can therefore have two interpretations. Require one specified insertion span and unique generated-harness occurrence after removal, and assign a typed refusal for absent or unreadable src/lib.rs. | spec/kani/functional/FR-017-kani-execution-evidence.md:931 |

## Verdict

Not mergeable until the findings above are resolved in this PR.
