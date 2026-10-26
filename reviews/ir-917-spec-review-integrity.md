---
id: SR-585004
title: 'spec-review/integrity review of IR-917 CG PR #585'
type: SpecReview
analysis: integrity
scope: agent-ix/quire-contract-codegen; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md
review_set: subset
---

## Summary

Reviewed CG PR #585 exact head 52a4dfde26116ea81cd2c313f6c8df14c75f057f against base 1175b995bc4314798cafe839a85a1ca2a745408c, limited to the FR-015-AC-41 row and TC-025 step 11 changes. FR-015-AC-7, FR-015-AC-37, FR-015-AC-119 and SR-583002 FND-007 were read as context. Ticket: IR-917. Reviewer model: Claude Opus 5.5 (claude-opus-5-5). Criterion-strength was skipped because no Jev client is available. Spec-only: there was no Cargo, Kani or full-gate run.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-015-AC-41 binds the `999` playback to the plus-one Kani run by oracle bytes, clause identity and native expectation, but not by the generated harness source or harness identity. 'The actual generated harness' does not say it is the same harness that run executed. FR-015-AC-119 in the same FR does list the generated harness source among the artifacts that must be unchanged across its runs. Fix: add 'generated harness source' (or its harness identity record) to the compared set in FR-015-AC-41 and in TC-025 step 11 ('Compare its oracle bytes, clause identity, generated harness source and native expectation with the plus-one Kani run'). | spec/kani/functional/FR-015-bounded-kani-obligations.md:789, spec/kani/matrix/TC-025-bounded-kani-obligations.md:305-321 |

## Verdict

PASS WITH LOW FINDING. FR-015-AC-41 and TC-025 step 11 agree with each other. One low gap: the playback is not bound to the same generated harness source.
