---
id: "SR-2402"
title: "spec-ears-analysis of IR-670 FR-034 native ABI admission statements"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen@24b4135daad56b1673e2107e953cb823a9cac5ad; diff e5b303f5e5010906567a81bbcdbd6d42c58b5407...24b4135daad56b1673e2107e953cb823a9cac5ad; spec/kani/functional/FR-034-caller-death-ownership.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-2402: spec-ears-analysis of IR-670 FR-034 native ABI admission statements

## Summary

Ticket: IR-670. EARS conformance of the new normative statements in FR-034 (replaced sentence L616-620 and the four new paragraphs L635-667). The event-driven refusal ('When ... lacks ..., C shall refuse'), the ubiquitous enforcement clause and the post-Dispatch classification clause are EARS-shaped. One paragraph has two clauses that are not.

Reviewer: session 931a2951-8e8f-4dc1-b745-5c804f4eb6a6, model claude-opus-5-5, run 0fde4e67-c6f4-484d-8e20-14fb1de71f48.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Two clauses in the new enforcement paragraph are not EARS-shaped: 'An unsupported architecture or x32 alias shall not reach an allowed native-syscall action' makes a syscall property the subject instead of the system (the installed policy or I), and 'native entry followed by a compat syscall must receive the same enforcement' uses 'must' rather than 'shall'. | spec/kani/functional/FR-034-caller-death-ownership.md:643; spec/kani/functional/FR-034-caller-death-ownership.md:644 |

## Finding detail

### FND-001

Confidence: high. Check: other. Unit: FR-034 (spec/kani/functional/FR-034-caller-death-ownership.md:643-645).

Failure scenario: A requirement extractor that keys on '<system> shall' skips the 'must' clause, so the native-entry compat-syscall obligation is not counted as a requirement, although TC-049 step 26 tests it.

## Scope

- `FR-034 L616-620` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L635-639` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L641-645` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L645-648` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L650-654` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L654-658` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L660-664` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L664-667` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined

## Verdict

Mostly conformant; the descriptive disclaimers ('imposes no ... precondition', 'claims no binding') are scope statements, not requirements, and are acceptable as prose. AC-39 bundles many obligations in one row, matching the existing AC-35..AC-38 convention in this file, so it is not raised as an EARS defect here.

## Dispositions

Round 1, reviewed at agent-ix/quire-contract-codegen@aef3d5715543db202d37d28c5fa4f422b99dc369 (fix diff 24b4135..aef3d57; 79a1a9a only adds the seven original SR files under reviews/, byte-identical to this file's original prefix). Reviewer session 931a2951-8e8f-4dc1-b745-5c804f4eb6a6, model claude-opus-5-5, run fea14e45-7035-49c6-9f42-ef4ddc5a1514. Every outcome was re-measured against the fix head; the author's fix map was read as data only. No build, test, Kani or full gate was run.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | aef3d57: Both clauses now have the installed policy as subject with 'shall' (FR-034:668-674): event-driven 'When a syscall has an unsupported audit architecture, the installed policy shall terminate ...' and 'When native entry issues a compat syscall, the installed policy shall apply ...'; no 'must' remains in the new FR section (L633-700). |
