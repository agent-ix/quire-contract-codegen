---
id: SR-1643
title: "IR-635 spec review (EARS conformance): FR-033 and the FR-025/028/029 amendments"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen#298; spec/kani/functional/FR-025-generated-subject-abi.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: reviews
---

# SR-1643: IR-635 spec review, EARS conformance

## Summary

Ticket: IR-635. PR: quire-contract-codegen#298 (spec only; the reviewed head is recorded in the private ticket marker). Reviewer: claude-opus-5-5, session 2f7447c0-1934-4031-8988-fab17e2f41b7. I checked the new and edited requirement statements and criteria for EARS form, atomicity and unambiguous terms. Two medium findings.

## Method

I read every `shall` statement the diff adds:
- FR-025, "Composite leaf bindings";
- FR-028, "Parity settlement projection";
- FR-029, "Composite shadow publication", the edited `Tested` bullet, AC-6 and AC-17;
- FR-033 Description, Behavior and Setup Refusal Precedence.

I also read every FR-033 criterion. For each, I checked the trigger and condition form, whether it names one responsible system, whether it is atomic enough for the computed matrix (which binds tests per criterion id), and whether its terms are used consistently with FR-028.

## Verdict

**PASS with two medium findings.** Clean: FR-025's refusal statement uses the EARS unwanted-behaviour form ("If ..., then the generator shall refuse"). The FR-033 Behavior bullets each name the builder, converter or driver. FR-033-AC-4 states measurable boundaries: at the limit and one byte over, a 100,000-link value, a 512 KiB stack, and a budget one byte too small.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-029-AC-17 packs about fourteen independently failing obligations into one criterion. These are the strength projection, five priority rows, coverage refusal, the falsified divergence and agreement mapping, stop-to-Tested exclusion, cross-claim refusal, interim refusals, build-totality and record-category preservation. The matrix binds tests per criterion id, so one test tagged FR-029-AC-17 (for example the interim refusal, which is the only part that can be built before QSL-640) would mark the whole map `tagged`. That is the coverage inflation SR-1642 FND-002 records for AC-6. Split AC-17 into separately traceable criteria: the projection, the verified priority order, the falsified mapping, the interim refusal and totality. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:265 |
| FND-002 | medium | FR-028's new paragraph says "a native run stopped at either recorded ceiling remains `shadow_proved_refinement_inconclusive`". FR-028-AC-17 assigns that strength when the refinement run reaches a ceiling (FR-028-AC-24). A backend shadow run stopped at a ceiling is an `inconclusive` outcome with no proof strength. "Native run" can be read either way, and FR-029 line 173 uses "backend/native execution" for the other case. One implementer could give a stopped Kani run a strength and project it to `CeilingReached`. Say "a refinement run stopped at either recorded ceiling" here, and use "refinement run" against "backend run" consistently in FR-029 lines 173-176. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:180, spec/kani/functional/FR-029-run-outcome-terminal-record.md:173 |
