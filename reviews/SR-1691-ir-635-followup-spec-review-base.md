---
id: SR-1691
title: "IR-635 follow-up spec review: O-09 versus record versus content roles, Refinement and staged settlement"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@04608d2942dc27f47d0b820a8a58a1b82b1669c3; spec/assurance/AD-003-evidence-chain.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/matrix/tests.md, spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md, spec/replay/matrix/tests.md, spec/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/AD-003
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: reviews
---

# SR-1691: IR-635 follow-up spec review

## Summary

Ticket: IR-635. PR: quire-contract-codegen#303 (spec only). Reviewer: claude-opus-5-5, session 94d28b25-7330-445c-a85c-7e177ba3c080. This is the base spec review. The sub-analyses are SR-1692 (integrity, one high), SR-1693 (EARS, one low) and SR-1694 (failure domain, one medium). The base review adds two low findings.

## Method

I read the changed requirements and tests for quality, consistency and completeness. My focus was the separation of three roles: the exact O-09 claim preimage, the full FR-015-AC-76 CG record, and the canonical proved-content tie. I also checked:

- the shared `Refinement`
- identity-first refusal
- F-1 to F-6 and the verified order
- the narrowed FR-028-AC-2/3
- that the planned criteria stay untagged

FR-015-AC-76 was read for context. I skipped the object, dependency and scope-boundary sub-analyses:

- **Dependency:** the change adds only two `references` edges, AD-003↔FR-033.
- **Object and scope-boundary:** the change adds no domain object and moves no ownership boundary. The CG converter owns the content-tie check, as it already did in FR-033 Behavior, and QSL owns settlement.

## Verdict

**FAIL, because of the high finding in SR-1692.** This base review adds two low findings. What the PR gets right:

- O-09 is now separated from the full CG record and from the content tie in AD-003, FR-033 Inputs and AC-11.
- Function and frame preimages are explicitly unchanged.
- No tracking digest or pin is added.
- The QSL-640 code gate and the unverified representation are kept.
- FR-029-AC-23/24 stay planned and untagged.
- FR-028-AC-2/3 are narrowed only to separate the backend classification from the final terminal.
- No row produces `Refuted`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | "pair count" is used in two senses. In the O-09 exclusion list (FR-033 Inputs, AC-11, AD-003 E-1 extension, TC-048 step 9) it means FR-015-AC-76's static "closure's pair-node count" in the harness identity record. In F-6, AC-7 and `EqualityOutcome` it means the runtime occurrence-pair count. AC-11 says "changing only … pair count … does not [change O-09]", and TC-048 step 9 says "change only … pair count … and require the same O-09 digest". A tester can satisfy both by mutating the runtime count, which was never an identity input, and so never exercise the record-only member. Use AC-76's term "pair-node count" wherever the record field is meant. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:79, spec/replay/functional/FR-033-composite-parity-replay-binding.md:230, spec/replay/matrix/TC-048-composite-parity-replay-binding.md:97 |
| FND-002 | low | TC-048 step 9 requires a test to "replace the proved artifact or original context while retaining O-09 and require the separate canonical content tie to refuse". The same step ends "no new CG tracking digest or speculative content-binding API is tested". Read literally, the step asks for a content-tie refusal test and forbids testing the content-binding API that refusal goes through. Say what the step means: test the refusal through the delivered owning API once QSL-640 lands, and invent no API before then. That keeps the step from reading as permission to skip the artifact-replacement check. | spec/replay/matrix/TC-048-composite-parity-replay-binding.md:97 |
