---
id: SR-1763
title: "IR-639 guardian-test-support spec review (EARS conformance)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen@7742a43f62fc3feddde61a529a2bab37b4d0a67b; spec/kani/functional/FR-034-caller-death-ownership.md (PR #305 diff against main 28553daeb1e9cfd88bb6620125bb1df27273c68e)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
---

# SR-1763: IR-639 guardian-test-support spec review, EARS conformance

## Summary

Ticket: IR-639. PR: quire-contract-codegen#305. Reviewer: claude-opus-5-5, session
9072f908-e626-4176-ae33-69e582990445. I checked the new and edited requirement statements in
FR-034 against EARS grammar: one system subject, "shall", one response, and an explicit trigger or
state where conditional. One low finding.

## Method

I examined each sentence of the new "Opt-in guardian fixture observation" section and the edited
sentence at the end of "Production lease-close cancellation". I also checked whether FR-034's
Behavior "shall" list, which enumerates every other obligation, gained matching statements.
Scope units examined: the FR-034 statement (new section and edited paragraph). The acceptance
criteria are not EARS statements and were not judged here.

## Verdict

**FAIL: one low finding.** Most new statements use a single subject and "shall" ("The
`guardian-test-support` Cargo feature shall be off by default...", "The feature shall add no
branch inside a production stage...").

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Several new obligations in the opt-in section are descriptive rather than EARS "shall" statements: "Delivery selects the package helper from the consumer manifest using `cargo -p`", "Both refusal and export absence require verification", and "These are planned obligations". "Production-driver dependency-edge verification shall assert..." puts the obligation on an external actor, not the system. The Behavior "shall" list (lines 58-105), which enumerates every other FR-034 obligation, gained no entry for the feature. Rewrite these as system-subject "shall" statements, and add the feature's obligations to the Behavior list. | spec/kani/functional/FR-034-caller-death-ownership.md:211, spec/kani/functional/FR-034-caller-death-ownership.md:215-222, spec/kani/functional/FR-034-caller-death-ownership.md:58-105 |

## Dispositions

Round 1, reviewed at ef9ee8ec51b81c4f9a967c6ea738e16cc9e4528d.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | ef9ee8ec51b81c4f9a967c6ea738e16cc9e4528d: The Behavior list gains system-subject shall entries for the feature, and the new section's normative text is rewritten as shall statements. The probe feasibility and downstream IR-649 sentences remain as context prose. |
