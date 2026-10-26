---
id: SR-585002
title: 'spec-review/base review of IR-917 CG PR #585'
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md
review_set: subset
---

## Summary

Reviewed CG PR #585 exact head 52a4dfde26116ea81cd2c313f6c8df14c75f057f against base 1175b995bc4314798cafe839a85a1ca2a745408c, limited to the FR-015-AC-41 row and TC-025 step 11 changes. FR-015-AC-7, FR-015-AC-37, FR-015-AC-119 and SR-583002 FND-007 were read as context. Ticket: IR-917. Reviewer model: Claude Opus 5.5 (claude-opus-5-5). Criterion-strength was skipped because no Jev client is available. Spec-only: there was no Cargo, Kani or full-gate run.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS. The change resolves SR-583002 FND-007: the backend may choose any in-domain witness, and the exact `999` control moves to a separate deterministic playback. Hand-restated assertions and harness probes are excluded explicitly. TC-025 step 11 matches FR-015-AC-41. See SR-585004 and SR-585005 for the two sub-analysis findings.
