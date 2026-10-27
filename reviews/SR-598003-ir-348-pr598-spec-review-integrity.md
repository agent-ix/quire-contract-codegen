---
id: SR-598003
title: "IR-348 PR #598 spec-review/integrity (hold exception against existing gate obligations)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; PR #598 head d9ce52c931dbcf0bad43a35d4ff0706705814842 against origin/main e45be9eb; spec/kani/functional/FR-041-claimed-module-proof-gate.md, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md, spec/kani/matrix/TC-056-claimed-module-proof-gate.md, spec/kani/matrix/tests.md, spec/replay/matrix/tests.md; IR-348"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-041
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/NFR-006
    type: reviews
---

## Summary

Ticket: IR-348. This integrity lens checks whether the new hold bullets in FR-041 and NFR-006
agree with the obligations already in those documents and in their matrix rows. It also checks
that the replay matrix row split leaves no AC uncovered or duplicated.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The hold bullet does not say whether the gate exits zero for the held row, and it has no precedence over an existing rule. FR-041 Behavior bullet 1 and AC-1 say a declared module absent from the build SHALL fail the gate. The hold bullet says the absent V1 module "is reported as held rather than accepted as a discharged claim". Taken alone, that phrase still lets bullet 1 fail the gate, so two implementers would build opposite exit behavior. The statement should name bullet 1 and the AC-4 positive and mutation obligation as excepted for the held row only, and state the exit status. | spec/kani/functional/FR-041-claimed-module-proof-gate.md:36-39,61-72,78,82 |
| FND-002 | medium | NFR-006 requires a `held` count "separately from `ran` and `expected`". The next bullet, unchanged, fixes the exact run-result line as `kani-gate: result=<passed\|failed> ran=<n> expected=<n> elapsed=<s>s kani=<version> tree=<clean\|dirty>`, and TC-045 (Covered) asserts its exact fields. Nothing says whether `held=<n>` and the hold reason and expiry join that line, which would break the fixed form and TC-045, or go on a separate line. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:44-55; spec/kani/matrix/tests.md:89; scripts/cg_gates.rs:201 |

## Verdict

Changes requested for FND-001 and FND-002. Everything else checked is consistent:

- **Replay matrix split.** It covers AC-1–5, AC-8 and AC-9–23, with AC-6, AC-24 and AC-25 on
  their own rows. Nothing is dropped or duplicated.
- **Kani matrix.** The FR-041, NFR-006 (AC-19) and TC-056 summary rows agree with each other and
  with FR-041 Status and TC-056 Status (all Open, IR-19, hold under IR-348, V2 under IR-1047).
- **TC-056 step 6.** It tests the held-row invariants that the FR-041 and NFR-006 statements
  define.

## Coverage

```yaml
scope:
- {id: FR-041:behavior-1, path: spec/kani/functional/FR-041-claimed-module-proof-gate.md, role: examined, excerpt: "It SHALL fail for an empty declaration, a declared module absent from the build, or a claim removed since that base, naming each missing or removed module."}
- {id: FR-041:hold-bullet, path: spec/kani/functional/FR-041-claimed-module-proof-gate.md, role: examined, excerpt: "Its absent compiled V1 module is reported as held rather than accepted as a discharged claim; the declaration row and its merge-base identity remain visible."}
- {id: FR-041-AC-1, path: spec/kani/functional/FR-041-claimed-module-proof-gate.md, role: examined, excerpt: "An absent compiled module, a removed claim, and an empty list each name the claim or condition and fail"}
- {id: FR-041-AC-4, path: spec/kani/functional/FR-041-claimed-module-proof-gate.md, role: examined, excerpt: "Each claimed module and computed shared helper has a positive unmodified real-Kani proof and a source mutation that compiles, executes, and reports a failed proposition check."}
- {id: NFR-006:hold-bullet, path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md, role: examined, excerpt: "count it as held separately from ran and expected."}
- {id: NFR-006:result-line, path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md, role: examined, excerpt: "The make kani-gate target shall print one run-result line of the form kani-gate: result=<passed|failed> ran=<n> expected=<n> elapsed=<s>s kani=<version> tree=<clean|dirty>"}
- {id: NFR-006-AC-19, path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md, role: examined, excerpt: "a failure of any claim or shared-helper control makes the gate exit non-zero."}
- {id: TC-056:step-6, path: spec/kani/matrix/TC-056-claimed-module-proof-gate.md, role: examined, excerpt: "Assert that a hold on another row, a missing reason or expiry, a removed declaration row, and any failed, missing or skipped non-held control make the gate fail."}
- {id: replay-tests.md:FR-016-rows, path: spec/replay/matrix/tests.md, role: examined, excerpt: "FR-016-AC-1 through FR-016-AC-5, FR-016-AC-9 through FR-016-AC-23 ... FR-016-AC-8 ... FR-016-AC-6 ... FR-016-AC-24 ... FR-016-AC-25"}
- {id: kani-tests.md:rows, path: spec/kani/matrix/tests.md, role: examined, excerpt: "Open (IR-19): only the V1 claim row is operator-held, printed with reason/expiry and counted separately"}
```
