---
id: SR-616006
title: "IR-1092 PR #618 evidence review"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-015-bounded-kani-obligations.md:851; spec/kani/matrix/TC-025-bounded-kani-obligations.md:66-90; spec/kani/matrix/tests.md:39,113"
review_set: subset
---

## Summary

Reviewed the changed AC-101 Test verification and TC-025 owner-boundary controls against the current test inventory. The QSL and IR cases have no execution evidence in this PR, and the criterion retains PARTIAL status.

## Verdict

**CONDITIONAL** — TC-025 specifies an admitted CG negative case that cannot be produced through the strict public path.

## Scope

Examined FR-015-AC-101, TC-025 selected-field procedure, and Kani matrix status rows. Read spec/core/functional/interface-001-codegen-api.md and spec/tests.md as context; neither contains an AC-101-specific status claim. No applicable AssuranceProfile was found under spec/.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The revised TC-025 public admitted-claim CG negative control has no reachable fixture: the same procedure says malformed projection is rejected at strict admission, and CG declared_field documents that admitted input reaches neither missing-field branch. Separate a defensive helper invariant from authentic QSL/IR boundary tests and keep its evidence status partial. | spec/kani/matrix/TC-025-bounded-kani-obligations.md:71-79; src/kani/generate/collection.rs:443-466 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The TC-025 procedure now distinguishes authentic upstream refusal tests from CG’s defensive helper invariant and keeps the public CG negative control conditional on a reachable strict-admission path. Both matrix rows preserve PARTIAL status and source-located evidence gaps. |

## Disposition Verdict

**PASS at fb2ec40ad3a5f1cfcebda14a7c4ba291295366ee** — evidence plan is internally consistent. Execution evidence is still outstanding as the spec says; this spec-only PR does not claim it.
