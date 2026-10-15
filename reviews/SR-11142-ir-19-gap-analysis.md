---
id: SR-11142
title: "IR-19 claimed-module proof gate gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@486f11c56db3f0c3db656ceb8ba3b7299eed0384; spec/kani/, scripts/kani_claims.json, scripts/kani_gate.sh, tests/it/kani_gate.rs, tests/it/kani_generation_claims.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-041", type: references }
---

## Summary

Ticket: IR-19. Planless repository matrix and focused reverse-gap audit of the frozen seven-file diff. Plan completion: not assessed. New trace tags exist, but required adverse controls are absent and the repository matrix has pre-existing untagged criteria.

## Reviewed units

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-041-AC-1 | examined | A nonempty reviewed declaration is checked against the current compiled-module census and the merge-base declaration. An absent compiled module, a removed claim, and an empty list each name the claim or condition and fail; adding a compiled claim does not erase an older one. |
| FR-041-AC-2 | examined | With two claimed modules sharing one `src/lib.rs`, a SUCCESS location inside only the first and an UNREACHABLE check inside the second reports the second `unreached` and fails. A SUCCESS check inside each passes this census; a SATISFIED cover, unknown location or non-SUCCESS status cannot supply the missing check. |
| FR-041-AC-3 | examined | The current build's transitive expectation/subject reachability yields the shared-helper intersection; the gate fails when the checked-in generated evidence omits or adds a helper, and records an explicit empty set when none is shared. For an exact operation, the expectation uses QSpec vectors or a model with no `quire-exact` call. |
| FR-041-AC-4 | examined | Each claimed module and computed shared helper has a positive unmodified real-Kani proof and a source mutation that compiles, executes, and reports a failed proposition check. Deleting a control or making its mutant verify, fail to compile, time out, skip execution, or merely trip the dirty-tree guard fails the gate; evidence names the exact diff, command and failed check from this run. |
| FR-041-AC-5 | examined | A stubbed executor, predetermined outcome or build-only run produces no qualifying module evidence and cannot pass the gate. |
| NFR-006-AC-19 | examined | On a clean current candidate, `make kani-gate` runs FR-041's module census and real mutation controls in its one required lane, and a failure of any claim or shared-helper control makes the gate exit non-zero. An optional mutation target or saved earlier result cannot substitute. |
| TC-056 | examined | The procedure requires two-module census, derived-helper disagreement, mutant compilation, timeout, inconclusive and stubbed-executor negative controls. |

## Verdict

**FAIL** — the computed repository matrix has untagged criteria and the changed feature lacks required negative control coverage.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | No integration control adds two real compiled claimed modules and verifies their separate SUCCESS and UNREACHABLE attribution; the unit test supplies invented extents and check records | tests/it/kani_generation_claims.rs:497 |
| FND-002 | high | Required negative mutation controls for compile failure, timeout, skipped execution, inconclusive result and verified mutant are absent; the sole real-Kani test is a positive path | tests/it/kani_generation_claims.rs:326 |
| FND-003 | high | The stand-in gate test only proves rejection when report files are absent; it does not test a stand-in that supplies nonempty fake reports and output, so FR-041-AC-5 has a hollow binding | tests/it/kani_gate.rs:228 |

## Coverage

- Reconciliation: `quoin matrix --repo /home/peter/dev/worktrees/cg-ir19-kani-claim-gate --json`, Quoin 0.28.3 and Quire 0.36.2; 769 criteria: 418 tagged, 296 untagged, 14 tagged-by-ignored-test, 41 method-without-symbol. No run evidence was bound. These repository-wide untagged criteria predate this diff and are not assigned to IR-19.
- Changed scope: FR-041-AC-1 through AC-3 are tagged; FR-041-AC-4, AC-5 and NFR-006-AC-19 are tagged only by the ignored real-Kani test, which the required `make kani-gate` lane selects. The author's focused positive run is reported separately; this matrix has no run evidence.
- Reverse gap: changed gate and test behavior is owned by FR-041 and NFR-006-AC-19. No new production API, vendored file or obvious source stub was found. The new tests and shell control are partially ineffective as described above.
- Plan completion: not assessed
- Semantic review: skipped; not requested as a separate opt-in.
