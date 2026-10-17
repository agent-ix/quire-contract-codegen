---
id: SR-12203
title: "Evidence review of IR-598 per-family corpus retirement (CG #423 delta)"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen; PR #423 delta f22c59d9..1163a768 (IR-598): spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/assurance/AD-001-codegen-architecture.md, spec/assurance/AD-004-cg-crate-layout.md, spec/core/functional/interface-001-codegen-api.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/spec.md, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, spec/kani/matrix/tests.md"
review_set: subset
date: 2026-10-17
---

## Summary

Ticket: IR-598. Checked that the verification method and evidence fit FR-015-AC-109/110/111 and the TC-023 retirement procedure. 'Test (TC-023)' fits the absence searches and the refusal checks. The 'green before deletion' preconditions are temporal and rely on #[ignore] kani-lane tests, so no post-merge test can check them as written.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | 'Green before the deletion' (AC-110, and AC-109's 'green before the arm is deleted', TC-023) is a temporal precondition that no post-merge test can check. The real-Kani tests are #[ignore] kani-lane tests (for example kani_obligations_symbolic_domains.rs:2077), so default CI never runs them. Specify the evidence: the named kani-lane test is run through `make kani` or `make kani-gate` on the slice PR's head, and its result is recorded on the ticket. Use the method Test + Inspection. | FR-015-AC-110, FR-015-AC-109, TC-023 (spec/kani/functional/FR-015-bounded-kani-obligations.md:800-801) |

## Verdict

The search-based absence checks and the refusal tests are appropriate Test evidence. The real-Kani preconditions need an evidence form that someone can audit.
