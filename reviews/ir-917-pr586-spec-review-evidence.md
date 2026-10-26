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

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | TC-027 intro still assigns the installed 999 playback and attribution to TC-025 step 11, while new TC-027 step 5 and FR-017-AC-60 also claim that same control through the public API. Assign TC-025 the unconstrained Falsified proof and TC-027 the deterministic 999 playback; cross-reference the two. | spec/kani/matrix/TC-027-kani-execution-evidence.md:219-221 |
| FND-003 | low | FR-017-AC-58 prescribes a fixed playback argv that omits proof-time -Z function-contracts without saying why. Kani documentation says compilation options used during verification must be used for playback. Derive compile-relevant -Z flags from the retained harness option vector or state the evidence-backed reason they can be dropped; have TC-027 assert the chosen argv. The V2 harness has no contract attributes, so this is a low-risk specification gap. | spec/kani/functional/FR-017-kani-execution-evidence.md:941 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | New AC-60 and TC-027 step 5 require installed healthy and expected-panic mutant runs through the exact public API, with assertion attribution, bounded output and owned teardown. |
| FND-002 | still-open | New in this fix: TC-027 introduction and new step 5 both assign the 999 playback control to different test cases. |
| FND-003 | still-open | New in this fix: AC-58 drops proof-time compilation option -Z function-contracts without a stated derivation or justification. |
