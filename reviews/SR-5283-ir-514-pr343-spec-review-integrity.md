---
id: SR-5283
title: spec-review/integrity review of IR-514 native run result consumer
type: SpecReview
analysis: integrity
scope: agent-ix/quire-contract-codegen@e512f543675dd9e86bf397f993f8d6f5079d71f9; spec/core/functional/interface-001-codegen-api.md,
  spec/evidence/functional/FR-004-vacuity-evidence.md, spec/evidence/matrix/TC-006-vacuity.md,
  spec/evidence/matrix/tests.md
review_set: subset
---

## Summary

The revised interface, FR-004 acceptance criteria, TC-006 controls and evidence matrix were reviewed against QSL FR-267/FR-109 at 7024215f. The QSL typed reader and CG receipt ownership align with the merged contract.
 Ticket: IR-514. Reviewed commit: e512f543675dd9e86bf397f993f8d6f5079d71f9.

## Verdict

**PASS** — no additional finding in this method.

## Examined scope

- `interface-001/analyze_coverage` (`spec/core/functional/interface-001-codegen-api.md`, examined): inputs: [bound executable population, generated source and maps actually used by the native coverage producer, LLVM coverage JSON bytes and producer tool/version, CG-owned authenticated producer/artifact binding receipt associating the exact QSL result and generated-campaign execution with the actual source and map, QSL strict-reader typed clause-run or command-error result for native-run-result/2 or its located refusal, source root, runtime campaign report]
- `FR-004-AC-10` (`spec/evidence/functional/FR-004-vacuity-evidence.md`, examined): PLANNED. A CG-owned authenticated producer/artifact receipt binds the exact decoded QSL result and producer execution to the analyzed generated source and source map actually run, matches their requirement and revision to the campaign, and retains the LLVM producer tool and version; a changed or missing result/run/map/source association or producer identity yields structured non-success with no campaign qualification, measured coverage classification or coverage discharge. A QSL `native-run-result/2` document alone does not supply this receipt.
- `FR-004-AC-11` (`spec/evidence/functional/FR-004-vacuity-evidence.md`, examined): PLANNED. The consuming report retains vacuity and native outcome as evidence for IR FR-045; an oracle-success run with an unobserved consequent denies the coverage obligation, and LLVM probe observations alone produce no completed-proof credit or separate eligible denominator.
- `FR-004-AC-12` (`spec/evidence/functional/FR-004-vacuity-evidence.md`, examined): PLANNED. CG consumes QSL's typed strict-reader clause-run or command-error variant for exact `native-run-result/2`: a decisive basis retains its witness with assigned members, while `closed-scope` or `unavailable` retains none. Missing or other versions, missing/unknown basis, malformed witness, contradictory basis/witness and mixed/invalid variant shape are located wire refusals before binding; no `/1` fallback or CG copy of the wire reader exists.
- `FR-004-AC-13` (`spec/evidence/functional/FR-004-vacuity-evidence.md`, examined): PLANNED. QSL `run_clause` success and a decisive witness remain source-clause facts and do not count as generated-campaign execution or LLVM coverage; QSL violation, refusal, undefined, incomplete, cancelled, unsupported and internal failure remain distinct typed adverse/non-success facts. A matching authenticated receipt does not upgrade any QSL disposition or itself grant campaign success, measured classification, proof credit or coverage discharge. The same reader's command-error variant retains stage/code/optional cause/message/details and unavailable basis as non-success, with no campaign
- `FR-004-AC-14` (`spec/evidence/functional/FR-004-vacuity-evidence.md`, examined): PLANNED. A valid `/2` clause-run result with absent receipt, omitted association or only caller-declared paths, QSL `package_id` or self-declared producer metadata yields a structured absent/unauthenticated binding diagnostic while retaining the decoded outcome and available identities; no campaign qualification, measured classification or coverage discharge occurs. A fully authenticated receipt associating the exact result, producer execution, generated source and source map admits binding and retains the unchanged QSL outcome.
- `FR-004-AC-15` (`spec/evidence/functional/FR-004-vacuity-evidence.md`, examined): PLANNED. Independently replacing the result, producer execution, generated source or source map after authenticated binding refuses with the mismatched association identified, even when paths or QSL `package_id` match; the original decoded outcome remains distinct from the binding diagnostic. A valid typed command-error variant stays non-success even with a matching receipt, while an invalid `/2` or unsupported version refuses at QSL reading before receipt verification.
- `TC-006/native-run-result-2` (`spec/evidence/matrix/TC-006-vacuity.md`, examined): ## Planned native-run-result/2 consumer controls
- `TM-007/FR-004` (`spec/evidence/matrix/tests.md`, examined): | FR-004 | FR-004-AC-12 through FR-004-AC-15 | TC-006 | 🚧 Planned; QSL `/2` strict reader and CG authenticated producer/artifact binding are not implemented |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
