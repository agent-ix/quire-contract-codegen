---
id: SR-646001
title: IR-1091 AC-96 evidence-status correction base spec review
type: SpecReview
analysis: base
scope: "CG PR 646; FR-015-AC-96, TC-025, and the FR-015 and TC-025 rows in spec/kani/matrix/tests.md"
review_set: base
---

## Summary

Base spec review of the three changed status files. No applicable AssuranceProfile was found in the scoped spec tree. The normative FR-015-AC-96 text is unchanged; the edited status now credits the installed-Kani OrderedSet controls at their exercised bounds and keeps AC-96 PARTIAL for admitted clause-body claims and incomplete TC-025 controls, owned by IR-1091. The status-only PR is READY after disposition of the findings below. This review does not accept the remaining AC-96 implementation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Initial status edit left both matrix rows saying installed-backend proof was pending. Fixed in this PR. | spec/kani/matrix/tests.md:40, spec/kani/matrix/tests.md:114 |
| FND-002 | low | Initial installed-Kani status implied broader domain coverage than the exercised IntRange lengths 2–3 and Boolean length 2. Fixed in this PR. | FR-015-AC-96, TC-025 |
| FND-003 | high | Initial proposal to mark AC-96 IMPLEMENTED overlooked the admitted clause-body path and incomplete TC-025 capacity and precedence controls. Fixed by retaining PARTIAL and naming IR-1091 as the residual owner. | FR-015-AC-96, TC-025 |
| FND-004 | high | Edited status rows retained historical commit-revision markers barred by the repository review checklist. Removed those markers in this PR. | FR-015-AC-96, TC-025, spec/kani/matrix/tests.md |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Both matrix rows now state the exact installed controls and PARTIAL residual consistently with FR-015 and TC-025. |
| FND-002 | fixed | All four status sites distinguish the generated symbolic-domain tests from the installed runs at the exercised bounds. |
| FND-003 | fixed | FR-015, TC-025 and both matrix rows retain PARTIAL and name the clause-body and TC-025 residual owned by IR-1091. The normative criterion was not narrowed. |
| FND-004 | fixed | The edited status rows identify issues and PRs without recorded commit revisions. |

## Coverage and checks

The changed files retain their requirement and criterion IDs, relationships, verification methods and acceptance text. Quire validation passed on the three changed files and this review artifact. Quoin matrix reports FR-015-AC-96 statically tagged by 16 binders, including the installed-Kani controls; its store has no bound run record. The independent Opus review read the generated source and installed test bodies, verified the recorded #616 run, and recommended the final PARTIAL status. No Rust build or Kani run was made for this Markdown-only PR.
