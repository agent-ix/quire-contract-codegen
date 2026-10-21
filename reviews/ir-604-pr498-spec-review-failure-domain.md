---
id: SR-15265
title: "IR-604 PR498 failure-domain review"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen; spec/replay/functional/FR-016, spec/replay/matrix/TC-026, spec/replay/matrix/tests.md"
review_set: subset
---

## Summary

Reviewed multiple playback blocks and identity selection in PR #498 at frozen head 56207dce812d88740800bcb1528d6c2719354ae5. Checked the installed Kani playback selector and the independent stdout oracle.

## Verdict

**CONDITIONAL — the oracle lacks an ordering rule for multiple assertion blocks from one harness.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The independent stdout oracle does not specify which assertion playback to select when one harness prints multiple valid falsifying blocks. Production takes the first eligible block in print order; an oracle choosing a later block would disagree on an unmutated run or miss a substitution. Require the first matching assertion block and exclude cover blocks. | spec/replay/matrix/TC-026-witness-native-replay.md:36-48 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | df8b019 names the first eligible same-harness assertion block in stdout print order, excludes covers and other harnesses, compares the full trimmed fence interior, and adds a two-target-block selection control. |
