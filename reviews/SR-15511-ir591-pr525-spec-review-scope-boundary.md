---
id: SR-15511
title: "CG PR #525 generated-closure corpus scope-boundary review"
type: SpecReview
analysis: scope-boundary
scope: "CG PR #525 at 8346f8f7: FR-044, TC-058 and QSL/CG ownership edges"
review_set: subset
---
# SR-15511: CG PR #525 generated-closure corpus scope-boundary review

## Summary

Reviewed the CG producer against QSL's shared evaluator ownership and the
current CG-facing facade rule. CG correctly leaves comparison and encoding to
QSL, but its proposed direct shared-interface call lacks the required
architecture-boundary dependency.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | FR-044 requires CG to call QSL's shared checked-closure/application interface, while current QSL FR-060 T12-A still permits CG calls only through `qsl-replay`. IR-853 is Done and its decision calls for a later narrow T12-A amendment; its status cannot itself authorize this path. Name the amendment as a blocking dependency and make the planned producer contingent on it (or constrain the path to the current facade). The QSL-owned corpus loader's unknown-format/admission refusal should be defined by STD-300 and referenced here, rather than specified independently in CG. | `spec/oracle/functional/FR-044-export-generated-closure-evaluation-corpus.md:18-21,41-44,68-72`; QSL `spec/functional/FR-060-check-qsl-api-surface-boundary.md` T12-A; IR-853 decision | wrong-requirement |
