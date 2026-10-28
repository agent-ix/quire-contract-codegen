---
id: SR-619008
title: "IR-1085 PR 621 spec review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #621 at bdaac83aedc4dbbce0da7e933d56a8a4e74ee26e; spec/assurance/AD-004-cg-crate-layout.md, spec/core/functional/interface-001-codegen-api.md; IR-1085 AC1-4 and FR-016-AC-1/2/3/4/5/6/8/25"
review_set: subset
---

## Summary

Ticket: IR-1085. The interface entry accurately names typed playback and the AD-004 interim table keeps frame replay open. The AD narrative overstates a caller path and a corpus limit source.

## Verdict

**CONDITIONAL** — the two wording defects below need correction before the AD's current-state account is accurate. The acceptance criteria themselves are unchanged.

## Coverage

- Examined IR-1085 AC1-4, the changed interface operations, AD-004 L-6/current-state and interim rows, and FR-016-AC-1/2/3/4/5/6/8/25 as context.
- The AD keeps frame replay and frame renderer residuals; it does not narrow any acceptance criterion. No new hash, digest, pin, or cross-repository copy is introduced by the spec diff.
- The other existing L-6 residuals were read as context; no full spec review or project-wide validation was claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The AD states that function callers parse complete captured text once at the output boundary, but this PR adds only test callers and no active production caller to substantiate that current-state claim. | spec/assurance/AD-004-cg-crate-layout.md:810 |
| FND-002 | low | The AD says function replay checks its own proving-run limits universally; corpus replay builds its limits from the caller-supplied `limits` argument, so the statement needs to distinguish the corpus entry. | spec/assurance/AD-004-cg-crate-layout.md:810 |
| FND-003 | low | The frame renderer residual says IR-1084 waits on the remaining function and frame L-6 slices even though the same AD marks the function slice done. | spec/assurance/AD-004-cg-crate-layout.md:1493 |
