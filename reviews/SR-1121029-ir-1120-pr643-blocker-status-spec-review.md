---
id: SR-1121029
title: "IR-1120 CG #643 blocker-status spec-review/base"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #643 head a2efb50caad8a066bd47b9ac908d830617f127b4; spec-review/base of the status edits since f8b6b0573c71cb0476a67253c00c1cb3a34005e6 in FR-045 Status, TC-059 Description and spec/routed/matrix/tests.md, plus the PR body as data"
review_set: subset
---

## Summary

Review of the QSL-1306 blocker-status edits at head a2efb50caad8a066bd47b9ac908d830617f127b4. The status text points to QSL-1306, keeps AC5 and TC-059 step 6 PARTIAL, lists every CG residual, and leaves the criteria unchanged. Two low wording findings: the edit left "that reader" without an antecedent, and one Status sentence does not say it describes QSL main, row 1, rather than the QSL revision CG pins.

## Verdict

PASS. Neither finding blocks the PR, overstates progress, or narrows a criterion. Nothing here claims Verified. QSL-1306's ticket state and the structured blocked-by edge were not read, because Linear is unavailable in this session; the pointer is confirmed as text only.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The edit left a dangling reference. "CG SHALL pass the exact extracted interior bytes through that reader" follows sentences that never name a reader, so "that reader" has no antecedent. The meaning is recoverable from TC-059 step 6, which names "QSL's FR-314 reader". Suggested text: "through QSL's FR-314 reader". | spec/routed/functional/FR-045-hyper-smt-induction-map.md:215-220; spec/routed/matrix/TC-059-hyper-smt-induction-map.md:78 |
| FND-002 | low | "QSL still returns `Unverifiable` at the first unsupported `rare_rewrite` command" is unscoped. It was measured on QSL main `5f3bef7b`, row 1 only. CG's Cargo.lock at this head pins QSL `93fcdc17`, where the same row 1 readback gives `ProofStepInvalid` at index 43. The Status therefore does not hold for the revision CG builds against, and Base and the other rows were not reread at `5f3bef7b`. The PR body states the `5f3bef7b` scope and both revisions, so only the spec text lacks it. This understates no blocker: closing AC5 still needs a CG pin at or after the fix. | spec/routed/functional/FR-045-hyper-smt-induction-map.md:214-216; Cargo.lock:1283; qsl-main-93fcdc17-row1-readback.log:29; qsl-main-5f3bef7b-row1-readback.log:210 |

## Coverage

Requested confirmations at this head:

| Check | Result | Evidence |
| --- | --- | --- |
| Status points to QSL-1306 | confirmed as text | FR-045:217 says QSL-1306 owns FR-315 query normalization for the remaining TC-059 step 6 blocker. TC-059:21, tests.md:34 and tests.md:66 name it. No spec surface outside the historical mention at FR-045:215 still names IR-1139. |
| AC5 and TC-059 step 6 stay PARTIAL | confirmed | FR-045:204 says PARTIAL. FR-045:217-220 bars claiming AC-5 or step 6 before reader acceptance. tests.md:34, tests.md:66 and spec/spec.md:96 say partial. |
| CG production driver listed | confirmed | FR-045:211-212; TC-059:20; tests.md:34; tests.md:66; PR body line 14 |
| Zero-step control listed | confirmed | FR-045:212-213; TC-059:20; tests.md:34; tests.md:66; PR body line 14 |
| Step 8 controls listed | confirmed | FR-045:213-214; TC-059:20; tests.md:34; tests.md:66. The PR body lists them by content, as FR-315 refusal plus `Sat`/failed-process/reached-limit. |
| No normative AC changed or narrowed | confirmed | See the limits below. FR-045 AC-1 through AC-7 match the local main checkout. TC-059 steps 5 and 7 match main, and steps 6 and 8 only add requirements. |

Evidence claims measured from local logs:

| Claim | Result | Evidence |
| --- | --- | --- |
| Head is a2efb50c | confirmed | Branch ref and worktree reflog entry 49 give the full SHA; entry 47 gives f8b6b057. |
| `5f3bef7b` row 1 returns `Unverifiable` at step 8 | confirmed | qsl-main-5f3bef7b-row1-readback.log:210, one test passed. |
| Step 8 is the first `rare_rewrite` | confirmed by trace | ir1139-cvc5-options-r2/baseline.stdout: line 3 is index 0, so line 11 is index 8, `t1 :rule rare_rewrite`, and it is the first such line. The same mapping puts `t36` at line 46 and index 43. This closes the trace SR-1121026 left open. |
| Prior `t36` rejection | confirmed | qsl-main-93fcdc17-row1-readback.log:29 gives `ProofStepInvalid` at index 43. |
| Saved query is 1,130 bytes | confirmed | ir1139-row1-query.smt2 matches a 1,130-character whole-file pattern. |
| Fresh query byte-equal to saved query | not measured | The 5f3bef7b log prints only the verdict, and SHA-256 could not be computed with read and search tools. |
| cvc5 version 1.4.2 | not measured | The manifest records no solver version. |
| QSL #1873 merged at `5f3bef7b`, QSL-1306 scope and edge | not measured | The log shows only the commit label. Linear, GitHub and a local QSL checkout were not available. These rest on the lead's report. |

Method limits: this session had no git, so no literal `git diff f8b6b057..a2efb50c` was run. The baseline was reconstructed from the main checkout and from the line citations in SR-1121026 and SR-1121028. Those citations still resolve at this head: FR-045:125 and TC-059:71-88 hold the same content, and the IR-1139 blocker range of FR-045:214-217 is now FR-045:214-220. That places the edits inside the Status. Main predates the IR-1120 content, so this is strong indirect evidence rather than a byte diff. The lead's aggregate diff check should confirm it.

The readback evidence uses cvc5 flags that include `--proof-granularity=dsl-rewrite-strict`, which `src/` does not select. qsl1873-dsl-row1-readback.log:25-27 gives the same row 1 verdict with and without it, so this does not change the status.

This reviewer ran no build, `make spec` or Quire validation, so this artifact's validity is unconfirmed; its structure follows SR-1121028. The `/spec-review` skill was not loaded, so the base method was applied from the brief. Only the base analysis was performed. Plan completion: not assessed. No branch edit or tracker post was made.

## Examined scope and bindings

```yaml
scope:
- id: FR-045-AC-4
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: For a refinement with seven step rows, CG retains one `Base` and exactly
    seven typed `Step { row }` records numbered 0 through 6 from the recompiled, canonically
    ordered `steps` array, even when records arrive out of order or the source rows
    were written in another order. Each record binds the refinement item, subjects,
    distinct invocation attempt, role and row, and exact FR-315 `Initial` or `Step
    { row }` query bytes sent on stdin; changing a query byte or exchanging the base
    and a step role refuses the handoff. With zero step rows, the complete shape has
    only `Base`.
- id: FR-045-AC-5
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: 'Given seven completed `Unsat` step records and one completed `Unsat` base
    record with present Alethe proofs, CG emits one deterministic handoff sequence
    in `Base`, `Step { row: 0 }` through `Step { row: 6 }` order for either input
    permutation. Every entry has separate length-delimited query and proof byte fields
    equal to the original FR-315 query and emitted Alethe proof, including a present
    empty proof. Repeating serialization of the same typed fields gives identical
    bytes; changing a proof byte changes only that entry''s proof field. The QSL adapter
    can construct `RefinementSmtCertificate` wi'
- id: FR-045-AC-7
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: Starting from AC-5's collection, making row 3's query or proof field absent
    yields a typed missing-field refusal and no handoff; making the proof present
    but empty preserves the empty bytes in the handoff. An FR-315 refusal, a `Sat`
    or `Unknown` answer, a missing or malformed response, a failed solver invocation
    or a reached limit remains typed non-proof evidence and produces no proof-bearing
    handoff.
bindings: []
```

## Dispositions

| Finding | Outcome | Evidence |
| --- | --- | --- |
| FND-001 | still-open: the lead decides; a one-phrase Status fix, not merge-blocking | FR-045:219 and TC-059:78 |
| FND-002 | still-open: the lead decides whether to scope the sentence to QSL main row 1; AC5 closure needs a CG pin at or after the fix either way | FR-045:214-216; Cargo.lock:1283 |
