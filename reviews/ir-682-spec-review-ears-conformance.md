---
id: SR-2981
title: "IR-682 spec-review/ears-conformance review"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen PR #320 head (commit subject: Specify build-specific native workspace conformance analysis); spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
---

# SR-2981: IR-682 spec-review/ears-conformance review

## Summary

Ticket: IR-682. PR: quire-contract-codegen#320. This review checks the EARS form of the new FR-034
statements (lines 1190-1264) and criteria FR-034-AC-41 through FR-034-AC-50. Each criterion has
one subject, "the analysis", and a pass or fail outcome: UNPROVEN, or a failed conformance claim.
The statements use EARS ubiquitous and conditional forms ("For each ... SHALL", "If any required
path ... then ... SHALL remain UNPROVEN"). The criterion ids are unique: no other file, open PR or
remote branch uses FR-034-AC-41 through FR-034-AC-50.

## Method

Classified each new normative sentence by EARS pattern and checked that each criterion has a
single responsibility, uses a measurable term, and avoids negative-only or vacuous wording. Grepped
the head tree and every remote branch for collisions on FR-034-AC-41 to FR-034-AC-50. Compared the
normative keyword usage against the base FR-034, which has no uppercase SHALL.

## Verdict

**PASS WITH LOW FINDINGS.** The statements are atomic enough and every criterion can fail. Two
wording defects remain.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | The section mixes uppercase "SHALL" (9 uses) and lowercase "shall" in the same sentences (for example lines 1196-1200). The base FR-034 and the rest of the file use only "shall". Readers may infer two strengths of obligation where none is intended. | spec/kani/functional/FR-034-caller-death-ownership.md:1192-1264 | wrong-requirement |
| FND-002 | low | "Stable objdump disassembly" is undefined. objdump has no stable or unstable mode, and it is unclear whether GNU binutils or llvm-objdump is meant. The intended constraint, stable compiler flags only (no `-Z` options such as `-Z emit-stack-sizes`), is not stated anywhere, so FR-034-AC-42 does not forbid nightly-only build flags. | spec/kani/functional/FR-034-caller-death-ownership.md:1225; FR-034-AC-42 | wrong-requirement |
