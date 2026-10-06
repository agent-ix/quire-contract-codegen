---
id: SR-1646
title: "IR-635 spec review (dependency): FR-033 and TC-048 relationship edges"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-contract-codegen#298; spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-048
    type: reviews
---

# SR-1646: IR-635 spec review, dependency

## Summary

Ticket: IR-635. PR: quire-contract-codegen#298 (spec only; the reviewed head is recorded in the private ticket marker). Reviewer: claude-opus-5-5, session 2f7447c0-1934-4031-8988-fab17e2f41b7. I resolved every `relationships` target in FR-033 and TC-048 against the owning repository. One medium finding.

## Method

Local targets (StR-001, FR-015, FR-025, FR-028, FR-029, AD-002) resolve in this repository. For upstream targets, I resolved each against QSL main and QSpec main:
- `quire-spec-language/FR-070`, `ADR-013`, `ADR-014` and `ADR-021` exist on QSL main.
- `quire-spec-language/FR-358` exists only on QSL's open PR #645.
- `quire-specification/FR-181` (typed canonical form) exists on QSpec main.
- `quire-spec-language/FR-322` exists on QSL main.

## Verdict

**PASS with one medium finding.** Clean: every CG-local edge resolves. TC-048's `verifies` edges match its Description. FR-070, FR-181 and the three ADRs are the right owners for the claims made of them, and FR-358 is correctly marked as a proposed upstream obligation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-033's edge `ix://agent-ix/quire-spec-language/FR-322` and its prose "the proposed QSL ... FR-322 node selector" (lines 47-48) and "select the claimed FR-322 node" (line 85) name the wrong artifact. QSL FR-322 is "Evaluate union construction and case at S6a", and QSL PR #645 does not change it. The checked package and node identity that QSL ADR-013 cites as "FR-322 (`quire.checked-package/v2`)" is QSpec FR-322, the checked-package-artifact interface. No QSL node-selector requirement numbered FR-322 exists at QSL main or the PR #645 head. An implementer following the edge reads an unrelated union-evaluation requirement. Retarget the edge to `ix://agent-ix/quire-specification/FR-322` for package and node membership, and name the node-selected parity claim as a pending QSL-640 obligation rather than as FR-322. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:20, spec/replay/functional/FR-033-composite-parity-replay-binding.md:48, spec/replay/functional/FR-033-composite-parity-replay-binding.md:85 |
