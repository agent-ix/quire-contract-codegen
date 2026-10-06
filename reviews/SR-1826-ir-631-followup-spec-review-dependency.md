---
id: SR-1826
title: "IR-631 follow-up dependency: FR-032 relationship edges to its normative upstream"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-contract-codegen@f84c4fc93fe4ba5f8460e60bb8e75998d5cbcfd8; spec/assurance/AD-003-evidence-chain.md, spec/replay/functional/FR-032-routed-scalar-replay-binding.md, spec/replay/matrix/TC-047-routed-scalar-replay-binding.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
---

# SR-1826: IR-631 follow-up dependency

## Summary

Ticket: IR-631. PR: quire-contract-codegen, not yet opened. It is spec only. Reviewer: claude-opus-5-5, session 8950150c-5937-457d-9408-c19aa15729b8, run f92b6bc1-4101-4d05-affe-ea238bac36af. One low finding.

## Method

I compared the `relationships` frontmatter of FR-032, TC-047 and AD-003 with the upstream authorities their bodies now treat as normative:

- QSL FR-357
- ADR-013 O-09 and section 2
- IR-648, which is a ticket and not a spec artifact
- AD-002 R-6/R-7

I checked that the enablement work (IR-648 accessor, CG retention, driver authentication) is separated from feature work, and that no dependency cycle is introduced.

## Verdict

**PASS with one low finding.**

- The new AD-003 to FR-032 `references` edge is correct.
- IR-648 is cited as a ticket gate, not as a relationship, which is right.
- The code gates order enablement before the admitted route.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-032's body now depends normatively on merged QSL FR-357: its operator facade, `claim()` on every outcome, the parity preimage and the scalar terminal causes. TC-047 steps 5 to 8 test against it too. Yet FR-032's `relationships` still name only `quire-spec-language/ADR-013` upstream. A `quire extract` or blast-radius query from FR-357 will not reach FR-032, so a later FR-357 change does not flag this requirement. Add `ix://agent-ix/quire-spec-language/FR-357` with `depends_on`. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:5-23 |
