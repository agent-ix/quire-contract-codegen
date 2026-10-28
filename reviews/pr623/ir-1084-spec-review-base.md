---
id: SR-623003
title: Base spec review of CG PR 623 AD-004 step 4i
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; IR-1084; CG PR 623; spec/assurance/AD-004-cg-crate-layout.md step 4i, H-3, R-1, L-3"
review_set: subset
---

## Summary

Base checklist review of the AD-004 amendment at `081d9aad28de2b4d5b4e6d9e24dc35e32e58bbd4`. The amendment names step 4i, retains the 4g corpus exception, and does not edit or narrow FR-015 criteria. No applicable AssuranceProfile was found under `spec/`.

## Scope

```yaml
scope:
  - {id: AD-004-H-3, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "Invariants, enforced by the family-specific `HarnessSpec` constructors. A refusal is a typed `HarnessSpecError`; none is a panic."}
  - {id: AD-004-R-1, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "It holds no family-specific template text."}
  - {id: AD-004-L-3, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "Step 4i removed the frame template from `kani/generate/frame.rs`; the corpus template in `kani/generate/corpus/bounded_kani_corpus.rs` remains until 4g."}
  - {id: IR-1084-AC-4, path: spec/assurance/AD-004-cg-crate-layout.md, role: context_only, excerpt: "Run the applicable final reviewed PR gates once, record the exact head and outcome, and update AD-004's step table/interim exceptions with evidence. No FR-015 AC is removed or narrowed and no source or fixture is copied from another repository."}
```

## Analysis

The changed AD uses the existing identifiers and step numbering, preserves explicit interim exceptions, and validates with `quire validate --scope . spec/assurance/AD-004-cg-crate-layout.md` (exit 0). `git diff --check` passes. The two retained contradictory design claims are recorded in the separate integrity analysis, so this base artifact does not duplicate its findings.

## Verdict

PASS for base structure and scope; see SR-623004 for two internal consistency defects.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
